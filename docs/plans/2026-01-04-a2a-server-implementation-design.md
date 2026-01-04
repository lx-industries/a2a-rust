# A2A Server Implementation Design

## Overview

Implement the server as an HTTP handler that exports `wasi:http/incoming-handler`. The component handles the full HTTP layer (JSON-RPC parsing, routing, SSE streaming) while delegating agent logic to an imported interface.

## Architecture

```
External A2A Client
    │ HTTP
    ▼
┌─────────────────────────────────────────────────────────────┐
│ WASM Host Runtime (e.g., Wassette)                          │
│   • TCP/TLS termination                                     │
│   • Provides agent interface implementation                 │
└─────────────────────────┬───────────────────────────────────┘
                          │
┌─────────────────────────▼───────────────────────────────────┐
│ a2a-wasm-component                                          │
│   export wasi:http/incoming-handler { handle }              │
│   import agent { on-message, on-get-task, on-cancel,        │
│                  get-agent-card, on-stream-message }        │
│   export client { send-message, get-task, cancel-task }     │
└─────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities

| Layer | Responsibility |
|-------|----------------|
| **Host Runtime** | TCP/TLS, process management, provides agent implementation |
| **HTTP Handler** (component) | Parse HTTP, route requests, JSON-RPC encode/decode, SSE formatting |
| **Agent Logic** (host via import) | Process messages, manage tasks, generate responses, stream events |

## HTTP Endpoints

### GET /.well-known/agent.json

Returns the agent card for discovery.

```
Request:  GET /.well-known/agent.json
          Accept: application/json

Response: 200 OK
          Content-Type: application/json

          { "name": "...", "capabilities": { "streaming": true, ... } }
```

### POST / (JSON-RPC)

All A2A operations use JSON-RPC over POST.

| Method | Agent Function | Response Type |
|--------|---------------|---------------|
| `message/send` | `on-message` | JSON-RPC (Task or Message) |
| `message/stream` | `on-stream-message` | SSE stream |
| `tasks/get` | `on-get-task` | JSON-RPC (Task or null) |
| `tasks/cancel` | `on-cancel-task` | JSON-RPC (Task or null) |
| `tasks/resubscribe` | `on-resubscribe` | SSE stream |

## WIT Interface Changes

### Remove `server` interface, add HTTP handler export

```wit
// crates/a2a-wasm-component/wit/a2a.wit

/// Agent interface imported from host - provides actual agent logic
interface agent {
    use types.{task, message-send-params, send-response, error};

    /// Get agent card (JSON string)
    get-agent-card: func() -> result<string, error>;

    /// Process incoming message (blocking)
    on-message: func(params: message-send-params) -> result<send-response, error>;

    /// Retrieve task by ID
    on-get-task: func(id: string, history-length: option<u32>) -> result<option<task>, error>;

    /// Handle cancellation
    on-cancel-task: func(id: string) -> result<option<task>, error>;
}

world a2a-component {
    import wasi:http/outgoing-handler;
    import agent;

    export wasi:http/incoming-handler;  // HTTP server
    export client;                      // Client interface (for MCP tools)
}
```

Note: The old `server` interface is removed entirely - its functionality is now in the HTTP handler.

## HTTP Handler Implementation

### Request Flow

```rust
// Pseudocode for wasi:http/incoming-handler::handle

fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
    let method = request.method();
    let path = request.path_with_query().unwrap_or("/");

    match (method, path.as_str()) {
        (Method::Get, "/.well-known/agent.json") => {
            handle_agent_card(response_out)
        }
        (Method::Post, "/" | "") => {
            let body = read_body(&request);
            handle_jsonrpc(body, response_out)
        }
        _ => {
            send_error(response_out, 404, "Not Found")
        }
    }
}

