# Add missing address format detectors

**Type:** enhancement  
**Status:** proposal  
**Created:** 2025-11-30  
**Source:** Post-merge introspection

---

## 🧠 Context

In `lib.rs`, there's a TODO comment indicating that other format detectors should be added:
```rust
// TODO: Add other format detectors (TON, Algorand, Near, etc.)
```

The current implementation supports:
- EVM chains (Ethereum, Polygon, BSC, etc.)
- Bitcoin ecosystem (Bitcoin, Litecoin, Dogecoin)
- Solana
- Tron
- Cosmos ecosystem
- Substrate/Polkadot ecosystem
- Cardano

But several important blockchains are missing.

## 🎯 Goal

Add address detection for missing blockchain formats:
1. TON (The Open Network) - Base64-like addresses
2. Algorand - Base32 addresses
3. Near Protocol - Base58 addresses with specific prefixes
4. Other important chains as identified

## 📏 Success Metrics

- [ ] TON address detection implemented
- [ ] Algorand address detection implemented
- [ ] Near Protocol address detection implemented
- [ ] All new detectors follow the same pattern as existing ones
- [ ] All new detectors use `shared/` utilities where possible
- [ ] Comprehensive test coverage

## 🧩 Acceptance Criteria

- [ ] New address detectors in `address/detection/`
- [ ] Proper normalization for each format
- [ ] Chain identification with confidence scores
- [ ] Tests with real addresses
- [ ] Documentation updated
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline

1. Research each blockchain's address format specification
2. Create detection module in `address/detection/`
3. Implement detection logic using `shared/` utilities
4. Add normalization logic
5. Add chain identification
6. Write comprehensive tests
7. Update `lib.rs` to include new detectors
8. Update documentation

## 🔍 Alternatives Considered

- Keep TODO for future → Rejected: These are important chains that should be supported
- Add all at once → Rejected: Better to add incrementally with proper testing

## ⚠️ Risks / Mitigations

- Incorrect format detection → Mitigation: Use official specifications and test with real addresses
- Breaking changes → Mitigation: Add as new functionality, don't modify existing

