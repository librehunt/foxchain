# Implement proper SS58 two-byte prefix decoding

**Type:** enhancement  
**Status:** done  
**Created:** 2025-11-30  
**Source:** Post-merge analysis of Substrate address detection implementation

---

## 🧠 Context

The current Substrate address detection implementation handles two-byte prefixes (64-16383) but uses only the first byte for chain identification instead of properly decoding the two-byte prefix. This limits accurate chain identification for chains using two-byte prefixes.

SS58 two-byte prefix encoding:
- First byte: `0x40 + (prefix >> 8) & 0x3f` (bits 8-13 of prefix)
- Second byte: `prefix & 0xff` (bits 0-7 of prefix)
- Decoding: `prefix = ((first_byte & 0x3f) << 8) | second_byte`

## 🎯 Goal

Implement proper SS58 two-byte prefix decoding to accurately identify chains using two-byte prefixes.

## 📏 Success Metrics

- [ ] Implement two-byte prefix decoding function
- [ ] Update prefix extraction logic in `detect_substrate()`
- [ ] Support chains with two-byte prefixes
- [ ] Add tests for two-byte prefix addresses
- [ ] Update prefix-to-chain mapping if needed

## 🧩 Acceptance Criteria

- [ ] Two-byte prefixes are correctly decoded
- [ ] Chain identification works for two-byte prefix chains
- [ ] Single-byte prefixes still work correctly
- [ ] Comprehensive test coverage
- [ ] Documentation updated

## 🛠️ Implementation Outline

1. Implement `decode_ss58_prefix()` function to handle both single and two-byte prefixes
2. Update `detect_substrate()` to use proper prefix decoding
3. Research and add common two-byte prefix chains to mapping
4. Add tests for two-byte prefix addresses
5. Update documentation

## 🔍 Alternatives Considered

- Keep current simplified approach → Rejected: Limits accuracy for many Substrate chains
- Use `ss58` crate → Could be considered, but we want to keep control

## ⚠️ Risks / Mitigations

- Breaking changes → Mitigation: Single-byte prefixes should still work
- Complexity → Mitigation: Well-documented encoding format

## 🔗 Discussion Notes

Current implementation comment: "For simplicity, we'll use the first byte for chain identification. Full implementation would decode the two-byte prefix properly."

This proposal addresses this limitation.

## 📚 References

- [SS58 Address Format](https://docs.substrate.io/reference/address-formats/)
- Current implementation: `crates/foxchain-id/src/formats/substrate.rs:46-52`

