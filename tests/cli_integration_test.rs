use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_integration() {
    let mut cmd = Command::cargo_bin("Rust_CLI_sol").unwrap();
    cmd.arg("--token-address")
        .arg("So11111111111111111111111111111111111111112")  // Wrapped SOL token address
        .assert()
        .success()
        .stdout(predicate::str::contains("Fetching info for token: So11111111111111111111111111111111111111112"))
        .stdout(predicate::str::contains("Token Name: Wrapped SOL (Source: Jup API)"))
        .stdout(predicate::str::contains("Token Symbol: SOL (Source: Jup API)"))
        .stdout(predicate::str::contains("Total Supply:").and(predicate::str::contains("(Source: On-chain Mint)")))
        .stdout(predicate::str::contains("Website:").and(predicate::str::contains("(Source: Jup API)").or(predicate::str::contains("Not available"))));
}

#[test]
fn test_cli_invalid_address() {
    let mut cmd = Command::cargo_bin("Rust_CLI_sol").unwrap();
    cmd.arg("--token-address")
        .arg("InvalidAddress")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error: Invalid token address: Invalid Base58 string"));
}