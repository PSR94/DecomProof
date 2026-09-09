use crate::target::Target;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path, str::FromStr};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LifecycleState { Active, Deprecated, Observing, Quiescent, Ready, Removed, Verified }

impl std::fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", format!("{:?}", self).to_ascii_uppercase()) }
}
impl FromStr for LifecycleState {
    type Err = LifecycleError;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw.trim().to_ascii_uppercase().as_str() {
            "ACTIVE" => Ok(Self::Active), "DEPRECATED" => Ok(Self::Deprecated), "OBSERVING" => Ok(Self::Observing),
            "QUIESCENT" => Ok(Self::Quiescent), "READY" => Ok(Self::Ready), "REMOVED" => Ok(Self::Removed), "VERIFIED" => Ok(Self::Verified),
            _ => Err(LifecycleError::UnknownState(raw.into())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleEvent { pub state: LifecycleState, pub at: DateTime<Utc>, #[serde(skip_serializing_if="Option::is_none")] pub note: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleRecord { pub target: String, pub state: LifecycleState, pub updated_at: DateTime<Utc>, pub history: Vec<LifecycleEvent> }
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LifecycleStore { pub targets: BTreeMap<String, LifecycleRecord> }

#[derive(Debug, Error)]
pub enum LifecycleError {
    #[error("unknown lifecycle state {0}")] UnknownState(String),
    #[error("invalid lifecycle transition {0} -> {1}")] InvalidTransition(LifecycleState, LifecycleState),
    #[error("read lifecycle state: {0}")] Io(#[from] std::io::Error),
    #[error("parse lifecycle state: {0}")] Json(#[from] serde_json::Error),
}

impl LifecycleStore {
    pub fn load(path: &Path) -> Result<Self, LifecycleError> { if path.exists() { Ok(serde_json::from_str(&fs::read_to_string(path)?)?) } else { Ok(Self::default()) } }
    pub fn save(&self, path: &Path) -> Result<(), LifecycleError> { if let Some(parent)=path.parent(){fs::create_dir_all(parent)?;} fs::write(path, serde_json::to_string_pretty(self)? + "\n")?; Ok(()) }
    pub fn record_for(&mut self, target: &Target, now: DateTime<Utc>) -> &mut LifecycleRecord {
        self.targets.entry(target.stable_id()).or_insert_with(|| LifecycleRecord { target: target.stable_id(), state: LifecycleState::Active, updated_at: now, history: vec![LifecycleEvent { state: LifecycleState::Active, at: now, note: Some("initialized".into()) }] })
    }
    pub fn transition(&mut self, target: &Target, next: LifecycleState, now: DateTime<Utc>, note: Option<String>) -> Result<(), LifecycleError> {
        let record=self.record_for(target,now); if !allowed(record.state,next){return Err(LifecycleError::InvalidTransition(record.state,next));} record.state=next;record.updated_at=now;record.history.push(LifecycleEvent{state:next,at:now,note});Ok(())
    }
}
fn allowed(from:LifecycleState,to:LifecycleState)->bool{from==to||matches!((from,to),(LifecycleState::Active,LifecycleState::Deprecated)|(LifecycleState::Deprecated,LifecycleState::Observing)|(LifecycleState::Observing,LifecycleState::Quiescent)|(LifecycleState::Quiescent,LifecycleState::Ready)|(LifecycleState::Ready,LifecycleState::Removed)|(LifecycleState::Removed,LifecycleState::Verified))}

#[cfg(test)]mod tests{use super::*;#[test]fn prevents_skipping_to_ready(){let t:Target="service:x".parse().unwrap();let mut s=LifecycleStore::default();assert!(s.transition(&t,LifecycleState::Ready,Utc::now(),None).is_err());}}
