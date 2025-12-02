//! Registry builder for automatic category grouping
//!
//! This module builds the registry that precomputes category groups at startup,
//! automatically organizing chains by their format signatures.

use crate::formats;
use crate::input::CategorySignature;
use crate::registry::{groups::CandidateGroups, ChainMetadata};
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
}

impl Registry {
    /// Build the registry with all chain metadata and automatic grouping
    pub fn build() -> Self {
        // Load all chain metadata from formats/* modules
        let chains = formats::all_metadata();
        
        // Wrap chains in Arc for sharing
        let chains_arc: Vec<Arc<ChainMetadata>> = chains.into_iter().map(Arc::new).collect();
        
        // Build groups by category signature
        let mut groups: CandidateGroups = HashMap::new();
        
        for chain in &chains_arc {
            // Group by address formats
            for addr_format in &chain.address_formats {
                let signature = CategorySignature::from_metadata(addr_format);
                groups
                    .entry(signature)
                    .or_insert_with(Vec::new)
                    .push(Arc::clone(chain));
            }
            
            // Group by public key formats
            for pk_format in &chain.public_key_formats {
                // For public keys, we need to create a signature from the format
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
                groups
                    .entry(signature)
                    .or_insert_with(Vec::new)
                    .push(Arc::clone(chain));
            }
        }
        
        // Convert back to owned chains for storage
        let chains: Vec<ChainMetadata> = chains_arc.into_iter().map(|arc| (*arc).clone()).collect();
        
        Registry { chains, groups }
    }
    
    /// Get candidate chains for a given signature
    pub fn get_candidates(&self, signature: &CategorySignature) -> Vec<&ChainMetadata> {
        // Try exact match first
        if let Some(candidates) = self.groups.get(signature) {
            // Convert Arc<ChainMetadata> to &ChainMetadata
            // We need to find the chains in self.chains that match
            let mut result = Vec::new();
            for arc_chain in candidates {
                // Find matching chain in self.chains
                if let Some(chain) = self.chains.iter().find(|c| c.id == arc_chain.id) {
                    result.push(chain);
                }
            }
            return result;
        }
        
        // TODO: Implement fuzzy matching for similar signatures
        // For now, return empty
        Vec::new()
    }
    
    /// Get the global registry instance
    pub fn get() -> &'static Registry {
        REGISTRY.get_or_init(|| Registry::build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_build() {
        let registry = Registry::build();
        assert!(registry.chains.is_empty());
        assert!(registry.groups.is_empty());
    }

    #[test]
    fn test_registry_get() {
        let registry = Registry::get();
        assert!(registry.chains.is_empty());
    }
}

