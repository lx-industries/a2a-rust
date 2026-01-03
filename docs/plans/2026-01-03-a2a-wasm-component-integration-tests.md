# A2A WASM Component Integration Tests Design

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Full compliance integration tests for the WASM component client interface using the Python A2A reference implementation.

**Architecture:** Rust test harness using wasmtime to load the WASM component, making real HTTP calls to a vendored Python helloworld agent server spawned via `uv run`.

**Tech Stack:** wasmtime + wasmtime-wasi-http, tokio, insta snapshots, Python a2a-sdk

---

## Test Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Rust Test Harness                        │
│  ┌─────────────────┐    ┌──────────────────────────────────┐│
│  │ wasmtime Engine │───▶│ a2a_wasm_component.wasm          ││
│  │ + wasi-http     │    │ (exports: client interface)      ││
│  └────────┬────────┘    └──────────────────────────────────┘│
│           │ HTTP                                             │
└───────────┼──────────────────────────────────────────────────┘
            ▼
┌─────────────────────────────────────────────────────────────┐
│              Python A2A Reference Server                     │
│  helloworld agent on localhost:9999                          │
│  - message/send → returns Task with "Hello World"            │
│  - tasks/get → returns task by ID                            │
│  - tasks/cancel → returns TaskNotCancelableError (-32002)    │
└─────────────────────────────────────────────────────────────┘
```

## Scope

- **In scope:** Blocking operations (`message/send`, `tasks/get`, `tasks/cancel`)
- **Deferred:** Streaming, push notifications, agent cards, FilePart/DataPart, gRPC

## File Structure

```
crates/a2a-wasm-component/
├── tests/
│   ├── integration_test.rs      # Main integration tests
│   ├── common/
│   │   ├── mod.rs               # Test utilities
│   │   ├── server.rs            # Python server spawn/teardown
│   │   └── wasm_runner.rs       # Wasmtime + component loading
│   ├── fixtures/
│   │   └── helloworld/          # Vendored Python agent
│   │       ├── pyproject.toml
│   │       ├── __main__.py
│   │       └── agent_executor.py
│   └── snapshots/               # insta snapshot files
│       ├── integration_test__send_message_response.snap
│       ├── integration_test__task_structure.snap
│       └── integration_test__error_responses.snap
```

## Test Scenarios

### Integration Tests (16 tests)

#### Core Operations (5 tests)

| Test | Operation | Expected |
|------|-----------|----------|
| `send_message_text_returns_task` | `send_message` | `SendResponse::Task` |
| `send_message_with_blocking_config` | `send_message` + config | Config respected |
| `get_task_existing` | `get_task` | `Some(Task)` |
| `get_task_with_history_length` | `get_task` + history | History populated |
| `cancel_task_not_cancelable` | `cancel_task` | Error -32002 |

#### Error Handling (6 tests)

| Test | Trigger | Expected |
|------|---------|----------|
| `get_task_not_found_returns_none` | Invalid task ID | `Ok(None)` |
| `cancel_task_not_found_returns_none` | Invalid task ID | `Ok(None)` |
| `invalid_url_transport_error` | `http://invalid` | Error -32000 |
| `connection_refused_error` | Server not running | Error -32000 |
| `invalid_method_error` | (mock server) | Error -32601 |
| `internal_server_error` | (mock server) | Error -32603 |

#### Response Validation (5 tests)

| Test | Validates |
|------|-----------|
| `response_task_has_id_and_context` | Task.id, Task.context_id |
| `response_task_status_correct` | TaskStatus.state |
| `response_message_role_agent` | Role::Agent |
| `response_part_is_text` | Part::Text variant |
| `response_text_content_hello_world` | "Hello World" |

### Unit Tests (9 tests)

Extend existing 5 tests to 9:

| Test | Validates |
|------|-----------|
| `role_conversion` | (existing) |
| `task_state_conversion` | (existing) |
| `text_part_conversion` | (existing) |
| `file_part_not_implemented` | (existing) |
| `data_part_not_implemented` | (existing) |
| `message_send_params_to_a2a` | Full params conversion |
| `task_from_a2a_with_history` | History list conversion |
| `error_from_jsonrpc_preserves_code` | Code passthrough |
| `error_from_transport_uses_32000` | Transport → -32000 |

