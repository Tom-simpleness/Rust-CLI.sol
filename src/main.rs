use {
  anyhow::{Result, anyhow},
  mpl_token_metadata::accounts::Metadata,
  solana_sdk::pubkey::Pubkey,
  solana_client::rpc_client::RpcClient,
  spl_token::state::Mint,
  solana_sdk::program_pack::Pack,
};

fn main() -> Result<()> {
  // Initialize the RPC client with your Solana network URL
  let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
  let client = RpcClient::new(rpc_url);

  // The token address you want to fetch info for
  let token_address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".parse::<Pubkey>()?;

  // Find the metadata PDA
  let (metadata_address, _) = Metadata::find_pda(&token_address);

  // Fetch the metadata account
  let metadata_account = client.get_account(&metadata_address)?;

  // Deserialize the metadata
  let metadata = Metadata::from_bytes(&metadata_account.data)
      .map_err(|e| anyhow!("Failed to deserialize metadata: {}", e))?;

  // Fetch the mint account
  let mint_account = client.get_account(&token_address)?;

  // Deserialize the mint account
  let mint = Mint::unpack(&mint_account.data)
      .map_err(|e| anyhow!("Failed to deserialize mint account: {}", e))?;

  // Print out basic info
  println!("Token Name: {}", metadata.name);
  println!("Token Symbol: {}", metadata.symbol);
  println!("Token URI: {}", metadata.uri);
  println!("Token Supply: {}", mint.supply);
  println!("Token Decimals: {}", mint.decimals);

  // If you need more detailed info, you can access other fields of the metadata struct
  if let Some(creators) = metadata.creators {
      println!("Creators:");
      for creator in creators {
          println!("  Address: {}, Share: {}", creator.address, creator.share);
      }
  }

  println!("Seller Fee Basis Points: {}", metadata.seller_fee_basis_points);
  println!("Is Mutable: {}", metadata.is_mutable);

  if let Some(collection) = metadata.collection {
      println!("Collection: {}", collection.key);
      println!("Verified: {}", collection.verified);
  }

  Ok(())
}