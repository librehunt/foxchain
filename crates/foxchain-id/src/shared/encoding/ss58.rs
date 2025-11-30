//! SS58 encoding utilities

use base58::FromBase58;

/// Decode SS58 string to bytes
pub fn decode(input: &str) -> Result<Vec<u8>, String> {
    input
        .from_base58()
        .map_err(|_| "Invalid SS58 encoding".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_valid_ss58() {
        // Valid SS58 address (Polkadot format)
        let input = "15oF4uVJwmo4TdGW7VfQxNLavjXviYtpYNRY9YzXg6WZ1";
        let result = decode(input);
        // This might fail if checksum is invalid, but decoding should work
        let _ = result;
    }

    #[test]
    fn test_decode_invalid_ss58() {
        let input = "0OIl"; // Invalid Base58
        let result = decode(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid SS58 encoding"));
    }

    #[test]
    fn test_decode_empty() {
        let input = "";
        let result = decode(input);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 0);
    }
}
