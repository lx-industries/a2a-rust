# HTTP+JSON/REST Binding Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add HTTP+JSON/REST protocol binding alongside JSON-RPC with auto-negotiating client.

**Architecture:** Path-based routing (`/v1/*` for REST, `POST /` for JSON-RPC). Client auto-discovers agent card and selects binding. Layered error types preserve protocol details.

**Tech Stack:** Rust, serde, thiserror, a2a-types (typify-generated)

---

## Task 1: Add TaskId Newtype to a2a-types

**Files:**
- Modify: `crates/a2a-types/src/lib.rs`

**Step 1: Add TaskId type after the generated module re-export**

```rust
// After: pub use generated::*;

use serde::{Deserialize, Serialize};

/// Strongly-typed task identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskId(pub String);

impl TaskId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for TaskId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for TaskId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
```

**Step 2: Verify it compiles**

Run: `cargo check -p a2a-types`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-types/src/lib.rs
git commit -m "feat(a2a-types): add TaskId newtype"
```

---

## Task 2: Add Binding and ProtocolBinding Types to a2a-types

**Files:**
- Modify: `crates/a2a-types/src/lib.rs`

**Step 1: Add binding types after TaskId**

```rust
/// Protocol binding type for client/server communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binding {
    JsonRpc,
    Rest,
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binding::JsonRpc => write!(f, "JSONRPC"),
            Binding::Rest => write!(f, "HTTP+JSON"),
        }
    }
}

/// Protocol binding as declared in Agent Card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolBinding {
    #[serde(rename = "JSONRPC")]
    JsonRpc,
    #[serde(rename = "HTTP+JSON")]
    Rest,
    #[serde(rename = "GRPC")]
    Grpc,
}

impl From<ProtocolBinding> for Option<Binding> {
    fn from(pb: ProtocolBinding) -> Self {
        match pb {
            ProtocolBinding::JsonRpc => Some(Binding::JsonRpc),
            ProtocolBinding::Rest => Some(Binding::Rest),
            ProtocolBinding::Grpc => None, // Not supported
        }
    }
}
```

**Step 2: Verify it compiles**

Run: `cargo check -p a2a-types`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-types/src/lib.rs
git commit -m "feat(a2a-types): add Binding and ProtocolBinding types"
```

---

## Task 3: Add AgentInterface Type to a2a-types

**Files:**
- Modify: `crates/a2a-types/src/lib.rs`

**Step 1: Add AgentInterface after ProtocolBinding**

```rust
/// Interface declaration in Agent Card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentInterface {
    pub protocol_binding: ProtocolBinding,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}
```

**Step 2: Verify it compiles**

Run: `cargo check -p a2a-types`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-types/src/lib.rs
git commit -m "feat(a2a-types): add AgentInterface type"
```

---

## Task 4: Refactor a2a-client Error Types

**Files:**
- Modify: `crates/a2a-client/src/error.rs`

**Step 1: Replace existing error types with layered design**

```rust
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
```

**Step 2: Verify it compiles**

Run: `cargo check -p a2a-client`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-client/src/error.rs
git commit -m "refactor(a2a-client): layered error types with protocol details"
```

---

## Task 5: Create Binding Module in a2a-client

**Files:**
- Create: `crates/a2a-client/src/binding.rs`
- Modify: `crates/a2a-client/src/lib.rs`

**Step 1: Create binding.rs with SelectedBinding enum**

```rust
// crates/a2a-client/src/binding.rs
//! Protocol binding selection and configuration.

use a2a_types::Binding;

/// Selected binding for client communication.
#[derive(Debug, Clone)]
pub enum SelectedBinding {
    JsonRpc { url: String },
    Rest { url: String },
}

impl SelectedBinding {
    pub fn binding(&self) -> Binding {
        match self {
            Self::JsonRpc { .. } => Binding::JsonRpc,
            Self::Rest { .. } => Binding::Rest,
        }
    }

    pub fn url(&self) -> &str {
        match self {
            Self::JsonRpc { url } | Self::Rest { url } => url,
        }
    }
}
```

**Step 2: Add module to lib.rs**

In `crates/a2a-client/src/lib.rs`, add after `pub mod sse;`:

