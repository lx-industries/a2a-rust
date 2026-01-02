# WASI HTTP Client/Server Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the `todo!()` placeholders in `a2a-transport-wasi` with actual `wasi:http` implementation using poll-based async.

**Architecture:** WASIP2 poll-based async model. Client uses `outgoing_handler::handle()` with manual polling via `FutureIncomingResponse::subscribe()`. Server exports `wasi:http/incoming-handler` for the host to call.

**Tech Stack:** wasi 0.14.7 (re-exports wasip2), futures-core 0.3.31, bytes 1.11.0

---

## Task 1: Expand Error Types

**Files:**
- Modify: `crates/a2a-transport-wasi/src/error.rs`

**Step 1: Add comprehensive error variants**

Replace the error module with variants that map to wasi:http ErrorCode:

```rust
// crates/a2a-transport-wasi/src/error.rs
use thiserror::Error;

/// WASI HTTP transport errors.
#[derive(Debug, Error)]
pub enum WasiError {
    #[error("DNS lookup failed: {0}")]
    DnsError(String),

    #[error("Connection timeout")]
    ConnectionTimeout,

    #[error("Connection refused")]
    ConnectionRefused,

    #[error("Connection reset")]
    ConnectionReset,

    #[error("Connection terminated")]
    ConnectionTerminated,

    #[error("TLS protocol error: {0}")]
    TlsProtocolError(String),

    #[error("TLS certificate error: {0}")]
    TlsCertificateError(String),

    #[error("TLS alert: {alert_id} {alert_message}")]
    TlsAlertReceived { alert_id: u8, alert_message: String },

    #[error("HTTP protocol error: {0}")]
    HttpProtocolError(String),

    #[error("Request body error: {0}")]
    HttpRequestBodyError(String),

    #[error("Response body error: {0}")]
    HttpResponseBodyError(String),

    #[error("Request denied: {0}")]
    HttpRequestDenied(String),

    #[error("Request timeout")]
    HttpRequestTimeout,

    #[error("Response header size exceeded")]
    HttpResponseHeaderSectionSize,

    #[error("Response body size exceeded")]
    HttpResponseBodySize,

    #[error("Response trailer size exceeded")]
    HttpResponseTrailerSectionSize,

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Body already consumed")]
    BodyAlreadyConsumed,

    #[error("Stream error: {0}")]
    StreamError(String),
}
```

**Step 2: Verify it compiles**

Run: `cargo check -p a2a-transport-wasi`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-transport-wasi/src/error.rs
git commit -m "feat(transport-wasi): expand WasiError variants for wasi:http"
```

---

## Task 2: Add Poll-to-Async Bridge

**Files:**
- Create: `crates/a2a-transport-wasi/src/poll.rs`
- Modify: `crates/a2a-transport-wasi/src/lib.rs` (add module)

**Step 1: Create poll module with WasiPollable future wrapper**

```rust
// crates/a2a-transport-wasi/src/poll.rs
//! Poll-to-async bridge for WASI pollables.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasi::io::poll::{poll, Pollable};

/// A future that wraps a WASI `Pollable` and yields when it becomes ready.
///
/// This allows WASI poll-based async to integrate with Rust's `Future` trait.
pub struct WasiPollFuture<'a> {
    pollable: &'a Pollable,
}

impl<'a> WasiPollFuture<'a> {
    /// Create a new poll future from a WASI pollable.
    pub fn new(pollable: &'a Pollable) -> Self {
        Self { pollable }
    }
}

impl Future for WasiPollFuture<'_> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check if already ready without blocking
        if self.pollable.ready() {
            return Poll::Ready(());
        }
        // Block on the pollable - in WASM, this yields to the host
        poll(&[self.pollable]);
        Poll::Ready(())
    }
}

/// Extension trait for WASI Pollable to convert to a Future.
pub trait PollableExt {
    /// Wait for this pollable to become ready.
    fn wait(&self) -> WasiPollFuture<'_>;
}

