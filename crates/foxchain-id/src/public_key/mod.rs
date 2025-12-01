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
            // EVM address derivation
            if derive_evm_address(&key_bytes)?.is_some() {
                candidates.push(ChainCandidate {
                    chain: Chain::Ethereum,
                    confidence: 0.85,
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
            if let Some(_) = derive_solana_address(&key_bytes) {
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
        PublicKeyType::Secp256k1 => {
            derive_evm_address(&key_bytes)?.unwrap_or_else(|| "unknown".to_string())
        }
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
        // Should have EVM and Bitcoin candidates
        assert!(id_result
            .candidates
            .iter()
            .any(|c| matches!(c.chain, Chain::Ethereum)));
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
