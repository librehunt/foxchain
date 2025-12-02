use crate::Error;
use crate::shared::crypto::hash::{double_sha256, keccak256};
use crate::shared::crypto::secp256k1;
use base58::ToBase58;
use serde_json::Value;

/// Execute Tron address derivation pipeline
pub fn execute_tron_pipeline(pk_bytes: &[u8], _params: &Value) -> Result<String, Error> {
    // Extract 64-byte key
    let key_64 = extract_64_bytes(pk_bytes)?;
    
    // Hash with Keccak256
    let hash = keccak256(&key_64);
    
    // Slice last 20 bytes
    let address_bytes = &hash[12..32];
    
    // Prefix with Tron version byte (0x41)
    let payload = [&[0x41u8], address_bytes].concat();
    
    // Double SHA256 for checksum
    let checksum_hash = double_sha256(&payload);
    let checksum = &checksum_hash[..4];
    
    // Append checksum and encode as Base58
    let full_bytes = [payload, checksum.to_vec()].concat();
    Ok(full_bytes.to_base58())
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

