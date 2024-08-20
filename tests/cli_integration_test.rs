use assert_cmd::Command;

#[test]
fn test_cli_integration() {
    let mut cmd = Command::cargo_bin("Rust_CLI_sol").unwrap();
    cmd.arg("--token-address")
        .arg("TestAddress123")
        .assert()
        .success()
        .stdout(predicates::str::contains("Fetching info for token: TestAddress123"));
}