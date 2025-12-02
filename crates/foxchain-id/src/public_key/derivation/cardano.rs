//! Cardano address derivation from Ed25519 public keys

use crate::shared::encoding::bech32 as bech32_encoding;
use crate::{Chain, Error};
use bech32::Variant;

/// Compute SHA3-256 hash
fn sha3_256(data: &[u8]) -> [u8; 32] {
    use sha3::{Digest, Sha3_256};
    Sha3_256::digest(data).into()
}

/// Cardano address type
#[derive(Debug, Clone, Copy, PartialEq)]
enum CardanoAddressType {
    /// Payment address (type 0)
    Payment,
    /// Stake address (type 14)
    Stake,
}

/// Network identifier
#[derive(Debug, Clone, Copy, PartialEq)]
enum Network {
    /// Mainnet
    Mainnet,
    /// Testnet
    Testnet,
}

/// Derive Cardano address from Ed25519 public key
///
/// Process:
/// 1. Take Ed25519 public key (32 bytes)
/// 2. Compute SHA3-256 hash
/// 3. Take first 28 bytes
/// 4. Create header byte based on address type and network
/// 5. Encode as Bech32 with appropriate HRP
///
/// Returns both payment and stake addresses for mainnet and testnet
pub fn derive_cardano_address(public_key: &[u8]) -> Result<Vec<(Chain, String)>, Error> {
    if public_key.len() != 32 {
        return Ok(Vec::new());
    }

    // Compute SHA3-256 hash
    let hash = sha3_256(public_key);

    // Take first 28 bytes
    let payload = &hash[..28];

    let mut addresses = Vec::new();

    // Derive addresses for all combinations:
    // - Payment mainnet (addr)
    // - Payment testnet (addr_test)
    // - Stake mainnet (stake)
    // - Stake testnet (stake_test)

    // Payment mainnet: header = 0x00 (type 0, mainnet)
    let payment_mainnet =
        create_cardano_address(payload, CardanoAddressType::Payment, Network::Mainnet)?;
    addresses.push((Chain::Cardano, payment_mainnet));

    // Payment testnet: header = 0x10 (type 0, testnet)
    let payment_testnet =
        create_cardano_address(payload, CardanoAddressType::Payment, Network::Testnet)?;
    addresses.push((Chain::Cardano, payment_testnet));

    // Stake mainnet: header = 0xE0 (type 14, mainnet)
    let stake_mainnet =
        create_cardano_address(payload, CardanoAddressType::Stake, Network::Mainnet)?;
    addresses.push((Chain::Cardano, stake_mainnet));

    // Stake testnet: header = 0xF0 (type 14, testnet)
    let stake_testnet =
        create_cardano_address(payload, CardanoAddressType::Stake, Network::Testnet)?;
    addresses.push((Chain::Cardano, stake_testnet));

    Ok(addresses)
}

