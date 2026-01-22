//! Tests for configuration schema and loading

#[test]
fn test_parse_valid_config() {
    let config_json = r#"{
        "version": 1,
        "defaults": {
            "sound": "Glass",
            "icon": "@claude",
            "json": false,
            "log_level": "warn",
            "persistent": false
        }
    }"#;

    let config: claude_bell::config::Config = serde_json::from_str(config_json).unwrap();
    assert_eq!(config.version, 1);
    assert_eq!(config.defaults.sound, Some("Glass".to_string()));
}

#[test]
fn test_config_defaults() {
    let config = claude_bell::config::Config::default();
    assert_eq!(config.version, 1);
}

#[test]
fn test_validate_valid_config() {
    let config = claude_bell::config::Config::default();
    assert!(claude_bell::config::validate_config(&config).is_ok());
}

#[test]
fn test_validate_invalid_log_level() {
    let config_json = r#"{
        "version": 1,
        "defaults": {
            "log_level": "invalid"
        }
    }"#;

    let config: claude_bell::config::Config = serde_json::from_str(config_json).unwrap();
    assert!(claude_bell::config::validate_config(&config).is_err());
}

#[test]
fn test_validate_invalid_version() {
    let config_json = r#"{
        "version": 99,
        "defaults": {
            "log_level": "info"
        }
    }"#;

    let config: claude_bell::config::Config = serde_json::from_str(config_json).unwrap();
    assert!(claude_bell::config::validate_config(&config).is_err());
}

#[test]
fn test_validate_valid_timeout() {
    let config_json = r#"{
        "version": 1,
        "defaults": {
            "log_level": "info"
        },
        "timeout": "30s"
    }"#;

    let config: claude_bell::config::Config = serde_json::from_str(config_json).unwrap();
    assert!(claude_bell::config::validate_config(&config).is_ok());
}

#[test]
fn test_validate_invalid_timeout_format() {
    let config_json = r#"{
        "version": 1,
        "defaults": {
            "log_level": "info"
        },
        "timeout": "invalid"
    }"#;

    let config: claude_bell::config::Config = serde_json::from_str(config_json).unwrap();
    assert!(claude_bell::config::validate_config(&config).is_err());
}

#[test]
fn test_validate_invalid_timeout_unit() {
    let config_json = r#"{
        "version": 1,
        "defaults": {
            "log_level": "info"
        },
        "timeout": "30x"
    }"#;

    let config: claude_bell::config::Config = serde_json::from_str(config_json).unwrap();
    assert!(claude_bell::config::validate_config(&config).is_err());
}
