//! Solana chain metadata definitions

use crate::registry::{
    AddressMetadata, ChainMetadata, CharSet, EncodingType, Network, PublicKeyMetadata,
    PublicKeyType,
};
use crate::Chain;

/// Get Solana chain metadata
pub fn solana_metadata() -> Vec<ChainMetadata> {
    vec![ChainMetadata {
        id: Chain::Solana,
        name: "Solana".to_string(),
        address_formats: vec![AddressMetadata {
            encoding: EncodingType::Base58,
            char_set: Some(CharSet::Base58),
            exact_length: None,
            length_range: Some((32, 44)), // 32-44 bytes when decoded
            prefixes: vec![],            // No prefix requirement
            hrps: vec![],
            version_bytes: vec![],
            checksum: None, // Base58 doesn't have built-in checksum
            network: Some(Network::Mainnet),
        }],
        public_key_formats: vec![PublicKeyMetadata {
            encoding: EncodingType::Base58,
            char_set: Some(CharSet::Base58),
            exact_length: None,
            length_range: Some((32, 44)),
            prefixes: vec![],
            hrps: vec![],
            key_type: PublicKeyType::Ed25519,
            checksum: None,
        }],
    }]
}
