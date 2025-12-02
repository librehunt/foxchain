//! Case normalization utilities

/// Normalize Bech32 string to lowercase (Bech32 is case-insensitive)
#[allow(dead_code)] // Reserved for future use
pub fn bech32_to_lowercase(input: &str) -> String {
    input.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bech32_to_lowercase() {
        assert_eq!(bech32_to_lowercase("COSMOS1ABC"), "cosmos1abc");
        assert_eq!(bech32_to_lowercase("cosmos1abc"), "cosmos1abc");
        assert_eq!(bech32_to_lowercase("CosMos1AbC"), "cosmos1abc");
    }
}
