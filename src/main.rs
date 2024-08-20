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
            println!("Token Name: {}", info.name);
            println!("Token Symbol: {}", info.symbol);
            println!("Total Supply: {}", info.total_supply);
            if let Some(website) = info.website {
                println!("Website: {}", website);
            }
            Ok(())
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}