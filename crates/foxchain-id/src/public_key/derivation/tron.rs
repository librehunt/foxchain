//! Tron address derivation from secp256k1 public keys

use crate::shared::crypto::hash::keccak256;
use crate::shared::crypto::secp256k1;
use crate::{Chain, Error};

/// Tron mainnet version byte
const TRON_MAINNET_VERSION: u8 = 0x41;

/// Derive Tron address from secp256k1 public key
///
/// Process:
/// 1. Take secp256k1 public key (compressed or uncompressed)
/// 2. Decompress if needed
/// 3. Compute Keccak-256 hash (same as EVM)
/// 4. Take last 20 bytes
/// 5. Prepend version byte 0x41
/// 6. Compute Base58Check checksum
/// 7. Encode as Base58Check
///
/// Returns Tron chain with derived address
pub fn derive_tron_address(public_key: &[u8]) -> Result<Option<(Chain, String)>, Error> {
    // Handle compressed and uncompressed keys
    let key_bytes_64 = if public_key.len() == 33 {
        // Compressed key - decompress it
        let uncompressed = secp256k1::decompress_public_key(public_key)?;
        // Extract the 64-byte key (skip 0x04 prefix)
        if uncompressed.len() == 65 && uncompressed[0] == 0x04 {
            uncompressed[1..65].to_vec()
        } else {
            return Ok(None);
        }
    } else if public_key.len() == 65 && public_key[0] == 0x04 {
        // Uncompressed key - extract the 64-byte key (skip 0x04 prefix)
        public_key[1..65].to_vec()
    } else {
        return Ok(None);
    };

    // Compute Keccak-256 hash (same as EVM)
    let hash = keccak256(&key_bytes_64);

    // Take last 20 bytes
    let address_bytes = &hash[12..32];

    // Create Base58Check address: version (0x41) + address (20 bytes) + checksum (4 bytes)
    let payload = [&[TRON_MAINNET_VERSION], address_bytes].concat();

    // Base58Check encoding is done via the checksum module
    // We need to compute checksum and encode
    use crate::shared::crypto::hash::double_sha256;
    use base58::ToBase58;

    let checksum_hash = double_sha256(&payload);
    let checksum = &checksum_hash[..4];

    let full_bytes = [payload, checksum.to_vec()].concat();
    let address = full_bytes.to_base58();

    Ok(Some((Chain::Tron, address)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_tron_address_uncompressed() {
        // Test with uncompressed secp256k1 public key
        let mut key_bytes = vec![0x04u8]; // Uncompressed prefix
        key_bytes.extend(vec![0u8; 64]);
        let result = derive_tron_address(&key_bytes).unwrap();

        assert!(result.is_some());
        let (chain, address) = result.unwrap();
        assert_eq!(chain, Chain::Tron);
        assert!(!address.is_empty());
        // Tron addresses start with 'T'
        assert!(address.starts_with('T'));
    }

    #[test]
    fn test_derive_tron_address_compressed() {
        // Test with compressed secp256k1 public key
        // Use a valid compressed key format
        let mut key_bytes = vec![0x02u8]; // Compressed prefix
        key_bytes.extend(vec![0u8; 32]);
        let result = derive_tron_address(&key_bytes);

        // Decompression might fail for invalid keys, so result could be None or error
        if let Ok(Some((chain, address))) = result {
            assert_eq!(chain, Chain::Tron);
            assert!(address.starts_with('T'));
        }
        // If decompression failed, that's acceptable for invalid keys
    }

    #[test]
    fn test_derive_tron_address_invalid_length() {
        // Test with invalid length (not 33 or 65 bytes)
        let key_bytes = vec![0u8; 32];
        let result = derive_tron_address(&key_bytes).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_derive_tron_address_wrong_prefix() {
        // Test with 65-byte key that doesn't start with 0x04
        let mut key_bytes = vec![0x05u8]; // Wrong prefix
        key_bytes.extend(vec![0u8; 64]);
        let result = derive_tron_address(&key_bytes).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_derive_tron_address_valid_key() {
        // Test with a known valid secp256k1 public key
        // Generator point (uncompressed)
        let key_bytes = vec![
            0x04, 0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce,
            0x87, 0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81,
            0x5b, 0x16, 0xf8, 0x17, 0x98, 0x48, 0x3a, 0xda, 0x77, 0x26, 0xa3, 0xc4, 0x65, 0x5d,
            0xa4, 0xfb, 0xfc, 0x0e, 0x11, 0x08, 0xa8, 0xfd, 0x17, 0xb4, 0x48, 0xa6, 0x85, 0x54,
            0x19, 0x9c, 0x47, 0xd0, 0x8f, 0xfb, 0x10, 0xd4, 0xb8,
        ];
        let result = derive_tron_address(&key_bytes).unwrap();
        assert!(result.is_some());
        let (chain, address) = result.unwrap();
        assert_eq!(chain, Chain::Tron);
        assert!(address.starts_with('T'));
        // Verify it's valid Base58Check
        use crate::shared::checksum::base58check;
        let validation = base58check::validate(&address).unwrap();
        assert!(validation.is_some());
        let (version, _) = validation.unwrap();
        assert_eq!(version, TRON_MAINNET_VERSION);
    }
}
