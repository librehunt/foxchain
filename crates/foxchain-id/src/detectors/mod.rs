//! Unified detectors module
//!
//! This module provides unified detectors that use metadata instead of
//! hardcoded heuristics.

pub mod address;

pub use address::{detect_address, DetectionResult};

