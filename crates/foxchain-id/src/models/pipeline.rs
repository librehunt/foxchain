use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // Fields used for JSON deserialization, may not all be read
pub struct AddressPipeline {
    pub id: String,
    pub curve: String,
    pub steps: Vec<PipelineStep>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)] // Fields used for JSON deserialization, may not all be read
pub struct PipelineStep {
    #[serde(rename = "type")]
    pub step_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_byte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<usize>,
}

