// crates/a2a-transport-wasi/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WasiError {
    #[error("WASI HTTP error: {0}")]
    Http(String),

    #[error("Stream error: {0}")]
    Stream(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}
