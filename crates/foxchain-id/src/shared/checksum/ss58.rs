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
