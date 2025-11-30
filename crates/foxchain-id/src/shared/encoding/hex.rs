//! Hex encoding utilities

use hex;

/// Decode a hex string (with or without 0x prefix) to bytes
pub fn decode(input: &str) -> Result<Vec<u8>, String> {
    let hex_str = input.strip_prefix("0x").unwrap_or(input);
    hex::decode(hex_str).map_err(|e| format!("Invalid hex: {}", e))
}

/// Encode bytes to hex string with 0x prefix
pub fn encode(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(bytes))
}
