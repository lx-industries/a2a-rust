# A2A WASM Component Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement WIT exports for a2a-wasm-component aligned with A2A protocol, supporting client and server interfaces.

**Architecture:** Component exports `client` interface (calls other A2A agents via wasi:http) and `server` interface (stub). Type conversions bridge WIT types to a2a-types. Only text-part implemented initially.

**Tech Stack:** wit-bindgen, a2a-types, a2a-client, a2a-transport-wasi, wasm32-wasip2

---

### Task 1: Update WIT Interface

**Files:**
- Modify: `crates/a2a-wasm-component/wit/a2a.wit`

**Step 1: Replace WIT file with new interface**

```wit
// crates/a2a-wasm-component/wit/a2a.wit
package a2a:protocol@0.2.0;

// TODO: Add json-value variant for metadata support
// variant json-value {
//     null,
//     bool(bool),
//     number(f64),
//     str(string),
//     array(list<json-value>),
//     object(list<tuple<string, json-value>>),
// }

interface types {
    /// Message sender role
    enum role {
        user,
        agent,
    }

    /// Task lifecycle state (A2A TaskState)
    enum task-state {
        submitted,
        working,
        input-required,
        completed,
        canceled,
        failed,
        rejected,
        auth-required,
        unknown,
    }

    /// Text content part
    record text-part {
        text: string,
        // TODO: metadata: option<list<tuple<string, json-value>>>,
    }

    /// File content reference or inline data
    record file-content {
        name: option<string>,
        mime-type: option<string>,
        uri: option<string>,
        bytes: option<list<u8>>,
    }

    /// File content part
    record file-part {
        file: file-content,
        // TODO: metadata: option<list<tuple<string, json-value>>>,
    }

    /// Structured data part (JSON)
    record data-part {
        data: string,
        mime-type: option<string>,
        // TODO: metadata: option<list<tuple<string, json-value>>>,
    }

    /// Content part within a message or artifact
    variant part {
        text(text-part),
        file(file-part),
        data(data-part),
    }

    /// A2A protocol message
    record message {
        role: role,
        parts: list<part>,
        message-id: option<string>,
        task-id: option<string>,
        context-id: option<string>,
        // TODO: reference-task-ids: option<list<string>>,
        // TODO: extensions: option<list<string>>,
        // TODO: metadata: option<list<tuple<string, json-value>>>,
    }

    /// Task output artifact
    record artifact {
        artifact-id: string,
        name: option<string>,
        description: option<string>,
        parts: list<part>,
        // TODO: extensions: option<list<string>>,
        // TODO: metadata: option<list<tuple<string, json-value>>>,
    }

    /// Task status with state and optional details
    record task-status {
        state: task-state,
        message: option<message>,
        timestamp: option<string>,
    }

    /// A2A task
    record task {
        id: string,
        context-id: string,
        status: task-status,
        history: option<list<message>>,
        artifacts: option<list<artifact>>,
        // TODO: metadata: option<list<tuple<string, json-value>>>,
    }

    /// Configuration for message/send
    record message-send-config {
        accepted-output-modes: option<list<string>>,
        history-length: option<u32>,
        blocking: option<bool>,
        // TODO: push-notification-config
    }

    /// Parameters for message/send
    record message-send-params {
        message: message,
        configuration: option<message-send-config>,
        // TODO: metadata: option<list<tuple<string, json-value>>>,
    }

    /// Response from message/send
    variant send-response {
        task(task),
        message(message),
    }

    /// JSON-RPC error
    record error {
        code: s32,
        message: string,
        // TODO: data: option<json-value>,
    }
}

/// Client interface for calling other A2A agents (outgoing requests)
interface client {
    use types.{task, message-send-params, send-response, error};

    /// Send a message to an A2A agent (JSON-RPC: message/send)
    send-message: func(
        agent-url: string,
        params: message-send-params
    ) -> result<send-response, error>;

    /// Get a task by ID (JSON-RPC: tasks/get)
    get-task: func(
        agent-url: string,
        id: string,
        history-length: option<u32>
    ) -> result<option<task>, error>;

    /// Cancel a task (JSON-RPC: tasks/cancel)
    cancel-task: func(
        agent-url: string,
        id: string
    ) -> result<option<task>, error>;
}

/// Server interface for handling incoming A2A requests (as an agent)
interface server {
    use types.{task, message-send-params, send-response, error};

    /// Handle incoming message/send request
    on-message: func(params: message-send-params) -> result<send-response, error>;

    /// Handle incoming tasks/get request
    on-get-task: func(id: string, history-length: option<u32>) -> result<option<task>, error>;

    /// Handle incoming tasks/cancel request
    on-cancel-task: func(id: string) -> result<option<task>, error>;
}

world a2a-component {
    export client;
    export server;
}
```

