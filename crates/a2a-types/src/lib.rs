//! A2A Protocol type definitions.

pub mod error;

#[allow(clippy::derivable_impls, clippy::clone_on_copy, clippy::large_enum_variant)]
mod generated {
    // Re-export our error module so generated code can find ConversionError
    pub use super::error;

    // Include generated types
    include!(concat!(env!("OUT_DIR"), "/generated_types.rs"));
}

// Re-export all generated types at the crate root
pub use generated::*;
