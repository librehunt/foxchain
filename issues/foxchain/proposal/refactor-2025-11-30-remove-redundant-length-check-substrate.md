# Remove redundant length check in Substrate detection

**Type:** refactor  
**Status:** proposal  
**Created:** 2025-11-30  
**Source:** Post-merge analysis of SS58 two-byte prefix decoding implementation

---

## 🧠 Context

The `detect_substrate()` function in `crates/foxchain-id/src/formats/substrate.rs` has a redundant length check. The function checks `decoded.len() < 35` at line 97 and again at line 110. The second check is unnecessary since we've already validated the length earlier.

## 🎯 Goal

Remove the redundant length check to improve code clarity and eliminate unnecessary validation.

## 📏 Success Metrics

- [ ] Remove redundant `decoded.len() < 35` check at line 110
- [ ] Verify tests still pass
- [ ] No functional changes

## 🧩 Acceptance Criteria

- [ ] Redundant check removed
- [ ] Function still works correctly
- [ ] All tests pass
- [ ] Code is cleaner

## 🛠️ Implementation Outline

1. Review `detect_substrate()` function
2. Remove redundant length check at line 110
3. Verify tests still pass
4. Update if needed

## 🔍 Alternatives Considered

- Keep redundant check → Rejected: Unnecessary code, reduces clarity
- Add comment explaining why → Rejected: Better to remove redundant code

## ⚠️ Risks / Mitigations

- Minimal risk: This is a refactoring task with no functional changes

## 🔗 Discussion Notes

Current implementation has:
- Line 97: `if decoded.len() < 35 || decoded.len() > 50 { return Ok(None); }`
- Line 110: `if decoded.len() < 35 { return Ok(None); }`

The check at line 110 is redundant since we've already validated the length at line 97.

## 📚 References

- Current implementation: `crates/foxchain-id/src/formats/substrate.rs:97,110`


