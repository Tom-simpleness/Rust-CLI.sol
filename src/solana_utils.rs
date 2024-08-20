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
    
    let jupiter_info: JupiterTokenInfo = serde_json::from_value(token_info.clone())?;
    Ok(jupiter_info)
}

async fn fetch_website_from_coinmarketcap(symbol: &str, api_key: &str) -> Result<Option<String>> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("X-CMC_PRO_API_KEY", reqwest::header::HeaderValue::from_str(api_key)?);
    headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));
    headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("rust-reqwest"));

    let url = format!("https://pro-api.coinmarketcap.com/v1/cryptocurrency/info?symbol={}", symbol);
    
    let response: serde_json::Value = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;
    
    if let Some(crypto_info) = response["data"][symbol.to_uppercase()].as_object() {
        if let Some(websites) = crypto_info["urls"]["website"].as_array() {
            if let Some(website) = websites.get(0) {
                return Ok(Some(website.as_str().unwrap_or_default().to_string()));
            }
        }
    }

    Ok(None)
}

pub async fn fetch_token_info(address: &str, cmc_api_key: &str) -> Result<TokenInfo> {
    let pubkey = address.parse::<Pubkey>().map_err(|e| anyhow!("Invalid token address: {}", e))?;
    
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let account = rpc_client.get_account(&pubkey)?;
    let mint = Mint::unpack(&account.data)?;

    let jupiter_info = fetch_jupiter_info(address).await?;

    let website = if let Some(jup_website) = &jupiter_info.website {
        Some(jup_website.clone())
    } else {
        fetch_website_from_coinmarketcap(&jupiter_info.symbol, cmc_api_key).await?
    };

    Ok(TokenInfo {
        name: jupiter_info.name,
        symbol: jupiter_info.symbol,
        total_supply: mint.supply.to_string(),
        website,
    })
}