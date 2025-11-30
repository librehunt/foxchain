//! Substrate/Polkadot ecosystem address detection and normalization
//!
//! Substrate uses SS58 encoding for addresses. SS58 is a variant of Base58 with chain-specific
//! prefixes and checksums. Different chains use different prefixes (Polkadot uses 0, Kusama uses 2, etc.).

use crate::shared::checksum::ss58 as ss58_checksum;
use crate::shared::encoding::ss58;
use crate::{Chain, ChainCandidate, Error, IdentificationResult};

/// Decode SS58 prefix from decoded bytes
/// Returns (prefix_value, prefix_length_in_bytes)
///
/// SS58 prefix encoding:
/// - Single-byte prefixes (0-63): The prefix is the first byte directly
/// - Two-byte prefixes (64-16383):
///   - First byte: `0x40 + (prefix >> 8) & 0x3f` (bits 8-13 of prefix)
///   - Second byte: `prefix & 0xff` (bits 0-7 of prefix)
///   - Decoding: `prefix = ((first_byte & 0x3f) << 8) | second_byte`
fn decode_ss58_prefix(decoded: &[u8]) -> Option<(u16, usize)> {
    if decoded.is_empty() {
        return None;
    }

    if decoded[0] < 64 {
        // Single-byte prefix (0-63)
        Some((decoded[0] as u16, 1))
    } else if decoded.len() >= 2 && decoded[0] < 128 {
        // Two-byte prefix (64-16383)
        // Format: ((first_byte & 0x3f) << 8) | second_byte
        let first_byte = decoded[0] & 0x3f; // Extract bits 0-5 (the prefix bits)
        let second_byte = decoded[1];
        let prefix = ((first_byte as u16) << 8) | (second_byte as u16);
        Some((prefix, 2))
    } else {
        None
    }
}

/// Map SS58 prefix to Substrate chain
/// Supports both single-byte (u8) and two-byte (u16) prefixes
fn identify_chain_from_prefix(prefix: u16) -> Option<Chain> {
    match prefix {
        0 => Some(Chain::Polkadot),
        2 => Some(Chain::Kusama),
        42 => Some(Chain::Substrate), // Generic Substrate
        _ => None,
    }
}

