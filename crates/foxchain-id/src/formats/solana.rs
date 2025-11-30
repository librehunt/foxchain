//! Solana address detection and normalization
//!
//! Solana uses Base58 encoding for addresses (public keys). Addresses are 32-44 bytes
//! when decoded, typically 32 bytes for standard addresses.

use crate::{Chain, ChainCandidate, Error, IdentificationResult};
use base58::FromBase58;

/// Detect if input is a Solana address and return identification result
pub fn detect_solana(input: &str) -> Result<Option<IdentificationResult>, Error> {
    // Solana addresses are base58 encoded
    // They don't have a specific prefix, so we rely on length and base58 validation
    
    // Try to decode as base58
    let decoded = match input.from_base58() {
        Ok(bytes) => bytes,
        Err(_) => return Ok(None), // Not valid base58
    };
    
    // Solana addresses are 32-44 bytes when decoded
    // Standard addresses are exactly 32 bytes
    if decoded.len() < 32 || decoded.len() > 44 {
        return Ok(None);
    }
    
    // Additional validation: check if it looks like a Solana address
    // Solana addresses are typically 32-44 characters in base58
    // and don't start with common prefixes from other chains
    if input.starts_with('1') || input.starts_with('3') {
        // These are likely Bitcoin addresses (P2PKH/P2SH)
        return Ok(None);
    }
    
    if input.starts_with("0x") {
        // This is an EVM address
        return Ok(None);
    }
    
    if input.starts_with("bc1") || input.starts_with("ltc1") || input.starts_with("lt1") {
        // This is a Bech32 address
        return Ok(None);
    }
    
    // Calculate confidence based on length
    // Standard 32-byte addresses have higher confidence
    let confidence = if decoded.len() == 32 {
        0.90 // High confidence for standard 32-byte addresses
    } else {
        0.75 // Lower confidence for non-standard lengths
    };
    
    // Normalize: Solana addresses are case-sensitive, but we keep as-is
    // (Base58 is case-sensitive, so we preserve the original)
    let normalized = input.to_string();
    
    Ok(Some(IdentificationResult {
        normalized,
        candidates: vec![ChainCandidate {
            chain: Chain::Solana,
            confidence,
            reasoning: format!(
                "Solana address (Base58, {} bytes)",
                decoded.len()
            ),
        }],
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_solana_standard() {
        // Standard 32-byte Solana address
        let input = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
        let result = detect_solana(input).unwrap();
        assert!(result.is_some(), "Should detect Solana address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Solana);
        assert_eq!(id_result.candidates[0].confidence, 0.90);
        assert_eq!(id_result.normalized, input);
    }

    #[test]
    fn test_detect_solana_non_standard_length() {
        // Non-standard length (but still valid)
        // Note: This is a placeholder test - actual Solana addresses are typically 32 bytes
        // We test that the function accepts 32-44 byte range
        let input = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM"; // 32 bytes
        let result = detect_solana(input).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_detect_solana_too_short() {
        // Too short to be a Solana address
        let input = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAW"; // Less than 32 bytes
        let result = detect_solana(input).unwrap();
        assert!(result.is_none(), "Should reject addresses shorter than 32 bytes");
    }

    #[test]
    fn test_detect_solana_too_long() {
        // Too long to be a Solana address
        let input = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM"; // More than 44 bytes
        let result = detect_solana(input).unwrap();
        assert!(result.is_none(), "Should reject addresses longer than 44 bytes");
    }

    #[test]
    fn test_detect_solana_invalid_base58() {
        // Invalid base58 encoding
        let input = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM0"; // Contains '0' which is invalid in base58
        let result = detect_solana(input).unwrap();
        assert!(result.is_none(), "Should reject invalid base58");
    }

    #[test]
    fn test_detect_solana_bitcoin_prefix() {
        // Address starting with '1' should be rejected (likely Bitcoin P2PKH)
        let input = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
        let result = detect_solana(input).unwrap();
        assert!(result.is_none(), "Should reject addresses starting with '1'");
    }

    #[test]
    fn test_detect_solana_bitcoin_p2sh_prefix() {
        // Address starting with '3' should be rejected (likely Bitcoin P2SH)
        let input = "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy";
        let result = detect_solana(input).unwrap();
        assert!(result.is_none(), "Should reject addresses starting with '3'");
    }

    #[test]
    fn test_detect_solana_evm_prefix() {
        // Address starting with '0x' should be rejected (EVM)
        let input = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
        let result = detect_solana(input).unwrap();
        assert!(result.is_none(), "Should reject EVM addresses");
    }

    #[test]
    fn test_detect_solana_bech32_prefix() {
        // Address starting with 'bc1' should be rejected (Bech32)
        let input = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let result = detect_solana(input).unwrap();
        assert!(result.is_none(), "Should reject Bech32 addresses");
    }

    #[test]
    fn test_identify_solana() {
        // Test integration with identify() function
        use crate::identify;
        let input = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
        let result = identify(input);
        assert!(result.is_ok());
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Solana);
        assert!(!id_result.normalized.is_empty());
    }

    #[test]
    fn test_identify_evm_takes_precedence() {
        // EVM addresses should be detected before Solana
        use crate::identify;
        let input = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
        let result = identify(input);
        assert!(result.is_ok());
        let id_result = result.unwrap();
        // Should be EVM, not Solana
        assert_ne!(id_result.candidates[0].chain, Chain::Solana);
    }

    #[test]
    fn test_identify_bitcoin_takes_precedence() {
        // Bitcoin addresses should be detected before Solana
        use crate::identify;
        let input = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
        let result = identify(input);
        assert!(result.is_ok());
        let id_result = result.unwrap();
        // Should be Bitcoin, not Solana
        assert_ne!(id_result.candidates[0].chain, Chain::Solana);
    }

    #[test]
    fn test_solana_case_sensitive() {
        // Solana addresses are case-sensitive (Base58 is case-sensitive)
        // We preserve the original case
        let input = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
        let result = detect_solana(input).unwrap();
        assert!(result.is_some());
        let id_result = result.unwrap();
        // Should preserve original case
        assert_eq!(id_result.normalized, input);
    }
}

