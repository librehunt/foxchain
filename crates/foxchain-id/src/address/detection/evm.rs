//! EVM (Ethereum Virtual Machine) address detection and normalization
//!
//! Supports EIP-55 checksum validation and multi-chain candidate generation
//! for EVM-compatible chains.

use crate::shared::checksum::eip55;
use crate::shared::encoding::hex;
use crate::{Chain, ChainCandidate, Error, IdentificationResult};

/// Detect if input is an EVM address and return identification result
pub fn detect_evm(input: &str) -> Result<Option<IdentificationResult>, Error> {
    // Check if input matches EVM address format: 0x followed by 40 hex characters
    if !input.starts_with("0x") || input.len() != 42 {
        return Ok(None);
    }

    let hex_part = &input[2..];

    // Validate hex characters
    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(None);
    }

    // Validate address length (20 bytes = 40 hex chars)
    let bytes = hex::decode(input).map_err(Error::InvalidInput)?;
    if bytes.len() != 20 {
        return Ok(None);
    }

    // Check EIP-55 checksum
    let checksum_valid = eip55::validate(input);
    let normalized = eip55::normalize(input)?;

    // Generate candidates for all EVM chains
    let candidates = generate_evm_candidates(checksum_valid);

    Ok(Some(IdentificationResult {
        normalized,
        candidates,
    }))
}

/// Generate chain candidates for EVM addresses
///
/// EVM addresses are valid across many chains, so we return all major
/// EVM-compatible chains as candidates with confidence scores based on
/// checksum validity.
fn generate_evm_candidates(checksum_valid: bool) -> Vec<ChainCandidate> {
    let base_confidence = if checksum_valid { 0.95 } else { 0.85 };
    let reasoning = if checksum_valid {
        "Valid EVM address with EIP-55 checksum".to_string()
    } else {
        "Valid EVM address format (lowercase, no checksum)".to_string()
    };

    vec![
        ChainCandidate {
            chain: Chain::Ethereum,
            confidence: base_confidence,
            reasoning: reasoning.clone(),
        },
        ChainCandidate {
            chain: Chain::Polygon,
            confidence: base_confidence - 0.05,
            reasoning: "EVM-compatible chain (Polygon)".to_string(),
        },
        ChainCandidate {
            chain: Chain::BSC,
            confidence: base_confidence - 0.05,
            reasoning: "EVM-compatible chain (BSC)".to_string(),
        },
        ChainCandidate {
            chain: Chain::Avalanche,
            confidence: base_confidence - 0.05,
            reasoning: "EVM-compatible chain (Avalanche)".to_string(),
        },
        ChainCandidate {
            chain: Chain::Arbitrum,
            confidence: base_confidence - 0.05,
            reasoning: "EVM-compatible chain (Arbitrum)".to_string(),
        },
        ChainCandidate {
            chain: Chain::Optimism,
            confidence: base_confidence - 0.05,
            reasoning: "EVM-compatible chain (Optimism)".to_string(),
        },
        ChainCandidate {
            chain: Chain::Base,
            confidence: base_confidence - 0.05,
            reasoning: "EVM-compatible chain (Base)".to_string(),
        },
        ChainCandidate {
            chain: Chain::Fantom,
            confidence: base_confidence - 0.05,
            reasoning: "EVM-compatible chain (Fantom)".to_string(),
        },
        ChainCandidate {
            chain: Chain::Celo,
            confidence: base_confidence - 0.05,
            reasoning: "EVM-compatible chain (Celo)".to_string(),
        },
        ChainCandidate {
            chain: Chain::Gnosis,
            confidence: base_confidence - 0.05,
            reasoning: "EVM-compatible chain (Gnosis)".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_evm_valid_address() {
        let result = detect_evm("0x742d35Cc6634C0532925a3b844Bc454e4438f44e").unwrap();
        assert!(result.is_some());
        let id_result = result.unwrap();
        assert!(!id_result.candidates.is_empty());
        assert_eq!(id_result.candidates[0].chain, Chain::Ethereum);
    }

    #[test]
    fn test_detect_evm_invalid_format() {
        let result = detect_evm("0x123").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_detect_evm_not_evm() {
        let result = detect_evm("bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_validate_eip55_checksum() {
        // Test that our normalization produces valid checksums
        let lowercase = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
        let normalized = eip55::normalize(lowercase).unwrap();
        assert!(eip55::validate(&normalized));

        // Lowercase (no checksum)
        assert!(!eip55::validate(
            "0xd8da6bf26964af9d7eed9e03e53415d37aa96045"
        ));

        // All uppercase (no checksum)
        assert!(!eip55::validate(
            "0xD8DA6BF26964AF9D7EED9E03E53415D37AA96045"
        ));
    }

    #[test]
    fn test_normalize_to_eip55() {
        // Test normalization produces checksummed format
        let lowercase = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
        let normalized = eip55::normalize(lowercase).unwrap();
        // Verify it's different from input (checksummed)
        assert_ne!(normalized, lowercase);
        assert!(normalized.starts_with("0x"));
        assert_eq!(normalized.len(), 42);
        // Verify it validates as correct checksum
        assert!(eip55::validate(&normalized));

        // Test with another address
        let lowercase2 = "0x742d35cc6634c0532925a3b844bc454e4438f44e";
        let normalized2 = eip55::normalize(lowercase2).unwrap();
        assert_ne!(normalized2, lowercase2);
        assert!(normalized2.starts_with("0x"));
        assert_eq!(normalized2.len(), 42);
        assert!(eip55::validate(&normalized2));
    }

    #[test]
    fn test_generate_evm_candidates() {
        let candidates = generate_evm_candidates(true);
        assert_eq!(candidates.len(), 10);
        assert_eq!(candidates[0].chain, Chain::Ethereum);
        assert_eq!(candidates[0].confidence, 0.95);
    }
}
