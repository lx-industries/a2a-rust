# Server Integration Test Refactor Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Refactor monolithic `test_wasm_server.py` into individual scenario files with insta snapshots.

**Architecture:** Parameterized Rust tests using `test-case` crate run individual Python scenario scripts, capture NDJSON stdout, parse into arrays, and snapshot with insta redactions.

**Tech Stack:** Rust (test-case, insta), Python (a2a-sdk, httpx), NDJSON output format.

---

### Task 1: Add test-case dependency

**Files:**
- Modify: `crates/a2a-wasm-component/Cargo.toml`

**Step 1: Add test-case to dev-dependencies**

Add after the `insta` line in `[dev-dependencies]`:

```toml
test-case = "3.3"
```

**Step 2: Verify it compiles**

Run: `cargo check -p a2a-wasm-component`
Expected: Compiles without errors

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/Cargo.toml
git commit -m "chore(a2a-wasm-component): add test-case dependency"
```

---

### Task 2: Create scenarios directory and first scenario

**Files:**
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/agent_card_discovery.py`

**Step 1: Create scenarios directory**

```bash
mkdir -p crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios
```

**Step 2: Write agent_card_discovery.py**

```python
"""Test agent card discovery endpoint."""

import asyncio
import json
import os

import httpx
from a2a.client.client_factory import ClientFactory


async def main():
    server_url = os.environ["WASM_SERVER_URL"]

    # Test 1: GET agent card returns valid JSON
    async with httpx.AsyncClient() as http_client:
        response = await http_client.get(f"{server_url}/.well-known/agent-card.json")

    print(json.dumps({
        "step": "get_agent_card",
        "status_code": response.status_code,
        "content_type": response.headers["content-type"],
        "name": response.json().get("name"),
        "has_capabilities": "capabilities" in response.json(),
    }))

    # Test 2: ClientFactory.connect works
    client = await ClientFactory.connect(server_url)
    print(json.dumps({
        "step": "client_factory_connect",
        "success": client is not None,
    }))


if __name__ == "__main__":
    asyncio.run(main())
```

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/
git commit -m "feat(a2a-wasm-component): add agent_card_discovery scenario"
```

---

### Task 3: Write send_message scenarios

**Files:**
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/send_message_success.py`
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/send_message_creates_task.py`

**Step 1: Write send_message_success.py**

```python
"""Test SendMessage returns a response."""

import asyncio
import json
import os

from a2a.client.client_factory import ClientFactory
from a2a.types import Message, Part, Role


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    message = Message(
        role=Role.ROLE_USER,
        parts=[Part(text="Hello from Python SDK")],
    )

    responses = []
    async for response, task in client.send_message(message):
        responses.append((response, task))

    print(json.dumps({
        "step": "send_message",
        "response_count": len(responses),
        "has_responses": len(responses) > 0,
    }))


if __name__ == "__main__":
    asyncio.run(main())
```

**Step 2: Write send_message_creates_task.py**

```python
"""Test SendMessage creates a retrievable task."""

import asyncio
import json
import os

from a2a.client.client_factory import ClientFactory
from a2a.types import Message, Part, Role, GetTaskRequest


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    message = Message(
        role=Role.ROLE_USER,
        parts=[Part(text="Hello")],
    )

    task_id = None
    async for response, task in client.send_message(message):
        if task is not None:
            task_id = task.id
            break

    print(json.dumps({
        "step": "send_message",
        "got_task_id": task_id is not None,
    }))

    if task_id:
        request = GetTaskRequest(name=task_id)
        retrieved_task = await client.get_task(request)
        print(json.dumps({
            "step": "get_task",
            "task_id_matches": retrieved_task.id == task_id,
        }))


if __name__ == "__main__":
    asyncio.run(main())
```

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/
git commit -m "feat(a2a-wasm-component): add send_message scenarios"
```

---

### Task 4: Write get_task scenarios

**Files:**
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/get_task_not_found.py`
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/get_task_after_send.py`

**Step 1: Write get_task_not_found.py**

