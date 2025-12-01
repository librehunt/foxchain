//! Public key detection
//!
//! This module detects public keys in various formats (hex, base58, bech32).

use crate::Error;
use bech32;

/// Public key format
#[derive(Debug, Clone, PartialEq)]
pub enum PublicKeyFormat {
    /// Hex-encoded public key (compressed or uncompressed)
    Hex,
    /// Base58-encoded public key
    Base58,
    /// Bech32-encoded public key
    Bech32,
}

/// Public key type
#[derive(Debug, Clone, PartialEq)]
pub enum PublicKeyType {
    /// secp256k1 public key (used by Bitcoin, EVM)
    Secp256k1,
    /// Ed25519 public key (used by Solana, Cosmos)
    Ed25519,
    /// Unknown public key type
    #[allow(dead_code)]
    Unknown,
}

/// Detect if input is a public key
///
/// Returns the format, key bytes, and key type if detected.
pub fn detect(input: &str) -> Result<Option<(PublicKeyFormat, Vec<u8>, PublicKeyType)>, Error> {
    // Try to detect public key format
    match detect_hex_public_key(input)? {
        Some((bytes, key_type)) => Ok(Some((PublicKeyFormat::Hex, bytes, key_type))),
        None => match detect_base58_public_key(input)? {
            Some((bytes, key_type)) => Ok(Some((PublicKeyFormat::Base58, bytes, key_type))),
            None => match detect_bech32_public_key(input)? {
                Some((bytes, key_type)) => Ok(Some((PublicKeyFormat::Bech32, bytes, key_type))),
                None => Ok(None),
            },
        },
    }
}

/// Detect hex-encoded public key
///
/// Supports:
/// - Uncompressed secp256k1: 65 bytes (0x04 prefix + 64 bytes)
/// - Compressed secp256k1: 33 bytes (0x02 or 0x03 prefix + 32 bytes)
/// - Ed25519: 32 bytes (no prefix)
pub fn detect_hex_public_key(input: &str) -> Result<Option<(Vec<u8>, PublicKeyType)>, Error> {
    // Remove 0x prefix if present
    let hex_str = input.strip_prefix("0x").unwrap_or(input);

    // Must be valid hex
    if !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(None);
    }

    // Must be even length
    if !hex_str.len().is_multiple_of(2) {
        return Ok(None);
    }

    use crate::shared::encoding::hex;
    let bytes = hex::decode(hex_str).map_err(Error::InvalidInput)?;

    // Check for secp256k1 public keys
    if bytes.len() == 65 && bytes[0] == 0x04 {
        // Uncompressed secp256k1
        return Ok(Some((bytes, PublicKeyType::Secp256k1)));
    }

    if bytes.len() == 33 && (bytes[0] == 0x02 || bytes[0] == 0x03) {
        // Compressed secp256k1
        return Ok(Some((bytes, PublicKeyType::Secp256k1)));
    }

    // Check for Ed25519 public keys (32 bytes, no specific prefix)
    if bytes.len() == 32 {
        // Could be Ed25519, but we can't be 100% sure
        // We'll treat it as Ed25519 for now
        return Ok(Some((bytes, PublicKeyType::Ed25519)));
    }

    Ok(None)
}

/// Detect base58-encoded public key
pub fn detect_base58_public_key(input: &str) -> Result<Option<(Vec<u8>, PublicKeyType)>, Error> {
    use base58::FromBase58;
    // Try to decode as base58
    let bytes = match input.from_base58() {
        Ok(b) => b,
        Err(_) => return Ok(None),
    };

    // Check for secp256k1 public keys
    if bytes.len() == 65 && bytes[0] == 0x04 {
        return Ok(Some((bytes, PublicKeyType::Secp256k1)));
    }

    if bytes.len() == 33 && (bytes[0] == 0x02 || bytes[0] == 0x03) {
        return Ok(Some((bytes, PublicKeyType::Secp256k1)));
    }

    // Check for Ed25519 public keys (32 bytes)
    if bytes.len() == 32 {
        return Ok(Some((bytes, PublicKeyType::Ed25519)));
    }

    Ok(None)
}

/// Detect bech32-encoded public key
pub fn detect_bech32_public_key(input: &str) -> Result<Option<(Vec<u8>, PublicKeyType)>, Error> {
    // Try to decode as bech32
    let (_hrp, data, _variant) = match bech32::decode(input) {
        Ok(result) => result,
        Err(_) => return Ok(None),
    };

    // Convert 5-bit groups to bytes
    let bytes = bech32::convert_bits(&data, 5, 8, false)
        .map_err(|e| Error::InvalidInput(format!("Bech32 conversion error: {}", e)))?;

    // Check for known public key HRPs
    // Common HRPs: "npub" (Nostr), "pub" (generic), etc.
    // For now, we'll accept any bech32 with valid key length
    if bytes.len() == 32 {
        // Likely Ed25519
        return Ok(Some((bytes, PublicKeyType::Ed25519)));
    }

    if bytes.len() == 33 || bytes.len() == 65 {
        // Likely secp256k1
        return Ok(Some((bytes, PublicKeyType::Secp256k1)));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_hex_public_key_uncompressed() {
        // Uncompressed secp256k1 public key (65 bytes: 0x04 + 64 bytes)
        let key_hex = "0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
        let result = detect_hex_public_key(key_hex).unwrap();
        assert!(result.is_some());
        let (bytes, key_type) = result.unwrap();
        assert_eq!(bytes.len(), 65);
        assert_eq!(key_type, PublicKeyType::Secp256k1);
    }

    #[test]
    fn test_detect_hex_public_key_compressed() {
        // Compressed secp256k1 public key (33 bytes: 0x02/0x03 + 32 bytes)
        let key_hex = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let result = detect_hex_public_key(key_hex).unwrap();
        assert!(result.is_some());
        let (bytes, key_type) = result.unwrap();
        assert_eq!(bytes.len(), 33);
        assert_eq!(key_type, PublicKeyType::Secp256k1);
    }

    #[test]
    fn test_detect_hex_public_key_ed25519() {
        // Ed25519 public key (32 bytes)
        let key_hex = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let result = detect_hex_public_key(key_hex).unwrap();
        assert!(result.is_some());
        let (bytes, key_type) = result.unwrap();
        assert_eq!(bytes.len(), 32);
        assert_eq!(key_type, PublicKeyType::Ed25519);
    }

    #[test]
    fn test_detect_hex_public_key_with_prefix() {
        // Hex with 0x prefix
        let key_hex = "0x0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
        let result = detect_hex_public_key(key_hex).unwrap();
        assert!(result.is_some());
    }

    #[test]
    fn test_detect_hex_public_key_invalid() {
        // Invalid hex
        let key_hex = "not-hex";
        let result = detect_hex_public_key(key_hex).unwrap();
        assert!(result.is_none());
    }
}
