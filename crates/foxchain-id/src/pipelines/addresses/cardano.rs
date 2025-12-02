use crate::Error;
use crate::shared::encoding::bech32 as bech32_encoding;
use bech32::{u5, Variant};
use serde_json::Value;
use sha3::{Digest, Sha3_256};

/// Execute Cardano address derivation pipeline
pub fn execute_cardano_pipeline(pk_bytes: &[u8], params: &Value) -> Result<String, Error> {
    if pk_bytes.len() != 32 {
        return Err(Error::InvalidInput(format!(
            "Invalid Ed25519 key length: {} bytes (expected 32)",
            pk_bytes.len()
        )));
    }
    
    // Hash with SHA3-256
    let hash = Sha3_256::digest(pk_bytes);
    
    // Slice first 28 bytes
    let payload = &hash[..28];
    
    // Get header and HRP from params
    let header: u8 = params
        .get("header")
        .and_then(|v| v.as_u64())
        .map(|v| v as u8)
        .unwrap_or(0x00);
    
    let hrp = params
        .get("hrp")
        .and_then(|v| v.as_str())
        .unwrap_or("addr");
    
    // Prefix with header
    let address_bytes = [&[header], payload].concat();
    
    // Convert to base32
    let data_u5 = bech32_encoding::convert_bits(&address_bytes, 8, 5, true)
        .map_err(|e| Error::InvalidInput(format!("Bech32 conversion error: {}", e)))?;
    let data_u5_vec: Vec<u5> = bech32_encoding::bytes_to_u5(&data_u5);
    
    // Encode as Bech32
    bech32_encoding::encode(hrp, &data_u5_vec, Variant::Bech32)
        .map_err(|e| Error::InvalidInput(format!("Bech32 encoding error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_cardano_pipeline_valid_key() {
        // Use a valid 32-byte Ed25519 key
        let key = vec![0u8; 32];
        let params = json!({"hrp": "addr", "header": 0x00});
        
        let result = execute_cardano_pipeline(&key, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("addr1"));
    }

    #[test]
    fn test_cardano_pipeline_invalid_length() {
        let invalid_key = vec![0u8; 33]; // Wrong length
        let params = json!({"hrp": "addr"});
        
        let result = execute_cardano_pipeline(&invalid_key, &params);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("32") || error_msg.contains("Invalid"));
    }

    #[test]
    fn test_cardano_pipeline_default_hrp() {
        let key = vec![0u8; 32];
        let params = json!({}); // No HRP specified, should default to "addr"
        
        let result = execute_cardano_pipeline(&key, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("addr1"));
    }

    #[test]
    fn test_cardano_pipeline_custom_hrp() {
        let key = vec![0u8; 32];
        let params = json!({"hrp": "stake"});
        
        let result = execute_cardano_pipeline(&key, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("stake1"));
    }

    #[test]
    fn test_cardano_pipeline_custom_header() {
        let key = vec![0u8; 32];
        let params = json!({"hrp": "addr", "header": 0x01});
        
        let result = execute_cardano_pipeline(&key, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("addr1"));
    }
}

