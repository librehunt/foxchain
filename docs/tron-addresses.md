# Tron Address Format

Tron uses Base58Check encoding for addresses, similar to Bitcoin but with different prefixes and version bytes.

## Format Specification

- **Encoding**: Base58Check
- **Prefix**: `T` (Tron mainnet)
- **Length**: 21 bytes when decoded (1 byte version + 20 bytes address + 4 bytes checksum)
- **Display length**: Typically 34 characters
- **Version byte**: 0x41 for Tron mainnet

### Example Addresses

```
TQn9Y2khEsLMWDmH6s2L8J5K3vF7zX9mN4pL6kH8jG2dF5sA  (Tron mainnet)
```

## Base58Check Encoding

Tron uses the same Base58Check encoding as Bitcoin:

- **Character set**: Base58 (excludes 0, O, I, l)
- **Checksum**: SHA256(SHA256(version + address))[0:4]
- **Structure**: Version byte + 20-byte address + 4-byte checksum

### Character Set

```
123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
```

## Address Format

Tron addresses are 20 bytes (same length as EVM addresses):

1. **Version byte**: 0x41 (Tron mainnet)
2. **Address bytes**: 20 bytes (derived from public key)
3. **Checksum**: 4 bytes (first 4 bytes of double SHA256)

### Address Derivation

Tron addresses are derived from public keys:

1. Take the public key (typically secp256k1, 65 bytes uncompressed)
2. Compute Keccak-256 hash (same as Ethereum)
3. Take last 20 bytes (rightmost 20 bytes)
4. Prepend version byte (0x41)
5. Compute checksum
6. Encode in Base58Check

## Validation Rules

An address is valid if:

1. ✅ Starts with `T` (Tron mainnet)
2. ✅ Valid Base58Check encoding
3. ✅ Correct length (21 bytes when decoded: 1 version + 20 address + 4 checksum)
4. ✅ Valid checksum
5. ✅ Version byte is 0x41 (mainnet)

## Normalization

`foxchain-id` normalizes Tron addresses:

- **Input**: Any valid Tron address
- **Output**: Standard Base58Check format (canonical representation)
- **Process**: Decode, validate, re-encode

### Case Handling

Base58Check is case-sensitive. Tron addresses are typically written in a consistent case, and the library normalizes to standard format.

## Multi-chain Considerations

Tron addresses are **unique to Tron**:

- Format is similar to Bitcoin but with different version byte
- Can be distinguished from Bitcoin by version byte (0x41 vs 0x00)
- Can be distinguished from EVM by encoding (Base58Check vs hex)

### Distinguishing from Other Formats

Tron addresses can be distinguished by:

- **Prefix**: `T` (different from Bitcoin's `1` or `3`)
- **Version byte**: 0x41 (different from Bitcoin's 0x00 or 0x05)
- **Encoding**: Base58Check (different from EVM's hex)
- **Length**: 21 bytes when decoded (different from EVM's 20 bytes)

## Implementation Details

### Detection Algorithm

1. Check if address starts with `T`
2. Validate Base58Check encoding
3. Decode and verify length (21 bytes)
4. Verify version byte is 0x41
5. Validate checksum
6. Return Tron as candidate

### Confidence Scoring

- **Valid Tron address with correct checksum**: 0.95 confidence
- **Valid format but checksum issues**: Lower confidence

## Examples

### Valid Addresses

```rust
use foxchain_id::identify;

// Tron mainnet address
let result = identify("TQn9Y2khEsLMWDmH6s2L8J5K3vF7zX9mN4pL6kH8jG2dF5sA")?;
```

### Invalid Addresses

```rust
// Wrong prefix
identify("1Qn9Y2khEsLMWDmH6s2L8J5K3vF7zX9mN4pL6kH8jG2dF5sA"); // Error: InvalidInput (Bitcoin format)

// Invalid Base58Check
identify("TQn9Y2khEsLMWDmH6s2L8J5K3vF7zX9mN4pL6kH8jG2dF0O"); // Error: InvalidInput (contains 0 or O)

// Wrong checksum
identify("TQn9Y2khEsLMWDmH6s2L8J5K3vF7zX9mN4pL6kH8jG2dF5sB"); // Error: InvalidInput
```

## Technical Details

### Version Bytes

| Network | Version Byte | Prefix |
|---------|--------------|--------|
| Tron Mainnet | 0x41 | `T` |
| Tron Testnet | 0xa0 | `27` or `2a` |

### Address Length

- **When decoded**: 21 bytes (1 version + 20 address + 4 checksum)
- **When encoded**: ~34 characters (Base58 encoding)
- **Address part**: 20 bytes (same as EVM addresses)

## Use Cases

- **Wallet addresses**: Standard Tron wallet addresses
- **Smart contract addresses**: TRC-20 and TRC-721 token contracts
- **Exchange addresses**: Tron addresses for exchanges
- **DApp addresses**: Addresses used in Tron DApps

## Comparison with EVM

Tron addresses are similar to EVM addresses in that they:
- Use 20-byte addresses (derived from Keccak-256 of public key)
- Are derived from the same cryptographic process

But differ in:
- **Encoding**: Base58Check vs hexadecimal
- **Prefix**: `T` vs `0x`
- **Checksum**: Base58Check checksum vs EIP-55 checksum

## Technical References

- [Tron Protocol Documentation](https://developers.tron.network/)
- [Tron Address Format](https://developers.tron.network/docs/account)
- [Base58Check Encoding](https://en.bitcoin.it/wiki/Base58Check_encoding)
- [Keccak-256 Hash Function](https://keccak.team/keccak.html)

## See Also

- [EVM Addresses](evm-addresses.md) - Similar 20-byte addresses but hex encoding
- [Bitcoin Addresses](bitcoin-addresses.md) - Base58Check with different version bytes
- [Solana Addresses](solana-addresses.md) - Base58 without checksum

