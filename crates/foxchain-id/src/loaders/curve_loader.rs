use crate::models::curve::CurveMetadata;
use serde_json;

/// Load curve metadata by ID
#[allow(dead_code)] // Reserved for future use
pub fn load_curve(id: &str) -> Result<CurveMetadata, String> {
    let json = match id {
        "secp256k1" => include_str!("../../metadata/curves/secp256k1.json"),
        "ed25519" => include_str!("../../metadata/curves/ed25519.json"),
        "sr25519" => include_str!("../../metadata/curves/sr25519.json"),
        _ => return Err(format!("Unknown curve: {}", id)),
    };
    serde_json::from_str(json).map_err(|e| format!("Failed to parse curve JSON for {}: {}", id, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_secp256k1() {
        let curve = load_curve("secp256k1");
        assert!(curve.is_ok());
        let curve = curve.unwrap();
        assert_eq!(curve.id, "secp256k1");
    }

    #[test]
    fn test_load_ed25519() {
        let curve = load_curve("ed25519");
        assert!(curve.is_ok());
        let curve = curve.unwrap();
        assert_eq!(curve.id, "ed25519");
    }

    #[test]
    fn test_load_sr25519() {
        let curve = load_curve("sr25519");
        assert!(curve.is_ok());
        let curve = curve.unwrap();
        assert_eq!(curve.id, "sr25519");
    }

    #[test]
    fn test_load_unknown_curve() {
        let result = load_curve("unknown");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown curve"));
    }
}
