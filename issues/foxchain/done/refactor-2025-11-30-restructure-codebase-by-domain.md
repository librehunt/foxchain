# Restructure codebase by domain (address, public_key, transaction) with shared utilities

**Type:** refactor  
**Status:** in_progress  
**Created:** 2025-11-30  
**Source:** Code organization improvement discussion

---

## 🧠 Context

The current codebase structure has all modules under `formats/` directory, which doesn't clearly show:
- Shared encoding utilities (Base58, Bech32, Hex) used across addresses, public keys, and future transactions
- Shared cryptographic functions (SHA256, Keccak, RIPEMD160, Blake2b) used across domains
- Shared checksum validation logic (Base58Check, Bech32, EIP-55, SS58)
- Domain separation between addresses, public keys, and future transaction support

The current flat structure in `formats/` makes it hard to see:
- What encoding/crypto utilities are shared
- How addresses and public keys relate
- Where to add transaction support in the future

## 🎯 Goal

Restructure the codebase to:
1. Separate by domain: `address/`, `public_key/`, `transaction/` (future)
2. Extract shared utilities: `shared/encoding/`, `shared/crypto/`, `shared/checksum/`, `shared/normalize/`
3. Make code reuse and dependencies visible in the file tree
4. Eliminate code duplication (e.g., Base58Check validation duplicated in Bitcoin and Tron)

## 📏 Success Metrics

- [ ] Create `shared/` directory with encoding, crypto, checksum, normalize modules
- [ ] Move address detection to `address/detection/`
- [ ] Move public key logic to `public_key/detection/` and `public_key/derivation/`
- [ ] Extract Base58Check validation to `shared/checksum/base58check.rs` (remove duplication)
- [ ] Extract Bech32 helpers to `shared/encoding/bech32.rs`
- [ ] Extract hash functions to `shared/crypto/hash.rs`
- [ ] Update all imports to use new structure
- [ ] All tests pass
- [ ] Code formatted and clippy clean
- [ ] Documentation updated

## 🧩 Acceptance Criteria

- [ ] No code duplication (Base58Check, Bech32 helpers, hash functions)
- [ ] Clear domain separation (address, public_key, transaction)
- [ ] Shared utilities clearly visible in `shared/` directory
- [ ] All existing functionality preserved
- [ ] All tests pass
- [ ] Code is more maintainable and extensible

## 🛠️ Implementation Outline

### Phase 1: Create shared utilities
1. Create `src/shared/` directory structure
2. Extract Base58Check validation from `bitcoin.rs` and `tron.rs` → `shared/checksum/base58check.rs`
3. Extract Bech32 helpers → `shared/encoding/bech32.rs`
4. Extract hash functions (SHA256, Keccak, RIPEMD160, Blake2b) → `shared/crypto/hash.rs`
5. Extract secp256k1 and Ed25519 helpers → `shared/crypto/secp256k1.rs` and `shared/crypto/ed25519.rs`
6. Extract normalization logic → `shared/normalize/`

### Phase 2: Restructure address domain
1. Create `src/address/detection/` directory
2. Move address detection modules:
   - `formats/evm.rs` → `address/detection/evm.rs`
   - `formats/bitcoin.rs` → `address/detection/bitcoin.rs`
   - `formats/tron.rs` → `address/detection/tron.rs`
   - `formats/solana.rs` → `address/detection/solana.rs`
   - `formats/cosmos.rs` → `address/detection/cosmos.rs`
   - `formats/substrate.rs` → `address/detection/substrate.rs`
   - `formats/cardano.rs` → `address/detection/cardano.rs`
3. Update imports to use `shared/` utilities
4. Create `address/mod.rs` with re-exports

### Phase 3: Restructure public key domain
1. Create `src/public_key/detection/` and `src/public_key/derivation/` directories
2. Split `formats/public_key.rs`:
   - Detection logic → `public_key/detection/mod.rs`
   - Derivation logic → `public_key/derivation/evm.rs`, `bitcoin.rs`, `solana.rs`, `cosmos.rs`
3. Update imports to use `shared/` utilities
4. Create `public_key/mod.rs` with re-exports

### Phase 4: Update main library
1. Update `src/lib.rs` to use new module structure
2. Update `src/formats/mod.rs` or remove it if no longer needed
3. Update all imports across the codebase

