# Master Plan: Metadata-Driven Detection Pipeline Refactor

**Type**: Refactor 🔧  
**Created**: 2025-12-02  
**Status**: done  
**Priority**: High

## Summary

This is the master issue tracking the complete refactoring of the detection pipeline from hardcoded heuristics to a metadata-driven architecture. This refactor enables:

- Adding new chains by defining metadata only (no code changes)
- Automatic category grouping (no manual categories)
- Support for ambiguous inputs (multiple valid matches)
- Extensibility for future types (transactions, blocks, private keys)

## Related Issues

This refactor is broken down into 10 specific issues:

1. **refactor-2025-12-02-01** - Create Metadata Structures
2. **refactor-2025-12-02-02** - Create InputCharacteristics Extractor
3. **refactor-2025-12-02-03** - Create CategorySignature System
4. **refactor-2025-12-02-04** - Create Registry with Automatic Grouping
5. **refactor-2025-12-02-05** - Create Unified Address Detector
6. **refactor-2025-12-02-06** - Create Unified Public Key Detector
7. **refactor-2025-12-02-07** - Refactor identify() Pipeline
8. **refactor-2025-12-02-08** - Migrate Format Definitions to Metadata
9. **refactor-2025-12-02-09** - Update Tests for Multiple Matches
10. **refactor-2025-12-02-10** - Remove Old Detection Code

## Implementation Order

### Phase 1: Foundation (Issues 1-4)
Build the infrastructure:
1. Metadata structures
2. Input analysis
3. Category signatures
4. Registry system

### Phase 2: Detectors (Issues 5-6)
Create unified detectors:
5. Address detector
6. Public key detector

### Phase 3: Integration (Issues 7-8)
Wire everything together:
7. Refactor identify() pipeline
8. Migrate format definitions

### Phase 4: Testing & Cleanup (Issues 9-10)
Finalize:
9. Update tests
10. Remove old code

## Architecture Overview

```
Input String
    ↓
InputCharacteristics (extract)
    ↓
CategorySignature (derive)
    ↓
Registry.get_candidates() (lookup)
    ↓
Filter by Metadata (matches_metadata)
    ↓
Unified Detectors (detect_address/detect_public_key)
    ↓
Vec<IdentificationCandidate> (sorted by confidence)
```

## Key Design Decisions

1. **Vec-based fields**: Empty `Vec` means "no requirement" (no `Option` needed)
2. **Multiple formats per chain**: `Vec<AddressMetadata>` in `ChainMetadata`
3. **Automatic grouping**: `CategorySignature` eliminates manual categories
4. **Multiple matches**: Return all valid candidates, sorted by confidence
5. **Metadata-driven**: All format logic in metadata, detectors only validate

## Success Criteria

- [ ] All 10 sub-issues completed
- [ ] All existing tests pass
- [ ] New tests for ambiguous inputs pass
- [ ] Performance is acceptable (no significant regression)
- [ ] Adding a new chain requires only metadata definition
- [ ] Documentation updated

## Notes

- This is a large refactor - implement incrementally
- Keep old code working until new system is complete
- Consider feature flag for gradual rollout
- Maintain backward compatibility where possible (e.g., `identify_first()` helper)

