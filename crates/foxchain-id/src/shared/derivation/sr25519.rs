//! sr25519 public key derivation functions

use crate::shared::encoding::ss58;
use crate::Error;

/// Derive SS58 address from sr25519 public key
/// Returns address with specified SS58 prefix (chain-agnostic)
pub fn derive_ss58_address(key_bytes: &[u8], prefix: u16) -> Result<String, Error> {
    if key_bytes.len() != 32 {
        return Err(Error::InvalidInput(format!(
            "Invalid sr25519 key length: {} bytes (expected 32)",
            key_bytes.len()
        )));
    }

    ss58::encode(prefix, key_bytes)
        .map_err(|e| Error::InvalidInput(format!("SS58 encoding error: {}", e)))
}

