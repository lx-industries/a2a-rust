// crates/a2a-server/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Types(#[from] a2a_types::error::Error),

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("Store error: {0}")]
    Store(String),

    #[error("Handler error: {0}")]
    Handler(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Method not found: {0}")]
    MethodNotFound(String),

    #[error("Invalid params: {0}")]
    InvalidParams(String),
}

pub type Result<T> = std::result::Result<T, Error>;