**Step 2: Verify WIT syntax**

Run: `cargo build -p a2a-wasm-component --target wasm32-wasip2 2>&1 | head -50`
Expected: May fail on Rust code, but WIT should parse (no "wit parse error")

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/wit/a2a.wit
git commit -m "feat(a2a-wasm-component): update WIT interface to align with A2A protocol

- Package: a2a:protocol@0.2.0
- Add full part types (text, file, data)
- Add proper task-state enum with all 9 A2A states
- Add message-send-params and message-send-config
- Add server interface with on-message, on-get-task, on-cancel-task
- Remove non-A2A list-tasks function
- Add agent-url parameter to client functions
"
```

---

### Task 2: Implement lib.rs with wit_bindgen

**Files:**
- Modify: `crates/a2a-wasm-component/src/lib.rs`
- Create: `crates/a2a-wasm-component/src/convert.rs`
- Create: `crates/a2a-wasm-component/src/client.rs`
- Create: `crates/a2a-wasm-component/src/server.rs`

**Step 1: Create module structure in lib.rs**

```rust
// crates/a2a-wasm-component/src/lib.rs
//! A2A WASM component with a2a:protocol interface.
//!
//! This component exports the A2A client and server interfaces for use in
//! WASM runtimes like Wassette.

mod client;
mod convert;
mod server;

// Generate WIT bindings
wit_bindgen::generate!({
    world: "a2a-component",
    path: "wit",
});

use exports::a2a::protocol::{client as client_exports, server as server_exports};

/// Component struct that implements both client and server interfaces.
struct Component;

export!(Component);

impl client_exports::Guest for Component {
    fn send_message(
        agent_url: String,
        params: client_exports::MessageSendParams,
    ) -> Result<client_exports::SendResponse, client_exports::Error> {
        client::send_message(agent_url, params)
    }

    fn get_task(
        agent_url: String,
        id: String,
        history_length: Option<u32>,
    ) -> Result<Option<client_exports::Task>, client_exports::Error> {
        client::get_task(agent_url, id, history_length)
    }

    fn cancel_task(
        agent_url: String,
        id: String,
    ) -> Result<Option<client_exports::Task>, client_exports::Error> {
        client::cancel_task(agent_url, id)
    }
}

impl server_exports::Guest for Component {
    fn on_message(
        params: server_exports::MessageSendParams,
    ) -> Result<server_exports::SendResponse, server_exports::Error> {
        server::on_message(params)
    }

    fn on_get_task(
        id: String,
        history_length: Option<u32>,
    ) -> Result<Option<server_exports::Task>, server_exports::Error> {
        server::on_get_task(id, history_length)
    }

    fn on_cancel_task(
        id: String,
    ) -> Result<Option<server_exports::Task>, server_exports::Error> {
        server::on_cancel_task(id)
    }
}
```

**Step 2: Create empty module files**

Create `crates/a2a-wasm-component/src/convert.rs`:
```rust
//! Type conversions between WIT types and a2a-types.

