# Implement EVM Chains Address Derivation from Public Keys

**Type**: Enhancement ✨  
**Created**: 2025-12-02  
**Status**: in_progress

## Summary

Currently, when a secp256k1 public key is detected, only Ethereum is returned as a candidate chain. However, EVM addresses are valid across all EVM-compatible chains. This enhancement modifies the EVM address derivation to return all 10 EVM chains as candidates.

## Problem

When detecting a secp256k1 public key and deriving an EVM address, the code only adds `Chain::Ethereum` as a candidate:

```rust
// Current code in public_key/mod.rs
if derive_evm_address(&key_bytes)?.is_some() {
    candidates.push(ChainCandidate {
        chain: Chain::Ethereum,  // Only Ethereum!
        confidence: 0.85,
        ...
    });
}
```

This is inconsistent with address detection, where all 10 EVM chains are returned as candidates.

## Proposed Solution

Modify `derive_evm_address` to return all EVM chains as candidates, similar to how `detect_evm` works for addresses.

### Changes Required

1. **Modify `crates/foxchain-id/src/public_key/derivation/evm.rs`**:
   - Change return type from `Result<Option<String>, Error>` to `Result<Vec<(Chain, String)>, Error>`
   - Return all 10 EVM chains with the same derived address
   - Each chain gets the same address (EVM addresses are identical across chains)

2. **Update `crates/foxchain-id/src/public_key/mod.rs`**:
   - Modify the secp256k1 handling to iterate over all returned chains
   - Add all 10 EVM chains to candidates with appropriate confidence scores

### Chains to Add

- Polygon
- BSC
- Avalanche
- Arbitrum
- Optimism
- Base
- Fantom
- Celo
- Gnosis

(Plus Ethereum, which is already supported)

## Benefits

- **Consistency**: Public key derivation matches address detection behavior
- **Completeness**: All 10 EVM chains can be identified from public keys
- **User Experience**: Users get all possible EVM chain candidates, not just Ethereum

## Implementation Details

### Function Signature Change

```rust
// Before
pub fn derive_evm_address(public_key: &[u8]) -> Result<Option<String>, Error>

// After
pub fn derive_evm_address(public_key: &[u8]) -> Result<Vec<(Chain, String)>, Error>
```

### Example Output

For a secp256k1 public key, instead of:
- Ethereum (confidence: 0.85)

We get:
- Ethereum (confidence: 0.85)
- Polygon (confidence: 0.80)
- BSC (confidence: 0.80)
- Avalanche (confidence: 0.80)
- Arbitrum (confidence: 0.80)
- Optimism (confidence: 0.80)
- Base (confidence: 0.80)
- Fantom (confidence: 0.80)
- Celo (confidence: 0.80)
- Gnosis (confidence: 0.80)

## Related Files

- `crates/foxchain-id/src/public_key/derivation/evm.rs` - Main implementation
- `crates/foxchain-id/src/public_key/mod.rs` - Integration point
- `crates/foxchain-id/src/address/detection/evm.rs` - Reference for multi-chain candidate generation

## Acceptance Criteria

- [ ] `derive_evm_address` returns all 10 EVM chains
- [ ] All chains have the same derived address (EVM addresses are identical)
- [ ] Confidence scores are appropriate (Ethereum highest, others slightly lower)
- [ ] Existing tests updated to reflect new return type
- [ ] New tests verify all 10 chains are returned
- [ ] Integration tests in `public_key/mod.rs` verify all candidates are added