impl PollableExt for Pollable {
    fn wait(&self) -> WasiPollFuture<'_> {
        WasiPollFuture::new(self)
    }
}
```

**Step 2: Add module to lib.rs**

Add to `crates/a2a-transport-wasi/src/lib.rs` after the error module:

```rust
pub mod poll;
pub use poll::{PollableExt, WasiPollFuture};
```

**Step 3: Verify it compiles**

Run: `cargo check -p a2a-transport-wasi`
Expected: Success

**Step 4: Commit**

```bash
git add crates/a2a-transport-wasi/src/poll.rs crates/a2a-transport-wasi/src/lib.rs
git commit -m "feat(transport-wasi): add poll-to-async bridge"
```

---

## Task 3: Implement HTTP Client - Request Building

**Files:**
- Create: `crates/a2a-transport-wasi/src/client.rs`
- Modify: `crates/a2a-transport-wasi/src/lib.rs`

**Step 1: Create client module with WasiHttpClient and request conversion**

```rust
// crates/a2a-transport-wasi/src/client.rs
//! WASI HTTP client implementation.

use crate::error::WasiError;
use crate::poll::PollableExt;
use a2a_transport::{HttpClient, HttpRequest, HttpResponse, Method};
use bytes::Bytes;
use futures_core::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasi::http::outgoing_handler;
use wasi::http::types::{
    ErrorCode, Fields, FutureIncomingResponse, IncomingBody, IncomingResponse,
    OutgoingBody, OutgoingRequest, Scheme,
};

/// WASI HTTP client using wasi:http/outgoing-handler.
pub struct WasiHttpClient;

impl WasiHttpClient {
    /// Create a new WASI HTTP client.
    pub fn new() -> Self {
        Self
    }
}

impl Default for WasiHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a2a-transport Method to wasi:http Method.
fn to_wasi_method(method: &Method) -> wasi::http::types::Method {
    match method {
        Method::Get => wasi::http::types::Method::Get,
        Method::Post => wasi::http::types::Method::Post,
        Method::Put => wasi::http::types::Method::Put,
        Method::Delete => wasi::http::types::Method::Delete,
        Method::Patch => wasi::http::types::Method::Patch,
    }
}

/// Convert wasi:http ErrorCode to WasiError.
fn from_error_code(code: ErrorCode) -> WasiError {
    match code {
        ErrorCode::DnsTimeout | ErrorCode::DnsError(_) => {
            WasiError::DnsError(format!("{code:?}"))
        }
        ErrorCode::ConnectionTimeout => WasiError::ConnectionTimeout,
        ErrorCode::ConnectionRefused => WasiError::ConnectionRefused,
        ErrorCode::ConnectionReset => WasiError::ConnectionReset,
        ErrorCode::ConnectionTerminated => WasiError::ConnectionTerminated,
        ErrorCode::TlsProtocolError => WasiError::TlsProtocolError(String::new()),
        ErrorCode::TlsCertificateError => WasiError::TlsCertificateError(String::new()),
        ErrorCode::TlsAlertReceived(alert) => WasiError::TlsAlertReceived {
            alert_id: alert.alert_id.unwrap_or(0),
            alert_message: alert.alert_message.unwrap_or_default(),
        },
        ErrorCode::HttpProtocolError => WasiError::HttpProtocolError(String::new()),
        ErrorCode::HttpRequestBodySize(_) => {
            WasiError::HttpRequestBodyError("body size exceeded".into())
        }
        ErrorCode::HttpResponseBodySize(_) => WasiError::HttpResponseBodySize,
        ErrorCode::HttpResponseHeaderSectionSize(_) => WasiError::HttpResponseHeaderSectionSize,
        ErrorCode::HttpResponseTrailerSectionSize(_) => WasiError::HttpResponseTrailerSectionSize,
        ErrorCode::HttpRequestDenied => WasiError::HttpRequestDenied(String::new()),
        ErrorCode::HttpRequestTimeout => WasiError::HttpRequestTimeout,
        ErrorCode::InternalError(msg) => {
            WasiError::InternalError(msg.unwrap_or_default())
        }
        _ => WasiError::InternalError(format!("{code:?}")),
    }
}

