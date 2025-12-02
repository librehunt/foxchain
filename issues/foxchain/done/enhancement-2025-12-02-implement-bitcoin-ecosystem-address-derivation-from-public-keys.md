# Implement Bitcoin Ecosystem Address Derivation from Public Keys

**Type**: Enhancement ✨  
**Created**: 2025-12-02  
**Status**: done

## Summary

Currently, `derive_bitcoin_addresses` only returns Bitcoin addresses. This enhancement extends it to also derive addresses for Litecoin and Dogecoin, which use the same secp256k1 public key format but with different version bytes.

## Problem

The `derive_bitcoin_addresses` function only derives Bitcoin addresses (version byte 0x00 for P2PKH):

```rust
// Current code in public_key/derivation/bitcoin.rs
let p2pkh_address = derive_p2pkh_address(&hash160_bytes, 0x00)?;  // Only Bitcoin!
if let Some(addr) = p2pkh_address {
    addresses.push((Chain::Bitcoin, addr));
}
```

Litecoin and Dogecoin use the same derivation algorithm but with different version bytes, so they should also be derivable from the same public key.

## Proposed Solution

Extend `derive_bitcoin_addresses` to derive addresses for all 3 Bitcoin ecosystem chains using their respective version bytes.

### Version Bytes

| Chain | P2PKH Version | P2SH Version |
|-------|--------------|-------------|
| Bitcoin | 0x00 | 0x05 |
| Litecoin | 0x30 | 0x32 |
| Dogecoin | 0x1e | 0x16 |

### Changes Required

1. **Modify `crates/foxchain-id/src/public_key/derivation/bitcoin.rs`**:
   - Derive P2PKH addresses for all 3 chains (Bitcoin, Litecoin, Dogecoin)
   - Use appropriate version bytes for each chain
   - Return all derived addresses in the result vector

2. **Update tests**:
   - Add tests for Litecoin address derivation
   - Add tests for Dogecoin address derivation
   - Verify all 3 chains are returned

## Benefits

- **Completeness**: All Bitcoin ecosystem chains can be identified from public keys
- **Consistency**: Matches address detection behavior (Bitcoin, Litecoin, Dogecoin are all detected)
- **User Experience**: Users get all possible Bitcoin ecosystem candidates

## Implementation Details

### Function Behavior

The function already returns `Vec<(Chain, String)>`, so we just need to add more entries:

```rust
// After modification
pub fn derive_bitcoin_addresses(public_key: &[u8]) -> Result<Vec<(Chain, String)>, Error> {
    let mut addresses = Vec::new();
    
    // ... existing hash160 computation ...
    
    // Bitcoin P2PKH
    if let Some(addr) = derive_p2pkh_address(&hash160_bytes, 0x00)? {
        addresses.push((Chain::Bitcoin, addr));
    }
    
    // Litecoin P2PKH
    if let Some(addr) = derive_p2pkh_address(&hash160_bytes, 0x30)? {
        addresses.push((Chain::Litecoin, addr));
    }
    
    // Dogecoin P2PKH
    if let Some(addr) = derive_p2pkh_address(&hash160_bytes, 0x1e)? {
        addresses.push((Chain::Dogecoin, addr));
    }
    
    Ok(addresses)
}
```

## Related Files

- `crates/foxchain-id/src/public_key/derivation/bitcoin.rs` - Main implementation
- `crates/foxchain-id/src/address/detection/bitcoin.rs` - Reference for version byte mapping

## Acceptance Criteria

- [ ] `derive_bitcoin_addresses` returns addresses for Bitcoin, Litecoin, and Dogecoin
- [ ] Each chain uses the correct version byte (0x00, 0x30, 0x1e for P2PKH)
- [ ] All addresses are valid Base58Check encoded
- [ ] Tests verify all 3 chains are returned
- [ ] Tests verify addresses match expected formats for each chain
- [ ] Integration tests in `public_key/mod.rs` verify all candidates are added

