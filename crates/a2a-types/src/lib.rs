//! A2A Protocol type definitions.

pub mod error;
mod generated;

// Re-export all generated types at the crate root
pub use generated::a2a::v1::*;

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

/// Protocol binding type for client/server communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binding {
    JsonRpc,
    Rest,
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binding::JsonRpc => write!(f, "JSONRPC"),
            Binding::Rest => write!(f, "HTTP+JSON"),
        }
    }
}

/// Protocol binding as declared in Agent Card (parses proto's string field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolBinding {
    #[serde(rename = "JSONRPC")]
    JsonRpc,
    #[serde(rename = "HTTP+JSON")]
    Rest,
    #[serde(rename = "GRPC")]
    Grpc,
}

impl From<ProtocolBinding> for Option<Binding> {
    fn from(pb: ProtocolBinding) -> Self {
        match pb {
            ProtocolBinding::JsonRpc => Some(Binding::JsonRpc),
            ProtocolBinding::Rest => Some(Binding::Rest),
            ProtocolBinding::Grpc => None, // Not supported
        }
    }
}

impl std::str::FromStr for ProtocolBinding {
    type Err = error::ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JSONRPC" => Ok(Self::JsonRpc),
            "HTTP+JSON" => Ok(Self::Rest),
            "GRPC" => Ok(Self::Grpc),
            _ => Err(error::ConversionError::new(format!(
                "unknown protocol binding: {s}"
            ))),
        }
    }
}