// Placeholder - will be implemented in Task 3
```

Create `crates/a2a-wasm-component/src/client.rs`:
```rust
//! Client interface implementation.

use crate::exports::a2a::protocol::client::{Error, MessageSendParams, SendResponse, Task};

pub fn send_message(
    _agent_url: String,
    _params: MessageSendParams,
) -> Result<SendResponse, Error> {
    Err(Error {
        code: -32601,
        message: "Not implemented".to_string(),
    })
}

pub fn get_task(
    _agent_url: String,
    _id: String,
    _history_length: Option<u32>,
) -> Result<Option<Task>, Error> {
    Err(Error {
        code: -32601,
        message: "Not implemented".to_string(),
    })
}

pub fn cancel_task(
    _agent_url: String,
    _id: String,
) -> Result<Option<Task>, Error> {
    Err(Error {
        code: -32601,
        message: "Not implemented".to_string(),
    })
}
```

Create `crates/a2a-wasm-component/src/server.rs`:
```rust
//! Server interface implementation (stub).

use crate::exports::a2a::protocol::server::{Error, MessageSendParams, SendResponse, Task};

pub fn on_message(_params: MessageSendParams) -> Result<SendResponse, Error> {
    Err(Error {
        code: -32601,
        message: "Server not implemented".to_string(),
    })
}

pub fn on_get_task(
    _id: String,
    _history_length: Option<u32>,
) -> Result<Option<Task>, Error> {
    Err(Error {
        code: -32601,
        message: "Server not implemented".to_string(),
    })
}

pub fn on_cancel_task(_id: String) -> Result<Option<Task>, Error> {
    Err(Error {
        code: -32601,
        message: "Server not implemented".to_string(),
    })
}
```

**Step 3: Verify build compiles**

Run: `cargo build -p a2a-wasm-component --target wasm32-wasip2`
Expected: Build succeeds (stub implementation)

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/src/
git commit -m "feat(a2a-wasm-component): add wit_bindgen scaffold with stub implementations

- Generate bindings for a2a-component world
- Implement client::Guest with stub returning -32601
- Implement server::Guest with stub returning -32601
- Add convert, client, server modules
"
```

---

### Task 3: Implement Type Conversions

**Files:**
- Modify: `crates/a2a-wasm-component/src/convert.rs`

**Step 1: Implement WIT to a2a-types conversions**

