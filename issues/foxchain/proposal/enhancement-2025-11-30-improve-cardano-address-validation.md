# Improve Cardano address validation with header parsing and canonical normalization

**Type:** enhancement  
**Status:** proposal  
**Created:** 2025-11-30  
**Source:** Code review and improvement suggestions

---

## 🧠 Context

The current Cardano address detection implementation validates HRP and data length but doesn't validate the actual Cardano address structure. Cardano addresses have a specific header byte that encodes address type and network ID according to CIP-19. The current implementation also uses simple `to_lowercase()` for normalization instead of re-encoding, which doesn't guarantee canonical format.

## 🎯 Goal

Improve Cardano address detection by:
1. Converting 5-bit groups to bytes and validating the header byte
2. Extracting address type and network ID from the header
3. Validating expected payload lengths for different address types
4. Producing canonical normalized addresses by re-encoding
5. Replacing arbitrary test vectors with real Cardano addresses

## 📏 Success Metrics

- [ ] Use `bech32::convert_bits` to convert 5-bit groups to bytes
- [ ] Parse and validate Cardano address header byte
- [ ] Extract address type (payment, stake, reward) and network ID (mainnet/testnet)
- [ ] Validate payload lengths according to address type
- [ ] Re-encode addresses for canonical normalization
- [ ] Add real Cardano address test vectors (mainnet and testnet)
- [ ] Add negative test vectors (cosmos, bc1, other Bech32 HRPs)
- [ ] Maintain or improve test coverage

## 🧩 Acceptance Criteria

- [ ] Header byte validation implemented
- [ ] Address type and network ID extraction working
- [ ] Payload length validation per address type
- [ ] Canonical normalization via re-encoding
- [ ] Real Cardano addresses used in tests
- [ ] No false positives with other Bech32 addresses
- [ ] All existing tests pass
- [ ] Code formatted and clippy clean

## 🛠️ Implementation Outline

1. Research Cardano address header byte structure (CIP-19)
2. Implement `bech32::convert_bits` to convert 5-bit groups to bytes
3. Parse header byte to extract:
   - Address type (payment, stake, reward)
   - Network ID (mainnet/testnet)
4. Validate payload lengths based on address type
5. Replace normalization: use `bech32::encode` instead of `to_lowercase()`
6. Add real Cardano address test vectors:
   - Mainnet payment addresses
   - Mainnet stake addresses
   - Testnet addresses
7. Add negative test vectors:
   - Cosmos addresses (should not match)
   - Bitcoin Bech32 addresses (bc1, tb1)
   - Other Bech32 HRPs
8. Update documentation

## 🔍 Alternatives Considered

- Keep current simple validation → Rejected: Too permissive, allows false positives
- Only validate length → Rejected: Doesn't validate actual Cardano structure
- Manual bit manipulation → Rejected: `convert_bits` is the standard approach

## ⚠️ Risks / Mitigations

- Header byte parsing complexity → Mitigation: Follow CIP-19 specification exactly
- Breaking changes to normalization → Mitigation: Test thoroughly, ensure backward compatibility
- Real address vectors may become invalid → Mitigation: Use well-known addresses from Cardano documentation

## 🔗 Discussion Notes

This improvement will make Cardano detection more accurate and prevent false positives. The header byte validation is crucial for proper Cardano address detection.

## 📚 References

- CIP-19: Cardano Address Format
- bech32 crate documentation: `convert_bits` function
- Cardano address specification

