// crates/a2a-transport/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: status {status}")]
    Http { status: u16, body: Option<String> },

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Timeout")]
    Timeout,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

pub type Result<T> = std::result::Result<T, Error>;
