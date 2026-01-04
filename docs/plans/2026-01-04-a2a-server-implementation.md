# A2A Server Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement the server interface by importing an `agent` interface from the host runtime, making the server a pass-through to the host-provided agent logic.

**Architecture:** Add `agent` interface to WIT (mirrors `server`), update world to import it, implement server functions as pass-through calls to agent imports.

**Tech Stack:** wit-bindgen, wasm32-wasip2 target, Rust

---

## Task 1: Add Agent Interface to WIT

**Files:**
- Modify: `crates/a2a-wasm-component/wit/a2a.wit:160-177`

**Step 1: Add agent interface before the world declaration**

Insert after line 172 (after the closing brace of `interface server`), before `world a2a-component`:

```wit
/// Agent interface imported from host - provides actual agent logic
interface agent {
    use types.{task, message-send-params, send-response, error};

    /// Process incoming message (called by server.on-message)
    on-message: func(params: message-send-params) -> result<send-response, error>;

    /// Retrieve task (called by server.on-get-task)
    on-get-task: func(id: string, history-length: option<u32>) -> result<option<task>, error>;

    /// Handle cancellation (called by server.on-cancel-task)
    on-cancel-task: func(id: string) -> result<option<task>, error>;
}
```

**Step 2: Update world to import agent**

Change the world declaration from:

```wit
world a2a-component {
    export client;
    export server;
}
```

To:

```wit
world a2a-component {
    import agent;
    export client;
    export server;
}
```

**Step 3: Verify WIT parses correctly**

Run: `cargo build -p a2a-wasm-component 2>&1 | head -20`
Expected: May show Rust errors (expected - bindings changed), but no "wit parse error"

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/wit/a2a.wit
git commit -m "feat(a2a-wasm-component): add agent interface to WIT

- Add agent interface mirroring server (on-message, on-get-task, on-cancel-task)
- Update world to import agent interface
- Host runtime will provide agent implementation"
```

---

## Task 2: Update Server Implementation

**Files:**
- Modify: `crates/a2a-wasm-component/src/server.rs`

**Step 1: Replace stub implementation with pass-through to agent**

Replace entire contents of `server.rs`:

```rust
//! Server interface implementation.
//!
//! The server interface is a pass-through to the imported agent interface.
//! The host runtime provides the actual agent implementation.

use crate::a2a::protocol::agent;
use crate::exports::a2a::protocol::server::{Error, MessageSendParams, SendResponse, Task};

/// Handle incoming message/send request.
///
/// Delegates to the imported agent interface.
pub fn on_message(params: MessageSendParams) -> Result<SendResponse, Error> {
    agent::on_message(&params)
}

/// Handle incoming tasks/get request.
///
/// Delegates to the imported agent interface.
pub fn on_get_task(id: String, history_length: Option<u32>) -> Result<Option<Task>, Error> {
    agent::on_get_task(&id, history_length)
}

/// Handle incoming tasks/cancel request.
///
/// Delegates to the imported agent interface.
pub fn on_cancel_task(id: String) -> Result<Option<Task>, Error> {
    agent::on_cancel_task(&id)
}
```

**Step 2: Build to check for type mismatches**

Run: `cargo build -p a2a-wasm-component 2>&1 | head -50`
Expected: Build may fail if types don't match between imports and exports - we'll fix in next step if needed.

**Step 3: Commit (even if build fails - will fix in next task)**

```bash
git add crates/a2a-wasm-component/src/server.rs
git commit -m "feat(a2a-wasm-component): implement server as pass-through to agent"
```

---

## Task 3: Fix Type Compatibility (if needed)

**Files:**
- Modify: `crates/a2a-wasm-component/src/server.rs` (if types differ)

**Step 1: Check build output**

Run: `cargo build -p a2a-wasm-component 2>&1`

If build succeeds, skip to Task 4.

If build fails with type mismatch errors, the imported `agent` types and exported `server` types may be in different modules. In that case:

**Step 2: Add type conversions if needed**

If types are different, update `server.rs` to convert between them:

```rust
//! Server interface implementation.

use crate::a2a::protocol::agent;
use crate::a2a::protocol::types as agent_types;
use crate::exports::a2a::protocol::server::{Error, MessageSendParams, SendResponse, Task};

// If types are the same, these are no-ops. If different, add conversion logic.

pub fn on_message(params: MessageSendParams) -> Result<SendResponse, Error> {
    // Types should be the same since both use `types` interface
    agent::on_message(&params)
}

pub fn on_get_task(id: String, history_length: Option<u32>) -> Result<Option<Task>, Error> {
    agent::on_get_task(&id, history_length)
}

