//! Case normalization utilities

/// Normalize Bech32 string to lowercase (Bech32 is case-insensitive)
pub fn bech32_to_lowercase(input: &str) -> String {
    input.to_lowercase()
}
