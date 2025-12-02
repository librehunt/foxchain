//! Unified detectors module
//!
//! This module provides unified detectors that use metadata instead of
//! hardcoded heuristics.

pub mod address;
pub mod public_key;

pub use address::{detect_address, DetectionResult};
pub use public_key::detect_public_key;

