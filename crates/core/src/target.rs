use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt, str::FromStr};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Target {
    pub kind: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TargetError {
    #[error("target must use kind:id syntax")]
    MissingSeparator,
    #[error("target kind cannot be empty")]
    EmptyKind,
    #[error("target id cannot be empty")]
    EmptyId,
}

impl Target {
    pub fn stable_id(&self) -> String {
        format!("{}:{}", self.kind, self.id)
    }
    pub fn fingerprint(&self) -> String {
        let mut h = Sha256::new();
        h.update(self.stable_id().as_bytes());
        for (k, v) in &self.metadata {
            h.update(k.as_bytes());
            h.update([0]);
            h.update(v.as_bytes());
            h.update([0]);
        }
        format!("sha256:{}", hex::encode(h.finalize()))
    }
}

impl FromStr for Target {
    type Err = TargetError;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let (kind, id) = raw.split_once(':').ok_or(TargetError::MissingSeparator)?;
        if kind.trim().is_empty() {
            return Err(TargetError::EmptyKind);
        }
        if id.trim().is_empty() {
            return Err(TargetError::EmptyId);
        }
        Ok(Self {
            kind: kind.trim().to_ascii_lowercase(),
            id: id.trim().to_string(),
            metadata: BTreeMap::new(),
        })
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.stable_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_target_with_colons_in_id() {
        let t: Target = "api:POST:/v1/export".parse().unwrap();
        assert_eq!(t.kind, "api");
        assert_eq!(t.id, "POST:/v1/export");
    }
    #[test]
    fn fingerprint_is_stable() {
        let a: Target = "service:legacy-export".parse().unwrap();
        assert_eq!(a.fingerprint(), a.fingerprint());
    }
}
