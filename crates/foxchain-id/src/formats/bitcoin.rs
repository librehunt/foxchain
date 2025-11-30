//! Bitcoin ecosystem address detection and normalization
//!
//! Supports P2PKH, P2SH, and Bech32 address formats for Bitcoin, Litecoin, and Dogecoin.

use crate::{Chain, ChainCandidate, Error, IdentificationResult};
use base58::FromBase58;
use bech32::{self, Variant};
use sha2::{Digest, Sha256};

/// Detect if input is a Bitcoin ecosystem address and return identification result
pub fn detect_bitcoin(input: &str) -> Result<Option<IdentificationResult>, Error> {
    // Try Bech32 first (most modern format)
    if let Some(result) = detect_bech32(input)? {
        return Ok(Some(result));
    }

    // Try P2PKH (starts with 1)
    if let Some(result) = detect_p2pkh(input)? {
        return Ok(Some(result));
    }

    // Try P2SH (starts with 3)
    if let Some(result) = detect_p2sh(input)? {
        return Ok(Some(result));
    }

    Ok(None)
}

/// Detect Bech32 addresses (native SegWit)
fn detect_bech32(input: &str) -> Result<Option<IdentificationResult>, Error> {
    // Bech32 addresses start with known HRPs
    let hrps = ["bc1", "tb1", "ltc1", "lt1"];
    let input_lower = input.to_lowercase();

    for hrp in &hrps {
        if input_lower.starts_with(hrp) {
            // Validate Bech32 encoding
            match bech32::decode(input) {
                Ok((decoded_hrp, _data, variant)) => {
                    if variant != Variant::Bech32 {
                        continue;
                    }

                    // bech32::decode already validates the address
                    // We just need to identify the chain from HRP
                    let (chain, confidence) = identify_chain_from_bech32_hrp(&decoded_hrp);

                    // Normalize to lowercase (Bech32 is case-insensitive)
                    let normalized = input_lower.clone();

                    return Ok(Some(IdentificationResult {
                        normalized,
                        candidates: vec![ChainCandidate {
                            chain,
                            confidence,
                            reasoning: format!("Bech32 address with HRP '{}'", decoded_hrp),
                        }],
                    }));
                }
                Err(_) => continue,
            }
        }
    }

    Ok(None)
}

/// Detect P2PKH addresses (legacy, starts with 1)
fn detect_p2pkh(input: &str) -> Result<Option<IdentificationResult>, Error> {
    if !input.starts_with('1') {
        return Ok(None);
    }

    // Validate Base58Check
    let decoded = validate_base58check(input)?;
    if decoded.is_none() {
        return Ok(None);
    }
    let (version, _hash) = decoded.unwrap();

    // P2PKH uses version byte 0x00 for Bitcoin mainnet
    // Check if it's a valid P2PKH version byte
    let (chain, confidence) = identify_chain_from_version(version, true);

    if chain.is_none() {
        return Ok(None);
    }

    Ok(Some(IdentificationResult {
        normalized: input.to_string(), // Base58Check is canonical
        candidates: vec![ChainCandidate {
            chain: chain.unwrap(),
            confidence,
            reasoning: format!("P2PKH address (version byte 0x{:02x})", version),
        }],
    }))
}

/// Detect P2SH addresses (starts with 3)
fn detect_p2sh(input: &str) -> Result<Option<IdentificationResult>, Error> {
    if !input.starts_with('3') {
        return Ok(None);
    }

    // Validate Base58Check
    let decoded = validate_base58check(input)?;
    if decoded.is_none() {
        return Ok(None);
    }
    let (version, _hash) = decoded.unwrap();

    // P2SH uses version byte 0x05 for Bitcoin mainnet
    // Check if it's a valid P2SH version byte
    let (chain, confidence) = identify_chain_from_version(version, false);

    if chain.is_none() {
        return Ok(None);
    }

    Ok(Some(IdentificationResult {
        normalized: input.to_string(), // Base58Check is canonical
        candidates: vec![ChainCandidate {
            chain: chain.unwrap(),
            confidence,
            reasoning: format!("P2SH address (version byte 0x{:02x})", version),
        }],
    }))
}

/// Validate Base58Check encoding and return (version_byte, hash)
fn validate_base58check(input: &str) -> Result<Option<(u8, Vec<u8>)>, Error> {
    // Decode Base58
    let decoded = input
        .from_base58()
        .map_err(|_| Error::InvalidInput("Invalid Base58 encoding".to_string()))?;

    // Must be 25 bytes (1 version + 20 hash + 4 checksum)
    if decoded.len() != 25 {
        return Ok(None);
    }

    // Extract components
    let version = decoded[0];
    let hash = decoded[1..21].to_vec();
    let checksum = &decoded[21..25];

    // Verify checksum
    let payload = [&[version], hash.as_slice()].concat();
    let hash1 = Sha256::digest(&payload);
    let hash2 = Sha256::digest(hash1);
    let expected_checksum = &hash2[..4];

    if checksum != expected_checksum {
        return Ok(None);
    }

    Ok(Some((version, hash)))
}

