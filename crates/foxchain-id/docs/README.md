# Blockchain Address Format Documentation

This directory contains detailed documentation about various blockchain address formats supported (or planned to be supported) by `foxchain-id`.

## Available Documentation

### Implemented Formats

- [EVM Addresses](evm-addresses.md) - Ethereum and EVM-compatible chains

### Planned Formats

- [Bitcoin Addresses](bitcoin-addresses.md) - Bitcoin, Litecoin, Dogecoin (P2PKH, P2SH, Bech32)
- [Solana Addresses](solana-addresses.md) - Solana public keys (base58)
- [Cosmos Addresses](cosmos-addresses.md) - Cosmos ecosystem (bech32 with HRP)
- [Substrate Addresses](substrate-addresses.md) - Polkadot, Kusama, and parachains (SS58)
- [Tron Addresses](tron-addresses.md) - Tron addresses (base58check)

## Format Documentation Structure

Each format documentation includes:

- **Format Specification**: Technical details about the address format
- **Encoding/Decoding**: How addresses are encoded and decoded
- **Validation Rules**: What makes an address valid
- **Examples**: Valid and invalid address examples
- **Implementation Details**: How `foxchain-id` handles the format
- **Multi-chain Considerations**: How the format relates to multiple chains

## Learning Resources

These documents serve as both user guides and educational resources. They explain not just how to use `foxchain-id`, but also how blockchain address formats work in general.