/// Build an OutgoingRequest from HttpRequest.
fn build_outgoing_request(request: &HttpRequest) -> Result<OutgoingRequest, WasiError> {
    // Build headers
    let headers = Fields::new();
    for (name, value) in &request.headers {
        headers
            .append(name, value.as_bytes())
            .map_err(|_| WasiError::InvalidRequest(format!("invalid header: {name}")))?;
    }

    // Create outgoing request
    let outgoing = OutgoingRequest::new(headers);

    // Set method
    outgoing
        .set_method(&to_wasi_method(&request.method))
        .map_err(|()| WasiError::InvalidRequest("failed to set method".into()))?;

    // Parse URL and set scheme/authority/path
    let url = &request.url;
    let scheme = if url.starts_with("https://") {
        Scheme::Https
    } else if url.starts_with("http://") {
        Scheme::Http
    } else {
        return Err(WasiError::InvalidRequest("unsupported URL scheme".into()));
    };
    outgoing
        .set_scheme(Some(&scheme))
        .map_err(|()| WasiError::InvalidRequest("failed to set scheme".into()))?;

    // Extract authority (host:port) and path
    let without_scheme = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);

    let (authority, path) = match without_scheme.find('/') {
        Some(idx) => (&without_scheme[..idx], &without_scheme[idx..]),
        None => (without_scheme, "/"),
    };

    outgoing
        .set_authority(Some(authority))
        .map_err(|()| WasiError::InvalidRequest("failed to set authority".into()))?;
    outgoing
        .set_path_with_query(Some(path))
        .map_err(|()| WasiError::InvalidRequest("failed to set path".into()))?;

    Ok(outgoing)
}
```

**Step 2: Add module reference to lib.rs**

Add to `crates/a2a-transport-wasi/src/lib.rs`:

```rust
pub mod client;
pub use client::WasiHttpClient;
```

**Step 3: Verify it compiles**

Run: `cargo check -p a2a-transport-wasi`
Expected: Success

**Step 4: Commit**

```bash
git add crates/a2a-transport-wasi/src/client.rs crates/a2a-transport-wasi/src/lib.rs
git commit -m "feat(transport-wasi): add client request building"
```

---

## Task 4: Implement HTTP Client - Request/Response

**Files:**
- Modify: `crates/a2a-transport-wasi/src/client.rs`

**Step 1: Add request body writing function**

Add to `client.rs` after `build_outgoing_request`:

```rust
/// Write the request body to an OutgoingBody.
fn write_body(body: &OutgoingBody, data: &[u8]) -> Result<(), WasiError> {
    if data.is_empty() {
        return Ok(());
    }
    let stream = body
        .write()
        .map_err(|()| WasiError::HttpRequestBodyError("failed to get write stream".into()))?;

    // Write in chunks (WASI streams have limited buffer sizes)
    const CHUNK_SIZE: usize = 4096;
    for chunk in data.chunks(CHUNK_SIZE) {
        stream
            .blocking_write_and_flush(chunk)
            .map_err(|e| WasiError::HttpRequestBodyError(format!("{e:?}")))?;
    }
    drop(stream);
    Ok(())
}

/// Read all data from an IncomingBody.
fn read_body(body: IncomingBody) -> Result<Vec<u8>, WasiError> {
    let stream = body
        .stream()
        .map_err(|()| WasiError::HttpResponseBodyError("failed to get read stream".into()))?;

    let mut data = Vec::new();
    loop {
        match stream.read(4096) {
            Ok(chunk) => {
                if chunk.is_empty() {
                    break;
                }
                data.extend_from_slice(&chunk);
            }
            Err(wasi::io::streams::StreamError::Closed) => break,
            Err(e) => {
                return Err(WasiError::HttpResponseBodyError(format!("{e:?}")));
            }
        }
    }
    drop(stream);
    IncomingBody::finish(body);
    Ok(data)
}

