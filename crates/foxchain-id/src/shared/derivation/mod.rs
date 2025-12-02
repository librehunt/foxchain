//! Public key decoding utilities
//!
//! This module provides utilities for decoding public keys from various encodings.
//! Address derivation is now handled by the pipeline system.

use crate::Error;
use crate::input::DetectedKeyType;

/// Decode public key from input string based on encoding
pub fn decode_public_key(
    input: &str,
    chars: &crate::input::InputCharacteristics,
    key_type: DetectedKeyType,
) -> Result<Vec<u8>, Error> {
    use crate::shared::encoding::{base58, bech32 as bech32_encoding, hex};
    
    // Try all possible encodings to decode the input
    let mut bytes = None;
    let mut last_error = None;
    
    for encoding in &chars.encoding {
        let decoded = match encoding {
            crate::registry::EncodingType::Hex => {
                hex::decode(input).map_err(|e| Error::InvalidInput(format!("Hex decode error: {}", e)))
            }
            crate::registry::EncodingType::Base58 | crate::registry::EncodingType::Base58Check => {
                base58::decode(input).map_err(|e| Error::InvalidInput(format!("Base58 decode error: {}", e)))
            }
            crate::registry::EncodingType::Bech32 | crate::registry::EncodingType::Bech32m => {
                let (_, data, _) = bech32_encoding::decode(input)
                    .map_err(|e| Error::InvalidInput(format!("Bech32 decode error: {}", e)))?;
                let u5_bytes: Vec<u8> = data.iter().map(|u5| u8::from(*u5)).collect();
                bech32_encoding::convert_bits(&u5_bytes, 5, 8, false)
                    .map_err(|e| Error::InvalidInput(format!("Bit conversion error: {}", e)))
            }
            crate::registry::EncodingType::SS58 => {
                base58::decode(input).map_err(|e| Error::InvalidInput(format!("Base58 decode error: {}", e)))
            }
        };
        
        match decoded {
            Ok(decoded_bytes) => {
                bytes = Some(decoded_bytes);
                break; // Use first successful decode
            }
            Err(e) => {
                last_error = Some(e);
            }
        }
    }
    
    let bytes = bytes.ok_or_else(|| {
        last_error.unwrap_or_else(|| Error::InvalidInput("Unknown encoding type or decode failed".to_string()))
    })?;
    
    // Validate key length matches key type
    match key_type {
        DetectedKeyType::Secp256k1 { .. } => {
            if bytes.len() != 33 && bytes.len() != 65 && bytes.len() != 64 {
                return Err(Error::InvalidInput(format!(
                    "Invalid secp256k1 key length: {} bytes",
                    bytes.len()
                )));
            }
        }
        DetectedKeyType::Ed25519 | DetectedKeyType::Sr25519 => {
            if bytes.len() != 32 {
                return Err(Error::InvalidInput(format!(
                    "Invalid Ed25519/sr25519 key length: {} bytes (expected 32)",
                    bytes.len()
                )));
            }
        }
    }
    
    Ok(bytes)
}
