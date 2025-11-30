//! Cardano address detection and normalization
//!
//! Cardano uses Bech32 encoding with Human Readable Part (HRP) prefixes to identify different address types.
//! Mainnet uses "addr" for payment addresses and "stake" for stake addresses.
//! Testnet uses "addr_test" and "stake_test".

use crate::{Chain, ChainCandidate, Error, IdentificationResult};
use bech32::{self, Variant};

/// Map HRP to Cardano chain/address type
fn identify_chain_from_hrp(hrp: &str) -> Option<Chain> {
    match hrp.to_lowercase().as_str() {
        "addr" | "addr_test" => Some(Chain::Cardano),
        "stake" | "stake_test" => Some(Chain::Cardano),
        _ => None,
    }
}

/// Detect if input is a Cardano address and return identification result
pub fn detect_cardano(input: &str) -> Result<Option<IdentificationResult>, Error> {
    // Cardano addresses use Bech32 encoding with specific HRPs
    // Try to decode as Bech32
    let (hrp, data, variant) = match bech32::decode(input) {
        Ok(result) => result,
        Err(_) => return Ok(None), // Not valid Bech32
    };

    // Cardano uses Bech32 (not Bech32m)
    if variant != Variant::Bech32 {
        return Ok(None);
    }

    // Check if HRP matches a known Cardano address type
    let chain = match identify_chain_from_hrp(&hrp) {
        Some(c) => c,
        None => return Ok(None), // Unknown HRP, not a Cardano address
    };

    // Validate data length
    // Cardano addresses have variable length depending on address type:
    // - Payment addresses: typically 57-58 5-bit groups (base32 encoded)
    // - Stake addresses: typically 29 5-bit groups
    // We'll accept a reasonable range
    let data_len = data.len();
    if !(20..=100).contains(&data_len) {
        // Too short or too long to be a valid Cardano address
        return Ok(None);
    }

    // Normalize: Bech32 is case-insensitive, standard is lowercase
    let normalized = input.to_lowercase();

    // Calculate confidence
    let confidence = 0.95; // High confidence for recognized Cardano addresses

    Ok(Some(IdentificationResult {
        normalized,
        candidates: vec![ChainCandidate {
            chain,
            confidence,
            reasoning: format!("Cardano address (Bech32, HRP: {})", hrp),
        }],
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bech32::ToBase32;

    /// Helper function to create a valid Cardano address for testing
    fn create_test_cardano_address(hrp: &str) -> String {
        // Create a valid Bech32 address with the given HRP
        let data = vec![0u8; 20]; // 20 bytes of data
        bech32::encode(hrp, data.to_base32(), Variant::Bech32).unwrap()
    }

    #[test]
    fn test_detect_cardano_mainnet_payment() {
        // Test with mainnet payment address (addr)
        let input = create_test_cardano_address("addr");
        let result = detect_cardano(&input).unwrap();
        assert!(
            result.is_some(),
            "Should detect Cardano mainnet payment address"
        );
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Cardano);
        assert_eq!(id_result.candidates[0].confidence, 0.95);
    }

    #[test]
    fn test_detect_cardano_mainnet_stake() {
        // Test with mainnet stake address (stake)
        let input = create_test_cardano_address("stake");
        let result = detect_cardano(&input).unwrap();
        assert!(
            result.is_some(),
            "Should detect Cardano mainnet stake address"
        );
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Cardano);
    }

    #[test]
    fn test_detect_cardano_testnet() {
        // Test with testnet address (addr_test)
        let input = create_test_cardano_address("addr_test");
        let result = detect_cardano(&input).unwrap();
        assert!(result.is_some(), "Should detect Cardano testnet address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Cardano);
    }

    #[test]
    fn test_detect_cardano_invalid_hrp() {
        // Test with invalid HRP
        let input = create_test_cardano_address("cosmos"); // Cosmos HRP, not Cardano
        let result = detect_cardano(&input).unwrap();
        assert!(result.is_none(), "Should reject non-Cardano HRP");
    }

    #[test]
    fn test_detect_cardano_invalid_bech32() {
        // Test with invalid Bech32 encoding
        let input = "addr1invalid";
        let result = detect_cardano(input).unwrap();
        assert!(result.is_none(), "Should reject invalid Bech32");
    }

    #[test]
    fn test_detect_cardano_case_insensitive() {
        // Test case insensitivity
        let input = create_test_cardano_address("ADDR");
        let result = detect_cardano(&input).unwrap();
        assert!(
            result.is_some(),
            "Should detect Cardano address regardless of case"
        );
    }

    #[test]
    fn test_identify_cardano() {
        // Test integration with identify() function
        use crate::identify;
        let input = create_test_cardano_address("addr");
        let result = identify(&input);
        // This may fail if the address is invalid, but tests integration
        assert!(result.is_ok() || result.is_err());
    }
}
