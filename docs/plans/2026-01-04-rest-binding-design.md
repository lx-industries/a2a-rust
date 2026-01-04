# HTTP+JSON/REST Binding Design

Add HTTP+JSON/REST protocol binding alongside existing JSON-RPC, with auto-negotiating client.

## Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Scope | Both client and server | Full REST support across the stack |
| Endpoints | Core + Agent card | Matches current JSON-RPC functionality |
| Coexistence | Path-based routing | Spec-compliant; agent card declares both bindings |
| Path syntax | Google-style `:action` | Follow spec exactly (`/v1/message:send`) |
| Client API | Auto-negotiate with preference | Clean unified API; binding is implementation detail |
| Default preference | JSON-RPC first | Already implemented and proven |
| Error handling | Layered errors | Unified handling with optional protocol-specific details |
| Server architecture | Separate handler modules | `handler.rs` routes to `jsonrpc.rs` or `rest.rs` |
| Agent card path | `/.well-known/agent-card.json` | Current spec (not old `/.well-known/agent.json`) |

## Architecture Overview

### Protocol Binding URLs

- **JSON-RPC:** `POST /` (unchanged)
- **REST:** `/v1/message:send`, `/v1/tasks/{id}`, `/v1/tasks/{id}:cancel`, `/v1/agentCard`

### Agent Card Declaration

```json
{
  "supportedInterfaces": [
    { "protocolBinding": "JSONRPC", "url": "https://example.com/" },
    { "protocolBinding": "HTTP+JSON", "url": "https://example.com/v1" }
  ]
}
```

## Client Architecture

### Unified API

```rust
// Construction - discovers agent and auto-selects binding
let client = Client::new(transport, "https://agent.example.com").await?;

// Unified API - binding is an implementation detail
let task = client.send_message(message).await?;
let task = client.get_task(task_id).await?;
let task = client.cancel_task(task_id).await?;
let card = client.agent_card();  // cached from discovery
```

### Auto-Negotiation Flow

1. `Client::new()` fetches `/.well-known/agent-card.json`
2. Parses `supportedInterfaces` array
3. Selects first matching binding (preference: JSON-RPC > REST)
4. Stores selected binding URL and type
5. Subsequent calls use selected binding

### Builder Configuration

```rust
// Override preference order
let client = Client::builder(transport, base_url)
    .prefer(&[Binding::Rest, Binding::JsonRpc])
    .build()
    .await?;

// Force specific binding (skip negotiation)
let client = Client::builder(transport, base_url)
    .binding(Binding::JsonRpc)
    .build()
    .await?;
```

### Internal Structure

```rust
pub struct Client<T: HttpClient> {
    transport: T,
    agent_card: AgentCard,
    binding: SelectedBinding,
}

enum SelectedBinding {
    JsonRpc { url: Url },
    Rest { url: Url },
}
```

## Server Architecture (WASM Component)

### Module Structure

```
a2a-wasm-component/src/
├── lib.rs           # WIT bindings, Component struct
├── handler.rs       # Top-level HTTP routing
├── jsonrpc.rs       # JSON-RPC protocol handling
├── rest.rs          # REST protocol handling (new)
├── convert.rs       # Type conversions
└── ...
```

### Routing Logic (handler.rs)

```rust
fn handle_request(request: IncomingRequest) -> Result<...> {
    let method = request.method();
    let path = request.path_with_query().unwrap_or_default();

    // Agent card discovery
    if path == "/.well-known/agent-card.json" && method == Method::Get {
        return handle_agent_card();
    }

    // REST binding: /v1/* paths
    if path.starts_with("/v1/") {
        return rest::handle(method, &path, &request);
    }

    // JSON-RPC binding: POST /
    if (path == "/" || path.is_empty()) && method == Method::Post {
        return jsonrpc::handle(&request);
    }

    // CORS preflight
    if method == Method::Options {
        return Ok((204, "text/plain", vec![]));
    }

    Err((404, "Not Found".to_string()))
}
```

### REST Endpoint Handling (rest.rs)

