//! Foxchain: Multi-chain blockchain address identification and analysis library
//!
//! This library provides two main crates:
//! - `foxchain-id`: Identify which blockchain(s) an address belongs to
//! - `foxchain-analysis`: Retrieve on-chain data for identified wallets
//!
//! # Example
//!
//! ```no_run
//! use foxchain::foxchain_id::identify;
//! use foxchain::foxchain_analysis::Client;
//! use foxchain::Chain;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let input = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
//!     let id = identify(input)?; // returns candidates + normalized form
//!
//!     // Pick Ethereum if present
//!     if let Some(candidate) = id.candidates.iter().find(|c| c.chain == Chain::Ethereum) {
//!         let addr = &id.normalized;
//!         let client = Client::for_chain(Chain::Ethereum)?; // uses env-configured providers
//!         let summary = client.account_summary(addr)?; // balances, tx count, tokens, etc.
//!         println!("{:?}", summary);
//!     }
//!     Ok(())
//! }
//! ```

// Re-export foxchain-id crate
pub use foxchain_id;

// Re-export foxchain-analysis crate
pub use foxchain_analysis;

// Re-export commonly used types for convenience
pub use foxchain_id::{Chain, identify, IdentificationResult, ChainCandidate};
pub use foxchain_analysis::{Client, AccountSummary, TokenBalance};
