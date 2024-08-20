# Solana Token CLI

A command-line interface tool for fetching information about Solana tokens.

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/Tom-simpleness/Rust-CLI.sol.git
   cd Rust-CLI.sol
   ```

2. Create a `.env` file in the project root and add your CoinMarketCap API key:

   ```
   CMC_API_KEY=your_api_key_here
   ```

3. Build the project:
   ```
   cargo build
   ```

## Usage

To run the CLI tool:

```
cargo run -- --token-address <SOLANA_TOKEN_ADDRESS>
```

Replace `<SOLANA_TOKEN_ADDRESS>` with the address of the Solana token you want to query.

### Examples

For USDC:

```
cargo run -- --token-address EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
```

For Wrapped SOL:

```
cargo run -- --token-address So11111111111111111111111111111111111111112
```

## Testing

To run all tests (unit and integration):

```
cargo test
```

## Development

This project uses:

- `clap` for CLI argument parsing
- `tokio` for async runtime
- `anyhow` for error handling
- `reqwest` for making API requests to Jupiter
- `serde` for JSON parsing and serialization

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
