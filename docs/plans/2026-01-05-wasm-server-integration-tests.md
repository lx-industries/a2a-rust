# WASM Server Integration Tests Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Test the WASM component's HTTP server using Python A2A SDK client.

**Architecture:** Rust test harness runs WASM component as HTTP server via hyper, Python client tests against it. Uses wasmtime-wasi-http pattern from their documentation.

**Tech Stack:** wasmtime 29.0.1, wasmtime-wasi-http 29.0.1, hyper 1.8.1, tokio, Python a2a-sdk

---

## Task 1: Add hyper and uuid dependencies to dev-dependencies

**Files:**
- Modify: `crates/a2a-wasm-component/Cargo.toml`

**Step 1: Add dependencies**

Add to `[dev-dependencies]` section:

```toml
hyper = { version = "1.8", features = ["server", "http1"] }
http-body-util = "0.1"
uuid = { version = "1.0", features = ["v4"] }
```

**Step 2: Verify build**

Run: `cargo build -p a2a-wasm-component --tests`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/Cargo.toml
git commit -m "chore(a2a-wasm-component): add hyper, http-body-util, uuid dev-dependencies"
```

---

## Task 2: Create StatefulMockAgent with task storage

**Files:**
- Modify: `crates/a2a-wasm-component/tests/common/wasm_runner.rs`

**Step 1: Add imports and TaskStore struct**

Add after line 9 (after the existing imports):

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
```

Add after line 31 (after the type re-exports):

```rust
/// Shared task storage for stateful mock agent
#[derive(Default)]
pub struct TaskStore {
    tasks: HashMap<String, Task>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self { tasks: HashMap::new() }
    }

    pub fn create_task(&mut self, message: &Message) -> Task {
        let id = uuid::Uuid::new_v4().to_string();
        let context_id = message.context_id.clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        // Extract text from first part for echo
        let echo_text = message.parts.first()
            .and_then(|part| match part {
                Part::Text(t) => Some(format!("Hello World")),
                _ => None,
            })
            .unwrap_or_else(|| "Hello World".to_string());

        let task = Task {
            id: id.clone(),
            context_id,
            status: TaskStatus {
                state: TaskState::Completed,
                message: Some(Message {
                    role: Role::Agent,
                    parts: vec![Part::Text(TextPart { text: echo_text })],
                    message_id: Some(uuid::Uuid::new_v4().to_string()),
                    task_id: Some(id.clone()),
                    context_id: None,
                }),
                timestamp: Some(chrono::Utc::now().to_rfc3339()),
            },
            history: None,
            artifacts: None,
        };

        self.tasks.insert(id, task.clone());
        task
    }

    pub fn get_task(&self, id: &str) -> Option<Task> {
        self.tasks.get(id).cloned()
    }

    pub fn cancel_task(&mut self, id: &str) -> Option<Task> {
        self.tasks.get_mut(id).map(|task| {
            task.status.state = TaskState::Canceled;
            task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
            task.clone()
        })
    }
}
```

**Step 2: Add chrono dependency**

In `Cargo.toml` dev-dependencies:

```toml
chrono = "0.4"
```

**Step 3: Update TestState to include shared TaskStore**

Modify `TestState` struct (around line 34):

```rust
/// State held by the wasmtime Store, providing WASI and HTTP contexts.
pub struct TestState {
    wasi: WasiCtx,
    http: WasiHttpCtx,
    table: ResourceTable,
    pub task_store: Arc<Mutex<TaskStore>>,
}
```

**Step 4: Update AgentHost implementation to use TaskStore**

Replace the `AgentHost` implementation (lines 65-122) with:

```rust
impl AgentHost for TestState {
    async fn get_agent_card(&mut self) -> Result<String, A2aError> {
        Ok(r#"{"name":"test-wasm-agent","description":"Test WASM agent","url":"http://localhost:9998","version":"1.0.0","capabilities":{},"defaultInputModes":["text"],"defaultOutputModes":["text"],"skills":[]}"#.to_string())
    }

    async fn on_message(&mut self, params: MessageSendParams) -> Result<SendResponse, A2aError> {
        let task = self.task_store.lock().unwrap().create_task(&params.message);
        Ok(SendResponse::Task(task))
    }

    async fn on_get_task(
        &mut self,
        id: String,
        _history_length: Option<u32>,
    ) -> Result<Option<Task>, A2aError> {
        Ok(self.task_store.lock().unwrap().get_task(&id))
    }

    async fn on_cancel_task(&mut self, id: String) -> Result<Option<Task>, A2aError> {
        Ok(self.task_store.lock().unwrap().cancel_task(&id))
    }
}
```

