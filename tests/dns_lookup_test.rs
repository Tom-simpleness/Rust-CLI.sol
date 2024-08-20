use assert_cmd::Command;
use predicates::prelude::*;

#[tokio::test]
async fn test_dns_lookup() {
    let mut cmd = Command::cargo_bin("rust_cli_sol").unwrap();
    cmd.arg("--token-address")
        .arg("So11111111111111111111111111111111111111112")  // Wrapped SOL token address
        .assert()
        .success()
        .stdout(predicate::str::contains("DNS Records for"))
        .stdout(predicate::str::contains("Domain searched:"));
}