```rust
pub mod binding;
```

**Step 3: Verify it compiles**

Run: `cargo check -p a2a-client`
Expected: Success

**Step 4: Commit**

```bash
git add crates/a2a-client/src/binding.rs crates/a2a-client/src/lib.rs
git commit -m "feat(a2a-client): add binding module"
```

---

## Task 6: Add REST Module to a2a-client

**Files:**
- Create: `crates/a2a-client/src/rest.rs`
- Modify: `crates/a2a-client/src/lib.rs`

**Step 1: Create rest.rs with REST request helpers**

```rust
// crates/a2a-client/src/rest.rs
//! REST binding implementation.

use a2a_transport::HttpRequest;
use a2a_types::TaskId;

/// Build REST endpoint URL.
pub fn endpoint(base_url: &str, path: &str) -> String {
    format!("{}{}", base_url.trim_end_matches('/'), path)
}

/// POST /v1/message:send
pub fn send_message_request(base_url: &str, body: Vec<u8>) -> HttpRequest {
    HttpRequest::post(&endpoint(base_url, "/v1/message:send"), body)
        .with_header("Content-Type", "application/json")
        .with_header("Accept", "application/json")
}

/// GET /v1/tasks/{id}
pub fn get_task_request(base_url: &str, task_id: &TaskId) -> HttpRequest {
    HttpRequest::get(&endpoint(base_url, &format!("/v1/tasks/{}", task_id.as_str())))
        .with_header("Accept", "application/json")
}

/// GET /v1/tasks/{id}?historyLength={n}
pub fn get_task_with_history_request(
    base_url: &str,
    task_id: &TaskId,
    history_length: u32,
) -> HttpRequest {
    HttpRequest::get(&endpoint(
        base_url,
        &format!("/v1/tasks/{}?historyLength={}", task_id.as_str(), history_length),
    ))
    .with_header("Accept", "application/json")
}

/// POST /v1/tasks/{id}:cancel
pub fn cancel_task_request(base_url: &str, task_id: &TaskId) -> HttpRequest {
    HttpRequest::post(
        &endpoint(base_url, &format!("/v1/tasks/{}:cancel", task_id.as_str())),
        vec![],
    )
    .with_header("Accept", "application/json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_with_trailing_slash() {
        assert_eq!(
            endpoint("https://example.com/", "/v1/message:send"),
            "https://example.com/v1/message:send"
        );
    }

    #[test]
    fn test_endpoint_without_trailing_slash() {
        assert_eq!(
            endpoint("https://example.com", "/v1/message:send"),
            "https://example.com/v1/message:send"
        );
    }

    #[test]
    fn test_get_task_request() {
        let req = get_task_request("https://example.com", &TaskId::new("task-123"));
        assert!(req.url.contains("/v1/tasks/task-123"));
    }

    #[test]
    fn test_cancel_task_request() {
        let req = cancel_task_request("https://example.com", &TaskId::new("task-456"));
        assert!(req.url.contains("/v1/tasks/task-456:cancel"));
    }
}
```

**Step 2: Add module to lib.rs**

In `crates/a2a-client/src/lib.rs`, add after `pub mod binding;`:

```rust
pub mod rest;
```

**Step 3: Run tests**

Run: `cargo test -p a2a-client rest::`
Expected: All tests pass

**Step 4: Commit**

```bash
git add crates/a2a-client/src/rest.rs crates/a2a-client/src/lib.rs
git commit -m "feat(a2a-client): add REST binding module"
```

---

## Task 7: Refactor WASM Component - Extract JSON-RPC Handler

**Files:**
- Modify: `crates/a2a-wasm-component/src/server.rs`
- Modify: `crates/a2a-wasm-component/src/jsonrpc.rs`

**Step 1: Move JSON-RPC handling functions to jsonrpc.rs**

Add to `crates/a2a-wasm-component/src/jsonrpc.rs` (after existing types):

```rust
// Add at the end of the file, after TASK_NOT_CANCELABLE constant

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

    let params: a2a_types::MessageSendParams = match serde_json::from_value(request.params.clone())
    {
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
```

**Step 2: Verify it compiles**

