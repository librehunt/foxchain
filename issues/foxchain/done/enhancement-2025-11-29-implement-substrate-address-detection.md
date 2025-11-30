# Implement Substrate/Polkadot ecosystem address detection and normalization

**Type:** enhancement  
**Status:** done  
**Branch:** feat/substrate-address-detection  
**Linked roadmap section:** v0 - EVM/BTC/Solana/Cosmos/Substrate coverage

---

## 🧠 Context
The Substrate ecosystem (Polkadot, Kusama, and many parachains) uses SS58 encoding for addresses. SS58 is a variant of base58 with a chain-specific prefix and checksum. Different chains use different prefixes (Polkadot uses 0, Kusama uses 2, etc.). SS58 addresses encode account IDs (32 bytes) with chain-specific prefixes.

## 🎯 Goal
Implement Substrate/Polkadot ecosystem address detection, validation, and normalization in the foxchain-id crate. Support major Substrate chains through SS58 prefix detection.

## 📏 Success Metrics
- [ ] Detect SS58 addresses (base58 with chain-specific prefixes)
- [ ] Validate SS58 encoding and checksum
- [ ] Identify chain from SS58 prefix (Polkadot, Kusama, etc.)
- [ ] Support at least 5 major Substrate chains
- [ ] Decode account ID from SS58 address
- [ ] Normalize addresses
- [ ] Return appropriate `Chain` variant for detected chain

## 🧩 Acceptance Criteria
- [ ] `identify()` function detects Substrate addresses correctly
- [ ] SS58 validation implemented
- [ ] Prefix-to-chain mapping for major chains
- [ ] Chain identification from SS58 prefix
- [ ] Account ID extraction
- [ ] Address normalization
- [ ] Comprehensive test coverage (multiple chains, invalid formats)
- [ ] Documentation with examples
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/substrate-address-detection`
2. Add dependencies: `ss58` (or implement SS58 encoding/decoding)
3. Create `formats/substrate.rs` module
4. Implement SS58 address detection
5. Implement SS58 validation and checksum verification
6. Implement prefix extraction
7. Create prefix-to-chain mapping (Polkadot=0, Kusama=2, etc.)
8. Add Substrate chain variants to `Chain` enum (Polkadot, Kusama, etc.)
9. Implement chain identification from prefix
10. Implement account ID extraction
11. Implement address normalization
12. Add comprehensive tests for multiple chains
13. Update `identify()` function to use Substrate detector
14. Move this file to `in_progress/` then `done/`
15. Create PR referencing this issue

## 🔍 Alternatives Considered
- Single Substrate chain → Rejected: Ecosystem has many chains, need multi-chain support
- No prefix detection → Rejected: Chain identification requires prefix mapping

## ⚠️ Risks / Mitigations
- SS58 library availability → Mitigation: Use `ss58` crate or implement SS58 encoding
- Many Substrate chains → Mitigation: Start with major chains (Polkadot, Kusama), make prefix mapping extensible

## 🔗 Discussion Notes
Substrate ecosystem uses SS58 encoding which is similar to base58 but with chain-specific prefixes. Need to support multiple chains through prefix detection.

