//! Registry module for chain metadata and automatic grouping
//!
//! This module provides the registry system that precomputes category groups
//! at startup, automatically organizing chains by their format signatures.

pub mod build;
pub mod chain_converter;
pub mod groups;
pub mod metadata;

pub use build::Registry;
pub use metadata::{
    AddressMetadata, ChainMetadata, CharSet, ChecksumType, EncodingType, Network,
    PublicKeyMetadata, PublicKeyType,
};

