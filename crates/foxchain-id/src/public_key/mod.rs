//! Public key detection and address derivation
//!
//! This module detects public keys in various formats (hex, base58, bech32) and
//! derives addresses for supported blockchains.

use crate::shared::crypto::hash::{double_sha256, hash160, keccak256, sha256};
use crate::shared::encoding::bech32 as bech32_encoding;
use crate::{Chain, ChainCandidate, Error, IdentificationResult};
use base58::{FromBase58, ToBase58};
use bech32::{self, u5, Variant};

/// Public key format
#[derive(Debug, Clone, PartialEq)]
enum PublicKeyFormat {
    /// Hex-encoded public key (compressed or uncompressed)
    Hex,
    /// Base58-encoded public key
    Base58,
    /// Bech32-encoded public key
    Bech32,
}

/// Public key type
#[derive(Debug, Clone, PartialEq)]
enum PublicKeyType {
    /// secp256k1 public key (used by Bitcoin, EVM)
    Secp256k1,
    /// Ed25519 public key (used by Solana, Cosmos)
    Ed25519,
    /// Unknown public key type
    #[allow(dead_code)]
    Unknown,
}

/// Detect if input is a public key and derive addresses
pub fn detect_public_key(input: &str) -> Result<Option<IdentificationResult>, Error> {
    // Try to detect public key format
    let (format, key_bytes, key_type) = match detect_hex_public_key(input)? {
        Some((bytes, key_type)) => (PublicKeyFormat::Hex, bytes, key_type),
        None => match detect_base58_public_key(input)? {
            Some((bytes, key_type)) => (PublicKeyFormat::Base58, bytes, key_type),
            None => match detect_bech32_public_key(input)? {
                Some((bytes, key_type)) => (PublicKeyFormat::Bech32, bytes, key_type),
                None => return Ok(None),
            },
        },
    };

    // Derive addresses based on public key type
    let mut candidates = Vec::new();

    match key_type {
        PublicKeyType::Secp256k1 => {
            // EVM address derivation
            if derive_evm_address(&key_bytes)?.is_some() {
                candidates.push(ChainCandidate {
                    chain: Chain::Ethereum,
                    confidence: 0.85,
                    reasoning: format!(
                        "EVM address derived from {} secp256k1 public key",
                        match format {
                            PublicKeyFormat::Hex => "hex",
                            PublicKeyFormat::Base58 => "base58",
                            PublicKeyFormat::Bech32 => "bech32",
                        }
                    ),
                });
            }

            // Bitcoin address derivation
            let bitcoin_addresses = derive_bitcoin_addresses(&key_bytes)?;
            for (chain, _address) in bitcoin_addresses {
                candidates.push(ChainCandidate {
                    chain,
                    confidence: 0.80,
                    reasoning: format!(
                        "Bitcoin address derived from {} secp256k1 public key",
                        match format {
                            PublicKeyFormat::Hex => "hex",
                            PublicKeyFormat::Base58 => "base58",
                            PublicKeyFormat::Bech32 => "bech32",
                        }
                    ),
                });
            }
        }
        PublicKeyType::Ed25519 => {
            // Solana address derivation
            if key_bytes.len() == 32 {
                candidates.push(ChainCandidate {
                    chain: Chain::Solana,
                    confidence: 0.85,
                    reasoning: format!(
                        "Solana address derived from {} Ed25519 public key",
                        match format {
                            PublicKeyFormat::Hex => "hex",
                            PublicKeyFormat::Base58 => "base58",
                            PublicKeyFormat::Bech32 => "bech32",
                        }
                    ),
                });
            }

            // Cosmos address derivation
            if derive_cosmos_address(&key_bytes)?.is_some() {
                candidates.push(ChainCandidate {
                    chain: Chain::CosmosHub,
                    confidence: 0.80,
                    reasoning: format!(
                        "Cosmos address derived from {} Ed25519 public key",
                        match format {
                            PublicKeyFormat::Hex => "hex",
                            PublicKeyFormat::Base58 => "base58",
                            PublicKeyFormat::Bech32 => "bech32",
                        }
                    ),
                });
            }
        }
        PublicKeyType::Unknown => {
            // For unknown key types, we can't derive addresses
            return Ok(None);
        }
    }

    if candidates.is_empty() {
        return Ok(None);
    }

    // Use the first derived address as normalized representation
    // For secp256k1, prefer EVM address; for Ed25519, prefer Solana
    let normalized = match key_type {
        PublicKeyType::Secp256k1 => {
            derive_evm_address(&key_bytes)?.unwrap_or_else(|| "unknown".to_string())
        }
        PublicKeyType::Ed25519 => {
            if key_bytes.len() == 32 {
                // Solana address is the public key itself (base58 encoded)
                key_bytes.as_slice().to_base58()
            } else {
                derive_cosmos_address(&key_bytes)?.unwrap_or_else(|| "unknown".to_string())
            }
        }
        PublicKeyType::Unknown => return Ok(None),
    };

    Ok(Some(IdentificationResult {
        normalized,
        candidates,
    }))
}

