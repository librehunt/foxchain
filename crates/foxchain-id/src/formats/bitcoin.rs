//! Bitcoin ecosystem chain metadata definitions
//!
//! This module defines metadata for Bitcoin, Litecoin, and Dogecoin.

use crate::registry::{
    AddressMetadata, ChainMetadata, CharSet, ChecksumType, EncodingType, Network,
    PublicKeyMetadata, PublicKeyType,
};
use crate::Chain;

/// Get all Bitcoin ecosystem chain metadata
pub fn bitcoin_metadata() -> Vec<ChainMetadata> {
    vec![
        ChainMetadata {
            id: Chain::Bitcoin,
            name: "Bitcoin".to_string(),
            address_formats: vec![
                // P2PKH (starts with 1)
                AddressMetadata {
                    encoding: EncodingType::Base58Check,
                    char_set: Some(CharSet::Base58),
                    exact_length: None,
                    length_range: Some((26, 35)),
                    prefixes: vec!["1".to_string()],
                    hrps: vec![],
                    version_bytes: vec![0x00], // Mainnet P2PKH
                    checksum: Some(ChecksumType::Base58Check),
                    network: Some(Network::Mainnet),
                },
                // P2SH (starts with 3)
                AddressMetadata {
                    encoding: EncodingType::Base58Check,
                    char_set: Some(CharSet::Base58),
                    exact_length: None,
                    length_range: Some((26, 35)),
                    prefixes: vec!["3".to_string()],
                    hrps: vec![],
                    version_bytes: vec![0x05], // Mainnet P2SH
                    checksum: Some(ChecksumType::Base58Check),
                    network: Some(Network::Mainnet),
                },
                // Bech32 (native SegWit)
                AddressMetadata {
                    encoding: EncodingType::Bech32,
                    char_set: Some(CharSet::Base32),
                    exact_length: None,
                    length_range: Some((14, 74)),
                    prefixes: vec![],
                    hrps: vec!["bc1".to_string()],
                    version_bytes: vec![],
                    checksum: Some(ChecksumType::Bech32),
                    network: Some(Network::Mainnet),
                },
            ],
            public_key_formats: vec![PublicKeyMetadata {
                encoding: EncodingType::Hex,
                char_set: Some(CharSet::Hex),
                exact_length: None,
                length_range: Some((66, 130)),
                prefixes: vec!["0x".to_string()],
                hrps: vec![],
                key_type: PublicKeyType::Secp256k1,
                checksum: None,
            }],
        },
        ChainMetadata {
            id: Chain::Litecoin,
            name: "Litecoin".to_string(),
            address_formats: vec![
                AddressMetadata {
                    encoding: EncodingType::Base58Check,
                    char_set: Some(CharSet::Base58),
                    exact_length: None,
                    length_range: Some((26, 35)),
                    prefixes: vec!["L".to_string(), "M".to_string()],
                    hrps: vec![],
                    version_bytes: vec![0x30], // Mainnet P2PKH
                    checksum: Some(ChecksumType::Base58Check),
                    network: Some(Network::Mainnet),
                },
                AddressMetadata {
                    encoding: EncodingType::Base58Check,
                    char_set: Some(CharSet::Base58),
                    exact_length: None,
                    length_range: Some((26, 35)),
                    prefixes: vec!["3".to_string()],
                    hrps: vec![],
                    version_bytes: vec![0x32], // Mainnet P2SH
                    checksum: Some(ChecksumType::Base58Check),
                    network: Some(Network::Mainnet),
                },
                AddressMetadata {
                    encoding: EncodingType::Bech32,
                    char_set: Some(CharSet::Base32),
                    exact_length: None,
                    length_range: Some((14, 74)),
                    prefixes: vec![],
                    hrps: vec!["ltc1".to_string(), "lt1".to_string()],
                    version_bytes: vec![],
                    checksum: Some(ChecksumType::Bech32),
                    network: Some(Network::Mainnet),
                },
            ],
            public_key_formats: vec![PublicKeyMetadata {
                encoding: EncodingType::Hex,
                char_set: Some(CharSet::Hex),
                exact_length: None,
                length_range: Some((66, 130)),
                prefixes: vec!["0x".to_string()],
                hrps: vec![],
                key_type: PublicKeyType::Secp256k1,
                checksum: None,
            }],
        },
        ChainMetadata {
            id: Chain::Dogecoin,
            name: "Dogecoin".to_string(),
            address_formats: vec![
                AddressMetadata {
                    encoding: EncodingType::Base58Check,
                    char_set: Some(CharSet::Base58),
                    exact_length: None,
                    length_range: Some((26, 35)),
                    prefixes: vec!["D".to_string()],
                    hrps: vec![],
                    version_bytes: vec![0x1e], // Mainnet P2PKH
                    checksum: Some(ChecksumType::Base58Check),
                    network: Some(Network::Mainnet),
                },
                AddressMetadata {
                    encoding: EncodingType::Base58Check,
                    char_set: Some(CharSet::Base58),
                    exact_length: None,
                    length_range: Some((26, 35)),
                    prefixes: vec!["3".to_string()],
                    hrps: vec![],
                    version_bytes: vec![0x16], // Mainnet P2SH
                    checksum: Some(ChecksumType::Base58Check),
                    network: Some(Network::Mainnet),
                },
            ],
            public_key_formats: vec![PublicKeyMetadata {
                encoding: EncodingType::Hex,
                char_set: Some(CharSet::Hex),
                exact_length: None,
                length_range: Some((66, 130)),
                prefixes: vec!["0x".to_string()],
                hrps: vec![],
                key_type: PublicKeyType::Secp256k1,
                checksum: None,
            }],
        },
    ]
}
