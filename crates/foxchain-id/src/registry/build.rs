//! Registry builder for automatic category grouping
//!
//! This module builds the registry that precomputes category groups at startup,
//! automatically organizing chains by their format signatures.

use crate::input::CategorySignature;
use crate::registry::{groups::CandidateGroups, ChainMetadata};
use std::collections::HashMap;
use std::sync::OnceLock;

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
        // TODO: Load all chain metadata from formats/* modules
        // For now, return empty registry
        let chains = Vec::new();
        let groups = HashMap::new();
        
        Registry { chains, groups }
    }
    
    /// Get candidate chains for a given signature
    pub fn get_candidates(&self, signature: &CategorySignature) -> Vec<&ChainMetadata> {
        // Try exact match first
        if let Some(candidates) = self.groups.get(signature) {
            return candidates.clone();
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

