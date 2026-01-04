//! JSON-RPC 2.0 types for A2A protocol.

// Allow unused constants - these are defined for completeness and may be used later
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC 2.0 request.
#[derive(Debug, Deserialize)]
pub struct Request {
    /// Protocol version, must be "2.0"
    #[allow(dead_code)]
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: Value,
    pub id: Value,
}

/// JSON-RPC 2.0 response.
#[derive(Debug, Serialize)]
pub struct Response {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
    pub id: Value,
}

/// JSON-RPC 2.0 error object.
#[derive(Debug, Serialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl Response {
    pub fn success(id: Value, result: impl Serialize) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(serde_json::to_value(result).unwrap_or(Value::Null)),
            error: None,
            id,
        }
    }

    pub fn error(id: Value, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(RpcError {
                code,
                message: message.into(),
                data: None,
            }),
            id,
        }
    }

    pub fn method_not_found(id: Value) -> Self {
        Self::error(id, -32601, "Method not found")
    }

    pub fn invalid_params(id: Value, msg: impl Into<String>) -> Self {
        Self::error(id, -32602, msg)
    }

    pub fn internal_error(id: Value, msg: impl Into<String>) -> Self {
        Self::error(id, -32603, msg)
    }
}

// Standard JSON-RPC error codes
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

// A2A-specific error codes
pub const TASK_NOT_FOUND: i32 = -32001;
pub const TASK_NOT_CANCELABLE: i32 = -32002;

use crate::convert;
use crate::wasi::http::types::IncomingRequest;

/// Maximum request body size (1 MB).
const MAX_BODY_SIZE: usize = 1024 * 1024;

/// Handle a JSON-RPC request.
pub fn handle(request: &IncomingRequest) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    let body = read_request_body(request)?;
    handle_jsonrpc(&body)
}

fn handle_jsonrpc(body: &[u8]) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    // Parse JSON-RPC request
    let request: Request = match serde_json::from_slice(body) {
        Ok(r) => r,
        Err(e) => {
            let response = Response::error(
                serde_json::Value::Null,
                PARSE_ERROR,
                format!("Parse error: {e}"),
            );
            let body = serde_json::to_vec(&response).unwrap_or_default();
            return Ok((400, "application/json", body));
        }
    };

    // Route to handler
    let response = match request.method.as_str() {
        "message/send" => handle_message_send(&request),
        "tasks/get" => handle_tasks_get(&request),
        "tasks/cancel" => handle_tasks_cancel(&request),
        "message/stream" | "tasks/resubscribe" => {
            Response::error(request.id.clone(), -32601, "Streaming not implemented")
        }
        _ => Response::method_not_found(request.id.clone()),
    };

    let body = serde_json::to_vec(&response).unwrap_or_default();
    Ok((200, "application/json", body))
}

fn handle_message_send(request: &Request) -> Response {
    use crate::a2a::protocol::agent;

    // In prost-generated types, MessageSendParams is now SendMessageRequest
    let params: a2a_types::SendMessageRequest =
        match serde_json::from_value(request.params.clone()) {
            Ok(p) => p,
            Err(e) => return Response::invalid_params(request.id.clone(), e.to_string()),
        };

    let wit_params = match convert::message_send_params_to_wit(&params) {
        Ok(p) => p,
        Err(e) => return Response::invalid_params(request.id.clone(), e),
    };

    match agent::on_message(&wit_params) {
        Ok(response) => {
            let a2a_response = convert::send_response_from_wit(&response);
            Response::success(request.id.clone(), a2a_response)
        }
        Err(e) => Response::error(request.id.clone(), e.code, e.message),
    }
}

fn handle_tasks_get(request: &Request) -> Response {
    use crate::a2a::protocol::agent;

    #[derive(serde::Deserialize)]
    struct Params {
        id: String,
        #[serde(rename = "historyLength")]
        history_length: Option<u32>,
    }

    let params: Params = match serde_json::from_value(request.params.clone()) {
        Ok(p) => p,
        Err(e) => return Response::invalid_params(request.id.clone(), e.to_string()),
    };

    match agent::on_get_task(&params.id, params.history_length) {
        Ok(Some(task)) => {
            let a2a_task = convert::task_from_wit(&task);
            Response::success(request.id.clone(), a2a_task)
        }
        Ok(None) => Response::success(request.id.clone(), serde_json::Value::Null),
        Err(e) => Response::error(request.id.clone(), e.code, e.message),
    }
}

fn handle_tasks_cancel(request: &Request) -> Response {
    use crate::a2a::protocol::agent;

    #[derive(serde::Deserialize)]
    struct Params {
        id: String,
    }

    let params: Params = match serde_json::from_value(request.params.clone()) {
        Ok(p) => p,
        Err(e) => return Response::invalid_params(request.id.clone(), e.to_string()),
    };

    match agent::on_cancel_task(&params.id) {
        Ok(Some(task)) => {
            let a2a_task = convert::task_from_wit(&task);
            Response::success(request.id.clone(), a2a_task)
        }
        Ok(None) => Response::success(request.id.clone(), serde_json::Value::Null),
        Err(e) => Response::error(request.id.clone(), e.code, e.message),
    }
}

fn read_request_body(request: &IncomingRequest) -> Result<Vec<u8>, (u16, String)> {
    let body = request
        .consume()
        .map_err(|_| (500, "Failed to consume request body".to_string()))?;

    let stream = body
        .stream()
        .map_err(|_| (500, "Failed to get body stream".to_string()))?;

    let mut data = Vec::new();
    loop {
        match stream.blocking_read(64 * 1024) {
            Ok(chunk) => {
                if chunk.is_empty() {
                    break;
                }
                if data.len() + chunk.len() > MAX_BODY_SIZE {
                    return Err((413, "Request body too large".to_string()));
                }
                data.extend_from_slice(&chunk);
            }
            Err(crate::wasi::io::streams::StreamError::Closed) => break,
            Err(_) => return Err((500, "Failed to read body".to_string())),
        }
    }

    Ok(data)
}
