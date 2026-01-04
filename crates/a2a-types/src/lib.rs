//! A2A Protocol type definitions.

pub mod error;

#[allow(
    clippy::derivable_impls,
    clippy::clone_on_copy,
    clippy::large_enum_variant
)]
mod generated {
    // Re-export our error module so generated code can find ConversionError
    pub use super::error;

    // Include generated types
    include!(concat!(env!("OUT_DIR"), "/generated_types.rs"));
}

// Re-export all generated types at the crate root
pub use generated::*;

use serde::{Deserialize, Serialize};

/// Strongly-typed task identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskId(pub String);

impl TaskId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for TaskId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for TaskId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
