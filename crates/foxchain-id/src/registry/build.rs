//! Registry builder for automatic category grouping
//!
//! This module builds the registry that precomputes category groups at startup,
//! automatically organizing chains by their format signatures.

use crate::loaders::{load_index, load_chain};
use crate::input::CategorySignature;
use crate::registry::{groups::CandidateGroups, AddressMetadata, ChainMetadata};
use crate::registry::chain_converter::convert_chain_config;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

/// Global registry instance
static REGISTRY: OnceLock<Registry> = OnceLock::new();

/// Registry containing all chain metadata and precomputed groups
pub struct Registry {
    /// All chain metadata
    pub chains: Vec<ChainMetadata>,
    /// Groups organized by category signature
    pub groups: CandidateGroups,
    /// Chain configs (for pipeline access)
    pub chain_configs: HashMap<String, crate::models::chain::ChainConfig>,
}

impl Registry {
    /// Build the registry with all chain metadata and automatic grouping
    pub fn build() -> Self {
        // Load index to get all chain IDs
        let index = load_index().expect("Failed to load metadata index");
        
        // Load all chain configs and convert to ChainMetadata using functional style
        let (chains, chain_configs_vec): (Vec<_>, Vec<_>) = index.chains
            .iter()
            .filter_map(|chain_id| {
                load_chain(chain_id)
                    .map_err(|e| {
                        eprintln!("Warning: Failed to load chain {}: {}", chain_id, e);
                        e
                    })
                    .ok()
                    .and_then(|config| {
                        let chain_id_clone = chain_id.clone();
                        let config_clone = config.clone();
                        convert_chain_config(config)
                            .map_err(|e| {
                                eprintln!("Warning: Failed to convert chain {}: {}", chain_id_clone, e);
                                e
                            })
                            .ok()
                            .map(|chain_metadata| (chain_metadata, (chain_id_clone, config_clone)))
                    })
            })
            .unzip();
        
        let chain_configs: HashMap<String, _> = chain_configs_vec.into_iter().collect();
        
        // Wrap chains in Arc for sharing
        let chains_arc: Vec<Arc<ChainMetadata>> = chains.iter().map(|c| Arc::new(c.clone())).collect();
        
        // Build groups by category signature using functional style
        let groups: CandidateGroups = chains_arc
            .iter()
            .flat_map(|chain| {
                // Address format signatures
                let addr_signatures = chain.address_formats.iter()
                    .map(|addr_format| {
                        (CategorySignature::from_metadata(addr_format), Arc::clone(chain))
                    });
                
                // Public key format signatures
                let pk_signatures = chain.public_key_formats.iter()
                    .map(move |pk_format| {
                        let signature = CategorySignature {
                            char_set: pk_format.char_set,
                            min_len: pk_format
                                .exact_length
                                .unwrap_or_else(|| pk_format.length_range.map(|(min, _)| min).unwrap_or(0)),
                            max_len: pk_format
                                .exact_length
                                .unwrap_or_else(|| pk_format.length_range.map(|(_, max)| max).unwrap_or(usize::MAX)),
                            has_hrp: !pk_format.hrps.is_empty(),
                            prefixes: pk_format.prefixes.clone(),
                            hrp_prefixes: pk_format.hrps.clone(),
                            encoding_type: Some(pk_format.encoding),
                        };
                        (signature, Arc::clone(chain))
                    });
                
                addr_signatures.chain(pk_signatures)
            })
            .fold(HashMap::new(), |mut acc, (signature, chain)| {
                acc.entry(signature)
                    .or_insert_with(Vec::new)
                    .push(chain);
                acc
            });
        
        Registry { chains, groups, chain_configs }
    }
    
