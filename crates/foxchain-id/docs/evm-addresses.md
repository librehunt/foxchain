# EVM Address Format

EVM (Ethereum Virtual Machine) addresses are used across many blockchain networks including Ethereum, Polygon, BSC, Avalanche, Arbitrum, Optimism, Base, Fantom, Celo, Gnosis, and many others.

## Format Specification

EVM addresses follow a simple but important format:

- **Prefix**: `0x` (hexadecimal indicator)
- **Length**: Exactly 40 hexadecimal characters (20 bytes)
- **Character set**: `0-9`, `a-f`, `A-F`
- **Total length**: 42 characters (including `0x` prefix)

### Example Addresses

```
0xd8da6bf26964af9d7eed9e03e53415d37aa96045  (lowercase)
0xD8DA6BF26964AF9D7EED9E03E53415D37AA96045  (uppercase)
0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045  (EIP-55 checksummed)
```

## EIP-55 Checksum

EIP-55 (Ethereum Improvement Proposal 55) introduces a checksum mechanism for Ethereum addresses to prevent errors when manually entering addresses.

### How It Works

1. **Hash the lowercase address**: Compute Keccak-256 hash of the lowercase address string (including `0x` prefix)
2. **For each character**: If the character is a letter (a-f), check the corresponding nibble in the hash
3. **Uppercase if needed**: If the nibble is >= 8, uppercase the character; otherwise keep it lowercase

### Validation

A valid EIP-55 checksummed address:
- Must have mixed case (not all lowercase or all uppercase)
- Each alphabetic character's case must match the checksum algorithm
- The checksum is case-sensitive and must be exact

### Benefits

- **Error detection**: Typos in addresses are more likely to be detected
- **Visual verification**: Users can verify addresses by checking the pattern
- **Standardization**: Provides a canonical representation

## Encoding/Decoding

### Encoding

EVM addresses are derived from public keys:

1. Take the public key (65 bytes uncompressed or 33 bytes compressed)
2. Compute Keccak-256 hash of the public key
3. Take the last 20 bytes (rightmost 20 bytes)
4. Convert to hexadecimal string
5. Prepend `0x`
6. Optionally apply EIP-55 checksum

### Decoding

1. Remove `0x` prefix
2. Validate hex characters
3. Decode hex string to 20 bytes
4. Validate length is exactly 20 bytes

## Validation Rules

An address is valid if:

1. ✅ Starts with `0x`
2. ✅ Has exactly 40 hex characters after `0x`
3. ✅ All characters are valid hexadecimal (0-9, a-f, A-F)
4. ✅ Decodes to exactly 20 bytes
5. ✅ (Optional) EIP-55 checksum is valid if mixed case

## Normalization

`foxchain-id` normalizes EVM addresses to EIP-55 checksum format:

- **Input**: Any valid EVM address (lowercase, uppercase, or checksummed)
- **Output**: EIP-55 checksummed address (canonical format)
- **Process**: Compute checksum from lowercase version and apply case

### Example

```rust
use foxchain_id::identify;

// Input: lowercase address
let result = identify("0xd8da6bf26964af9d7eed9e03e53415d37aa96045")?;

// Output: EIP-55 checksummed
assert_eq!(result.normalized, "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
```

## Multi-chain Considerations

EVM addresses are **valid across all EVM-compatible chains**. This means:

- The same address format works on Ethereum, Polygon, BSC, etc.
- An address like `0x742d35Cc6634C0532925a3b844Bc454e4438f44e` could belong to any EVM chain
- `foxchain-id` returns **all EVM chains as candidates** with confidence scores

### Confidence Scoring

- **With valid EIP-55 checksum**: 0.95 confidence (high confidence, properly formatted)
- **Without checksum (lowercase)**: 0.85 confidence (valid format, but not checksummed)

The confidence difference reflects that checksummed addresses are more likely to be intentionally formatted and less likely to be typos.

## Implementation Details

### Detection Algorithm

1. Check if input starts with `0x` and has length 42
2. Validate all characters are hexadecimal
3. Decode and verify length is 20 bytes
4. Validate EIP-55 checksum if mixed case
5. Normalize to EIP-55 format
6. Generate candidates for all EVM chains

### Supported Chains

Currently, `foxchain-id` returns candidates for:

- Ethereum (highest confidence)
- Polygon
- BSC (Binance Smart Chain)
- Avalanche C-Chain
- Arbitrum
- Optimism
- Base
- Fantom
- Celo
- Gnosis

## Examples

### Valid Addresses

```rust
use foxchain_id::identify;

// Lowercase (will be normalized)
let result = identify("0xd8da6bf26964af9d7eed9e03e53415d37aa96045")?;

// Already checksummed
let result = identify("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")?;

// All uppercase (will be normalized)
let result = identify("0xD8DA6BF26964AF9D7EED9E03E53415D37AA96045")?;
```

### Invalid Addresses

```rust
// Too short
identify("0x123"); // Error: InvalidInput

// Invalid hex character
identify("0x742d35Cc6634C0532925a3b844Bc454e4438f44g"); // Error: InvalidInput

// Wrong length
identify("0x742d35Cc6634C0532925a3b844Bc454e4438f44"); // Error: InvalidInput
```

## Technical References

- [EIP-55: Mixed-case checksum address encoding](https://eips.ethereum.org/EIPS/eip-55)
- [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)
- [Keccak-256 Hash Function](https://keccak.team/keccak.html)

## See Also

- [Bitcoin Addresses](bitcoin-addresses.md) - Different format for Bitcoin ecosystem
- [Solana Addresses](solana-addresses.md) - Base58 encoding used by Solana
- [Cosmos Addresses](cosmos-addresses.md) - Bech32 encoding with HRP