```rust
//! Type conversions between WIT types and a2a-types.

use crate::exports::a2a::protocol::client as wit;

/// Convert WIT Role to a2a-types Role
pub fn role_to_a2a(role: wit::Role) -> a2a_types::Role {
    match role {
        wit::Role::User => a2a_types::Role::User,
        wit::Role::Agent => a2a_types::Role::Agent,
    }
}

/// Convert a2a-types Role to WIT Role
pub fn role_from_a2a(role: a2a_types::Role) -> wit::Role {
    match role {
        a2a_types::Role::User => wit::Role::User,
        a2a_types::Role::Agent => wit::Role::Agent,
    }
}

/// Convert WIT TaskState to a2a-types TaskState
pub fn task_state_to_a2a(state: wit::TaskState) -> a2a_types::TaskState {
    match state {
        wit::TaskState::Submitted => a2a_types::TaskState::Submitted,
        wit::TaskState::Working => a2a_types::TaskState::Working,
        wit::TaskState::InputRequired => a2a_types::TaskState::InputRequired,
        wit::TaskState::Completed => a2a_types::TaskState::Completed,
        wit::TaskState::Canceled => a2a_types::TaskState::Canceled,
        wit::TaskState::Failed => a2a_types::TaskState::Failed,
        wit::TaskState::Rejected => a2a_types::TaskState::Rejected,
        wit::TaskState::AuthRequired => a2a_types::TaskState::AuthRequired,
        wit::TaskState::Unknown => a2a_types::TaskState::Unknown,
    }
}

/// Convert a2a-types TaskState to WIT TaskState
pub fn task_state_from_a2a(state: a2a_types::TaskState) -> wit::TaskState {
    match state {
        a2a_types::TaskState::Submitted => wit::TaskState::Submitted,
        a2a_types::TaskState::Working => wit::TaskState::Working,
        a2a_types::TaskState::InputRequired => wit::TaskState::InputRequired,
        a2a_types::TaskState::Completed => wit::TaskState::Completed,
        a2a_types::TaskState::Canceled => wit::TaskState::Canceled,
        a2a_types::TaskState::Failed => wit::TaskState::Failed,
        a2a_types::TaskState::Rejected => wit::TaskState::Rejected,
        a2a_types::TaskState::AuthRequired => wit::TaskState::AuthRequired,
        a2a_types::TaskState::Unknown => wit::TaskState::Unknown,
    }
}

/// Convert WIT Part to a2a-types Part (only TextPart supported)
pub fn part_to_a2a(part: wit::Part) -> Result<a2a_types::Part, &'static str> {
    match part {
        wit::Part::Text(text_part) => Ok(a2a_types::Part::Text(a2a_types::TextPart {
            kind: "text".to_string(),
            text: text_part.text,
            metadata: Default::default(),
        })),
        wit::Part::File(_) => Err("FilePart not implemented"),
        wit::Part::Data(_) => Err("DataPart not implemented"),
    }
}

/// Convert a2a-types Part to WIT Part
pub fn part_from_a2a(part: &a2a_types::Part) -> wit::Part {
    match part {
        a2a_types::Part::Text(text_part) => wit::Part::Text(wit::TextPart {
            text: text_part.text.clone(),
        }),
        a2a_types::Part::File(file_part) => wit::Part::File(wit::FilePart {
            file: wit::FileContent {
                name: file_part.file.name.clone(),
                mime_type: file_part.file.mime_type.clone(),
                uri: file_part.file.uri.clone(),
                bytes: file_part.file.bytes.as_ref().map(|b| {
                    use base64::Engine;
                    base64::engine::general_purpose::STANDARD
                        .decode(b)
                        .unwrap_or_default()
                }),
            },
        }),
        a2a_types::Part::Data(data_part) => wit::Part::Data(wit::DataPart {
            data: serde_json::to_string(&data_part.data).unwrap_or_default(),
            mime_type: data_part.mime_type.clone(),
        }),
    }
}

/// Convert WIT Message to a2a-types Message
pub fn message_to_a2a(msg: wit::Message) -> Result<a2a_types::Message, &'static str> {
    let parts: Result<Vec<_>, _> = msg.parts.into_iter().map(part_to_a2a).collect();
    Ok(a2a_types::Message {
        role: role_to_a2a(msg.role),
        parts: parts?,
        message_id: msg.message_id,
        task_id: msg.task_id,
        context_id: msg.context_id,
        reference_task_ids: vec![],
        extensions: vec![],
        metadata: Default::default(),
        kind: Some("message".to_string()),
    })
}

/// Convert a2a-types Message to WIT Message
pub fn message_from_a2a(msg: &a2a_types::Message) -> wit::Message {
    wit::Message {
        role: role_from_a2a(msg.role.clone()),
        parts: msg.parts.iter().map(part_from_a2a).collect(),
        message_id: msg.message_id.clone(),
        task_id: msg.task_id.clone(),
        context_id: msg.context_id.clone(),
    }
}

/// Convert a2a-types Artifact to WIT Artifact
pub fn artifact_from_a2a(artifact: &a2a_types::Artifact) -> wit::Artifact {
    wit::Artifact {
        artifact_id: artifact.artifact_id.clone(),
        name: artifact.name.clone(),
        description: artifact.description.clone(),
        parts: artifact.parts.iter().map(part_from_a2a).collect(),
    }
}

/// Convert a2a-types TaskStatus to WIT TaskStatus
pub fn task_status_from_a2a(status: &a2a_types::TaskStatus) -> wit::TaskStatus {
    wit::TaskStatus {
        state: task_state_from_a2a(status.state.clone()),
        message: status.message.as_ref().map(message_from_a2a),
        timestamp: status.timestamp.clone(),
    }
}

/// Convert a2a-types Task to WIT Task
pub fn task_from_a2a(task: &a2a_types::Task) -> wit::Task {
    wit::Task {
        id: task.id.clone(),
        context_id: task.context_id.clone(),
        status: task_status_from_a2a(&task.status),
        history: task.history.as_ref().map(|h| h.iter().map(message_from_a2a).collect()),
        artifacts: task.artifacts.as_ref().map(|a| a.iter().map(artifact_from_a2a).collect()),
    }
}

/// Convert WIT MessageSendParams to a2a-types MessageSendParams
pub fn message_send_params_to_a2a(
    params: wit::MessageSendParams,
) -> Result<a2a_types::MessageSendParams, &'static str> {
    Ok(a2a_types::MessageSendParams {
        message: message_to_a2a(params.message)?,
        configuration: params.configuration.map(|c| a2a_types::MessageSendConfiguration {
            accepted_output_modes: c.accepted_output_modes.unwrap_or_default(),
            history_length: c.history_length.map(|h| h as i64),
            blocking: c.blocking,
            push_notification_config: None,
        }),
        metadata: Default::default(),
    })
}

/// Convert a2a-types SendMessageResponse to WIT SendResponse
pub fn send_response_from_a2a(response: a2a_types::SendMessageResponse) -> wit::SendResponse {
    match response {
        a2a_types::SendMessageResponse::Task(task) => {
            wit::SendResponse::Task(task_from_a2a(&task))
        }
        a2a_types::SendMessageResponse::Message(msg) => {
            wit::SendResponse::Message(message_from_a2a(&msg))
        }
    }
}
```

