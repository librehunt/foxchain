use crate::Error;
use crate::shared::crypto::hash::{double_sha256, hash160};
use crate::shared::crypto::secp256k1;
use base58::ToBase58;
use serde_json::Value;

/// Execute Bitcoin P2PKH address derivation pipeline
pub fn execute_bitcoin_p2pkh_pipeline(pk_bytes: &[u8], params: &Value) -> Result<String, Error> {
    // Extract 64-byte key
    let key_64 = extract_64_bytes(pk_bytes)?;
    
    // Hash with RIPEMD160 (which internally does SHA256 then RIPEMD160)
    let payload = hash160(&key_64);
    
    // Get version byte from params (default to 0x00 for Bitcoin mainnet)
    let version: u8 = params
        .get("version_byte")
        .and_then(|v| v.as_u64())
        .map(|v| v as u8)
        .unwrap_or(0x00);
    
    // Prefix with version byte
    let mut versioned = vec![version];
    versioned.extend_from_slice(&payload);
    
    // Double SHA256 for checksum
    let checksum_hash = double_sha256(&versioned);
    let checksum = &checksum_hash[..4];
    
    // Append checksum and encode as Base58
    let mut full = versioned;
    full.extend_from_slice(checksum);
    Ok(full.as_slice().to_base58())
}

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

