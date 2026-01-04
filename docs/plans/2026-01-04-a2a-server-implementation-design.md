# A2A Server Implementation Design

## Overview

Implement the server interface for `a2a-wasm-component` by importing an `agent` interface from the host runtime. The component handles A2A protocol concerns while delegating actual agent logic to the host.

## Architecture

```
External A2A Client
    │ HTTP (JSON-RPC)
    ▼
┌─────────────────────────────────────────────────────────┐
│ WASM Host Runtime (e.g., Wassette)                      │
│   • Handles HTTP protocol                               │
│   • Provides agent interface implementation             │
└─────────────────────────┬───────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────┐
│ a2a-wasm-component                                      │
│   • Handles A2A protocol                                │
│   import agent { on-message, on-get-task, on-cancel }   │
│   export server { on-message, on-get-task, on-cancel }  │
│   export client { send-message, get-task, cancel-task } │
└─────────────────────────────────────────────────────────┘
```

### Layer Responsibilities

| Layer | Responsibility |
|-------|----------------|
| **HTTP** (host runtime) | Parse JSON-RPC, route requests, send HTTP responses |
| **A2A Protocol** (component) | Protocol compliance, type handling, pass-through to agent |
| **Agent Logic** (host via import) | Process messages, manage tasks, business logic |

## WIT Interface Changes

Add an `agent` interface that the component imports, mirroring the `server` interface:

```wit
// crates/a2a-wasm-component/wit/a2a.wit

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

world a2a-component {
    import agent;
    export client;
    export server;
}
```

### Type Sharing

Both `server` (export) and `agent` (import) use the same `types` interface:

```wit
interface server {
    use types.{task, message-send-params, send-response, error};
    // ...
}

interface agent {
    use types.{task, message-send-params, send-response, error};
    // ...
}
```

This means wit-bindgen generates compatible types with no conversion needed.

## Server Implementation

The server export becomes a pass-through to the imported agent interface:

```rust
// crates/a2a-wasm-component/src/server.rs

use crate::bindings::a2a::protocol::agent;
use crate::exports::a2a::protocol::server::{Error, MessageSendParams, SendResponse, Task};

pub fn on_message(params: MessageSendParams) -> Result<SendResponse, Error> {
    agent::on_message(params)
}

pub fn on_get_task(id: String, history_length: Option<u32>) -> Result<Option<Task>, Error> {
    agent::on_get_task(&id, history_length)
}

pub fn on_cancel_task(id: String) -> Result<Option<Task>, Error> {
    agent::on_cancel_task(&id)
}
```

## Error Handling

Errors flow through three mechanisms (as designed earlier):

| Scenario | Error Handling |
|----------|----------------|
| Agent processed but failed | Agent returns `Task` with `TaskState::Failed` |
| Protocol error | Agent returns `Error` with appropriate code |
| Host/transport error | Agent returns `Error` with code `-32000` |

Standard JSON-RPC error codes:
- `-32000` - Transport/internal error
- `-32001` - Not found (task, agent)
- `-32601` - Method not found
- `-32602` - Invalid params

## Testing Strategy

### Unit Tests

Mock the imported agent interface to verify server behavior:

```rust
// Test that server correctly passes through to agent
#[test]
fn test_on_message_passes_through() {
    // Setup mock agent that returns a known response
    // Call server.on_message
    // Verify agent.on_message was called with same params
    // Verify response matches agent's response
}
```

### Integration Tests

Use existing test infrastructure with a simple echo agent:

```rust
impl agent::Guest for EchoAgent {
    fn on_message(params: MessageSendParams) -> Result<SendResponse, Error> {
        Ok(SendResponse::Task(Task {
            id: generate_id(),
            context_id: params.message.context_id.unwrap_or_default(),
            status: TaskStatus {
                state: TaskState::Completed,
                message: Some(params.message),
                timestamp: Some(now()),
            },
            history: None,
            artifacts: None,
        }))
    }

    fn on_get_task(id: String, _: Option<u32>) -> Result<Option<Task>, Error> {
        // Return stored task or None
    }

    fn on_cancel_task(id: String) -> Result<Option<Task>, Error> {
        // Mark task as canceled and return
    }
}
```

## Out of Scope

| Feature | Status | Reason |
|---------|--------|--------|
| Task storage in component | Deferred | Agent/host handles storage |
| FilePart / DataPart support | Deferred | Existing WIT TODOs |
| Streaming responses | Deferred | WIT only has blocking operations |
| Metadata fields | Deferred | Requires json-value variant |
| history_length trimming in component | Deferred | Agent can handle this |

## Implementation Tasks

1. Update `wit/a2a.wit` - Add `agent` interface, update world
2. Update `src/lib.rs` - Add agent import bindings
3. Update `src/server.rs` - Implement pass-through to agent
4. Add unit tests for server pass-through behavior
5. Update documentation to reflect new architecture

## Use Case: Wassette Integration

The primary use case is Microsoft Wassette:

- **Host runtime**: Wassette handles HTTP, provides agent implementation
- **Component exports**: `server` receives A2A requests, `client` exposed as MCP tools
- **Component imports**: `agent` interface implemented by Wassette (may call LLM, compose with other components, etc.)

This enables an LLM to:
1. Receive A2A requests (via server → agent → LLM)
2. Call other A2A agents (via client MCP tools)
