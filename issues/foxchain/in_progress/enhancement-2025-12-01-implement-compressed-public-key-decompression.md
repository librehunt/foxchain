# Implement compressed public key decompression

**Type:** enhancement  
**Status:** in_progress  
**Created:** 2025-12-01  
**Source:** Post-merge introspection

---

## 🧠 Context

After the public_key module refactoring, the codebase can detect compressed secp256k1 public keys (33 bytes with 0x02/0x03 prefix) but cannot derive addresses from them. Both `derive_evm_address` and `derive_bitcoin_addresses` currently skip compressed keys with comments like "Compressed keys need decompression - skip for now".

Compressed public keys are commonly used in Bitcoin and EVM ecosystems for efficiency (33 bytes vs 65 bytes). Supporting them would significantly improve the library's utility.

## 🎯 Goal

Implement secp256k1 compressed public key decompression to enable address derivation from compressed keys:

1. Add decompression function to `shared/crypto/secp256k1.rs`
2. Update `derive_evm_address` to handle compressed keys
3. Update `derive_bitcoin_addresses` to handle compressed keys
4. Add comprehensive tests for compressed key handling

## 📏 Success Metrics

- [ ] Compressed public keys (33 bytes) can be decompressed to uncompressed format (65 bytes)
- [ ] EVM address derivation works with compressed keys
- [ ] Bitcoin address derivation works with compressed keys
- [ ] All existing tests still pass
- [ ] New tests cover compressed key scenarios

## 🧩 Acceptance Criteria

- [ ] Decompression function correctly converts 33-byte compressed keys to 65-byte uncompressed keys
- [ ] Both 0x02 and 0x03 prefix compressed keys are supported
- [ ] EVM address derivation from compressed keys produces correct addresses
- [ ] Bitcoin address derivation from compressed keys produces correct addresses
- [ ] Error handling for invalid compressed keys
- [ ] Performance is acceptable (decompression is a mathematical operation)

## 🛠️ Implementation Outline

1. **Add secp256k1 decompression utility**
   - Create `shared/crypto/secp256k1.rs` with `decompress_public_key` function
   - Use secp256k1 library (e.g., `secp256k1` crate) for elliptic curve operations
   - Handle both 0x02 (even y) and 0x03 (odd y) prefixes

2. **Update EVM derivation**
   - Modify `derive_evm_address` to decompress compressed keys before hashing
   - Test with known compressed/uncompressed key pairs

3. **Update Bitcoin derivation**
   - Modify `derive_bitcoin_addresses` to decompress compressed keys
   - Test with known compressed/uncompressed key pairs

4. **Add tests**
   - Test decompression with 0x02 prefix keys
   - Test decompression with 0x03 prefix keys
   - Test EVM derivation with compressed keys
   - Test Bitcoin derivation with compressed keys
   - Test error cases (invalid compressed keys)

## 🔍 Alternatives Considered

- **Keep current limitation** → Rejected: Reduces library utility significantly
- **Use external library for decompression** → Preferred: Use well-tested `secp256k1` crate
- **Implement decompression manually** → Rejected: Complex and error-prone, better to use library

## ⚠️ Risks / Mitigations

- **Cryptographic errors** → Mitigation: Use well-tested `secp256k1` crate, add comprehensive tests
- **Performance impact** → Mitigation: Decompression is fast, only done when needed
- **Breaking changes** → Mitigation: This is an enhancement, not a breaking change
- **Dependency addition** → Mitigation: `secp256k1` is already a dependency

## 📚 References

- SEC 2: Recommended Elliptic Curve Domain Parameters
- Bitcoin BIP 32/39/44 (compressed key usage)
- Ethereum yellow paper (Keccak-256 hashing)
