// crates/a2a-transport-wasi/src/lib.rs
//! WASI HTTP transport implementation.

pub mod error;
pub mod poll;

pub use error::WasiError;
pub use poll::{PollableExt, WasiPollFuture};

use a2a_transport::{HttpClient, HttpRequest, HttpResponse};
use bytes::Bytes;
use futures_core::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// WASI HTTP client using wasi:http.
pub struct WasiHttpClient;

impl WasiHttpClient {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WasiHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient for WasiHttpClient {
    type Error = WasiError;

    fn request(
        &self,
        _request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> + Send {
        async move {
            // TODO: Implement using wasi::http::outgoing_handler
            todo!("WASI HTTP client not yet implemented")
        }
    }

    fn request_stream(
        &self,
        _request: HttpRequest,
    ) -> impl Future<
        Output = Result<
            impl Stream<Item = Result<Bytes, Self::Error>> + Send,
            Self::Error,
        >,
    > + Send {
        async move {
            // TODO: Implement streaming using wasi::http
            Ok(EmptyStream(std::marker::PhantomData))
        }
    }
}

/// An empty stream that immediately returns `None`.
struct EmptyStream<T>(std::marker::PhantomData<T>);

impl<T> Stream for EmptyStream<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}

// SAFETY: EmptyStream contains no data, just a PhantomData marker
unsafe impl<T> Send for EmptyStream<T> {}

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
