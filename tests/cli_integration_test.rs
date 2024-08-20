use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_integration() {
    let mut cmd = Command::cargo_bin("Rust_CLI_sol").unwrap();
    cmd.arg("--token-address")
        .arg("11111111111111111111111111111111")  // Valid Solana address
        .assert()
        .success()
        .stdout(predicate::str::contains("Fetching info for token: 11111111111111111111111111111111"))
        .stdout(predicate::str::contains("Token Name: Token 11111111"))
        .stdout(predicate::str::contains("Token Symbol: TKN"))
        .stdout(predicate::str::contains("Total Supply: 1000000"))
        .stdout(predicate::str::contains("Website: https://example.com"));
}

#[test]
fn test_cli_invalid_address() {
    let mut cmd = Command::cargo_bin("Rust_CLI_sol").unwrap();
    cmd.arg("--token-address")
        .arg("InvalidAddress")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error: Invalid token address"));
}