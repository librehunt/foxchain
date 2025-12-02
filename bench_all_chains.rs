use foxchain_id::identify;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::time::Instant;

// Constants: sample public keys
// Uncompressed secp256k1 public key (65 bytes, 0x04 prefix)
const SECP256K1_UNCOMPRESSED: &str = "0x0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
// Ed25519 public key (32 bytes)
const ED25519_HEX: &str = "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
// Solana public key (base58) - use USDC mint (32 bytes)
const SOLANA_BASE58_PK: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

fn load_index(repo_root: &Path) -> Result<Value, Box<dyn std::error::Error>> {
    let p = repo_root.join("metadata/index.json");
    let s = fs::read_to_string(p)?;
    Ok(serde_json::from_str(&s)?)
}

fn load_chain(repo_root: &Path, chain_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let p = repo_root.join("metadata/chains").join(format!("{}.json", chain_id));
    let s = fs::read_to_string(p)?;
    Ok(serde_json::from_str(&s)?)
}

fn pk_input_for_chain(chain_cfg: &Value, chain_id: &str) -> &'static str {
    match chain_cfg.get("curve").and_then(|v| v.as_str()) {
        Some("secp256k1") => SECP256K1_UNCOMPRESSED,
        Some("ed25519") => {
            if chain_id == "solana" {
                SOLANA_BASE58_PK
            } else {
                ED25519_HEX
            }
        }
        _ => ED25519_HEX,
    }
}

fn fallback_address(chain_id: &str) -> Option<&'static str> {
    match chain_id {
        // EVM chains - canonical burn address
        "ethereum" | "polygon" | "bsc" | "avalanche" | "arbitrum" | "optimism" | "base" | "fantom" | "celo" | "gnosis" =>
            Some("0x000000000000000000000000000000000000dEaD"),
        // Bitcoin family
        "bitcoin" => Some("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
        "litecoin" => Some("LcNS6c8RddAMjewDrUAAi8BzecKoosnkN3"),
        "dogecoin" => Some("DH5yaieqoZN36fDVciNyRueRGvGLR3mr7L"),
        // Cosmos family (use standard example payload with HRP)
        "cosmos_hub" => Some("cosmos1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"),
        "osmosis" => Some("osmo1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"),
        "juno" => Some("juno1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"),
        "akash" => Some("akash1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"),
        "stargaze" => Some("stars1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"),
        "secret_network" => Some("secret1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"),
        "terra" => Some("terra1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"),
        "kava" => Some("kava1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"),
        "regen" => Some("regen1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"),
        "sentinel" => Some("sent1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"),
        // Substrate/Polkadot family
        "polkadot" => Some("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"),
        // Kusama/Substrate generic: if derivation fails, skip; fallback not guaranteed
        "solana" => Some("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
        "tron" => Some("T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb"),
        "cardano" => Some("addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3jcu5d8ps7zex2k2xt3uqxgjqnnjhl2zqwpg7h3vj6"),
        _ => None,
    }
}

fn time_ms<F: Fn()> (iters: usize, f: F) -> f64 {
    let start = Instant::now();
    for _ in 0..iters {
        f();
    }
    let elapsed = start.elapsed();
    (elapsed.as_secs_f64() * 1000.0) / (iters as f64)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Repo root for this crate (examples/ is under crate dir)
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let index = load_index(repo_root)?;
    let chains = index
        .get("chains")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "invalid index.json: missing chains")?;

    // Keep results
    let mut results: Vec<(String, Option<f64>, Option<f64>)> = Vec::new();

    // Warm up: run one identify to initialize registry, etc.
    let _ = identify("0x000000000000000000000000000000000000dEaD");

    // Iterations per measurement
    let iters = 50usize;

    for chain_val in chains {
        let chain_id = chain_val.as_str().unwrap().to_string();
        let chain_cfg = load_chain(repo_root, &chain_id)?;

        // Build public key input for this chain
        let pk_input = pk_input_for_chain(&chain_cfg, &chain_id);

        // Time public key identification (may yield 0 candidates for some chains)
        let pk_ms = time_ms(iters, || {
            let _ = identify(pk_input);
        });

        // Try to get a derived address for this specific chain via PK identification
        let addr_from_pk: Option<String> = identify(pk_input)
            .ok()
            .and_then(|candidates| candidates.into_iter().find(|c| c.chain == chain_id).map(|c| c.normalized));

        // Choose address input: derived if available, else fallback known example
        let address_input: Option<String> = match addr_from_pk {
            Some(addr) => Some(addr),
            None => fallback_address(&chain_id).map(|s| s.to_string()),
        };

        // Time address identification if we have an address
        let addr_ms = address_input.as_ref().map(|addr| {
            time_ms(iters, || {
                let _ = identify(addr);
            })
        });

        results.push((chain_id, addr_ms, Some(pk_ms)));
    }

    // Print results
    // Format: chain,address_ms,public_key_ms (ms)
    println!("chain,address_ms,public_key_ms");
    for (chain, addr_ms, pk_ms) in &results {
        let addr_s = addr_ms.map(|v| format!("{:.3}", v)).unwrap_or_else(|| "N/A".to_string());
        let pk_s = pk_ms.map(|v| format!("{:.3}", v)).unwrap_or_else(|| "N/A".to_string());
        println!("{},{},{}", chain, addr_s, pk_s);
    }

    Ok(())
}
