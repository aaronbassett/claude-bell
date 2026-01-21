use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("claude-bell"));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("macOS notifications"));
}
