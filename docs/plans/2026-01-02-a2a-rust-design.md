# A2A Protocol Rust Implementation Design

## Overview

Rust implementation of the [Agent2Agent (A2A) Protocol](https://a2a-protocol.org/), targeting Linux x86_64 and WASM (WASIP2/P3). HTTP-only transport (no gRPC).

## Crate Structure

```
a2a-rust/
├── crates/
│   ├── a2a-types/              # Generated protocol types
│   ├── a2a-transport/          # HttpClient + HttpServer traits
│   ├── a2a-transport-wasi/     # wasi:http implementation
│   ├── a2a-transport-reqwest/  # Native client (later)
│   ├── a2a-transport-hyper/    # Native server (later)
│   ├── a2a-client/             # Client API
│   ├── a2a-server/             # Server API + TaskStore
│   └── a2a-wasm-component/     # WIT exports for agent-compose:a2a
```

## Crate Details

### `a2a-types`

Generated types from A2A JSON Schema using `typify`.

**Dependencies:** `serde`, `serde_json` only.

**Generation approach:**
```rust
// build.rs
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let schema = std::fs::read_to_string("schema/a2a.json").unwrap();
    let schema: schemars::schema::RootSchema = serde_json::from_str(&schema).unwrap();

    let mut settings = TypeSpaceSettings::default();
    settings.with_derive("Clone".into());
    settings.with_derive("PartialEq".into());

    let type_space = TypeSpace::new(&settings).add_root_schema(schema).unwrap();
    // write to OUT_DIR...
}
```

**Key types:**
- `AgentCard`, `AgentSkill`, `AgentCapabilities`
- `Task`, `TaskState`, `TaskStatus`
- `Message`, `Part` (TextPart, FilePart, DataPart)
- `Artifact`
- `SendMessageRequest`, `SendMessageResponse`, etc.
- JSON-RPC error types

**Manual additions:** Helper constructors, ergonomic `impl` blocks.

---

### `a2a-transport`

Defines transport traits. No HTTP implementation, just abstractions.

**Dependencies:** `a2a-types`, `thiserror`, `futures-core`

```rust
use std::future::Future;
use futures_core::Stream;

/// HTTP client trait for making outgoing requests
pub trait HttpClient: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    fn request(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> + Send;

    fn request_stream(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = Result<impl Stream<Item = Result<Bytes, Self::Error>>, Self::Error>> + Send;
}

/// HTTP server trait for handling incoming requests
pub trait HttpServer: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    fn serve<H, F>(
        &self,
        handler: H,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        H: Fn(HttpRequest) -> F + Send + Sync,
        F: Future<Output = HttpResponse> + Send;
}

/// Simple HTTP types (not tied to any HTTP crate)
pub struct HttpRequest {
    pub method: Method,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}
```

---

### `a2a-transport-wasi`

WASI implementation using pre-generated bindings from `wasi` crate.

**Dependencies:** `a2a-transport`, `wasi` (or `wasip3` for async)

```rust
use wasi::http::outgoing_handler;
use wasi::http::types::*;

pub struct WasiHttpClient;

impl HttpClient for WasiHttpClient {
    type Error = WasiError;

    async fn request(&self, req: HttpRequest) -> Result<HttpResponse, Self::Error> {
        // Use wasi::http::outgoing_handler::handle()
        // Poll incoming-body stream
        // Convert to HttpResponse
    }

    async fn request_stream(&self, req: HttpRequest) -> Result<impl Stream<...>, Self::Error> {
        // Return chunked body stream
    }
}

pub struct WasiHttpServer;

impl HttpServer for WasiHttpServer {
    type Error = WasiError;

    async fn serve<H, F>(&self, handler: H) -> Result<(), Self::Error>
    where
        H: Fn(HttpRequest) -> F + Send + Sync,
        F: Future<Output = HttpResponse> + Send,
    {
        // Use wasi:http/incoming-handler
    }
}
```

---

### `a2a-transport-reqwest` (later)

Native Linux HTTP client.

**Dependencies:** `a2a-transport`, `reqwest`

```rust
pub struct ReqwestClient {
    client: reqwest::Client,
}

impl HttpClient for ReqwestClient { ... }
```

---

### `a2a-transport-hyper` (later)

Native Linux HTTP server.

**Dependencies:** `a2a-transport`, `hyper`, `tokio`

```rust
pub struct HyperServer {
    addr: SocketAddr,
}

impl HttpServer for HyperServer { ... }
```

---

### `a2a-client`

Client API for interacting with A2A agents.

**Dependencies:** `a2a-types`, `a2a-transport`, `thiserror`, `futures-core`

```rust
pub struct Client<T: HttpClient> {
    transport: T,
    agent_card: AgentCard,
}

impl<T: HttpClient> Client<T> {
    /// Create client from a discovered agent card
    pub fn new(transport: T, agent_card: AgentCard) -> Self;

    /// Discover an agent by fetching /.well-known/agent.json
    pub async fn discover(transport: T, base_url: &str) -> Result<Self, Error>;

    /// Send a message (non-streaming)
    pub async fn send_message(&self, message: Message) -> Result<SendMessageResponse, Error>;

    /// Send a message with streaming response
    pub async fn send_message_stream(
        &self,
        message: Message,
    ) -> Result<impl Stream<Item = Result<StreamEvent, Error>>, Error>;

    /// Get task by ID
    pub async fn get_task(&self, task_id: &str) -> Result<Task, Error>;

    /// Cancel a task
    pub async fn cancel_task(&self, task_id: &str) -> Result<Task, Error>;

    /// Subscribe to task updates (streaming)
    pub async fn subscribe_to_task(
        &self,
        task_id: &str,
    ) -> Result<impl Stream<Item = Result<StreamEvent, Error>>, Error>;
}
```

**Internal helpers:**
- `build_jsonrpc_request()` - constructs JSON-RPC 2.0 envelope
- `parse_jsonrpc_response()` - handles success/error responses
- `SseParser` - parses SSE frames into `StreamEvent`

---

### `a2a-server`

Server API for building A2A agents.

**Dependencies:** `a2a-types`, `a2a-transport`, `thiserror`, `futures-core`

```rust
/// User-implemented handler for agent logic
pub trait AgentHandler: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    type EventStream: Stream<Item = StreamEvent> + Send;

    async fn handle_message(
        &self,
        message: Message,
        context: RequestContext,
    ) -> Result<SendMessageResponse, Self::Error>;

    async fn handle_message_stream(
        &self,
        message: Message,
        context: RequestContext,
    ) -> Result<Self::EventStream, Self::Error>;

    async fn handle_cancel(&self, task_id: &str) -> Result<Task, Self::Error>;
}

pub struct RequestContext {
    pub task_id: Option<String>,
    pub context_id: Option<String>,
}

/// Task storage trait
pub trait TaskStore: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn get(&self, task_id: &str) -> Result<Option<Task>, Self::Error>;
    async fn save(&self, task: &Task) -> Result<(), Self::Error>;
    async fn list(&self, filter: TaskFilter) -> Result<Vec<Task>, Self::Error>;
    async fn delete(&self, task_id: &str) -> Result<(), Self::Error>;
}

/// Default in-memory implementation
pub struct InMemoryTaskStore { ... }

/// A2A server
pub struct Server<H: AgentHandler, S: TaskStore, T: HttpServer> {
    handler: H,
    store: S,
    transport: T,
    agent_card: AgentCard,
}

impl<H, S, T> Server<H, S, T>
where
    H: AgentHandler,
    S: TaskStore,
    T: HttpServer,
{
    pub fn new(handler: H, store: S, transport: T, agent_card: AgentCard) -> Self;
    pub async fn serve(&self) -> Result<(), Error>;
}
```

**Routing:** `message/send`, `message/stream`, `tasks/get`, `tasks/cancel`, `tasks/subscribe`, `/.well-known/agent.json`

---

### `a2a-wasm-component`

WASM component with `agent-compose:a2a` WIT interface.

**Dependencies:** `a2a-client`, `a2a-server`, `a2a-transport-wasi`, `wit-bindgen`

**WIT interface:**
```wit
package agent-compose:a2a@0.1.0;

interface types {
    enum role { user, agent }

    record message {
        role: role,
        content: string,
        context-id: option<string>,
    }

    enum task-status {
        submitted, working, completed, failed, cancelled,
    }

    record task {
        id: string,
        status: task-status,
        context-id: option<string>,
        messages: list<message>,
        artifacts: list<string>,
    }

    variant send-response {
        message(message),
        task(task),
    }

    record pending-item {
        id: string,
        from: string,
        content: string,
        context-id: option<string>,
        task-id: option<string>,
    }
}

interface client {
    use types.{message, task, task-status, send-response};

    send-message: func(to: string, content: string, context-id: option<string>) -> send-response;
    get-task: func(task-id: string) -> option<task>;
    list-tasks: func(context-id: option<string>, status: option<task-status>) -> list<task>;
    cancel-task: func(task-id: string) -> option<task>;
}

interface server {
    use types.{pending-item};

    get-inbox: func(agent-name: string) -> list<pending-item>;
    acknowledge: func(agent-name: string, item-id: string) -> bool;
}
```

**Implementation:** Delegates to `a2a-client` and `a2a-server` crates.

---

## Error Handling

Per-crate error types using `thiserror`:

```rust
// a2a-types/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid message: {0}")]
    InvalidMessage(String),
    #[error("Invalid task state: {0}")]
    InvalidTaskState(String),
}

// a2a-transport/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: status {status}")]
    Http { status: u16, body: Option<String> },
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Timeout")]
    Timeout,
}

// a2a-client/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Types(#[from] a2a_types::Error),
    #[error(transparent)]
    Transport(#[from] a2a_transport::Error),
    #[error("JSON-RPC error {code}: {message}")]
    JsonRpc { code: i32, message: String, data: Option<serde_json::Value> },
    #[error("Agent not found at {0}")]
    AgentNotFound(String),
    #[error("Task not found: {0}")]
    TaskNotFound(String),
}

// a2a-server/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Types(#[from] a2a_types::Error),
    #[error(transparent)]
    Transport(#[from] a2a_transport::Error),
    #[error(transparent)]
    Store(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Handler error: {0}")]
    Handler(String),
}
```

---

## Key Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Types generation | JSON Schema + typify | JSON-native for HTTP/JSON-RPC transport |
| Async runtime | Native async traits (Rust 1.75+) | WASIP3 + tokio compatibility |
| Streaming | `impl Stream<Item = Event>` | Idiomatic Rust, composable |
| Error handling | Per-crate with thiserror | Focused errors, easy composition |
| HTTP abstraction | Separate trait + impl crates | Minimal deps for WASM |
| Task storage | Trait + InMemoryTaskStore | Flexibility with sensible default |
| MSRV | Latest stable | Maximum feature availability |

---

## Implementation Order

1. `a2a-types` - Foundation, unblocks everything
2. `a2a-transport` - Define HttpClient + HttpServer traits
3. `a2a-transport-wasi` - WASM support (primary target)
4. `a2a-client` - Test against existing A2A servers
5. `a2a-server` - Handler trait, routing, task store
6. `a2a-wasm-component` - WIT exports wrapping client + server
7. `a2a-transport-reqwest` - Native client (later)
8. `a2a-transport-hyper` - Native server (later)

---

## Targets

- `wasm32-wasip2` / `wasm32-wasip3` - Primary target
- `x86_64-unknown-linux-gnu` - Native Linux support

---

## Testing Strategy

Cross-implementation testing using existing A2A test suites. No mock servers/clients in Rust - real implementations test each other.

### Test Layers

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Unit Tests (Rust only)                                   │
│    - Type serialization/deserialization round-trips         │
│    - JSON-RPC envelope building/parsing                     │
│    - SSE frame parsing                                      │
│    - Error type mapping                                     │
├─────────────────────────────────────────────────────────────┤
│ 2. Cross-Implementation Tests                               │
│                                                             │
│    Rust Client testing:                                     │
│    ┌─────────────────┐     ┌─────────────────┐             │
│    │  Rust Client    │────▶│  JS SUT Agent   │             │
│    └─────────────────┘     └─────────────────┘             │
│                                                             │
│    Rust Server testing:                                     │
│    ┌─────────────────┐     ┌─────────────────┐             │
│    │  Python Client  │────▶│  Rust Server    │             │
│    └─────────────────┘     └─────────────────┘             │
│    ┌─────────────────┐     ┌─────────────────┐             │
│    │  JS Client      │────▶│  Rust Server    │             │
│    └─────────────────┘     └─────────────────┘             │
└─────────────────────────────────────────────────────────────┘
```

### External Test Resources

**JavaScript TCK (Test Compatibility Kit):**
- Location: `a2a-js/tck/agent/`
- SUT Agent runs on `http://localhost:41241`
- Endpoints: `/a2a/jsonrpc` (JSON-RPC), `/a2a/rest` (REST)
- Agent card: `/.well-known/agent-card.json`
- Launch: `npm run tck:sut-agent`

**Python Test Suite:**
- `tests/integration/test_client_server_integration.py` - Client against real servers
- `tests/e2e/push_notifications/agent_app.py` - Complete working agent
- `tests/test_types.py` - All type fixtures and validation

**Go Test Suite:**
- `e2e/jsonrpc_test.go` - E2E streaming validation
- `a2a/json_test.go` - Serialization round-trip tests

### Test Execution

**Testing Rust client:**
1. Start JS SUT Agent: `cd a2a-js && npm run tck:sut-agent`
2. Run Rust client tests against `http://localhost:41241`

**Testing Rust server:**
1. Start Rust server on a port
2. Run Python integration tests: `pytest tests/integration/test_client_server_integration.py`
3. Run JS e2e tests adapted to point at Rust server

### Unit Test Coverage (Rust)

| Crate | Test Focus |
|-------|------------|
| `a2a-types` | Serde round-trips, type validation, fixture compatibility with Python/Go/JS |
| `a2a-transport` | HttpRequest/HttpResponse building |
| `a2a-client` | JSON-RPC envelope construction, SSE parsing |
| `a2a-server` | Request routing, TaskStore trait |

---

## References

- [A2A Protocol Specification](https://a2a-protocol.org/latest/specification/)
- [A2A Protocol Definitions](https://a2a-protocol.org/latest/definitions/)
- [a2a-go](https://github.com/a2aproject/a2a-go)
- [a2a-python](https://github.com/a2aproject/a2a-python)
- [a2a-js](https://github.com/a2aproject/a2a-js) (includes TCK)
- [wasi crate](https://docs.rs/wasi)
- [wasip3 crate](https://crates.io/crates/wasip3)
- [typify](https://github.com/oxidecomputer/typify)
