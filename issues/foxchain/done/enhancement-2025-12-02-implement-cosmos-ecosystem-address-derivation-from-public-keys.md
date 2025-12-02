# Implement Cosmos Ecosystem Address Derivation from Public Keys

**Type**: Enhancement ✨  
**Created**: 2025-12-02  
**Status**: done

## Summary

Currently, `derive_cosmos_address` only returns Cosmos Hub addresses (HRP: `cosmos`). This enhancement extends it to derive addresses for all 10 Cosmos ecosystem chains, which use the same Ed25519 public key format but with different HRPs.

## Problem

The `derive_cosmos_address` function only derives Cosmos Hub addresses:

```rust
// Current code in public_key/derivation/cosmos.rs
let address = bech32_encoding::encode("cosmos", &data_u5, Variant::Bech32)?;  // Only Cosmos Hub!
```

All Cosmos chains use the same derivation algorithm (SHA256 hash + Bech32 encoding) but with different HRPs, so they should all be derivable from the same Ed25519 public key.

## Proposed Solution

Extend `derive_cosmos_address` to derive addresses for all 10 Cosmos ecosystem chains using their respective HRPs.

### HRP Mapping

| Chain | HRP |
|-------|-----|
| Cosmos Hub | `cosmos` |
| Osmosis | `osmo` |
| Juno | `juno` |
| Akash | `akash` |
| Stargaze | `stars` |
| Secret Network | `secret` |
| Terra | `terra` |
| Kava | `kava` |
| Regen | `regen` |
| Sentinel | `sent` |

### Changes Required

1. **Modify `crates/foxchain-id/src/public_key/derivation/cosmos.rs`**:
   - Change return type from `Result<Option<String>, Error>` to `Result<Vec<(Chain, String)>, Error>`
   - Derive addresses for all 10 Cosmos chains with their respective HRPs
   - Return all derived addresses in the result vector

2. **Update `crates/foxchain-id/src/public_key/mod.rs`**:
   - Modify the Ed25519 handling to iterate over all returned chains
   - Add all 10 Cosmos chains to candidates with appropriate confidence scores

## Benefits

- **Completeness**: All Cosmos ecosystem chains can be identified from public keys
- **Consistency**: Matches address detection behavior (all Cosmos chains are detected)
- **User Experience**: Users get all possible Cosmos chain candidates

## Implementation Details

### Function Signature Change

```rust
// Before
pub fn derive_cosmos_address(public_key: &[u8]) -> Result<Option<String>, Error>

// After
pub fn derive_cosmos_address(public_key: &[u8]) -> Result<Vec<(Chain, String)>, Error>
```

### Implementation Approach

```rust
pub fn derive_cosmos_address(public_key: &[u8]) -> Result<Vec<(Chain, String)>, Error> {
    if public_key.len() != 32 {
        return Ok(Vec::new());
    }

    // Compute SHA256 hash (same for all Cosmos chains)
    let hash = sha256(public_key);
    let address_bytes = &hash[..20];
    
    // Convert to 5-bit groups
    let data = bech32_encoding::convert_bits(address_bytes, 8, 5, true)?;
    let data_u5: Vec<u5> = bech32_encoding::bytes_to_u5(&data);

    // Derive addresses for all Cosmos chains
    let mut addresses = Vec::new();
    let hrps = [
        ("cosmos", Chain::CosmosHub),
        ("osmo", Chain::Osmosis),
        ("juno", Chain::Juno),
        ("akash", Chain::Akash),
        ("stars", Chain::Stargaze),
        ("secret", Chain::SecretNetwork),
        ("terra", Chain::Terra),
        ("kava", Chain::Kava),
        ("regen", Chain::Regen),
        ("sent", Chain::Sentinel),
    ];

    for (hrp, chain) in hrps {
        let address = bech32_encoding::encode(hrp, &data_u5, Variant::Bech32)?;
        addresses.push((chain, address));
    }

    Ok(addresses)
}
```

## Related Files

- `crates/foxchain-id/src/public_key/derivation/cosmos.rs` - Main implementation
- `crates/foxchain-id/src/public_key/mod.rs` - Integration point
- `crates/foxchain-id/src/address/detection/cosmos.rs` - Reference for HRP mapping

## Acceptance Criteria

- [ ] `derive_cosmos_address` returns all 10 Cosmos chains
- [ ] Each chain uses the correct HRP
- [ ] All addresses are valid Bech32 encoded
- [ ] Tests verify all 10 chains are returned
- [ ] Tests verify addresses match expected formats for each chain
- [ ] Integration tests in `public_key/mod.rs` verify all candidates are added

