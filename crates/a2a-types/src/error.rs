//! Error types for the A2A protocol.

use std::borrow::Cow;
use thiserror::Error;

/// Errors that can occur when working with A2A types.
#[derive(Debug, Error)]
pub enum Error {
    /// JSON serialization or deserialization error.
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid message format or content.
    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    /// Invalid task state transition or value.
    #[error("Invalid task state: {0}")]
    InvalidTaskState(String),

    /// Invalid part type in a message.
    #[error("Invalid part type: {0}")]
    InvalidPartType(String),
}

/// A specialized Result type for A2A operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error from a `TryFrom` or `FromStr` implementation.
///
/// This type is used by typify-generated code for string conversion errors.
#[derive(Debug, Clone)]
pub struct ConversionError(Cow<'static, str>);

impl ConversionError {
    /// Create a new conversion error with the given message.
    pub fn new(msg: impl Into<Cow<'static, str>>) -> Self {
        Self(msg.into())
    }
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for ConversionError {}

impl From<&str> for ConversionError {
    fn from(value: &str) -> Self {
        Self(Cow::Owned(value.to_owned()))
    }
}

impl From<String> for ConversionError {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}