```python
"""Test GetTask for non-existent task returns error."""

import asyncio
import json
import os

from a2a.client.client_factory import ClientFactory
from a2a.types import GetTaskRequest


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    request = GetTaskRequest(name="non-existent-task-id")

    try:
        task = await client.get_task(request)
        print(json.dumps({
            "step": "get_task_not_found",
            "error": False,
            "task_is_none": task is None,
        }))
    except Exception as e:
        print(json.dumps({
            "step": "get_task_not_found",
            "error": True,
            "error_type": type(e).__name__,
        }))


if __name__ == "__main__":
    asyncio.run(main())
```

**Step 2: Write get_task_after_send.py**

```python
"""Test GetTask returns task created by SendMessage."""

import asyncio
import json
import os

from a2a.client.client_factory import ClientFactory
from a2a.types import Message, Part, Role, GetTaskRequest


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    # First create a task
    message = Message(
        role=Role.ROLE_USER,
        parts=[Part(text="Hello")],
    )

    task_id = None
    async for response, task in client.send_message(message):
        if task is not None:
            task_id = task.id
            break

    print(json.dumps({
        "step": "send_message",
        "got_task_id": task_id is not None,
    }))

    assert task_id is not None, "Expected to get a task from send_message"

    # Now get the task
    request = GetTaskRequest(name=task_id)
    retrieved_task = await client.get_task(request)

    print(json.dumps({
        "step": "get_task",
        "task_id_matches": retrieved_task.id == task_id,
    }))


if __name__ == "__main__":
    asyncio.run(main())
```

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/
git commit -m "feat(a2a-wasm-component): add get_task scenarios"
```

---

### Task 5: Write cancel_task scenarios

**Files:**
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/cancel_task_not_found.py`
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/cancel_task_success.py`

**Step 1: Write cancel_task_not_found.py**

```python
"""Test CancelTask for non-existent task returns error."""

import asyncio
import json
import os

from a2a.client.client_factory import ClientFactory
from a2a.types import CancelTaskRequest


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    request = CancelTaskRequest(name="non-existent-task-id")

    try:
        task = await client.cancel_task(request)
        print(json.dumps({
            "step": "cancel_task_not_found",
            "error": False,
            "task_is_none": task is None,
        }))
    except Exception as e:
        print(json.dumps({
            "step": "cancel_task_not_found",
            "error": True,
            "error_type": type(e).__name__,
        }))


if __name__ == "__main__":
    asyncio.run(main())
```

**Step 2: Write cancel_task_success.py**

```python
"""Test CancelTask cancels a task created by SendMessage."""

import asyncio
import json
import os

from a2a.client.client_factory import ClientFactory
from a2a.types import Message, Part, Role, CancelTaskRequest


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    # First create a task
    message = Message(
        role=Role.ROLE_USER,
        parts=[Part(text="Hello")],
    )

    task_id = None
    async for response, task in client.send_message(message):
        if task is not None:
            task_id = task.id
            break

    print(json.dumps({
        "step": "send_message",
        "got_task_id": task_id is not None,
    }))

    assert task_id is not None, "Expected to get a task from send_message"

    # Now cancel the task
    request = CancelTaskRequest(name=task_id)
    cancelled_task = await client.cancel_task(request)

    print(json.dumps({
        "step": "cancel_task",
        "task_id_matches": cancelled_task.id == task_id,
    }))


if __name__ == "__main__":
    asyncio.run(main())
```

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/
git commit -m "feat(a2a-wasm-component): add cancel_task scenarios"
```

---

### Task 6: Write JSON-RPC error scenario

**Files:**
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/json_rpc_invalid_method.py`

**Step 1: Write json_rpc_invalid_method.py**

```python
"""Test JSON-RPC invalid method returns error."""

import asyncio
import json
import os

import httpx


async def main():
    server_url = os.environ["WASM_SERVER_URL"]

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

    result = response.json()

    print(json.dumps({
        "step": "invalid_method",
        "status_code": response.status_code,
        "has_error": "error" in result,
        "error_code": result.get("error", {}).get("code"),
    }))


