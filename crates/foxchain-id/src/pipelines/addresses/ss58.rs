use crate::Error;
use crate::shared::crypto::hash::blake2b_256;
use crate::shared::encoding::ss58;
use serde_json::Value;

/// Execute SS58 address derivation pipeline
pub fn execute_ss58_pipeline(pk_bytes: &[u8], params: &Value) -> Result<String, Error> {
    // Get prefix from params (default to 0 for Polkadot)
    let prefix: u16 = params
        .get("prefix")
        .and_then(|v| v.as_u64())
        .map(|v| v as u16)
        .unwrap_or(0);
    
    // For secp256k1, hash first; for Ed25519/sr25519, use directly
    let account_id = if pk_bytes.len() == 64 || pk_bytes.len() == 33 || pk_bytes.len() == 65 {
        // secp256k1: hash the key
        blake2b_256(pk_bytes).to_vec()
    } else if pk_bytes.len() == 32 {
        // Ed25519/sr25519: use directly
        pk_bytes.to_vec()
    } else {
        return Err(Error::InvalidInput(format!(
            "Invalid key length: {} bytes",
            pk_bytes.len()
        )));
    };
    
    ss58::encode(prefix, &account_id)
        .map_err(|e| Error::InvalidInput(format!("SS58 encoding error: {}", e)))
}

