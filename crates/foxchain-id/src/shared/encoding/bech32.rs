//! Bech32 encoding utilities

use bech32::{self, u5, Variant};

/// Decode a Bech32 string
pub fn decode(input: &str) -> Result<(String, Vec<u5>, Variant), String> {
    bech32::decode(input).map_err(|e| format!("Bech32 decode error: {}", e))
}

/// Encode data as Bech32
pub fn encode(hrp: &str, data: &[u5], variant: Variant) -> Result<String, String> {
    bech32::encode(hrp, data, variant).map_err(|e| format!("Bech32 encode error: {}", e))
}

/// Convert bits from one base to another
pub fn convert_bits(
    data: &[u8],
    from_bits: u32,
    to_bits: u32,
    pad: bool,
) -> Result<Vec<u8>, String> {
    bech32::convert_bits(data, from_bits, to_bits, pad)
        .map_err(|e| format!("Bit conversion error: {}", e))
}

/// Convert bytes to u5 vector for Bech32 encoding
pub fn bytes_to_u5(data: &[u8]) -> Vec<u5> {
    data.iter().map(|&b| u5::try_from_u8(b).unwrap()).collect()
}
