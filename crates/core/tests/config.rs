use decomproof_core::config::{Config, ConfigError};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

fn temp_path(name: &str) -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).expect("clock").as_nanos();
    std::env::temp_dir().join(format!("decomproof-{name}-{unique}.yml"))
}

#[test]
fn absent_config_uses_conservative_defaults() {
    let path = temp_path("missing");
    let config = Config::load_or_default_if_missing(&path).expect("missing config should use defaults");

    assert_eq!(config.project.name, "decomproof-project");
    assert_eq!(config.policy.runtime_minimum_window_days, 30);
    assert_eq!(config.policy.external_minimum_window_days, 45);
    assert_eq!(config.policy.scheduled_minimum_cycles, 3);
    assert!(config.policy.unidentified_consumers_block);
    assert!(config.policy.public_api_requires_deprecation);
    assert!(config.privacy.hash_consumer_ids);
}

#[test]
fn malformed_config_is_rejected_instead_of_defaulted() {
    let path = temp_path("malformed");
    fs::write(&path, "policy: [this is: not valid").expect("write malformed config");

    let error = Config::load_or_default_if_missing(&path).expect_err("malformed config must fail");
    assert!(matches!(error, ConfigError::Parse(_)));
    let _ = fs::remove_file(path);
}

#[test]
fn unknown_policy_keys_are_rejected() {
    let path = temp_path("unknown-key");
    fs::write(
        &path,
        "project:\n  name: atlas\npolicy:\n  runtime_minimum_window_days: 30\n  pretend_missing_evidence_is_clear: true\n",
    )
    .expect("write config");

    let error = Config::load_or_default_if_missing(&path).expect_err("unknown key must fail");
    assert!(matches!(error, ConfigError::Parse(_)));
    let _ = fs::remove_file(path);
}

#[test]
fn partial_config_keeps_safe_defaults_for_omitted_fields() {
    let path = temp_path("partial");
    fs::write(&path, "project:\n  name: atlas\npolicy:\n  runtime_minimum_window_days: 60\n")
        .expect("write config");

    let config = Config::load_or_default_if_missing(&path).expect("valid partial config");
    assert_eq!(config.project.name, "atlas");
    assert_eq!(config.policy.runtime_minimum_window_days, 60);
    assert_eq!(config.policy.external_minimum_window_days, 45);
    assert!(config.policy.unidentified_consumers_block);
    assert!(config.privacy.hash_consumer_ids);
    let _ = fs::remove_file(path);
}
