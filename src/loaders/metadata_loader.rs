use crate::models::chain::MetadataIndex;
use serde_json;

/// Load the global metadata index
pub fn load_index() -> Result<MetadataIndex, String> {
    let json = include_str!("../../metadata/index.json");
    serde_json::from_str(json).map_err(|e| format!("Failed to parse index JSON: {}", e))
}
