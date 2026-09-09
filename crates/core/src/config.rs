use serde::{Deserialize, Serialize};
use std::{fs, io::ErrorKind, path::Path};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub project: Project,
    #[serde(default)]
    pub policy: PolicyConfig,
    #[serde(default)]
    pub privacy: PrivacyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    #[serde(default = "default_project")]
    pub name: String,
}

fn default_project() -> String {
    "decomproof-project".into()
}

impl Default for Project {
    fn default() -> Self {
        Self {
            name: default_project(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyConfig {
    #[serde(default = "default_runtime_days")]
    pub runtime_minimum_window_days: u32,
    #[serde(default = "default_external_days")]
    pub external_minimum_window_days: u32,
    #[serde(default = "default_age_hours")]
    pub evidence_maximum_age_hours: u32,
    #[serde(default = "default_cycles")]
    pub scheduled_minimum_cycles: u32,
    #[serde(default = "yes")]
    pub unidentified_consumers_block: bool,
    #[serde(default = "yes")]
    pub public_api_requires_deprecation: bool,
    #[serde(default = "default_sunset_days")]
    pub api_minimum_sunset_days: u32,
    #[serde(default = "default_data_days")]
    pub data_zero_writes_days: u32,
}

fn default_runtime_days() -> u32 {
    30
}
fn default_external_days() -> u32 {
    45
}
fn default_age_hours() -> u32 {
    24
}
fn default_cycles() -> u32 {
    3
}
fn default_sunset_days() -> u32 {
    30
}
fn default_data_days() -> u32 {
    30
}
fn yes() -> bool {
    true
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            runtime_minimum_window_days: default_runtime_days(),
            external_minimum_window_days: default_external_days(),
            evidence_maximum_age_hours: default_age_hours(),
            scheduled_minimum_cycles: default_cycles(),
            unidentified_consumers_block: true,
            public_api_requires_deprecation: true,
            api_minimum_sunset_days: default_sunset_days(),
            data_zero_writes_days: default_data_days(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrivacyConfig {
    #[serde(default)]
    pub redact_regex: Vec<String>,
    #[serde(default = "yes")]
    pub hash_consumer_ids: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            redact_regex: Vec::new(),
            hash_consumer_ids: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: Project::default(),
            policy: PolicyConfig::default(),
            privacy: PrivacyConfig::default(),
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("read config: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse config: {0}")]
    Parse(#[from] serde_yaml::Error),
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        Ok(serde_yaml::from_str(&fs::read_to_string(path)?)?)
    }

    /// Load configuration when present. A genuinely absent file uses documented
    /// conservative defaults; malformed, unreadable, or unknown configuration is
    /// always an error and must never silently change verdict policy.
    pub fn load_or_default_if_missing(path: &Path) -> Result<Self, ConfigError> {
        match fs::read_to_string(path) {
            Ok(raw) => Ok(serde_yaml::from_str(&raw)?),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(ConfigError::Io(error)),
        }
    }
}
