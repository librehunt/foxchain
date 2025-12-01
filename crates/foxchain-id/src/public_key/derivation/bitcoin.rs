//! Bitcoin address derivation from secp256k1 public keys

use crate::shared::crypto::hash::{double_sha256, hash160};
use crate::{Chain, Error};
use base58::ToBase58;

/// Derive Bitcoin addresses from secp256k1 public key
///
/// Returns list of (chain, address) pairs for different Bitcoin address formats
pub fn derive_bitcoin_addresses(public_key: &[u8]) -> Result<Vec<(Chain, String)>, Error> {
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
}
