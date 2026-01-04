# A2A Server Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement the server as an HTTP handler that exports `wasi:http/incoming-handler`, handling JSON-RPC routing and delegating to an imported agent interface.

**Architecture:** Replace stub `server` interface with `wasi:http/incoming-handler` export. Component handles HTTP layer (routing, JSON-RPC) while agent interface provides logic.

**Tech Stack:** wit-bindgen, wasm32-wasip2 target, wasi:http, serde_json

---

## Task 1: Revert Broken Pass-Through Commits

**Files:** None (git operations only)

**Step 1: Check current commits**

Run: `git log --oneline -10`

**Step 2: Identify commits to revert**

The following commits implemented the incorrect pass-through architecture:
- `e13e0b2` - add agent interface to WIT
- `b79e278` - implement server as pass-through
- `491e4d6` - update module docs
- `87bb747` - add server module test placeholder

**Step 3: Revert commits**

Run: `git revert --no-commit 87bb747 491e4d6 b79e278 e13e0b2`

Note: We revert in reverse order. If commits are not linear, use individual reverts.

**Step 4: Commit the revert**

```bash
git commit -m "revert: remove pass-through server architecture

The pass-through design was incorrect. Server should be an HTTP handler
that exports wasi:http/incoming-handler, not a thin wrapper.

Reverts:
- e13e0b2: add agent interface to WIT
- b79e278: implement server as pass-through
- 491e4d6: update module docs
- 87bb747: add server module test placeholder"
```

---

## Task 2: Update WIT with Correct Architecture

**Files:**
- Modify: `crates/a2a-wasm-component/wit/a2a.wit`

**Step 1: Update agent interface**

Keep the agent interface but add `get-agent-card`:

Find the `interface agent` section and replace with:

```wit
/// Agent interface imported from host - provides actual agent logic
interface agent {
    use types.{task, message-send-params, send-response, error};

    /// Get agent card as JSON string
    get-agent-card: func() -> result<string, error>;

    /// Process incoming message (blocking)
    on-message: func(params: message-send-params) -> result<send-response, error>;

    /// Retrieve task by ID
    on-get-task: func(id: string, history-length: option<u32>) -> result<option<task>, error>;

    /// Handle cancellation
    on-cancel-task: func(id: string) -> result<option<task>, error>;
}
```

**Step 2: Remove server interface export, add HTTP handler**

Replace the world declaration:

```wit
world a2a-component {
    import wasi:http/outgoing-handler;
    import agent;

    export wasi:http/incoming-handler;
    export client;
}
```

Note: The `server` interface definition can remain in the file (useful for documentation) but is no longer exported.

**Step 3: Add deps folder with WASI HTTP**

Create `crates/a2a-wasm-component/wit/deps/` and add wasi:http WIT files.

Run:
```bash
mkdir -p crates/a2a-wasm-component/wit/deps
# Download WASI HTTP WIT package (use wit-deps or manual copy)
```

**Step 4: Verify WIT parses**

Run: `cargo build -p a2a-wasm-component 2>&1 | head -30`

**Step 5: Commit**

```bash
git add crates/a2a-wasm-component/wit/
git commit -m "feat(a2a-wasm-component): update WIT for HTTP handler architecture

- Add get-agent-card to agent interface
- Remove server export from world
- Add wasi:http/incoming-handler export
- Import wasi:http/outgoing-handler (already used by client)"
```

---

## Task 3: Add JSON-RPC Types

**Files:**
- Create: `crates/a2a-wasm-component/src/jsonrpc.rs`

**Step 1: Create JSON-RPC types**

```rust
//! JSON-RPC 2.0 types for A2A protocol.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC 2.0 request.
#[derive(Debug, Deserialize)]
pub struct Request {
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
```

**Step 2: Add module to lib.rs**

Add `mod jsonrpc;` to `lib.rs`.

**Step 3: Run tests**

Run: `cargo test -p a2a-wasm-component --lib`

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/src/jsonrpc.rs crates/a2a-wasm-component/src/lib.rs
git commit -m "feat(a2a-wasm-component): add JSON-RPC types for HTTP handler"
```

---

## Task 4: Implement HTTP Handler Scaffold

**Files:**
- Modify: `crates/a2a-wasm-component/src/server.rs`

**Step 1: Replace server.rs with HTTP handler**

```rust
//! HTTP handler implementation for A2A server.
//!
//! Exports wasi:http/incoming-handler to handle incoming A2A requests.

use crate::bindings::exports::wasi::http::incoming_handler::Guest;
use crate::bindings::wasi::http::types::{
    Headers, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};
use crate::jsonrpc::{self, Response};

