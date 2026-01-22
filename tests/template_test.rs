//! Template schema tests

#[test]
fn test_parse_valid_template() {
    let template_json = r#"{
        "name": "build-result",
        "description": "Notify when build finishes",
        "title": "Build {{ status | title }}",
        "message": "Completed",
        "sound": "@success",
        "defaults": {
            "status": "complete"
        }
    }"#;

    let template: claude_bell::template::Template = serde_json::from_str(template_json).unwrap();
    assert_eq!(template.name, "build-result");
    assert_eq!(template.title, "Build {{ status | title }}");
}