**Step 5: Update WasmRunner::new() to initialize TaskStore**

Update the state creation (around line 170):

```rust
        let state = TestState {
            wasi,
            http: WasiHttpCtx::new(),
            table: ResourceTable::new(),
            task_store: Arc::new(Mutex::new(TaskStore::new())),
        };
```

**Step 6: Verify build**

Run: `cargo build -p a2a-wasm-component --tests`
Expected: Build succeeds

**Step 7: Run existing tests to ensure no regression**

Run: `cargo test -p a2a-wasm-component --lib`
Expected: All tests pass

**Step 8: Commit**

```bash
git add crates/a2a-wasm-component/Cargo.toml crates/a2a-wasm-component/tests/common/wasm_runner.rs
git commit -m "feat(a2a-wasm-component): add StatefulMockAgent with task storage"
```

---

## Task 3: Create WasmServer struct

**Files:**
- Create: `crates/a2a-wasm-component/tests/common/wasm_server.rs`
- Modify: `crates/a2a-wasm-component/tests/common/mod.rs`

**Step 1: Create wasm_server.rs**

Create `crates/a2a-wasm-component/tests/common/wasm_server.rs`:

```rust
//! WASM HTTP server for integration tests.
//!
//! This module provides a test harness that runs the WASM component as an HTTP server,
//! allowing external clients (like Python A2A SDK) to test against it.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use super::wasm_runner::{TaskStore, TestState};

// Re-use bindgen from wasm_runner
wasmtime::component::bindgen!({
    world: "a2a-component",
    path: "wit",
    async: true,
});

use a2a::protocol::agent::Host as AgentHost;
use a2a::protocol::types::{
    Error as A2aError, Message, MessageSendParams, Part, Role, SendResponse, Task, TaskState,
    TaskStatus, TextPart,
};

/// Path to the compiled WASM component.
const WASM_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../target/wasm32-wasip2/release/a2a_wasm_component.wasm"
);

/// Server state shared across requests
struct ServerState {
    engine: Engine,
    component: Component,
    linker: Linker<ServerClientState>,
    task_store: Arc<Mutex<TaskStore>>,
}

/// Per-request state
struct ServerClientState {
    wasi: WasiCtx,
    http: WasiHttpCtx,
    table: ResourceTable,
    task_store: Arc<Mutex<TaskStore>>,
}

impl WasiView for ServerClientState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiHttpView for ServerClientState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl AgentHost for ServerClientState {
    async fn get_agent_card(&mut self) -> Result<String, A2aError> {
        Ok(r#"{"name":"test-wasm-agent","description":"Test WASM agent","url":"http://localhost:9998","version":"1.0.0","capabilities":{},"defaultInputModes":["text"],"defaultOutputModes":["text"],"skills":[]}"#.to_string())
    }

    async fn on_message(&mut self, params: MessageSendParams) -> Result<SendResponse, A2aError> {
        let task = self.task_store.lock().unwrap().create_task(&params.message);
        Ok(SendResponse::Task(task))
    }

    async fn on_get_task(
        &mut self,
        id: String,
        _history_length: Option<u32>,
    ) -> Result<Option<Task>, A2aError> {
        Ok(self.task_store.lock().unwrap().get_task(&id))
    }

    async fn on_cancel_task(&mut self, id: String) -> Result<Option<Task>, A2aError> {
        Ok(self.task_store.lock().unwrap().cancel_task(&id))
    }
}

/// WASM HTTP server for testing.
pub struct WasmServer {
    shutdown_tx: Option<oneshot::Sender<()>>,
    handle: tokio::task::JoinHandle<()>,
    pub url: String,
}

impl WasmServer {
    /// Start the WASM server on port 9998.
    pub async fn start() -> Self {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (ready_tx, ready_rx) = oneshot::channel();

        let handle = tokio::spawn(async move {
            run_server(shutdown_rx, ready_tx).await;
        });

        // Wait for server to be ready
        ready_rx.await.expect("Server failed to start");

        Self {
            shutdown_tx: Some(shutdown_tx),
            handle,
            url: "http://localhost:9998".to_string(),
        }
    }
}

impl Drop for WasmServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        self.handle.abort();
    }
}

async fn run_server(mut shutdown_rx: oneshot::Receiver<()>, ready_tx: oneshot::Sender<()>) {
    // Setup wasmtime engine and component
    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    let engine = Engine::new(&config).expect("Failed to create engine");

    let component = Component::from_file(&engine, WASM_PATH)
        .expect("Failed to load WASM component");

    let mut linker = Linker::<ServerClientState>::new(&engine);
    wasmtime_wasi::add_to_linker_async(&mut linker).expect("Failed to add WASI");
    wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)
        .expect("Failed to add WASI HTTP");
    a2a::protocol::agent::add_to_linker(&mut linker, |state| state)
        .expect("Failed to add agent interface");

    let task_store = Arc::new(Mutex::new(TaskStore::new()));

    let server_state = Arc::new(ServerState {
        engine,
        component,
        linker,
        task_store,
    });

    // Bind to port 9998
    let addr: SocketAddr = "127.0.0.1:9998".parse().unwrap();
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");

    // Signal that we're ready
    let _ = ready_tx.send(());

    loop {
        tokio::select! {
            _ = &mut shutdown_rx => {
                break;
            }
            result = listener.accept() => {
                match result {
                    Ok((stream, _addr)) => {
                        let server_state = server_state.clone();
                        tokio::spawn(async move {
                            let service = service_fn(move |req| {
                                let server_state = server_state.clone();
                                async move {
                                    handle_request(server_state, req).await
                                }
                            });

                            if let Err(e) = http1::Builder::new()
                                .keep_alive(true)
                                .serve_connection(TokioIo::new(stream), service)
                                .await
                            {
                                eprintln!("Error serving connection: {e}");
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {e}");
                    }
                }
            }
        }
    }
}

async fn handle_request(
    server_state: Arc<ServerState>,
    req: Request<Incoming>,
) -> Result<Response<HyperOutgoingBody>, hyper::Error> {
    let wasi = WasiCtxBuilder::new().inherit_env().build();
    let state = ServerClientState {
        wasi,
        http: WasiHttpCtx::new(),
        table: ResourceTable::new(),
        task_store: server_state.task_store.clone(),
    };

    let mut store = Store::new(&server_state.engine, state);

    let (sender, receiver) = oneshot::channel();
    let incoming_req = store.data_mut().new_incoming_request(Scheme::Http, req)
        .expect("Failed to create incoming request");
    let out = store.data_mut().new_response_outparam(sender)
        .expect("Failed to create response outparam");

    let instance = server_state.linker
        .instantiate_async(&mut store, &server_state.component)
        .await
        .expect("Failed to instantiate");

    // Get the incoming-handler export
    let handler_indices = exports::wasi::http::incoming_handler::GuestIndices::new_instance(
        &mut store,
        &instance,
    ).expect("Failed to get incoming-handler indices");
    let handler = handler_indices.load(&mut store, &instance)
        .expect("Failed to load incoming-handler");

    // Spawn the handler
    let handle_task = tokio::spawn(async move {
        handler.call_handle(&mut store, incoming_req, out).await
    });

    match receiver.await {
        Ok(Ok(resp)) => Ok(resp),
        Ok(Err(e)) => {
            // Return error response
            let body = format!("Error: {e:?}");
            Ok(Response::builder()
                .status(500)
                .body(HyperOutgoingBody::default())
                .unwrap())
        }
        Err(_) => {
            // Receiver dropped - check task result
            match handle_task.await {
                Ok(Ok(())) => {
                    Ok(Response::builder()
                        .status(500)
                        .body(HyperOutgoingBody::default())
                        .unwrap())
                }
                Ok(Err(e)) => {
                    eprintln!("Handler error: {e}");
                    Ok(Response::builder()
                        .status(500)
                        .body(HyperOutgoingBody::default())
                        .unwrap())
                }
                Err(e) => {
                    eprintln!("Task join error: {e}");
                    Ok(Response::builder()
                        .status(500)
                        .body(HyperOutgoingBody::default())
                        .unwrap())
                }
            }
        }
    }
}
```

