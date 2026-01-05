mod v1;
pub use v1::*;

// pbjson-generated serde implementations (proto3 JSON compliant)
// This file contains `impl Serialize` and `impl Deserialize` blocks
include!("v1.serde.rs");
