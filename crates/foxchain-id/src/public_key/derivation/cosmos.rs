//! Cosmos address derivation from Ed25519 public keys

use crate::shared::crypto::hash::sha256;
use crate::shared::encoding::bech32 as bech32_encoding;
use crate::Error;
use bech32::{u5, Variant};

/// Derive Cosmos address from Ed25519 public key
///
/// Process:
/// 1. Compute SHA256 hash of public key
/// 2. Take first 20 bytes
/// 3. Encode as Bech32 with "cosmos" HRP
pub fn derive_cosmos_address(public_key: &[u8]) -> Result<Option<String>, Error> {
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
    fn test_derive_cosmos_address() {
        // Test with Ed25519 public key (32 bytes)
        let key_bytes = vec![0u8; 32];
        let result = derive_cosmos_address(&key_bytes).unwrap();
        assert!(result.is_some());
        let address = result.unwrap();
        assert!(address.starts_with("cosmos1"));
    }
}
