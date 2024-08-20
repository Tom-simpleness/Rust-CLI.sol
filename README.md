# Solana Token CLI

A command-line interface tool for fetching information about Solana tokens.

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/solana-token-cli.git
   cd solana-token-cli
   ```

2. Build the project:
   ```
   cargo build
   ```

## Usage

To run the CLI tool:

```
cargo run -- --token-address <SOLANA_TOKEN_ADDRESS>
```

Replace `<SOLANA_TOKEN_ADDRESS>` with the address of the Solana token you want to query.

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
