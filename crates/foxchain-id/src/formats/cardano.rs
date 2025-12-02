//! Cardano chain metadata definitions

use crate::registry::{
    AddressMetadata, ChainMetadata, CharSet, ChecksumType, EncodingType, Network,
    PublicKeyMetadata, PublicKeyType,
};
use crate::Chain;

/// Get Cardano chain metadata
pub fn cardano_metadata() -> Vec<ChainMetadata> {
    vec![ChainMetadata {
        id: Chain::Cardano,
        name: "Cardano".to_string(),
        address_formats: vec![
            // Payment addresses (mainnet)
            AddressMetadata {
                encoding: EncodingType::Bech32,
                char_set: Some(CharSet::Base32),
                exact_length: None,
                length_range: Some((20, 100)), // Cardano addresses vary in length
                prefixes: vec![],
                hrps: vec!["addr".to_string()],
                version_bytes: vec![],
                checksum: Some(ChecksumType::Bech32),
                network: Some(Network::Mainnet),
            },
            // Stake addresses (mainnet)
            AddressMetadata {
                encoding: EncodingType::Bech32,
                char_set: Some(CharSet::Base32),
                exact_length: None,
                length_range: Some((20, 100)),
                prefixes: vec![],
                hrps: vec!["stake".to_string()],
                version_bytes: vec![],
                checksum: Some(ChecksumType::Bech32),
                network: Some(Network::Mainnet),
            },
            // Payment addresses (testnet)
            AddressMetadata {
                encoding: EncodingType::Bech32,
                char_set: Some(CharSet::Base32),
                exact_length: None,
                length_range: Some((20, 100)),
                prefixes: vec![],
                hrps: vec!["addr_test".to_string()],
                version_bytes: vec![],
                checksum: Some(ChecksumType::Bech32),
                network: Some(Network::Testnet),
            },
            // Stake addresses (testnet)
            AddressMetadata {
                encoding: EncodingType::Bech32,
                char_set: Some(CharSet::Base32),
                exact_length: None,
                length_range: Some((20, 100)),
                prefixes: vec![],
                hrps: vec!["stake_test".to_string()],
                version_bytes: vec![],
                checksum: Some(ChecksumType::Bech32),
                network: Some(Network::Testnet),
            },
        ],
        public_key_formats: vec![PublicKeyMetadata {
            encoding: EncodingType::Hex,
            char_set: Some(CharSet::Hex),
            exact_length: Some(64), // 32 bytes = 64 hex chars (Ed25519)
            length_range: None,
            prefixes: vec!["0x".to_string()],
            hrps: vec![],
            key_type: PublicKeyType::Ed25519,
            checksum: None,
        }],
    }]
}