/// Convert IncomingResponse to HttpResponse.
fn to_http_response(response: IncomingResponse) -> Result<HttpResponse, WasiError> {
    let status = response.status();
    let wasi_headers = response.headers();
    let entries = wasi_headers.entries();

    let mut headers = Vec::with_capacity(entries.len());
    for (name, value) in entries {
        let value_str = String::from_utf8(value)
            .map_err(|_| WasiError::HttpProtocolError("invalid header value".into()))?;
        headers.push((name, value_str));
    }
    drop(wasi_headers);

    let body = response
        .consume()
        .map_err(|()| WasiError::BodyAlreadyConsumed)?;
    let body_data = read_body(body)?;

    Ok(HttpResponse {
        status,
        headers,
        body: Bytes::from(body_data),
    })
}
```

**Step 2: Verify it compiles**

Run: `cargo check -p a2a-transport-wasi`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-transport-wasi/src/client.rs
git commit -m "feat(transport-wasi): add request/response body handling"
```

---

## Task 5: Implement HttpClient Trait

**Files:**
- Modify: `crates/a2a-transport-wasi/src/client.rs`
- Modify: `crates/a2a-transport-wasi/src/lib.rs`

**Step 1: Implement HttpClient for WasiHttpClient**

Add to `client.rs`:

```rust
impl HttpClient for WasiHttpClient {
    type Error = WasiError;

    fn request(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> + Send {
        async move {
            // Build and send request
            let outgoing = build_outgoing_request(&request)?;
            let body = outgoing
                .body()
                .map_err(|()| WasiError::HttpRequestBodyError("failed to get body".into()))?;

            // Write body
            write_body(&body, &request.body)?;
            OutgoingBody::finish(body, None)
                .map_err(|e| WasiError::HttpRequestBodyError(format!("{e:?}")))?;

            // Send request
            let future_response = outgoing_handler::handle(outgoing, None)
                .map_err(from_error_code)?;

            // Wait for response
            let pollable = future_response.subscribe();
            pollable.wait().await;

            // Get response
            let response = future_response
                .get()
                .ok_or(WasiError::InternalError("response not ready".into()))?
                .map_err(from_error_code)?;

            to_http_response(response)
        }
    }

    fn request_stream(
        &self,
        request: HttpRequest,
    ) -> impl Future<
        Output = Result<
            impl Stream<Item = Result<Bytes, Self::Error>> + Send,
            Self::Error,
        >,
    > + Send {
        async move {
            // Build and send request
            let outgoing = build_outgoing_request(&request)?;
            let body = outgoing
                .body()
                .map_err(|()| WasiError::HttpRequestBodyError("failed to get body".into()))?;

            write_body(&body, &request.body)?;
            OutgoingBody::finish(body, None)
                .map_err(|e| WasiError::HttpRequestBodyError(format!("{e:?}")))?;

            let future_response = outgoing_handler::handle(outgoing, None)
                .map_err(from_error_code)?;

            let pollable = future_response.subscribe();
            pollable.wait().await;

            let response = future_response
                .get()
                .ok_or(WasiError::InternalError("response not ready".into()))?
                .map_err(from_error_code)?;

            let incoming_body = response
                .consume()
                .map_err(|()| WasiError::BodyAlreadyConsumed)?;

            Ok(WasiBodyStream::new(incoming_body))
        }
    }
}

/// Streaming body reader for WASI HTTP responses.
pub struct WasiBodyStream {
    body: Option<IncomingBody>,
    stream: Option<wasi::io::streams::InputStream>,
}

impl WasiBodyStream {
    fn new(body: IncomingBody) -> Self {
        let stream = body.stream().ok();
        Self {
            body: Some(body),
            stream,
        }
    }
}

impl Stream for WasiBodyStream {
    type Item = Result<Bytes, WasiError>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let stream = match &self.stream {
            Some(s) => s,
            None => return Poll::Ready(None),
        };

        match stream.read(4096) {
            Ok(chunk) if chunk.is_empty() => {
                // Clean up
                self.stream = None;
                if let Some(body) = self.body.take() {
                    IncomingBody::finish(body);
                }
                Poll::Ready(None)
            }
            Ok(chunk) => Poll::Ready(Some(Ok(Bytes::from(chunk)))),
            Err(wasi::io::streams::StreamError::Closed) => {
                self.stream = None;
                if let Some(body) = self.body.take() {
                    IncomingBody::finish(body);
                }
                Poll::Ready(None)
            }
            Err(e) => Poll::Ready(Some(Err(WasiError::StreamError(format!("{e:?}"))))),
        }
    }
}

// SAFETY: WasiBodyStream is single-threaded (WASM is single-threaded)
unsafe impl Send for WasiBodyStream {}
```

