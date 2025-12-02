# Implement Tron and Cardano Address Derivation from Public Keys

**Type**: Enhancement ✨  
**Created**: 2025-12-02  
**Status**: Proposal

## Summary

Implement address derivation from public keys for Tron (secp256k1) and Cardano (Ed25519). These chains use different derivation algorithms than the existing implementations.

## Problem

Currently, Tron and Cardano addresses cannot be derived from public keys. Both chains require specific derivation logic:
- **Tron**: Uses secp256k1 with Keccak-256 (like EVM) but encodes as Base58Check with version byte 0x41
- **Cardano**: Uses Ed25519 with SHA3-256 hash and Bech32 encoding with specific HRPs and address types

## Proposed Solution

Create two new derivation functions:
1. `derive_tron_address` for Tron (secp256k1)
2. `derive_cardano_address` for Cardano (Ed25519)

### Tron Address Derivation

**Process**:
1. Take secp256k1 public key (compressed or uncompressed)
2. Decompress if needed
3. Compute Keccak-256 hash (same as EVM)
4. Take last 20 bytes
5. Prepend version byte 0x41
6. Compute Base58Check checksum
7. Encode as Base58Check

**Key Type**: secp256k1 (33 or 65 bytes)

### Cardano Address Derivation

**Process**:
1. Take Ed25519 public key (32 bytes)
2. Compute SHA3-256 hash
3. Take first 28 bytes
4. Encode as Bech32 with HRP:
   - Mainnet: `addr` (payment), `stake` (stake)
   - Testnet: `addr_test` (payment), `stake_test` (stake)
5. Support address types:
   - Type 0: Payment address
   - Type 14: Stake address

**Key Type**: Ed25519 (32 bytes)

### Changes Required

1. **Create `crates/foxchain-id/src/public_key/derivation/tron.rs`**:
   - Implement `derive_tron_address` function
   - Handle compressed/uncompressed secp256k1 keys
   - Use Keccak-256 hash (reuse from EVM)
   - Encode as Base58Check with version byte 0x41
   - Return Tron chain candidate

2. **Create `crates/foxchain-id/src/public_key/derivation/cardano.rs`**:
   - Implement `derive_cardano_address` function
   - Handle Ed25519 keys (32 bytes)
   - Use SHA3-256 hash (new dependency)
   - Encode as Bech32 with Cardano HRPs
   - Support payment and stake addresses
   - Return Cardano chain candidate

3. **Update `crates/foxchain-id/src/public_key/derivation/mod.rs`**:
   - Export `derive_tron_address` and `derive_cardano_address`

4. **Update `crates/foxchain-id/src/public_key/mod.rs`**:
   - Add Tron derivation for secp256k1 keys
   - Add Cardano derivation for Ed25519 keys
   - Add chain candidates to results

## Benefits

- **Completeness**: Tron and Cardano can be identified from public keys
- **Consistency**: Matches address detection behavior
- **User Experience**: Users get all possible chain candidates

## Implementation Details

### Tron Function Signature

```rust
pub fn derive_tron_address(public_key: &[u8]) -> Result<Option<String>, Error>
```

### Cardano Function Signature

```rust
pub fn derive_cardano_address(public_key: &[u8], address_type: CardanoAddressType) -> Result<Vec<String>, Error>
// Or return both payment and stake addresses
pub fn derive_cardano_address(public_key: &[u8]) -> Result<Vec<(String, String)>, Error>  // (payment, stake)
```

### Tron Implementation Notes

- Reuse `keccak256` from `crates/foxchain-id/src/shared/crypto/hash.rs`
- Reuse `secp256k1::decompress_public_key` for compressed keys
- Use Base58Check encoding (similar to Bitcoin)
- Version byte: 0x41 (Tron mainnet)

### Cardano Implementation Notes

- Need SHA3-256 hash (not SHA-256)
- Address structure: 1 byte header + 28 bytes payload
- Header encodes address type and network (mainnet/testnet)
- Bech32 encoding with chain-specific HRPs
- May need to derive both payment and stake addresses

## Dependencies

- **Tron**: No new dependencies (reuse existing crypto utilities)
- **Cardano**: May need `sha3` crate for SHA3-256 hash

## Related Files

- `crates/foxchain-id/src/public_key/derivation/tron.rs` - New file
- `crates/foxchain-id/src/public_key/derivation/cardano.rs` - New file
- `crates/foxchain-id/src/public_key/mod.rs` - Integration point
- `crates/foxchain-id/src/shared/crypto/hash.rs` - Keccak-256 for Tron
- `crates/foxchain-id/src/shared/crypto/secp256k1.rs` - Decompression for Tron
- `crates/foxchain-id/src/address/detection/tron.rs` - Reference for Tron format
- `crates/foxchain-id/src/address/detection/cardano.rs` - Reference for Cardano format

## Acceptance Criteria

### Tron
- [ ] `derive_tron_address` function created
- [ ] Supports compressed secp256k1 keys (33 bytes)
- [ ] Supports uncompressed secp256k1 keys (65 bytes)
- [ ] Uses Keccak-256 hash (same as EVM)
- [ ] Encodes as Base58Check with version byte 0x41
- [ ] Returns valid Tron address format (starts with `T`)
- [ ] Tests for compressed and uncompressed keys
- [ ] Integration test in `public_key/mod.rs`

### Cardano
- [ ] `derive_cardano_address` function created
- [ ] Supports Ed25519 keys (32 bytes)
- [ ] Uses SHA3-256 hash
- [ ] Derives payment addresses (type 0)
- [ ] Derives stake addresses (type 14)
- [ ] Supports mainnet HRPs (`addr`, `stake`)
- [ ] Supports testnet HRPs (`addr_test`, `stake_test`)
- [ ] Encodes as Bech32
- [ ] Returns valid Cardano address format
- [ ] Tests for payment and stake addresses
- [ ] Integration test in `public_key/mod.rs`

