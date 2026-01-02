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