/// Implement the wasi:http/incoming-handler interface.
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

    match (method, path.as_str()) {
        (wasi::http::types::Method::Get, "/.well-known/agent.json") => {
            handle_agent_card()
        }
        (wasi::http::types::Method::Post, "/" | "") => {
            let body = read_request_body(&request)?;
            handle_jsonrpc(&body)
        }
        (wasi::http::types::Method::Options, _) => {
            // CORS preflight
            Ok((204, "text/plain", vec![]))
        }
        _ => Err((404, "Not Found".to_string())),
    }
}

fn handle_agent_card() -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::bindings::a2a::protocol::agent;

    match agent::get_agent_card() {
        Ok(card_json) => Ok((200, "application/json", card_json.into_bytes())),
        Err(e) => {
            let response = Response::internal_error(serde_json::Value::Null, e.message);
            let body = serde_json::to_vec(&response).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
}

fn handle_jsonrpc(body: &[u8]) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    // Parse JSON-RPC request
    let request: jsonrpc::Request = match serde_json::from_slice(body) {
        Ok(r) => r,
        Err(e) => {
            let response = Response::error(
                serde_json::Value::Null,
                jsonrpc::PARSE_ERROR,
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
            // Streaming not yet implemented
            Response::error(request.id.clone(), -32601, "Streaming not implemented")
        }
        _ => Response::method_not_found(request.id.clone()),
    };

    let body = serde_json::to_vec(&response).unwrap_or_default();
    Ok((200, "application/json", body))
}

fn handle_message_send(request: &jsonrpc::Request) -> Response {
    use crate::bindings::a2a::protocol::agent;
    use crate::convert;

    // Parse params
    let params: a2a_types::MessageSendParams = match serde_json::from_value(request.params.clone())
    {
        Ok(p) => p,
        Err(e) => return Response::invalid_params(request.id.clone(), e.to_string()),
    };

    // Convert to WIT types
    let wit_params = match convert::message_send_params_to_wit(&params) {
        Ok(p) => p,
        Err(e) => return Response::invalid_params(request.id.clone(), e),
    };

    // Call agent
    match agent::on_message(&wit_params) {
        Ok(response) => {
            let a2a_response = convert::send_response_from_wit(&response);
            Response::success(request.id.clone(), a2a_response)
        }
        Err(e) => Response::error(request.id.clone(), e.code, e.message),
    }
}

fn handle_tasks_get(request: &jsonrpc::Request) -> Response {
    use crate::bindings::a2a::protocol::agent;
    use crate::convert;

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

fn handle_tasks_cancel(request: &jsonrpc::Request) -> Response {
    use crate::bindings::a2a::protocol::agent;
    use crate::convert;

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
                data.extend_from_slice(&chunk);
            }
            Err(wasi::io::streams::StreamError::Closed) => break,
            Err(_) => return Err((500, "Failed to read body".to_string())),
        }
    }

    Ok(data)
}

fn send_response(response_out: ResponseOutparam, status: u16, content_type: &str, body: Vec<u8>) {
    let headers = Headers::new();
    headers
        .set(&"Content-Type".to_string(), &[content_type.as_bytes().to_vec()])
        .ok();
    headers
        .set(
            &"Content-Length".to_string(),
            &[body.len().to_string().into_bytes()],
        )
        .ok();

    let response = OutgoingResponse::new(headers);
    response.set_status_code(status).ok();

    let outgoing_body = response.body().unwrap();
    ResponseOutparam::set(response_out, Ok(response));

    // Write body
    let stream = outgoing_body.write().unwrap();
    stream.blocking_write_and_flush(&body).ok();
    drop(stream);

    OutgoingBody::finish(outgoing_body, None).ok();
}
```

**Step 2: Update lib.rs exports**

Update `lib.rs` to properly wire up the HTTP handler export.

**Step 3: Build and check errors**

Run: `cargo build -p a2a-wasm-component 2>&1 | head -50`

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/src/server.rs crates/a2a-wasm-component/src/lib.rs
git commit -m "feat(a2a-wasm-component): implement HTTP handler for A2A server

- Export wasi:http/incoming-handler
- Route GET /.well-known/agent.json to agent card
- Route POST / to JSON-RPC handler
- Implement message/send, tasks/get, tasks/cancel
- Return proper JSON-RPC errors"
```

---

## Task 5: Add Convert Functions for JSON ↔ WIT

**Files:**
- Modify: `crates/a2a-wasm-component/src/convert.rs`

**Step 1: Add missing conversion functions**

Add to `convert.rs`:

```rust
/// Convert a2a-types MessageSendParams to WIT MessageSendParams.
pub fn message_send_params_to_wit(
    params: &a2a_types::MessageSendParams,
) -> Result<crate::bindings::a2a::protocol::types::MessageSendParams, String> {
    // Implementation using existing helpers
    todo!()
}

/// Convert WIT SendResponse to a2a-types SendResponse.
pub fn send_response_from_wit(
    response: &crate::bindings::a2a::protocol::types::SendResponse,
) -> a2a_types::SendResponse {
    // Implementation
    todo!()
}

/// Convert WIT Task to a2a-types Task.
pub fn task_from_wit(
    task: &crate::bindings::a2a::protocol::types::Task,
) -> a2a_types::Task {
    // Implementation
    todo!()
}
```

