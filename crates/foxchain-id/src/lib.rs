//! Foxchain ID: Multi-chain blockchain address identification
//!
//! This crate provides functionality to identify which blockchain(s) an input
//! string (address, public key, or private key) belongs to.

mod formats;

use formats::evm;

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

    // TODO: Add other format detectors (Bitcoin, Solana, Cosmos, etc.)

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
    // Other chains
    Bitcoin,
    Solana,
    Cosmos,
    Polkadot,
    Tron,
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
    }
}
