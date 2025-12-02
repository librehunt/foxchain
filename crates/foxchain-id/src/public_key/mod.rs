//! Public key detection and address derivation
//!
//! This module detects public keys in various formats (hex, base58, bech32) and
//! derives addresses for supported blockchains.

pub mod derivation;
pub mod detection;

use crate::{Chain, ChainCandidate, Error, IdentificationResult};
use derivation::{
    derive_bitcoin_addresses, derive_cosmos_address, derive_evm_address, derive_solana_address,
};
use detection::{detect, PublicKeyFormat, PublicKeyType};

/// Detect if input is a public key and derive addresses
pub fn detect_public_key(input: &str) -> Result<Option<IdentificationResult>, Error> {
    // Try to detect public key format
    let (format, key_bytes, key_type) = match detect(input)? {
        Some((fmt, bytes, kt)) => (fmt, bytes, kt),
        None => return Ok(None),
    };

    // Derive addresses based on public key type
    let mut candidates = Vec::new();

    match key_type {
        PublicKeyType::Secp256k1 => {
            // EVM address derivation - returns all 10 EVM chains
            let evm_addresses = derive_evm_address(&key_bytes)?;
            for (chain, _address) in evm_addresses {
                let confidence = if matches!(chain, Chain::Ethereum) {
                    0.85
                } else {
                    0.80
                };
                candidates.push(ChainCandidate {
                    chain,
                    confidence,
                    reasoning: format!(
                        "EVM address derived from {} secp256k1 public key",
                        match format {
                            PublicKeyFormat::Hex => "hex",
                            PublicKeyFormat::Base58 => "base58",
                            PublicKeyFormat::Bech32 => "bech32",
                        }
                    ),
                });
            }

            // Bitcoin address derivation
            let bitcoin_addresses = derive_bitcoin_addresses(&key_bytes)?;
            for (chain, _address) in bitcoin_addresses {
                candidates.push(ChainCandidate {
                    chain,
                    confidence: 0.80,
                    reasoning: format!(
                        "Bitcoin address derived from {} secp256k1 public key",
                        match format {
                            PublicKeyFormat::Hex => "hex",
                            PublicKeyFormat::Base58 => "base58",
                            PublicKeyFormat::Bech32 => "bech32",
                        }
                    ),
                });
            }
        }
        PublicKeyType::Ed25519 => {
            // Solana address derivation
            if derive_solana_address(&key_bytes).is_some() {
                candidates.push(ChainCandidate {
                    chain: Chain::Solana,
                    confidence: 0.85,
                    reasoning: format!(
                        "Solana address derived from {} Ed25519 public key",
                        match format {
                            PublicKeyFormat::Hex => "hex",
                            PublicKeyFormat::Base58 => "base58",
                            PublicKeyFormat::Bech32 => "bech32",
                        }
                    ),
                });
            }

            // Cosmos address derivation
            if derive_cosmos_address(&key_bytes)?.is_some() {
                candidates.push(ChainCandidate {
                    chain: Chain::CosmosHub,
                    confidence: 0.80,
                    reasoning: format!(
                        "Cosmos address derived from {} Ed25519 public key",
                        match format {
                            PublicKeyFormat::Hex => "hex",
                            PublicKeyFormat::Base58 => "base58",
                            PublicKeyFormat::Bech32 => "bech32",
                        }
                    ),
                });
            }
        }
        PublicKeyType::Unknown => {
            // For unknown key types, we can't derive addresses
            return Ok(None);
        }
    }

    if candidates.is_empty() {
        return Ok(None);
    }

    // Use the first derived address as normalized representation
    // For secp256k1, prefer EVM address; for Ed25519, prefer Solana
    let normalized = match key_type {
        PublicKeyType::Secp256k1 => derive_evm_address(&key_bytes)?
            .first()
            .map(|(_, addr)| addr.clone())
            .unwrap_or_else(|| "unknown".to_string()),
        PublicKeyType::Ed25519 => {
            if let Some(addr) = derive_solana_address(&key_bytes) {
                addr
            } else {
                derive_cosmos_address(&key_bytes)?.unwrap_or_else(|| "unknown".to_string())
            }
        }
        PublicKeyType::Unknown => return Ok(None),
    };

    Ok(Some(IdentificationResult {
        normalized,
        candidates,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_public_key_hex_secp256k1() {
        // Test full detection flow with hex secp256k1 public key
        let key_hex = "0x0479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
        let result = detect_public_key(key_hex).unwrap();
        assert!(result.is_some());
        let id_result = result.unwrap();
        assert!(!id_result.candidates.is_empty());
        // Should have all 10 EVM chains and Bitcoin candidates
        let evm_chains: Vec<_> = id_result
            .candidates
            .iter()
            .filter(|c| {
                matches!(
                    c.chain,
                    Chain::Ethereum
                        | Chain::Polygon
                        | Chain::BSC
                        | Chain::Avalanche
                        | Chain::Arbitrum
                        | Chain::Optimism
                        | Chain::Base
                        | Chain::Fantom
                        | Chain::Celo
                        | Chain::Gnosis
                )
            })
            .collect();
        assert_eq!(evm_chains.len(), 10, "Should have all 10 EVM chains");
        // Verify Ethereum has highest confidence
        let ethereum = id_result
            .candidates
            .iter()
            .find(|c| matches!(c.chain, Chain::Ethereum))
            .unwrap();
        assert_eq!(ethereum.confidence, 0.85);
        // Verify other EVM chains have 0.80 confidence
        for candidate in evm_chains.iter() {
            if !matches!(candidate.chain, Chain::Ethereum) {
                assert_eq!(candidate.confidence, 0.80);
            }
        }
        // Should also have all Bitcoin ecosystem candidates
        let bitcoin_chains: Vec<_> = id_result
            .candidates
            .iter()
            .filter(|c| matches!(c.chain, Chain::Bitcoin | Chain::Litecoin | Chain::Dogecoin))
            .collect();
        assert_eq!(
            bitcoin_chains.len(),
            3,
            "Should have all 3 Bitcoin ecosystem chains"
        );
        assert!(id_result
            .candidates
            .iter()
            .any(|c| matches!(c.chain, Chain::Bitcoin)));
        assert!(id_result
            .candidates
            .iter()
            .any(|c| matches!(c.chain, Chain::Litecoin)));
        assert!(id_result
            .candidates
            .iter()
            .any(|c| matches!(c.chain, Chain::Dogecoin)));
    }

    #[test]
    fn test_detect_public_key_hex_ed25519() {
        // Test full detection flow with hex Ed25519 public key
        let key_hex = "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let result = detect_public_key(key_hex).unwrap();
        assert!(result.is_some());
        let id_result = result.unwrap();
        assert!(!id_result.candidates.is_empty());
        // Should have Solana and Cosmos candidates
        assert!(id_result
            .candidates
            .iter()
            .any(|c| matches!(c.chain, Chain::Solana | Chain::CosmosHub)));
    }
}
