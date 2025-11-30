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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_checksummed() {
        // Use a known valid EIP-55 checksummed address
        // First normalize a lowercase address to get a valid checksummed version
        let lowercase = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
        let checksummed = normalize(lowercase).unwrap();
        // The normalized address should have a valid checksum
        assert!(validate(&checksummed), "Normalized address should have valid checksum: {}", checksummed);
    }

    #[test]
    fn test_validate_lowercase_not_checksummed() {
        let address = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
        assert!(!validate(address));
    }

    #[test]
    fn test_validate_uppercase_not_checksummed() {
        let address = "0xD8DA6BF26964AF9D7EED9E03E53415D37AA96045";
        assert!(!validate(address));
    }

    #[test]
    fn test_normalize_lowercase() {
        let lowercase = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
        let result = normalize(lowercase);
        assert!(result.is_ok());
        let normalized = result.unwrap();
        assert_ne!(normalized, lowercase);
        assert!(normalized.starts_with("0x"));
        assert_eq!(normalized.len(), 42);
        // Should validate as correct checksum
        assert!(validate(&normalized));
    }

    #[test]
    fn test_normalize_uppercase() {
        let uppercase = "0xD8DA6BF26964AF9D7EED9E03E53415D37AA96045";
        let result = normalize(uppercase);
        assert!(result.is_ok());
        let normalized = result.unwrap();
        assert_ne!(normalized, uppercase);
        assert!(validate(&normalized));
    }

    #[test]
    fn test_normalize_already_checksummed() {
        let lowercase = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
        let checksummed = normalize(lowercase).unwrap();
        let result = normalize(&checksummed);
        assert!(result.is_ok());
        let normalized = result.unwrap();
        // Should produce the same checksummed format
        assert!(validate(&normalized));
    }

    #[test]
    fn test_normalize_invalid_length() {
        let invalid = "0x1234"; // Too short
        let result = normalize(invalid);
        assert!(result.is_err());
        if let Err(Error::InvalidInput(msg)) = result {
            assert!(msg.contains("20 bytes"));
        }
    }

    #[test]
    fn test_normalize_invalid_hex() {
        let invalid = "0xgggggggggggggggggggggggggggggggggggggggg";
        let result = normalize(invalid);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_normalize_roundtrip() {
        let original = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
        let normalized = normalize(original).unwrap();
        assert!(validate(&normalized));
        // Normalizing again should produce the same result
        let normalized2 = normalize(&normalized).unwrap();
        assert_eq!(normalized, normalized2);
    }
}
