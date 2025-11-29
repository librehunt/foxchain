# foxchain
Multi-chain blockchain address identification and analysis library

[![GitHub last commit](https://img.shields.io/github/last-commit/librehunt/foxchain)](https://github.com/librehunt/foxchain/commits/main)
[![CI](https://github.com/librehunt/foxchain/workflows/CI/badge.svg)](https://github.com/librehunt/foxchain/actions)
[![Codecov](https://codecov.io/gh/librehunt/foxchain/branch/main/graph/badge.svg)](https://codecov.io/gh/librehunt/foxchain)
[![Docs](https://docs.rs/foxchain/badge.svg)](https://docs.rs/foxchain)
[![Crates.io](https://img.shields.io/crates/v/foxchain.svg)](https://crates.io/crates/foxchain)
[![crates.io](https://img.shields.io/crates/d/foxchain)](https://crates.io/crates/foxchain)

---

## Foxchain: Crates Overview

Foxchain will provide two Rust crates that work together:

- Identification crate (working name: foxchain-id)
  - Purpose: Given an input string (address, public key, or private key), detect the most likely blockchain(s) it belongs to.
  - Output: normalized representation (e.g., checksum-cased for EVM, bech32 HRP for Cosmos/BTC SegWit, SS58 for Substrate), a list of candidate chains, confidence scores, and reasoning.
  - Ambiguity: When an input could belong to multiple chains (e.g., EVM-style addresses valid across many EVM chains), it returns all candidates with confidence levels rather than guessing.

- Analysis crate (working name: foxchain-analysis)
  - Purpose: For an identified wallet (address/public key; optionally private key where explicitly allowed), retrieve on-chain data such as balances, transaction history, token transfers (ERC-20/721/1155), and chain-specific artifacts.
  - Output: strongly typed Rust structs suitable for library use, with optional JSON serialization for downstream tooling.
  - Backends: Designed to plug into explorer APIs (Etherscan-like), full-node RPC providers (e.g., Alchemy/Infura/QuickNode), and chain-specific services (e.g., Solana/TON/Cosmos endpoints), configured via environment variables.

### Inputs recognized (non-exhaustive)
- Addresses: EVM (0x…), BTC (P2PKH/P2SH/bech32), LTC/DOGE, TRON (base58check), Solana (base58), Cosmos (bech32 HRPs), Polkadot/Substrate (SS58), TON, etc.
- Public keys: hex/base58/bech32 where applicable.
- Private keys (optional): Hex/WIF/SS58, etc. If enabled, they are used only locally for derivation; never logged or transmitted.

### Typical outputs
- For identification: chain candidates with confidence, normalized address, and parsing notes.
- For analysis: balances (native and tokens), transactions (with pagination and time range), token/NFT transfers where applicable, gas/fees, and basic labeling if available from providers.

### Configuration
- The analysis crate reads provider credentials from environment variables. Common patterns include: ETHERSCAN_API_KEY, ALCHEMY_API_KEY, INFURA_API_KEY, POLYGONSCAN_API_KEY, TRON_GRID_API_KEY, SOLANA_RPC_URL, etc. Only set what you need for the chains you use.
- Secrets are never printed by the library. Prefer per-project .env files or your secret manager.

### Integration via Foxchain
Foxchain (top-level) will expose both crates so downstream consumers can depend on a single entry point. A simple flow:

1) Run identification on an input to get candidate chains and normalized address.
2) Select a chain (or iterate candidates) and run analysis to pull balances/transactions.

### Minimal usage sketch (names illustrative)
```rust path=null start=null
use foxchain_id::identify;
use foxchain_analysis::{Client, Chain};

fn main() -> anyhow::Result<()> {
    let input = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
    let id = identify(input)?; // returns candidates + normalized form

    // Pick Ethereum if present
    if let Some(candidate) = id.candidates.iter().find(|c| c.chain == Chain::Ethereum) {
        let addr = &candidate.normalized;
        let client = Client::for_chain(Chain::Ethereum)?; // uses env-configured providers
        let summary = client.account_summary(addr)?; // balances, tx count, tokens, etc.
        println!("{:?}", summary);
    }
    Ok(())
}
```

### Security notes
- Prefer addresses/public keys. Private key handling is optional and opt-in; if enabled, keep it offline and ephemeral. Do not persist, print, or transmit keys.
- Respect third-party API rate limits and terms.

### Roadmap (initial)
- v0: robust format detection, multi-chain address normalization, EVM/BTC/Solana/Cosmos/Substrate coverage, explorer/RPC adapters, pagination.
- v0.x: richer token/NFT coverage, internal txs, labels/tags where available, CLI wrapper, basic HTTP service.

If crate names differ in your workspace, adjust references here after you finalize naming. This section is appended without altering existing README content.
