# Complete public_key module structure

**Type:** refactor  
**Status:** done  
**Created:** 2025-11-30  
**Source:** Post-merge introspection

---

## 🧠 Context

During the codebase restructuring, the `public_key` module was moved from `formats/public_key.rs` to `public_key/mod.rs`, but the planned structure with separate `detection/` and `derivation/` subdirectories was not fully implemented. The current structure has all logic in a single `mod.rs` file, which doesn't match the intended domain separation.

The original plan was to split:
- Detection logic → `public_key/detection/mod.rs`
- Derivation logic → `public_key/derivation/evm.rs`, `bitcoin.rs`, `solana.rs`, `cosmos.rs`

## 🎯 Goal

Complete the public_key module restructuring to match the intended architecture:
1. Split detection logic into `public_key/detection/mod.rs`
2. Split derivation logic into separate files: `public_key/derivation/evm.rs`, `bitcoin.rs`, `solana.rs`, `cosmos.rs`
3. Update imports to use `shared/` utilities consistently
4. Maintain all existing functionality

## 📏 Success Metrics

- [ ] `public_key/detection/mod.rs` contains only detection logic
- [ ] `public_key/derivation/` contains separate files for each chain
- [ ] All derivation functions use `shared/` utilities
- [ ] All tests pass
- [ ] Code is more maintainable and follows domain separation

## 🧩 Acceptance Criteria

- [ ] Clear separation between detection and derivation
- [ ] Each derivation file is focused on a single chain
- [ ] No code duplication
- [ ] All existing functionality preserved
- [ ] All tests pass

## 🛠️ Implementation Outline

1. Create `public_key/detection/mod.rs` with detection functions
2. Create `public_key/derivation/mod.rs` with re-exports
3. Create `public_key/derivation/evm.rs` for EVM address derivation
4. Create `public_key/derivation/bitcoin.rs` for Bitcoin address derivation
5. Create `public_key/derivation/solana.rs` for Solana address derivation
6. Create `public_key/derivation/cosmos.rs` for Cosmos address derivation
7. Update `public_key/mod.rs` to re-export from detection and derivation
8. Update all imports
9. Run tests and fix any issues

## 🔍 Alternatives Considered

- Keep current single-file structure → Rejected: Doesn't match intended architecture
- Only split detection/derivation without chain separation → Rejected: Less maintainable

## ⚠️ Risks / Mitigations

- Breaking changes → Mitigation: Use re-exports to maintain API compatibility
- Test failures → Mitigation: Run tests after each step

