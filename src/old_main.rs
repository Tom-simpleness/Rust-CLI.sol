mod solana_utils;
mod dns_lookup;

use clap::Parser;
use anyhow::{Result, Context};
use solana_utils::fetch_token_info;
use dns_lookup::lookup_dns_records;
use std::process;
use dotenv::dotenv;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    token_address: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().context("Failed to load .env file")?;

    let args = Args::parse();
    let cmc_api_key = env::var("CMC_API_KEY").context("CMC_API_KEY not set in .env file")?;

    println!("Fetching info for token: {}", args.token_address);

    match fetch_token_info(&args.token_address, &cmc_api_key).await {
        Ok(info) => {
            println!("Token Name: {} (Source: Jup API)", info.name);
            println!("Token Symbol: {} (Source: Jup API)", info.symbol);
            println!("Total Supply: {} (Source: On-chain Mint)", info.total_supply);
            if let Some(website) = info.website {
                println!("Website: {} (Source: Jup API)", website);
                println!("DNS lookup research ...");

                match lookup_dns_records(&website).await {
                    Ok((domain, dns_records)) => println!("DNS Records for {}: {} (Domain searched: {})", website, dns_records, domain),
                    Err(e) => eprintln!("Failed to perform DNS lookup: {:#}", e),
                }
            } else {
                println!("Website: Not available");
            }

            if let Some(price) = info.price {
                println!("Price (USD): ${:.2}", price);
                if let Some(price_change_24h) = info.price_change_24h {
                    println!("24h Change: {:.2}%", price_change_24h);
                }
            } else {
                println!("Price: Not available");
            }

            Ok(())
        },
        Err(e) => {
            eprintln!("Error: {:#}", e);
            process::exit(1);
        }
    }
}
