# Substrate/Polkadot Ecosystem Address Format

The Substrate ecosystem (Polkadot, Kusama, and many parachains) uses SS58 encoding for addresses. SS58 is a variant of Base58 with chain-specific prefixes and checksums.

## Format Specification

- **Encoding**: SS58 (Substrate-specific Base58 variant)
- **Structure**: Base58-encoded account ID with chain-specific prefix
- **Prefix**: Chain-specific byte identifying the network
- **Account ID**: 32 bytes (256 bits)
- **Checksum**: Built into SS58 encoding

### Example Addresses

```
5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY  (Polkadot)
EXtQYCr8j2vB5A1wF8q2h5b3vF7zX9mN4pL6kH8jG2dF5sA  (Kusama)
```

## SS58 Encoding

SS58 is similar to Base58Check but with Substrate-specific modifications:

- **Base58 character set**: Same as Bitcoin (excludes 0, O, I, l)
- **Prefix encoding**: Chain-specific prefix byte
- **Checksum**: Computed differently than Base58Check
- **Account ID**: Always 32 bytes for standard addresses

### Character Set

```
123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
```

Same as Bitcoin Base58.

## Chain Identification via Prefix

Different Substrate chains use different prefix bytes:

| Chain | Prefix | Example Format |
|-------|--------|----------------|
| Polkadot | 0 | `1...` (starts with 1) |
| Kusama | 2 | `C...` or `F...` (typically) |
| Generic Substrate | 42 | Variable |
| Testnet | Various | Variable |

### Prefix Encoding

The prefix is encoded as part of the address:
1. Take account ID (32 bytes)
2. Prepend prefix byte
3. Compute SS58 checksum
4. Encode in Base58

## Account ID

Substrate account IDs are:

- **Length**: 32 bytes (256 bits)
- **Source**: Derived from public key or generated deterministically
- **Format**: Raw bytes, no version prefix (unlike Bitcoin)

### Derivation

Account IDs can be:
- **Direct**: Public key bytes (for Ed25519, SR25519)
- **Hashed**: Hash of public key (for secp256k1, ECDSA)
- **Derived**: Program-derived addresses (PDAs)

## Validation Rules

An address is valid if:

1. ✅ Valid SS58 encoding
2. ✅ Valid checksum
3. ✅ Decodes to correct length (prefix + 32 bytes account ID + checksum)
4. ✅ Prefix matches known chain (optional, for chain identification)

## Normalization

`foxchain-id` normalizes Substrate addresses:

- **Input**: Any valid SS58 address
- **Output**: Standard SS58 format (canonical representation)
- **Process**: Decode, validate, re-encode

### Case Handling

SS58 addresses are case-sensitive, but there's a standard format. The library normalizes to a consistent representation.

## Multi-chain Considerations

The Substrate ecosystem includes:

- **Polkadot**: Main network, prefix 0
- **Kusama**: Canary network, prefix 2
- **Parachains**: Many chains with various prefixes
- **Testnets**: Various test networks

### Chain Identification

`foxchain-id` identifies chains from SS58 prefix:

1. Decode SS58 address
2. Extract prefix byte
3. Match prefix to known chain
4. Return chain-specific candidate

### Supported Chains

Planned support for:
- Polkadot (prefix 0)
- Kusama (prefix 2)
- Major parachains
- Generic Substrate (for unknown prefixes)

## Implementation Details

### Detection Algorithm

1. Check if input is valid Base58
2. Decode SS58 address
3. Extract prefix byte
4. Validate checksum
5. Verify account ID length (32 bytes)
6. Match prefix to known chain
7. Return chain-specific candidate

### SS58 Checksum

SS58 checksum calculation:
1. Take prefix + account ID
2. Compute `blake2b_512(SS58PRE + prefix + account_id)`
3. Take first bytes as checksum (length depends on address length)
4. Append checksum
5. Encode in Base58

Where `SS58PRE` is the constant `"SS58PRE"` (0x53533538505245).

### Prefix Mapping

```rust
// Example mapping
0 => Chain::Polkadot
2 => Chain::Kusama
42 => Chain::Substrate  // Generic
// etc.
```

## Examples

### Valid Addresses

```rust
use foxchain_id::identify;

// Polkadot (prefix 0)
let result = identify("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")?;

// Kusama (prefix 2)
let result = identify("EXtQYCr8j2vB5A1wF8q2h5b3vF7zX9mN4pL6kH8jG2dF5sA")?;
```

### Invalid Addresses

```rust
// Invalid SS58 encoding
identify("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKut0O"); // Error: InvalidInput (contains 0 or O)

// Wrong checksum
identify("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQX"); // Error: InvalidInput

// Wrong length
identify("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKut"); // Error: InvalidInput
```

## Technical Details

### SS58 vs Base58Check

Differences from Bitcoin Base58Check:
- **Checksum algorithm**: Uses Blake2b instead of double SHA256
- **Prefix encoding**: Chain-specific prefix byte
- **No version byte**: Prefix serves similar purpose but is chain-specific

### Address Length

- **Standard**: ~47-48 characters
- **Account ID**: 32 bytes
- **Prefix**: 1 byte
- **Checksum**: Variable (typically 1-2 bytes)
- **Total encoded**: ~47-48 characters

## Use Cases

- **Wallet addresses**: Standard Substrate wallet addresses
- **Validator addresses**: Validator stash and controller addresses
- **Parachain addresses**: Addresses on various parachains
- **Cross-chain**: Same format across Substrate ecosystem

## Technical References

- [SS58 Address Format](https://docs.substrate.io/reference/address-formats/)
- [Polkadot Wiki - Accounts](https://wiki.polkadot.network/docs/learn-accounts)
- [Substrate Documentation](https://docs.substrate.io/)
- [Blake2b Hash Function](https://www.blake2.net/)

## See Also

- [Bitcoin Addresses](bitcoin-addresses.md) - Base58Check encoding
- [Cosmos Addresses](cosmos-addresses.md) - Bech32 encoding
- [EVM Addresses](evm-addresses.md) - Hexadecimal format
