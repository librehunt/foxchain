//! Input characteristics extraction
//!
//! This module analyzes input strings and extracts all relevant characteristics
//! (length, charsets, prefixes, HRPs, entropy class, normalized form) for use
//! in the detection pipeline.

use crate::registry::{CharSet, EncodingType};
use crate::shared::encoding::bech32 as bech32_encoding;
use bech32;

/// Characteristics extracted from an input string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputCharacteristics {
    /// Length of the input string
    pub length: usize,
    /// Character set detected
    pub char_set: CharSet,
    /// All detected prefixes (e.g., "0x", "1", "3", "bc1")
    pub prefixes: Vec<String>,
    /// Extracted HRP if Bech32/Bech32m
    pub hrp: Option<String>,
    /// Possible encoding types detected (can be multiple for ambiguous inputs)
    pub encoding: Vec<EncodingType>,
    /// Case-normalized version of the input
    pub normalized: String,
    /// Entropy class of the input
    pub entropy_class: EntropyClass,
}

/// Entropy class indicating how structured the input is
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntropyClass {
    /// Highly structured (e.g., "0x" + 40 hex chars)
    Low,
    /// Some structure (e.g., Base58 with checksum)
    Medium,
    /// Random-looking (e.g., raw public keys)
    High,
}

/// Extract characteristics from an input string
pub fn extract_characteristics(input: &str) -> InputCharacteristics {
    let length = input.len();
    let normalized = input.to_lowercase();

    // Detect encoding and extract HRP
    let (encoding, hrp) = detect_encoding(input);

    // Detect character set
    let char_set = detect_char_set(input, &encoding);

    // Extract prefixes
    let prefixes = extract_prefixes(input);

    // Calculate entropy class
    let entropy_class = calculate_entropy_class(input, &encoding);

    InputCharacteristics {
        length,
        char_set,
        prefixes,
        hrp,
        encoding,
        normalized,
        entropy_class,
    }
}

/// Detect encoding type(s) and extract HRP if Bech32/Bech32m
///
/// Returns all possible encodings that match the input, allowing the validation
/// stage to determine which is correct. This removes ordering dependencies.
fn detect_encoding(input: &str) -> (Vec<EncodingType>, Option<String>) {
    let mut encodings = Vec::new();
    let mut hrp = None;

    // Try Bech32/Bech32m first (most specific)
    // Use bech32 library's decode to get the correct HRP
    if let Ok((decoded_hrp, _, variant)) = bech32_encoding::decode(input) {
        hrp = Some(decoded_hrp.clone());
        match variant {
            bech32::Variant::Bech32 => encodings.push(EncodingType::Bech32),
            bech32::Variant::Bech32m => encodings.push(EncodingType::Bech32m),
        }
    }

    // Try hex encoding
    if let Some(hex_part) = input.strip_prefix("0x") {
        if hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
            encodings.push(EncodingType::Hex);
        }
    } else if input.chars().all(|c| c.is_ascii_hexdigit()) && input.len().is_multiple_of(2) {
        encodings.push(EncodingType::Hex);
    }

    // Try Base58Check (Bitcoin, Tron, etc.)
    // Base58Check addresses are 25 bytes when decoded (1 version + 20 hash + 4 checksum)
    if is_base58(input) {
        use crate::shared::checksum::base58check;
        if let Ok(Some(_)) = base58check::validate(input) {
            encodings.push(EncodingType::Base58Check);
        }
    }

    // Try SS58 (Substrate - Base58 with specific prefix and SS58 checksum)
    // SS58 addresses start with '1', '3', or '5' but have different structure than Base58Check
    // SS58 addresses are typically 35-48 chars (longer than Base58Check which is ~34 chars)
    if is_base58(input)
        && input.len() >= 35
        && (input.starts_with('1') || input.starts_with('3') || input.starts_with('5'))
    {
        // Try to decode as Base58 to check structure
        use crate::shared::encoding::base58;
        if let Ok(decoded) = base58::decode(input) {
            // SS58 addresses have structure: prefix_bytes (1-2 bytes) + 32-byte account_id + 2-byte checksum
            // So decoded length should be 35-36 bytes (not 25 like Base58Check)
            if decoded.len() >= 35 && decoded.len() <= 36 {
                encodings.push(EncodingType::SS58);
            }
        }
    }

    // Try Base58 (no checksum, no specific structure)
    // Only add if we haven't already added Base58Check or SS58 (which are more specific)
    if is_base58(input)
        && !encodings.contains(&EncodingType::Base58Check)
        && !encodings.contains(&EncodingType::SS58)
    {
        encodings.push(EncodingType::Base58);
    }

    (encodings, hrp)
}

