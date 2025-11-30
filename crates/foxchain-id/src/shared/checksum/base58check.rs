//! Base58Check validation
//!
//! Base58Check is used by Bitcoin and Tron addresses.
//! Format: 25 bytes total (1 version + 20 hash + 4 checksum)

use crate::shared::crypto::hash::double_sha256;
use crate::shared::encoding::base58::decode;
use crate::Error;

/// Validate Base58Check encoding and extract version byte and hash
///
/// Returns (version_byte, hash_bytes) if valid, None otherwise
/// Base58Check format: 25 bytes total (1 version + 20 hash + 4 checksum)
pub fn validate(input: &str) -> Result<Option<(u8, Vec<u8>)>, Error> {
    // Decode Base58
    let decoded = match decode(input) {
        Ok(bytes) => bytes,
        Err(_) => return Ok(None),
    };

    // Must be 25 bytes (1 version + 20 hash + 4 checksum)
    if decoded.len() != 25 {
        return Ok(None);
    }

    // Extract components
    let version = decoded[0];
    let hash = decoded[1..21].to_vec();
    let checksum = &decoded[21..25];

    // Verify checksum (double SHA256)
    let payload = [&[version], hash.as_slice()].concat();
    let hash_result = double_sha256(&payload);
    let expected_checksum = &hash_result[..4];

    if checksum != expected_checksum {
        return Ok(None);
    }

    Ok(Some((version, hash)))
}
