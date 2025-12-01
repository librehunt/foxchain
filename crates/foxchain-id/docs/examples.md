# Usage Examples

This document provides comprehensive examples showing how to use `foxchain-id` and work with identification results.

## Basic Identification

### EVM Address (Multiple Candidates)

EVM addresses are valid across all EVM-compatible chains, so `identify()` returns multiple candidates:

```rust
use foxchain_id::{identify, Chain};

let result = identify("0xd8da6bf26964af9d7eed9e03e53415d37aa96045")?;

// Normalized address (EIP-55 checksummed)
println!("Normalized: {}", result.normalized);
// Output: Normalized: 0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045

// All candidate chains
for candidate in &result.candidates {
    println!("Chain: {:?}, Confidence: {:.2}, Reasoning: {}", 
             candidate.chain, 
             candidate.confidence, 
             candidate.reasoning);
}

// Example output:
// Chain: Ethereum, Confidence: 0.95, Reasoning: Valid EVM address with EIP-55 checksum
// Chain: Polygon, Confidence: 0.90, Reasoning: EVM-compatible chain (Polygon)
// Chain: BSC, Confidence: 0.90, Reasoning: EVM-compatible chain (BSC)
// Chain: Avalanche, Confidence: 0.90, Reasoning: EVM-compatible chain (Avalanche)
// ... (10 total candidates)
```

### Bitcoin Address (Single Candidate)

Bitcoin addresses are chain-specific, so they return a single candidate:

```rust
use foxchain_id::{identify, Chain};

let result = identify("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa")?;

println!("Normalized: {}", result.normalized);
// Output: Normalized: 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa

println!("Candidates: {}", result.candidates.len());
// Output: Candidates: 1

let candidate = &result.candidates[0];
println!("Chain: {:?}, Confidence: {:.2}, Reasoning: {}", 
         candidate.chain, 
         candidate.confidence, 
         candidate.reasoning);
// Output: Chain: Bitcoin, Confidence: 0.95, Reasoning: P2PKH address (version byte 0x00)
```

## Working with Results

### Filtering by Confidence

```rust
use foxchain_id::{identify, Chain};

let result = identify("0x742d35Cc6634C0532925a3b844Bc454e4438f44e")?;

// Get high confidence candidates (>= 0.9)
let high_confidence: Vec<_> = result.candidates
    .iter()
    .filter(|c| c.confidence >= 0.9)
    .collect();

for candidate in high_confidence {
    println!("High confidence: {:?} ({:.2})", candidate.chain, candidate.confidence);
}
```

### Finding Specific Chain

```rust
use foxchain_id::{identify, Chain};

let result = identify("0xd8da6bf26964af9d7eed9e03e53415d37aa96045")?;

// Find Ethereum candidate
if let Some(ethereum) = result.candidates.iter().find(|c| c.chain == Chain::Ethereum) {
    println!("Ethereum confidence: {:.2}", ethereum.confidence);
    println!("Reasoning: {}", ethereum.reasoning);
}
```

### Getting Highest Confidence Candidate

```rust
use foxchain_id::identify;

let result = identify("0x742d35Cc6634C0532925a3b844Bc454e4438f44e")?;

// Get candidate with highest confidence
let best_match = result.candidates
    .iter()
    .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap());

if let Some(best) = best_match {
    println!("Best match: {:?} with confidence {:.2}", best.chain, best.confidence);
    println!("Reasoning: {}", best.reasoning);
}
```

## Public Key Detection and Address Derivation

### Detecting Public Keys

```rust
use foxchain_id::{identify, Chain};

// Uncompressed secp256k1 public key (65 bytes)
let key_hex = "0x0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
let result = identify(key_hex)?;

println!("Normalized (derived address): {}", result.normalized);
// Output: Normalized (derived address): 0x... (EVM address)

// Candidates include both EVM and Bitcoin (since secp256k1 is used by both)
for candidate in &result.candidates {
    println!("Chain: {:?}, Confidence: {:.2}, Reasoning: {}", 
             candidate.chain, 
             candidate.confidence, 
             candidate.reasoning);
}

// Example output:
// Chain: Ethereum, Confidence: 0.85, Reasoning: EVM address derived from hex secp256k1 public key
// Chain: Bitcoin, Confidence: 0.80, Reasoning: Bitcoin address derived from hex secp256k1 public key
```

### Compressed Public Key Support

The library now supports compressed secp256k1 public keys (33 bytes):

```rust
use foxchain_id::{identify, Chain};

// Compressed secp256k1 public key (33 bytes with 0x02 prefix)
let compressed_key = "0x0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
let result = identify(compressed_key)?;

println!("Normalized (derived from compressed key): {}", result.normalized);
// The library automatically decompresses the key before deriving addresses

// Candidates are the same as with uncompressed keys
for candidate in &result.candidates {
    println!("Chain: {:?}, Confidence: {:.2}", candidate.chain, candidate.confidence);
}
```

