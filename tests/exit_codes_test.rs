use assert_cmd::Command;

#[test]
fn test_success_exit_code() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    cmd.arg("--version").assert().code(0);
}

#[test]
fn test_user_error_exit_code() {
    let mut cmd = Command::cargo_bin("cb").unwrap();
    // Invalid subcommand should return exit code 2
    cmd.args(["invalid-subcommand"]).assert().code(2); // clap uses 2 for invalid args by default
}
