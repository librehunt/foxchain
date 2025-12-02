# Supported Chains and Formats

This document provides a comprehensive reference of all blockchain chains, address formats, and public key types supported by `foxchain-id`.

## Overview

`foxchain-id` currently supports **29 blockchain chains** across multiple ecosystems, **7 address format types**, and **3 public key encoding formats** with **2 key types**.

## Supported Chains (29 Total)

### EVM-Compatible Chains (10)

All EVM chains share the same address format (`0x` + 40 hex characters) and are identified together:

1. **Ethereum** - Main EVM chain
2. **Polygon** - Layer 2 scaling solution
3. **BSC** (Binance Smart Chain) - EVM-compatible chain
4. **Avalanche** - EVM-compatible C-Chain
5. **Arbitrum** - Layer 2 rollup
6. **Optimism** - Layer 2 rollup
7. **Base** - Layer 2 chain
8. **Fantom** - EVM-compatible chain
9. **Celo** - EVM-compatible chain
10. **Gnosis** - EVM-compatible chain

**Address Format**: `0x` followed by 40 hexadecimal characters  
**Normalization**: EIP-55 checksum validation and normalization  
**Example**: `0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045`

### Bitcoin Ecosystem (3)

1. **Bitcoin** - Original cryptocurrency
2. **Litecoin** - Bitcoin fork with faster blocks
3. **Dogecoin** - Meme coin based on Bitcoin

**Address Formats**:
- **P2PKH** (Legacy): Starts with `1` (Bitcoin), `L` (Litecoin), `D` (Dogecoin)
- **P2SH** (Script Hash): Starts with `3`
- **Bech32** (Native SegWit): Starts with `bc1`/`tb1` (Bitcoin), `ltc1`/`lt1` (Litecoin)

**Examples**:
- P2PKH: `1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa`
- P2SH: `3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy`
- Bech32: `bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4`

### Cosmos Ecosystem (10)

All Cosmos chains use Bech32 encoding with chain-specific HRP (Human Readable Part) prefixes:

1. **Cosmos Hub** - HRP: `cosmos`
2. **Osmosis** - HRP: `osmo`
3. **Juno** - HRP: `juno`
4. **Akash** - HRP: `akash`
5. **Stargaze** - HRP: `stars`
6. **Secret Network** - HRP: `secret`
7. **Terra** - HRP: `terra`
8. **Kava** - HRP: `kava`
9. **Regen** - HRP: `regen`
10. **Sentinel** - HRP: `sent`

**Address Format**: Bech32 with chain-specific HRP  
**Normalization**: Case-insensitive (standardized to lowercase)  
**Example**: `cosmos1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4`

### Substrate/Polkadot Ecosystem (3)

1. **Polkadot** - SS58 prefix: `0`
2. **Kusama** - SS58 prefix: `2`
3. **Substrate** (Generic) - Other SS58 prefixes

**Address Format**: SS58 encoding (Base58 with chain-specific prefixes)  
**Structure**: Prefix (1-2 bytes) + Account ID (32 bytes) + Checksum (1-2 bytes)  
**Example**: `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY`

### Other Chains (3)

1. **Solana** - Base58 encoding, 32-44 bytes (standard: 32 bytes)
2. **Tron** - Base58Check encoding, starts with `T`, version byte `0x41`
3. **Cardano** - Bech32 encoding with HRP (`addr`, `stake`, `addr_test`, `stake_test`)

**Examples**:
- Solana: `9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM`
- Tron: `TQn9Y2khEsLMWDmH5V1XvY8vJzKJqJqJqJ`
- Cardano: `addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3jcu5d8ps7zex2k2xt3uqxgjqnnjhl2zqwpg7h3vj6`

## Address Format Types (7)

### 1. EVM Address Format
- **Encoding**: Hexadecimal with `0x` prefix
- **Length**: 42 characters (0x + 40 hex chars)
- **Validation**: EIP-55 checksum validation
- **Normalization**: EIP-55 checksummed format
- **Chains**: All 10 EVM chains

