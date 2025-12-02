//! Candidate groups for category signatures
//!
//! This module defines the type for grouping chains by their format signatures.

use crate::input::CategorySignature;
use crate::registry::ChainMetadata;
use std::collections::HashMap;

/// Groups of chains organized by category signature
pub type CandidateGroups = HashMap<CategorySignature, Vec<&'static ChainMetadata>>;

