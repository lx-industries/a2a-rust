# Server Integration Test Refactor Design

## Problem

The current `test_wasm_server.py` is monolithic and hard to maintain. We want:
- One Python file per test case
- One Python file per user journey
- Insta snapshots for stdout comparison

## Design

### File Structure

```
crates/a2a-wasm-component/tests/
├── fixtures/
│   └── wasm_server_tests/
│       ├── scenarios/
│       │   ├── send_message_success.py
│       │   ├── send_message_creates_task.py
│       │   ├── get_task_not_found.py
│       │   ├── get_task_after_send.py
│       │   ├── cancel_task_not_found.py
│       │   ├── cancel_task_success.py
│       │   ├── agent_card_discovery.py
│       │   ├── json_rpc_invalid_method.py
│       │   ├── journey_basic_flow.py
│       │   └── journey_error_handling.py
│       ├── conftest.py
│       └── pyproject.toml
├── snapshots/
│   └── server_integration_test__*.snap
└── server_integration_test.rs
```

### Rust Test Runner

Uses `test-case` crate for parameterized tests:

```rust
use test_case::test_case;

#[test_case("send_message_success")]
#[test_case("send_message_creates_task")]
#[test_case("get_task_not_found")]
#[test_case("get_task_after_send")]
#[test_case("cancel_task_not_found")]
#[test_case("cancel_task_success")]
#[test_case("agent_card_discovery")]
#[test_case("json_rpc_invalid_method")]
#[test_case("journey_basic_flow")]
#[test_case("journey_error_handling")]
fn test_scenario(scenario: &str) {
    // 1. Start WasmServer
    // 2. Run: python scenarios/{scenario}.py with WASM_SERVER_URL env var
    // 3. Capture stdout (NDJSON)
    // 4. Parse into JSON array, apply insta redactions, snapshot
}
```

Insta configuration with redactions for dynamic values:

```rust
let steps: Vec<serde_json::Value> = stdout
    .lines()
    .filter(|l| !l.is_empty())
    .map(|l| serde_json::from_str(l).expect("valid JSON"))
    .collect();

insta::with_settings!({
    filters => vec![
        (r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", "[UUID]"),
        (r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}", "[TIMESTAMP]"),
    ]
}, {
    insta::assert_json_snapshot!(format!("{scenario}"), steps);
});
```

### Python Scenario Files

Each scenario is a standalone script that:
1. Imports the `a2a-sdk` client
2. Connects to server via `WASM_SERVER_URL` env var
3. Performs test actions
4. Prints JSON to stdout after each step (NDJSON format)

Example unit test (`send_message_success.py`):

```python
import asyncio
import json
import os
from a2a import ClientFactory

async def main():
    client = await ClientFactory.connect(os.environ["WASM_SERVER_URL"])
    response = await client.send_message("hello")
    print(json.dumps({
        "status": response.status.state,
        "message": response.status.message.parts[0].text,
        "has_task_id": response.task_id is not None,
    }))

asyncio.run(main())
```

Example journey test (`journey_basic_flow.py`):

```python
async def main():
    client = await ClientFactory.connect(os.environ["WASM_SERVER_URL"])

    send_resp = await client.send_message("hello")
    print(json.dumps({"step": "send_message", "status": send_resp.status.state}))

    get_resp = await client.get_task(send_resp.task_id)
    print(json.dumps({"step": "get_task", "status": get_resp.status.state, "history_count": len(get_resp.history)}))

    cancel_resp = await client.cancel_task(send_resp.task_id)
    print(json.dumps({"step": "cancel_task", "status": cancel_resp.status.state}))
```

Example error test (`get_task_not_found.py`):

```python
async def main():
    client = await ClientFactory.connect(os.environ["WASM_SERVER_URL"])

    try:
        await client.get_task("nonexistent-task-id")
        print(json.dumps({"error": "expected TaskNotFound, got success"}))
    except A2AError as e:
        print(json.dumps({"error_code": e.code, "error_type": "TaskNotFound"}))
```

### Error Handling

- **Expected errors**: Output error details as JSON (snapshotted)
- **Unexpected errors**: Python exits non-zero, Rust test fails with stderr in error message (not snapshotted)

```rust
let output = Command::new("python").arg(script).output()?;

if !output.status.success() {
    panic!(
        "Scenario {scenario} failed:\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
```

### Snapshot Format

Pretty-printed JSON array:

```json
[
  {
    "step": "send_message",
    "status": "completed"
  },
  {
    "step": "get_task",
    "status": "completed",
    "history_count": 1
  }
]
```

## Developer Workflow

**Adding a new scenario:**
1. Create `scenarios/new_scenario.py`
2. Add `#[test_case("new_scenario")]` to `server_integration_test.rs`
3. Run `cargo test -p a2a-wasm-component --test server_integration_test`
4. Review and accept: `cargo insta review`

**Running specific scenario:**
```bash
cargo test -p a2a-wasm-component --test server_integration_test "send_message_success"
```

## Migration

**Delete:**
- `test_wasm_server.py`

**Create:**
- `scenarios/` directory with 10 Python files
- Rewritten `server_integration_test.rs`

**Keep:**
- `conftest.py`, `pyproject.toml`
- Existing `common/` test infrastructure
- `integration_test.rs` (unrelated WASM client tests)

**Add dependency:**
- `test-case` crate to `Cargo.toml`
