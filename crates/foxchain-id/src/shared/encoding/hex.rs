//! Hex encoding utilities

use hex;

/// Decode a hex string (with or without 0x prefix) to bytes
pub fn decode(input: &str) -> Result<Vec<u8>, String> {
    let hex_str = input.strip_prefix("0x").unwrap_or(input);
    hex::decode(hex_str).map_err(|e| format!("Invalid hex: {}", e))
}

/// Encode bytes to hex string with 0x prefix
pub fn encode(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_with_prefix() {
        let input = "0x1234";
        let result = decode(input);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes, vec![0x12, 0x34]);
    }

    #[test]
    fn test_decode_without_prefix() {
        let input = "1234";
        let result = decode(input);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes, vec![0x12, 0x34]);
    }

    #[test]
    fn test_decode_invalid_hex() {
        let input = "0xgg";
        let result = decode(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid hex"));
    }

    #[test]
    fn test_decode_odd_length() {
        let input = "0x123";
        let result = decode(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode() {
        let bytes = vec![0x12, 0x34, 0xAB, 0xCD];
        let result = encode(&bytes);
        assert_eq!(result, "0x1234abcd");
        assert!(result.starts_with("0x"));
    }

    #[test]
    fn test_encode_empty() {
        let bytes = vec![];
        let result = encode(&bytes);
        assert_eq!(result, "0x");
    }

    #[test]
    fn test_decode_encode_roundtrip() {
        let original = vec![0x00, 0xFF, 0x12, 0x34, 0xAB, 0xCD];
        let encoded = encode(&original);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_decode_evm_address() {
        // Test with a valid EVM address
        let input = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
        let result = decode(input);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 20); // EVM addresses are 20 bytes
    }
}
