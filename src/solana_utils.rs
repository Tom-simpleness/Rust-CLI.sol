use anyhow::{Result, anyhow};
use solana_sdk::pubkey::Pubkey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub website: Option<String>,
}

pub async fn fetch_token_info(address: &str) -> Result<TokenInfo> {
    let pubkey = address.parse::<Pubkey>().map_err(|e| anyhow!("Invalid token address: {}", e))?;
    
    // For now, we'll still return dummy data, but only if the pubkey is valid
    Ok(TokenInfo {
        name: format!("Token {}", pubkey.to_string()),
        symbol: "TKN".to_string(),
        total_supply: 1_000_000,
        website: Some("https://example.com".to_string()),
    })
}