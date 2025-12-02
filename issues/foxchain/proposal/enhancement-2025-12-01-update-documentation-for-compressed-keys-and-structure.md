# Update documentation for compressed keys and code structure

**Type:** enhancement  
**Status:** proposal  
**Created:** 2025-12-01  
**Source:** Manual review

---

## 🧠 Context

After implementing compressed public key decompression support and restructuring the codebase, the documentation needs to be updated to reflect:

1. **Compressed key support**: The library now supports deriving addresses from compressed secp256k1 public keys (33 bytes), but this is not clearly documented
2. **Code structure**: The new modular structure (`shared/`, `address/detection/`, `public_key/detection/`, `public_key/derivation/`) is not documented
3. **Examples**: Missing examples showing the structure of `ChainCandidate` results after `identify()` calls
4. **API documentation**: Need better examples of how to work with identification results

## 🎯 Goal

Update all documentation to accurately reflect current functionality:

1. Update README.md to mention compressed key support explicitly
2. Update EVM and Bitcoin address documentation to explain compressed key support
3. Add code structure documentation explaining the modular architecture
4. Add comprehensive examples showing `ChainCandidate` structure and how to work with results
5. Update docs/README.md to include public key derivation documentation

## 📏 Success Metrics

- [ ] README.md mentions compressed key decompression support
- [ ] EVM addresses doc explains compressed key support
- [ ] Bitcoin addresses doc explains compressed key support
- [ ] Code structure/architecture is documented
- [ ] Examples show `ChainCandidate` structure with confidence scores and reasoning
- [ ] Examples demonstrate working with multiple candidates
- [ ] Public key derivation examples are included

## 🧩 Acceptance Criteria

- [ ] All new features (compressed keys) are documented
- [ ] Code structure is explained for developers
- [ ] Examples are clear and demonstrate real-world usage
- [ ] Examples show actual output structure (candidates, confidence, reasoning)
- [ ] Documentation is consistent across all files

## 🛠️ Implementation Outline

1. **Update main README.md**
   - Add explicit mention of compressed key support
   - Add architecture/structure section
   - Enhance examples to show candidate structure

2. **Update EVM addresses documentation**
   - Add section on compressed public key support
   - Update encoding process to mention automatic decompression

3. **Update Bitcoin addresses documentation**
   - Add section on compressed public key support
   - Update encoding process to mention automatic decompression

4. **Update docs/README.md**
   - Add section on public key detection and derivation
   - Link to relevant documentation

5. **Add comprehensive examples**
   - Create examples showing `ChainCandidate` structure with actual output
   - Show how to filter by confidence
   - Show how to access reasoning
   - Show multi-chain scenarios (EVM addresses return multiple candidates)
   - Show examples with actual confidence scores and reasoning strings
   - Add examples for public key detection showing derived addresses

## 🔍 Alternatives Considered

- **Keep current documentation** → Rejected: Missing important features and confusing for users
- **Only update README** → Rejected: Need comprehensive updates across all docs

## ⚠️ Risks / Mitigations

- **Documentation drift** → Mitigation: Review all docs systematically
- **Outdated examples** → Mitigation: Test all examples before committing
- **Missing information** → Mitigation: Cross-reference with code implementation

## 📚 References

- Current README.md
- Current docs/evm-addresses.md
- Current docs/bitcoin-addresses.md
- Current code structure in src/

