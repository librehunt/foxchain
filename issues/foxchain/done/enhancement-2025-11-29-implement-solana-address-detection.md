# Implement Solana address detection and normalization

**Type:** enhancement  
**Status:** done  
**Branch:** feat/solana-address-detection  
**Linked roadmap section:** v0 - EVM/BTC/Solana/Cosmos/Substrate coverage

---

## 🧠 Context
Solana uses base58 encoding for addresses (public keys). Solana addresses are 32-44 bytes when decoded, typically 32 bytes for standard addresses. They don't have a specific prefix like Bitcoin addresses, so detection relies on length and base58 validation. Solana addresses are case-sensitive and use base58 encoding without checksums in the traditional sense.

## 🎯 Goal
Implement Solana address detection, validation, and normalization in the foxchain-id crate. Support standard Solana public key addresses with proper validation.

## 📏 Success Metrics
- [ ] Detect Solana addresses (base58, 32-44 bytes when decoded)
- [ ] Validate base58 encoding
- [ ] Validate address length (32 bytes standard)
- [ ] Normalize addresses (lowercase base58)
- [ ] Return `Chain::Solana` as candidate
- [ ] Handle edge cases (short/long addresses)

## 🧩 Acceptance Criteria
- [ ] `identify()` function detects Solana addresses correctly
- [ ] Base58 validation implemented
- [ ] Length validation (32 bytes standard, up to 44 bytes)
- [ ] Address normalization
- [ ] Comprehensive test coverage (valid addresses, invalid formats, edge cases)
- [ ] Documentation with examples
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/solana-address-detection`
2. Add dependencies: `base58`
3. Create `formats/solana.rs` module
4. Implement Solana address format detection (base58, length check)
5. Implement base58 validation
6. Implement length validation (32-44 bytes)
7. Implement address normalization
8. Add Solana to `Chain` enum (if not already present)
9. Add comprehensive tests
10. Update `identify()` function to use Solana detector
11. Move this file to `in_progress/` then `done/`
12. Create PR referencing this issue

## 🔍 Alternatives Considered
- Strict 32-byte only → Rejected: Solana supports variable length addresses
- No normalization → Rejected: Normalization improves consistency

## ⚠️ Risks / Mitigations
- Base58 ambiguity with other chains → Mitigation: Use length and context to distinguish
- Variable length handling → Mitigation: Support standard 32 bytes, allow up to 44

## 🔗 Discussion Notes
Solana addresses are simpler than Bitcoin (no multiple formats) but require careful length validation to distinguish from other base58 addresses.