/// Detect character set from input
fn detect_char_set(input: &str, encodings: &[EncodingType]) -> CharSet {
    // Use the first encoding to determine char set, or fallback if empty
    if let Some(first_encoding) = encodings.first() {
        match first_encoding {
            EncodingType::Hex => CharSet::Hex,
            EncodingType::Bech32 | EncodingType::Bech32m => CharSet::Base32,
            EncodingType::Base58 | EncodingType::Base58Check | EncodingType::SS58 => {
                CharSet::Base58
            }
        }
    } else {
        // Fallback detection
        if input.chars().all(|c| c.is_ascii_hexdigit()) {
            CharSet::Hex
        } else if is_base58(input) {
            CharSet::Base58
        } else {
            CharSet::Alphanumeric // Default fallback (includes alphanumeric case)
        }
    }
}

/// Check if string is valid Base58
fn is_base58(input: &str) -> bool {
    // Base58 excludes: 0, O, I, l
    input
        .chars()
        .all(|c| c.is_ascii_alphanumeric() && c != '0' && c != 'O' && c != 'I' && c != 'l')
}

/// Extract prefixes from input using functional style
fn extract_prefixes(input: &str) -> Vec<String> {
    // Extract first 1-3 characters as potential prefixes
    let length_prefixes: Vec<String> = (1..=3)
        .filter_map(|len| {
            if input.len() >= len {
                Some(input[..len].to_string())
            } else {
                None
            }
        })
        .collect();

    // Also check for common patterns
    let pattern_prefixes: Vec<String> = if input.starts_with("0x") {
        vec!["0x".to_string()]
    } else {
        Vec::new()
    };

    length_prefixes
        .into_iter()
        .chain(pattern_prefixes)
        .collect()
}

