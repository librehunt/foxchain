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
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let input = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
//!     let candidates = identify(input)?; // returns Vec<IdentificationCandidate>
//!
//!     // Pick Ethereum if present
//!     if let Some(candidate) = candidates.iter().find(|c| c.chain == "ethereum") {
//!         let addr = &candidate.normalized;
//!         let client = Client::for_chain("ethereum")?; // uses env-configured providers
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
pub use foxchain_analysis::{AccountSummary, Client, TokenBalance};
pub use foxchain_id::{identify, IdentificationCandidate, InputType};
