//! SS58 encoding utilities

use base58::FromBase58;

/// Decode SS58 string to bytes
pub fn decode(input: &str) -> Result<Vec<u8>, String> {
    input
        .from_base58()
        .map_err(|_| "Invalid SS58 encoding".to_string())
}