## Implementation Details

### Server Management (`common/server.rs`)

```rust
pub struct TestServer {
    process: Child,
    pub url: String,
}

impl TestServer {
    pub fn start() -> Self {
        let process = Command::new("uv")
            .args(["run", "."])
            .current_dir("tests/fixtures/helloworld")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("uv must be installed");

        wait_for_port(9999, Duration::from_secs(10));

        Self {
            process,
            url: "http://localhost:9999".into(),
        }
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}
```

### WASM Runner (`common/wasm_runner.rs`)

```rust
pub struct WasmRunner {
    store: Store<TestState>,
    instance: Instance,
}

impl WasmRunner {
    pub fn new() -> Self {
        let engine = Engine::new(Config::new().async_support(true))?;
        let component = Component::from_file(&engine, WASM_PATH)?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_async(&mut linker)?;
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)?;

        // ... instantiate
    }

    pub async fn send_message(&mut self, url: &str, params: ...) -> Result<...> {
        // Call exported function
    }
}
```

### Snapshot Strategy

| Test Category | Snapshot Type | What's Captured |
|---------------|---------------|-----------------|
| Response structure | `assert_json_snapshot!` | Full Task/Message JSON |
| Error responses | `assert_json_snapshot!` | Error code + message |
| Type conversions | `assert_debug_snapshot!` | Rust debug repr |

Redact dynamic fields:

```rust
with_settings!({
    filters => vec![
        (r#""id": "[^"]+""#, r#""id": "[TASK_ID]""#),
        (r#""timestamp": "[^"]+""#, r#""timestamp": "[TS]""#),
    ]
}, {
    assert_json_snapshot!("send_message_response", result);
});
```

## Dependencies

```toml
# crates/a2a-wasm-component/Cargo.toml
[dev-dependencies]
tokio = { version = "1.48", features = ["rt-multi-thread", "macros"] }
wasmtime = "29"
wasmtime-wasi = "29"
wasmtime-wasi-http = "29"
insta = { version = "1.42", features = ["json", "redactions"] }
serde_json = "1.0"
```

## CI Integration

```yaml
# .gitlab/ci/rust.gitlab-ci.yml (additions)

test:wasm-integration:
  stage: test
  image: rust:1.92.0
  variables:
    WASM_TARGET: wasm32-wasip2
  before_script:
    - curl -LsSf https://astral.sh/uv/install.sh | sh
    - source $HOME/.local/bin/env
    - rustup target add wasm32-wasip2
  script:
    - cargo build -p a2a-wasm-component --target wasm32-wasip2 --release
    - cargo test -p a2a-wasm-component --test integration_test -- --test-threads=1
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
```

## Test Execution

```bash
# Local development
cargo build -p a2a-wasm-component --target wasm32-wasip2 --release
cargo test -p a2a-wasm-component --test integration_test -- --test-threads=1

# Update snapshots
cargo insta test -p a2a-wasm-component --test integration_test
cargo insta review
```

**Note:** `--test-threads=1` required because tests share the Python server on port 9999.

## Deferred Items

| Feature | Reason | Future Test |
|---------|--------|-------------|
| Streaming responses | Requires SSE handling | `send_message_streaming` |
| Push notifications | Not in WIT interface | `set_task_callback` |
| Agent card discovery | Not in WIT interface | `get_agent_card` |
| FilePart / DataPart | Returns error currently | `send_message_with_file` |
| gRPC transport | HTTP only | `grpc_*` tests |
| Card signing | Not implemented | `verify_signed_card` |

## References

- [a2a-python SDK](https://github.com/a2aproject/a2a-python) - Reference implementation
- [a2a-samples helloworld](https://github.com/a2aproject/a2a-samples/tree/main/samples/python/agents/helloworld) - Test server source
- [wasmtime-wasi-http](https://docs.rs/wasmtime-wasi-http/latest/wasmtime_wasi_http/) - WASM HTTP runtime