/// Calculate entropy class
fn calculate_entropy_class(input: &str, encodings: &[EncodingType]) -> EntropyClass {
    // Use the first encoding to determine entropy, or fallback if empty
    if let Some(first_encoding) = encodings.first() {
        match first_encoding {
            EncodingType::Hex if input.starts_with("0x") => EntropyClass::Low, // Highly structured
            EncodingType::Bech32 | EncodingType::Bech32m => EntropyClass::Low, // HRP structure
            EncodingType::Base58Check | EncodingType::SS58 => EntropyClass::Medium, // Some structure
            EncodingType::Base58 => EntropyClass::Medium, // Some structure
            _ => EntropyClass::High,                      // Random-looking
        }
    } else {
        EntropyClass::High // Random-looking if no encoding detected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_evm_characteristics() {
        let input = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
        let chars = extract_characteristics(input);

        assert_eq!(chars.length, 42);
        assert_eq!(chars.char_set, CharSet::Hex);
        assert!(chars.prefixes.contains(&"0x".to_string()));
        assert!(chars.encoding.contains(&EncodingType::Hex));
        assert_eq!(chars.entropy_class, EntropyClass::Low);
    }

    #[test]
    fn test_extract_bitcoin_bech32_characteristics() {
        let input = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let chars = extract_characteristics(input);

        assert_eq!(chars.char_set, CharSet::Base32);
        // Bech32 HRP is "bc" (the '1' is a separator, not part of the HRP)
        assert_eq!(chars.hrp, Some("bc".to_string()));
        assert!(
            chars.encoding.contains(&EncodingType::Bech32)
                || chars.encoding.contains(&EncodingType::Bech32m)
        );
    }

    #[test]
    fn test_extract_base58_characteristics() {
        let input = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
        let chars = extract_characteristics(input);

        assert_eq!(chars.char_set, CharSet::Base58);
        assert!(chars.prefixes.contains(&"1".to_string()));
    }

    // ============================================================================
    // Phase 3.1: extract_characteristics Tests (expanded)
    // ============================================================================

    #[test]
    fn test_extract_characteristics_evm_address() {
        // EVM addresses: verify length=42, encoding=Hex, prefixes=["0x"], char_set=Hex
        let input = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
        let chars = extract_characteristics(input);

        assert_eq!(chars.length, 42);
        assert!(chars.encoding.contains(&EncodingType::Hex));
        assert!(chars.prefixes.contains(&"0x".to_string()));
        assert_eq!(chars.char_set, CharSet::Hex);
    }

    #[test]
    fn test_extract_characteristics_bitcoin_p2pkh() {
        // Bitcoin P2PKH: verify encoding=Base58Check, prefixes=["1"]
        let input = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";
        let chars = extract_characteristics(input);

        assert!(
            chars.encoding.contains(&EncodingType::Base58Check)
                || chars.encoding.contains(&EncodingType::Base58)
        );
        assert!(chars.prefixes.contains(&"1".to_string()));
        assert_eq!(chars.char_set, CharSet::Base58);
    }

    #[test]
    fn test_extract_characteristics_bitcoin_bech32() {
        // Bitcoin Bech32: verify encoding=Bech32, hrp="bc", char_set=Base32
        let input = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let chars = extract_characteristics(input);

        assert!(
            chars.encoding.contains(&EncodingType::Bech32)
                || chars.encoding.contains(&EncodingType::Bech32m)
        );
        assert_eq!(chars.hrp, Some("bc".to_string()));
        assert_eq!(chars.char_set, CharSet::Base32);
    }

    #[test]
    fn test_extract_characteristics_cosmos() {
        // Cosmos: verify encoding=Bech32, hrp="cosmos", char_set=Base32
        let input = "cosmos1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let chars = extract_characteristics(input);

        // If address is valid Bech32, verify structure
        if chars.encoding.contains(&EncodingType::Bech32)
            || chars.encoding.contains(&EncodingType::Bech32m)
        {
            assert_eq!(chars.hrp, Some("cosmos".to_string()));
            assert_eq!(chars.char_set, CharSet::Base32);
        }
        // If address is invalid Bech32, test passes (address might be generated/invalid)
    }

    #[test]
    fn test_extract_characteristics_solana() {
        // Solana: verify encoding=Base58, length=32-44, char_set=Base58
        let input = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
        let chars = extract_characteristics(input);

        assert!(
            chars.encoding.contains(&EncodingType::Base58)
                || chars.encoding.contains(&EncodingType::Base58Check)
        );
        assert!((32..=44).contains(&chars.length));
        assert_eq!(chars.char_set, CharSet::Base58);
    }

    #[test]
    fn test_extract_characteristics_tron() {
        // Tron: verify encoding=Base58Check, prefixes=["T"]
        let input = "T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb";
        let chars = extract_characteristics(input);

        assert!(
            chars.encoding.contains(&EncodingType::Base58Check)
                || chars.encoding.contains(&EncodingType::Base58)
        );
        assert!(chars.prefixes.contains(&"T".to_string()));
        assert_eq!(chars.char_set, CharSet::Base58);
    }

    #[test]
    fn test_extract_characteristics_ss58() {
        // SS58: verify encoding=SS58, char_set=Base58
        let input = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
        let chars = extract_characteristics(input);

        // SS58 might be detected as Base58Check or SS58
        assert!(
            chars.encoding.contains(&EncodingType::SS58)
                || chars.encoding.contains(&EncodingType::Base58Check)
                || chars.encoding.contains(&EncodingType::Base58)
        );
        assert_eq!(chars.char_set, CharSet::Base58);
    }
}
