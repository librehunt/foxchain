//! Solana address derivation from Ed25519 public keys

use base58::ToBase58;

/// Derive Solana address from Ed25519 public key
///
/// Solana addresses are simply the public key encoded in Base58.
pub fn derive_solana_address(public_key: &[u8]) -> Option<String> {
    if public_key.len() == 32 {
        // Solana address is the public key itself (base58 encoded)
        Some(public_key.to_base58())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_solana_address() {
        // Test with Ed25519 public key (32 bytes)
        let key_bytes = vec![0u8; 32];
        let result = derive_solana_address(&key_bytes);
        assert!(result.is_some());
        let address = result.unwrap();
        // Should be base58 encoded
        assert!(!address.is_empty());
    }

    #[test]
    fn test_derive_solana_address_invalid_length() {
        // Test with invalid length
        let key_bytes = vec![0u8; 33];
        let result = derive_solana_address(&key_bytes);
        assert!(result.is_none());
    }
}
