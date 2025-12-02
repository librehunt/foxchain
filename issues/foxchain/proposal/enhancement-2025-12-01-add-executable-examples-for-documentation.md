# Add Executable Examples for Documentation

**Type**: Enhancement ✨  
**Created**: 2025-12-01  
**Status**: Proposal

## Summary

The `docs/examples.md` file contains comprehensive usage examples, but these examples are not executable or tested. This means:
1. Examples may become outdated if the API changes
2. Users cannot directly run the examples to verify behavior
3. No automated validation that examples match actual library behavior

## Problem

Currently, `docs/examples.md` contains static code examples with commented expected outputs. These examples:
- Are not compiled or executed
- May contain outdated confidence scores or reasoning strings
- Cannot be validated against actual library behavior
- Require manual verification when the code changes

## Proposed Solution

1. **Create executable examples** in `crates/foxchain-id/examples/` directory:
   - `basic_identification.rs` - Basic identification examples
   - `public_key_detection.rs` - Public key detection examples
   - `multi_chain_scenarios.rs` - Multi-chain scenario examples
   - `error_handling.rs` - Error handling examples

2. **Add integration tests** that validate the examples in `docs/examples.md`:
   - Extract key assertions from the documentation
   - Verify confidence scores match expected ranges
   - Verify reasoning strings match actual output
   - Verify normalized addresses match expected format

3. **Add doctests** to the main `identify` function and key types:
   - Include examples in doc comments that are automatically tested
   - Ensure examples stay in sync with implementation

## Benefits

- **Automated validation**: Examples are tested on every CI run
- **Better user experience**: Users can run examples directly
- **Documentation accuracy**: Examples stay in sync with code
- **Easier maintenance**: Changes to API will break examples, alerting maintainers

## Implementation Details

### Executable Examples Structure

```rust
// crates/foxchain-id/examples/basic_identification.rs
use foxchain_id::{identify, Chain};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // EVM address example
    let result = identify("0xd8da6bf26964af9d7eed9e03e53415d37aa96045")?;
    println!("Normalized: {}", result.normalized);
    println!("Candidates: {}", result.candidates.len());
    
    // Verify expected behavior
    assert!(result.candidates.len() >= 10);
    assert!(result.candidates.iter().any(|c| c.chain == Chain::Ethereum));
    
    Ok(())
}
```

### Integration Tests

```rust
// crates/foxchain-id/tests/examples_validation.rs
#[test]
fn test_evm_address_example() {
    let result = identify("0xd8da6bf26964af9d7eed9e03e53415d37aa96045").unwrap();
    
    // Verify from docs/examples.md
    assert_eq!(result.candidates.len(), 10);
    let ethereum = result.candidates.iter()
        .find(|c| c.chain == Chain::Ethereum)
        .unwrap();
    assert_eq!(ethereum.confidence, 0.85); // From actual implementation
    assert_eq!(ethereum.reasoning, "Valid EVM address format (lowercase, no checksum)");
}
```

## Related Files

- `crates/foxchain-id/docs/examples.md` - Current static examples
- `crates/foxchain-id/src/lib.rs` - Main `identify` function
- `Cargo.toml` - Will need `[[example]]` sections added

## Acceptance Criteria

- [ ] Executable examples created in `examples/` directory
- [ ] Integration tests validate examples from documentation
- [ ] Doctests added to key public functions
- [ ] All examples compile and run successfully
- [ ] CI validates examples on every run
- [ ] Documentation updated to reference executable examples

