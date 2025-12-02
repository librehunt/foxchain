//! Address derivation from public keys
//!
//! This module derives blockchain addresses from public keys for supported chains.

pub mod bitcoin;
pub mod cardano;
pub mod cosmos;
pub mod evm;
pub mod solana;
pub mod substrate;
pub mod tron;

// Re-export main derivation functions
pub use bitcoin::derive_bitcoin_addresses;
pub use cardano::derive_cardano_address;
pub use cosmos::derive_cosmos_address;
pub use evm::derive_evm_address;
pub use solana::derive_solana_address;
pub use substrate::derive_substrate_address;
pub use tron::derive_tron_address;
