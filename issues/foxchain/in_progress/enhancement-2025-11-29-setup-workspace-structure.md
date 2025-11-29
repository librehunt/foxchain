# Setup workspace structure for foxchain-id and foxchain-analysis crates

**Type:** enhancement  
**Status:** in_progress  
**Branch:** feat/setup-workspace-structure  
**Linked roadmap section:** v0 - Initial setup

---

## 🧠 Context
The project needs to be restructured as a Cargo workspace containing two crates (foxchain-id and foxchain-analysis) plus a root library that exposes both. Currently, the project is a single package. According to the README, foxchain will provide:
- Identification crate (foxchain-id): detect blockchain(s) from input strings
- Analysis crate (foxchain-analysis): retrieve on-chain data for identified wallets
- Root library (foxchain): exposes both crates as a single entry point

## 🎯 Goal
Convert the current single-package structure to a Cargo workspace with:
1. Root workspace Cargo.toml
2. `crates/foxchain-id/` crate with basic structure
3. `crates/foxchain-analysis/` crate with basic structure
4. Root `src/lib.rs` that re-exports both crates
5. Maintain backward compatibility where possible

## 📏 Success Metrics
- [ ] Workspace Cargo.toml created with both crates as members
- [ ] foxchain-id crate scaffold created (Cargo.toml, src/lib.rs)
- [ ] foxchain-analysis crate scaffold created (Cargo.toml, src/lib.rs)
- [ ] Root lib.rs re-exports both crates
- [ ] `cargo check` passes for workspace
- [ ] `cargo build` succeeds for all crates

## 🧩 Acceptance Criteria
- [ ] Workspace structure follows Rust workspace best practices
- [ ] Both crates have proper Cargo.toml metadata (name, version, edition, license)
- [ ] Root library can be used to access both crates
- [ ] All crates build successfully
- [ ] No breaking changes to existing API (if any exists)
- [ ] CHANGELOG entry added

## 🛠️ Implementation Outline
1. Create/switch to branch `feat/setup-workspace-structure`
2. Convert root Cargo.toml to workspace format
3. Create `crates/foxchain-id/` directory structure
4. Create `crates/foxchain-id/Cargo.toml` with proper metadata
5. Create `crates/foxchain-id/src/lib.rs` with basic module structure
6. Create `crates/foxchain-analysis/` directory structure
7. Create `crates/foxchain-analysis/Cargo.toml` with proper metadata
8. Create `crates/foxchain-analysis/src/lib.rs` with basic module structure
9. Update root `src/lib.rs` to re-export both crates
10. Verify workspace builds successfully
11. Move this file to `in_progress/` then `done/`
12. Create PR referencing this issue

## 🔍 Alternatives Considered
- Keep single package → Rejected: Need separation of concerns and independent versioning
- Separate repositories → Rejected: Workspace provides better integration and shared dependencies

## ⚠️ Risks / Mitigations
- Breaking existing code → Mitigation: Root lib re-exports maintain compatibility
- Dependency management complexity → Mitigation: Use workspace dependencies where appropriate

## 🔗 Discussion Notes
This is the foundational structure for the project. The workspace allows both crates to be developed independently while maintaining a unified entry point through the root library.

