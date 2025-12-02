//! Unified metadata-driven public key detector
//!
//! This module provides a unified public key detector that uses metadata instead
//! of hardcoded format checks.

use crate::input::InputCharacteristics;
use crate::registry::{Chain, EncodingType, PublicKeyMetadata, PublicKeyType};
use crate::Error;
use crate::shared::encoding::{base58, bech32 as bech32_encoding, hex};

/// Detect public key using metadata
pub fn detect_public_key(
    input: &str,
    chars: &InputCharacteristics,
    metadata: &PublicKeyMetadata,
    chain: Chain,
) -> Result<Option<super::address::DetectionResult>, Error> {
    // Decode based on encoding type
    let key_bytes = match metadata.encoding {
        EncodingType::Hex => {
            hex::decode(input)?
        }
        EncodingType::Base58 => {
            base58::decode(input)?
        }
        EncodingType::Bech32 | EncodingType::Bech32m => {
            let (_, data, _) = bech32_encoding::decode(input)?;
            // Convert u5 vector to bytes
            // u5 values are 0-31, we need to convert 5-bit groups to 8-bit bytes
            let u5_bytes: Vec<u8> = data.iter().map(|u5| u8::from(*u5)).collect();
            bech32_encoding::convert_bits(&u5_bytes, 5, 8, false)?
        }
        EncodingType::Base58Check => {
            // For Base58Check, we need to validate and extract payload
            // For now, just decode as Base58
            base58::decode(input)?
        }
        EncodingType::SS58 => {
            // SS58 decoding is complex, delegate to shared module
            // For now, just decode as Base58
            base58::decode(input)?
        }
    };
    
    // Validate key type
    let key_type_valid = validate_key_type(&key_bytes, metadata.key_type)?;
    if !key_type_valid {
        return Ok(None);
    }
    
    // Validate key length
    let length_valid = validate_key_length(&key_bytes, metadata)?;
    if !length_valid {
        return Ok(None);
    }
    
    // Normalize the public key
    let normalized = normalize_public_key(input, metadata)?;
    
    // Calculate confidence score
    let confidence = calculate_confidence(metadata);
    
    // Generate reasoning
    let reasoning = generate_reasoning(metadata);
    
    Ok(Some(super::address::DetectionResult {
        chain,
        encoding: metadata.encoding,
        normalized,
        confidence,
        reasoning,
    }))
}

/// Validate key type matches metadata
fn validate_key_type(key_bytes: &[u8], expected_type: PublicKeyType) -> Result<bool, Error> {
    match expected_type {
        PublicKeyType::Secp256k1 => {
            // secp256k1 keys are 33 bytes (compressed) or 65 bytes (uncompressed)
            Ok(key_bytes.len() == 33 || key_bytes.len() == 65)
        }
        PublicKeyType::Ed25519 => {
            // Ed25519 keys are exactly 32 bytes
            Ok(key_bytes.len() == 32)
        }
        PublicKeyType::Sr25519 => {
            // sr25519 keys are exactly 32 bytes (indistinguishable from Ed25519)
            Ok(key_bytes.len() == 32)
        }
    }
}

/// Validate key length matches metadata
fn validate_key_length(key_bytes: &[u8], metadata: &PublicKeyMetadata) -> Result<bool, Error> {
    let length = key_bytes.len();
    
    if let Some(exact) = metadata.exact_length {
        return Ok(length == exact);
    }
    
    if let Some((min, max)) = metadata.length_range {
        return Ok(length >= min && length <= max);
    }
    
    // No length requirement
    Ok(true)
}

/// Normalize public key based on metadata
fn normalize_public_key(input: &str, metadata: &PublicKeyMetadata) -> Result<String, Error> {
    match metadata.encoding {
        EncodingType::Hex => {
            // Hex keys: normalize to lowercase with 0x prefix
            if input.starts_with("0x") {
                Ok(input.to_lowercase())
            } else {
                Ok(format!("0x{}", input.to_lowercase()))
            }
        }
        EncodingType::Bech32 | EncodingType::Bech32m => {
            // Bech32 is case-insensitive, normalize to lowercase
            Ok(input.to_lowercase())
        }
        EncodingType::Base58 | EncodingType::Base58Check | EncodingType::SS58 => {
            // Base58 is case-sensitive, return as-is
            Ok(input.to_string())
        }
    }
}

/// Calculate confidence score
fn calculate_confidence(metadata: &PublicKeyMetadata) -> f64 {
    let mut confidence = 0.7; // Base confidence for public keys (lower than addresses)
    
    // Boost for exact length match
    if metadata.exact_length.is_some() {
        confidence += 0.1;
    }
    
    // Boost for checksum validation
    if metadata.checksum.is_some() {
        confidence += 0.1;
    }
    
    // Cap at 1.0
    confidence.min(1.0)
}

/// Generate reasoning string
fn generate_reasoning(metadata: &PublicKeyMetadata) -> String {
    format!(
        "{} {} public key",
        metadata.encoding,
        match metadata.key_type {
            PublicKeyType::Secp256k1 => "secp256k1",
            PublicKeyType::Ed25519 => "Ed25519",
            PublicKeyType::Sr25519 => "sr25519",
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::extract_characteristics;
    use crate::registry::{CharSet, PublicKeyMetadata};

    #[test]
    fn test_detect_hex_secp256k1_key() {
        let input = "0x0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
        let chars = extract_characteristics(input);
        
        let metadata = PublicKeyMetadata {
            encoding: EncodingType::Hex,
            char_set: Some(CharSet::Hex),
            exact_length: None,
            length_range: Some((33, 65)),
            prefixes: vec!["0x".to_string()],
            hrps: vec![],
            key_type: PublicKeyType::Secp256k1,
            checksum: None,
        };
        
        let result = detect_public_key(input, &chars, &metadata, Chain::Ethereum);
        assert!(result.is_ok());
    }
}

