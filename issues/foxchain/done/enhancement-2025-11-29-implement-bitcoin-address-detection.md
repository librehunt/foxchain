# Implement Bitcoin ecosystem address detection and normalization

**Type:** enhancement  
**Status:** done  
**Branch:** feat/bitcoin-address-detection  
**Linked roadmap section:** v0 - EVM/BTC/Solana/Cosmos/Substrate coverage

---

## 🧠 Context
Bitcoin and related chains (Litecoin, Dogecoin) use multiple address formats: P2PKH (legacy, starts with 1), P2SH (starts with 3), and Bech32 (native SegWit, starts with bc1 for Bitcoin, lt1 for Litecoin, etc.). Each format has different characteristics and validation rules. Bitcoin addresses are base58check encoded, while Bech32 uses bech32 encoding.

## 🎯 Goal
Implement Bitcoin, Litecoin, and Dogecoin address detection, validation, and normalization in the foxchain-id crate. Support all three address formats (P2PKH, P2SH, Bech32) with proper format detection and normalization.

## 📏 Success Metrics
- [ ] Detect P2PKH addresses (starts with 1, base58check)
- [ ] Detect P2SH addresses (starts with 3, base58check)
- [ ] Detect Bech32 addresses (starts with bc1/lt1/etc., bech32)
- [ ] Validate base58check checksums
- [ ] Validate bech32 checksums
- [ ] Normalize addresses to preferred format (Bech32 when possible)
- [ ] Support Bitcoin, Litecoin, and Dogecoin
- [ ] Distinguish between chains based on address prefixes

## 🧩 Acceptance Criteria
- [ ] `identify()` function detects Bitcoin ecosystem addresses correctly
- [ ] Base58check validation implemented
- [ ] Bech32 validation implemented
- [ ] Address format detection (P2PKH/P2SH/Bech32)
- [ ] Chain identification (Bitcoin/Litecoin/Dogecoin) based on prefixes
- [ ] Address normalization to Bech32 when applicable
- [ ] Comprehensive test coverage (all formats, all chains, invalid cases)
- [ ] Documentation with examples
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/bitcoin-address-detection`
2. Add dependencies: `base58`, `bech32`, `sha2` (for checksum validation)
3. Create `formats/bitcoin.rs` module
4. Implement P2PKH address detection and validation
5. Implement P2SH address detection and validation
6. Implement Bech32 address detection and validation
7. Add Bitcoin, Litecoin, Dogecoin to `Chain` enum
8. Implement chain identification based on address prefixes
9. Implement address normalization (prefer Bech32)
10. Add comprehensive tests for all formats and chains
11. Update `identify()` function to use Bitcoin detector
12. Move this file to `in_progress/` then `done/`
13. Create PR referencing this issue

## 🔍 Alternatives Considered
- Single format support → Rejected: Need to support all formats for compatibility
- No normalization → Rejected: Normalization improves usability

## ⚠️ Risks / Mitigations
- Base58check validation complexity → Mitigation: Use well-tested base58 library
- Bech32 HRP detection → Mitigation: Clear mapping of HRPs to chains

## 🔗 Discussion Notes
Bitcoin ecosystem is second priority after EVM. Multiple address formats require careful validation and normalization logic.