### 2. Bitcoin P2PKH Format
- **Encoding**: Base58Check
- **Prefix**: Starts with `1` (Bitcoin), `L` (Litecoin), `D` (Dogecoin)
- **Structure**: Version byte + 20-byte hash + 4-byte checksum
- **Validation**: Base58Check checksum validation
- **Chains**: Bitcoin, Litecoin, Dogecoin

### 3. Bitcoin P2SH Format
- **Encoding**: Base58Check
- **Prefix**: Starts with `3`
- **Structure**: Version byte + 20-byte hash + 4-byte checksum
- **Validation**: Base58Check checksum validation
- **Chains**: Bitcoin, Litecoin, Dogecoin

### 4. Bitcoin Bech32 Format
- **Encoding**: Bech32
- **Prefix**: `bc1`/`tb1` (Bitcoin), `ltc1`/`lt1` (Litecoin)
- **Structure**: HRP + 5-bit groups + checksum
- **Validation**: Bech32 checksum validation
- **Chains**: Bitcoin, Litecoin

### 5. Solana Address Format
- **Encoding**: Base58
- **Length**: 32-44 bytes when decoded (standard: 32 bytes)
- **Validation**: Base58 decoding + length validation
- **Chains**: Solana

### 6. Tron Address Format
- **Encoding**: Base58Check
- **Prefix**: Starts with `T`
- **Structure**: Version byte (0x41) + 20-byte address + 4-byte checksum
- **Length**: 25 bytes when decoded
- **Validation**: Base58Check checksum validation
- **Chains**: Tron

### 7. Cosmos/Cardano Bech32 Format
- **Encoding**: Bech32
- **Structure**: HRP + 5-bit groups + checksum
- **Validation**: Bech32 checksum validation
- **Normalization**: Case-insensitive (lowercase)
- **Chains**: All Cosmos chains (10), Cardano

### 8. Substrate SS58 Format
- **Encoding**: SS58 (Base58 variant)
- **Structure**: Prefix (1-2 bytes) + Account ID (32 bytes) + Checksum (1-2 bytes)
- **Validation**: SS58 checksum validation (Blake2b)
- **Chains**: Polkadot, Kusama, Generic Substrate

## Public Key Support

### Encoding Formats (3)

1. **Hex** - Hexadecimal encoding (with optional `0x` prefix)
2. **Base58** - Base58 encoding
3. **Bech32** - Bech32 encoding

### Key Types (2)

#### 1. secp256k1
- **Uncompressed**: 65 bytes (prefix `0x04` + 64 bytes)
- **Compressed**: 33 bytes (prefix `0x02` or `0x03` + 32 bytes)
- **Used by**: Bitcoin, EVM chains
- **Address Derivation**: 
  - ✅ EVM addresses (all 10 chains)
  - ✅ Bitcoin addresses (P2PKH)

#### 2. Ed25519
- **Length**: 32 bytes (no specific prefix)
- **Used by**: Solana, Cosmos chains
- **Address Derivation**:
  - ✅ Solana addresses
  - ✅ Cosmos Hub addresses

## Quick Reference Tables

### Chain → Format Mapping

