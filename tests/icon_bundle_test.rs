use assert_cmd::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_add_icon_alias_with_icns_file() {
    let temp = TempDir::new().unwrap();
    let icon_path = temp.path().join("test.icns");
    fs::write(&icon_path, b"fake icns").unwrap();

    // Set config dir to temp
    std::env::set_var("HOME", temp.path());

    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("icon")
        .arg("add")
        .arg("myicon")
        .arg(icon_path.to_str().unwrap());

    cmd.assert().success();

    // Verify bundle was created
    let bundle_path = temp.path().join(".claude-bell/icons/bundles/myicon.app");
    assert!(bundle_path.exists());
    assert!(bundle_path.join("Contents/Info.plist").exists());
    assert!(bundle_path.join("Contents/Resources/icon.icns").exists());
}

#[test]
fn test_add_icon_alias_with_image_file() {
    let temp = TempDir::new().unwrap();
    let icon_path = temp.path().join("test.png");
    fs::write(&icon_path, b"fake png").unwrap();

    std::env::set_var("HOME", temp.path());

    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("icon")
        .arg("add")
        .arg("myicon")
        .arg(icon_path.to_str().unwrap());

    cmd.assert().success();

    // Verify bundle was created with converted ICNS
    let bundle_path = temp.path().join(".claude-bell/icons/bundles/myicon.app");
    assert!(bundle_path.exists());
    assert!(bundle_path.join("Contents/Resources/icon.icns").exists());
}

#[test]
fn test_add_icon_alias_with_app_bundle() {
    let temp = TempDir::new().unwrap();

    // Create a fake .app bundle
    let source_app = temp.path().join("Source.app");
    let contents = source_app.join("Contents");
    fs::create_dir_all(&contents).unwrap();
    fs::write(contents.join("Info.plist"), b"<plist></plist>").unwrap();

    std::env::set_var("HOME", temp.path());

    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("icon")
        .arg("add")
        .arg("myicon")
        .arg(source_app.to_str().unwrap());

    cmd.assert().success();

    // Verify reference was stored
    let bundle_path = temp.path().join(".claude-bell/icons/bundles/myicon.app");
    assert!(bundle_path.exists());
}

#[test]
fn test_resolve_icon_with_generated_bundle() {
    // Test that resolve_icon returns the full path to generated bundle
    use claude_bell::alias::resolve_icon;

    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Create bundle structure
    let bundle_path = temp.path().join(".claude-bell/icons/bundles/test.app");
    fs::create_dir_all(&bundle_path).unwrap();

    // Add alias
    let aliases_json = r#"{"test": "test.app"}"#;
    let aliases_path = temp.path().join(".claude-bell/icons/aliases.json");
    fs::create_dir_all(aliases_path.parent().unwrap()).unwrap();
    fs::write(&aliases_path, aliases_json).unwrap();

    let resolved = resolve_icon("@test").unwrap();
    assert!(resolved.is_some());
    assert!(resolved.unwrap().ends_with("test.app"));
}
