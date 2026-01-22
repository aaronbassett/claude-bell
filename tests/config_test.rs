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
