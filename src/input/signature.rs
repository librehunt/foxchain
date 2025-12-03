//! Category signature system for automatic grouping
//!
//! This module provides the CategorySignature system that automatically groups
//! chains by shared format characteristics, eliminating the need for manually
//! maintained categories.

use crate::input::InputCharacteristics;
use crate::registry::{AddressMetadata, CharSet, EncodingType};

/// Signature used to group chains with similar format characteristics
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CategorySignature {
    /// Character set (if specific)
    pub char_set: Option<CharSet>,
    /// Minimum length
    pub min_len: usize,
    /// Maximum length
    pub max_len: usize,
    /// Whether HRP is required
    pub has_hrp: bool,
    /// Required prefixes (empty vec = no prefix requirement)
    pub prefixes: Vec<String>,
    /// Required HRP prefixes (empty vec = no HRP requirement)
    pub hrp_prefixes: Vec<String>,
    /// Encoding type (if specific)
    pub encoding_type: Option<EncodingType>,
}

impl CategorySignature {
    /// Derive signature from input characteristics
    #[allow(dead_code)] // Used in tests and may be useful for future code
    pub fn from(chars: &InputCharacteristics) -> Self {
        // Normalize prefixes: keep only meaningful standard prefixes
        // For EVM: "0x" (not "0" or "0xd")
        // For Bech32: HRP is handled separately via hrp_prefixes
        let normalized_prefixes: Vec<String> = {
            let mut prefixes = chars.prefixes.clone();
            prefixes.dedup();
            // Prefer standard prefixes: "0x", "bc1", "tb1", "ltc1"
            if prefixes.contains(&"0x".to_string()) {
                vec!["0x".to_string()]
            } else if let Some(bech32) = prefixes
                .iter()
                .find(|p| p.starts_with("bc1") || p.starts_with("tb1") || p.starts_with("ltc1"))
            {
                vec![bech32.clone()]
            } else if !prefixes.is_empty() {
                // Keep longest prefix as fallback
                let mut sorted = prefixes;
                sorted.sort_by_key(|b| std::cmp::Reverse(b.len()));
                vec![sorted[0].clone()]
            } else {
                vec![]
            }
        };

        CategorySignature {
            char_set: Some(chars.char_set),
            min_len: chars.length,
            max_len: chars.length,
            has_hrp: chars.hrp.is_some(),
            prefixes: normalized_prefixes,
            hrp_prefixes: chars
                .hrp
                .as_ref()
                .map(|h| vec![h.clone()])
                .unwrap_or_default(),
            encoding_type: chars.encoding.first().copied(), // Use first encoding for signature
        }
    }

    /// Derive signature from address metadata
    pub fn from_metadata(metadata: &AddressMetadata) -> Self {
        let (min_len, max_len) = if let Some(exact) = metadata.exact_length {
            (exact, exact)
        } else if let Some((min, max)) = metadata.length_range {
            (min, max)
        } else {
            // Default range if no length specified
            (0, usize::MAX)
        };

        CategorySignature {
            char_set: metadata.char_set,
            min_len,
            max_len,
            has_hrp: !metadata.hrps.is_empty(),
            prefixes: metadata.prefixes.clone(),
            hrp_prefixes: metadata.hrps.clone(),
            encoding_type: Some(metadata.encoding),
        }
    }

    /// Check if this signature matches input characteristics
    #[allow(dead_code)] // Reserved for future use
    pub fn matches(&self, chars: &InputCharacteristics) -> bool {
        // Check character set
        if let Some(ref char_set) = self.char_set {
            if chars.char_set != *char_set {
                return false;
            }
        }

        // Check length range
        if chars.length < self.min_len || chars.length > self.max_len {
            return false;
        }

        // Check HRP requirement
        if self.has_hrp && chars.hrp.is_none() {
            return false;
        }
        if !self.has_hrp && chars.hrp.is_some() {
            // If signature doesn't require HRP but input has one, still match
            // (HRP is optional in signature)
        }

        // Check prefixes
        if !self.prefixes.is_empty() && !self.prefixes.iter().any(|p| chars.prefixes.contains(p)) {
            return false;
        }

        // Check HRP prefixes
        if !self.hrp_prefixes.is_empty() {
            if let Some(ref hrp) = chars.hrp {
                if !self.hrp_prefixes.iter().any(|h| hrp.starts_with(h)) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check encoding type - match if any of the detected encodings matches
        if let Some(ref encoding) = self.encoding_type {
            if !chars.encoding.contains(encoding) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::extract_characteristics;
    use crate::registry::{AddressMetadata, ChecksumType, EncodingType, Network};

    #[test]
    fn test_signature_from_characteristics() {
        let input = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
        let chars = extract_characteristics(input);
        let sig = CategorySignature::from(&chars);

        assert_eq!(sig.char_set, Some(CharSet::Hex));
        assert_eq!(sig.min_len, 42);
        assert_eq!(sig.max_len, 42);
        assert!(sig.prefixes.contains(&"0x".to_string()));
        assert_eq!(sig.encoding_type, Some(EncodingType::Hex));
    }

    #[test]
    fn test_signature_from_metadata() {
        let metadata = AddressMetadata {
            encoding: EncodingType::Hex,
            char_set: Some(CharSet::Hex),
            exact_length: Some(42),
            length_range: None,
            prefixes: vec!["0x".to_string()],
            hrps: vec![],
            version_bytes: vec![],
            checksum: Some(ChecksumType::EIP55),
            network: Some(Network::Mainnet),
        };

        let sig = CategorySignature::from_metadata(&metadata);

        assert_eq!(sig.char_set, Some(CharSet::Hex));
        assert_eq!(sig.min_len, 42);
        assert_eq!(sig.max_len, 42);
        assert!(sig.prefixes.contains(&"0x".to_string()));
        assert_eq!(sig.encoding_type, Some(EncodingType::Hex));
    }

    #[test]
    fn test_signature_matches() {
        let input = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e";
        let chars = extract_characteristics(input);
        let sig = CategorySignature::from(&chars);

        assert!(sig.matches(&chars));
    }
}