**Step 2: Update lib.rs to remove old implementation**

Replace `crates/a2a-transport-wasi/src/lib.rs` with:

```rust
// crates/a2a-transport-wasi/src/lib.rs
//! WASI HTTP transport implementation.

pub mod client;
pub mod error;
pub mod poll;

pub use client::{WasiBodyStream, WasiHttpClient};
pub use error::WasiError;
pub use poll::{PollableExt, WasiPollFuture};
```

**Step 3: Verify it compiles**

Run: `cargo check -p a2a-transport-wasi`
Expected: Success

**Step 4: Commit**

```bash
git add crates/a2a-transport-wasi/src/client.rs crates/a2a-transport-wasi/src/lib.rs
git commit -m "feat(transport-wasi): implement HttpClient trait"
```

---

## Task 6: Add Server Module - Request Handling

**Files:**
- Create: `crates/a2a-transport-wasi/src/server.rs`
- Modify: `crates/a2a-transport-wasi/src/lib.rs`

**Step 1: Create server module with request/response conversion**

```rust
// crates/a2a-transport-wasi/src/server.rs
//! WASI HTTP server implementation via incoming-handler export.

use crate::error::WasiError;
use a2a_transport::{HttpRequest, HttpResponse, Method};
use bytes::Bytes;
use wasi::http::types::{
    Fields, IncomingBody, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

/// Convert wasi:http Method to a2a-transport Method.
fn from_wasi_method(method: wasi::http::types::Method) -> Method {
    match method {
        wasi::http::types::Method::Get => Method::Get,
        wasi::http::types::Method::Post => Method::Post,
        wasi::http::types::Method::Put => Method::Put,
        wasi::http::types::Method::Delete => Method::Delete,
        wasi::http::types::Method::Patch => Method::Patch,
        _ => Method::Get, // Default for unsupported methods
    }
}

/// Read the incoming request body.
fn read_incoming_body(body: IncomingBody) -> Result<Vec<u8>, WasiError> {
    let stream = body
        .stream()
        .map_err(|()| WasiError::HttpRequestBodyError("failed to get stream".into()))?;

    let mut data = Vec::new();
    loop {
        match stream.read(4096) {
            Ok(chunk) if chunk.is_empty() => break,
            Ok(chunk) => data.extend_from_slice(&chunk),
            Err(wasi::io::streams::StreamError::Closed) => break,
            Err(e) => return Err(WasiError::HttpRequestBodyError(format!("{e:?}"))),
        }
    }
    drop(stream);
    IncomingBody::finish(body);
    Ok(data)
}

/// Convert IncomingRequest to HttpRequest.
pub fn from_incoming_request(request: IncomingRequest) -> Result<HttpRequest, WasiError> {
    let method = from_wasi_method(request.method());

    // Build URL from scheme, authority, path
    let scheme = match request.scheme() {
        Some(wasi::http::types::Scheme::Http) => "http",
        Some(wasi::http::types::Scheme::Https) | None => "https",
        Some(wasi::http::types::Scheme::Other(s)) => {
            return Err(WasiError::InvalidRequest(format!("unsupported scheme: {s}")));
        }
    };
    let authority = request.authority().unwrap_or_default();
    let path = request.path_with_query().unwrap_or_else(|| "/".into());
    let url = format!("{scheme}://{authority}{path}");

    // Convert headers
    let wasi_headers = request.headers();
    let entries = wasi_headers.entries();
    let mut headers = Vec::with_capacity(entries.len());
    for (name, value) in entries {
        let value_str = String::from_utf8(value)
            .map_err(|_| WasiError::HttpProtocolError("invalid header value".into()))?;
        headers.push((name, value_str));
    }
    drop(wasi_headers);

    // Read body
    let body = request
        .consume()
        .map_err(|()| WasiError::BodyAlreadyConsumed)?;
    let body_data = read_incoming_body(body)?;

    Ok(HttpRequest {
        method,
        url,
        headers,
        body: Bytes::from(body_data),
    })
}

/// Send an HttpResponse via ResponseOutparam.
pub fn send_response(response: HttpResponse, outparam: ResponseOutparam) -> Result<(), WasiError> {
    // Build headers
    let headers = Fields::new();
    for (name, value) in &response.headers {
        headers
            .append(name, value.as_bytes())
            .map_err(|_| WasiError::InvalidRequest(format!("invalid header: {name}")))?;
    }

    // Create response with status
    let outgoing = OutgoingResponse::new(headers);
    outgoing
        .set_status_code(response.status)
        .map_err(|()| WasiError::InvalidRequest("invalid status code".into()))?;

    // Get body handle
    let body = outgoing
        .body()
        .map_err(|()| WasiError::HttpResponseBodyError("failed to get body".into()))?;

    // Send response headers
    ResponseOutparam::set(outparam, Ok(outgoing));

    // Write body
    if !response.body.is_empty() {
        let stream = body
            .write()
            .map_err(|()| WasiError::HttpResponseBodyError("failed to get write stream".into()))?;

        const CHUNK_SIZE: usize = 4096;
        for chunk in response.body.chunks(CHUNK_SIZE) {
            stream
                .blocking_write_and_flush(chunk)
                .map_err(|e| WasiError::HttpResponseBodyError(format!("{e:?}")))?;
        }
        drop(stream);
    }

    // Finish body
    OutgoingBody::finish(body, None)
        .map_err(|e| WasiError::HttpResponseBodyError(format!("{e:?}")))?;

    Ok(())
}
```