### Ed25519 Public Key (Solana/Cosmos)

```rust
use foxchain_id::{identify, Chain};

// Ed25519 public key (32 bytes)
let ed25519_key = "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
let result = identify(ed25519_key)?;

println!("Normalized (derived address): {}", result.normalized);
// Output: Normalized (derived address): ... (Solana or Cosmos address)

// Candidates include Solana and Cosmos (both use Ed25519)
for candidate in &result.candidates {
    println!("Chain: {:?}, Confidence: {:.2}, Reasoning: {}", 
             candidate.chain, 
             candidate.confidence, 
             candidate.reasoning);
}

// Example output:
// Chain: Solana, Confidence: 0.85, Reasoning: Solana address derived from hex Ed25519 public key
// Chain: CosmosHub, Confidence: 0.80, Reasoning: Cosmos address derived from hex Ed25519 public key
```

## Multi-Chain Scenarios

### EVM Address on Multiple Chains

Since EVM addresses are valid across all EVM-compatible chains, you get multiple candidates:

```rust
use foxchain_id::{identify, Chain};

let result = identify("0x000000000000000000000000000000000000dEaD")?;

println!("Found {} candidate chains", result.candidates.len());
// Output: Found 10 candidate chains

// Group by confidence level
let high_conf: Vec<_> = result.candidates.iter()
    .filter(|c| c.confidence >= 0.9)
    .map(|c| c.chain)
    .collect();

println!("High confidence chains: {:?}", high_conf);
// Output: High confidence chains: [Ethereum, Polygon, BSC, ...]
```

### Chain-Specific Addresses

Some addresses are chain-specific and return a single candidate:

```rust
use foxchain_id::{identify, Chain};

// Bitcoin address
let btc_result = identify("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa")?;
assert_eq!(btc_result.candidates.len(), 1);
assert_eq!(btc_result.candidates[0].chain, Chain::Bitcoin);

// Solana address
let sol_result = identify("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM")?;
assert_eq!(sol_result.candidates.len(), 1);
assert_eq!(sol_result.candidates[0].chain, Chain::Solana);

// Cosmos address
let cosmos_result = identify("cosmos1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4")?;
assert_eq!(cosmos_result.candidates.len(), 1);
assert_eq!(cosmos_result.candidates[0].chain, Chain::CosmosHub);
```

## Error Handling

```rust
use foxchain_id::{identify, Error};

// Invalid address
match identify("invalid-address") {
    Ok(result) => {
        println!("Identified: {}", result.normalized);
    }
    Err(Error::InvalidInput(msg)) => {
        println!("Invalid input: {}", msg);
    }
    Err(Error::NotImplemented) => {
        println!("Feature not yet implemented");
    }
}
```

## Complete Example: Processing Multiple Addresses

```rust
use foxchain_id::{identify, Chain};

let addresses = vec![
    "0xd8da6bf26964af9d7eed9e03e53415d37aa96045", // EVM
    "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa",        // Bitcoin
    "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM", // Solana
];

for addr in addresses {
    match identify(addr) {
        Ok(result) => {
            println!("\nAddress: {}", addr);
            println!("Normalized: {}", result.normalized);
            println!("Candidates:");
            
            for candidate in &result.candidates {
                println!("  - {:?}: {:.2} ({})", 
                        candidate.chain, 
                        candidate.confidence, 
                        candidate.reasoning);
            }
        }
        Err(e) => {
            println!("Error identifying {}: {}", addr, e);
        }
    }
}
```

## Understanding Confidence Scores

Confidence scores range from 0.0 to 1.0:

- **0.90-1.0**: High confidence - Strong indicators (valid checksum, recognized format)
- **0.75-0.89**: Medium confidence - Valid format but less specific indicators
- **0.50-0.74**: Lower confidence - Possible match but ambiguous

### Example Confidence Scores

```rust
use foxchain_id::identify;

// EVM address with EIP-55 checksum
let result1 = identify("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")?;
// Ethereum: 0.95 (high - valid checksum)
// Other EVM chains: 0.90 (high - valid format)

// EVM address without checksum (lowercase)
let result2 = identify("0xd8da6bf26964af9d7eed9e03e53415d37aa96045")?;
// Ethereum: 0.85 (medium - valid but no checksum)
// Other EVM chains: 0.80 (medium - valid format)

// Bitcoin address
let result3 = identify("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa")?;
// Bitcoin: 0.95 (high - valid P2PKH address)

// Solana address (standard 32 bytes)
let result4 = identify("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM")?;
// Solana: 0.90 (high - standard length)
```

