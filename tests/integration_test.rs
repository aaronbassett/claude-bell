use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_full_workflow_template_and_notification() {
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Create template via JSON
    let create_json = r#"{"name": "test", "title": "Test Title", "subtitle": "Test Subtitle"}"#;
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("create").arg("--json")
        .write_stdin(create_json);
    cmd.assert().success();

    // List templates
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test"));

    // Show template
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("show").arg("test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Test Title"));
}

#[test]
fn test_config_workflow() {
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Set config value
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("config").arg("set")
        .arg("defaults.sound").arg("Ping");
    cmd.assert().success();

    // Show config
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("config").arg("show").arg("--pretty");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ping"));

    // Unset config value
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("config").arg("unset").arg("defaults.sound");
    cmd.assert().success();

    // Verify unset
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("config").arg("show").arg("--pretty");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ping").not());
}

#[test]
fn test_config_json_workflow() {
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Set config via JSON
    let json = r#"{"defaults": {"sound": "Basso", "icon": "@info"}}"#;
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("config").arg("set").arg("--json")
        .write_stdin(json);
    cmd.assert().success();

    // Verify both values set
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("config").arg("show").arg("--pretty");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Basso"))
        .stdout(predicate::str::contains("@info"));
}

#[test]
fn test_template_update_workflow() {
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Create template
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("create")
        .arg("--name").arg("test")
        .arg("--title").arg("Original Title");
    cmd.assert().success();

    // Update template with CLI flags
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("update")
        .arg("test")
        .arg("--title").arg("Updated Title");
    cmd.assert().success();

    // Verify update
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("show").arg("test");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Updated Title"));
}

#[test]
fn test_sound_prune_dry_run() {
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Prune dry-run on empty system
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("sound").arg("prune").arg("--dry-run");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No cleanup needed").or(predicate::str::contains("dry run")));
}

#[test]
fn test_icon_prune_dry_run() {
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Prune dry-run on empty system
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("icon").arg("prune").arg("--dry-run");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No cleanup needed").or(predicate::str::contains("dry run")));
}

#[test]
fn test_template_validate() {
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Create template
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("create")
        .arg("--name").arg("valid")
        .arg("--title").arg("Valid Template");
    cmd.assert().success();

    // Validate specific template
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("validate").arg("valid");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("valid"));

    // Validate all templates
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("validate").arg("--all");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}

#[test]
fn test_config_validate() {
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Validate default config
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("config").arg("validate");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("valid"));
}

#[test]
fn test_config_reset() {
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Set some config
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("config").arg("set")
        .arg("defaults.sound").arg("Ping");
    cmd.assert().success();

    // Reset config
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("config").arg("reset");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("reset"));

    // Verify reset
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("config").arg("show").arg("--pretty");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ping").not());
}

#[test]
fn test_template_delete() {
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Create template
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("create")
        .arg("--name").arg("todelete")
        .arg("--title").arg("Delete Me");
    cmd.assert().success();

    // Delete template
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("delete").arg("todelete");
    cmd.assert().success();

    // Verify deletion
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("template").arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("todelete").not());
}
