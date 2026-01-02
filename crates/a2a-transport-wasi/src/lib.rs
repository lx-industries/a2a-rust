// crates/a2a-transport-wasi/src/lib.rs
//! WASI HTTP transport implementation.

pub mod client;
pub mod error;
pub mod poll;
pub mod server;

pub use client::{WasiBodyStream, WasiHttpClient};
pub use error::WasiError;
pub use poll::{PollableExt, WasiPollFuture};
pub use server::{from_incoming_request, send_response};