| Chain | Address Format | Encoding |
|-------|---------------|----------|
| Ethereum | EVM | Hex (EIP-55) |
| Polygon | EVM | Hex (EIP-55) |
| BSC | EVM | Hex (EIP-55) |
| Avalanche | EVM | Hex (EIP-55) |
| Arbitrum | EVM | Hex (EIP-55) |
| Optimism | EVM | Hex (EIP-55) |
| Base | EVM | Hex (EIP-55) |
| Fantom | EVM | Hex (EIP-55) |
| Celo | EVM | Hex (EIP-55) |
| Gnosis | EVM | Hex (EIP-55) |
| Bitcoin | P2PKH/P2SH/Bech32 | Base58Check/Bech32 |
| Litecoin | P2PKH/P2SH/Bech32 | Base58Check/Bech32 |
| Dogecoin | P2PKH/P2SH | Base58Check |
| Solana | Base58 | Base58 |
| Tron | Base58Check | Base58Check |
| Cosmos Hub | Bech32 | Bech32 |
| Osmosis | Bech32 | Bech32 |
| Juno | Bech32 | Bech32 |
| Akash | Bech32 | Bech32 |
| Stargaze | Bech32 | Bech32 |
| Secret Network | Bech32 | Bech32 |
| Terra | Bech32 | Bech32 |
| Kava | Bech32 | Bech32 |
| Regen | Bech32 | Bech32 |
| Sentinel | Bech32 | Bech32 |
| Polkadot | SS58 | SS58 |
| Kusama | SS58 | SS58 |
| Substrate | SS58 | SS58 |
| Cardano | Bech32 | Bech32 |

### Format → Chains Mapping

| Format | Chains |
|--------|--------|
| EVM (Hex) | Ethereum, Polygon, BSC, Avalanche, Arbitrum, Optimism, Base, Fantom, Celo, Gnosis (10 chains) |
| Bitcoin P2PKH | Bitcoin, Litecoin, Dogecoin (3 chains) |
| Bitcoin P2SH | Bitcoin, Litecoin, Dogecoin (3 chains) |
| Bitcoin Bech32 | Bitcoin, Litecoin (2 chains) |
| Solana Base58 | Solana (1 chain) |
| Tron Base58Check | Tron (1 chain) |
| Cosmos Bech32 | Cosmos Hub, Osmosis, Juno, Akash, Stargaze, Secret Network, Terra, Kava, Regen, Sentinel (10 chains) |
| Substrate SS58 | Polkadot, Kusama, Generic Substrate (3 chains) |
| Cardano Bech32 | Cardano (1 chain) |

### Public Key Type → Supported Chains

| Key Type | Format | Address Derivation |
|----------|--------|-------------------|
| secp256k1 (uncompressed) | Hex, Base58, Bech32 | EVM (10 chains), Bitcoin |
| secp256k1 (compressed) | Hex, Base58, Bech32 | EVM (10 chains), Bitcoin |
| Ed25519 | Hex, Base58, Bech32 | Solana, Cosmos Hub |

## Address Derivation Capabilities

The library can derive addresses from public keys for the following chains:

1. **EVM Chains** (10 chains) - From secp256k1 public keys
   - Process: Decompress (if compressed) → Keccak-256 hash → Last 20 bytes → EIP-55 checksum

2. **Bitcoin** - From secp256k1 public keys
   - Process: Decompress (if compressed) → Hash160 → P2PKH address

3. **Solana** - From Ed25519 public keys
   - Process: Base58 encode the 32-byte public key

4. **Cosmos Hub** - From Ed25519 public keys
   - Process: SHA-256 hash → First 20 bytes → Bech32 encode with `cosmos` HRP

## Summary Statistics

- **Total Chains**: 29
- **EVM Chains**: 10
- **Bitcoin Ecosystem**: 3
- **Cosmos Ecosystem**: 10
- **Substrate Ecosystem**: 3
- **Other Chains**: 3 (Solana, Tron, Cardano)
- **Address Format Types**: 7
- **Public Key Encoding Formats**: 3 (Hex, Base58, Bech32)
- **Public Key Types**: 2 (secp256k1, Ed25519)
- **Address Derivation**: 4 chains (EVM, Bitcoin, Solana, Cosmos Hub)

## See Also

- [Usage Examples](examples.md) - Code examples showing how to use the library
- [EVM Addresses](evm-addresses.md) - Detailed EVM address documentation
- [Bitcoin Addresses](bitcoin-addresses.md) - Detailed Bitcoin address documentation
- [Cosmos Addresses](cosmos-addresses.md) - Detailed Cosmos address documentation
- [Substrate Addresses](substrate-addresses.md) - Detailed Substrate address documentation