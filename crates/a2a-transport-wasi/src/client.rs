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
        ErrorCode::HttpResponseTimeout => WasiError::HttpRequestTimeout,
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
