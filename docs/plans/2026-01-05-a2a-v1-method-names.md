# A2A v1.0 JSON-RPC Method Names Migration

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Update all JSON-RPC method names from v0.x REST-style (`message/send`) to v1.0 gRPC-style (`SendMessage`).

**Architecture:** A2A v1.0 specifies PascalCase method names matching gRPC conventions. We need to update both client-side RPC calls and server-side routing to use the new names.

**Tech Stack:** Rust, a2a-client, a2a-wasm-component

---

## Background

Per [A2A v1.0 Specification Section 9.1](https://a2a-protocol.org/latest/specification/#9-json-rpc-protocol-binding):

> **Method Naming: PascalCase method names matching gRPC conventions (e.g., SendMessage, GetTask)**

| v0.x (old) | v1.0 (new) |
|------------|------------|
| `message/send` | `SendMessage` |
| `message/stream` | `SendStreamingMessage` |
| `tasks/get` | `GetTask` |
| `tasks/cancel` | `CancelTask` |
| `tasks/resubscribe` | `SubscribeToTask` |

---

### Task 1: Update WASM Component Server-Side Routing

**Files:**
- Modify: `crates/a2a-wasm-component/src/jsonrpc.rs:116-122`

**Step 1: Update method routing in handle_jsonrpc_request**

Change lines 116-122 from:
```rust
    let response = match request.method.as_str() {
        "message/send" => handle_message_send(&request),
        "tasks/get" => handle_tasks_get(&request),
        "tasks/cancel" => handle_tasks_cancel(&request),
        "message/stream" | "tasks/resubscribe" => {
            Response::error(request.id.clone(), -32601, "Streaming not implemented")
        }
```

To:
```rust
    let response = match request.method.as_str() {
        "SendMessage" => handle_message_send(&request),
        "GetTask" => handle_tasks_get(&request),
        "CancelTask" => handle_tasks_cancel(&request),
        "SendStreamingMessage" | "SubscribeToTask" => {
            Response::error(request.id.clone(), -32601, "Streaming not implemented")
        }
```

**Step 2: Build to verify no compile errors**

Run: `cargo build -p a2a-wasm-component`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/src/jsonrpc.rs
git commit -m "fix(a2a-wasm-component): use A2A v1.0 JSON-RPC method names in server routing"
```

---

### Task 2: Update WASM Component Client-Side RPC Calls

**Files:**
- Modify: `crates/a2a-wasm-component/src/client.rs:87,121,163`

**Step 1: Update send_message RPC call (line 87)**

Change:
```rust
        block_on(client.rpc("message/send", &a2a_params));
```

To:
```rust
        block_on(client.rpc("SendMessage", &a2a_params));
```

**Step 2: Update get_task RPC call (line 121)**

Change:
```rust
    let result: Result<a2a_types::Task, _> = block_on(client.rpc("tasks/get", &params));
```

To:
```rust
    let result: Result<a2a_types::Task, _> = block_on(client.rpc("GetTask", &params));
```

**Step 3: Update cancel_task RPC call (line 163)**

Change:
```rust
    let result: Result<a2a_types::Task, _> = block_on(client.rpc("tasks/cancel", &params));
```

To:
```rust
    let result: Result<a2a_types::Task, _> = block_on(client.rpc("CancelTask", &params));
```

**Step 4: Build to verify no compile errors**

Run: `cargo build -p a2a-wasm-component`
Expected: Build succeeds

**Step 5: Commit**

```bash
git add crates/a2a-wasm-component/src/client.rs
git commit -m "fix(a2a-wasm-component): use A2A v1.0 JSON-RPC method names in client RPC calls"
```

---

### Task 3: Verify a2a-client Already Updated

**Files:**
- Verify: `crates/a2a-client/src/lib.rs:109,199,294`
- Verify: `crates/a2a-client/src/jsonrpc.rs:52`

**Step 1: Verify lib.rs uses v1.0 method names**

Run: `grep -n '"SendMessage"\|"GetTask"\|"CancelTask"' crates/a2a-client/src/lib.rs`
Expected: Lines 109, 199, 294 show the new method names

**Step 2: Verify jsonrpc.rs test uses v1.0 method names**

Run: `grep -n '"SendMessage"' crates/a2a-client/src/jsonrpc.rs`
Expected: Line 52 shows `SendMessage`

**Step 3: Commit staged changes if any**

```bash
git add crates/a2a-client/
git commit -m "fix(a2a-client): use A2A v1.0 JSON-RPC method names"
```

---

### Task 4: Run Unit Tests

**Step 1: Run all unit tests**

Run: `cargo test --workspace`
Expected: All tests pass

**Step 2: Fix any failing tests**

If tests fail due to old method names in test assertions, update them.

---

### Task 5: Run Integration Tests (Manual Verification)

**Note:** Integration tests require the Python A2A server with the PR branch SDK.

**Step 1: Ensure test server is using updated SDK**

Run:
```bash
cd crates/a2a-wasm-component/tests/fixtures/helloworld
uv sync
```

**Step 2: Run integration tests**

Run: `cargo test -p a2a-wasm-component --test integration_test -- --nocapture`

Expected: Tests may still fail due to Part serialization format (separate task), but method routing errors should be resolved.

---

## Notes

- The `a2a-client` crate was already partially updated in the current session
- Integration tests may still fail due to Part serialization format mismatch (proto3 oneof vs JSON Schema `kind` discriminator) - this is a separate issue
- Old plans in `docs/plans/` still reference old method names - these are historical documents and don't need updating
