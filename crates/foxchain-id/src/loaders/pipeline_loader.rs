use crate::models::pipeline::AddressPipeline;
use serde_json;

/// Load address pipeline metadata by ID
pub fn load_pipeline(id: &str) -> Result<AddressPipeline, String> {
    let json = match id {
        "evm" => include_str!("../../metadata/pipelines/addresses/evm.json"),
        "bitcoin_p2pkh" => include_str!("../../metadata/pipelines/addresses/bitcoin_p2pkh.json"),
        "bitcoin_bech32" => include_str!("../../metadata/pipelines/addresses/bitcoin_bech32.json"),
        "cosmos" => include_str!("../../metadata/pipelines/addresses/cosmos.json"),
        "solana" => include_str!("../../metadata/pipelines/addresses/solana.json"),
        "ss58" => include_str!("../../metadata/pipelines/addresses/ss58.json"),
        "cardano" => include_str!("../../metadata/pipelines/addresses/cardano.json"),
        "tron" => include_str!("../../metadata/pipelines/addresses/tron.json"),
        _ => return Err(format!("Unknown pipeline: {}", id)),
    };
    serde_json::from_str(json)
        .map_err(|e| format!("Failed to parse pipeline JSON for {}: {}", id, e))
}
