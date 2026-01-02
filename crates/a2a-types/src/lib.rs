//! A2A Protocol type definitions.

pub mod error;

mod generated {
    // Re-export our error module so generated code can find ConversionError
    pub use super::error;

    // Include generated types
    include!(concat!(env!("OUT_DIR"), "/generated_types.rs"));
}

// Re-export all generated types at the crate root
pub use generated::*;
