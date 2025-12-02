use crate::models::chain::ChainConfig;
use serde_json;

/// Load chain metadata by ID
/// This function uses include_str! to load JSON at compile time
pub fn load_chain(id: &str) -> Result<ChainConfig, String> {
    let json = match id {
        "ethereum" => include_str!("../../metadata/chains/ethereum.json"),
        "polygon" => include_str!("../../metadata/chains/polygon.json"),
        "bsc" => include_str!("../../metadata/chains/bsc.json"),
        "avalanche" => include_str!("../../metadata/chains/avalanche.json"),
        "arbitrum" => include_str!("../../metadata/chains/arbitrum.json"),
        "optimism" => include_str!("../../metadata/chains/optimism.json"),
        "base" => include_str!("../../metadata/chains/base.json"),
        "fantom" => include_str!("../../metadata/chains/fantom.json"),
        "celo" => include_str!("../../metadata/chains/celo.json"),
        "gnosis" => include_str!("../../metadata/chains/gnosis.json"),
        "bitcoin" => include_str!("../../metadata/chains/bitcoin.json"),
        "litecoin" => include_str!("../../metadata/chains/litecoin.json"),
        "dogecoin" => include_str!("../../metadata/chains/dogecoin.json"),
        "solana" => include_str!("../../metadata/chains/solana.json"),
        "tron" => include_str!("../../metadata/chains/tron.json"),
        "cosmos_hub" => include_str!("../../metadata/chains/cosmos_hub.json"),
        "osmosis" => include_str!("../../metadata/chains/osmosis.json"),
        "juno" => include_str!("../../metadata/chains/juno.json"),
        "akash" => include_str!("../../metadata/chains/akash.json"),
        "stargaze" => include_str!("../../metadata/chains/stargaze.json"),
        "secret_network" => include_str!("../../metadata/chains/secret_network.json"),
        "terra" => include_str!("../../metadata/chains/terra.json"),
        "kava" => include_str!("../../metadata/chains/kava.json"),
        "regen" => include_str!("../../metadata/chains/regen.json"),
        "sentinel" => include_str!("../../metadata/chains/sentinel.json"),
        "polkadot" => include_str!("../../metadata/chains/polkadot.json"),
        "kusama" => include_str!("../../metadata/chains/kusama.json"),
        "substrate" => include_str!("../../metadata/chains/substrate.json"),
        "cardano" => include_str!("../../metadata/chains/cardano.json"),
        _ => return Err(format!("Unknown chain: {}", id)),
    };
    serde_json::from_str(json).map_err(|e| format!("Failed to parse chain JSON for {}: {}", id, e))
}
