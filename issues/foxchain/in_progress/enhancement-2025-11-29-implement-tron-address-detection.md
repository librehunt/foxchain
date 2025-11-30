# Implement Tron address detection and normalization

**Type:** enhancement  
**Status:** in_progress  
**Branch:** feat/tron-address-detection  
**Linked roadmap section:** v0 - EVM/BTC/Solana/Cosmos/Substrate coverage

---

## 🧠 Context
Tron uses base58check encoding for addresses, similar to Bitcoin but with different prefixes. Tron addresses start with 'T' for mainnet. The address format is base58check encoded with a specific version byte. Tron addresses are 21 bytes (including version byte) when decoded, displayed as base58check strings starting with 'T'.

## 🎯 Goal
Implement Tron address detection, validation, and normalization in the foxchain-id crate. Support Tron mainnet addresses with proper base58check validation.

## 📏 Success Metrics
- [ ] Detect Tron addresses (base58check, starts with T)
- [ ] Validate base58check encoding and checksum
- [ ] Validate address length (21 bytes when decoded)
- [ ] Normalize addresses
- [ ] Return `Chain::Tron` as candidate
- [ ] Handle mainnet addresses (T prefix)

## 🧩 Acceptance Criteria
- [ ] `identify()` function detects Tron addresses correctly
- [ ] Base58check validation implemented
- [ ] Length validation (21 bytes)
- [ ] Version byte validation (Tron mainnet)
- [ ] Address normalization
- [ ] Comprehensive test coverage (valid addresses, invalid formats)
- [ ] Documentation with examples
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/tron-address-detection`
2. Add dependencies: `base58`, `sha2` (for checksum validation)
3. Create `formats/tron.rs` module
4. Implement Tron address format detection (starts with T, base58check)
5. Implement base58check validation
6. Implement length validation (21 bytes)
7. Implement version byte validation
8. Add Tron to `Chain` enum (if not already present)
9. Implement address normalization
10. Add comprehensive tests
11. Update `identify()` function to use Tron detector
12. Move this file to `in_progress/` then `done/`
13. Create PR referencing this issue

## 🔍 Alternatives Considered
- No version byte validation → Rejected: Version byte ensures correct chain identification
- Generic base58check → Rejected: Tron has specific format requirements

## ⚠️ Risks / Mitigations
- Base58check validation complexity → Mitigation: Use well-tested base58 library
- Similarity to Bitcoin addresses → Mitigation: Use prefix (T) and version byte to distinguish

## 🔗 Discussion Notes
Tron addresses are similar to Bitcoin (base58check) but have distinct characteristics (T prefix, 21 bytes). Need careful validation to distinguish from Bitcoin addresses.

