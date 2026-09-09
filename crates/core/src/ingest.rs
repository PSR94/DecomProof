use crate::{evidence::{Confidence,Consumer,ConsumerIdentity,Evidence,ObservationWindow},target::Target};
use chrono::{DateTime,Utc};
use serde::Deserialize;
use std::{collections::BTreeMap,fs,path::Path};
use thiserror::Error;

#[derive(Debug,Error)] pub enum IngestError{#[error("read evidence: {0}")]Io(#[from]std::io::Error),#[error("invalid json line {line}: {source}")]Json{line:usize,source:serde_json::Error}}
#[derive(Debug,Deserialize)] struct Generic{signal:String,#[serde(default="source")]source:String,observed_at:DateTime<Utc>,#[serde(default)]window_start:Option<DateTime<Utc>>,#[serde(default)]window_end:Option<DateTime<Utc>>,#[serde(default)]count:Option<u64>,#[serde(default)]active:Option<bool>,#[serde(default)]consumer_type:Option<String>,#[serde(default)]consumer_id:Option<String>,#[serde(default)]consumer_identification:Option<String>,#[serde(flatten)]extra:BTreeMap<String,serde_json::Value>}
fn source()->String{"generic-json".into()}

pub fn ingest_jsonl(path:&Path,target:&Target)->Result<Vec<Evidence>,IngestError>{
 let raw=fs::read_to_string(path)?;let mut out=Vec::new();
 for (i,line) in raw.lines().enumerate(){if line.trim().is_empty(){continue}let row:Generic=serde_json::from_str(line).map_err(|source|IngestError::Json{line:i+1,source})?;let mut ev=Evidence::build(&row.signal,&row.source,target,row.observed_at,Some(path.display().to_string()),line.as_bytes());ev.confidence=Confidence::High;
  if let(Some(a),Some(b))=(row.window_start,row.window_end){ev.observation_window=Some(ObservationWindow{start:a,end:b,gaps_seconds:0});}
  if let Some(c)=row.count{ev.normalized.insert("count".into(),c.into());} if let Some(a)=row.active{ev.normalized.insert("active".into(),a.into());}
  for(k,v)in row.extra{ev.observed.insert(k,v);}
  if let Some(id)=row.consumer_id{let ident=match row.consumer_identification.as_deref(){Some("identified")=>ConsumerIdentity::Identified,Some("partial")=>ConsumerIdentity::Partial,_=>ConsumerIdentity::Unknown};ev.consumer=Some(Consumer{identity:ident,consumer_type:row.consumer_type.unwrap_or_else(||"unknown".into()),id:hash_identifier(&id)});}
  out.push(ev);
 } Ok(out)
}
pub fn hash_identifier(raw:&str)->String{use sha2::{Digest,Sha256};format!("sha256:{}",&hex::encode(Sha256::digest(raw.as_bytes()))[..20])}
