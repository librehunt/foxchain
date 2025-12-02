use crate::Error;
use crate::shared::crypto::hash::sha256;
use crate::shared::encoding::bech32 as bech32_encoding;
use bech32::{u5, Variant};
use serde_json::Value;

/// Execute Cosmos address derivation pipeline
pub fn execute_cosmos_pipeline(pk_bytes: &[u8], params: &Value) -> Result<String, Error> {
    if pk_bytes.len() != 32 {
        return Err(Error::InvalidInput(format!(
            "Invalid Ed25519 key length: {} bytes (expected 32)",
            pk_bytes.len()
        )));
    }
    
    // Hash with SHA256
    let hash = sha256(pk_bytes);
    
    // Slice first 20 bytes
    let address_bytes = &hash[..20];
    
    // Get HRP from params (default to "cosmos")
    let hrp = params
        .get("hrp")
        .and_then(|v| v.as_str())
        .unwrap_or("cosmos");
    
    // Convert to base32
    let data = bech32_encoding::convert_bits(address_bytes, 8, 5, true)
        .map_err(|e| Error::InvalidInput(format!("Bech32 conversion error: {}", e)))?;
    let data_u5: Vec<u5> = bech32_encoding::bytes_to_u5(&data);
    
    // Encode as Bech32
    bech32_encoding::encode(hrp, &data_u5, Variant::Bech32)
        .map_err(|e| Error::InvalidInput(format!("Bech32 encoding error: {}", e)))
}

