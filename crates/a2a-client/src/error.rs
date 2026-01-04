// crates/a2a-client/src/error.rs
use a2a_types::{Binding, TaskId};
use thiserror::Error;

/// Client errors with layered protocol details.
#[derive(Debug, Error)]
pub enum Error {
    #[error("agent not found: {0}")]
    AgentNotFound(String),

    #[error("no compatible binding: server supports {available:?}")]
    NoCompatibleBinding { available: Vec<Binding> },

    #[error("task not found: {0}")]
    TaskNotFound(TaskId),

    #[error("invalid params: {0}")]
    InvalidParams(ParamError),

    #[error("agent error: {message}")]
    Agent {
        message: String,
        #[source]
        source: ProtocolError,
    },

    #[error("transport error: {0}")]
    Transport(String),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid url: {0}")]
    InvalidUrl(String),
}

/// Parameter validation error.
#[derive(Debug, Error)]
pub enum ParamError {
    #[error("missing required field: {field}")]
    MissingField { field: &'static str },

    #[error("invalid value for {field}: {reason}")]
    InvalidValue { field: &'static str, reason: String },
}

/// Protocol-specific error details.
#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("JSON-RPC error {code}: {message}")]
    JsonRpc {
        code: JsonRpcErrorCode,
        message: String,
        data: Option<serde_json::Value>,
    },

    #[error("REST error {status}: {}", body.as_ref().map(|v| v.to_string()).unwrap_or_default())]
    Rest {
        status: u16,
        body: Option<serde_json::Value>,
    },
}

/// Standard JSON-RPC error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonRpcErrorCode {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    InternalError,
    ServerError(i32),
    ApplicationError(i32),
}

impl JsonRpcErrorCode {
    pub fn from_code(code: i32) -> Self {
        match code {
            -32700 => Self::ParseError,
            -32600 => Self::InvalidRequest,
            -32601 => Self::MethodNotFound,
            -32602 => Self::InvalidParams,
            -32603 => Self::InternalError,
            c if (-32099..=-32000).contains(&c) => Self::ServerError(c),
            c => Self::ApplicationError(c),
        }
    }

    pub fn code(&self) -> i32 {
        match self {
            Self::ParseError => -32700,
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::InvalidParams => -32602,
            Self::InternalError => -32603,
            Self::ServerError(c) | Self::ApplicationError(c) => *c,
        }
    }
}

impl std::fmt::Display for JsonRpcErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
