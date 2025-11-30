# Implement public key detection and address derivation

**Type:** enhancement  
**Status:** done  
**Branch:** feat/public-key-detection  
**Linked roadmap section:** v0 - Initial features

---

## 🧠 Context
Users may provide public keys instead of addresses. Public keys come in various formats: hex (compressed/uncompressed), base58, bech32, etc. The library should detect public keys and derive addresses where applicable. This enables identification even when only public keys are available.

## 🎯 Goal
Implement public key detection and address derivation in the foxchain-id crate. Support common public key formats and derive addresses for chains where derivation is possible.

## 📏 Success Metrics
- [ ] Detect hex public keys (65 bytes uncompressed, 33 bytes compressed)
- [ ] Detect base58 public keys
- [ ] Detect bech32 public keys
- [ ] Derive EVM addresses from public keys
- [ ] Derive Bitcoin addresses from public keys (P2PKH, P2SH, Bech32)
- [ ] Derive Solana addresses from public keys
- [ ] Derive Cosmos addresses from public keys
- [ ] Return derived addresses in identification results

## 🧩 Acceptance Criteria
- [ ] `identify()` function detects public keys correctly
- [ ] Public key format detection (hex, base58, bech32)
- [ ] Address derivation for supported chains
- [ ] Proper error handling for unsupported derivations
- [ ] Comprehensive test coverage (all formats, all chains)
- [ ] Documentation with examples
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/public-key-detection`
2. Add dependencies: `secp256k1` (for Bitcoin/EVM), `ed25519` (for Solana), etc.
3. Create `formats/public_key.rs` module
4. Implement hex public key detection (compressed/uncompressed)
5. Implement base58 public key detection
6. Implement bech32 public key detection
7. Implement EVM address derivation (keccak256 hash of public key)
8. Implement Bitcoin address derivation (P2PKH, P2SH, Bech32)
9. Implement Solana address derivation
10. Implement Cosmos address derivation
11. Add comprehensive tests
12. Update `identify()` function to handle public keys
13. Move this file to `in_progress/` then `done/`
14. Create PR referencing this issue

## 🔍 Alternatives Considered
- No public key support → Rejected: Users may only have public keys
- Derivation for all chains → Rejected: Some chains don't support derivation, implement incrementally

## ⚠️ Risks / Mitigations
- Cryptographic complexity → Mitigation: Use well-tested crypto libraries
- Derivation errors → Mitigation: Proper error handling, clear error messages

## 🔗 Discussion Notes
Public key detection and derivation is important for flexibility. Start with major chains (EVM, Bitcoin, Solana) and extend to others incrementally.


