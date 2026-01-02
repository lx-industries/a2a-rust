// crates/a2a-transport/src/lib.rs
//! A2A transport traits and HTTP types.

pub mod error;
pub mod types;

pub use error::{Error, Result};
pub use types::{HttpRequest, HttpResponse, Method};

use bytes::Bytes;
use futures_core::Stream;
use std::future::Future;

/// HTTP client trait for making outgoing requests.
pub trait HttpClient: Send + Sync {
    /// Error type for this client.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Send a request and receive a complete response.
    fn request(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = std::result::Result<HttpResponse, Self::Error>> + Send;

    /// Send a request and receive a streaming response.
    fn request_stream(
        &self,
        request: HttpRequest,
    ) -> impl Future<
        Output = std::result::Result<
            impl Stream<Item = std::result::Result<Bytes, Self::Error>> + Send,
            Self::Error,
        >,
    > + Send;
}

/// HTTP server trait for handling incoming requests.
pub trait HttpServer: Send + Sync {
    /// Error type for this server.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Start serving requests, calling the handler for each one.
    fn serve<H, F>(
        &self,
        handler: H,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send
    where
        H: Fn(HttpRequest) -> F + Send + Sync + 'static,
        F: Future<Output = HttpResponse> + Send;
}
