//! Base58 encoding utilities

use base58::FromBase58;

/// Decode a Base58 string to bytes
pub fn decode(input: &str) -> Result<Vec<u8>, String> {
    input
        .from_base58()
        .map_err(|_| "Invalid Base58 encoding".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_valid_base58() {
        let input = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
        let result = decode(input);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 25); // Bitcoin address length
    }

    #[test]
    fn test_decode_invalid_base58() {
        let input = "0OIl"; // Contains invalid characters (0, O, I, l)
        let result = decode(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid Base58 encoding"));
    }

    #[test]
    fn test_decode_empty() {
        let input = "";
        let result = decode(input);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_decode_solana_address() {
        let input = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
        let result = decode(input);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes.len(), 32); // Solana address length
    }
}
