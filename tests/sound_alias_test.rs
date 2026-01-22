//! Sound alias tests

use std::collections::HashMap;

#[test]
fn test_parse_sound_aliases() {
    let aliases_json = r#"{
        "bark": "/path/to/bark.aiff",
        "ding": "files/abc123.aiff"
    }"#;

    let aliases: HashMap<String, String> = serde_json::from_str(aliases_json).unwrap();
    assert_eq!(aliases.get("bark"), Some(&"/path/to/bark.aiff".to_string()));
}
