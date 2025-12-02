use crate::Error;
use crate::loaders::load_pipeline;
use serde_json::Value;

/// Execute a pipeline by ID
pub fn execute_pipeline(
    pipeline_id: &str,
    pk_bytes: &[u8],
    params: &Value,
) -> Result<String, Error> {
    let pipeline = load_pipeline(pipeline_id)
        .map_err(|e| Error::InvalidInput(format!("Failed to load pipeline {}: {}", pipeline_id, e)))?;
    
    match pipeline.id.as_str() {
        "evm" => evm::execute_evm_pipeline(pk_bytes, params),
        "bitcoin_p2pkh" => bitcoin_p2pkh::execute_bitcoin_p2pkh_pipeline(pk_bytes, params),
        "bitcoin_bech32" => bitcoin_bech32::execute_bitcoin_bech32_pipeline(pk_bytes, params),
        "cosmos" => cosmos::execute_cosmos_pipeline(pk_bytes, params),
        "solana" => solana::execute_solana_pipeline(pk_bytes, params),
        "ss58" => ss58::execute_ss58_pipeline(pk_bytes, params),
        "cardano" => cardano::execute_cardano_pipeline(pk_bytes, params),
        "tron" => tron::execute_tron_pipeline(pk_bytes, params),
        _ => Err(Error::InvalidInput(format!("Unknown pipeline: {}", pipeline_id))),
    }
}

// Import pipeline executors
use super::{
    evm, bitcoin_p2pkh, bitcoin_bech32, cosmos, solana, ss58, cardano, tron,
};