if __name__ == "__main__":
    asyncio.run(main())
```

**Step 2: Commit**

```bash
git add crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/
git commit -m "feat(a2a-wasm-component): add json_rpc_invalid_method scenario"
```

---

### Task 7: Write journey scenarios

**Files:**
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/journey_basic_flow.py`
- Create: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/journey_error_handling.py`

**Step 1: Write journey_basic_flow.py**

```python
"""Test basic flow: send message -> get task -> cancel task."""

import asyncio
import json
import os

from a2a.client.client_factory import ClientFactory
from a2a.types import Message, Part, Role, GetTaskRequest, CancelTaskRequest


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    # Step 1: Send message
    message = Message(
        role=Role.ROLE_USER,
        parts=[Part(text="Hello from journey test")],
    )

    task_id = None
    async for response, task in client.send_message(message):
        if task is not None:
            task_id = task.id
            break

    print(json.dumps({
        "step": "send_message",
        "got_task_id": task_id is not None,
    }))

    assert task_id is not None, "Expected to get a task from send_message"

    # Step 2: Get task
    get_request = GetTaskRequest(name=task_id)
    retrieved_task = await client.get_task(get_request)

    print(json.dumps({
        "step": "get_task",
        "task_id_matches": retrieved_task.id == task_id,
        "has_history": len(retrieved_task.history) > 0 if retrieved_task.history else False,
    }))

    # Step 3: Cancel task
    cancel_request = CancelTaskRequest(name=task_id)
    cancelled_task = await client.cancel_task(cancel_request)

    print(json.dumps({
        "step": "cancel_task",
        "task_id_matches": cancelled_task.id == task_id,
    }))


if __name__ == "__main__":
    asyncio.run(main())
```

**Step 2: Write journey_error_handling.py**

```python
"""Test error handling: various error conditions."""

import asyncio
import json
import os

import httpx
from a2a.client.client_factory import ClientFactory
from a2a.types import GetTaskRequest, CancelTaskRequest


async def main():
    server_url = os.environ["WASM_SERVER_URL"]
    client = await ClientFactory.connect(server_url)

    # Error 1: Get non-existent task
    try:
        await client.get_task(GetTaskRequest(name="nonexistent-1"))
        print(json.dumps({"step": "get_nonexistent", "error": False}))
    except Exception as e:
        print(json.dumps({"step": "get_nonexistent", "error": True, "error_type": type(e).__name__}))

    # Error 2: Cancel non-existent task
    try:
        await client.cancel_task(CancelTaskRequest(name="nonexistent-2"))
        print(json.dumps({"step": "cancel_nonexistent", "error": False}))
    except Exception as e:
        print(json.dumps({"step": "cancel_nonexistent", "error": True, "error_type": type(e).__name__}))

    # Error 3: Invalid JSON-RPC method
    async with httpx.AsyncClient() as http_client:
        response = await http_client.post(
            f"{server_url}/",
            json={"jsonrpc": "2.0", "id": "1", "method": "BadMethod", "params": {}},
            headers={"Content-Type": "application/json"},
        )
    result = response.json()
    print(json.dumps({
        "step": "invalid_method",
        "has_error": "error" in result,
        "error_code": result.get("error", {}).get("code"),
    }))


if __name__ == "__main__":
    asyncio.run(main())
```

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/scenarios/
git commit -m "feat(a2a-wasm-component): add journey scenarios"
```

---

### Task 8: Rewrite server_integration_test.rs

**Files:**
- Modify: `crates/a2a-wasm-component/tests/server_integration_test.rs`

**Step 1: Write the new parameterized test runner**

Replace entire contents of `server_integration_test.rs`:

