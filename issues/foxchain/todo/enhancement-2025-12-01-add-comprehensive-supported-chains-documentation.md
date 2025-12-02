# Add Comprehensive Supported Chains Documentation

**Type**: Enhancement ✨  
**Created**: 2025-12-01  
**Status**: todo

## Summary

Create comprehensive documentation listing all supported blockchain chains, address formats, and public key types that `foxchain-id` can identify. This documentation should serve as a complete reference for users and developers.

## Problem

Currently, the supported chains and formats are scattered across:
- `src/lib.rs` (Chain enum)
- Individual detection modules
- README.md (partial list)
- Various format-specific documentation files

There's no single, comprehensive reference document that lists:
- All supported chains (28 total)
- All address formats (7 types)
- All public key formats (3 encodings, 2 types)
- Address derivation capabilities (4 chains)

## Proposed Solution

Create a comprehensive documentation file `docs/supported-chains.md` that includes:

1. **Complete Chain List** (28 chains):
   - EVM chains (10): Ethereum, Polygon, BSC, Avalanche, Arbitrum, Optimism, Base, Fantom, Celo, Gnosis
   - Bitcoin ecosystem (3): Bitcoin, Litecoin, Dogecoin
   - Cosmos ecosystem (10): Cosmos Hub, Osmosis, Juno, Akash, Stargaze, Secret Network, Terra, Kava, Regen, Sentinel
   - Substrate ecosystem (3): Polkadot, Kusama, Generic Substrate
   - Other chains (2): Solana, Tron, Cardano

2. **Address Format Details**:
   - Format specification for each type
   - Encoding scheme used
   - Validation rules
   - Examples for each format

3. **Public Key Support**:
   - Formats: Hex, Base58, Bech32
   - Types: secp256k1 (compressed/uncompressed), Ed25519
   - Address derivation capabilities

4. **Quick Reference Tables**:
   - Chain → Format mapping
   - Format → Chains mapping
   - Public key type → Supported chains

## Benefits

- **Single source of truth**: One document with all supported chains
- **Better discoverability**: Users can quickly see what's supported
- **Developer reference**: Clear documentation for contributors
- **Completeness**: Ensures no chain is forgotten in documentation

## Implementation Details

1. Create `crates/foxchain-id/docs/supported-chains.md`
2. Structure the document with:
   - Overview section
   - Chain list by ecosystem
   - Address format details
   - Public key support
   - Quick reference tables
   - Examples for each chain
3. Update `docs/README.md` to link to this new document
4. Ensure all 28 chains are listed and documented

## Related Files

- `crates/foxchain-id/src/lib.rs` - Chain enum definition
- `crates/foxchain-id/src/address/detection/*.rs` - Detection modules
- `crates/foxchain-id/src/public_key/detection/mod.rs` - Public key detection
- `crates/foxchain-id/README.md` - Main README
- `crates/foxchain-id/docs/README.md` - Documentation index

## Acceptance Criteria

- [ ] `docs/supported-chains.md` created with all 28 chains listed
- [ ] All address formats documented with examples
- [ ] All public key formats documented
- [ ] Quick reference tables included
- [ ] `docs/README.md` updated with link
- [ ] Examples provided for each major chain type
- [ ] Documentation is clear and comprehensive

