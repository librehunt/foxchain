//! EVM address derivation from secp256k1 public keys

use crate::shared::crypto::hash::keccak256;
use crate::shared::encoding::hex;
use crate::Error;

/// Derive EVM address from secp256k1 public key
///
/// Process:
/// 1. Take public key (compressed or uncompressed)
/// 2. If compressed, decompress (we'll skip this for now and use uncompressed)
/// 3. Compute Keccak-256 hash
/// 4. Take last 20 bytes
/// 5. Format as 0x-prefixed hex
pub fn derive_evm_address(public_key: &[u8]) -> Result<Option<String>, Error> {
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
    Ok(Some(hex::encode(address_bytes)))
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
        assert!(result.is_some());
        let address = result.unwrap();
        assert!(address.starts_with("0x"));
        assert_eq!(address.len(), 42);
    }

    #[test]
    fn test_derive_evm_address_compressed() {
        // Test with compressed public key (should return None)
        let key_bytes = vec![0x02; 33];
        let result = derive_evm_address(&key_bytes).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_derive_evm_address_invalid_length() {
        // Test with invalid length (not 33 or 65 bytes)
        let key_bytes = vec![0u8; 32];
        let result = derive_evm_address(&key_bytes).unwrap();
        assert!(result.is_none());
    }
}
