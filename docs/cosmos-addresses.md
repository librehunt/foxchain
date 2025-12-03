# Cosmos Ecosystem Address Format

The Cosmos ecosystem uses Bech32 encoding with Human Readable Part (HRP) prefixes to identify different chains. All Cosmos addresses use the same encoding scheme, only the HRP differs.

## Format Specification

- **Encoding**: Bech32
- **Structure**: `{hrp}1{data}`
- **HRP**: Human-readable part identifying the chain (e.g., `cosmos`, `osmo`, `juno`)
- **Separator**: `1` (always present)
- **Data**: Base32-encoded address data
- **Checksum**: Built into Bech32 encoding

### Example Addresses

```
cosmos1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4    (Cosmos Hub)
osmo1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4      (Osmosis)
juno1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4       (Juno)
```

## Bech32 Encoding

Bech32 is a checksummed base32 encoding format designed for Bitcoin SegWit addresses and adopted by Cosmos.

### Character Set

```
qpzry9x8gf2tvdw0s3jn54khce6mua7l
```

### Encoding Process

1. Convert 8-bit bytes to 5-bit groups
2. Append checksum (6 characters computed from HRP + data)
3. Encode using Bech32 character set
4. Format as `{hrp}1{encoded_data}`

### Checksum

The Bech32 checksum:
- Is computed from both HRP and data
- Prevents typos and transcription errors
- Is case-insensitive (addresses can be written in any case)

## HRP (Human Readable Part)

The HRP identifies which Cosmos chain the address belongs to:

| Chain | HRP | Example |
|-------|-----|---------|
| Cosmos Hub | `cosmos` | `cosmos1...` |
| Osmosis | `osmo` | `osmo1...` |
| Juno | `juno` | `juno1...` |
| Akash | `akash` | `akash1...` |
| Stargaze | `stars` | `stars1...` |
| Secret Network | `secret` | `secret1...` |
| Terra Classic | `terra` | `terra1...` |

### HRP Variations

Some chains use variations:
- Cosmos Hub: `cosmos` (mainnet), `cosmosvaloper` (validators)
- Some chains may use different HRPs for different address types

## Address Derivation

Cosmos addresses are derived from public keys:

1. Take the public key (typically Ed25519 or secp256k1)
2. Compute SHA256 hash
3. Take first 20 bytes (RIPEMD160 can also be used)
4. Encode in Bech32 with chain-specific HRP

## Validation Rules

An address is valid if:

1. ✅ Starts with known HRP (e.g., `cosmos`, `osmo`)
2. ✅ Has `1` separator after HRP
3. ✅ Valid Bech32 encoding
4. ✅ Valid checksum
5. ✅ Correct data length (typically 20 bytes when decoded)

## Normalization

`foxchain-id` normalizes Cosmos addresses:

- **Input**: Any valid Bech32 Cosmos address (any case)
- **Output**: Lowercase Bech32 address (canonical format)
- **Process**: Extract HRP, validate, normalize case

### Case Handling

Bech32 is case-insensitive, but the standard is lowercase:
- `COSMOS1...` → `cosmos1...`
- `Cosmos1...` → `cosmos1...`
- `cosmos1...` → `cosmos1...` (already normalized)

## Multi-chain Considerations

The Cosmos ecosystem has **many chains** using the same address format:

- Same encoding scheme (Bech32)
- Same validation rules
- Only HRP differs

### Chain Identification

`foxchain-id` identifies the chain from the HRP:

1. Extract HRP from address
2. Match HRP to known chain
3. Return appropriate `Chain` variant
4. If HRP unknown, return generic `Chain::Cosmos` with note

### Supported Chains

Planned support for major Cosmos chains:
- Cosmos Hub
- Osmosis
- Juno
- Akash
- Stargaze
- Secret Network
- And more...

## Implementation Details

### Detection Algorithm

1. Check if address matches Bech32 format
2. Extract HRP (characters before `1`)
3. Validate Bech32 encoding and checksum
4. Match HRP to known chain
5. Return chain-specific candidate

### HRP Mapping

The library maintains a mapping of HRPs to chains:

```rust
// Example mapping
"cosmos" => Chain::CosmosHub
"osmo" => Chain::Osmosis
"juno" => Chain::Juno
// etc.
```

### Extensibility

The HRP mapping is designed to be extensible:
- Easy to add new chains
- Unknown HRPs still validate as Cosmos addresses
- Confidence scores reflect HRP recognition

## Examples

### Valid Addresses

```rust
use foxchain_id::identify;

// Cosmos Hub
let result = identify("cosmos1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4")?;

// Osmosis
let result = identify("osmo1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4")?;

// Juno
let result = identify("juno1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4")?;
```

### Invalid Addresses

```rust
// Invalid HRP
identify("unknown1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"); // May still validate as Cosmos but unknown chain

// Invalid Bech32
identify("cosmos1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3tx"); // Error: InvalidInput (bad checksum)

// Missing separator
identify("cosmosqw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"); // Error: InvalidInput
```

## Technical Details

### Bech32 Specification

Bech32 uses:
- **Base32 encoding**: 5-bit groups
- **Checksum**: 6 characters, computed from HRP + data
- **Character set**: `qpzry9x8gf2tvdw0s3jn54khce6mua7l`
- **Case handling**: Case-insensitive, but lowercase is standard

### Address Length

- **Standard**: ~38-45 characters (HRP length + 1 + encoded data)
- **Data**: Typically 20 bytes (40 hex chars) when decoded
- **Encoded**: ~32 characters in Bech32

## Use Cases

- **Wallet addresses**: Standard Cosmos wallet addresses
- **Validator addresses**: Validator operator addresses
- **Contract addresses**: Smart contract addresses (on chains that support them)
- **Multi-chain operations**: Same format across Cosmos ecosystem

## Technical References

- [BIP 173: Base32 address format for native v0-16 witness outputs](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki) (Bech32 specification)
- [Cosmos SDK Documentation](https://docs.cosmos.network/)
- [Bech32 Implementation](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki#Bech32)

## See Also

- [Bitcoin Addresses](bitcoin-addresses.md) - Bech32 also used for Bitcoin SegWit
- [EVM Addresses](evm-addresses.md) - Different format for Ethereum
- [Substrate Addresses](substrate-addresses.md) - SS58 encoding for Polkadot
