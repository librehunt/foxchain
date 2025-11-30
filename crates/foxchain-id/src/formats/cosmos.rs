//! Cosmos ecosystem address detection and normalization
//!
//! Cosmos uses Bech32 encoding with Human Readable Part (HRP) prefixes to identify different chains.
//! All Cosmos addresses use the same Bech32 encoding scheme, only the HRP differs.

use crate::{Chain, ChainCandidate, Error, IdentificationResult};
use bech32::{self, Variant};

/// Map HRP to Cosmos chain
fn identify_chain_from_hrp(hrp: &str) -> Option<Chain> {
    match hrp.to_lowercase().as_str() {
        "cosmos" => Some(Chain::CosmosHub),
        "osmo" => Some(Chain::Osmosis),
        "juno" => Some(Chain::Juno),
        "akash" => Some(Chain::Akash),
        "stars" => Some(Chain::Stargaze),
        "secret" => Some(Chain::SecretNetwork),
        "terra" => Some(Chain::Terra),
        "kava" => Some(Chain::Kava),
        "regen" => Some(Chain::Regen),
        "sent" => Some(Chain::Sentinel),
        _ => None,
    }
}

/// Detect if input is a Cosmos address and return identification result
pub fn detect_cosmos(input: &str) -> Result<Option<IdentificationResult>, Error> {
    // Cosmos addresses use Bech32 encoding with chain-specific HRPs
    // Try to decode as Bech32
    let (hrp, data, variant) = match bech32::decode(input) {
        Ok(result) => result,
        Err(_) => return Ok(None), // Not valid Bech32
    };

    // Cosmos uses Bech32 (not Bech32m)
    if variant != Variant::Bech32 {
        return Ok(None);
    }

    // Check if HRP matches a known Cosmos chain
    let chain = match identify_chain_from_hrp(&hrp) {
        Some(c) => c,
        None => return Ok(None), // Unknown HRP, not a Cosmos address
    };

    // Validate data length (typically 20 bytes when decoded from base32)
    // Bech32 data is in 5-bit groups, so 20 bytes = 32 5-bit groups
    // But we'll be lenient and accept reasonable lengths
    // Minimum is around 20 5-bit groups (for 20-byte addresses)
    // Maximum is around 90 5-bit groups (for longer addresses)
    let data_len = data.len();
    if !(20..=90).contains(&data_len) {
        // Too short or too long to be a valid Cosmos address
        return Ok(None);
    }

    // Normalize: Bech32 is case-insensitive, standard is lowercase
    let normalized = input.to_lowercase();

    // Calculate confidence based on HRP recognition
    let confidence = 0.95; // High confidence for recognized Cosmos chains

    Ok(Some(IdentificationResult {
        normalized,
        candidates: vec![ChainCandidate {
            chain,
            confidence,
            reasoning: format!("Cosmos address (Bech32, HRP: {})", hrp),
        }],
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bech32::ToBase32;

    /// Helper function to create a valid Cosmos address for testing
    fn create_test_cosmos_address(hrp: &str) -> String {
        // Create a valid Bech32 address with the given HRP
        // Use 20 bytes of address data (standard Cosmos address length)
        let address_bytes = vec![0u8; 20];
        let data = address_bytes.to_base32();
        bech32::encode(hrp, data, Variant::Bech32).unwrap()
    }

    #[test]
    fn test_detect_cosmos_hub() {
        // Cosmos Hub address
        let input = create_test_cosmos_address("cosmos");
        let result = detect_cosmos(&input).unwrap();
        assert!(result.is_some(), "Should detect Cosmos Hub address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::CosmosHub);
        assert_eq!(id_result.normalized, input);
    }

    #[test]
    fn test_detect_osmosis() {
        // Osmosis address
        let input = create_test_cosmos_address("osmo");
        let result = detect_cosmos(&input).unwrap();
        assert!(result.is_some(), "Should detect Osmosis address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Osmosis);
    }

    #[test]
    fn test_detect_juno() {
        // Juno address
        let input = create_test_cosmos_address("juno");
        let result = detect_cosmos(&input).unwrap();
        assert!(result.is_some(), "Should detect Juno address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Juno);
    }

    #[test]
    fn test_detect_cosmos_case_insensitive() {
        // Test case insensitivity
        let input = create_test_cosmos_address("cosmos");
        let input_upper = input.to_uppercase();
        let result = detect_cosmos(&input_upper).unwrap();
        assert!(result.is_some());
        let id_result = result.unwrap();
        // Should normalize to lowercase
        assert_eq!(id_result.normalized, input);
    }

    #[test]
    fn test_detect_cosmos_invalid_hrp() {
        // Address with unknown HRP (Bitcoin Bech32)
        let input = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let result = detect_cosmos(input).unwrap();
        assert!(result.is_none(), "Should reject Bitcoin Bech32 addresses");
    }

    #[test]
    fn test_detect_cosmos_invalid_bech32() {
        // Invalid Bech32 encoding
        let input = "cosmos1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4x"; // Invalid checksum
        let result = detect_cosmos(input).unwrap();
        assert!(result.is_none(), "Should reject invalid Bech32");
    }

    #[test]
    fn test_identify_chain_from_hrp() {
        assert_eq!(identify_chain_from_hrp("cosmos"), Some(Chain::CosmosHub));
        assert_eq!(identify_chain_from_hrp("osmo"), Some(Chain::Osmosis));
        assert_eq!(identify_chain_from_hrp("juno"), Some(Chain::Juno));
        assert_eq!(identify_chain_from_hrp("akash"), Some(Chain::Akash));
        assert_eq!(identify_chain_from_hrp("stars"), Some(Chain::Stargaze));
        assert_eq!(
            identify_chain_from_hrp("secret"),
            Some(Chain::SecretNetwork)
        );
        assert_eq!(identify_chain_from_hrp("terra"), Some(Chain::Terra));
        assert_eq!(identify_chain_from_hrp("unknown"), None);
    }

    #[test]
    fn test_identify_cosmos() {
        // Test integration with identify() function
        use crate::identify;
        let input = create_test_cosmos_address("cosmos");
        let result = identify(&input);
        assert!(result.is_ok(), "Should identify Cosmos address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::CosmosHub);
        assert!(!id_result.normalized.is_empty());
    }
}
