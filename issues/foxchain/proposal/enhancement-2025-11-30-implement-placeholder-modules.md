# Implement placeholder modules in shared/

**Type:** enhancement  
**Status:** proposal  
**Created:** 2025-11-30  
**Source:** Post-merge introspection

---

## 🧠 Context

During the codebase restructuring, several placeholder modules were created in `shared/` that contain only comments:
- `shared/crypto/secp256k1.rs` - Placeholder for secp256k1 helpers
- `shared/crypto/ed25519.rs` - Placeholder for Ed25519 helpers
- `shared/checksum/bech32.rs` - Placeholder for Bech32 checksum helpers
- `shared/normalize/case.rs` - Very basic implementation (just `to_lowercase()`)
- `shared/normalize/checksum.rs` - Just a comment

These placeholders were created to establish the structure, but they should either be implemented with useful functionality or removed if not needed.

## 🎯 Goal

Implement useful functionality in placeholder modules or remove them if not needed:
1. Add secp256k1 helpers (key compression/decompression, validation)
2. Add Ed25519 helpers (key validation, signature verification helpers)
3. Add Bech32 checksum validation helpers (if needed beyond what bech32 crate provides)
4. Enhance normalize modules with more comprehensive normalization functions
5. Remove placeholders that aren't needed

## 📏 Success Metrics

- [ ] All placeholder modules either implemented or removed
- [ ] Useful helper functions added where appropriate
- [ ] No empty/comment-only modules remain
- [ ] All functionality is tested

## 🧩 Acceptance Criteria

- [ ] secp256k1 module has useful helpers (if needed)
- [ ] Ed25519 module has useful helpers (if needed)
- [ ] Bech32 checksum module has useful helpers (if needed)
- [ ] Normalize modules have comprehensive functions
- [ ] All modules are tested
- [ ] No dead code or empty modules

## 🛠️ Implementation Outline

1. Review each placeholder module to determine if it's needed
2. For modules that are needed:
   - Implement useful helper functions
   - Add comprehensive tests
   - Update documentation
3. For modules that aren't needed:
   - Remove the module
   - Update imports
4. Ensure all modules follow consistent patterns

## 🔍 Alternatives Considered

- Keep placeholders for future use → Rejected: Creates confusion, better to implement or remove
- Remove all placeholders → Rejected: Some may be useful (e.g., secp256k1 helpers)

## ⚠️ Risks / Mitigations

- Over-engineering → Mitigation: Only implement what's actually needed
- Breaking changes → Mitigation: Use feature flags if needed

