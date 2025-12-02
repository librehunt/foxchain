use crate::Error;
use crate::shared::crypto::hash::hash160;
use crate::shared::crypto::secp256k1;
use crate::shared::encoding::bech32 as bech32_encoding;
use bech32::{u5, Variant};
use serde_json::Value;

/// Execute Bitcoin Bech32 address derivation pipeline
pub fn execute_bitcoin_bech32_pipeline(pk_bytes: &[u8], params: &Value) -> Result<String, Error> {
    // Extract 64-byte key
    let key_64 = extract_64_bytes(pk_bytes)?;
    
    // Hash with RIPEMD160 (which internally does SHA256 then RIPEMD160)
    let payload = hash160(&key_64);
    
    // Get HRP from params (default to "bc" for Bitcoin mainnet)
    let hrp = params
        .get("hrp")
        .and_then(|v| v.as_str())
        .unwrap_or("bc");
    
    // Convert to base32
    let data = bech32_encoding::convert_bits(&payload, 8, 5, true)
        .map_err(|e| Error::InvalidInput(format!("Bech32 conversion error: {}", e)))?;
    let data_u5: Vec<u5> = bech32_encoding::bytes_to_u5(&data);
    
    // Encode as Bech32
    bech32_encoding::encode(hrp, &data_u5, Variant::Bech32)
        .map_err(|e| Error::InvalidInput(format!("Bech32 encoding error: {}", e)))
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