### Phase 5: Testing and validation
1. Run all tests to ensure nothing broke
2. Fix any import errors
3. Run `cargo fmt` and `cargo clippy`
4. Update documentation

## 📁 Proposed Structure

```
src/
├── lib.rs
├── types.rs                    # Chain, Error, IdentificationResult, ChainCandidate
│
├── shared/                     # ⭐ Shared utilities
│   ├── mod.rs
│   ├── encoding/
│   │   ├── mod.rs
│   │   ├── base58.rs           # Base58 encoding/decoding
│   │   ├── bech32.rs           # Bech32 encoding/decoding helpers
│   │   ├── hex.rs              # Hex encoding/decoding
│   │   └── ss58.rs             # SS58 encoding/decoding
│   ├── crypto/
│   │   ├── mod.rs
│   │   ├── hash.rs             # Hash functions (SHA256, Keccak, RIPEMD160, Blake2b)
│   │   ├── secp256k1.rs        # secp256k1 helpers
│   │   └── ed25519.rs          # Ed25519 helpers
│   ├── checksum/
│   │   ├── mod.rs
│   │   ├── base58check.rs      # Base58Check validation (shared by Bitcoin, Tron)
│   │   ├── bech32.rs           # Bech32 checksum validation
│   │   ├── eip55.rs            # EIP-55 checksum validation
│   │   └── ss58.rs             # SS58 checksum validation
│   └── normalize/
│       ├── mod.rs
│       ├── case.rs             # Case normalization (Bech32)
│       └── checksum.rs         # Checksum normalization (EIP-55)
│
├── address/                    # Address domain
│   ├── mod.rs
│   └── detection/
│       ├── mod.rs
│       ├── evm.rs              # Uses: shared/encoding/hex, shared/checksum/eip55
│       ├── bitcoin.rs          # Uses: shared/encoding/base58, shared/encoding/bech32, shared/checksum/base58check
│       ├── tron.rs             # Uses: shared/encoding/base58, shared/checksum/base58check
│       ├── solana.rs           # Uses: shared/encoding/base58
│       ├── cosmos.rs           # Uses: shared/encoding/bech32, shared/checksum/bech32
│       ├── substrate.rs        # Uses: shared/encoding/ss58, shared/checksum/ss58
│       └── cardano.rs          # Uses: shared/encoding/bech32, shared/checksum/bech32
│
├── public_key/                 # Public key domain
│   ├── mod.rs
│   ├── detection/
│   │   └── mod.rs              # Uses: shared/encoding/*, shared/crypto/*
│   └── derivation/
│       ├── mod.rs
│       ├── evm.rs              # Uses: shared/crypto/secp256k1, shared/crypto/hash
│       ├── bitcoin.rs          # Uses: shared/crypto/secp256k1, shared/crypto/hash
│       ├── solana.rs           # Uses: shared/crypto/ed25519
│       └── cosmos.rs           # Uses: shared/crypto/ed25519, shared/crypto/hash
│
└── transaction/                # Future: Transaction domain
    └── mod.rs                  # (placeholder for future work)
```

## 🔍 Alternatives Considered

- Keep current `formats/` structure → Rejected: Doesn't show shared utilities, harder to extend
- Structure by encoding type only → Rejected: Doesn't separate domains (address vs public_key)
- Structure by function only → Rejected: Doesn't show domain separation

## ⚠️ Risks / Mitigations

- Breaking changes to public API → Mitigation: Use re-exports in `mod.rs` files to maintain compatibility
- Import errors during migration → Mitigation: Do migration incrementally, test after each phase
- Missing shared code → Mitigation: Review all modules carefully before extracting
- Test failures → Mitigation: Run tests after each phase, fix immediately

## 🔗 Discussion Notes

This refactoring will make the codebase:
- More maintainable: Clear separation of concerns
- More extensible: Easy to add transaction support
- Less duplicated: Shared utilities in one place
- More discoverable: File tree shows dependencies and relationships

The `shared/` directory name was chosen over `common/` for better expressivity.

## 📚 References

- Current structure: `crates/foxchain-id/src/formats/`
- Discussion about structure options and shared elements

