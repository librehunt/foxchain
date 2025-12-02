//! Tron chain metadata definitions

use crate::registry::{
    AddressMetadata, ChainMetadata, CharSet, ChecksumType, EncodingType, Network,
    PublicKeyMetadata, PublicKeyType,
};
use crate::Chain;

/// Get Tron chain metadata
pub fn tron_metadata() -> Vec<ChainMetadata> {
    vec![ChainMetadata {
        id: Chain::Tron,
        name: "Tron".to_string(),
        address_formats: vec![AddressMetadata {
            encoding: EncodingType::Base58Check,
            char_set: Some(CharSet::Base58),
            exact_length: None,
            length_range: Some((34, 34)), // Tron addresses are typically 34 chars
            prefixes: vec!["T".to_string()], // Mainnet starts with T
            hrps: vec![],
            version_bytes: vec![0x41], // Tron mainnet version byte
            checksum: Some(ChecksumType::Base58Check),
            network: Some(Network::Mainnet),
        }],
        public_key_formats: vec![PublicKeyMetadata {
            encoding: EncodingType::Hex,
            char_set: Some(CharSet::Hex),
            exact_length: None,
            length_range: Some((66, 130)), // secp256k1
            prefixes: vec!["0x".to_string()],
            hrps: vec![],
            key_type: PublicKeyType::Secp256k1,
            checksum: None,
        }],
    }]
}
