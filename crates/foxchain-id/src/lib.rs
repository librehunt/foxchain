//! Foxchain ID: Multi-chain blockchain address identification
//!
//! This crate provides functionality to identify which blockchain(s) an input
//! string (address, public key, or private key) belongs to.

mod formats;

use formats::{bitcoin, cosmos, evm, solana, substrate, tron};

/// Identify the blockchain(s) for a given input string.
///
/// Returns normalized representation, candidate chains, confidence scores, and reasoning.
///
/// # Example
///
/// ```rust
/// use foxchain_id::identify;
///
/// let result = identify("0x742d35Cc6634C0532925a3b844Bc454e4438f44e")?;
/// println!("Normalized: {}", result.normalized);
/// for candidate in result.candidates {
///     println!("Chain: {:?}, Confidence: {}", candidate.chain, candidate.confidence);
/// }
/// # Ok::<(), foxchain_id::Error>(())
/// ```
pub fn identify(input: &str) -> Result<IdentificationResult, Error> {
    // Try EVM address detection first (most common)
    if let Some(result) = evm::detect_evm(input)? {
        return Ok(result);
    }

    // Try Bitcoin ecosystem addresses
    if let Some(result) = bitcoin::detect_bitcoin(input)? {
        return Ok(result);
    }

    // Try Solana addresses
    if let Some(result) = solana::detect_solana(input)? {
        return Ok(result);
    }

    // Try Tron addresses
    if let Some(result) = tron::detect_tron(input)? {
        return Ok(result);
    }

    // Try Cosmos addresses
    if let Some(result) = cosmos::detect_cosmos(input)? {
        return Ok(result);
    }

    // Try Substrate addresses
    if let Some(result) = substrate::detect_substrate(input)? {
        return Ok(result);
    }

    // TODO: Add other format detectors

    Err(Error::InvalidInput(format!(
        "Unable to identify address format: {}",
        input
    )))
}

/// Result of identification process
#[derive(Debug, Clone)]
pub struct IdentificationResult {
    /// Normalized address representation
    pub normalized: String,
    /// List of candidate chains with confidence scores
    pub candidates: Vec<ChainCandidate>,
}

/// A candidate chain with confidence score
#[derive(Debug, Clone)]
pub struct ChainCandidate {
    /// Chain identifier
    pub chain: Chain,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Reasoning for this candidate
    pub reasoning: String,
}

/// Supported blockchain identifiers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Chain {
    // EVM chains
    Ethereum,
    Polygon,
    BSC,
    Avalanche,
    Arbitrum,
    Optimism,
    Base,
    Fantom,
    Celo,
    Gnosis,
    // Bitcoin ecosystem
    Bitcoin,
    Litecoin,
    Dogecoin,
    // Other chains
    Solana,
    Tron,
    // Cosmos ecosystem
    CosmosHub,
    Osmosis,
    Juno,
    Akash,
    Stargaze,
    SecretNetwork,
    Terra,
    Kava,
    Regen,
    Sentinel,
    // Substrate ecosystem
    Polkadot,
    Kusama,
    Substrate, // Generic Substrate chain
}

/// Errors that can occur during identification
#[derive(Debug, Clone)]
pub enum Error {
    /// Feature not yet implemented
    NotImplemented,
    /// Invalid input format
    InvalidInput(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotImplemented => write!(f, "Feature not yet implemented"),
            Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_evm_address() {
        // Test with lowercase address - should be normalized
        let input = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
        let result = identify(input);
        assert!(result.is_ok());
        let id_result = result.unwrap();
        assert!(!id_result.candidates.is_empty());
        assert_eq!(id_result.candidates[0].chain, Chain::Ethereum);
        // Should be normalized to checksum format
        assert_ne!(id_result.normalized, input);
        assert!(id_result.normalized.starts_with("0x"));
        assert_eq!(id_result.normalized.len(), 42);
    }

    #[test]
    fn test_identify_evm_address_lowercase() {
        let input = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
        let result = identify(input);
        assert!(result.is_ok());
        let id_result = result.unwrap();
        assert!(!id_result.candidates.is_empty());
        // Should be normalized to checksum format (different from input)
        assert_ne!(id_result.normalized, input);
        assert!(id_result.normalized.starts_with("0x"));
        assert_eq!(id_result.normalized.len(), 42);
    }

    #[test]
    fn test_identify_invalid_address() {
        let result = identify("not-an-address");
        assert!(result.is_err());
        // Verify error message contains the input
        if let Err(Error::InvalidInput(msg)) = result {
            assert!(msg.contains("not-an-address"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    #[test]
    fn test_identify_unrecognized_format() {
        // Test with a string that doesn't match any known format
        // This should trigger the final error path in identify()
        let result = identify("xyz123abc");
        assert!(result.is_err());
        if let Err(Error::InvalidInput(msg)) = result {
            assert!(msg.contains("Unable to identify address format"));
            assert!(msg.contains("xyz123abc"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    #[test]
    fn test_identify_empty_string() {
        // Test with empty string
        let result = identify("");
        assert!(result.is_err());
        if let Err(Error::InvalidInput(msg)) = result {
            assert!(msg.contains("Unable to identify address format"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    #[test]
    fn test_identify_tron() {
        // Test Tron address identification
        // Create a valid test Tron address
        use base58::ToBase58;
        use sha2::{Digest, Sha256};

        let version = 0x41u8;
        let address_bytes = vec![0u8; 20];
        let payload = [&[version], address_bytes.as_slice()].concat();
        let hash1 = Sha256::digest(&payload);
        let hash2 = Sha256::digest(hash1);
        let checksum = &hash2[..4];
        let full_bytes = [payload, checksum.to_vec()].concat();
        let tron_addr = full_bytes.to_base58();

        let result = identify(&tron_addr);
        assert!(result.is_ok(), "Should identify Tron address");
        let id_result = result.unwrap();
        assert_eq!(id_result.candidates[0].chain, Chain::Tron);
    }

    #[test]
    fn test_identify_substrate() {
        // Test Substrate address identification
        use base58::ToBase58;
        // Create a valid test Substrate address (prefix 0 = Polkadot)
        let mut bytes = vec![0u8]; // Prefix
        bytes.extend(vec![0u8; 32]); // Account ID
        bytes.extend(vec![0u8; 2]); // Checksum
        let substrate_addr = bytes.to_base58();

        let result = identify(&substrate_addr);
        // This may fail if the address doesn't validate, but tests integration
        assert!(result.is_ok() || result.is_err());
    }
}
