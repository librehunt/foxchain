# Implement EVM address detection and normalization

**Type:** enhancement  
**Status:** done  
**Branch:** feat/evm-address-detection  
**Linked roadmap section:** v0 - EVM/BTC/Solana/Cosmos/Substrate coverage

---

## 🧠 Context
EVM (Ethereum Virtual Machine) addresses are used across many blockchain networks including Ethereum, Polygon, BSC, Avalanche, Arbitrum, Optimism, and many others. They follow a common format: 0x followed by 40 hex characters (20 bytes). The EIP-55 standard provides checksum validation. This is the highest priority implementation as EVM addresses are the most common multi-chain format.

## 🎯 Goal
Implement robust EVM address detection, validation, and normalization in the foxchain-id crate. Support all major EVM-compatible chains with proper confidence scoring for multi-chain candidates.

## 📏 Success Metrics
- [ ] Detect EVM addresses (0x + 40 hex chars)
- [ ] Validate EIP-55 checksum when present
- [ ] Normalize addresses to EIP-55 checksum format
- [ ] Return multiple EVM chain candidates with confidence scores
- [ ] Handle both lowercase and checksummed addresses
- [ ] Support at least 10 major EVM chains (Ethereum, Polygon, BSC, Avalanche, Arbitrum, Optimism, Base, Fantom, Celo, Gnosis)

## 🧩 Acceptance Criteria
- [ ] `identify()` function detects EVM addresses correctly
- [ ] EIP-55 checksum validation implemented
- [ ] Address normalization to checksum format
- [ ] Returns `Chain::Ethereum` and other EVM chains as candidates
- [ ] Confidence scores reflect checksum validity
- [ ] Comprehensive test coverage (valid addresses, invalid formats, checksum cases)
- [ ] Documentation with examples
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/evm-address-detection`
2. Add dependencies: `hex`, `keccak` (for EIP-55 checksum)
3. Create `formats/evm.rs` module
4. Implement EVM address format detection
5. Implement EIP-55 checksum validation
6. Implement address normalization
7. Add EVM chains to `Chain` enum (Ethereum, Polygon, BSC, Avalanche, Arbitrum, Optimism, Base, Fantom, Celo, Gnosis)
8. Implement multi-chain candidate generation for EVM addresses
9. Add comprehensive tests
10. Update `identify()` function to use EVM detector
11. Move this file to `in_progress/` then `done/`
12. Create PR referencing this issue

## 🔍 Alternatives Considered
- Single chain detection → Rejected: EVM addresses are valid across many chains, need multi-chain support
- No checksum validation → Rejected: EIP-55 is standard and improves confidence scoring

## ⚠️ Risks / Mitigations
- Checksum validation complexity → Mitigation: Use well-tested keccak library
- Too many chain candidates → Mitigation: Use confidence scores, allow filtering

## 🔗 Discussion Notes
EVM addresses are the most common format and should be implemented first. The multi-chain nature requires returning all EVM-compatible chains as candidates with appropriate confidence scores.

