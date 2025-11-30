# Implement proper SS58 checksum validation using Blake2b

**Type:** enhancement  
**Status:** in_progress  
**Created:** 2025-11-30  
**Source:** Post-merge analysis of Substrate address detection implementation

---

## 🧠 Context

The current Substrate address detection implementation validates the structure of SS58 addresses (prefix + 32-byte account ID + 2-byte checksum) but does not verify the Blake2b checksum. This is a security and correctness issue, as invalid addresses with correct structure but wrong checksum will be accepted.

The SS58 checksum algorithm uses Blake2b-512 hash:
1. Take prefix + account ID
2. Compute `blake2b_512(SS58PRE + prefix + account_id)` where `SS58PRE = "SS58PRE"` (0x53533538505245)
3. Take first bytes as checksum (length depends on address length)
4. Verify checksum matches

## 🎯 Goal

Implement proper SS58 checksum validation using Blake2b hash to ensure only valid SS58 addresses are accepted.

## 📏 Success Metrics

- [ ] Add `blake2` dependency (or use existing if available)
- [ ] Implement SS58 checksum calculation
- [ ] Implement SS58 checksum verification
- [ ] Update `detect_substrate()` to validate checksum
- [ ] Add tests for valid checksums
- [ ] Add tests for invalid checksums (should be rejected)
- [ ] Update documentation

## 🧩 Acceptance Criteria

- [ ] SS58 checksum validation implemented using Blake2b
- [ ] Invalid addresses with wrong checksum are rejected
- [ ] Valid addresses with correct checksum are accepted
- [ ] Comprehensive test coverage
- [ ] No performance regression
- [ ] Documentation updated

## 🛠️ Implementation Outline

1. Add `blake2` dependency to `Cargo.toml` (or verify if already available)
2. Implement `calculate_ss58_checksum()` function
3. Implement `validate_ss58_checksum()` function
4. Update `detect_substrate()` to call checksum validation
5. Add tests with real SS58 addresses (Polkadot, Kusama)
6. Add tests with invalid checksums
7. Update documentation

## 🔍 Alternatives Considered

- Use `ss58` crate → Rejected: We want to keep dependencies minimal, but this could be reconsidered
- Skip checksum validation → Rejected: Security and correctness issue

## ⚠️ Risks / Mitigations

- Performance impact → Mitigation: Blake2b is fast, minimal overhead
- Complexity → Mitigation: Well-documented algorithm, can reference Substrate docs

## 🔗 Discussion Notes

Current implementation has a comment: "Note: Full SS58 validation requires blake2b checksum verification. This is a simplified implementation that validates structure. For production use, consider using the `ss58` crate for proper validation."

This proposal addresses this limitation.

## 📚 References

- [SS58 Address Format](https://docs.substrate.io/reference/address-formats/)
- [Blake2b Hash Function](https://www.blake2.net/)
- Current implementation: `crates/foxchain-id/src/formats/substrate.rs:73-75`

