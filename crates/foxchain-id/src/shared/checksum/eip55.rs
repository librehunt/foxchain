//! EIP-55 checksum validation and normalization
//!
//! EIP-55 specifies that addresses should use mixed-case checksumming:
//! - If the i-th character is a letter, it should be uppercase if the i-th bit
//!   of the hash of the lowercase address is 1, lowercase otherwise.

use crate::shared::crypto::hash::keccak256;
use crate::shared::encoding::hex::decode;
use crate::Error;

/// Validate EIP-55 checksum
///
/// Returns true if the address has a valid EIP-55 checksum
pub fn validate(address: &str) -> bool {
    // If address is all lowercase or all uppercase, it's not checksummed
    if address == address.to_lowercase() || address == address.to_uppercase() {
        return false;
    }

    // Compute checksum hash
    let lowercase = address.to_lowercase();
    let hash = keccak256(lowercase.as_bytes());

    // Check each character
    let hex_part = &address[2..]; // Skip "0x"
    for (i, char) in hex_part.chars().enumerate() {
        if char.is_alphabetic() {
            let byte_index = i / 2;
            let nibble = if i % 2 == 0 {
                hash[byte_index] >> 4
            } else {
                hash[byte_index] & 0x0f
            };

            let should_be_uppercase = nibble >= 8;
            let is_uppercase = char.is_uppercase();

            if should_be_uppercase != is_uppercase {
                return false;
            }
        }
    }

    true
}

/// Normalize address to EIP-55 checksum format
pub fn normalize(address: &str) -> Result<String, Error> {
    let lowercase = address.to_lowercase();
    let hex_part = &lowercase[2..];

    // Decode to bytes to validate
    let bytes = decode(&lowercase).map_err(|e| Error::InvalidInput(e))?;

    if bytes.len() != 20 {
        return Err(Error::InvalidInput("Address must be 20 bytes".to_string()));
    }

    // Compute checksum
    let hash = keccak256(lowercase.as_bytes());
    let mut normalized = String::from("0x");

    for (i, char) in hex_part.chars().enumerate() {
        if char.is_alphabetic() {
            let byte_index = i / 2;
            let nibble = if i % 2 == 0 {
                hash[byte_index] >> 4
            } else {
                hash[byte_index] & 0x0f
            };

            if nibble >= 8 {
                normalized.push(char.to_uppercase().next().unwrap());
            } else {
                normalized.push(char);
            }
        } else {
            normalized.push(char);
        }
    }

    Ok(normalized)
}
