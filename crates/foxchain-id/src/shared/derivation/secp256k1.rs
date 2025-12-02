//! secp256k1 public key derivation functions

use crate::shared::crypto::hash::{double_sha256, hash160, keccak256};
use crate::shared::crypto::secp256k1;
use crate::shared::encoding::hex;
use crate::Error;
use base58::ToBase58;

/// Derive EVM address from secp256k1 public key
/// Returns just the address string (chain-agnostic)
pub fn derive_evm_address(public_key: &[u8]) -> Result<String, Error> {
    let key_bytes_64 = extract_64_bytes(public_key)?;
    let hash = keccak256(&key_bytes_64);
    let address_bytes = &hash[12..32];
    Ok(hex::encode(address_bytes))
}

/// Derive Base58Check P2PKH address from secp256k1 public key
/// Returns address with specified version byte (chain-agnostic)
pub fn derive_base58check_p2pkh(public_key: &[u8], version: u8) -> Result<String, Error> {
    let key_bytes_64 = extract_64_bytes(public_key)?;
    let hash160_bytes = hash160(&key_bytes_64);
    derive_p2pkh_address(&hash160_bytes, version)
        .and_then(|opt| opt.ok_or_else(|| Error::InvalidInput("Failed to derive P2PKH address".to_string())))
}

/// Derive Tron address from secp256k1 public key
/// Returns just the address string (chain-agnostic)
pub fn derive_tron_address(public_key: &[u8]) -> Result<String, Error> {
    const TRON_MAINNET_VERSION: u8 = 0x41;
    
    let key_bytes_64 = extract_64_bytes(public_key)?;
    let hash = keccak256(&key_bytes_64);
    let address_bytes = &hash[12..32];
    let payload = [&[TRON_MAINNET_VERSION], address_bytes].concat();
    let checksum_hash = double_sha256(&payload);
    let checksum = &checksum_hash[..4];
    let full_bytes = [payload, checksum.to_vec()].concat();
    Ok(full_bytes.to_base58())
}

/// Derive SS58 address from secp256k1 public key
/// Returns address with specified SS58 prefix (chain-agnostic)
pub fn derive_ss58_address(public_key: &[u8], prefix: u16) -> Result<String, Error> {
    use crate::shared::crypto::hash::blake2b_256;
    use crate::shared::encoding::ss58;
    
    let key_bytes_64 = extract_64_bytes(public_key)?;
    let account_id = blake2b_256(&key_bytes_64).to_vec();
    ss58::encode(prefix, &account_id)
        .map_err(|e| Error::InvalidInput(format!("SS58 encoding error: {}", e)))
}

/// Extract 64-byte key from compressed/uncompressed secp256k1 key
fn extract_64_bytes(public_key: &[u8]) -> Result<Vec<u8>, Error> {
    if public_key.len() == 33 {
        let uncompressed = secp256k1::decompress_public_key(public_key)?;
        if uncompressed.len() == 65 && uncompressed[0] == 0x04 {
            Ok(uncompressed[1..65].to_vec())
        } else {
            Err(Error::InvalidInput("Invalid decompressed key format".to_string()))
        }
    } else if public_key.len() == 65 && public_key[0] == 0x04 {
        Ok(public_key[1..65].to_vec())
    } else if public_key.len() == 64 {
        Ok(public_key.to_vec())
    } else {
        Err(Error::InvalidInput(format!(
            "Invalid secp256k1 key length: {} bytes",
            public_key.len()
        )))
    }
}

/// Derive P2PKH address from hash160
fn derive_p2pkh_address(hash160: &[u8], version: u8) -> Result<Option<String>, Error> {
    if hash160.len() != 20 {
        return Ok(None);
    }

    let mut payload = vec![version];
    payload.extend_from_slice(hash160);
    let checksum_hash = double_sha256(&payload);
    let checksum = &checksum_hash[..4];
    let mut full = payload;
    full.extend_from_slice(checksum);
    Ok(Some(full.as_slice().to_base58()))
}

