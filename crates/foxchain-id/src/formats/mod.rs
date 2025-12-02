//! Format definitions module
//!
//! This module contains declarative metadata definitions for all supported chains.
//! Each chain defines its supported address and public key formats.

pub mod bitcoin;
pub mod cardano;
pub mod cosmos;
pub mod evm;
pub mod solana;
pub mod substrate;
pub mod tron;

use crate::registry::ChainMetadata;

/// Get all chain metadata
pub fn all_metadata() -> Vec<ChainMetadata> {
    let mut all = Vec::new();
    all.extend(evm::evm_metadata());
    all.extend(bitcoin::bitcoin_metadata());
    all.extend(solana::solana_metadata());
    all.extend(cosmos::cosmos_metadata());
    all.extend(substrate::substrate_metadata());
    all.extend(tron::tron_metadata());
    all.extend(cardano::cardano_metadata());
    all
}