/// Create a Cardano address with given payload, type, and network
fn create_cardano_address(
    payload: &[u8],
    addr_type: CardanoAddressType,
    network: Network,
) -> Result<String, Error> {
    // Determine header byte based on type and network
    // Type 0 (Payment): mainnet = 0x00, testnet = 0x10
    // Type 14 (Stake): mainnet = 0xE0, testnet = 0xF0
    let header = match (addr_type, network) {
        (CardanoAddressType::Payment, Network::Mainnet) => 0x00,
        (CardanoAddressType::Payment, Network::Testnet) => 0x10,
        (CardanoAddressType::Stake, Network::Mainnet) => 0xE0,
        (CardanoAddressType::Stake, Network::Testnet) => 0xF0,
    };

    // Determine HRP based on type and network
    let hrp = match (addr_type, network) {
        (CardanoAddressType::Payment, Network::Mainnet) => "addr",
        (CardanoAddressType::Payment, Network::Testnet) => "addr_test",
        (CardanoAddressType::Stake, Network::Mainnet) => "stake",
        (CardanoAddressType::Stake, Network::Testnet) => "stake_test",
    };

    // Combine header + payload (1 + 28 = 29 bytes)
    let address_bytes = [&[header], payload].concat();

    // Convert to 5-bit groups for Bech32 encoding
    let data_u5 = bech32_encoding::convert_bits(&address_bytes, 8, 5, true)
        .map_err(|e| Error::InvalidInput(format!("Bech32 conversion error: {}", e)))?;
    let data_u5_vec: Vec<bech32::u5> = bech32_encoding::bytes_to_u5(&data_u5);

    // Encode as Bech32
    let address = bech32_encoding::encode(hrp, &data_u5_vec, Variant::Bech32)
        .map_err(|e| Error::InvalidInput(format!("Bech32 encoding error: {}", e)))?;

    Ok(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_cardano_address() {
        // Test with Ed25519 public key (32 bytes)
        let key_bytes = vec![0u8; 32];
        let result = derive_cardano_address(&key_bytes).unwrap();

        // Should return 4 addresses (payment mainnet, payment testnet, stake mainnet, stake testnet)
        assert_eq!(result.len(), 4, "Should return 4 Cardano addresses");

        // Verify all addresses are for Cardano chain
        for (chain, _) in &result {
            assert_eq!(*chain, Chain::Cardano);
        }

        // Verify addresses have correct HRPs
        let hrps: Vec<&str> = result
            .iter()
            .map(|(_, addr)| {
                if addr.starts_with("addr1") {
                    "addr"
                } else if addr.starts_with("addr_test1") {
                    "addr_test"
                } else if addr.starts_with("stake1") {
                    "stake"
                } else if addr.starts_with("stake_test1") {
                    "stake_test"
                } else {
                    "unknown"
                }
            })
            .collect();

        assert!(
            hrps.contains(&"addr"),
            "Should have payment mainnet address"
        );
        assert!(
            hrps.contains(&"addr_test"),
            "Should have payment testnet address"
        );
        assert!(hrps.contains(&"stake"), "Should have stake mainnet address");
        assert!(
            hrps.contains(&"stake_test"),
            "Should have stake testnet address"
        );
    }

    #[test]
    fn test_derive_cardano_address_invalid_length() {
        // Test with invalid length (not 32 bytes)
        let key_bytes = vec![0u8; 31];
        let result = derive_cardano_address(&key_bytes).unwrap();
        assert!(result.is_empty(), "Should return empty for invalid length");
    }

    #[test]
    fn test_derive_cardano_address_valid_key() {
        // Test with a valid Ed25519 key
        let key_bytes = vec![
            0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87,
            0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b,
            0x16, 0xf8, 0x17, 0x98,
        ];
        let result = derive_cardano_address(&key_bytes).unwrap();
        assert_eq!(result.len(), 4);

        // Verify all addresses are valid Bech32
        for (_, address) in &result {
            assert!(
                address.starts_with("addr1")
                    || address.starts_with("addr_test1")
                    || address.starts_with("stake1")
                    || address.starts_with("stake_test1"),
                "Address should have correct prefix: {}",
                address
            );
        }
    }

    #[test]
    fn test_sha3_256() {
        // Test SHA3-256 hash function
        let data = b"hello world";
        let hash = sha3_256(data);
        assert_eq!(hash.len(), 32);
        // Verify it's deterministic
        let hash2 = sha3_256(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_derive_cardano_address_empty_key() {
        // Test with empty key (0 bytes)
        let key_bytes = vec![];
        let result = derive_cardano_address(&key_bytes).unwrap();
        assert!(result.is_empty(), "Should return empty for empty key");
    }

    #[test]
    fn test_derive_cardano_address_33_bytes() {
        // Test with key that's too long (33 bytes instead of 32)
        let key_bytes = vec![0u8; 33];
        let result = derive_cardano_address(&key_bytes).unwrap();
        assert!(result.is_empty(), "Should return empty for wrong length");
    }

    #[test]
    fn test_derive_cardano_address_all_address_types() {
        // Test that all 4 address types are generated correctly
        let key_bytes = vec![0u8; 32];
        let result = derive_cardano_address(&key_bytes).unwrap();
        assert_eq!(result.len(), 4);

        // Verify we have exactly one of each type
        let mut payment_mainnet = false;
        let mut payment_testnet = false;
        let mut stake_mainnet = false;
        let mut stake_testnet = false;

        for (_, address) in &result {
            if address.starts_with("addr1") {
                payment_mainnet = true;
            } else if address.starts_with("addr_test1") {
                payment_testnet = true;
            } else if address.starts_with("stake1") {
                stake_mainnet = true;
            } else if address.starts_with("stake_test1") {
                stake_testnet = true;
            }
        }

        assert!(payment_mainnet, "Should have payment mainnet address");
        assert!(payment_testnet, "Should have payment testnet address");
        assert!(stake_mainnet, "Should have stake mainnet address");
        assert!(stake_testnet, "Should have stake testnet address");
    }
}
