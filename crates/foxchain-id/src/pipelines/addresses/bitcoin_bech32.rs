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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_bitcoin_bech32_pipeline_compressed_key() {
        // Use a valid compressed secp256k1 key (33 bytes)
        // This is the compressed form of the generator point
        let compressed_key = hex::decode("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798").unwrap();
        let params = json!({"hrp": "bc"});
        
        let result = execute_bitcoin_bech32_pipeline(&compressed_key, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("bc1"));
    }

    #[test]
    fn test_bitcoin_bech32_pipeline_uncompressed_key() {
        // Use a valid uncompressed secp256k1 key (65 bytes)
        let uncompressed_key = hex::decode("0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8").unwrap();
        let params = json!({"hrp": "bc"});
        
        let result = execute_bitcoin_bech32_pipeline(&uncompressed_key, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("bc1"));
    }

    #[test]
    fn test_bitcoin_bech32_pipeline_64_byte_key() {
        // Use a 64-byte key (without 0x04 prefix)
        let key_64 = hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8").unwrap();
        let params = json!({"hrp": "bc"});
        
        let result = execute_bitcoin_bech32_pipeline(&key_64, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("bc1"));
    }

    #[test]
    fn test_bitcoin_bech32_pipeline_invalid_length() {
        let invalid_key = vec![0u8; 32];
        let params = json!({"hrp": "bc"});
        
        let result = execute_bitcoin_bech32_pipeline(&invalid_key, &params);
        assert!(result.is_err());
    }

    #[test]
    fn test_bitcoin_bech32_pipeline_default_hrp() {
        let key_64 = hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8").unwrap();
        let params = json!({}); // No HRP specified, should default to "bc"
        
        let result = execute_bitcoin_bech32_pipeline(&key_64, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("bc1"));
    }

    #[test]
    fn test_bitcoin_bech32_pipeline_custom_hrp() {
        let key_64 = hex::decode("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8").unwrap();
        let params = json!({"hrp": "tb"}); // Testnet HRP
        
        let result = execute_bitcoin_bech32_pipeline(&key_64, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(address.starts_with("tb1"));
    }
}

