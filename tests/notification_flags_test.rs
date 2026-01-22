use assert_cmd::Command;
use predicates::prelude::*;

// Tests for notification flags after command dispatch is implemented.
// With title provided: Shows "Would send notification" message
// Without title: Shows "Use --help for usage information"

#[test]
fn test_title_flag_short() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["-t", "Test Title"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Would send notification"));
}

#[test]
fn test_title_flag_long() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--title", "Test Title"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Would send notification"));
}

#[test]
fn test_multiple_content_flags() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["-t", "Title", "-s", "Subtitle", "-m", "Message"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Would send notification"));
}

#[test]
fn test_flags_without_title_show_help() {
    // When no title and no subcommand, should show help message
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--sound", "Ping"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Use --help for usage information"));
}

#[test]
fn test_notification_with_all_content_flags() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args([
        "-t",
        "Title",
        "-s",
        "Subtitle",
        "-m",
        "Message",
        "--sound",
        "Ping",
        "--icon",
        "@terminal",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Would send notification"));
}

#[test]
fn test_short_flags_combined() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args([
        "-t", "Title", "-s", "Sub", "-m", "Msg", "-i", "/img.png", "-a", "OK,No", "-r", "Reply",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Would send notification"));
}

// Tests for subcommands
#[test]
fn test_doctor_command() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Config"));
}

#[test]
fn test_config_show_command() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("version"));
}

#[test]
fn test_sound_list_command() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["sound", "list"]).assert().success();
}

#[test]
fn test_icon_list_command() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["icon", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Bundled icons"));
}

#[test]
fn test_template_list_command() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["template", "list"]).assert().success();
}
