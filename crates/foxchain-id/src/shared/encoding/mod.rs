//! Encoding utilities (Base58, Bech32, Hex, SS58)

pub mod base58;
pub mod bech32;
pub mod hex;
pub mod ss58;

use crate::registry::EncodingType;

/// Decode input to bytes based on encoding type
///
/// This is a unified decoder that handles all encoding types.
/// Returns None if the encoding type is not recognized or decoding fails.
pub fn decode_to_bytes(input: &str, encoding: Option<EncodingType>) -> Option<Vec<u8>> {
    match encoding {
        Some(EncodingType::Hex) => hex::decode(input).ok(),
        Some(EncodingType::Base58) | Some(EncodingType::Base58Check) => {
            base58::decode(input).ok()
        }
        Some(EncodingType::Bech32) | Some(EncodingType::Bech32m) => {
            let (_, data, _) = bech32::decode(input).ok()?;
            // Convert u5 to bytes
            let u5_bytes: Vec<u8> = data.iter().map(|u5| u8::from(*u5)).collect();
            bech32::convert_bits(&u5_bytes, 5, 8, false).ok()
        }
        Some(EncodingType::SS58) => {
            // For SS58, try base58 decode first
            base58::decode(input).ok()
        }
        _ => None,
    }
}
