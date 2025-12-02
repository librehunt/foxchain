//! Metadata structures for chain and format definitions
//!
//! This module defines the foundational metadata structures that drive the
//! entire detection pipeline. All format detection logic is declarative,
//! eliminating the need for hardcoded heuristics.

/// Metadata for a blockchain chain
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainMetadata {
    /// Chain identifier
    pub id: crate::Chain,
    /// Human-readable chain name
    pub name: String,
    /// All supported address formats for this chain
    pub address_formats: Vec<AddressMetadata>,
    /// All supported public key formats for this chain
    pub public_key_formats: Vec<PublicKeyMetadata>,
}

/// Metadata for an address format
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressMetadata {
    /// Encoding type used for this address format
    pub encoding: EncodingType,
    /// Character set used (if specific)
    pub char_set: Option<CharSet>,
    /// Exact length requirement (if any)
    pub exact_length: Option<usize>,
    /// Length range requirement (if any)
    pub length_range: Option<(usize, usize)>,
    /// Required prefixes (empty vec = no prefix requirement)
    pub prefixes: Vec<String>,
    /// Required HRPs for Bech32/Bech32m (empty vec = no HRP requirement)
    pub hrps: Vec<String>,
    /// Version bytes for Base58Check formats (empty vec = no version requirement)
    pub version_bytes: Vec<u8>,
    /// Checksum type (if any)
    pub checksum: Option<ChecksumType>,
    /// Network (mainnet/testnet)
    pub network: Option<Network>,
}

/// Metadata for a public key format
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKeyMetadata {
    /// Encoding type used for this public key format
    pub encoding: EncodingType,
    /// Character set used (if specific)
    pub char_set: Option<CharSet>,
    /// Exact length requirement (if any)
    pub exact_length: Option<usize>,
    /// Length range requirement (if any)
    pub length_range: Option<(usize, usize)>,
    /// Required prefixes (empty vec = no prefix requirement)
    pub prefixes: Vec<String>,
    /// Required HRPs for Bech32/Bech32m (empty vec = no HRP requirement)
    pub hrps: Vec<String>,
    /// Public key type (secp256k1, Ed25519, sr25519)
    pub key_type: PublicKeyType,
    /// Checksum type (if any)
    pub checksum: Option<ChecksumType>,
}

/// Encoding type for addresses and public keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncodingType {
    /// Hexadecimal encoding (with or without 0x prefix)
    Hex,
    /// Base58 encoding (no checksum)
    Base58,
    /// Base58Check encoding (with checksum)
    Base58Check,
    /// Bech32 encoding
    Bech32,
    /// Bech32m encoding
    Bech32m,
    /// SS58 encoding (Substrate)
    SS58,
}

/// Character set used in the encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharSet {
    /// Hexadecimal characters (0-9, a-f, A-F)
    Hex,
    /// Base58 characters (alphanumeric excluding 0, O, I, l)
    Base58,
    /// Base32 characters (used in Bech32)
    Base32,
    /// Alphanumeric characters
    Alphanumeric,
}

/// Checksum type used for validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChecksumType {
    /// EIP-55 checksum (Ethereum)
    EIP55,
    /// Base58Check checksum (Bitcoin, etc.)
    Base58Check,
    /// Bech32 checksum
    Bech32,
    /// Bech32m checksum
    Bech32m,
    /// SS58 checksum (Substrate)
    SS58,
}

/// Network type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Network {
    /// Mainnet
    Mainnet,
    /// Testnet
    Testnet,
}

/// Public key type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PublicKeyType {
    /// secp256k1 public key (33 or 65 bytes)
    Secp256k1,
    /// Ed25519 public key (32 bytes)
    Ed25519,
    /// sr25519 public key (32 bytes)
    Sr25519,
}

