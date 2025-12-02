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
    serde_json::from_str(json)
        .map_err(|e| format!("Failed to parse curve JSON for {}: {}", id, e))
}

