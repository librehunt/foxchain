use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // Fields used for JSON deserialization, may not all be read
pub struct CurveMetadata {
    pub id: String,
    pub key_lengths: Vec<usize>,
    pub compression: bool,
    pub compatible_pipelines: Vec<String>,
}

