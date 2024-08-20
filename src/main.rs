use clap::Parser;
use anyhow::Result;

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
    
    // TODO: Implement token info fetching logic
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        let args = Args::parse_from(&["test", "--token-address", "TestAddress123"]);
        assert_eq!(args.token_address, "TestAddress123");
    }
}