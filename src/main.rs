mod solana_utils;
mod dns_lookup;

use clap::Parser;
use anyhow::Result;
use solana_utils::fetch_token_info;
use dns_lookup::lookup_dns_records;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    token_address: String,
    #[arg(short, long)]
    cmc_api_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Fetching info for token: {}", args.token_address);
    
    match fetch_token_info(&args.token_address, &args.cmc_api_key).await {
        Ok(info) => {
            println!("Token Name: {} (Source: Jupiter API)", info.name);
            println!("Token Symbol: {} (Source: Jupiter API)", info.symbol);
            println!("Total Supply: {} (Source: On-chain Mint)", info.total_supply);
            if let Some(website) = info.website {
                println!("Website: {} (Source: CoinMarketCap API)", website);
                println!("DNS lookup research ...");

                // Perform DNS lookup
                match lookup_dns_records(&website).await {
                    Ok((domain, dns_records)) => println!("DNS Records for {}: {} (Domain searched: {})", website, dns_records, domain),
                    Err(e) => println!("Failed to perform DNS lookup: {}", e),
                }
            } else {
                println!("Website: Not available");
            }
            Ok(())
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