    /// Get candidate chains for a given signature
    pub fn get_candidates(&self, signature: &CategorySignature) -> Vec<&ChainMetadata> {
        // Try exact match first
        if let Some(candidates) = self.groups.get(signature) {
            // Convert Arc<ChainMetadata> to &ChainMetadata using functional style
            return candidates
                .iter()
                .filter_map(|arc_chain| {
                    self.chains.iter().find(|c| c.id == arc_chain.id)
                })
                .collect();
        }
        
        // Fallback: check if signature matches any metadata signature
        // This handles cases where prefix normalization causes slight differences
        self.chains
            .iter()
            .filter_map(|chain| {
                chain.address_formats.iter()
                    .find(|addr_format| {
                        let metadata_sig = CategorySignature::from_metadata(addr_format);
                        // Check if signatures are compatible - be lenient with matching
                        // Encoding must match if both are Some
                        let encoding_match = match (metadata_sig.encoding_type, signature.encoding_type) {
                            (Some(m), Some(s)) => m == s,
                            (None, None) => true,
                            _ => false,
                        };
                        // Char set must match if both are Some
                        let char_set_match = match (metadata_sig.char_set, signature.char_set) {
                            (Some(m), Some(s)) => m == s,
                            (None, None) => true,
                            _ => false,
                        };
                        // Length must be in range
                        let length_match = signature.min_len >= metadata_sig.min_len
                            && signature.max_len <= metadata_sig.max_len;
                        // Prefixes must overlap (metadata prefixes empty means no requirement, or input has matching prefix)
                        let prefix_match = metadata_sig.prefixes.is_empty() || 
                            signature.prefixes.iter().any(|p| metadata_sig.prefixes.contains(p));
                        // HRP must match if required
                        let hrp_match = !metadata_sig.has_hrp || signature.has_hrp;
                        
                        encoding_match && char_set_match && length_match && prefix_match && hrp_match
                    })
                    .map(|_| chain)
            })
            .collect()
    }
    
    /// Get the global registry instance
    pub fn get() -> &'static Registry {
        REGISTRY.get_or_init(|| Registry::build())
    }
    
    /// Find all chains that support a given address format
    /// This matches an address string against all chain metadata
    #[allow(dead_code)] // Reserved for future use
    pub fn find_chains_for_address(&self, address: &str) -> Vec<&ChainMetadata> {
        use crate::input::extract_characteristics;
        
        let chars = extract_characteristics(address);
        
        self.chains
            .iter()
            .filter(|chain| {
                chain.address_formats.iter().any(|addr_format| {
                    // Check if address matches this format
                    matches_address_format(&chars, addr_format)
                })
            })
            .collect()
    }
    
    /// Find chains that support a given public key type (curve)
    pub fn find_chains_for_public_key_type(&self, key_type: crate::registry::PublicKeyType) -> Vec<&ChainMetadata> {
        self.chains
            .iter()
            .filter(|chain| {
                chain.public_key_formats.iter().any(|pk_format| {
                    pk_format.key_type == key_type
                })
            })
            .collect()
    }
    
    /// Find chains that match address format characteristics
    #[allow(dead_code)] // Reserved for future use
    pub fn find_chains_for_address_format(&self, chars: &crate::input::InputCharacteristics) -> Vec<&ChainMetadata> {
        self.chains
            .iter()
            .filter(|chain| {
                chain.address_formats.iter().any(|addr_format| {
                    matches_address_format(chars, addr_format)
                })
            })
            .collect()
    }
    
    /// Get chain config by ID
    pub fn get_chain_config(&self, chain_id: &str) -> Option<&crate::models::chain::ChainConfig> {
        self.chain_configs.get(chain_id)
    }
}

/// Check if input characteristics match address metadata
#[allow(dead_code)] // Used by find_chains_for_address methods
fn matches_address_format(chars: &crate::input::InputCharacteristics, metadata: &AddressMetadata) -> bool {
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
    
    // Check encoding type - match if any of the detected encodings matches
    if !chars.encoding.is_empty() {
        if !chars.encoding.contains(&metadata.encoding) {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_build() {
        let registry = Registry::build();
        assert!(!registry.chains.is_empty());
    }

    #[test]
    fn test_registry_get() {
        let registry = Registry::get();
        assert!(!registry.chains.is_empty());
    }
}
