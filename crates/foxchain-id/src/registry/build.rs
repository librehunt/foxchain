//! Registry builder for automatic category grouping
//!
//! This module builds the registry that precomputes category groups at startup,
//! automatically organizing chains by their format signatures.

use crate::loaders::{load_index, load_chain};
use crate::registry::ChainMetadata;
use crate::registry::chain_converter::convert_chain_config;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Global registry instance
static REGISTRY: OnceLock<Registry> = OnceLock::new();

/// Registry containing all chain metadata and precomputed groups
pub struct Registry {
    /// All chain metadata
    pub chains: Vec<ChainMetadata>,
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
        
        Registry { chains, chain_configs }
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
fn matches_address_format(chars: &crate::input::InputCharacteristics, metadata: &crate::registry::AddressMetadata) -> bool {
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
