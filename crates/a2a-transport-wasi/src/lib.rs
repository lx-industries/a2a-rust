// crates/a2a-transport-wasi/src/lib.rs
//! WASI HTTP transport implementation for the A2A protocol.
//!
//! This crate provides HTTP client and server implementations using the
//! `wasi:http` interface (WASIP2). It allows A2A protocol communication
//! from WebAssembly components.
//!
//! # Client Usage
//!
//! ```ignore
//! use a2a_transport::{HttpClient, HttpRequest, Method};
//! use a2a_transport_wasi::WasiHttpClient;
//! use bytes::Bytes;
//!
//! let client = WasiHttpClient::new();
//! let request = HttpRequest {
//!     method: Method::Post,
//!     url: "https://example.com/api".into(),
//!     headers: vec![("content-type".into(), "application/json".into())],
//!     body: Some(Bytes::from(r#"{"message": "hello"}"#)),
//! };
//!
//! // In an async context:
//! // let response = client.request(request).await?;
//! ```
//!
//! # Server Usage
//!
//! For server-side handling, use the `export_incoming_handler!` macro
//! to export your handler as a WASM component:
//!
//! ```ignore
//! use a2a_transport_wasi::export_incoming_handler;
//! use a2a_transport::{HttpRequest, HttpResponse};
//!
//! struct MyHandler;
//!
//! impl MyHandler {
//!     fn handle(request: HttpRequest) -> HttpResponse {
//!         HttpResponse {
//!             status: 200,
//!             headers: vec![],
//!             body: "OK".into(),
//!         }
//!     }
//! }
//!
//! export_incoming_handler!(MyHandler);
//! ```
//!
//! # Async Model
//!
//! This crate uses WASIP2's poll-based async model. The `WasiPollFuture`
//! type bridges WASI pollables to Rust's `Future` trait, allowing
//! integration with async runtimes that support single-threaded execution.

pub mod client;
pub mod error;
pub mod poll;
pub mod server;

pub use client::{WasiBodyStream, WasiHttpClient};
pub use error::WasiError;
pub use poll::{PollableExt, WasiPollFuture};
pub use server::{from_incoming_request, send_response};
