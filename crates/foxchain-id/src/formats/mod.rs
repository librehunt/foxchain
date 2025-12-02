//! Format definitions module
//!
//! This module contains declarative metadata definitions for all supported chains.
//! Each chain defines its supported address and public key formats.

pub mod bitcoin;
pub mod evm;
pub mod solana;

use crate::registry::ChainMetadata;

/// Get all chain metadata
pub fn all_metadata() -> Vec<ChainMetadata> {
    let mut all = Vec::new();
    all.extend(evm::evm_metadata());
    all.extend(bitcoin::bitcoin_metadata());
    all.extend(solana::solana_metadata());
    // TODO: Add other chains (Cosmos, Substrate, Tron, Cardano)
    all
}