Run: `cargo check -p a2a-wasm-component --target wasm32-wasip2`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/src/jsonrpc.rs
git commit -m "refactor(a2a-wasm-component): extract JSON-RPC handler functions"
```

---

## Task 8: Refactor WASM Component - Simplify server.rs Routing

**Files:**
- Modify: `crates/a2a-wasm-component/src/server.rs`

**Step 1: Replace server.rs with simplified routing**

```rust
//! HTTP handler implementation for A2A server.
//!
//! Routes requests to appropriate protocol handlers:
//! - JSON-RPC: `POST /`
//! - REST: `/v1/*` paths
//! - Agent card: `GET /.well-known/agent-card.json`

use crate::exports::wasi::http::incoming_handler::Guest;
use crate::jsonrpc;
use crate::wasi::http::types::{
    Headers, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

impl Guest for crate::Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let result = handle_request(request);

        match result {
            Ok((status, content_type, body)) => {
                send_response(response_out, status, content_type, body);
            }
            Err((status, message)) => {
                send_response(response_out, status, "text/plain", message.into_bytes());
            }
        }
    }
}

fn handle_request(request: IncomingRequest) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    let method = request.method();
    let path = request.path_with_query().unwrap_or_default();

    // Agent card discovery (new spec-compliant path)
    if path == "/.well-known/agent-card.json"
        && matches!(method, crate::wasi::http::types::Method::Get)
    {
        return handle_agent_card();
    }

    // REST binding: /v1/* paths (placeholder for Task 9)
    if path.starts_with("/v1/") {
        return Err((501, "REST binding not yet implemented".to_string()));
    }

    // JSON-RPC binding: POST /
    if (path == "/" || path.is_empty())
        && matches!(method, crate::wasi::http::types::Method::Post)
    {
        return jsonrpc::handle(&request);
    }

    // CORS preflight
    if matches!(method, crate::wasi::http::types::Method::Options) {
        return Ok((204, "text/plain", vec![]));
    }

    Err((404, "Not Found".to_string()))
}

