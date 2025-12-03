# Solana Address Format

Solana uses Base58 encoding for addresses (public keys). Solana addresses are simpler than Bitcoin addresses as they don't use checksums in the traditional sense.

## Format Specification

- **Encoding**: Base58
- **Length**: 32-44 bytes when decoded (typically 32 bytes for standard addresses)
- **Display length**: Typically 32-44 characters
- **Character set**: Base58 (same as Bitcoin: excludes 0, O, I, l)

### Example Addresses

```
9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM  (32 bytes)
```

## Public Key Format

Solana addresses are essentially public keys:

- **Standard addresses**: 32 bytes (256 bits)
- **Program-derived addresses**: Can be longer (up to 44 bytes)
- **No version byte**: Unlike Bitcoin, Solana doesn't use version prefixes
- **No checksum**: Base58 encoding without Base58Check

## Encoding/Decoding

### Encoding Process

1. Take the public key bytes (32 bytes standard)
2. Encode directly in Base58
3. No checksum appended

### Decoding Process

1. Decode Base58 string
2. Validate length (32-44 bytes)
3. Use as public key

## Validation Rules

An address is valid if:

1. ✅ Valid Base58 encoding
2. ✅ Length is 32-44 bytes when decoded
3. ✅ Standard addresses are exactly 32 bytes

### Length Validation

- **32 bytes**: Standard Solana addresses (most common)
- **33-44 bytes**: Program-derived addresses (PDAs) and other special cases
- **< 32 bytes**: Invalid
- **> 44 bytes**: Invalid

## Normalization

`foxchain-id` normalizes Solana addresses:

- **Input**: Any valid Base58 Solana address
- **Output**: Preserved as-is (Base58 is case-sensitive, original format is canonical)
- **Process**: Validate Base58 encoding and length, preserve original case

### Case Handling

Base58 is case-sensitive, so Solana addresses preserve their original case. The library validates the address but does not change the case, as Base58 encoding is case-sensitive and the original format is considered canonical.

## Multi-chain Considerations

Solana addresses are **unique to Solana**:

- Not used by other chains
- Format is distinct from Bitcoin (no version byte, no checksum)
- Format is distinct from EVM (not hexadecimal)

### Distinguishing from Other Formats

Solana addresses can be distinguished by:

- **Length**: 32-44 bytes (different from EVM's 20 bytes)
- **Encoding**: Base58 (different from EVM's hex)
- **No prefix**: No `0x` or `1`/`3` prefix
- **No checksum**: Unlike Bitcoin Base58Check

## Implementation Details

### Detection Algorithm

1. Check if input is valid Base58
2. Decode and verify length (32-44 bytes)
3. Validate it's not a Bitcoin address (no version byte)
4. Return Solana as candidate

### Confidence Scoring

- **32 bytes (standard)**: High confidence (0.95)
- **33-44 bytes (PDA)**: Medium confidence (0.85)
- **Ambiguous with Bitcoin**: Lower confidence if format could match Bitcoin

## Examples

### Valid Addresses

```rust
use foxchain_id::identify;

// Standard 32-byte address
let result = identify("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM")?;

// Program-derived address (longer)
let result = identify("...")?; // 33-44 byte address
```

### Invalid Addresses

```rust
// Too short
identify("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAW"); // Error: InvalidInput

// Invalid Base58 character
identify("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAW0O"); // Error: InvalidInput (contains 0 or O)

// Wrong length
identify("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM12345678901234567890"); // Error: InvalidInput
```

## Technical Details

### Base58 Character Set

```
123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
```

Same as Bitcoin, excludes:
- `0` (zero)
- `O` (capital O)
- `I` (capital i)
- `l` (lowercase L)

### Public Key Derivation

Solana public keys are typically:
- Generated from a seed phrase
- Derived using Ed25519 curve
- 32 bytes (256 bits)

## Use Cases

- **Wallet addresses**: Standard Solana wallet addresses
- **Program addresses**: Smart contract program addresses
- **Token accounts**: SPL token account addresses
- **Program-derived addresses**: PDAs for deterministic address generation

## Technical References

- [Solana Documentation - Keys and Wallets](https://docs.solana.com/developing/programming-model/accounts#keys)
- [Base58 Encoding](https://en.wikipedia.org/wiki/Base58)
- [Ed25519 Cryptography](https://ed25519.cr.yp.to/)

## See Also

- [EVM Addresses](evm-addresses.md) - Hexadecimal format for Ethereum
- [Bitcoin Addresses](bitcoin-addresses.md) - Base58Check with version bytes
- [Cosmos Addresses](cosmos-addresses.md) - Bech32 encoding with HRP
