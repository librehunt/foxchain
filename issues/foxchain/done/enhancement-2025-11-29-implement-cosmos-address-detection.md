# Implement Cosmos ecosystem address detection and normalization

**Type:** enhancement  
**Status:** done  
**Branch:** feat/cosmos-address-detection  
**Linked roadmap section:** v0 - EVM/BTC/Solana/Cosmos/Substrate coverage

---

## 🧠 Context
The Cosmos ecosystem uses bech32 encoding with Human Readable Part (HRP) prefixes to identify different chains. Examples: `cosmos1...` (Cosmos Hub), `osmo1...` (Osmosis), `atom1...` (Cosmos Hub alternative), `juno1...` (Juno), etc. All Cosmos addresses use bech32 encoding with chain-specific HRPs. The address format is consistent across the ecosystem, only the HRP differs.

## 🎯 Goal
Implement Cosmos ecosystem address detection, validation, and normalization in the foxchain-id crate. Support major Cosmos chains through HRP detection and support extensible HRP mapping.

## 📏 Success Metrics
- [ ] Detect Cosmos addresses (bech32 with known HRPs)
- [ ] Validate bech32 encoding and checksum
- [ ] Identify chain from HRP (cosmos, osmo, juno, atom, etc.)
- [ ] Support at least 10 major Cosmos chains
- [ ] Normalize addresses (extract and validate HRP)
- [ ] Return appropriate `Chain` variant for detected chain

## 🧩 Acceptance Criteria
- [ ] `identify()` function detects Cosmos addresses correctly
- [ ] Bech32 validation implemented
- [ ] HRP-to-chain mapping for major chains
- [ ] Chain identification from HRP
- [ ] Address normalization
- [ ] Extensible HRP mapping (easy to add new chains)
- [ ] Comprehensive test coverage (multiple chains, invalid formats)
- [ ] Documentation with examples
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/cosmos-address-detection`
2. Add dependencies: `bech32`
3. Create `formats/cosmos.rs` module
4. Implement bech32 address detection
5. Implement HRP extraction and validation
6. Create HRP-to-chain mapping (cosmos, osmo, juno, atom, akash, stargaze, etc.)
7. Add Cosmos chain variants to `Chain` enum (CosmosHub, Osmosis, Juno, etc.)
8. Implement chain identification from HRP
9. Implement address normalization
10. Add comprehensive tests for multiple chains
11. Update `identify()` function to use Cosmos detector
12. Move this file to `in_progress/` then `done/`
13. Create PR referencing this issue

## 🔍 Alternatives Considered
- Single Cosmos chain → Rejected: Ecosystem has many chains, need multi-chain support
- Generic bech32 without HRP mapping → Rejected: Chain identification is important

## ⚠️ Risks / Mitigations
- Many Cosmos chains → Mitigation: Start with major chains, make HRP mapping extensible
- HRP variations → Mitigation: Support common variations, document mapping

## 🔗 Discussion Notes
Cosmos ecosystem is large with many chains. Start with major chains (Cosmos Hub, Osmosis, Juno) and make it easy to extend with more chains.

