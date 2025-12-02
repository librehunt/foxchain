//! Ed25519 public key derivation functions

use crate::shared::crypto::hash::sha256;
use crate::shared::encoding::bech32 as bech32_encoding;
use crate::Error;
use base58::ToBase58;
use bech32::{u5, Variant};

/// Derive Base58 address from Ed25519 public key
/// Returns just the address string (chain-agnostic)
pub fn derive_base58_address(key_bytes: &[u8]) -> Result<String, Error> {
    if key_bytes.len() == 32 {
        Ok(key_bytes.to_base58())
    } else {
        Err(Error::InvalidInput(format!(
            "Invalid Ed25519 key length: {} bytes (expected 32)",
            key_bytes.len()
        )))
    }
}

/// Derive Bech32 address from Ed25519 public key
/// Returns address with specified HRP (chain-agnostic)
pub fn derive_bech32_address(key_bytes: &[u8], hrp: &str) -> Result<String, Error> {
    if key_bytes.len() != 32 {
        return Err(Error::InvalidInput(format!(
            "Invalid Ed25519 key length: {} bytes (expected 32)",
            key_bytes.len()
        )));
    }

    let hash = sha256(key_bytes);
    let address_bytes = &hash[..20];
    let data = bech32_encoding::convert_bits(address_bytes, 8, 5, true)
        .map_err(|e| Error::InvalidInput(format!("Bech32 conversion error: {}", e)))?;
    let data_u5: Vec<u5> = bech32_encoding::bytes_to_u5(&data);

    bech32_encoding::encode(hrp, &data_u5, Variant::Bech32)
        .map_err(|e| Error::InvalidInput(format!("Bech32 encoding error: {}", e)))
}

/// Derive Cardano address from Ed25519 public key
/// Returns address with specified header and HRP (chain-agnostic)
pub fn derive_cardano_address(key_bytes: &[u8], header: u8, hrp: &str) -> Result<String, Error> {
    if key_bytes.len() != 32 {
        return Err(Error::InvalidInput(format!(
            "Invalid Ed25519 key length: {} bytes (expected 32)",
            key_bytes.len()
        )));
    }

    let hash = sha3_256(key_bytes);
    let payload = &hash[..28];
    create_cardano_address(payload, header, hrp)
}

/// Derive SS58 address from Ed25519 public key
/// Returns address with specified SS58 prefix (chain-agnostic)
pub fn derive_ss58_address(key_bytes: &[u8], prefix: u16) -> Result<String, Error> {
    use crate::shared::encoding::ss58;
    
    if key_bytes.len() != 32 {
        return Err(Error::InvalidInput(format!(
            "Invalid Ed25519 key length: {} bytes (expected 32)",
            key_bytes.len()
        )));
    }

    ss58::encode(prefix, key_bytes)
        .map_err(|e| Error::InvalidInput(format!("SS58 encoding error: {}", e)))
}

/// Compute SHA3-256 hash
fn sha3_256(data: &[u8]) -> [u8; 32] {
    use sha3::{Digest, Sha3_256};
    Sha3_256::digest(data).into()
}

/// Create a Cardano address
fn create_cardano_address(
    payload: &[u8],
    header: u8,
    hrp: &str,
) -> Result<String, Error> {
    let address_bytes = [&[header], payload].concat();
    let data_u5 = bech32_encoding::convert_bits(&address_bytes, 8, 5, true)
        .map_err(|e| Error::InvalidInput(format!("Bech32 conversion error: {}", e)))?;
    let data_u5_vec: Vec<u5> = bech32_encoding::bytes_to_u5(&data_u5);
    bech32_encoding::encode(hrp, &data_u5_vec, Variant::Bech32)
        .map_err(|e| Error::InvalidInput(format!("Bech32 encoding error: {}", e)))
}

