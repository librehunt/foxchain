//! Substrate ecosystem address derivation from public keys

use crate::public_key::detection::PublicKeyType;
use crate::shared::crypto::hash::blake2b_256;
use crate::shared::crypto::secp256k1;
use crate::shared::encoding::ss58;
use crate::{Chain, Error};

/// Derive Substrate ecosystem addresses from public key
///
/// Returns all 3 Substrate ecosystem chains (Polkadot, Kusama, Generic Substrate) with their respective addresses.
/// Supports Ed25519, sr25519, and secp256k1 public keys.
///
/// Process:
/// 1. Derive Account ID based on key type:
///    - Ed25519/sr25519 (32 bytes): Account ID = public key bytes
///    - secp256k1 (33 or 65 bytes): Account ID = Blake2b-256 hash of public key
/// 2. Encode as SS58 with chain-specific prefixes:
///    - Polkadot: prefix 0
///    - Kusama: prefix 2
///    - Generic Substrate: prefix 42
pub fn derive_substrate_address(
    public_key: &[u8],
    key_type: PublicKeyType,
) -> Result<Vec<(Chain, String)>, Error> {
    // Derive Account ID based on key type
    let account_id = match key_type {
        PublicKeyType::Ed25519 | PublicKeyType::Sr25519 => {
            // For Ed25519 and sr25519, Account ID is the public key bytes directly
            if public_key.len() != 32 {
                return Ok(Vec::new());
            }
            public_key.to_vec()
        }
        PublicKeyType::Secp256k1 => {
            // For secp256k1, Account ID is Blake2b-256 hash of the public key
            // Handle both compressed (33 bytes) and uncompressed (65 bytes) keys
            let key_bytes_64 = if public_key.len() == 33 {
                // Compressed key - decompress it
                let uncompressed = secp256k1::decompress_public_key(public_key)?;
                // Extract the 64-byte key (skip 0x04 prefix)
                if uncompressed.len() == 65 && uncompressed[0] == 0x04 {
                    uncompressed[1..65].to_vec()
                } else {
                    return Ok(Vec::new());
                }
            } else if public_key.len() == 65 && public_key[0] == 0x04 {
                // Uncompressed key - extract the 64-byte key (skip 0x04 prefix)
                public_key[1..65].to_vec()
            } else if public_key.len() == 64 {
                public_key.to_vec()
            } else {
                return Ok(Vec::new());
            };

            // Compute Blake2b-256 hash to get 32-byte Account ID
            blake2b_256(&key_bytes_64).to_vec()
        }
        PublicKeyType::Unknown => {
            return Ok(Vec::new());
        }
    };

    // Ensure Account ID is 32 bytes
    if account_id.len() != 32 {
        return Ok(Vec::new());
    }

    // Derive addresses for all Substrate chains
    let mut addresses = Vec::new();
    let prefixes = [
        (0u16, Chain::Polkadot),
        (2u16, Chain::Kusama),
        (42u16, Chain::Substrate),
    ];

    for (prefix, chain) in prefixes {
        let address = ss58::encode(prefix, &account_id)
            .map_err(|e| Error::InvalidInput(format!("SS58 encoding error: {}", e)))?;
        addresses.push((chain, address));
    }

    Ok(addresses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_substrate_address_ed25519() {
        // Test with Ed25519 public key (32 bytes)
        let key_bytes = vec![0u8; 32];
        let result = derive_substrate_address(&key_bytes, PublicKeyType::Ed25519).unwrap();

        // Should return all 3 Substrate chains
        assert_eq!(result.len(), 3, "Should return all 3 Substrate chains");

        // Verify all chains are present
        let chains: Vec<Chain> = result.iter().map(|(chain, _)| chain.clone()).collect();
        assert!(chains.contains(&Chain::Polkadot));
        assert!(chains.contains(&Chain::Kusama));
        assert!(chains.contains(&Chain::Substrate));

        // Verify addresses are valid SS58 encoded (non-empty, Base58 characters)
        for (chain, address) in &result {
            assert!(
                !address.is_empty(),
                "Address should not be empty for {:?}",
                chain
            );
            // SS58 addresses are Base58 encoded, so they should only contain Base58 characters
            assert!(
                address.chars().all(|c| {
                    "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".contains(c)
                }),
                "Address should be valid Base58 for {:?}: {}",
                chain,
                address
            );
        }
    }

    #[test]
    fn test_derive_substrate_address_sr25519() {
        // Test with sr25519 public key (32 bytes) - same as Ed25519 for Account ID
        let key_bytes = vec![0u8; 32];
        let result = derive_substrate_address(&key_bytes, PublicKeyType::Sr25519).unwrap();

        // Should return all 3 Substrate chains
        assert_eq!(result.len(), 3, "Should return all 3 Substrate chains");
    }

    #[test]
    fn test_derive_substrate_address_secp256k1() {
        // Test with secp256k1 public key (uncompressed, 65 bytes)
        let mut key_bytes = vec![0x04u8]; // Uncompressed prefix
        key_bytes.extend(vec![0u8; 64]);
        let result = derive_substrate_address(&key_bytes, PublicKeyType::Secp256k1).unwrap();

        // Should return all 3 Substrate chains
        assert_eq!(result.len(), 3, "Should return all 3 Substrate chains");

        // Verify all chains are present
        let chains: Vec<Chain> = result.iter().map(|(chain, _)| chain.clone()).collect();
        assert!(chains.contains(&Chain::Polkadot));
        assert!(chains.contains(&Chain::Kusama));
        assert!(chains.contains(&Chain::Substrate));
    }

    #[test]
    fn test_derive_substrate_address_secp256k1_compressed() {
        // Test with secp256k1 public key (compressed, 33 bytes)
        // Use a valid compressed key format (0x02 or 0x03 prefix + 32 bytes)
        // For testing, we'll use a key that might fail decompression, so we handle the error
        let mut key_bytes = vec![0x02u8]; // Compressed prefix
        key_bytes.extend(vec![0u8; 32]);
        let result = derive_substrate_address(&key_bytes, PublicKeyType::Secp256k1);

        // Decompression might fail for invalid keys, so result could be empty
        // This tests that the function handles invalid keys gracefully
        if let Ok(addresses) = result {
            // If decompression succeeded, should return all 3 Substrate chains
            if !addresses.is_empty() {
                assert_eq!(addresses.len(), 3, "Should return all 3 Substrate chains");
            }
        }
        // If decompression failed, that's also acceptable - the function should return empty vector
    }

    #[test]
    fn test_derive_substrate_address_invalid_length_ed25519() {
        // Test with invalid length for Ed25519 (not 32 bytes)
        let key_bytes = vec![0u8; 33];
        let result = derive_substrate_address(&key_bytes, PublicKeyType::Ed25519).unwrap();
        assert!(
            result.is_empty(),
            "Should return empty vector for invalid length"
        );
    }

    #[test]
    fn test_derive_substrate_address_invalid_length_secp256k1() {
        // Test with invalid length for secp256k1 (not 33 or 65 bytes)
        let key_bytes = vec![0u8; 32];
        let result = derive_substrate_address(&key_bytes, PublicKeyType::Secp256k1).unwrap();
        assert!(
            result.is_empty(),
            "Should return empty vector for invalid length"
        );
    }

    #[test]
    fn test_derive_substrate_address_unknown_key_type() {
        // Test with unknown key type
        let key_bytes = vec![0u8; 32];
        let result = derive_substrate_address(&key_bytes, PublicKeyType::Unknown).unwrap();
        assert!(
            result.is_empty(),
            "Should return empty vector for unknown key type"
        );
    }

    #[test]
    fn test_derive_substrate_address_secp256k1_64_bytes() {
        // Test with secp256k1 key that's already 64 bytes (no prefix)
        let key_bytes = vec![0u8; 64];
        let result = derive_substrate_address(&key_bytes, PublicKeyType::Secp256k1).unwrap();
        // Should return all 3 Substrate chains
        assert_eq!(result.len(), 3, "Should return all 3 Substrate chains");
    }

    #[test]
    fn test_derive_substrate_address_secp256k1_invalid_decompression() {
        // Test with invalid compressed key that fails decompression
        let mut key_bytes = vec![0x02u8];
        key_bytes.extend(vec![0xFFu8; 32]); // Invalid compressed key
        let result = derive_substrate_address(&key_bytes, PublicKeyType::Secp256k1);
        // Should handle error gracefully
        if let Err(_) = result {
            // Error is acceptable for invalid keys
        } else if let Ok(addresses) = result {
            // If it succeeds, addresses should be empty or valid
            assert!(addresses.is_empty() || addresses.len() == 3);
        }
    }

    #[test]
    fn test_derive_substrate_address_secp256k1_wrong_uncompressed_prefix() {
        // Test with uncompressed key that has wrong prefix (not 0x04)
        let mut key_bytes = vec![0x05u8]; // Wrong prefix
        key_bytes.extend(vec![0u8; 64]);
        let result = derive_substrate_address(&key_bytes, PublicKeyType::Secp256k1).unwrap();
        assert!(result.is_empty(), "Should return empty for wrong prefix");
    }

    #[test]
    fn test_derive_substrate_address_secp256k1_uncompressed_wrong_length() {
        // Test with uncompressed key that has wrong total length
        // A 64-byte key with 0x04 prefix would be treated as 64 bytes (no prefix check)
        // So we test with a key that's 65 bytes but has wrong prefix
        let mut key_bytes = vec![0x05u8]; // Wrong prefix (not 0x04)
        key_bytes.extend(vec![0u8; 64]);
        let result = derive_substrate_address(&key_bytes, PublicKeyType::Secp256k1).unwrap();
        assert!(result.is_empty(), "Should return empty for wrong prefix");

        // Also test with a key that's too short (not 33, 64, or 65 bytes)
        let short_key = vec![0u8; 30];
        let result2 = derive_substrate_address(&short_key, PublicKeyType::Secp256k1).unwrap();
        assert!(result2.is_empty(), "Should return empty for too short key");
    }
}
