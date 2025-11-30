# Implement other blockchain address detection (TON, Cardano, etc.)

**Type:** enhancement  
**Status:** done  
**Branch:** feat/other-chains-detection  
**Linked roadmap section:** v0.x - Extended chain coverage

---

## 🧠 Context
Beyond the major blockchain ecosystems (EVM, Bitcoin, Solana, Cosmos, Substrate, Tron), there are other significant chains that should be supported: TON (The Open Network), Cardano, Algorand, Near, and others. Each has unique address formats and encoding schemes that need detection and normalization.

## 🎯 Goal
Implement address detection and normalization for additional blockchain networks including TON, Cardano, Algorand, Near, and other significant chains in the foxchain-id crate.

## 📏 Success Metrics
- [ ] Detect TON addresses (base64url or user-friendly format)
- [ ] Detect Cardano addresses (bech32 with specific HRPs)
- [ ] Detect Algorand addresses (base32 with checksum)
- [ ] Detect Near addresses (base58 with specific format)
- [ ] Support at least 3-5 additional chains
- [ ] Proper validation and normalization for each chain
- [ ] Return appropriate `Chain` variants

## 🧩 Acceptance Criteria
- [ ] `identify()` function detects addresses for each supported chain
- [ ] Proper encoding validation for each chain format
- [ ] Address normalization for each chain
- [ ] Chain variants added to `Chain` enum
- [ ] Comprehensive test coverage for each chain
- [ ] Documentation with examples for each chain
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/other-chains-detection`
2. Research address formats for target chains
3. Add necessary dependencies for each chain format
4. Create format modules for each chain:
   - `formats/ton.rs`
   - `formats/cardano.rs`
   - `formats/algorand.rs`
   - `formats/near.rs`
   - etc.
5. Implement detection, validation, and normalization for each chain
6. Add chain variants to `Chain` enum
7. Add comprehensive tests for each chain
8. Update `identify()` function to use new detectors
9. Update documentation
10. Move this file to `in_progress/` then `done/`
11. Create PR referencing this issue

## 🔍 Alternatives Considered
- Single issue for all chains → Rejected: Too large, better to group by ecosystem
- Skip less common chains → Rejected: Comprehensive coverage is a goal

## ⚠️ Risks / Mitigations
- Many different formats → Mitigation: Implement incrementally, one chain at a time
- Format complexity → Mitigation: Research thoroughly, use well-tested libraries where possible

## 🔗 Discussion Notes
This is a catch-all issue for chains that don't fit into the major ecosystem categories. Can be broken down further if needed. Priority chains: TON, Cardano, Algorand, Near.

