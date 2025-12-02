# Validate Documentation Examples Against Implementation

**Type**: Enhancement ✨  
**Created**: 2025-12-01  
**Status**: Proposal

## Summary

The `docs/examples.md` file contains expected outputs (confidence scores, reasoning strings, normalized addresses) that should match the actual implementation. Currently, there's no automated way to verify that these documented examples are accurate.

## Problem

The documentation shows expected outputs like:
- Confidence scores (e.g., "0.95", "0.85", "0.90")
- Reasoning strings (e.g., "Valid EVM address with EIP-55 checksum")
- Normalized addresses (e.g., "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045")
- Number of candidates (e.g., "10 total candidates")

However:
- These values are manually written and may not match actual implementation
- Changes to confidence calculation logic may not be reflected in docs
- Reasoning strings may differ from actual output
- No automated validation exists

## Proposed Solution

Create integration tests that:
1. **Extract test cases from documentation**: Parse `docs/examples.md` to identify test inputs and expected outputs
2. **Run actual identification**: Call `identify()` with documented inputs
3. **Validate outputs**: Compare actual results against documented expectations:
   - Confidence scores (with tolerance for floating point)
   - Reasoning strings (exact match or pattern matching)
   - Normalized addresses (exact match)
   - Number of candidates (exact match)
4. **Report discrepancies**: Fail tests if documented examples don't match implementation

## Implementation Approach

### Option 1: Markdown-based Test Extraction

Parse `docs/examples.md` to extract:
- Code blocks with `identify()` calls
- Commented expected outputs
- Assertions about confidence, reasoning, etc.

### Option 2: Structured Test Cases

Create a separate test file that mirrors the examples but in a structured format:

```rust
// crates/foxchain-id/tests/doc_examples.rs
use foxchain_id::{identify, Chain};

#[test]
fn test_evm_address_example_from_docs() {
    let input = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
    let result = identify(input).unwrap();
    
    // From docs/examples.md line 18
    assert_eq!(result.normalized, "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    
    // From docs/examples.md line 29-33
    let ethereum = result.candidates.iter()
        .find(|c| c.chain == Chain::Ethereum)
        .unwrap();
    assert_eq!(ethereum.confidence, 0.85); // Actual: 0.85 (no checksum)
    assert_eq!(ethereum.reasoning, "Valid EVM address format (lowercase, no checksum)");
    
    // From docs/examples.md line 33
    assert_eq!(result.candidates.len(), 10);
}
```

### Option 3: Snapshot Testing

Use snapshot testing to capture actual outputs and compare against documented examples:

```rust
#[test]
fn test_examples_snapshot() {
    let test_cases = vec![
        ("0xd8da6bf26964af9d7eed9e03e53415d37aa96045", "evm_address"),
        ("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", "bitcoin_address"),
        // ...
    ];
    
    for (input, name) in test_cases {
        let result = identify(input).unwrap();
        insta::assert_json_snapshot!(name, result);
    }
}
```

## Benefits

- **Documentation accuracy**: Ensures examples match implementation
- **Regression detection**: Catches when implementation changes break documented behavior
- **Maintenance**: Makes it easier to update docs when behavior changes
- **Confidence**: Users can trust that examples are accurate

## Related Files

- `crates/foxchain-id/docs/examples.md` - Documentation to validate
- `crates/foxchain-id/src/lib.rs` - Implementation to validate against
- `crates/foxchain-id/src/address/detection/*.rs` - Detection logic

## Acceptance Criteria

- [ ] Integration tests created that validate examples from `docs/examples.md`
- [ ] Tests verify confidence scores match documented values (within tolerance)
- [ ] Tests verify reasoning strings match documented values
- [ ] Tests verify normalized addresses match documented values
- [ ] Tests verify candidate counts match documented values
- [ ] CI runs these validation tests on every build
- [ ] Tests fail if documentation doesn't match implementation
- [ ] Documentation updated to note that examples are validated

