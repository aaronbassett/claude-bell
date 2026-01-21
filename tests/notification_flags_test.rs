use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_title_flag_short() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["-t", "Test Title"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Title"));
}

#[test]
fn test_title_flag_long() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--title", "Test Title"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Title"));
}

#[test]
fn test_multiple_content_flags() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["-t", "Title", "-s", "Subtitle", "-m", "Message"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Title"))
        .stdout(predicate::str::contains("Subtitle"))
        .stdout(predicate::str::contains("Message"));
}

#[test]
fn test_image_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--image", "/path/to/image.png"])
        .assert()
        .success()
        .stdout(predicate::str::contains("/path/to/image.png"));
}

#[test]
fn test_icon_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--icon", "@terminal"])
        .assert()
        .success()
        .stdout(predicate::str::contains("@terminal"));
}

#[test]
fn test_sound_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--sound", "Ping"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Ping"));
}

#[test]
fn test_actions_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--actions", "OK,Cancel"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OK"))
        .stdout(predicate::str::contains("Cancel"));
}

#[test]
fn test_reply_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--reply", "Enter response"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Enter response"));
}

#[test]
fn test_url_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--url", "https://example.com"])
        .assert()
        .success()
        .stdout(predicate::str::contains("https://example.com"));
}

#[test]
fn test_persistent_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--persistent", "-t", "Persistent"])
        .assert()
        .success()
        .stdout(predicate::str::contains("persistent: true"));
}

#[test]
fn test_not_persistent_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--not-persistent"])
        .assert()
        .success()
        .stdout(predicate::str::contains("not_persistent: true"));
}

#[test]
fn test_timeout_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--timeout", "30s"])
        .assert()
        .success()
        .stdout(predicate::str::contains("30s"));
}

#[test]
fn test_batch_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--batch"])
        .assert()
        .success()
        .stdout(predicate::str::contains("batch: true"));
}

#[test]
fn test_json_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--json", "stdout"])
        .assert()
        .success()
        .stdout(predicate::str::contains("stdout"));
}

#[test]
fn test_pretty_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--pretty"])
        .assert()
        .success()
        .stdout(predicate::str::contains("pretty: true"));
}

#[test]
fn test_quiet_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--quiet"])
        .assert()
        .success()
        .stdout(predicate::str::contains("quiet: true"));
}

#[test]
fn test_silent_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--silent"])
        .assert()
        .success()
        .stdout(predicate::str::contains("silent: true"));
}

#[test]
fn test_log_level_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--log-level", "debug"])
        .assert()
        .success()
        .stdout(predicate::str::contains("debug"));
}

#[test]
fn test_template_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--template", "success"])
        .assert()
        .success()
        .stdout(predicate::str::contains("success"));
}

#[test]
fn test_var_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--var", "key:value"])
        .assert()
        .success()
        .stdout(predicate::str::contains("key:value"));
}

#[test]
fn test_default_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--default", "dismissed"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dismissed"));
}

#[test]
fn test_on_dismiss_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--on-dismiss", "cancelled"])
        .assert()
        .success()
        .stdout(predicate::str::contains("cancelled"));
}

#[test]
fn test_on_timeout_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["--on-timeout", "expired"])
        .assert()
        .success()
        .stdout(predicate::str::contains("expired"));
}

#[test]
fn test_short_flags_combined() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.args(["-t", "Title", "-s", "Sub", "-m", "Msg", "-i", "/img.png", "-a", "OK,No", "-r", "Reply"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Title"))
        .stdout(predicate::str::contains("Sub"))
        .stdout(predicate::str::contains("Msg"));
}
