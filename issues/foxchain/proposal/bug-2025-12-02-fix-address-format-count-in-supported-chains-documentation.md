# Fix Address Format Count in Supported Chains Documentation

**Type**: Bug 🐞  
**Created**: 2025-12-02  
**Status**: Proposal

## Summary

The `supported-chains.md` documentation incorrectly states there are "7 address format types" when it actually documents 8 distinct address formats.

## Problem

The documentation has an inconsistency:
- Section header says: "## Address Format Types (7)"
- Summary statistics say: "- **Address Format Types**: 7"
- But the document actually lists 8 formats:
  1. EVM Address Format
  2. Bitcoin P2PKH Format
  3. Bitcoin P2SH Format
  4. Bitcoin Bech32 Format
  5. Solana Address Format
  6. Tron Address Format
  7. Cosmos/Cardano Bech32 Format
  8. Substrate SS58 Format

This creates confusion and makes the documentation inaccurate.

## Proposed Solution

Update the documentation to correctly reflect 8 address format types:
1. Change section header from "Address Format Types (7)" to "Address Format Types (8)"
2. Update summary statistics from "Address Format Types: 7" to "Address Format Types: 8"

## Related Files

- `crates/foxchain-id/docs/supported-chains.md` - Lines 7, 86, and 250

## Acceptance Criteria

- [ ] Section header updated to show "(8)" instead of "(7)"
- [ ] Summary statistics updated to show "8" instead of "7"
- [ ] Documentation is accurate and consistent

