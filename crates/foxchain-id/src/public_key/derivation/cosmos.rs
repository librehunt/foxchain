//! Cosmos ecosystem address derivation from Ed25519 public keys

use crate::shared::crypto::hash::sha256;
use crate::shared::encoding::bech32 as bech32_encoding;
use crate::{Chain, Error};
use bech32::{u5, Variant};

/// Derive Cosmos ecosystem addresses from Ed25519 public key
///
/// Returns all 10 Cosmos ecosystem chains with their respective addresses.
/// All Cosmos chains use the same derivation algorithm (SHA256 hash + Bech32 encoding) but with different HRPs.
///
/// Process:
/// 1. Compute SHA256 hash of public key
/// 2. Take first 20 bytes
/// 3. Encode as Bech32 with chain-specific HRPs
pub fn derive_cosmos_address(public_key: &[u8]) -> Result<Vec<(Chain, String)>, Error> {
    if public_key.len() != 32 {
        return Ok(Vec::new());
    }

    // Compute SHA256 hash (same for all Cosmos chains)
    let hash = sha256(public_key);

    // Take first 20 bytes
    let address_bytes = &hash[..20];

    // Convert to 5-bit groups
    let data = bech32_encoding::convert_bits(address_bytes, 8, 5, true)
        .map_err(|e| Error::InvalidInput(format!("Bech32 conversion error: {}", e)))?;

    // Convert Vec<u8> to Vec<u5> for bech32 encoding
    let data_u5: Vec<u5> = bech32_encoding::bytes_to_u5(&data);

    // Derive addresses for all Cosmos chains
    let mut addresses = Vec::new();
    let hrps = [
        ("cosmos", Chain::CosmosHub),
        ("osmo", Chain::Osmosis),
        ("juno", Chain::Juno),
        ("akash", Chain::Akash),
        ("stars", Chain::Stargaze),
        ("secret", Chain::SecretNetwork),
        ("terra", Chain::Terra),
        ("kava", Chain::Kava),
        ("regen", Chain::Regen),
        ("sent", Chain::Sentinel),
    ];

    for (hrp, chain) in hrps {
        let address = bech32_encoding::encode(hrp, &data_u5, Variant::Bech32)
            .map_err(|e| Error::InvalidInput(format!("Bech32 encoding error: {}", e)))?;
        addresses.push((chain, address));
    }

    Ok(addresses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_cosmos_address_all_chains() {
        // Test with Ed25519 public key (32 bytes)
        let key_bytes = vec![0u8; 32];
        let result = derive_cosmos_address(&key_bytes).unwrap();

        // Should return all 10 Cosmos chains
        assert_eq!(result.len(), 10, "Should return all 10 Cosmos chains");

        // Verify all chains are present
        let chains: Vec<Chain> = result.iter().map(|(chain, _)| chain.clone()).collect();
        assert!(chains.contains(&Chain::CosmosHub));
        assert!(chains.contains(&Chain::Osmosis));
        assert!(chains.contains(&Chain::Juno));
        assert!(chains.contains(&Chain::Akash));
        assert!(chains.contains(&Chain::Stargaze));
        assert!(chains.contains(&Chain::SecretNetwork));
        assert!(chains.contains(&Chain::Terra));
        assert!(chains.contains(&Chain::Kava));
        assert!(chains.contains(&Chain::Regen));
        assert!(chains.contains(&Chain::Sentinel));

        // Verify addresses match expected formats
        for (chain, address) in &result {
            match chain {
                Chain::CosmosHub => assert!(address.starts_with("cosmos1")),
                Chain::Osmosis => assert!(address.starts_with("osmo1")),
                Chain::Juno => assert!(address.starts_with("juno1")),
                Chain::Akash => assert!(address.starts_with("akash1")),
                Chain::Stargaze => assert!(address.starts_with("stars1")),
                Chain::SecretNetwork => assert!(address.starts_with("secret1")),
                Chain::Terra => assert!(address.starts_with("terra1")),
                Chain::Kava => assert!(address.starts_with("kava1")),
                Chain::Regen => assert!(address.starts_with("regen1")),
                Chain::Sentinel => assert!(address.starts_with("sent1")),
                _ => panic!("Unexpected chain: {:?}", chain),
            }
        }
    }

    #[test]
    fn test_derive_cosmos_address_invalid_length() {
        // Test with invalid length (not 32 bytes)
        let key_bytes = vec![0u8; 33];
        let result = derive_cosmos_address(&key_bytes).unwrap();
        assert!(
            result.is_empty(),
            "Should return empty vector for invalid length"
        );
    }

    #[test]
    fn test_derive_cosmos_address_empty_key() {
        // Test with empty key (0 bytes)
        let key_bytes = vec![];
        let result = derive_cosmos_address(&key_bytes).unwrap();
        assert!(
            result.is_empty(),
            "Should return empty vector for empty key"
        );
    }

    #[test]
    fn test_derive_cosmos_address_short_key() {
        // Test with key that's too short (31 bytes)
        let key_bytes = vec![0u8; 31];
        let result = derive_cosmos_address(&key_bytes).unwrap();
        assert!(
            result.is_empty(),
            "Should return empty vector for key that's too short"
        );
    }
}
