//! Input analysis module
//!
//! This module provides functionality to analyze input strings and extract
//! characteristics for use in the detection pipeline.

pub mod characteristics;
pub mod signature;

pub use characteristics::{extract_characteristics, EntropyClass, InputCharacteristics};
pub use signature::CategorySignature;