/// Detect hex-encoded public key
///
/// Supports:
/// - Uncompressed secp256k1: 65 bytes (0x04 prefix + 64 bytes)
/// - Compressed secp256k1: 33 bytes (0x02 or 0x03 prefix + 32 bytes)
/// - Ed25519: 32 bytes (no prefix)
fn detect_hex_public_key(input: &str) -> Result<Option<(Vec<u8>, PublicKeyType)>, Error> {
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
fn detect_base58_public_key(input: &str) -> Result<Option<(Vec<u8>, PublicKeyType)>, Error> {
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
fn detect_bech32_public_key(input: &str) -> Result<Option<(Vec<u8>, PublicKeyType)>, Error> {
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

/// Derive EVM address from secp256k1 public key
///
/// Process:
/// 1. Take public key (compressed or uncompressed)
/// 2. If compressed, decompress (we'll skip this for now and use uncompressed)
/// 3. Compute Keccak-256 hash
/// 4. Take last 20 bytes
/// 5. Format as 0x-prefixed hex
fn derive_evm_address(public_key: &[u8]) -> Result<Option<String>, Error> {
    // For compressed keys, we'd need to decompress, but for simplicity
    // we'll only handle uncompressed keys (65 bytes)
    let key_bytes = if public_key.len() == 33 {
        // Compressed key - we'd need to decompress, but that's complex
        // For now, return None for compressed keys
        return Ok(None);
    } else if public_key.len() == 65 && public_key[0] == 0x04 {
        // Uncompressed key - skip the 0x04 prefix
        &public_key[1..]
    } else {
        return Ok(None);
    };

    // Compute Keccak-256 hash
    let hash = keccak256(key_bytes);

    // Take last 20 bytes
    let address_bytes = &hash[12..32];

    // Format as hex with 0x prefix
    use crate::shared::encoding::hex;
    Ok(Some(hex::encode(address_bytes)))
}

/// Derive Bitcoin addresses from secp256k1 public key
///
/// Returns list of (chain, address) pairs for different Bitcoin address formats
fn derive_bitcoin_addresses(public_key: &[u8]) -> Result<Vec<(Chain, String)>, Error> {
    let mut addresses = Vec::new();

    // Get uncompressed public key bytes (skip 0x04 prefix if present)
    let key_bytes = if public_key.len() == 65 && public_key[0] == 0x04 {
        &public_key[1..]
    } else if public_key.len() == 64 {
        public_key
    } else {
        // Compressed keys need decompression - skip for now
        return Ok(addresses);
    };

    // Compute hash160: RIPEMD160(SHA256(public_key))
    let hash160_bytes = hash160(key_bytes);

    // Derive P2PKH address (version 0x00 for Bitcoin mainnet)
    let p2pkh_address = derive_p2pkh_address(&hash160_bytes, 0x00)?;
    if let Some(addr) = p2pkh_address {
        addresses.push((Chain::Bitcoin, addr));
    }

    Ok(addresses)
}

/// Derive P2PKH address from hash160
fn derive_p2pkh_address(hash160: &[u8], version: u8) -> Result<Option<String>, Error> {
    if hash160.len() != 20 {
        return Ok(None);
    }

    // Create payload: version + hash160
    let mut payload = vec![version];
    payload.extend_from_slice(hash160);

    // Compute checksum: first 4 bytes of SHA256(SHA256(payload))
    let checksum_hash = double_sha256(&payload);
    let checksum = &checksum_hash[..4];

    // Combine payload + checksum
    let mut full = payload;
    full.extend_from_slice(checksum);

    // Encode in base58
    Ok(Some(full.as_slice().to_base58()))
}

/// Derive Cosmos address from Ed25519 public key
///
/// Process:
/// 1. Compute SHA256 hash of public key
/// 2. Take first 20 bytes
/// 3. Encode as Bech32 with "cosmos" HRP
fn derive_cosmos_address(public_key: &[u8]) -> Result<Option<String>, Error> {
    if public_key.len() != 32 {
        return Ok(None);
    }

    // Compute SHA256 hash
    let hash = sha256(public_key);

    // Take first 20 bytes
    let address_bytes = &hash[..20];

    // Convert to 5-bit groups
    let data = bech32_encoding::convert_bits(address_bytes, 8, 5, true)
        .map_err(|e| Error::InvalidInput(format!("Bech32 conversion error: {}", e)))?;

    // Convert Vec<u8> to Vec<u5> for bech32 encoding
    let data_u5: Vec<u5> = bech32_encoding::bytes_to_u5(&data);

    // Encode as Bech32 with "cosmos" HRP
    let address = bech32_encoding::encode("cosmos", &data_u5, Variant::Bech32)
        .map_err(|e| Error::InvalidInput(format!("Bech32 encoding error: {}", e)))?;

    Ok(Some(address))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_hex_public_key_uncompressed() {
        // Uncompressed secp256k1 public key (65 bytes: 0x04 + 64 bytes)
        // Using a valid hex string with correct length (130 hex chars = 65 bytes)
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
        // Using a valid hex string with correct length (66 hex chars = 33 bytes)
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
        // Using a valid hex string with correct length (64 hex chars = 32 bytes)
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

    #[test]
    fn test_derive_evm_address() {
        // Test with uncompressed public key
        let key_bytes = vec![
            0x04, 0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce,
            0x87, 0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81,
            0x5b, 0x16, 0xf8, 0x17, 0x98, 0x48, 0x3a, 0xda, 0x77, 0x26, 0xa3, 0xc4, 0x65, 0x5d,
            0xa4, 0xfb, 0xfc, 0x0e, 0x11, 0x08, 0xa8, 0xfd, 0x17, 0xb4, 0x48, 0xa6, 0x85, 0x54,
            0x19, 0x9c, 0x47, 0xd0, 0x8f, 0xfb, 0x10, 0xd4, 0xb8,
        ];
        let result = derive_evm_address(&key_bytes).unwrap();
        assert!(result.is_some());
        let address = result.unwrap();
        assert!(address.starts_with("0x"));
        assert_eq!(address.len(), 42);
    }

    #[test]
    fn test_derive_bitcoin_addresses() {
        // Test with uncompressed public key
        let key_bytes = vec![
            0x04, 0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce,
            0x87, 0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81,
            0x5b, 0x16, 0xf8, 0x17, 0x98, 0x48, 0x3a, 0xda, 0x77, 0x26, 0xa3, 0xc4, 0x65, 0x5d,
            0xa4, 0xfb, 0xfc, 0x0e, 0x11, 0x08, 0xa8, 0xfd, 0x17, 0xb4, 0x48, 0xa6, 0x85, 0x54,
            0x19, 0x9c, 0x47, 0xd0, 0x8f, 0xfb, 0x10, 0xd4, 0xb8,
        ];
        let result = derive_bitcoin_addresses(&key_bytes).unwrap();
        assert!(!result.is_empty());
        assert_eq!(result[0].0, Chain::Bitcoin);
    }

    #[test]
    fn test_derive_cosmos_address() {
        // Test with Ed25519 public key (32 bytes)
        let key_bytes = vec![0u8; 32];
        let result = derive_cosmos_address(&key_bytes).unwrap();
        assert!(result.is_some());
        let address = result.unwrap();
        assert!(address.starts_with("cosmos1"));
    }

    #[test]
    fn test_detect_public_key_hex_secp256k1() {
        // Test full detection flow with hex secp256k1 public key
        let key_hex = "0x0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
        let result = detect_public_key(key_hex).unwrap();
        assert!(result.is_some());
        let id_result = result.unwrap();
        assert!(!id_result.candidates.is_empty());
        // Should have EVM and Bitcoin candidates
        assert!(id_result
            .candidates
            .iter()
            .any(|c| matches!(c.chain, Chain::Ethereum)));
    }

    #[test]
    fn test_detect_public_key_hex_ed25519() {
        // Test full detection flow with hex Ed25519 public key
        let key_hex = "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let result = detect_public_key(key_hex).unwrap();
        assert!(result.is_some());
        let id_result = result.unwrap();
        assert!(!id_result.candidates.is_empty());
        // Should have Solana and Cosmos candidates
        assert!(id_result
            .candidates
            .iter()
            .any(|c| matches!(c.chain, Chain::Solana | Chain::CosmosHub)));
    }
}