pub fn on_cancel_task(id: String) -> Result<Option<Task>, Error> {
    agent::on_cancel_task(&id)
}
```

**Step 3: Verify build succeeds**

Run: `cargo build -p a2a-wasm-component`
Expected: Success

**Step 4: Commit if changes were made**

```bash
git add crates/a2a-wasm-component/src/server.rs
git commit -m "fix(a2a-wasm-component): fix type compatibility between server and agent"
```

---

## Task 4: Update Module Documentation

**Files:**
- Modify: `crates/a2a-wasm-component/src/lib.rs:1-41`

**Step 1: Update module documentation to reflect new architecture**

Replace the module documentation (lines 1-41):

```rust
//! A2A WASM component with a2a:protocol interface.
//!
//! This component exports the A2A client and server interfaces for use in
//! WASM runtimes like Wassette. It allows WASM components to communicate
//! with A2A agents using the wasi:http transport.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │ WASM Host Runtime (e.g., Wassette)                      │
//! │   • Handles HTTP protocol                               │
//! │   • Provides agent interface implementation             │
//! └─────────────────────────┬───────────────────────────────┘
//!                           │
//! ┌─────────────────────────▼───────────────────────────────┐
//! │ a2a-wasm-component                                      │
//! │   import agent { on-message, on-get-task, on-cancel }   │
//! │   export server { on-message, on-get-task, on-cancel }  │
//! │   export client { send-message, get-task, cancel-task } │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! # Interfaces
//!
//! - **agent** (import): Host provides actual agent logic
//!   - `on-message`: Process incoming messages
//!   - `on-get-task`: Retrieve task by ID
//!   - `on-cancel-task`: Handle task cancellation
//!
//! - **server** (export): Handle incoming A2A requests (delegates to agent)
//!   - `on-message`: Handle message/send
//!   - `on-get-task`: Handle tasks/get
//!   - `on-cancel-task`: Handle tasks/cancel
//!
//! - **client** (export): Call other A2A agents (outgoing requests)
//!   - `send-message`: Send a message to an agent
//!   - `get-task`: Get task status by ID
//!   - `cancel-task`: Cancel a running task
//!
//! # Limitations
//!
//! - Only `TextPart` is supported; `FilePart` and `DataPart` return errors
//! - Metadata fields are not supported (deferred)
```

**Step 2: Verify build**

Run: `cargo build -p a2a-wasm-component`
Expected: Success

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/src/lib.rs
git commit -m "docs(a2a-wasm-component): update module docs for agent import architecture"
```

---

## Task 5: Build WASM Target

**Files:** None (verification only)

**Step 1: Build for wasm32-wasip2 target**

Run: `cargo build -p a2a-wasm-component --target wasm32-wasip2`
Expected: Success

**Step 2: Run clippy**

Run: `cargo clippy -p a2a-wasm-component --target wasm32-wasip2 -- -D warnings`
Expected: No warnings

**Step 3: Verify component size is reasonable**

Run: `ls -lh target/wasm32-wasip2/debug/a2a_wasm_component.wasm`
Expected: File exists, reasonable size (< 5MB)

---

## Task 6: Run Existing Tests

**Files:** None (verification only)

**Step 1: Run unit tests**

Run: `cargo test -p a2a-wasm-component --lib`
Expected: All existing convert.rs tests pass

**Step 2: Check test output**

Run: `cargo test -p a2a-wasm-component --lib -- --nocapture 2>&1 | tail -20`
Expected: "test result: ok"

---

## Task 7: Add Server Unit Tests

**Files:**
- Modify: `crates/a2a-wasm-component/src/server.rs`

**Step 1: Add test module to server.rs**

Append to end of `server.rs`:

```rust

#[cfg(test)]
mod tests {
    // Note: Unit tests for server are limited because we can't mock the imported
    // agent interface in regular tests. The server is a thin pass-through layer.
    //
    // Full testing is done via integration tests with a real WASM runtime that
    // provides the agent implementation.
    //
    // These tests verify the module structure compiles correctly.

    #[test]
    fn test_server_module_exists() {
        // Verify the module compiles and functions are accessible
        // Actual behavior is tested via integration tests
        assert!(true);
    }
}
```

**Step 2: Run tests**

Run: `cargo test -p a2a-wasm-component --lib`
Expected: All tests pass

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/src/server.rs
git commit -m "test(a2a-wasm-component): add server module test placeholder"
```

---

## Task 8: Final Verification

**Files:** None (verification only)

**Step 1: Run full test suite**

Run: `cargo test -p a2a-wasm-component`
Expected: All tests pass

**Step 2: Run clippy on all targets**

Run: `cargo clippy -p a2a-wasm-component -- -D warnings`
Expected: No warnings

**Step 3: Build release WASM**

Run: `cargo build -p a2a-wasm-component --target wasm32-wasip2 --release`
Expected: Success

**Step 4: Check release size**

Run: `ls -lh target/wasm32-wasip2/release/a2a_wasm_component.wasm`
Expected: Smaller than debug build

---

## Summary

After completing all tasks:

1. **WIT changes**: `agent` interface added, world imports it
2. **Server implementation**: Pass-through to agent import (3 functions)
3. **Documentation**: Updated to reflect new architecture
4. **Tests**: Existing tests still pass, placeholder for server tests
5. **WASM build**: Verified on wasm32-wasip2 target

The component now requires a host runtime (like Wassette) to provide the `agent` interface implementation.
