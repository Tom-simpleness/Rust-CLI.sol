use anyhow::{Result, anyhow};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::Mint;
use solana_sdk::program_pack::Pack;
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub total_supply: String,
    pub website: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JupiterTokenInfo {
    symbol: String,
    name: String,
    #[serde(default)]
    website: Option<String>,
}

async fn fetch_jupiter_info(address: &str) -> Result<JupiterTokenInfo> {
    let url = "https://token.jup.ag/all";
    let response: serde_json::Value = reqwest::get(url).await?.json().await?;
    
    let tokens = response.as_array()
        .ok_or_else(|| anyhow!("Invalid response format"))?;
    
    let token_info = tokens.iter()
        .find(|token| token["address"] == address)
        .ok_or_else(|| anyhow!("Token not found"))?;
    
    println!("Debug: Raw token info: {:?}", token_info);  // Debug print

    let jupiter_info: JupiterTokenInfo = serde_json::from_value(token_info.clone())?;
    
    println!("Debug: Parsed Jupiter info: {:?}", jupiter_info);  // Debug print

    Ok(jupiter_info)
}

pub async fn fetch_token_info(address: &str) -> Result<TokenInfo> {
    let pubkey = address.parse::<Pubkey>().map_err(|e| anyhow!("Invalid token address: {}", e))?;
    
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let account = rpc_client.get_account(&pubkey)?;
    let mint = Mint::unpack(&account.data)?;

    let jupiter_info = fetch_jupiter_info(address).await?;

    Ok(TokenInfo {
        name: jupiter_info.name,
        symbol: jupiter_info.symbol,
        total_supply: mint.supply.to_string(),
        website: jupiter_info.website,
    })
}