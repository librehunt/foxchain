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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_cosmos_pipeline_valid_key() {
        let key = vec![0u8; 32];
        let params = json!({"hrp": "cosmos"});
        
        let result = execute_cosmos_pipeline(&key, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("cosmos1"));
    }

    #[test]
    fn test_cosmos_pipeline_invalid_length() {
        let invalid_key = vec![0u8; 33];
        let params = json!({"hrp": "cosmos"});
        
        let result = execute_cosmos_pipeline(&invalid_key, &params);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("32") || error_msg.contains("Invalid"));
    }

    #[test]
    fn test_cosmos_pipeline_default_hrp() {
        let key = vec![0u8; 32];
        let params = json!({}); // No HRP, should default to "cosmos"
        
        let result = execute_cosmos_pipeline(&key, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("cosmos1"));
    }

    #[test]
    fn test_cosmos_pipeline_custom_hrp() {
        let key = vec![0u8; 32];
        let params = json!({"hrp": "osmo"});
        
        let result = execute_cosmos_pipeline(&key, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("osmo1"));
    }
}

