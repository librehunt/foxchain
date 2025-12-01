//! Bitcoin address derivation from secp256k1 public keys

use crate::shared::crypto::hash::{double_sha256, hash160};
use crate::shared::crypto::secp256k1;
use crate::{Chain, Error};
use base58::ToBase58;

/// Derive Bitcoin addresses from secp256k1 public key
///
/// Returns list of (chain, address) pairs for different Bitcoin address formats
pub fn derive_bitcoin_addresses(public_key: &[u8]) -> Result<Vec<(Chain, String)>, Error> {
    let mut addresses = Vec::new();

    // Get uncompressed public key bytes (skip 0x04 prefix if present)
    let key_bytes_64 = if public_key.len() == 33 {
        // Compressed key - decompress it
        let uncompressed = secp256k1::decompress_public_key(public_key)?;
        // Extract the 64-byte key (skip 0x04 prefix)
        if uncompressed.len() == 65 && uncompressed[0] == 0x04 {
            uncompressed[1..65].to_vec()
        } else {
            return Ok(addresses);
        }
    } else if public_key.len() == 65 && public_key[0] == 0x04 {
        // Uncompressed key - extract the 64-byte key (skip 0x04 prefix)
        public_key[1..65].to_vec()
    } else if public_key.len() == 64 {
        public_key.to_vec()
    } else {
        return Ok(addresses);
    };

    let key_bytes = &key_bytes_64;

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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_derive_bitcoin_addresses_64_bytes() {
        // Test with 64-byte public key (without 0x04 prefix)
        let key_bytes = vec![0u8; 64];
        let result = derive_bitcoin_addresses(&key_bytes).unwrap();
        assert!(!result.is_empty());
        assert_eq!(result[0].0, Chain::Bitcoin);
    }

    #[test]
    fn test_derive_bitcoin_addresses_compressed() {
        // Test with compressed public key (should now work after decompression)
        // Use a valid compressed key (generator point)
        use crate::shared::encoding::hex;
        let compressed =
            hex::decode("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
                .unwrap();
        let result = derive_bitcoin_addresses(&compressed).unwrap();
        assert!(!result.is_empty());
        assert_eq!(result[0].0, Chain::Bitcoin);
    }

    #[test]
    fn test_derive_p2pkh_address_invalid_length() {
        // Test with invalid hash160 length
        let hash160 = vec![0u8; 19]; // Wrong length
        let result = derive_p2pkh_address(&hash160, 0x00).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_derive_bitcoin_addresses_invalid_length() {
        // Test with invalid length (not 33, 64, or 65 bytes)
        let key_bytes = vec![0u8; 63];
        let result = derive_bitcoin_addresses(&key_bytes).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_derive_bitcoin_addresses_compressed_invalid_decompression() {
        // Test with compressed key that fails decompression validation
        // This tests the case where decompression succeeds but result doesn't have expected format
        // We'll use a key that decompresses but doesn't match expected format
        // Actually, if decompression succeeds, it should always produce 65 bytes with 0x04 prefix
        // So we test with an invalid compressed key that fails decompression
        let mut invalid_compressed = vec![0x02];
        invalid_compressed.extend(vec![0xFFu8; 32]); // Invalid curve point

        let result = derive_bitcoin_addresses(&invalid_compressed);
        // Should return error from decompression
        assert!(result.is_err());
    }
}
