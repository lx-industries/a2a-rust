# Integration Tests for a2a-wasm-component

This directory contains integration tests for the A2A WASM component. These tests verify that the WASM component correctly implements the A2A protocol by running against a real Python A2A helloworld server.

## Test Architecture

1. **TestServer** (`common/server.rs`) - Starts a Python A2A helloworld server
2. **WasmRunner** (`common/wasm_runner.rs`) - Instantiates and runs the WASM component using wasmtime with wasi-http support
3. **Integration tests** (`integration_test.rs`) - Test various A2A protocol operations

## Running Tests

```bash
# Run all tests
cargo test -p a2a-wasm-component

# Run with verbose output
cargo test -p a2a-wasm-component -- --test-threads=1 --nocapture
```

## Test Categories

### Core Operations (5 tests)
- `send_message_text_returns_response` - Basic message sending
- `send_message_with_blocking_config` - Blocking mode configuration
- `get_task_existing` - Task retrieval
- `get_task_with_history_length` - Task with history parameter
- `cancel_task_not_cancelable` - Cancel non-cancelable task

### Error Handling (4 tests)
- `get_task_not_found` - Non-existent task handling
- `cancel_task_not_found` - Cancel non-existent task
- `invalid_url_transport_error` - Invalid URL error
- `connection_refused_error` - Connection refused error

### Response Validation (5 tests)
- `response_has_valid_structure` - Response structure validation
- `response_task_status_correct` - Task status validation
- `response_message_role_agent` - Message role validation
- `response_part_is_text` - Part type validation
- `response_text_content_hello_world` - Content validation

### Additional Tests (2 tests)
- `multiple_messages_sequential` - Sequential message handling
- `response_parts_not_empty` - Non-empty parts validation