```rust
//! Server integration tests for the A2A WASM component.
//!
//! These tests run the WASM component as an HTTP server and test it
//! with the Python A2A SDK client via individual scenario scripts.
//!
//! Run with: `cargo test -p a2a-wasm-component --test server_integration_test`
//!
//! Prerequisites:
//! - Build the WASM component: `cargo build -p a2a-wasm-component --target wasm32-wasip2 --release`
//! - Install Python dependencies: `cd tests/fixtures/wasm_server_tests && uv sync`

mod common;

use common::wasm_server::WasmServer;
use std::process::Command;
use test_case::test_case;

const SCENARIOS_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/wasm_server_tests/scenarios"
);

#[test_case("agent_card_discovery")]
#[test_case("send_message_success")]
#[test_case("send_message_creates_task")]
#[test_case("get_task_not_found")]
#[test_case("get_task_after_send")]
#[test_case("cancel_task_not_found")]
#[test_case("cancel_task_success")]
#[test_case("json_rpc_invalid_method")]
#[test_case("journey_basic_flow")]
#[test_case("journey_error_handling")]
fn test_scenario(scenario: &str) {
    // Start the WASM server using tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    let server = rt.block_on(WasmServer::start());

    // Run Python scenario script
    let script_path = format!("{}/{}.py", SCENARIOS_DIR, scenario);
    let output = Command::new("uv")
        .args(["run", "python", &script_path])
        .current_dir(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/wasm_server_tests"
        ))
        .env("WASM_SERVER_URL", &server.url)
        .output()
        .expect("Failed to run scenario - is uv installed?");

    if !output.status.success() {
        panic!(
            "Scenario {} failed:\nstderr: {}",
            scenario,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Parse NDJSON stdout into JSON array
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in stdout");
    let steps: Vec<serde_json::Value> = stdout
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).expect("Invalid JSON in stdout"))
        .collect();

    // Snapshot with redactions for dynamic values
    insta::with_settings!({
        filters => vec![
            (r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", "[UUID]"),
            (r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}", "[TIMESTAMP]"),
        ]
    }, {
        insta::assert_json_snapshot!(scenario, steps);
    });
}
```

**Step 2: Run to verify it compiles**

Run: `cargo check -p a2a-wasm-component --test server_integration_test`
Expected: Compiles without errors

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/tests/server_integration_test.rs
git commit -m "refactor(a2a-wasm-component): rewrite server_integration_test with test-case"
```

---

### Task 9: Run tests and create initial snapshots

**Step 1: Build the WASM component**

Run: `cargo build -p a2a-wasm-component --target wasm32-wasip2 --release`
Expected: Build succeeds

**Step 2: Run the integration tests**

Run: `cargo test -p a2a-wasm-component --test server_integration_test -- --test-threads=1`
Expected: Tests fail because snapshots don't exist yet

**Step 3: Review and accept snapshots**

Run: `cargo insta review`
Expected: Review each snapshot, accept if correct

**Step 4: Verify tests pass**

Run: `cargo test -p a2a-wasm-component --test server_integration_test -- --test-threads=1`
Expected: All 10 tests pass

**Step 5: Commit snapshots**

```bash
git add crates/a2a-wasm-component/tests/snapshots/
git commit -m "test(a2a-wasm-component): add server integration test snapshots"
```

---

### Task 10: Delete old test file

**Files:**
- Delete: `crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/test_wasm_server.py`

**Step 1: Delete the old monolithic test file**

```bash
rm crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/test_wasm_server.py
```

**Step 2: Verify tests still pass**

Run: `cargo test -p a2a-wasm-component --test server_integration_test -- --test-threads=1`
Expected: All 10 tests pass

**Step 3: Commit**

```bash
git add -u crates/a2a-wasm-component/tests/fixtures/wasm_server_tests/
git commit -m "refactor(a2a-wasm-component): remove monolithic test_wasm_server.py"
```

---

### Task 11: Final verification

**Step 1: Run all a2a-wasm-component tests**

Run: `cargo test -p a2a-wasm-component -- --test-threads=1`
Expected: All tests pass (both integration_test and server_integration_test)

**Step 2: Verify insta snapshots are up to date**

Run: `cargo insta test -p a2a-wasm-component --test server_integration_test`
Expected: All snapshots match, no pending changes