**Step 2: Add base64 dependency to Cargo.toml**

Modify `crates/a2a-wasm-component/Cargo.toml` to add:
```toml
base64 = "0.22"
```

**Step 3: Verify build compiles**

Run: `cargo build -p a2a-wasm-component --target wasm32-wasip2`
Expected: Build succeeds

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/
git commit -m "feat(a2a-wasm-component): implement type conversions

- Add conversions for Role, TaskState, Part, Message, Task
- Support TextPart only (FilePart/DataPart return errors)
- Add artifact, task-status conversions
- Add message-send-params conversion
"
```

---

### Task 4: Implement Client send_message

**Files:**
- Modify: `crates/a2a-wasm-component/src/client.rs`

**Step 1: Implement send_message with WasiHttpClient**

```rust
//! Client interface implementation.

use crate::convert;
use crate::exports::a2a::protocol::client::{Error, MessageSendParams, SendResponse, Task};
use a2a_client::Client;
use a2a_transport_wasi::WasiHttpClient;

/// Simple blocking executor for WASI async operations.
fn block_on<F: std::future::Future>(future: F) -> F::Output {
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    fn noop_clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    fn noop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);
    let raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw_waker) };
    let mut cx = Context::from_waker(&waker);

    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(result) => return result,
            Poll::Pending => {
                // WASI pollables handle blocking internally
            }
        }
    }
}

fn make_error(code: i32, message: impl Into<String>) -> Error {
    Error {
        code,
        message: message.into(),
    }
}

