//! Main identification pipeline
//!
//! This module implements the 4-stage metadata-driven identification pipeline:
//! 1. Extract input characteristics
//! 2. Compute category signature
//! 3. Get candidate groups from registry
//! 4. Filter by metadata + run detectors

use crate::detectors::{detect_address, detect_public_key, DetectionResult};
use crate::input::{extract_characteristics, CategorySignature, InputCharacteristics};
use crate::registry::{AddressMetadata, ChainMetadata, PublicKeyMetadata, Registry};
use crate::{Chain, Error};

/// A candidate identification result
#[derive(Debug, Clone)]
pub struct IdentificationCandidate {
    /// Type of input (address or public key)
    pub input_type: InputType,
    /// Chain identifier
    pub chain: Chain,
    /// Encoding type used
    pub encoding: crate::registry::EncodingType,
    /// Normalized representation
    pub normalized: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Reasoning for this candidate
    pub reasoning: String,
}

/// Type of input being identified
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    /// Address input
    Address,
    /// Public key input
    PublicKey,
    // Future: Transaction, Block, PrivateKey
}

/// Identify the blockchain(s) for a given input string
///
/// Returns all valid candidates sorted by confidence (highest first).
/// This function supports ambiguous inputs that may match multiple chains.
pub fn identify(input: &str) -> Result<Vec<IdentificationCandidate>, Error> {
    // Step 1: Extract characteristics
    let chars = extract_characteristics(input);
    
    // Step 2: Compute signature
    let signature = CategorySignature::from(&chars);
    
    // Step 3: Get candidate groups from registry
    let registry = Registry::get();
    let candidate_chains = registry.get_candidates(&signature);
    
    // Step 4: Filter by metadata + run detectors
    let mut results: Vec<IdentificationCandidate> = candidate_chains
        .iter()
        .flat_map(|chain_metadata| {
            // Try address detection
            chain_metadata.address_formats
                .iter()
                .filter(|addr_format| matches_metadata(&chars, addr_format))
                .filter_map(|addr_format| {
                    detect_address(input, &chars, addr_format, chain_metadata.id.clone())
                        .ok()
                        .flatten()
                })
                .map(|result| IdentificationCandidate {
                    input_type: InputType::Address,
                    chain: result.chain,
                    encoding: result.encoding,
                    normalized: result.normalized,
                    confidence: result.confidence,
                    reasoning: result.reasoning,
                })
                .chain(
                    // Try public key detection
                    chain_metadata.public_key_formats
                        .iter()
                        .filter(|pk_format| matches_metadata_pk(&chars, pk_format))
                        .filter_map(|pk_format| {
                            detect_public_key(input, &chars, pk_format, chain_metadata.id.clone())
                                .ok()
                                .flatten()
                        })
                        .map(|result| IdentificationCandidate {
                            input_type: InputType::PublicKey,
                            chain: result.chain,
                            encoding: result.encoding,
                            normalized: result.normalized,
                            confidence: result.confidence,
                            reasoning: result.reasoning,
                        })
                )
        })
        .collect();
    
    // Sort by confidence (highest first)
    results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
    
    if results.is_empty() {
        Err(Error::InvalidInput(format!(
            "Unable to identify address format: {}",
            input
        )))
    } else {
        Ok(results)
    }
}

/// Check if input characteristics match address metadata
fn matches_metadata(chars: &InputCharacteristics, metadata: &AddressMetadata) -> bool {
    // Check length
    if let Some(exact) = metadata.exact_length {
        if chars.length != exact {
            return false;
        }
    }
    if let Some((min, max)) = metadata.length_range {
        if chars.length < min || chars.length > max {
            return false;
        }
    }
    
    // Check prefixes
    if !metadata.prefixes.is_empty() {
        if !metadata.prefixes.iter().any(|p| chars.prefixes.contains(p)) {
            return false;
        }
    }
    
    // Check HRP
    if !metadata.hrps.is_empty() {
        if let Some(ref hrp) = chars.hrp {
            if !metadata.hrps.iter().any(|h| hrp.starts_with(h)) {
                return false;
            }
        } else {
            return false;
        }
    }
    
    // Check character set
    if let Some(ref char_set) = metadata.char_set {
        if chars.char_set != *char_set {
            return false;
        }
    }
    
    // Check encoding type
    if let Some(ref encoding) = chars.encoding {
        if *encoding != metadata.encoding {
            return false;
        }
    }
    
    true
}

/// Check if input characteristics match public key metadata
fn matches_metadata_pk(chars: &InputCharacteristics, metadata: &PublicKeyMetadata) -> bool {
    // Check length
    if let Some(exact) = metadata.exact_length {
        if chars.length != exact {
            return false;
        }
    }
    if let Some((min, max)) = metadata.length_range {
        if chars.length < min || chars.length > max {
            return false;
        }
    }
    
    // Check prefixes
    if !metadata.prefixes.is_empty() {
        if !metadata.prefixes.iter().any(|p| chars.prefixes.contains(p)) {
            return false;
        }
    }
    
    // Check HRP
    if !metadata.hrps.is_empty() {
        if let Some(ref hrp) = chars.hrp {
            if !metadata.hrps.iter().any(|h| hrp.starts_with(h)) {
                return false;
            }
        } else {
            return false;
        }
    }
    
    // Check character set
    if let Some(ref char_set) = metadata.char_set {
        if chars.char_set != *char_set {
            return false;
        }
    }
    
    // Check encoding type
    if let Some(ref encoding) = chars.encoding {
        if *encoding != metadata.encoding {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identify_empty_input() {
        let result = identify("");
        assert!(result.is_err());
    }

    #[test]
    fn test_identify_invalid_input() {
        let result = identify("not-an-address");
        assert!(result.is_err());
    }
}

