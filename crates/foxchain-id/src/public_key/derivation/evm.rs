//! EVM address derivation from secp256k1 public keys

use crate::shared::crypto::hash::keccak256;
use crate::shared::crypto::secp256k1;
use crate::shared::encoding::hex;
use crate::{Chain, Error};

/// Derive EVM addresses from secp256k1 public key
///
/// Returns all 10 EVM-compatible chains with the same derived address.
/// EVM addresses are identical across all EVM chains.
///
/// Process:
/// 1. Take public key (compressed or uncompressed)
/// 2. If compressed, decompress to uncompressed format
/// 3. Compute Keccak-256 hash
/// 4. Take last 20 bytes
/// 5. Format as 0x-prefixed hex
/// 6. Return all 10 EVM chains with the same address
pub fn derive_evm_address(public_key: &[u8]) -> Result<Vec<(Chain, String)>, Error> {
    // Handle compressed and uncompressed keys
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
    } else {
        return Ok(Vec::new());
    };

    let key_bytes = &key_bytes_64;

    // Compute Keccak-256 hash
    let hash = keccak256(key_bytes);

    // Take last 20 bytes
    let address_bytes = &hash[12..32];

    // Format as hex with 0x prefix
    let address = hex::encode(address_bytes);

    // Return all 10 EVM chains with the same address
    Ok(vec![
        (Chain::Ethereum, address.clone()),
        (Chain::Polygon, address.clone()),
        (Chain::BSC, address.clone()),
        (Chain::Avalanche, address.clone()),
        (Chain::Arbitrum, address.clone()),
        (Chain::Optimism, address.clone()),
        (Chain::Base, address.clone()),
        (Chain::Fantom, address.clone()),
        (Chain::Celo, address.clone()),
        (Chain::Gnosis, address),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(result.len(), 10);
        // All chains should have the same address
        let address = &result[0].1;
        assert!(address.starts_with("0x"));
        assert_eq!(address.len(), 42);
        // Verify all chains are present
        assert!(result.iter().any(|(c, _)| matches!(c, Chain::Ethereum)));
        assert!(result.iter().any(|(c, _)| matches!(c, Chain::Polygon)));
        assert!(result.iter().any(|(c, _)| matches!(c, Chain::BSC)));
        assert!(result.iter().any(|(c, _)| matches!(c, Chain::Avalanche)));
        assert!(result.iter().any(|(c, _)| matches!(c, Chain::Arbitrum)));
        assert!(result.iter().any(|(c, _)| matches!(c, Chain::Optimism)));
        assert!(result.iter().any(|(c, _)| matches!(c, Chain::Base)));
        assert!(result.iter().any(|(c, _)| matches!(c, Chain::Fantom)));
        assert!(result.iter().any(|(c, _)| matches!(c, Chain::Celo)));
        assert!(result.iter().any(|(c, _)| matches!(c, Chain::Gnosis)));
        // Verify all addresses are the same
        for (_, addr) in &result {
            assert_eq!(addr, address);
        }
    }

    #[test]
    fn test_derive_evm_address_compressed() {
        // Test with compressed public key (should now work after decompression)
        // Use a valid compressed key (generator point)
        use crate::shared::encoding::hex;
        let compressed =
            hex::decode("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
                .unwrap();
        let result = derive_evm_address(&compressed).unwrap();
        assert_eq!(result.len(), 10);
        let address = &result[0].1;
        assert!(address.starts_with("0x"));
        assert_eq!(address.len(), 42);
        // Verify all addresses are the same
        for (_, addr) in &result {
            assert_eq!(addr, address);
        }
    }

    #[test]
    fn test_derive_evm_address_invalid_length() {
        // Test with invalid length (not 33 or 65 bytes)
        let key_bytes = vec![0u8; 32];
        let result = derive_evm_address(&key_bytes).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_derive_evm_address_compressed_invalid_decompression() {
        // Test with compressed key that fails decompression
        // This tests the error path when decompression fails
        let mut invalid_compressed = vec![0x02];
        invalid_compressed.extend(vec![0xFFu8; 32]); // Invalid curve point

        let result = derive_evm_address(&invalid_compressed);
        // Should return error from decompression
        assert!(result.is_err());
    }

    #[test]
    fn test_derive_evm_address_uncompressed_wrong_prefix() {
        // Test with 65-byte key that doesn't start with 0x04
        let mut key_bytes = vec![0x05]; // Wrong prefix
        key_bytes.extend(vec![0u8; 64]);
        let result = derive_evm_address(&key_bytes).unwrap();
        assert!(result.is_empty());
    }
}
