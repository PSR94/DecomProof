use crate::{config::PolicyConfig,evidence::{ConsumerIdentity,Evidence,Uncertainty}};
use chrono::{DateTime,NaiveDate,Utc};
use serde::{Deserialize,Serialize};

#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]#[serde(rename_all="kebab-case")]pub enum Verdict{Blocked,InsufficientEvidence,Observing,Quiescent,ReadyWithUncertainty,Ready,Removed,Verified}
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize)]#[serde(rename_all="lowercase")]pub enum Severity{Informational,Low,Medium,High,Critical}
#[derive(Debug,Clone,PartialEq,Eq,Serialize,Deserialize)]pub struct Blocker{pub id:String,pub severity:Severity,pub kind:String,pub message:String,pub hard:bool,#[serde(default)]pub evidence_ids:Vec<String>}
#[derive(Debug,Clone,PartialEq,Eq,Serialize,Deserialize)]pub struct PolicyDecision{pub policy:String,pub required:String,pub available:String,pub passed:bool,pub effect:String,#[serde(default)]pub evidence_ids:Vec<String>}
#[derive(Debug,Clone,Serialize,Deserialize)]pub struct PolicyEvaluation{pub decisions:Vec<PolicyDecision>,pub blockers:Vec<Blocker>,pub uncertainties:Vec<Uncertainty>}

