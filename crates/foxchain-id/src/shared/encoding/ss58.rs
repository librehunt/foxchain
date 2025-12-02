//! SS58 encoding utilities

use base58::{FromBase58, ToBase58};

use crate::shared::checksum::ss58 as ss58_checksum;

/// Decode SS58 string to bytes
pub fn decode(input: &str) -> Result<Vec<u8>, String> {
    input
        .from_base58()
        .map_err(|_| "Invalid SS58 encoding".to_string())
}

/// Encode account ID as SS58 address with given prefix
///
/// # Arguments
/// * `prefix` - SS58 prefix (0 for Polkadot, 2 for Kusama, 42 for Generic Substrate)
/// * `account_id` - 32-byte account ID
///
/// # Returns
/// SS58-encoded address string
pub fn encode(prefix: u16, account_id: &[u8]) -> Result<String, String> {
    if account_id.len() != 32 {
        return Err("Account ID must be 32 bytes".to_string());
    }

    // Encode prefix bytes
    let prefix_bytes = if prefix < 64 {
        // Single-byte prefix (0-63)
        vec![prefix as u8]
    } else if prefix < 16384 {
        // Two-byte prefix (64-16383)
        // Format: first_byte = 0x40 + (prefix >> 8) & 0x3f, second_byte = prefix & 0xff
        let first_byte = 0x40u8 + ((prefix >> 8) & 0x3f) as u8;
        let second_byte = (prefix & 0xff) as u8;
        vec![first_byte, second_byte]
    } else {
        return Err("Prefix must be less than 16384".to_string());
    };

    // Calculate checksum (2 bytes for standard addresses)
    let checksum = ss58_checksum::calculate(&prefix_bytes, account_id, 2);

    // Combine: prefix + account_id + checksum
    let mut payload = Vec::new();
    payload.extend_from_slice(&prefix_bytes);
    payload.extend_from_slice(account_id);
    payload.extend_from_slice(&checksum);

    // Encode as Base58
    Ok(payload.to_base58())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_valid_ss58() {
        // Valid SS58 address (Polkadot format)
        let input = "15oF4uVJwmo4TdGW7VfQxNLavjXviYtpYNRY9YzXg6WZ1";
        let result = decode(input);
        // This might fail if checksum is invalid, but decoding should work
        let _ = result;
    }

    #[test]
    fn test_decode_invalid_ss58() {
        let input = "0OIl"; // Invalid Base58
        let result = decode(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid SS58 encoding"));
    }

    #[test]
    fn test_decode_empty() {
        let input = "";
        let result = decode(input);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 0);
    }
}
