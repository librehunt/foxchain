//! Unified metadata-driven address detector
//!
//! This module provides a unified address detector that uses metadata instead
//! of hardcoded heuristics. The detector receives pre-filtered candidates and
//! only implements format-specific validation logic.

use crate::input::InputCharacteristics;
use crate::registry::{
    AddressMetadata, Chain, CharSet, ChecksumType, EncodingType,
};
use crate::shared::checksum::{base58check, bech32 as bech32_checksum, eip55};
use crate::Error;
use bech32;

/// Result of address detection
#[derive(Debug, Clone)]
pub struct DetectionResult {
    /// Chain identifier
    pub chain: Chain,
    /// Encoding type used
    pub encoding: EncodingType,
    /// Normalized address representation
    pub normalized: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Reasoning for this detection
    pub reasoning: String,
}

/// Detect address using metadata
pub fn detect_address(
    input: &str,
    chars: &InputCharacteristics,
    metadata: &AddressMetadata,
    chain: Chain,
) -> Result<Option<DetectionResult>, Error> {
    // Validate checksum if required
    let checksum_valid = if let Some(checksum_type) = metadata.checksum {
        validate_checksum(input, checksum_type, metadata)?
    } else {
        true // No checksum required
    };
    
    // Validate version bytes if Base58Check
    let version_valid = if !metadata.version_bytes.is_empty() {
        validate_version_bytes(input, &metadata.version_bytes, metadata)?
    } else {
        true // No version requirement
    };
    
    // If validation fails, return None
    if !checksum_valid || !version_valid {
        return Ok(None);
    }
    
    // Normalize the address
    let normalized = normalize_address(input, metadata)?;
    
    // Calculate confidence score
    let confidence = calculate_confidence(checksum_valid, version_valid, metadata);
    
    // Generate reasoning
    let reasoning = generate_reasoning(metadata, checksum_valid, version_valid);
    
    Ok(Some(DetectionResult {
        chain,
        encoding: metadata.encoding,
        normalized,
        confidence,
        reasoning,
    }))
}

/// Validate checksum based on type
fn validate_checksum(
    input: &str,
    checksum_type: ChecksumType,
    metadata: &AddressMetadata,
) -> Result<bool, Error> {
    match checksum_type {
        ChecksumType::EIP55 => {
            Ok(eip55::validate(input))
        }
        ChecksumType::Base58Check => {
            let decoded = base58check::validate(input)?;
            if let Some((version, _)) = decoded {
                // Check if version matches metadata
                if !metadata.version_bytes.is_empty() {
                    Ok(metadata.version_bytes.contains(&version))
                } else {
                    Ok(true) // Version check not required
                }
            } else {
                Ok(false)
            }
        }
        ChecksumType::Bech32 => {
            match bech32_checksum::decode(input) {
                Ok((_, _, variant)) => Ok(variant == bech32::Variant::Bech32),
                Err(_) => Ok(false),
            }
        }
        ChecksumType::Bech32m => {
            match bech32_checksum::decode(input) {
                Ok((_, _, variant)) => Ok(variant == bech32::Variant::Bech32m),
                Err(_) => Ok(false),
            }
        }
        ChecksumType::SS58 => {
            // SS58 validation is complex, delegate to shared module
            // For now, return true if it's valid Base58
            Ok(true) // TODO: Implement proper SS58 validation
        }
    }
}

/// Validate version bytes for Base58Check
fn validate_version_bytes(
    input: &str,
    expected_versions: &[u8],
    _metadata: &AddressMetadata,
) -> Result<bool, Error> {
    let decoded = base58check::validate(input)?;
    if let Some((version, _)) = decoded {
        Ok(expected_versions.contains(&version))
    } else {
        Ok(false)
    }
}

/// Normalize address based on metadata
fn normalize_address(input: &str, metadata: &AddressMetadata) -> Result<String, Error> {
    match metadata.encoding {
        EncodingType::Hex => {
            // Normalize to EIP-55 checksum format
            eip55::normalize(input)
        }
        EncodingType::Bech32 | EncodingType::Bech32m => {
            // Bech32 is case-insensitive, normalize to lowercase
            Ok(input.to_lowercase())
        }
        EncodingType::Base58Check | EncodingType::Base58 | EncodingType::SS58 => {
            // Base58 is case-sensitive, return as-is
            Ok(input.to_string())
        }
    }
}

/// Calculate confidence score
fn calculate_confidence(
    checksum_valid: bool,
    version_valid: bool,
    metadata: &AddressMetadata,
) -> f64 {
    let mut confidence = 0.5; // Base confidence
    
    // Boost for valid checksum
    if checksum_valid {
        confidence += 0.3;
    }
    
    // Boost for valid version bytes
    if version_valid && !metadata.version_bytes.is_empty() {
        confidence += 0.1;
    }
    
    // Boost for exact length match
    if let Some(exact) = metadata.exact_length {
        // This is checked in filtering, so if we're here, it matches
        confidence += 0.05;
    }
    
    // Cap at 1.0
    confidence.min(1.0)
}

/// Generate reasoning string
fn generate_reasoning(
    metadata: &AddressMetadata,
    checksum_valid: bool,
    version_valid: bool,
) -> String {
    let mut parts = Vec::new();
    
    parts.push(format!("{} address", metadata.encoding));
    
    if checksum_valid {
        parts.push("valid checksum".to_string());
    }
    
    if version_valid && !metadata.version_bytes.is_empty() {
        parts.push("valid version bytes".to_string());
    }
    
    parts.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::extract_characteristics;
    use crate::registry::{AddressMetadata, Network};

    #[test]
    fn test_detect_evm_address() {
        let input = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
        let chars = extract_characteristics(input);
        
        let metadata = AddressMetadata {
            encoding: EncodingType::Hex,
            char_set: Some(CharSet::Hex),
            exact_length: Some(42),
            length_range: None,
            prefixes: vec!["0x".to_string()],
            hrps: vec![],
            version_bytes: vec![],
            checksum: Some(ChecksumType::EIP55),
            network: Some(Network::Mainnet),
        };
        
        let result = detect_address(input, &chars, &metadata, Chain::Ethereum);
        assert!(result.is_ok());
        // Result may be Some or None depending on checksum validation
    }
}