**Step 2: Add server module to lib.rs**

Add to `crates/a2a-transport-wasi/src/lib.rs`:

```rust
pub mod server;
pub use server::{from_incoming_request, send_response};
```

**Step 3: Verify it compiles**

Run: `cargo check -p a2a-transport-wasi`
Expected: Success

**Step 4: Commit**

```bash
git add crates/a2a-transport-wasi/src/server.rs crates/a2a-transport-wasi/src/lib.rs
git commit -m "feat(transport-wasi): add server request/response handling"
```

---

## Task 7: Add Incoming Handler Export Macro

**Files:**
- Modify: `crates/a2a-transport-wasi/src/server.rs`

**Step 1: Add documentation and helper for component export**

Add to the end of `server.rs`:

```rust
/// Helper macro to implement the wasi:http/incoming-handler export.
///
/// This macro generates the necessary export implementation for your WASM component.
/// The handler function receives an `HttpRequest` and must return an `HttpResponse`.
///
/// # Example
///
/// ```ignore
/// use a2a_transport_wasi::{export_incoming_handler, server};
/// use a2a_transport::{HttpRequest, HttpResponse};
///
/// struct MyHandler;
///
/// impl MyHandler {
///     fn handle(request: HttpRequest) -> HttpResponse {
///         HttpResponse {
///             status: 200,
///             headers: vec![("content-type".into(), "text/plain".into())],
///             body: "Hello from WASI!".into(),
///         }
///     }
/// }
///
/// export_incoming_handler!(MyHandler);
/// ```
#[macro_export]
macro_rules! export_incoming_handler {
    ($handler:ty) => {
        // Export the handler using wit-bindgen generated code
        ::wasi::http::proxy::export!($handler);

        impl ::wasi::exports::http::incoming_handler::Guest for $handler {
            fn handle(
                request: ::wasi::http::types::IncomingRequest,
                response_out: ::wasi::http::types::ResponseOutparam,
            ) {
                // Convert incoming request to our type
                let http_request = match $crate::server::from_incoming_request(request) {
                    Ok(req) => req,
                    Err(e) => {
                        // Send error response
                        let error_response = ::a2a_transport::HttpResponse {
                            status: 500,
                            headers: vec![
                                ("content-type".into(), "text/plain".into()),
                            ],
                            body: ::bytes::Bytes::from(format!("Request parse error: {e}")),
                        };
                        let _ = $crate::server::send_response(error_response, response_out);
                        return;
                    }
                };

                // Call the handler
                let response = <$handler>::handle(http_request);

                // Send response
                if let Err(e) = $crate::server::send_response(response, response_out) {
                    // Can't do much here since we already consumed response_out
                    // Log would require a different mechanism
                    let _ = e;
                }
            }
        }
    };
}
```

**Step 2: Verify it compiles**

Run: `cargo check -p a2a-transport-wasi`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-transport-wasi/src/server.rs
git commit -m "feat(transport-wasi): add incoming handler export macro"
```

