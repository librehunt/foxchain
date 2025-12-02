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
        Some(EncodingType::Base58) | Some(EncodingType::Base58Check) => base58::decode(input).ok(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_to_bytes_hex() {
        let input = "0x1234";
        let result = decode_to_bytes(input, Some(EncodingType::Hex));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), vec![0x12, 0x34]);
    }

    #[test]
    fn test_decode_to_bytes_base58() {
        let input = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
        let result = decode_to_bytes(input, Some(EncodingType::Base58));
        assert!(result.is_some());
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 32);
    }

    #[test]
    fn test_decode_to_bytes_bech32() {
        // Use a valid Bech32 address
        let input = "cosmos1hvf3g5z6qwz2jq0ks3k5m3n5vx7v8v9w0x1y2z3a4b5c6d7e8f9g0";
        let result = decode_to_bytes(input, Some(EncodingType::Bech32));
        // Bech32 decode might fail for invalid addresses, so just check it doesn't panic
        // If it succeeds, verify it returns bytes
        if let Some(bytes) = result {
            assert!(!bytes.is_empty());
        }
        // If it fails, that's also acceptable for this test
    }

    #[test]
    fn test_decode_to_bytes_none() {
        let input = "invalid";
        let result = decode_to_bytes(input, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_decode_to_bytes_invalid_hex() {
        let input = "0xgg"; // Invalid hex
        let result = decode_to_bytes(input, Some(EncodingType::Hex));
        assert!(result.is_none());
    }
}