pub fn send_message(agent_url: String, params: MessageSendParams) -> Result<SendResponse, Error> {
    // Convert WIT params to a2a-types
    let a2a_params = convert::message_send_params_to_a2a(params)
        .map_err(|e| make_error(-32001, e))?;

    // Create HTTP client and A2A client
    let http_client = WasiHttpClient::new();
    let client = Client::new(&http_client, &agent_url);

    // Make the RPC call
    let result = block_on(client.rpc::<_, a2a_types::SendMessageResponse>(
        "message/send",
        a2a_params,
    ));

    match result {
        Ok(response) => Ok(convert::send_response_from_a2a(response)),
        Err(e) => match e {
            a2a_client::Error::JsonRpc { code, message, .. } => {
                Err(make_error(code, message))
            }
            a2a_client::Error::Transport(msg) => Err(make_error(-32000, msg)),
            _ => Err(make_error(-32000, e.to_string())),
        },
    }
}

pub fn get_task(
    agent_url: String,
    id: String,
    history_length: Option<u32>,
) -> Result<Option<Task>, Error> {
    let http_client = WasiHttpClient::new();
    let client = Client::new(&http_client, &agent_url);

    let params = a2a_types::TaskQueryParams {
        id,
        history_length: history_length.map(|h| h as i64),
        metadata: Default::default(),
    };

    let result = block_on(client.rpc::<_, a2a_types::Task>("tasks/get", params));

    match result {
        Ok(task) => Ok(Some(convert::task_from_a2a(&task))),
        Err(a2a_client::Error::JsonRpc { code: -32001, .. }) => Ok(None), // Task not found
        Err(e) => match e {
            a2a_client::Error::JsonRpc { code, message, .. } => {
                Err(make_error(code, message))
            }
            a2a_client::Error::Transport(msg) => Err(make_error(-32000, msg)),
            _ => Err(make_error(-32000, e.to_string())),
        },
    }
}

pub fn cancel_task(agent_url: String, id: String) -> Result<Option<Task>, Error> {
    let http_client = WasiHttpClient::new();
    let client = Client::new(&http_client, &agent_url);

    let params = a2a_types::TaskIdParams {
        id,
        metadata: Default::default(),
    };

    let result = block_on(client.rpc::<_, a2a_types::Task>("tasks/cancel", params));

    match result {
        Ok(task) => Ok(Some(convert::task_from_a2a(&task))),
        Err(a2a_client::Error::JsonRpc { code: -32001, .. }) => Ok(None), // Task not found
        Err(e) => match e {
            a2a_client::Error::JsonRpc { code, message, .. } => {
                Err(make_error(code, message))
            }
            a2a_client::Error::Transport(msg) => Err(make_error(-32000, msg)),
            _ => Err(make_error(-32000, e.to_string())),
        },
    }
}
```

**Step 2: Verify build compiles**

Run: `cargo build -p a2a-wasm-component --target wasm32-wasip2`
Expected: Build succeeds

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/src/client.rs
git commit -m "feat(a2a-wasm-component): implement client interface

- Implement send_message with WasiHttpClient
- Implement get_task and cancel_task
- Add block_on executor for async operations
- Convert between WIT and a2a-types
"
```

---

### Task 5: Verify Full WASM Build

**Files:**
- None (verification only)

**Step 1: Run full build**

Run: `cargo build -p a2a-wasm-component --target wasm32-wasip2 --release`
Expected: Build succeeds

**Step 2: Check component size**

Run: `ls -lh target/wasm32-wasip2/release/a2a_wasm_component.wasm`
Expected: Shows file size (should be reasonable, < 5MB)

**Step 3: Run clippy**

Run: `cargo clippy -p a2a-wasm-component --target wasm32-wasip2 -- -D warnings`
Expected: No warnings

**Step 4: Commit any fixes**

If clippy found issues, fix them and commit:
```bash
git add crates/a2a-wasm-component/
git commit -m "fix(a2a-wasm-component): address clippy warnings"
```

---

### Task 6: Add Unit Tests for Conversions

**Files:**
- Modify: `crates/a2a-wasm-component/src/convert.rs`

**Step 1: Add unit tests to convert.rs**

