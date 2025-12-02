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
    /// Encoding type detected
    pub encoding: Option<EncodingType>,
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
    let char_set = detect_char_set(input, encoding);
    
    // Extract prefixes
    let prefixes = extract_prefixes(input);
    
    // Calculate entropy class
    let entropy_class = calculate_entropy_class(input, encoding);
    
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

/// Detect encoding type and extract HRP if Bech32/Bech32m
fn detect_encoding(input: &str) -> (Option<EncodingType>, Option<String>) {
    // Try Bech32/Bech32m first (most specific)
    if let Some(hrp) = extract_bech32_hrp(input) {
        // Attempt decode to determine variant
        match bech32_encoding::decode(input) {
            Ok((_, _, variant)) => {
                match variant {
                    bech32::Variant::Bech32 => return (Some(EncodingType::Bech32), Some(hrp)),
                    bech32::Variant::Bech32m => return (Some(EncodingType::Bech32m), Some(hrp)),
                }
            }
            Err(_) => {}
        }
    }
    
    // Try hex encoding
    if input.starts_with("0x") {
        let hex_part = &input[2..];
        if hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
            return (Some(EncodingType::Hex), None);
        }
    } else if input.chars().all(|c| c.is_ascii_hexdigit()) && input.len() % 2 == 0 {
        return (Some(EncodingType::Hex), None);
    }
    
    // Try Base58 (no prefix, Base58 characters)
    if is_base58(input) {
        return (Some(EncodingType::Base58), None);
    }
    
    // Try Base58Check (Base58 with checksum validation happens later)
    if is_base58(input) {
        return (Some(EncodingType::Base58Check), None);
    }
    
    // Try SS58 (Substrate - Base58 with specific prefix)
    if is_base58(input) && (input.starts_with('1') || input.starts_with('3') || input.starts_with('5')) {
        return (Some(EncodingType::SS58), None);
    }
    
    (None, None)
}

/// Extract HRP from Bech32/Bech32m address
fn extract_bech32_hrp(input: &str) -> Option<String> {
    // Bech32 addresses have format: <hrp>1<data>
    if let Some(pos) = input.rfind('1') {
        if pos > 0 && pos < input.len() - 1 {
            let hrp = &input[..pos];
            // HRP must be lowercase and valid
            if hrp.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
                return Some(hrp.to_string());
            }
        }
    }
    None
}

/// Detect character set from input
fn detect_char_set(input: &str, encoding: Option<EncodingType>) -> CharSet {
    match encoding {
        Some(EncodingType::Hex) => CharSet::Hex,
        Some(EncodingType::Bech32) | Some(EncodingType::Bech32m) => CharSet::Base32,
        Some(EncodingType::Base58) | Some(EncodingType::Base58Check) | Some(EncodingType::SS58) => {
            CharSet::Base58
        }
        _ => {
            // Fallback detection
            if input.chars().all(|c| c.is_ascii_hexdigit()) {
                CharSet::Hex
            } else if is_base58(input) {
                CharSet::Base58
            } else if input.chars().all(|c| c.is_ascii_alphanumeric()) {
                CharSet::Alphanumeric
            } else {
                CharSet::Alphanumeric // Default fallback
            }
        }
    }
}

/// Check if string is valid Base58
fn is_base58(input: &str) -> bool {
    // Base58 excludes: 0, O, I, l
    input.chars().all(|c| {
        c.is_ascii_alphanumeric() && c != '0' && c != 'O' && c != 'I' && c != 'l'
    })
}

/// Extract prefixes from input
fn extract_prefixes(input: &str) -> Vec<String> {
    let mut prefixes = Vec::new();
    
    // Extract first 1-3 characters as potential prefixes
    if input.len() >= 1 {
        prefixes.push(input[..1].to_string());
    }
    if input.len() >= 2 {
        prefixes.push(input[..2].to_string());
    }
    if input.len() >= 3 {
        prefixes.push(input[..3].to_string());
    }
    
    // Also check for common patterns
    if input.starts_with("0x") {
        prefixes.push("0x".to_string());
    }
    
    prefixes
}

/// Calculate entropy class
fn calculate_entropy_class(input: &str, encoding: Option<EncodingType>) -> EntropyClass {
    match encoding {
        Some(EncodingType::Hex) if input.starts_with("0x") => EntropyClass::Low, // Highly structured
        Some(EncodingType::Bech32) | Some(EncodingType::Bech32m) => EntropyClass::Low, // HRP structure
        Some(EncodingType::Base58Check) | Some(EncodingType::SS58) => EntropyClass::Medium, // Some structure
        Some(EncodingType::Base58) => EntropyClass::Medium, // Some structure
        _ => EntropyClass::High, // Random-looking
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
        assert_eq!(chars.encoding, Some(EncodingType::Hex));
        assert_eq!(chars.entropy_class, EntropyClass::Low);
    }

    #[test]
    fn test_extract_bitcoin_bech32_characteristics() {
        let input = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let chars = extract_characteristics(input);
        
        assert_eq!(chars.char_set, CharSet::Base32);
        assert_eq!(chars.hrp, Some("bc1".to_string()));
        assert!(chars.encoding == Some(EncodingType::Bech32) || chars.encoding == Some(EncodingType::Bech32m));
    }

    #[test]
    fn test_extract_base58_characteristics() {
        let input = "1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2";
        let chars = extract_characteristics(input);
        
        assert_eq!(chars.char_set, CharSet::Base58);
        assert!(chars.prefixes.contains(&"1".to_string()));
    }
}

