use crate::Error;
use base58::ToBase58;
use serde_json::Value;

/// Execute Solana address derivation pipeline
pub fn execute_solana_pipeline(pk_bytes: &[u8], _params: &Value) -> Result<String, Error> {
    if pk_bytes.len() != 32 {
        return Err(Error::InvalidInput(format!(
            "Invalid Ed25519 key length: {} bytes (expected 32)",
            pk_bytes.len()
        )));
    }

    // Direct Base58 encoding
    Ok(pk_bytes.to_base58())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_solana_pipeline_valid_key() {
        let key = vec![0u8; 32];
        let params = json!({});

        let result = execute_solana_pipeline(&key, &params);
        assert!(result.is_ok());
        let address = result.unwrap();
        assert!(!address.is_empty());
    }

    #[test]
    fn test_solana_pipeline_invalid_length() {
        let invalid_key = vec![0u8; 33];
        let params = json!({});

        let result = execute_solana_pipeline(&invalid_key, &params);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("32") || error_msg.contains("Invalid"));
    }
}