Add at the end of `convert.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_roundtrip() {
        assert!(matches!(
            role_from_a2a(role_to_a2a(wit::Role::User)),
            wit::Role::User
        ));
        assert!(matches!(
            role_from_a2a(role_to_a2a(wit::Role::Agent)),
            wit::Role::Agent
        ));
    }

    #[test]
    fn test_task_state_roundtrip() {
        let states = [
            wit::TaskState::Submitted,
            wit::TaskState::Working,
            wit::TaskState::Completed,
            wit::TaskState::Failed,
            wit::TaskState::Canceled,
        ];
        for state in states {
            let a2a = task_state_to_a2a(state.clone());
            let back = task_state_from_a2a(a2a);
            assert_eq!(
                std::mem::discriminant(&state),
                std::mem::discriminant(&back)
            );
        }
    }

    #[test]
    fn test_text_part_to_a2a() {
        let wit_part = wit::Part::Text(wit::TextPart {
            text: "hello".to_string(),
        });
        let a2a_part = part_to_a2a(wit_part).unwrap();
        assert!(matches!(a2a_part, a2a_types::Part::Text(t) if t.text == "hello"));
    }

    #[test]
    fn test_file_part_not_implemented() {
        let wit_part = wit::Part::File(wit::FilePart {
            file: wit::FileContent {
                name: None,
                mime_type: None,
                uri: None,
                bytes: None,
            },
        });
        assert!(part_to_a2a(wit_part).is_err());
    }
}
```

**Step 2: Run tests**

Run: `cargo test -p a2a-wasm-component`
Expected: All tests pass

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/src/convert.rs
git commit -m "test(a2a-wasm-component): add unit tests for type conversions"
```

---

### Task 7: Update Documentation

**Files:**
- Modify: `crates/a2a-wasm-component/src/lib.rs`

**Step 1: Add module-level documentation**

Update the doc comment at the top of `lib.rs`:

```rust
//! A2A WASM component with a2a:protocol interface.
//!
//! This component exports the A2A client and server interfaces for use in
//! WASM runtimes like Wassette. It allows WASM components to communicate
//! with A2A agents using the wasi:http transport.
//!
//! # Interfaces
//!
//! - **client**: Call other A2A agents (outgoing requests)
//!   - `send-message`: Send a message to an agent
//!   - `get-task`: Get task status by ID
//!   - `cancel-task`: Cancel a running task
//!
//! - **server**: Handle incoming A2A requests (stub, not implemented)
//!   - `on-message`: Handle message/send
//!   - `on-get-task`: Handle tasks/get
//!   - `on-cancel-task`: Handle tasks/cancel
//!
//! # Limitations
//!
//! - Only `TextPart` is supported; `FilePart` and `DataPart` return errors
//! - Server interface returns "not implemented" errors
//! - Metadata fields are not supported (deferred)
//!
//! # Example
//!
//! ```ignore
//! // From a WASM runtime, call the exported client interface:
//! let response = client::send_message(
//!     "https://agent.example.com",
//!     MessageSendParams {
//!         message: Message {
//!             role: Role::User,
//!             parts: vec![Part::Text(TextPart { text: "Hello".into() })],
//!             ..Default::default()
//!         },
//!         configuration: None,
//!     },
//! );
//! ```
```

**Step 2: Commit**

```bash
git add crates/a2a-wasm-component/src/lib.rs
git commit -m "docs(a2a-wasm-component): add module documentation"
```

---

## Summary

| Task | Description | Files |
|------|-------------|-------|
| 1 | Update WIT interface | `wit/a2a.wit` |
| 2 | Implement lib.rs scaffold | `src/lib.rs`, `src/*.rs` |
| 3 | Implement type conversions | `src/convert.rs`, `Cargo.toml` |
| 4 | Implement client interface | `src/client.rs` |
| 5 | Verify WASM build | (verification) |
| 6 | Add unit tests | `src/convert.rs` |
| 7 | Update documentation | `src/lib.rs` |
