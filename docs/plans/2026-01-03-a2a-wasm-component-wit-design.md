# A2A WASM Component WIT Interface Design

## Goal

Align the `a2a-wasm-component` WIT interface with the A2A protocol specification, supporting both client (outgoing) and server (incoming) roles.

## WIT Interface

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

## A2A Protocol Alignment

| WIT | A2A JSON Schema | Notes |
|-----|-----------------|-------|
| `enum role { user, agent }` | `Role` | Match |
| `enum task-state { ... }` | `TaskState` | All 9 values |
| `record text-part { text }` | `TextPart` | metadata deferred |
| `record file-content { name?, mime-type?, uri?, bytes? }` | `FileContent` | Match |
| `record file-part { file }` | `FilePart` | metadata deferred |
| `record data-part { data, mime-type? }` | `DataPart` | metadata deferred |
| `variant part { text, file, data }` | `Part` | Match |
| `record message { role, parts, ... }` | `Message` | metadata deferred |
| `record artifact { artifact-id, name?, description?, parts }` | `Artifact` | metadata deferred |
| `record task-status { state, message?, timestamp? }` | `TaskStatus` | Match |
| `record task { id, context-id, status, history?, artifacts? }` | `Task` | metadata deferred |
| `record message-send-config { ... }` | `MessageSendConfiguration` | pushNotificationConfig omitted |
| `record message-send-params { message, configuration? }` | `MessageSendParams` | metadata deferred |
| `variant send-response { task, message }` | `SendMessageResponse` | Match |
| `record error { code, message }` | `JSONRPCError` | data deferred |

## Implementation Architecture

| Component | Responsibility |
|-----------|----------------|
| `src/lib.rs` | WIT bindings via `wit_bindgen::generate!`, implements `Guest` traits |
| `src/client.rs` | Implements `exports::a2a::protocol::client::Guest` trait |
| `src/server.rs` | Implements `exports::a2a::protocol::server::Guest` trait (stub) |
| `src/convert.rs` | Converts between WIT types and `a2a-types` generated types |

### Data Flow (client)

```
WIT send-message(agent-url, params)
  -> convert WIT params -> a2a-types::MessageSendParams
  -> a2a-client::Client::rpc("message/send", params)
  -> WasiHttpClient -> wasi:http/outgoing-handler
  -> parse response -> convert to WIT send-response
```

### Data Flow (server)

```
WIT on-message(params)
  -> return Err(error { code: -32601, message: "Not implemented" })
```

## Error Handling

| Scenario | Error Code | Message |
|----------|------------|---------|
| HTTP connection failed | `-32000` | Transport error details |
| Invalid agent URL | `-32001` | "Invalid agent URL" |
| JSON-RPC error from agent | Pass through | Pass through |
| Server not implemented | `-32601` | "Method not implemented" |

## Testing

| Test Type | Location | Approach |
|-----------|----------|----------|
| Unit tests | `src/convert.rs` | Test WIT <-> a2a-types conversions |
| Unit tests | `src/client.rs` | Mock `HttpClient` trait, verify JSON-RPC calls |
| Integration test | `tests/wasm_component.rs` | Build WASM, run with `wasmtime` (deferred) |

## Initial Scope

- **client**: Full implementation (text-part only, file-part and data-part return errors)
- **server**: Stub returning "not implemented" errors
- **metadata**: Deferred (requires json-value variant)

## Deferred Items

1. `json-value` variant for metadata support
2. `metadata` fields on all types
3. `reference-task-ids`, `extensions` on Message
4. `push-notification-config` in MessageSendConfig
5. `data` field on Error
6. `file-part` and `data-part` implementation
7. Server interface implementation
8. Integration tests with wasmtime