**Step 2: Implement conversions**

Use existing conversion patterns from the file.

**Step 3: Run tests**

Run: `cargo test -p a2a-wasm-component --lib`

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/src/convert.rs
git commit -m "feat(a2a-wasm-component): add JSON to WIT conversion functions"
```

---

## Task 6: Update Module Documentation

**Files:**
- Modify: `crates/a2a-wasm-component/src/lib.rs`

**Step 1: Update module documentation**

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
//! # Architecture
//!
//! ```text
//! External A2A Client
//!     │ HTTP
//!     ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │ WASM Host Runtime (e.g., Wassette)                      │
//! │   • TCP/TLS termination                                 │
//! │   • Provides agent interface implementation             │
//! └─────────────────────────┬───────────────────────────────┘
//!                           │
//! ┌─────────────────────────▼───────────────────────────────┐
//! │ a2a-wasm-component                                      │
//! │   export wasi:http/incoming-handler                     │
//! │   import agent { get-agent-card, on-message, ... }      │
//! │   export client { send-message, get-task, cancel-task } │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! # HTTP Endpoints
//!
//! - `GET /.well-known/agent.json` - Agent card discovery
//! - `POST /` - JSON-RPC (message/send, tasks/get, tasks/cancel)
//!
//! # Limitations
//!
//! - Only `TextPart` is supported; `FilePart` and `DataPart` return errors
//! - Streaming (message/stream) is not yet implemented
//! - Metadata fields are not supported (deferred)
```

**Step 2: Build**

Run: `cargo build -p a2a-wasm-component`

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/src/lib.rs
git commit -m "docs(a2a-wasm-component): update module docs for HTTP handler architecture"
```

---

## Task 7: Build and Test WASM Target

**Files:** None (verification only)

**Step 1: Build for wasm32-wasip2**

Run: `cargo build -p a2a-wasm-component --target wasm32-wasip2`

**Step 2: Run clippy**

Run: `cargo clippy -p a2a-wasm-component --target wasm32-wasip2 -- -D warnings`

**Step 3: Run unit tests**

Run: `cargo test -p a2a-wasm-component --lib`

**Step 4: Check component size**

Run: `ls -lh target/wasm32-wasip2/debug/a2a_wasm_component.wasm`

---

## Task 8: Update Integration Test Harness

**Files:**
- Modify: `crates/a2a-wasm-component/tests/common/wasm_runner.rs`

**Step 1: Add agent interface mock to linker**

The test harness needs to provide an agent implementation:

```rust
// Add to WasmRunner::new()

// Provide mock agent implementation
linker.func_wrap(
    "a2a:protocol/agent",
    "get-agent-card",
    |_caller: Caller<'_, TestState>| -> Result<(Result<String, Error>,), wasmtime::Error> {
        Ok((Ok(r#"{"name": "test-agent"}"#.to_string()),))
    },
)?;

// ... similar for on-message, on-get-task, on-cancel-task
```

**Step 2: Add server test methods**

```rust
impl WasmRunner {
    /// Send an HTTP request to the server handler.
    pub async fn http_request(
        &mut self,
        method: &str,
        path: &str,
        body: Option<&[u8]>,
    ) -> Result<(u16, String, Vec<u8>), String> {
        // Call wasi:http/incoming-handler::handle
        todo!()
    }
}
```

**Step 3: Run integration tests**

Run: `cargo test -p a2a-wasm-component --test integration_test -- --test-threads=1`

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/tests/
git commit -m "test(a2a-wasm-component): update test harness for HTTP handler"
```

---

## Task 9: Final Verification

**Files:** None (verification only)

**Step 1: Run all tests**

Run: `cargo test -p a2a-wasm-component`

**Step 2: Run clippy**

Run: `cargo clippy -p a2a-wasm-component -- -D warnings`

**Step 3: Build release**

Run: `cargo build -p a2a-wasm-component --target wasm32-wasip2 --release`

**Step 4: Check release size**

Run: `ls -lh target/wasm32-wasip2/release/a2a_wasm_component.wasm`

---

## Summary

After completing all tasks:

1. **Reverted** incorrect pass-through architecture
2. **WIT changes**: Added HTTP handler export, updated agent interface
3. **HTTP handler**: Implements routing and JSON-RPC
4. **JSON-RPC**: Proper parsing and error responses
5. **Conversions**: JSON ↔ WIT type conversions
6. **Tests**: Updated for new architecture
7. **WASM build**: Verified on wasm32-wasip2 target

The component now:
- Exports `wasi:http/incoming-handler` for A2A server
- Handles `GET /.well-known/agent.json` for discovery
- Handles `POST /` for JSON-RPC operations
- Imports `agent` interface for actual logic