```rust
pub fn handle(method: Method, path: &str, request: &IncomingRequest)
    -> Result<(u16, &'static str, Vec<u8>), (u16, String)>
{
    match (method, path) {
        (Method::Post, "/v1/message:send") => handle_send_message(request),

        (Method::Get, p) if is_task_get(p) => {
            let task_id = extract_task_id(p)?;
            handle_get_task(&task_id, request)
        }

        (Method::Post, p) if p.ends_with(":cancel") => {
            let task_id = extract_task_id_before_action(p)?;
            handle_cancel_task(&task_id)
        }

        (Method::Get, "/v1/agentCard") => handle_agent_card_extended(),

        _ => Err((404, "Not Found".to_string())),
    }
}
```

Both `jsonrpc.rs` and `rest.rs` call the same WIT agent interface.

## Error Handling

### Client Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("agent not found: {0}")]
    AgentNotFound(Url),

    #[error("no compatible binding: server supports {available:?}")]
    NoCompatibleBinding { available: Vec<Binding> },

    #[error("task not found: {0}")]
    TaskNotFound(TaskId),

    #[error("invalid params: {0}")]
    InvalidParams(ParamError),

    #[error("agent error: {message}")]
    Agent { message: String, source: ProtocolError },

    #[error("transport error: {0}")]
    Transport(#[from] a2a_transport::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid url: {0}")]
    InvalidUrl(#[from] url::ParseError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binding {
    JsonRpc,
    Rest,
}

#[derive(Debug, thiserror::Error)]
pub enum ParamError {
    #[error("missing required field: {field}")]
    MissingField { field: &'static str },

    #[error("invalid value for {field}: {reason}")]
    InvalidValue { field: &'static str, reason: String },
}

#[derive(Debug)]
pub enum ProtocolError {
    JsonRpc {
        code: JsonRpcErrorCode,
        message: String,
        data: Option<serde_json::Value>,
    },
    Rest {
        status: http::StatusCode,
        body: Option<serde_json::Value>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonRpcErrorCode {
    ParseError,            // -32700
    InvalidRequest,        // -32600
    MethodNotFound,        // -32601
    InvalidParams,         // -32602
    InternalError,         // -32603
    ServerError(i32),      // -32000 to -32099
    ApplicationError(i32), // other codes
}
```

### Server REST Responses

| Scenario | HTTP Status | Body |
|----------|-------------|------|
| Success | 200 | JSON result |
| Invalid JSON | 400 | `{"error": "Parse error: ..."}` |
| Invalid params | 400 | `{"error": "Invalid params: ..."}` |
| Task not found | 404 | `{"error": "Task not found"}` |
| Method not allowed | 405 | `{"error": "Method not allowed"}` |
| Internal error | 500 | `{"error": "Internal error: ..."}` |

## Types

### New Types in a2a-types

```rust
/// Strongly-typed task identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCard {
    pub name: String,
    pub description: Option<String>,
    pub protocol_version: String,
    pub supported_interfaces: Vec<AgentInterface>,
    // ... other fields from spec
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentInterface {
    pub protocol_binding: ProtocolBinding,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolBinding {
    #[serde(rename = "JSONRPC")]
    JsonRpc,
    #[serde(rename = "HTTP+JSON")]
    Rest,
    #[serde(rename = "GRPC")]
    Grpc,
}
```

## Files to Modify

| Crate | File | Change |
|-------|------|--------|
| `a2a-types` | `src/lib.rs` | Add `TaskId`, `AgentCard`, `AgentInterface`, `ProtocolBinding` |
| `a2a-client` | `src/lib.rs` | Refactor to unified API with auto-negotiation |
| `a2a-client` | `src/error.rs` | Strongly-typed layered errors |
| `a2a-client` | `src/rest.rs` | New REST binding implementation |
| `a2a-client` | `src/jsonrpc.rs` | Extract from current `rpc()` method |
| `a2a-wasm-component` | `src/handler.rs` | Top-level routing only |
| `a2a-wasm-component` | `src/jsonrpc.rs` | Extract JSON-RPC handling |
| `a2a-wasm-component` | `src/rest.rs` | New REST handling |

## References

- [A2A Protocol Specification](https://a2a-protocol.org/latest/specification/)
- [Agent Discovery](https://a2a-protocol.org/latest/topics/agent-discovery/)
