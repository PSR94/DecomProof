use crate::{config::PolicyConfig, evidence::{ConsumerIdentity, Evidence, Uncertainty}};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub enum Verdict { Blocked, InsufficientEvidence, Observing, Quiescent, ReadyWithUncertainty, Ready, Removed, Verified }
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Severity { Informational, Low, Medium, High, Critical }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Blocker { pub id:String, pub severity:Severity, pub kind:String, pub message:String, pub hard:bool, #[serde(default)] pub evidence_ids:Vec<String> }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDecision { pub policy:String, pub required:String, pub available:String, pub passed:bool, pub effect:String, #[serde(default)] pub evidence_ids:Vec<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluation { pub decisions:Vec<PolicyDecision>, pub blockers:Vec<Blocker>, pub uncertainties:Vec<Uncertainty> }

pub fn evaluate(policy:&PolicyConfig,evidence:&[Evidence],now:DateTime<Utc>)->PolicyEvaluation{
 let mut decisions=Vec::new();let mut blockers=Vec::new();let mut uncertainties=Vec::new();let max_age=(policy.evidence_maximum_age_hours as i64)*3600;
 for ev in evidence{
  if ev.age_seconds(now)>max_age&&is_dynamic(&ev.signal){uncertainties.push(Uncertainty{id:format!("unc_stale_{}",ev.id),kind:"stale-evidence".into(),message:format!("{} evidence is older than policy maximum",ev.signal),severity:"high".into(),evidence_ids:vec![ev.id.clone()]});}
  if ev.consumer.as_ref().map(|c|c.identity==ConsumerIdentity::Unknown).unwrap_or(false){blockers.push(Blocker{id:format!("blk_unknown_{}",ev.id),severity:Severity::High,kind:"unidentified-consumer".into(),message:"Runtime activity has an unidentified consumer".into(),hard:policy.unidentified_consumers_block,evidence_ids:vec![ev.id.clone()]});}
  if active(ev){let(sev,kind,msg)=active_blocker(ev);blockers.push(Blocker{id:format!("blk_active_{}",ev.id),severity:sev,kind,message:msg,hard:true,evidence_ids:vec![ev.id.clone()]});}
 }
 let runtime:Vec<_>=evidence.iter().filter(|e|e.signal.starts_with("runtime.")||e.signal.starts_with("metric.")).collect();let runtime_available=max_window(&runtime);let runtime_required=(policy.runtime_minimum_window_days as i64)*86400;
 if runtime.is_empty(){decisions.push(fail("runtime.minimum_observation_window",format!("{} days",policy.runtime_minimum_window_days),"no runtime evidence","insufficient-evidence"));}else{decisions.push(window_decision("runtime.minimum_observation_window",policy.runtime_minimum_window_days,runtime_available,&runtime));}
 let external_required=(policy.external_minimum_window_days as i64)*86400;if !runtime.is_empty(){decisions.push(PolicyDecision{policy:"external_consumers.minimum_observation_window".into(),required:format!("{} days",policy.external_minimum_window_days),available:format!("{:.1} days",runtime_available as f64/86400.0),passed:runtime_available>=external_required,effect:if runtime_available>=external_required{"none".into()}else{"observing".into()},evidence_ids:runtime.iter().map(|e|e.id.clone()).collect()});}
 let db:Vec<_>=evidence.iter().filter(|e|e.signal.starts_with("database.write")&&!e.signal.ends_with("static")).collect();if !db.is_empty(){decisions.push(window_decision("data.require_zero_writes_for",policy.data_zero_writes_days,max_window(&db),&db));}
 let schedules:Vec<_>=evidence.iter().filter(|e|e.signal=="schedule.observation").collect();if !schedules.is_empty(){let cycles=schedules.iter().filter_map(|e|e.normalized.get("cycles").and_then(|v|v.as_u64())).max().unwrap_or(0);decisions.push(PolicyDecision{policy:"schedules.minimum_observed_cycles".into(),required:format!("{} cycles",policy.scheduled_minimum_cycles),available:format!("{} cycles",cycles),passed:cycles>=policy.scheduled_minimum_cycles as u64,effect:if cycles>=policy.scheduled_minimum_cycles as u64{"none".into()}else{"observing".into()},evidence_ids:schedules.iter().map(|e|e.id.clone()).collect()});}
 let stale=uncertainties.iter().any(|u|u.kind=="stale-evidence");decisions.push(PolicyDecision{policy:"evidence.maximum_age".into(),required:format!("<= {} hours",policy.evidence_maximum_age_hours),available:if stale{"stale dynamic evidence present".into()}else{"within age bound or static".into()},passed:!stale,effect:if stale{"insufficient-evidence".into()}else{"none".into()},evidence_ids:vec![]});
 let _=runtime_required;PolicyEvaluation{decisions,blockers,uncertainties}
}
fn max_window(items:&[&Evidence])->i64{items.iter().filter_map(|e|e.observation_window.as_ref().map(|w|w.seconds())).max().unwrap_or(0)}
fn window_decision(name:&str,days:u32,available:i64,items:&[&Evidence])->PolicyDecision{let required=(days as i64)*86400;PolicyDecision{policy:name.into(),required:format!("{} days",days),available:format!("{:.1} days",available as f64/86400.0),passed:available>=required,effect:if available>=required{"none".into()}else{"observing".into()},evidence_ids:items.iter().map(|e|e.id.clone()).collect()}}
fn fail(policy:&str,required:String,available:&str,effect:&str)->PolicyDecision{PolicyDecision{policy:policy.into(),required,available:available.into(),passed:false,effect:effect.into(),evidence_ids:vec![]}}
fn is_dynamic(s:&str)->bool{s.starts_with("runtime.")||s.starts_with("database.")||s.starts_with("event.")||s.starts_with("metric.")||s=="schedule.observation"||s.starts_with("verification.")}
fn active(e:&Evidence)->bool{e.normalized.get("active").and_then(|v|v.as_bool()).unwrap_or(false)||(e.normalized.get("count").and_then(|v|v.as_u64()).unwrap_or(0)>0&&(e.signal.starts_with("runtime.")||e.signal.starts_with("database.write")||e.signal.starts_with("event.consume")))||e.signal=="schedule.reference"||e.signal=="contract.public"||e.signal=="sdk.export"||e.signal=="infra.resource"}
fn active_blocker(e:&Evidence)->(Severity,String,String){if e.signal.starts_with("runtime."){(Severity::Critical,"active-runtime-consumer".into(),"Runtime usage observed in the evidence window".into())}else if e.signal.starts_with("database.write"){(Severity::Critical,"active-data-write".into(),"Database writes observed in the evidence window".into())}else if e.signal=="schedule.reference"{(Severity::Critical,"active-schedule".into(),"Scheduled workload still references the target".into())}else if e.signal.starts_with("event.consume"){(Severity::Critical,"active-event-consumer".into(),"Event consumer still depends on the target".into())}else if e.signal=="contract.public"{(Severity::High,"public-contract".into(),"Target remains in a public API contract".into())}else if e.signal=="sdk.export"{(Severity::High,"public-sdk-export".into(),"Target remains exported by an SDK/package".into())}else{(Severity::High,"infrastructure-present".into(),"Infrastructure resource for target remains present".into())}}

#[cfg(test)]mod tests{use super::*;use crate::{target::Target,evidence::Evidence};#[test]fn missing_runtime_fails_conservatively(){let r=evaluate(&PolicyConfig::default(),&[],Utc::now());assert!(r.decisions.iter().any(|d|!d.passed&&d.effect=="insufficient-evidence"));}#[test]fn schedule_is_hard_blocker(){let t:Target="service:x".parse().unwrap();let mut e=Evidence::build("schedule.reference","static",&t,Utc::now(),None,b"cron");e.normalized.insert("active".into(),true.into());let r=evaluate(&PolicyConfig::default(),&[e],Utc::now());assert!(r.blockers.iter().any(|b|b.hard));}}
