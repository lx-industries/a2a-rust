# WASM Server Integration Tests Design

**Goal:** Test the WASM component's HTTP server (`wasi:http/incoming-handler`) using the Python A2A SDK client.

**Architecture:** Rust test harness runs WASM component as HTTP server, Python client tests against it.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Integration Test                             │
├─────────────────────────────────────────────────────────────────┤
│  1. Rust test starts WasmServer (port 9998)                     │
│  2. Rust test spawns Python client tests via uv                 │
│  3. Python tests use A2A SDK to call WASM server                │
│  4. WasmServer routes HTTP → incoming-handler → mock agent      │
└─────────────────────────────────────────────────────────────────┘

┌──────────────┐     HTTP      ┌──────────────────────────────────┐
│ Python A2A   │ ──────────►   │ WasmServer (Rust test harness)   │
│ SDK Client   │               │                                  │
└──────────────┘               │  ┌────────────────────────────┐  │
                               │  │ WASM Component             │  │
                               │  │  - incoming-handler        │  │
                               │  │  - imports mock agent      │  │
                               │  └────────────────────────────┘  │
                               │                                  │
                               │  ┌────────────────────────────┐  │
                               │  │ StatefulMockAgent          │  │
                               │  │  - task storage (HashMap)  │  │
                               │  │  - on_message → create     │  │
                               │  │  - on_get_task → retrieve  │  │
                               │  │  - on_cancel_task → cancel │  │
                               │  └────────────────────────────┘  │
                               └──────────────────────────────────┘
```

---

## Components

### WasmServer (`tests/common/wasm_server.rs`)

```rust
pub struct WasmServer {
    handle: tokio::task::JoinHandle<()>,
    pub url: String,
}

impl WasmServer {
    pub async fn start() -> Self {
        // 1. Load WASM component (same as WasmRunner)
        // 2. Create Linker with:
        //    - wasmtime_wasi (base WASI)
        //    - wasmtime_wasi_http (HTTP types)
        //    - StatefulMockAgent (a2a:protocol/agent)
        // 3. Use wasmtime_wasi_http::proxy to serve incoming-handler
        // 4. Bind to localhost:9998
        // 5. Spawn server task, return handle
    }
}

impl Drop for WasmServer {
    fn drop(&mut self) {
        self.handle.abort();
    }
}
```

Port 9998 avoids collision with Python helloworld server (9999).

### StatefulMockAgent

```rust
struct TaskStore {
    tasks: HashMap<String, Task>,
}

impl TaskStore {
    fn create_task(&mut self, message: &Message) -> Task {
        let id = uuid::Uuid::new_v4().to_string();
        let task = Task {
            id: id.clone(),
            context_id: message.context_id.clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            status: TaskStatus {
                state: TaskState::Completed,
                message: Some(Message {
                    role: Role::Agent,
                    parts: vec![Part::Text(TextPart { text: "Hello World".into() })],
                    ..
                }),
                timestamp: Some(now_iso8601()),
            },
            history: None,
            artifacts: None,
        };
        self.tasks.insert(id, task.clone());
        task
    }

    fn get_task(&self, id: &str) -> Option<Task> { ... }

    fn cancel_task(&mut self, id: &str) -> Option<Task> {
        // Set state to Canceled, return task
    }
}
```

### Python Test Fixture (`tests/fixtures/wasm_server_tests/`)

```
wasm_server_tests/
├── pyproject.toml      # a2a-sdk dependency, pytest
├── test_wasm_server.py # Test cases
└── conftest.py         # pytest fixtures (server URL from env)
```

`pyproject.toml`:
```toml
[project]
name = "wasm-server-tests"
dependencies = [
    "a2a-sdk @ git+https://github.com/muscariello/a2a-python@a2a_proto_refactor",
    "pytest",
    "pytest-asyncio",
]
```

### Rust Integration Test

```rust
#[tokio::test]
async fn test_wasm_server_with_python_client() {
    // 1. Start WASM server
    let server = WasmServer::start().await;

    // 2. Run Python tests via uv
    let status = Command::new("uv")
        .args(["run", "pytest", "-v"])
        .current_dir("tests/fixtures/wasm_server_tests")
        .env("WASM_SERVER_URL", &server.url)
        .status()
        .expect("Failed to run pytest");

    assert!(status.success(), "Python tests failed");
}
```

---

## Test Coverage

| Test | Endpoint | Validates |
|------|----------|-----------|
| `test_agent_card_discovery` | `GET /.well-known/agent-card.json` | Discovery endpoint |
| `test_send_message_jsonrpc` | `POST /` (JSON-RPC) | SendMessage routing |
| `test_send_message_rest` | `POST /v1/message:send` | REST binding |
| `test_get_task_jsonrpc` | `POST /` (JSON-RPC) | GetTask routing |
| `test_get_task_rest` | `GET /v1/tasks/{id}` | REST binding |
| `test_get_task_not_found` | Both bindings | 404/error handling |
| `test_cancel_task_jsonrpc` | `POST /` (JSON-RPC) | CancelTask routing |
| `test_cancel_task_rest` | `POST /v1/tasks/{id}:cancel` | REST binding |
| `test_invalid_method` | `POST /` | JSON-RPC error -32601 |

---

## Dependencies

Rust (already available):
- `wasmtime`, `wasmtime-wasi`, `wasmtime-wasi-http`
- `hyper` (for HTTP server)
- `tokio`

Python:
- `a2a-sdk` (proto_refactor branch)
- `pytest`, `pytest-asyncio`