fn handle_agent_card() -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::a2a::protocol::agent;

    match agent::get_agent_card() {
        Ok(card_json) => Ok((200, "application/json", card_json.into_bytes())),
        Err(e) => {
            let response = jsonrpc::Response::internal_error(serde_json::Value::Null, e.message);
            let body = serde_json::to_vec(&response).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
}

fn send_response(response_out: ResponseOutparam, status: u16, content_type: &str, body: Vec<u8>) {
    let headers = Headers::new();
    headers
        .set("Content-Type", &[content_type.as_bytes().to_vec()])
        .ok();
    headers
        .set("Content-Length", &[body.len().to_string().into_bytes()])
        .ok();

    let response = OutgoingResponse::new(headers);
    response.set_status_code(status).ok();

    let outgoing_body = response.body().unwrap();
    ResponseOutparam::set(response_out, Ok(response));

    let stream = outgoing_body.write().unwrap();
    stream.blocking_write_and_flush(&body).ok();
    drop(stream);

    OutgoingBody::finish(outgoing_body, None).ok();
}
```

**Step 2: Verify it compiles**

Run: `cargo check -p a2a-wasm-component --target wasm32-wasip2`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/src/server.rs
git commit -m "refactor(a2a-wasm-component): simplify server.rs routing"
```

---

## Task 9: Add REST Handler Module to WASM Component

**Files:**
- Create: `crates/a2a-wasm-component/src/rest.rs`
- Modify: `crates/a2a-wasm-component/src/lib.rs`
- Modify: `crates/a2a-wasm-component/src/server.rs`

**Step 1: Create rest.rs with REST endpoint handlers**

```rust
//! REST binding implementation for A2A server.
//!
//! Handles `/v1/*` paths per A2A HTTP+JSON/REST binding spec.

use crate::convert;
use crate::wasi::http::types::{IncomingRequest, Method};
use serde::Serialize;

/// Handle a REST request.
pub fn handle(
    method: Method,
    path: &str,
    request: &IncomingRequest,
) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    match (method, path) {
        // POST /v1/message:send
        (Method::Post, "/v1/message:send") => handle_send_message(request),

        // GET /v1/tasks/{id} or GET /v1/tasks/{id}?historyLength=N
        (Method::Get, p) if is_task_get(p) => {
            let (task_id, history_length) = parse_task_get(p)?;
            handle_get_task(&task_id, history_length)
        }

        // POST /v1/tasks/{id}:cancel
        (Method::Post, p) if p.starts_with("/v1/tasks/") && p.ends_with(":cancel") => {
            let task_id = extract_task_id_before_action(p)?;
            handle_cancel_task(&task_id)
        }

        // GET /v1/agentCard (extended, authenticated)
        (Method::Get, "/v1/agentCard") => handle_extended_agent_card(),

        _ => Err((404, "Not Found".to_string())),
    }
}

fn is_task_get(path: &str) -> bool {
    path.starts_with("/v1/tasks/") && !path.ends_with(":cancel")
}

fn parse_task_get(path: &str) -> Result<(String, Option<u32>), (u16, String)> {
    // Path format: /v1/tasks/{id} or /v1/tasks/{id}?historyLength=N
    let path = path.strip_prefix("/v1/tasks/").unwrap_or(path);

    let (task_id, query) = match path.split_once('?') {
        Some((id, q)) => (id, Some(q)),
        None => (path, None),
    };

    if task_id.is_empty() {
        return Err((400, "Missing task ID".to_string()));
    }

    let history_length = query
        .and_then(|q| {
            q.split('&')
                .find_map(|param| param.strip_prefix("historyLength="))
        })
        .and_then(|v| v.parse().ok());

    Ok((task_id.to_string(), history_length))
}

fn extract_task_id_before_action(path: &str) -> Result<String, (u16, String)> {
    // Path format: /v1/tasks/{id}:cancel
    let path = path
        .strip_prefix("/v1/tasks/")
        .ok_or((400, "Invalid path".to_string()))?;

    let task_id = path
        .strip_suffix(":cancel")
        .ok_or((400, "Invalid path".to_string()))?;

    if task_id.is_empty() {
        return Err((400, "Missing task ID".to_string()));
    }

    Ok(task_id.to_string())
}

fn handle_send_message(
    request: &IncomingRequest,
) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::a2a::protocol::agent;

    let body = read_request_body(request)?;

    let params: a2a_types::MessageSendParams = serde_json::from_slice(&body)
        .map_err(|e| (400, format!("Invalid JSON: {e}")))?;

    let wit_params = convert::message_send_params_to_wit(&params)
        .map_err(|e| (400, format!("Invalid params: {e}")))?;

    match agent::on_message(&wit_params) {
        Ok(response) => {
            let a2a_response = convert::send_response_from_wit(&response);
            let body = serde_json::to_vec(&a2a_response).unwrap_or_default();
            Ok((200, "application/json", body))
        }
        Err(e) => {
            let body = serde_json::to_vec(&ErrorResponse { error: e.message }).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
}

fn handle_get_task(
    task_id: &str,
    history_length: Option<u32>,
) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::a2a::protocol::agent;

    match agent::on_get_task(task_id, history_length) {
        Ok(Some(task)) => {
            let a2a_task = convert::task_from_wit(&task);
            let body = serde_json::to_vec(&a2a_task).unwrap_or_default();
            Ok((200, "application/json", body))
        }
        Ok(None) => {
            let body =
                serde_json::to_vec(&ErrorResponse { error: "Task not found".to_string() })
                    .unwrap_or_default();
            Ok((404, "application/json", body))
        }
        Err(e) => {
            let body = serde_json::to_vec(&ErrorResponse { error: e.message }).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
}

fn handle_cancel_task(task_id: &str) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::a2a::protocol::agent;

    match agent::on_cancel_task(task_id) {
        Ok(Some(task)) => {
            let a2a_task = convert::task_from_wit(&task);
            let body = serde_json::to_vec(&a2a_task).unwrap_or_default();
            Ok((200, "application/json", body))
        }
        Ok(None) => {
            let body =
                serde_json::to_vec(&ErrorResponse { error: "Task not found".to_string() })
                    .unwrap_or_default();
            Ok((404, "application/json", body))
        }
        Err(e) => {
            let body = serde_json::to_vec(&ErrorResponse { error: e.message }).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
}

fn handle_extended_agent_card() -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::a2a::protocol::agent;

    // For now, return the same card as the public one
    // In the future, this could return additional authenticated details
    match agent::get_agent_card() {
        Ok(card_json) => Ok((200, "application/json", card_json.into_bytes())),
        Err(e) => {
            let body = serde_json::to_vec(&ErrorResponse { error: e.message }).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
}

/// Maximum request body size (1 MB).
const MAX_BODY_SIZE: usize = 1024 * 1024;

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

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
```

**Step 2: Add module to lib.rs**

In `crates/a2a-wasm-component/src/lib.rs`, add after `mod server;`:

```rust
mod rest;
```

**Step 3: Update server.rs to route to REST handler**

Replace the REST placeholder in server.rs:

```rust
    // REST binding: /v1/* paths
    if path.starts_with("/v1/") {
        return crate::rest::handle(method, &path, &request);
    }
```

**Step 4: Verify it compiles**

Run: `cargo check -p a2a-wasm-component --target wasm32-wasip2`
Expected: Success

**Step 5: Commit**

```bash
git add crates/a2a-wasm-component/src/rest.rs crates/a2a-wasm-component/src/lib.rs crates/a2a-wasm-component/src/server.rs
git commit -m "feat(a2a-wasm-component): add REST handler module"
```

---

## Task 10: Update WASM Component lib.rs Documentation

**Files:**
- Modify: `crates/a2a-wasm-component/src/lib.rs`

**Step 1: Update module documentation to reflect new endpoints**

Replace the existing doc comment at the top:

```rust
//! A2A WASM component with HTTP server and client interfaces.
//!
//! This component exports:
//! - `wasi:http/incoming-handler` - HTTP server for A2A requests
//! - `a2a:protocol/client` - Client interface for calling other agents
//!
//! And imports:
//! - `wasi:http/outgoing-handler` - For client HTTP requests
//! - `a2a:protocol/agent` - Host-provided agent logic
//!
//! # HTTP Endpoints
//!
//! ## Discovery
//! - `GET /.well-known/agent-card.json` - Agent card discovery
//!
//! ## JSON-RPC Binding
//! - `POST /` - JSON-RPC (message/send, tasks/get, tasks/cancel)
//!
//! ## REST Binding (HTTP+JSON)
//! - `POST /v1/message:send` - Send a message
//! - `GET /v1/tasks/{id}` - Get task by ID
//! - `GET /v1/tasks/{id}?historyLength=N` - Get task with history
//! - `POST /v1/tasks/{id}:cancel` - Cancel a task
//! - `GET /v1/agentCard` - Extended agent card (authenticated)
//!
//! # Limitations
//!
//! - Only `TextPart` is supported; `FilePart` and `DataPart` return errors
//! - Streaming (message/stream) is not yet implemented
//! - Metadata fields are not supported (deferred)
```

**Step 2: Verify it compiles**

Run: `cargo check -p a2a-wasm-component --target wasm32-wasip2`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/src/lib.rs
git commit -m "docs(a2a-wasm-component): update module docs for REST endpoints"
```

---

## Task 11: Run Full Test Suite

**Files:** None (verification only)

**Step 1: Run all tests**

Run: `cargo test --workspace`
Expected: All tests pass

**Step 2: Build WASM component**

Run: `cargo build -p a2a-wasm-component --target wasm32-wasip2`
Expected: Success

**Step 3: Run clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: No warnings

---

## Summary

| Task | Description | Crate |
|------|-------------|-------|
| 1 | Add TaskId newtype | a2a-types |
| 2 | Add Binding/ProtocolBinding types | a2a-types |
| 3 | Add AgentInterface type | a2a-types |
| 4 | Refactor error types | a2a-client |
| 5 | Add binding module | a2a-client |
| 6 | Add REST module | a2a-client |
| 7 | Extract JSON-RPC handler | a2a-wasm-component |
| 8 | Simplify server routing | a2a-wasm-component |
| 9 | Add REST handler | a2a-wasm-component |
| 10 | Update documentation | a2a-wasm-component |
| 11 | Full test suite | all |

**Note:** This plan covers the server-side REST binding and client REST helpers. The full client refactoring (auto-negotiation, unified API) is deferred to a follow-up plan once the server is working.
