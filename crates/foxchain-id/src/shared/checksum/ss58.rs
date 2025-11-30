//! SS58 checksum validation

use blake2::{Blake2b512, Digest};

/// SS58PRE constant: "SS58PRE" in bytes
const SS58PRE: &[u8] = b"SS58PRE";

/// Calculate SS58 checksum for given prefix and account ID
///
/// Returns the checksum bytes (length depends on address length)
pub fn calculate(prefix: &[u8], account_id: &[u8], checksum_len: usize) -> Vec<u8> {
    // SS58 checksum: blake2b_512(SS58PRE + prefix + account_id)
    let mut hasher = Blake2b512::new();
    hasher.update(SS58PRE);
    hasher.update(prefix);
    hasher.update(account_id);
    let hash = hasher.finalize();

    hash[..checksum_len].to_vec()
}

/// Validate SS58 checksum
///
/// Returns true if checksum is valid
pub fn validate(prefix: &[u8], account_id: &[u8], checksum: &[u8]) -> bool {
    let expected_checksum = calculate(prefix, account_id, checksum.len());
    checksum == expected_checksum.as_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksum() {
        let prefix = vec![0u8];
        let account_id = vec![0u8; 32];
        let checksum = calculate(&prefix, &account_id, 2);
        assert_eq!(checksum.len(), 2);
    }

    #[test]
    fn test_calculate_checksum_different_lengths() {
        let prefix = vec![0u8];
        let account_id = vec![0u8; 32];

        let checksum1 = calculate(&prefix, &account_id, 1);
        assert_eq!(checksum1.len(), 1);

        let checksum2 = calculate(&prefix, &account_id, 2);
        assert_eq!(checksum2.len(), 2);

        let checksum3 = calculate(&prefix, &account_id, 3);
        assert_eq!(checksum3.len(), 3);
    }

    #[test]
    fn test_calculate_checksum_deterministic() {
        let prefix = vec![0u8];
        let account_id = vec![0u8; 32];

        let checksum1 = calculate(&prefix, &account_id, 2);
        let checksum2 = calculate(&prefix, &account_id, 2);
        assert_eq!(checksum1, checksum2);
    }

    #[test]
    fn test_validate_correct_checksum() {
        let prefix = vec![0u8];
        let account_id = vec![0u8; 32];
        let checksum = calculate(&prefix, &account_id, 2);

        assert!(validate(&prefix, &account_id, &checksum));
    }

    #[test]
    fn test_validate_wrong_checksum() {
        let prefix = vec![0u8];
        let account_id = vec![0u8; 32];
        let wrong_checksum = vec![0xFFu8, 0xFFu8];

        assert!(!validate(&prefix, &account_id, &wrong_checksum));
    }

    #[test]
    fn test_validate_different_prefix() {
        let prefix1 = vec![0u8];
        let prefix2 = vec![2u8];
        let account_id = vec![0u8; 32];

        let checksum1 = calculate(&prefix1, &account_id, 2);
        let checksum2 = calculate(&prefix2, &account_id, 2);

        // Different prefixes should produce different checksums
        assert_ne!(checksum1, checksum2);

        // Each should validate with its own prefix
        assert!(validate(&prefix1, &account_id, &checksum1));
        assert!(validate(&prefix2, &account_id, &checksum2));

        // But not with the other prefix
        assert!(!validate(&prefix1, &account_id, &checksum2));
        assert!(!validate(&prefix2, &account_id, &checksum1));
    }

    #[test]
    fn test_validate_different_account_id() {
        let prefix = vec![0u8];
        let account_id1 = vec![0u8; 32];
        let account_id2 = vec![1u8; 32];

        let checksum1 = calculate(&prefix, &account_id1, 2);
        let checksum2 = calculate(&prefix, &account_id2, 2);

        // Different account IDs should produce different checksums
        assert_ne!(checksum1, checksum2);

        // Each should validate with its own account ID
        assert!(validate(&prefix, &account_id1, &checksum1));
        assert!(validate(&prefix, &account_id2, &checksum2));
    }

    #[test]
    fn test_calculate_with_two_byte_prefix() {
        let prefix = vec![0x40u8, 0x64u8]; // Two-byte prefix
        let account_id = vec![0u8; 32];
        let checksum = calculate(&prefix, &account_id, 2);
        assert_eq!(checksum.len(), 2);
        assert!(validate(&prefix, &account_id, &checksum));
    }
}
