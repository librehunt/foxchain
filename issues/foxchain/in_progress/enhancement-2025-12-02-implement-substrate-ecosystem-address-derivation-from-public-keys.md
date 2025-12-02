# Implement Substrate Ecosystem Address Derivation from Public Keys

**Type**: Enhancement ✨  
**Created**: 2025-12-02  
**Status**: in_progress

## Summary

Implement address derivation from public keys for the Substrate ecosystem (Polkadot, Kusama, Generic Substrate). Substrate supports multiple key types (Ed25519, sr25519, secp256k1) and uses SS58 encoding with chain-specific prefixes.

## Problem

Currently, Substrate addresses cannot be derived from public keys. The Substrate ecosystem uses SS58 encoding and supports multiple cryptographic key types, requiring new derivation logic.

## Proposed Solution

Create a new `derive_substrate_address` function that:
1. Supports Ed25519, sr25519, and secp256k1 public keys
2. Derives Account ID (32 bytes) based on key type
3. Encodes as SS58 with chain-specific prefixes
4. Returns all 3 Substrate chains as candidates

### Key Type Handling

| Key Type | Account ID Derivation |
|----------|---------------------|
| Ed25519 | Account ID = public key bytes (32 bytes) |
| sr25519 | Account ID = public key bytes (32 bytes) |
| secp256k1 | Account ID = Blake2b hash of public key (32 bytes) |

### SS58 Prefixes

| Chain | SS58 Prefix |
|-------|-------------|
| Polkadot | 0 |
| Kusama | 2 |
| Substrate (Generic) | 42 |

### Changes Required

1. **Create `crates/foxchain-id/src/public_key/derivation/substrate.rs`**:
   - Implement `derive_substrate_address` function
   - Handle Ed25519 keys (32 bytes) → Account ID = key bytes
   - Handle sr25519 keys (32 bytes) → Account ID = key bytes
   - Handle secp256k1 keys (33 or 65 bytes) → Account ID = Blake2b hash
   - Encode Account ID as SS58 with all 3 chain prefixes
   - Return all 3 chains as candidates

2. **Update `crates/foxchain-id/src/public_key/derivation/mod.rs`**:
   - Export `derive_substrate_address`

3. **Update `crates/foxchain-id/src/public_key/detection/mod.rs`**:
   - Add detection for sr25519 keys (32 bytes, similar to Ed25519)
   - Add `PublicKeyType::Sr25519` variant

4. **Update `crates/foxchain-id/src/public_key/mod.rs`**:
   - Add Substrate derivation for Ed25519, sr25519, and secp256k1 keys
   - Add all 3 Substrate chains to candidates

## Benefits

- **Completeness**: All Substrate ecosystem chains can be identified from public keys
- **Flexibility**: Supports all major Substrate key types
- **Consistency**: Matches address detection behavior

## Implementation Details

### Function Signature

```rust
pub fn derive_substrate_address(public_key: &[u8], key_type: PublicKeyType) -> Result<Vec<(Chain, String)>, Error>
```

### Account ID Derivation

```rust
// For Ed25519/sr25519 (32 bytes)
let account_id = public_key.to_vec();  // Direct use

// For secp256k1 (33 or 65 bytes)
let account_id = blake2b_hash(public_key);  // Hash to 32 bytes
```

### SS58 Encoding

Use existing SS58 encoding utilities from `crates/foxchain-id/src/shared/encoding/ss58.rs` and `crates/foxchain-id/src/shared/checksum/ss58.rs`.

### Example Output

For an Ed25519 public key:
- Polkadot (confidence: 0.85)
- Kusama (confidence: 0.80)
- Substrate (confidence: 0.75)

## Dependencies

- May need `sr25519` crate for sr25519 key validation (if needed)
- Existing `blake2` crate for secp256k1 hashing
- Existing SS58 encoding utilities

## Related Files

- `crates/foxchain-id/src/public_key/derivation/substrate.rs` - New file
- `crates/foxchain-id/src/public_key/detection/mod.rs` - Add sr25519 detection
- `crates/foxchain-id/src/public_key/mod.rs` - Integration point
- `crates/foxchain-id/src/shared/encoding/ss58.rs` - SS58 encoding utilities
- `crates/foxchain-id/src/shared/checksum/ss58.rs` - SS58 checksum utilities
- `crates/foxchain-id/src/address/detection/substrate.rs` - Reference for SS58 prefix mapping

## Acceptance Criteria

- [ ] `derive_substrate_address` function created
- [ ] Supports Ed25519 keys (32 bytes → Account ID)
- [ ] Supports sr25519 keys (32 bytes → Account ID)
- [ ] Supports secp256k1 keys (33/65 bytes → Blake2b hash → Account ID)
- [ ] Returns all 3 Substrate chains (Polkadot, Kusama, Substrate)
- [ ] Each chain uses correct SS58 prefix (0, 2, 42)
- [ ] All addresses are valid SS58 encoded
- [ ] Tests for each key type
- [ ] Tests verify all 3 chains are returned
- [ ] Integration tests in `public_key/mod.rs` verify all candidates are added

