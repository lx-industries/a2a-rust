// crates/a2a-transport-wasi/src/lib.rs
//! WASI HTTP transport implementation.

pub mod client;
pub mod error;
pub mod poll;

pub use client::WasiHttpClient;
pub use error::WasiError;
pub use poll::{PollableExt, WasiPollFuture};

/// WASI HTTP server using wasi:http/incoming-handler.
pub struct WasiHttpServer;

impl WasiHttpServer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WasiHttpServer {
    fn default() -> Self {
        Self::new()
    }
}
