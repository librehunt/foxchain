//! Substrate ecosystem chain metadata definitions

use crate::registry::{
    AddressMetadata, ChainMetadata, CharSet, ChecksumType, EncodingType, Network,
    PublicKeyMetadata, PublicKeyType,
};
use crate::Chain;

/// Get all Substrate ecosystem chain metadata
pub fn substrate_metadata() -> Vec<ChainMetadata> {
    vec![
        ChainMetadata {
            id: Chain::Polkadot,
            name: "Polkadot".to_string(),
            address_formats: vec![AddressMetadata {
                encoding: EncodingType::SS58,
                char_set: Some(CharSet::Base58),
                exact_length: None,
                length_range: Some((35, 48)), // SS58 addresses vary in length
                prefixes: vec![],
                hrps: vec![],
                version_bytes: vec![0], // SS58 prefix 0 = Polkadot
                checksum: Some(ChecksumType::SS58),
                network: Some(Network::Mainnet),
            }],
            public_key_formats: vec![
                PublicKeyMetadata {
                    encoding: EncodingType::Hex,
                    char_set: Some(CharSet::Hex),
                    exact_length: Some(64), // 32 bytes = 64 hex chars
                    length_range: None,
                    prefixes: vec!["0x".to_string()],
                    hrps: vec![],
                    key_type: PublicKeyType::Ed25519,
                    checksum: None,
                },
                PublicKeyMetadata {
                    encoding: EncodingType::Hex,
                    char_set: Some(CharSet::Hex),
                    exact_length: Some(64),
                    length_range: None,
                    prefixes: vec!["0x".to_string()],
                    hrps: vec![],
                    key_type: PublicKeyType::Sr25519,
                    checksum: None,
                },
                PublicKeyMetadata {
                    encoding: EncodingType::Hex,
                    char_set: Some(CharSet::Hex),
                    exact_length: None,
                    length_range: Some((66, 130)), // secp256k1
                    prefixes: vec!["0x".to_string()],
                    hrps: vec![],
                    key_type: PublicKeyType::Secp256k1,
                    checksum: None,
                },
            ],
        },
        ChainMetadata {
            id: Chain::Kusama,
            name: "Kusama".to_string(),
            address_formats: vec![AddressMetadata {
                encoding: EncodingType::SS58,
                char_set: Some(CharSet::Base58),
                exact_length: None,
                length_range: Some((35, 48)),
                prefixes: vec![],
                hrps: vec![],
                version_bytes: vec![2], // SS58 prefix 2 = Kusama
                checksum: Some(ChecksumType::SS58),
                network: Some(Network::Mainnet),
            }],
            public_key_formats: vec![
                PublicKeyMetadata {
                    encoding: EncodingType::Hex,
                    char_set: Some(CharSet::Hex),
                    exact_length: Some(64),
                    length_range: None,
                    prefixes: vec!["0x".to_string()],
                    hrps: vec![],
                    key_type: PublicKeyType::Ed25519,
                    checksum: None,
                },
                PublicKeyMetadata {
                    encoding: EncodingType::Hex,
                    char_set: Some(CharSet::Hex),
                    exact_length: Some(64),
                    length_range: None,
                    prefixes: vec!["0x".to_string()],
                    hrps: vec![],
                    key_type: PublicKeyType::Sr25519,
                    checksum: None,
                },
                PublicKeyMetadata {
                    encoding: EncodingType::Hex,
                    char_set: Some(CharSet::Hex),
                    exact_length: None,
                    length_range: Some((66, 130)),
                    prefixes: vec!["0x".to_string()],
                    hrps: vec![],
                    key_type: PublicKeyType::Secp256k1,
                    checksum: None,
                },
            ],
        },
        ChainMetadata {
            id: Chain::Substrate,
            name: "Substrate".to_string(),
            address_formats: vec![AddressMetadata {
                encoding: EncodingType::SS58,
                char_set: Some(CharSet::Base58),
                exact_length: None,
                length_range: Some((35, 48)),
                prefixes: vec![],
                hrps: vec![],
                version_bytes: vec![42], // SS58 prefix 42 = Generic Substrate
                checksum: Some(ChecksumType::SS58),
                network: Some(Network::Mainnet),
            }],
            public_key_formats: vec![
                PublicKeyMetadata {
                    encoding: EncodingType::Hex,
                    char_set: Some(CharSet::Hex),
                    exact_length: Some(64),
                    length_range: None,
                    prefixes: vec!["0x".to_string()],
                    hrps: vec![],
                    key_type: PublicKeyType::Ed25519,
                    checksum: None,
                },
                PublicKeyMetadata {
                    encoding: EncodingType::Hex,
                    char_set: Some(CharSet::Hex),
                    exact_length: Some(64),
                    length_range: None,
                    prefixes: vec!["0x".to_string()],
                    hrps: vec![],
                    key_type: PublicKeyType::Sr25519,
                    checksum: None,
                },
                PublicKeyMetadata {
                    encoding: EncodingType::Hex,
                    char_set: Some(CharSet::Hex),
                    exact_length: None,
                    length_range: Some((66, 130)),
                    prefixes: vec!["0x".to_string()],
                    hrps: vec![],
                    key_type: PublicKeyType::Secp256k1,
                    checksum: None,
                },
            ],
        },
    ]
}
