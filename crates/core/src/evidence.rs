use crate::target::Target;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Confidence {
    Low,
    Medium,
    High,
    Confirmed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConsumerIdentity {
    Identified,
    Partial,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Consumer {
    pub identity: ConsumerIdentity,
    #[serde(rename = "type")]
    pub consumer_type: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationWindow {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    #[serde(default)]
    pub gaps_seconds: u64,
}
impl ObservationWindow {
    pub fn seconds(&self) -> i64 {
        (self.end - self.start).num_seconds().max(0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evidence {
    pub id: String,
    pub signal: String,
    pub source: String,
    pub target: String,
    pub observed_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_window: Option<ObservationWindow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness_seconds: Option<i64>,
    pub confidence: Confidence,
    #[serde(default)]
    pub observed: BTreeMap<String, serde_json::Value>,
    #[serde(default)]
    pub normalized: BTreeMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer: Option<Consumer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    pub raw_hash: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

impl Evidence {
    pub fn build(
        signal: &str,
        source: &str,
        target: &Target,
        observed_at: DateTime<Utc>,
        artifact: Option<String>,
        raw: &[u8],
    ) -> Self {
        let raw_hash = format!("sha256:{}", hex::encode(Sha256::digest(raw)));
        let stable = format!("{}\n{}\n{}\n{}", signal, source, target.stable_id(), raw_hash);
        let id = format!("ev_{}", &hex::encode(Sha256::digest(stable.as_bytes()))[..20]);
        Self {
            id,
            signal: signal.into(),
            source: source.into(),
            target: target.stable_id(),
            observed_at,
            observation_window: None,
            freshness_seconds: None,
            confidence: Confidence::Medium,
            observed: BTreeMap::new(),
            normalized: BTreeMap::new(),
            consumer: None,
            artifact,
            raw_hash,
            notes: vec![],
        }
    }
    pub fn age_seconds(&self, now: DateTime<Utc>) -> i64 {
        (now - self.observed_at).num_seconds().max(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Uncertainty {
    pub id: String,
    pub kind: String,
    pub message: String,
    pub severity: String,
    #[serde(default)]
    pub evidence_ids: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn evidence_id_is_content_stable() {
        let t: Target = "service:x".parse().unwrap();
        let now = Utc::now();
        let a = Evidence::build("static.reference", "scanner", &t, now, None, b"same");
        let b = Evidence::build("static.reference", "scanner", &t, now, None, b"same");
        assert_eq!(a.id, b.id);
    }
}
