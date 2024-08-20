mod solana_utils;

use clap::Parser;
use anyhow::Result;
use solana_utils::fetch_token_info;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    token_address: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Fetching info for token: {}", args.token_address);
    
    match fetch_token_info(&args.token_address).await {
        Ok(info) => {
            println!("Token Name: {} (Source: Jup API)", info.name);
            println!("Token Symbol: {} (Source: Jup API)", info.symbol);
            println!("Total Supply: {} (Source: On-chain Mint)", info.total_supply);
            if let Some(website) = info.website {
                println!("Website: {} (Source: Jup API)", website);
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