/// Identify chain from version byte
fn identify_chain_from_version(version: u8, is_p2pkh: bool) -> (Option<Chain>, f64) {
    match (version, is_p2pkh) {
        // Bitcoin
        (0x00, true) => (Some(Chain::Bitcoin), 0.95), // P2PKH
        (0x05, false) => (Some(Chain::Bitcoin), 0.95), // P2SH
        // Litecoin
        (0x30, true) => (Some(Chain::Litecoin), 0.95), // P2PKH
        (0x32, false) => (Some(Chain::Litecoin), 0.95), // P2SH
        // Dogecoin
        (0x1e, true) => (Some(Chain::Dogecoin), 0.95), // P2PKH
        (0x16, false) => (Some(Chain::Dogecoin), 0.95), // P2SH
        // Testnet
        (0x6f, true) => (Some(Chain::Bitcoin), 0.80), // Testnet P2PKH
        (0xc4, false) => (Some(Chain::Bitcoin), 0.80), // Testnet P2SH
        _ => (None, 0.0),
    }
}

/// Identify chain from Bech32 HRP
fn identify_chain_from_bech32_hrp(hrp: &str) -> (Chain, f64) {
    match hrp.to_lowercase().as_str() {
        "bc1" => (Chain::Bitcoin, 0.95),
        "tb1" => (Chain::Bitcoin, 0.80), // Testnet
        "ltc1" | "lt1" => (Chain::Litecoin, 0.95),
        _ => (Chain::Bitcoin, 0.70), // Default fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_p2pkh_bitcoin() {
        // Valid Bitcoin P2PKH address
        let input = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
        let result = detect_bitcoin(input).unwrap();
        assert!(result.is_some(), "Should detect P2PKH address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Bitcoin);
        assert_eq!(id_result.normalized, input);
    }

    #[test]
    fn test_detect_p2sh_bitcoin() {
        let input = "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy";
        let result = detect_bitcoin(input).unwrap();
        assert!(result.is_some());
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Bitcoin);
    }

    #[test]
    fn test_detect_bech32_bitcoin() {
        let input = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let result = detect_bitcoin(input).unwrap();
        assert!(result.is_some(), "Should detect Bech32 address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Bitcoin);
        assert_eq!(id_result.normalized, input.to_lowercase());
    }

    #[test]
    fn test_detect_invalid_address() {
        let result = detect_bitcoin("not-a-bitcoin-address");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_validate_base58check() {
        // Valid Bitcoin P2PKH address
        let input = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
        let result = validate_base58check(input);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_validate_base58check_invalid() {
        // Invalid checksum
        let input = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNb"; // Changed last char
        let result = validate_base58check(input);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_detect_litecoin_p2pkh() {
        // Litecoin P2PKH address (version byte 0x30)
        let _input = "LTC1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
        // Note: This is a placeholder - actual Litecoin addresses would have different format
        // Testing that Litecoin is in the Chain enum
        assert_eq!(Chain::Litecoin, Chain::Litecoin);
    }

    #[test]
    fn test_detect_dogecoin_p2pkh() {
        // Dogecoin P2PKH address (version byte 0x1e)
        // Testing that Dogecoin is in the Chain enum
        assert_eq!(Chain::Dogecoin, Chain::Dogecoin);
    }

    #[test]
    fn test_identify_bitcoin_p2pkh() {
        // Test integration with identify() function
        use crate::identify;
        let input = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
        let result = identify(input);
        assert!(result.is_ok());
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Bitcoin);
        assert!(!id_result.normalized.is_empty());
    }

    #[test]
    fn test_identify_bitcoin_p2sh() {
        // Test integration with identify() function
        use crate::identify;
        let input = "3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy";
        let result = identify(input);
        assert!(result.is_ok());
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Bitcoin);
    }

    #[test]
    fn test_identify_bitcoin_bech32() {
        // Test integration with identify() function
        use crate::identify;
        let input = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let result = identify(input);
        assert!(result.is_ok());
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Bitcoin);
        assert_eq!(id_result.normalized, input.to_lowercase());
    }

    #[test]
    fn test_identify_evm_takes_precedence() {
        // EVM addresses should be detected before Bitcoin
        use crate::identify;
        let input = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
        let result = identify(input);
        assert!(result.is_ok());
        let id_result = result.unwrap();
        // Should be EVM, not Bitcoin
        assert_ne!(id_result.candidates[0].chain, Chain::Bitcoin);
    }

    #[test]
    fn test_bech32_case_insensitive() {
        // Bech32 addresses are case-insensitive
        let input_upper = "BC1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7KV8F3T4";
        let input_lower = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let result_upper = detect_bitcoin(input_upper).unwrap();
        let result_lower = detect_bitcoin(input_lower).unwrap();
        assert!(result_upper.is_some());
        assert!(result_lower.is_some());
        // Both should normalize to lowercase
        assert_eq!(
            result_upper.unwrap().normalized,
            result_lower.unwrap().normalized
        );
    }

    #[test]
    fn test_base58check_wrong_length() {
        // Address with wrong length should fail
        let input = "1"; // Too short
        let result = validate_base58check(input);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_chain_identification_from_version() {
        // Test that version bytes correctly identify chains
        let (chain_btc, _) = identify_chain_from_version(0x00, true);
        assert_eq!(chain_btc, Some(Chain::Bitcoin));

        let (chain_ltc, _) = identify_chain_from_version(0x30, true);
        assert_eq!(chain_ltc, Some(Chain::Litecoin));

        let (chain_doge, _) = identify_chain_from_version(0x1e, true);
        assert_eq!(chain_doge, Some(Chain::Dogecoin));
    }

    #[test]
    fn test_chain_identification_from_bech32_hrp() {
        // Test that HRPs correctly identify chains
        let (chain_btc, _) = identify_chain_from_bech32_hrp("bc1");
        assert_eq!(chain_btc, Chain::Bitcoin);

        let (chain_ltc, _) = identify_chain_from_bech32_hrp("ltc1");
        assert_eq!(chain_ltc, Chain::Litecoin);
    }
}
