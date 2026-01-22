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

#[test]
fn test_render_template_simple() {
    let template_json = r#"{
        "name": "test",
        "title": "Hello {{ name }}",
        "defaults": {}
    }"#;

    let template: claude_bell::template::Template = serde_json::from_str(template_json).unwrap();
    let mut vars = std::collections::HashMap::new();
    vars.insert("name".to_string(), "World".to_string());

    let rendered = claude_bell::template::render_template(&template, &vars).unwrap();
    assert_eq!(rendered.title, "Hello World");
}

#[test]
fn test_render_template_with_defaults() {
    let template_json = r#"{
        "name": "test",
        "title": "Status: {{ status }}",
        "defaults": {
            "status": "pending"
        }
    }"#;

    let template: claude_bell::template::Template = serde_json::from_str(template_json).unwrap();
    let vars = std::collections::HashMap::new();

    let rendered = claude_bell::template::render_template(&template, &vars).unwrap();
    assert_eq!(rendered.title, "Status: pending");
}

#[test]
fn test_render_template_with_filter() {
    let template_json = r#"{
        "name": "test",
        "title": "{{ name | upper }}",
        "defaults": {}
    }"#;

    let template: claude_bell::template::Template = serde_json::from_str(template_json).unwrap();
    let mut vars = std::collections::HashMap::new();
    vars.insert("name".to_string(), "hello".to_string());

    let rendered = claude_bell::template::render_template(&template, &vars).unwrap();
    assert_eq!(rendered.title, "HELLO");
}