/// Detect if input is a Substrate address and return identification result
pub fn detect_substrate(input: &str) -> Result<Option<IdentificationResult>, Error> {
    // SS58 addresses are Base58 encoded with chain-specific prefixes
    // Try to decode as Base58
    let decoded = match ss58::decode(input) {
        Ok(bytes) => bytes,
        Err(_) => return Ok(None), // Not valid Base58
    };

    // SS58 addresses have a specific structure:
    // - Prefix byte(s) (1-2 bytes, encoded as variable-length)
    // - Account ID (32 bytes)
    // - Checksum (2 bytes)
    // Total length varies but typically 35-36 bytes when decoded

    // Minimum length: 1 byte prefix + 32 bytes account + 2 bytes checksum = 35 bytes
    // Maximum reasonable length: ~50 bytes (for longer prefixes)
    if decoded.len() < 35 || decoded.len() > 50 {
        return Ok(None);
    }

    // Decode SS58 prefix (handles both single-byte and two-byte prefixes)
    let (prefix_value, prefix_len) = match decode_ss58_prefix(&decoded) {
        Some((p, len)) => (p, len),
        None => return Ok(None),
    };

    let account_id_start = prefix_len;

    // Determine checksum length based on total decoded length
    // SS58 checksum length rules (from Substrate spec):
    // - For addresses < 64 bytes decoded: 1 byte checksum
    // - For addresses >= 64 bytes decoded: 2 bytes checksum
    // - For addresses >= 16384 bytes decoded: 3 bytes checksum (rare)
    // However, standard Substrate addresses (35-36 bytes: 1 prefix + 32 account + 2 checksum)
    // use 2-byte checksum despite being < 64 bytes. This is a special case in practice.
    // We determine checksum length by working backwards: if decoded.len() is 35 or 36,
    // it's likely a standard address with 2-byte checksum. Otherwise, use the spec rules.
    let checksum_len = if decoded.len() == 35 || decoded.len() == 36 {
        // Standard Substrate addresses use 2-byte checksum
        2
    } else if decoded.len() < 64 {
        1
    } else if decoded.len() < 16384 {
        2
    } else {
        3
    };

    // Ensure we have enough bytes for checksum
    if decoded.len() < account_id_start + 32 + checksum_len {
        return Ok(None);
    }

    let account_id_end = decoded.len() - checksum_len;
    let account_id = &decoded[account_id_start..account_id_end];
    let checksum = &decoded[account_id_end..];

    // Account ID should be 32 bytes
    if account_id.len() != 32 {
        return Ok(None);
    }

    // Extract prefix bytes for checksum validation
    let prefix_bytes = &decoded[0..prefix_len];

    // Validate SS58 checksum using shared utility
    if !ss58_checksum::validate(prefix_bytes, account_id, checksum) {
        return Ok(None);
    }

    // Check if prefix matches a known chain
    let chain = match identify_chain_from_prefix(prefix_value) {
        Some(c) => c,
        None => {
            // Unknown prefix, but might still be a valid Substrate address
            // Return generic Substrate chain
            Chain::Substrate
        }
    };

    // Normalize: SS58 addresses are case-sensitive, but we keep as-is
    // (Base58 is canonical, so we preserve the original)
    let normalized = input.to_string();

    // Calculate confidence based on prefix recognition
    let confidence = if identify_chain_from_prefix(prefix_value).is_some() {
        0.90 // High confidence for recognized chains
    } else {
        0.75 // Lower confidence for unknown prefixes
    };

    Ok(Some(IdentificationResult {
        normalized,
        candidates: vec![ChainCandidate {
            chain,
            confidence,
            reasoning: format!("Substrate address (SS58, prefix: {})", prefix_value),
        }],
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base58::ToBase58;

    /// Helper function to create a valid Substrate address for testing
    /// This creates a valid SS58 address structure with proper checksum
    fn create_test_substrate_address(prefix: u8) -> String {
        // Create a valid SS58 address structure:
        // Prefix byte + 32 bytes account ID + checksum
        let prefix_bytes = vec![prefix];
        let account_id = vec![0u8; 32];

        // Determine checksum length based on total address length
        // Standard addresses (35 bytes) use 2-byte checksum
        let checksum_len = 2;

        // Calculate checksum using shared utility
        let checksum = ss58_checksum::calculate(&prefix_bytes, &account_id, checksum_len);

        // Combine: prefix + account_id + checksum
        let mut full = prefix_bytes;
        full.extend_from_slice(&account_id);
        full.extend_from_slice(&checksum);

        // Encode in base58
        full.to_base58()
    }

    #[test]
    fn test_detect_polkadot() {
        // Test with Polkadot address (prefix 0)
        let input = create_test_substrate_address(0);
        let result = detect_substrate(&input).unwrap();
        assert!(result.is_some(), "Should detect Polkadot address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Polkadot);
    }

    #[test]
    fn test_detect_kusama() {
        // Test with Kusama address (prefix 2)
        let input = create_test_substrate_address(2);
        let result = detect_substrate(&input).unwrap();
        assert!(result.is_some(), "Should detect Kusama address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Kusama);
    }

    #[test]
    fn test_detect_substrate_unknown_prefix() {
        // Test with unknown prefix (should fall back to generic Substrate)
        // Use prefix 10 which is not in our mapping (0, 2, 42) and < 64 (single-byte prefix)
        let prefix_bytes = vec![10u8];
        let account_id = vec![0u8; 32];
        // For 35-byte address (1 prefix + 32 account + 2 checksum), use 2-byte checksum
        let checksum = ss58_checksum::calculate(&prefix_bytes, &account_id, 2);
        let mut full = prefix_bytes;
        full.extend_from_slice(&account_id);
        full.extend_from_slice(&checksum);

        // Verify length is 35 bytes
        assert_eq!(full.len(), 35, "Address should be 35 bytes");

        let input = full.to_base58();

        let result = detect_substrate(&input).unwrap();
        assert!(
            result.is_some(),
            "Should detect Substrate address with unknown prefix. Input: {}, Decoded len: {}",
            input,
            full.len()
        );
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Substrate);
    }

    #[test]
    fn test_detect_substrate_invalid_base58() {
        // Invalid base58 encoding
        let input = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY0"; // Contains '0' which is invalid in base58
        let result = detect_substrate(input).unwrap();
        assert!(result.is_none(), "Should reject invalid base58");
    }

    #[test]
    fn test_detect_substrate_too_short() {
        // Address too short
        let input = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQ"; // Too short
        let result = detect_substrate(input).unwrap();
        assert!(result.is_none(), "Should reject too short address");
    }

    #[test]
    fn test_detect_substrate_too_long() {
        // Address too long
        let long_bytes = vec![0u8; 100];
        let input = long_bytes.to_base58();
        let result = detect_substrate(&input).unwrap();
        assert!(result.is_none(), "Should reject too long address");
    }

    #[test]
    fn test_detect_substrate_wrong_account_id_length() {
        // Address with wrong account ID length
        // Create address with 31-byte account ID instead of 32
        let prefix = vec![0u8];
        let account_id = vec![0u8; 31]; // Wrong length
        let checksum = ss58_checksum::calculate(&prefix, &account_id, 2);
        let mut full = prefix;
        full.extend_from_slice(&account_id);
        full.extend_from_slice(&checksum);
        let input = full.to_base58();

        let result = detect_substrate(&input).unwrap();
        assert!(
            result.is_none(),
            "Should reject address with wrong account ID length"
        );
    }

    #[test]
    fn test_decode_ss58_prefix_single_byte() {
        // Test single-byte prefix decoding
        let decoded = vec![0u8, 1u8, 2u8];
        let result = decode_ss58_prefix(&decoded);
        assert_eq!(result, Some((0, 1)));

        let decoded2 = vec![42u8, 1u8, 2u8];
        let result2 = decode_ss58_prefix(&decoded2);
        assert_eq!(result2, Some((42, 1)));
    }

    #[test]
    fn test_decode_ss58_prefix_two_byte() {
        // Test two-byte prefix decoding
        // Prefix 64 = 0x40 (first byte) + 0x00 (second byte)
        // First byte: 0x40 = 64, which is >= 64, so it's a two-byte prefix
        // Decoding: ((0x40 & 0x3f) << 8) | 0x00 = (0x00 << 8) | 0x00 = 0
        // Wait, that's wrong. Let me check the encoding:
        // For prefix 64: first_byte = 0x40 + (64 >> 8) & 0x3f = 0x40 + 0 = 0x40
        // second_byte = 64 & 0xff = 0x40
        // So prefix 64 should be encoded as [0x40, 0x40]
        let decoded = vec![0x40u8, 0x40u8, 1u8, 2u8];
        let result = decode_ss58_prefix(&decoded);
        assert_eq!(result, Some((64, 2)));

        // Prefix 100 = 0x40 (first byte) + 0x64 (second byte)
        // For prefix 100: first_byte = 0x40 + (100 >> 8) & 0x3f = 0x40 + 0 = 0x40
        // second_byte = 100 & 0xff = 0x64
        let decoded2 = vec![0x40u8, 0x64u8, 1u8, 2u8];
        let result2 = decode_ss58_prefix(&decoded2);
        assert_eq!(result2, Some((100, 2)));
    }

    #[test]
    fn test_identify_chain_from_prefix() {
        assert_eq!(identify_chain_from_prefix(0), Some(Chain::Polkadot));
        assert_eq!(identify_chain_from_prefix(2), Some(Chain::Kusama));
        assert_eq!(identify_chain_from_prefix(42), Some(Chain::Substrate));
        assert_eq!(identify_chain_from_prefix(99), None);
    }

    #[test]
    fn test_calculate_ss58_checksum() {
        // Test that checksum calculation works
        let prefix = vec![0u8];
        let account_id = vec![0u8; 32];
        let checksum = ss58_checksum::calculate(&prefix, &account_id, 2);
        assert_eq!(checksum.len(), 2, "Checksum should be 2 bytes");
    }

    #[test]
    fn test_validate_ss58_checksum() {
        // Test that checksum validation works
        let prefix = vec![0u8];
        let account_id = vec![0u8; 32];
        let checksum = ss58_checksum::calculate(&prefix, &account_id, 2);
        assert!(ss58_checksum::validate(&prefix, &account_id, &checksum));

        // Test with wrong checksum
        let wrong_checksum = vec![0u8, 1u8];
        assert!(!ss58_checksum::validate(
            &prefix,
            &account_id,
            &wrong_checksum
        ));
    }

    #[test]
    fn test_identify_substrate() {
        // Test integration with identify() function
        use crate::identify;
        let input = create_test_substrate_address(0);
        let result = identify(&input);
        assert!(result.is_ok(), "Should identify Substrate address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Polkadot);
        assert!(!id_result.normalized.is_empty());
    }

    #[test]
    fn test_two_byte_prefix() {
        // Test with two-byte prefix (64-16383 range)
        // Create address with prefix 100 (two-byte)
        let prefix_bytes = vec![0x40u8, 0x64u8]; // Prefix 100
        let account_id = vec![0u8; 32];
        let checksum = ss58_checksum::calculate(&prefix_bytes, &account_id, 2);
        let mut full = prefix_bytes;
        full.extend_from_slice(&account_id);
        full.extend_from_slice(&checksum);
        let input = full.to_base58();

        let result = detect_substrate(&input).unwrap();
        assert!(
            result.is_some(),
            "Should detect address with two-byte prefix"
        );
        let id_result = result.unwrap();
        // Should be generic Substrate (prefix 100 is not in our mapping)
        assert_eq!(id_result.candidates[0].chain, Chain::Substrate);
    }
}