fn handle_jsonrpc(body: Vec<u8>, response_out: ResponseOutparam) {
    let request: JsonRpcRequest = serde_json::from_slice(&body)?;

    match request.method.as_str() {
        "message/send" => handle_message_send(request, response_out),
        "message/stream" => handle_message_stream(request, response_out),
        "tasks/get" => handle_tasks_get(request, response_out),
        "tasks/cancel" => handle_tasks_cancel(request, response_out),
        "tasks/resubscribe" => handle_tasks_resubscribe(request, response_out),
        _ => send_jsonrpc_error(response_out, -32601, "Method not found"),
    }
}
```

### Blocking Operations

For `message/send`, `tasks/get`, `tasks/cancel`:

```rust
fn handle_message_send(request: JsonRpcRequest, response_out: ResponseOutparam) {
    let params: MessageSendParams = serde_json::from_value(request.params)?;

    // Convert to WIT types and call agent
    let wit_params = convert_to_wit(params);
    let result = agent::on_message(&wit_params);

    // Convert result to JSON-RPC response
    let response = match result {
        Ok(send_response) => JsonRpcResponse::success(request.id, send_response),
        Err(error) => JsonRpcResponse::error(request.id, error),
    };

    send_json_response(response_out, 200, &response);
}
```

### Streaming Operations (Deferred)

For `message/stream` and `tasks/resubscribe`, we need to stream SSE events:

```rust
fn handle_message_stream(request: JsonRpcRequest, response_out: ResponseOutparam) {
    // Set up SSE response
    let response = OutgoingResponse::new(Headers::new());
    response.headers().set("Content-Type", "text/event-stream");
    response.headers().set("Cache-Control", "no-cache");

    let body = response.body();
    response_out.set(Ok(response));

    // Stream events from agent
    // (Requires streaming agent interface - deferred)

    // Write SSE events progressively
    write_sse_event(&body, "event: message\ndata: {...}\n\n");
    write_sse_event(&body, "event: message\ndata: {...}\n\n");
    // ...

    body.finish();
}
```

**Note:** Streaming requires the agent to provide incremental updates. This is deferred to a future iteration since it requires:
1. Streaming agent interface (`on-stream-message` returning a resource/stream)
2. WASM async/streaming support
3. SSE event formatting

## Error Handling

| HTTP Status | When |
|-------------|------|
| 200 | Successful JSON-RPC (including JSON-RPC errors) |
| 200 | SSE stream started |
| 400 | Malformed request body / invalid JSON |
| 404 | Unknown path |
| 405 | Wrong HTTP method |
| 500 | Internal error (with JSON-RPC error in body) |

JSON-RPC error codes:
- `-32000` - Transport/internal error
- `-32001` - Task/agent not found
- `-32002` - Task not cancelable
- `-32600` - Invalid request
- `-32601` - Method not found
- `-32602` - Invalid params
- `-32603` - Internal error

## Implementation Phases

### Phase 1: Blocking Operations (MVP)

1. Export `wasi:http/incoming-handler`
2. Handle `GET /.well-known/agent.json`
3. Handle `POST /` with JSON-RPC routing
4. Implement `message/send`, `tasks/get`, `tasks/cancel`
5. Return proper error responses

### Phase 2: Streaming (Future)

1. Add streaming agent interface
2. Implement `message/stream` with SSE
3. Implement `tasks/resubscribe`
4. Handle connection drops

## Testing Strategy

### Unit Tests

- JSON-RPC parsing/serialization
- Error response formatting
- Request routing logic

### Integration Tests

Use Python A2A SDK as client:

```python
# Test against our WASM component running in wasmtime
from a2a import A2AClient

client = A2AClient("http://localhost:9999")

# Test agent card discovery
card = await client.get_agent_card()
assert card["name"] == "test-agent"

# Test message/send
response = await client.send_message("Hello")
assert response.status.state == "completed"
```

## Out of Scope (Deferred)

| Feature | Status | Reason |
|---------|--------|--------|
| SSE streaming | Phase 2 | Requires streaming agent interface |
| Push notifications | Deferred | Requires webhook infrastructure |
| FilePart / DataPart | Deferred | Existing WIT TODOs |
| Metadata fields | Deferred | Requires json-value variant |
| gRPC transport | Deferred | HTTP only for now |

## References

- [A2A Specification](https://a2a-protocol.org/latest/specification/)
- [A2A Streaming & Async](https://a2a-protocol.org/latest/topics/streaming-and-async/)
- [WASI HTTP](https://github.com/WebAssembly/wasi-http)