**Step 2: Update mod.rs to export wasm_server**

Modify `crates/a2a-wasm-component/tests/common/mod.rs`:

```rust
pub mod server;
pub mod wasm_runner;
pub mod wasm_server;
```

**Step 3: Verify build**

Run: `cargo build -p a2a-wasm-component --tests`
Expected: Build succeeds (may have warnings, that's OK for now)

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/tests/common/
git commit -m "feat(a2a-wasm-component): add WasmServer for HTTP serving"
```

---

## Task 4: Create Python test fixture

**Files:**
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/pyproject.toml`
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/conftest.py`
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/test_wasm_server.py`

**Step 1: Create pyproject.toml**

Create `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/pyproject.toml`:

```toml
[project]
name = "wasm-server-tests"
version = "0.1.0"
description = "Integration tests for A2A WASM server"
requires-python = ">=3.11"

[tool.hatch.metadata]
allow-direct-references = true

dependencies = [
    "a2a-sdk @ git+https://github.com/muscariello/a2a-python@a2a_proto_refactor",
    "pytest>=8.0",
    "pytest-asyncio>=0.24",
    "httpx>=0.27",
]

[tool.pytest.ini_options]
asyncio_mode = "auto"
asyncio_default_fixture_loop_scope = "function"
```

**Step 2: Create conftest.py**

Create `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/conftest.py`:

```python
import os
import pytest


@pytest.fixture
def server_url():
    """Get the WASM server URL from environment."""
    url = os.environ.get("WASM_SERVER_URL", "http://localhost:9998")
    return url
```

**Step 3: Create test_wasm_server.py**

Create `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/test_wasm_server.py`:

```python
"""Integration tests for the A2A WASM server."""

import httpx
import pytest

from a2a.client import A2AClient
from a2a.types import (
    Message,
    MessageSendParams,
    TextPart,
)


class TestAgentCardDiscovery:
    """Test agent card discovery endpoint."""

    async def test_agent_card_returns_valid_json(self, server_url: str):
        """GET /.well-known/agent-card.json returns valid agent card."""
        async with httpx.AsyncClient() as client:
            response = await client.get(f"{server_url}/.well-known/agent-card.json")

        assert response.status_code == 200
        assert response.headers["content-type"] == "application/json"

        card = response.json()
        assert card["name"] == "test-wasm-agent"
        assert "capabilities" in card


class TestSendMessageJsonRpc:
    """Test SendMessage via JSON-RPC binding."""

    async def test_send_message_returns_task(self, server_url: str):
        """SendMessage via JSON-RPC returns a task response."""
        async with httpx.AsyncClient() as client:
            response = await client.post(
                f"{server_url}/",
                json={
                    "jsonrpc": "2.0",
                    "id": "1",
                    "method": "SendMessage",
                    "params": {
                        "message": {
                            "role": "user",
                            "parts": [{"type": "text", "text": "Hello"}],
                        }
                    },
                },
                headers={"Content-Type": "application/json"},
            )

        assert response.status_code == 200
        result = response.json()
        assert "result" in result
        # Response is either task or message
        assert result["result"].get("type") in ("task", "message") or "id" in result["result"]


class TestSendMessageRest:
    """Test SendMessage via REST binding."""

    async def test_send_message_rest(self, server_url: str):
        """POST /v1/message:send returns a response."""
        async with httpx.AsyncClient() as client:
            response = await client.post(
                f"{server_url}/v1/message:send",
                json={
                    "message": {
                        "role": "user",
                        "parts": [{"type": "text", "text": "Hello"}],
                    }
                },
                headers={"Content-Type": "application/json"},
            )

        assert response.status_code == 200


class TestGetTask:
    """Test GetTask operations."""

    async def test_get_task_not_found_jsonrpc(self, server_url: str):
        """GetTask for non-existent task returns appropriate error/null."""
        async with httpx.AsyncClient() as client:
            response = await client.post(
                f"{server_url}/",
                json={
                    "jsonrpc": "2.0",
                    "id": "1",
                    "method": "GetTask",
                    "params": {"id": "non-existent-task-id"},
                },
                headers={"Content-Type": "application/json"},
            )

        assert response.status_code == 200
        result = response.json()
        # Either null result or error
        assert result.get("result") is None or "error" in result

    async def test_get_task_after_send(self, server_url: str):
        """GetTask returns task created by SendMessage."""
        async with httpx.AsyncClient() as client:
            # First, send a message to create a task
            send_response = await client.post(
                f"{server_url}/",
                json={
                    "jsonrpc": "2.0",
                    "id": "1",
                    "method": "SendMessage",
                    "params": {
                        "message": {
                            "role": "user",
                            "parts": [{"type": "text", "text": "Hello"}],
                        }
                    },
                },
                headers={"Content-Type": "application/json"},
            )

            send_result = send_response.json()
            # Extract task ID from response
            task_id = None
            if "result" in send_result:
                result = send_result["result"]
                if isinstance(result, dict):
                    task_id = result.get("id") or result.get("task", {}).get("id")

            if task_id:
                # Now get the task
                get_response = await client.post(
                    f"{server_url}/",
                    json={
                        "jsonrpc": "2.0",
                        "id": "2",
                        "method": "GetTask",
                        "params": {"id": task_id},
                    },
                    headers={"Content-Type": "application/json"},
                )

                get_result = get_response.json()
                assert "result" in get_result
                assert get_result["result"] is not None


class TestCancelTask:
    """Test CancelTask operations."""

    async def test_cancel_task_not_found(self, server_url: str):
        """CancelTask for non-existent task returns appropriate error/null."""
        async with httpx.AsyncClient() as client:
            response = await client.post(
                f"{server_url}/",
                json={
                    "jsonrpc": "2.0",
                    "id": "1",
                    "method": "CancelTask",
                    "params": {"id": "non-existent-task-id"},
                },
                headers={"Content-Type": "application/json"},
            )

        assert response.status_code == 200
        result = response.json()
        # Either null result or error
        assert result.get("result") is None or "error" in result


class TestInvalidMethod:
    """Test error handling for invalid methods."""

    async def test_invalid_method_returns_error(self, server_url: str):
        """Invalid JSON-RPC method returns -32601 error."""
        async with httpx.AsyncClient() as client:
            response = await client.post(
                f"{server_url}/",
                json={
                    "jsonrpc": "2.0",
                    "id": "1",
                    "method": "InvalidMethod",
                    "params": {},
                },
                headers={"Content-Type": "application/json"},
            )

        assert response.status_code == 200
        result = response.json()
        assert "error" in result
        assert result["error"]["code"] == -32601  # Method not found
```

**Step 4: Verify Python setup**

Run:
```bash
cd crates/a2a-wasm-component/tests/fixtures/wasm_server_tests
uv sync
```
Expected: Dependencies install successfully

**Step 5: Commit**

```bash
git add crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/
git commit -m "feat(a2a-wasm-component): add Python test fixture for WASM server tests"
```

---

## Task 5: Create Rust integration test that runs Python tests

**Files:**
- Create: `crates/a2a-wasm-component/tests/server_integration_test.rs`

**Step 1: Create the integration test file**

Create `crates/a2a-wasm-component/tests/server_integration_test.rs`:

```rust
//! Server integration tests for the A2A WASM component.
//!
//! These tests run the WASM component as an HTTP server and test it
//! with the Python A2A SDK client.
//!
//! Run with: `cargo test -p a2a-wasm-component --test server_integration_test`

mod common;

use common::wasm_server::WasmServer;
use std::path::PathBuf;
use std::process::Command;

const FIXTURES_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/wasm_server_tests"
);

/// Test the WASM server with Python A2A SDK client.
#[tokio::test]
async fn test_wasm_server_with_python_client() {
    // Start the WASM server
    let server = WasmServer::start().await;
    println!("WASM server started at {}", server.url);

    // Run Python tests
    let fixture_path = PathBuf::from(FIXTURES_DIR);
    let status = Command::new("uv")
        .args(["run", "pytest", "-v", "--tb=short"])
        .current_dir(&fixture_path)
        .env("WASM_SERVER_URL", &server.url)
        .status()
        .expect("Failed to run pytest - is uv installed?");

    assert!(status.success(), "Python tests failed");
}
```

**Step 2: Ensure WASM component is built before tests**

The tests require the WASM component to be built. Add a note in the test file or ensure CI builds it first.

**Step 3: Verify test compiles**

Run: `cargo build -p a2a-wasm-component --tests`
Expected: Build succeeds

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/tests/server_integration_test.rs
git commit -m "feat(a2a-wasm-component): add server integration test runner"
```

---

## Task 6: Fix compilation issues and run tests

**Step 1: Build the WASM component**

Run:
```bash
cargo build -p a2a-wasm-component --target wasm32-wasip2 --release
```
Expected: WASM component builds

**Step 2: Run the server integration test**

Run:
```bash
cargo test -p a2a-wasm-component --test server_integration_test -- --nocapture
```

Expected: Test starts server, runs Python tests, all pass

**Step 3: Fix any issues**

If there are compilation or runtime issues, fix them iteratively.

**Step 4: Final commit**

```bash
git add -A
git commit -m "fix(a2a-wasm-component): fix server integration test issues"
```

---

## Notes

- Port 9998 is used to avoid collision with the Python helloworld server (9999)
- The `TaskStore` is shared across requests via `Arc<Mutex<>>`
- The WASM component must be built before running tests: `cargo build -p a2a-wasm-component --target wasm32-wasip2 --release`
- Python tests use `httpx` for raw HTTP requests to test both JSON-RPC and REST bindings
