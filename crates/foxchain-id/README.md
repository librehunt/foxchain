# foxchain-id

Multi-chain blockchain address identification library for Rust.

## Overview

`foxchain-id` provides functionality to identify which blockchain(s) an input string (address, public key, or private key) belongs to. It supports multiple blockchain address formats and returns normalized addresses with confidence scores for candidate chains.

## Features

- **Multi-chain support**: Identify addresses across multiple blockchain networks
- **Address normalization**: Convert addresses to their canonical format
- **Confidence scoring**: Get confidence scores for each candidate chain
- **Format detection**: Automatically detect address format (EVM, Bitcoin, Solana, etc.)
- **EIP-55 checksum validation**: Validate and normalize EVM addresses according to EIP-55

## Quick Start

```rust
use foxchain_id::identify;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Identify an EVM address
    let result = identify("0xd8da6bf26964af9d7eed9e03e53415d37aa96045")?;
    
    println!("Normalized address: {}", result.normalized);
    for candidate in result.candidates {
        println!("Chain: {:?}, Confidence: {:.2}", candidate.chain, candidate.confidence);
        println!("Reasoning: {}", candidate.reasoning);
    }
    
    Ok(())
}
```

## Supported Formats

### Currently Implemented

- **EVM Addresses** (Ethereum, Polygon, BSC, Avalanche, Arbitrum, Optimism, Base, Fantom, Celo, Gnosis)
  - Format: `0x` followed by 40 hex characters
  - EIP-55 checksum validation and normalization
  - See [EVM Addresses Documentation](docs/evm-addresses.md) for details

### Planned

- Bitcoin ecosystem (P2PKH, P2SH, Bech32)
- Solana addresses
- Cosmos ecosystem addresses
- Substrate/Polkadot addresses (SS58)
- Tron addresses
- And more...

See [Format Documentation](docs/) for detailed information about each format.

## Usage

### Basic Identification

```rust
use foxchain_id::identify;

let result = identify("0x742d35Cc6634C0532925a3b844Bc454e4438f44e")?;

// Get normalized address
let normalized = result.normalized;

// Get all candidate chains
for candidate in result.candidates {
    if candidate.confidence > 0.9 {
        println!("High confidence match: {:?}", candidate.chain);
    }
}
```

### Working with Results

```rust
use foxchain_id::{identify, Chain};

let result = identify("0xd8da6bf26964af9d7eed9e03e53415d37aa96045")?;

// Find specific chain
if let Some(ethereum) = result.candidates.iter().find(|c| c.chain == Chain::Ethereum) {
    println!("Ethereum confidence: {}", ethereum.confidence);
}

// Get highest confidence candidate
let best_match = result.candidates.iter()
    .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());
```

## Documentation

- [Format Documentation](docs/) - Detailed documentation for each address format
- [API Documentation](https://docs.rs/foxchain-id) - Full API reference (when published)

## Contributing

Contributions are welcome! Please see the main project repository for contribution guidelines.

## License

This project is licensed under the GPL-3.0 license.