---

## Task 8: Run Full Workspace Build

**Files:** None (verification only)

**Step 1: Run cargo check on entire workspace**

Run: `cargo check --workspace`
Expected: Success for all crates

**Step 2: Run cargo clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: No warnings

**Step 3: Fix any issues**

If there are warnings or errors, fix them.

**Step 4: Commit any fixes**

```bash
git add -A
git commit -m "fix(transport-wasi): address clippy warnings"
```

---

## Task 9: Add Integration Tests

**Files:**
- Create: `crates/a2a-transport-wasi/tests/client_test.rs`

**Step 1: Create test file (will only run in WASM)**

```rust
// crates/a2a-transport-wasi/tests/client_test.rs
//! Integration tests for WASI HTTP client.
//!
//! These tests require a WASI runtime (like wasmtime) to execute.
//! They are marked as ignored by default for regular cargo test.

#![cfg(target_arch = "wasm32")]

use a2a_transport::HttpClient;
use a2a_transport_wasi::WasiHttpClient;

#[test]
#[ignore = "requires WASI runtime"]
fn test_client_creation() {
    let _client = WasiHttpClient::new();
}
```

**Step 2: Verify tests compile**

Run: `cargo test --workspace --no-run`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-transport-wasi/tests/
git commit -m "test(transport-wasi): add integration test scaffold"
```

---

## Task 10: Update Documentation

**Files:**
- Modify: `crates/a2a-transport-wasi/src/lib.rs`

**Step 1: Add comprehensive module documentation**

Update the module doc at the top of `lib.rs`:

```rust
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
//!     body: Bytes::from(r#"{"message": "hello"}"#),
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
```

**Step 2: Verify docs build**

Run: `cargo doc -p a2a-transport-wasi --no-deps`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-transport-wasi/src/lib.rs
git commit -m "docs(transport-wasi): add comprehensive module documentation"
```

---

## Summary

After completing all tasks, `a2a-transport-wasi` will have:

1. **Error types** (`error.rs`) - Comprehensive error variants mapping to wasi:http ErrorCode
2. **Poll bridge** (`poll.rs`) - `WasiPollFuture` and `PollableExt` for async integration
3. **HTTP client** (`client.rs`) - Full `HttpClient` implementation using `wasi:http/outgoing-handler`
4. **HTTP server** (`server.rs`) - Request/response conversion and `export_incoming_handler!` macro
5. **Documentation** - Usage examples and module docs

The implementation uses WASIP2 poll-based async (wasi 0.14.7) and is ready for use in WASM components.
