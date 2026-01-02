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
        _ => Method::Get, // Default for unsupported methods (Patch, Head, Options, etc.)
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

    // Convert body to Option<Bytes> (None if empty, Some otherwise)
    let body = if body_data.is_empty() {
        None
    } else {
        Some(Bytes::from(body_data))
    };

    Ok(HttpRequest {
        method,
        url,
        headers,
        body,
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