pub fn evaluate(policy:&PolicyConfig,evidence:&[Evidence],now:DateTime<Utc>)->PolicyEvaluation{
 let mut decisions=Vec::new();let mut blockers=Vec::new();let mut uncertainties=Vec::new();let max_age=(policy.evidence_maximum_age_hours as i64)*3600;
 for ev in evidence{
  if ev.age_seconds(now)>max_age&&is_dynamic(&ev.signal){uncertainties.push(Uncertainty{id:format!("unc_stale_{}",ev.id),kind:"stale-evidence".into(),message:format!("{} evidence is older than policy maximum",ev.signal),severity:"high".into(),evidence_ids:vec![ev.id.clone()]});}
  if is_runtime_family(&ev.signal)&&is_active(ev)&&ev.consumer.as_ref().map(|c|c.identity==ConsumerIdentity::Unknown).unwrap_or(false){blockers.push(Blocker{id:format!("blk_unknown_{}",ev.id),severity:Severity::High,kind:"unidentified-consumer".into(),message:"Runtime activity has an unidentified consumer".into(),hard:policy.unidentified_consumers_block,evidence_ids:vec![ev.id.clone()]});}
  if let Some((severity,kind,message,hard))=blocker_for(ev){blockers.push(Blocker{id:format!("blk_{}_{}",kind.replace('-',"_"),ev.id),severity,kind,message,hard,evidence_ids:vec![ev.id.clone()]});}
 }
 let runtime:Vec<_>=evidence.iter().filter(|e|is_runtime_family(&e.signal)).collect();let runtime_available=max_window(&runtime);
 if runtime.is_empty(){decisions.push(fail("runtime.minimum_observation_window",format!("{} days",policy.runtime_minimum_window_days),"no runtime evidence","insufficient-evidence"));}else{decisions.push(window_decision("runtime.minimum_observation_window",policy.runtime_minimum_window_days,runtime_available,&runtime));decisions.push(window_decision("external_consumers.minimum_observation_window",policy.external_minimum_window_days,runtime_available,&runtime));}
 let db:Vec<_>=evidence.iter().filter(|e|e.signal=="database.write").collect();if !db.is_empty(){decisions.push(window_decision("data.require_zero_writes_for",policy.data_zero_writes_days,max_window(&db),&db));}
 let schedules:Vec<_>=evidence.iter().filter(|e|e.signal=="schedule.observation").collect();if !schedules.is_empty(){let cycles=schedules.iter().filter_map(|e|e.normalized.get("cycles").and_then(|v|v.as_u64())).max().unwrap_or(0);decisions.push(PolicyDecision{policy:"schedules.minimum_observed_cycles".into(),required:format!("{} cycles",policy.scheduled_minimum_cycles),available:format!("{} cycles",cycles),passed:cycles>=policy.scheduled_minimum_cycles as u64,effect:if cycles>=policy.scheduled_minimum_cycles as u64{"none".into()}else{"observing".into()},evidence_ids:schedules.iter().map(|e|e.id.clone()).collect()});}
 let contracts:Vec<_>=evidence.iter().filter(|e|e.signal=="contract.public").collect();if !contracts.is_empty(){if policy.public_api_requires_deprecation{let deprecated=contracts.iter().all(|e|e.normalized.get("deprecated").and_then(|v|v.as_bool()).unwrap_or(false));decisions.push(PolicyDecision{policy:"api.require_deprecation".into(),required:"deprecated: true".into(),available:if deprecated{"all public contracts deprecated".into()}else{"one or more public contracts not deprecated".into()},passed:deprecated,effect:if deprecated{"none".into()}else{"blocked".into()},evidence_ids:contracts.iter().map(|e|e.id.clone()).collect()});}
  let dates:Vec<_>=contracts.iter().filter_map(|e|e.normalized.get("sunset_date").and_then(|v|v.as_str()).and_then(|s|NaiveDate::parse_from_str(s,"%Y-%m-%d").ok())).collect();let passed=dates.len()==contracts.len()&&dates.iter().all(|d|now.date_naive().signed_duration_since(*d).num_days()>=policy.api_minimum_sunset_days as i64);decisions.push(PolicyDecision{policy:"api.minimum_sunset_window".into(),required:format!("{} days after declared sunset date",policy.api_minimum_sunset_days),available:if dates.len()!=contracts.len(){"sunset date missing".into()}else{format!("{} declared sunset date(s)",dates.len())},passed,effect:if passed{"none".into()}else{"blocked".into()},evidence_ids:contracts.iter().map(|e|e.id.clone()).collect()});}
 let stale=uncertainties.iter().any(|u|u.kind=="stale-evidence");decisions.push(PolicyDecision{policy:"evidence.maximum_age".into(),required:format!("<= {} hours",policy.evidence_maximum_age_hours),available:if stale{"stale dynamic evidence present".into()}else{"within age bound or static".into()},passed:!stale,effect:if stale{"insufficient-evidence".into()}else{"none".into()},evidence_ids:vec![]});
 PolicyEvaluation{decisions,blockers,uncertainties}
}
fn is_runtime_family(s:&str)->bool{s.starts_with("runtime.")||s.starts_with("metric.")}
fn is_dynamic(s:&str)->bool{is_runtime_family(s)||s=="database.write"||s.starts_with("event.consume")||s=="schedule.observation"||s.starts_with("verification.")}
fn is_active(e:&Evidence)->bool{e.normalized.get("active").and_then(|v|v.as_bool()).unwrap_or(false)||e.normalized.get("count").and_then(|v|v.as_u64()).unwrap_or(0)>0||e.normalized.get("count").and_then(|v|v.as_f64()).unwrap_or(0.0)>0.0}
fn blocker_for(e:&Evidence)->Option<(Severity,String,String,bool)>{
 match e.signal.as_str(){
  "schedule.reference"=>Some((Severity::Critical,"active-schedule".into(),"Scheduled workload still references the target".into(),true)),
  "contract.public"=>Some((Severity::High,"public-contract".into(),"Target remains in a public API contract".into(),true)),
  "sdk.export"=>Some((Severity::High,"public-sdk-export".into(),"Target remains exported by an SDK/package".into(),true)),
  "infra.resource"=>Some((Severity::High,"infrastructure-present".into(),"Infrastructure resource for target remains present".into(),true)),
  "ci.reference"=>Some((Severity::High,"ci-reference".into(),"CI/CD configuration still references the target".into(),true)),
  "static.reference"|"static.text-reference"=>Some((Severity::High,"static-reference".into(),"Source/configuration still references the target".into(),true)),
  "database.reference"=>Some((Severity::High,"data-reference".into(),"SQL still references the target".into(),true)),
  "database.write.static"=>Some((Severity::Critical,"static-data-write".into(),"A static SQL write path still targets the component/data object".into(),true)),
  "database.table.exists"|"database.column.exists"=>if is_active(e){Some((Severity::High,"data-object-present".into(),"Database object still exists".into(),true))}else{None},
  "database.rows"=>if is_active(e){Some((Severity::High,"data-present".into(),"Database object still contains rows".into(),true))}else{None},
  "database.views"|"database.foreign_keys"=>if is_active(e){Some((Severity::High,"data-dependency".into(),"Database dependency still references the object".into(),true))}else{None},
  "event.reference"=>Some((Severity::High,"event-reference".into(),"Source/configuration still references the event or topic".into(),true)),
  "documentation.reference"=>Some((Severity::Low,"documentation-reference".into(),"Current documentation still references the target".into(),false)),
  s if is_runtime_family(s)&&is_active(e)=>Some((Severity::Critical,"active-runtime-consumer".into(),"Runtime usage observed in the evidence window".into(),true)),
  "database.write" if is_active(e)=>Some((Severity::Critical,"active-data-write".into(),"Database writes observed in the evidence window".into(),true)),
  "event.consume" if is_active(e)=>Some((Severity::Critical,"active-event-consumer".into(),"Event consumer still depends on the target".into(),true)),
  _=>None,
 }
}
fn max_window(items:&[&Evidence])->i64{items.iter().filter_map(|e|e.observation_window.as_ref().map(|w|w.seconds())).max().unwrap_or(0)}
fn window_decision(name:&str,days:u32,available:i64,items:&[&Evidence])->PolicyDecision{let required=(days as i64)*86400;PolicyDecision{policy:name.into(),required:format!("{} days",days),available:format!("{:.1} days",available as f64/86400.0),passed:available>=required,effect:if available>=required{"none".into()}else{"observing".into()},evidence_ids:items.iter().map(|e|e.id.clone()).collect()}}
fn fail(policy:&str,required:String,available:&str,effect:&str)->PolicyDecision{PolicyDecision{policy:policy.into(),required,available:available.into(),passed:false,effect:effect.into(),evidence_ids:vec![]}}

#[cfg(test)]mod tests{use super::*;use crate::{target::Target,evidence::Evidence};#[test]fn missing_runtime_fails_conservatively(){let r=evaluate(&PolicyConfig::default(),&[],Utc::now());assert!(r.decisions.iter().any(|d|!d.passed&&d.effect=="insufficient-evidence"));}#[test]fn documentation_is_soft_not_infra(){let t:Target="service:x".parse().unwrap();let e=Evidence::build("documentation.reference","scanner",&t,Utc::now(),None,b"x");let r=evaluate(&PolicyConfig::default(),&[e],Utc::now());assert!(r.blockers.iter().any(|b|!b.hard&&b.kind=="documentation-reference"));assert!(!r.blockers.iter().any(|b|b.kind=="infrastructure-present"));}}
