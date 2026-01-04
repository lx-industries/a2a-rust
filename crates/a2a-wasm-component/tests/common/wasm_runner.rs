//! WASM component runner for integration tests.
//!
//! This module provides a test harness that loads the compiled A2A WASM component
//! and calls its exported functions using wasmtime's component model.

use serde_json::{Value, json};
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

/// Path to the compiled WASM component.
const WASM_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../target/wasm32-wasip2/release/a2a_wasm_component.wasm"
);

// Generate bindings for the a2a:protocol world from the WIT file.
// This creates typed Rust bindings for the component's exports.
wasmtime::component::bindgen!({
    world: "a2a-component",
    path: "wit",
    async: true,
});

// Re-export types from the generated bindings for convenience
use a2a::protocol::agent::Host as AgentHost;
use a2a::protocol::types::{
    Error as A2aError, Message, MessageSendParams, Part, Role, SendResponse, Task, TaskState,
    TaskStatus, TextPart,
};

/// State held by the wasmtime Store, providing WASI and HTTP contexts.
struct TestState {
    wasi: WasiCtx,
    http: WasiHttpCtx,
    table: ResourceTable,
}

impl WasiView for TestState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiHttpView for TestState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

/// Mock implementation of the A2A agent interface for testing.
///
/// This provides simple mock responses for the agent interface that the component imports.
/// These are only used for component instantiation - the actual tests use the client
/// interface to talk to external servers.
impl AgentHost for TestState {
    /// Get agent card as JSON string
    async fn get_agent_card(&mut self) -> Result<String, A2aError> {
        Ok(r#"{"name": "test-agent", "capabilities": {}}"#.to_string())
    }

    /// Process incoming message (blocking) - returns a simple echo response
    async fn on_message(&mut self, params: MessageSendParams) -> Result<SendResponse, A2aError> {
        // Extract text from the first part of the message for echoing
        let echo_text = params
            .message
            .parts
            .first()
            .and_then(|part| match part {
                Part::Text(text_part) => Some(format!("Echo: {}", text_part.text)),
                _ => None,
            })
            .unwrap_or_else(|| "Echo: (no text)".to_string());

        // Return a Task with completed status containing the echo
        let task = Task {
            id: "mock-task-id".to_string(),
            context_id: params
                .message
                .context_id
                .unwrap_or_else(|| "mock-context-id".to_string()),
            status: TaskStatus {
                state: TaskState::Completed,
                message: Some(Message {
                    role: Role::Agent,
                    parts: vec![Part::Text(TextPart { text: echo_text })],
                    message_id: Some("mock-response-id".to_string()),
                    task_id: Some("mock-task-id".to_string()),
                    context_id: None,
                }),
                timestamp: Some("2024-01-01T00:00:00Z".to_string()),
            },
            history: None,
            artifacts: None,
        };

        Ok(SendResponse::Task(task))
    }

    /// Retrieve task by ID - returns None (task not found)
    async fn on_get_task(
        &mut self,
        _id: String,
        _history_length: Option<u32>,
    ) -> Result<Option<Task>, A2aError> {
        Ok(None)
    }

    /// Handle cancellation - returns None (task not found)
    async fn on_cancel_task(&mut self, _id: String) -> Result<Option<Task>, A2aError> {
        Ok(None)
    }
}

/// WASM component runner for testing A2A client operations.
///
/// This struct handles:
/// - Loading and instantiating the WASM component
/// - Setting up WASI and WASI-HTTP contexts
/// - Calling the component's exported functions
///
/// Note: We use manual instantiation instead of `A2aComponent::instantiate_async`
/// to avoid type mismatches with the wasi:http/incoming-handler export which we don't need.
pub struct WasmRunner {
    store: Store<TestState>,
    client: exports::a2a::protocol::client::Guest,
}

impl WasmRunner {
    /// Create a new WasmRunner by loading and instantiating the component.
    ///
    /// # Panics
    ///
    /// Panics if the component cannot be loaded or instantiated.
    pub async fn new() -> Self {
        let mut config = Config::new();
        config.async_support(true);
        config.wasm_component_model(true);

        let engine = Engine::new(&config).expect("Failed to create wasmtime engine");
        let component =
            Component::from_file(&engine, WASM_PATH).expect("Failed to load WASM component");

        let mut linker = Linker::<TestState>::new(&engine);

        // Add WASI interfaces to the linker
        wasmtime_wasi::add_to_linker_async(&mut linker).expect("Failed to add WASI to linker");

        // Add only HTTP interfaces (WASI base already added above)
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)
            .expect("Failed to add WASI HTTP to linker");

        // Add the mock agent interface to the linker
        // The component imports a2a:protocol/agent which we provide with our mock implementation
        a2a::protocol::agent::add_to_linker(&mut linker, |state| state)
            .expect("Failed to add agent interface to linker");

        // Build WASI context with environment access
        let wasi = WasiCtxBuilder::new().inherit_env().build();

        let state = TestState {
            wasi,
            http: WasiHttpCtx::new(),
            table: ResourceTable::new(),
        };

        let mut store = Store::new(&engine, state);

        // Instantiate the component using Linker directly
        // This bypasses the typed bindings that would fail due to wasi:http version mismatch
        let instance = linker
            .instantiate_async(&mut store, &component)
            .await
            .expect("Failed to instantiate component");

        // Get only the client interface - we don't need incoming-handler for these tests
        let client_indices =
            exports::a2a::protocol::client::GuestIndices::new_instance(&mut store, &instance)
                .expect("Failed to find client export");
        let client = client_indices
            .load(&mut store, &instance)
            .expect("Failed to load client interface");

        Self { store, client }
    }

    /// Send a message to an A2A agent via the WASM component.
    ///
    /// # Arguments
    ///
    /// * `url` - The agent's URL
    /// * `message_text` - The message text to send
    ///
    /// # Returns
    ///
    /// The response as a JSON value, or an error string.
    pub async fn send_message(&mut self, url: &str, message_text: &str) -> Result<Value, String> {
        use a2a::protocol::types::{
            Message, MessageSendConfig, MessageSendParams, Part, Role, TextPart,
        };

        // Generate a unique message ID (simple counter-based for tests)
        static MESSAGE_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let message_id = format!(
            "test-msg-{}",
            MESSAGE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        );

        // Build the message parameters
        let params = MessageSendParams {
            message: Message {
                role: Role::User,
                parts: vec![Part::Text(TextPart {
                    text: message_text.to_string(),
                })],
                message_id: Some(message_id),
                task_id: None,
                context_id: None,
            },
            configuration: Some(MessageSendConfig {
                accepted_output_modes: None,
                history_length: None,
                blocking: Some(true),
            }),
        };

        // Call the component's send_message export
        let result = self
            .client
            .call_send_message(&mut self.store, url, &params)
            .await
            .map_err(|e| format!("WASM trap: {e}"))?;

        match result {
            Ok(response) => Ok(send_response_to_json(&response)),
            Err(err) => Err(format!("A2A error {}: {}", err.code, err.message)),
        }
    }

    /// Get a task by ID from an A2A agent via the WASM component.
    ///
    /// # Arguments
    ///
    /// * `url` - The agent's URL
    /// * `id` - The task ID
    /// * `history_length` - Optional maximum history length
    ///
    /// # Returns
    ///
    /// The task as a JSON value (or null if not found), or an error string.
    pub async fn get_task(
        &mut self,
        url: &str,
        id: &str,
        history_length: Option<u32>,
    ) -> Result<Option<Value>, String> {
        let result = self
            .client
            .call_get_task(&mut self.store, url, id, history_length)
            .await
            .map_err(|e| format!("WASM trap: {e}"))?;

        match result {
            Ok(Some(task)) => Ok(Some(task_to_json(&task))),
            Ok(None) => Ok(None),
            Err(err) => Err(format!("A2A error {}: {}", err.code, err.message)),
        }
    }

    /// Cancel a task by ID via the WASM component.
    ///
    /// # Arguments
    ///
    /// * `url` - The agent's URL
    /// * `id` - The task ID to cancel
    ///
    /// # Returns
    ///
    /// The canceled task as a JSON value (or null if not found), or an error string.
    pub async fn cancel_task(&mut self, url: &str, id: &str) -> Result<Option<Value>, String> {
        let result = self
            .client
            .call_cancel_task(&mut self.store, url, id)
            .await
            .map_err(|e| format!("WASM trap: {e}"))?;

        match result {
            Ok(Some(task)) => Ok(Some(task_to_json(&task))),
            Ok(None) => Ok(None),
            Err(err) => Err(format!("A2A error {}: {}", err.code, err.message)),
        }
    }
}

// Helper functions to convert WIT types to JSON for snapshot testing

fn send_response_to_json(response: &a2a::protocol::types::SendResponse) -> Value {
    use a2a::protocol::types::SendResponse;
    match response {
        SendResponse::Task(task) => json!({
            "type": "task",
            "task": task_to_json(task)
        }),
        SendResponse::Message(msg) => json!({
            "type": "message",
            "message": message_to_json(msg)
        }),
    }
}

fn task_to_json(task: &a2a::protocol::types::Task) -> Value {
    json!({
        "id": task.id,
        "context_id": task.context_id,
        "status": task_status_to_json(&task.status),
        "history": task.history.as_ref().map(|h| h.iter().map(message_to_json).collect::<Vec<_>>()),
        "artifacts": task.artifacts.as_ref().map(|a| a.iter().map(artifact_to_json).collect::<Vec<_>>()),
    })
}

fn task_status_to_json(status: &a2a::protocol::types::TaskStatus) -> Value {
    use a2a::protocol::types::TaskState;
    json!({
        "state": match status.state {
            TaskState::Submitted => "submitted",
            TaskState::Working => "working",
            TaskState::InputRequired => "input-required",
            TaskState::Completed => "completed",
            TaskState::Canceled => "canceled",
            TaskState::Failed => "failed",
            TaskState::Rejected => "rejected",
            TaskState::AuthRequired => "auth-required",
            TaskState::Unknown => "unknown",
        },
        "message": status.message.as_ref().map(message_to_json),
        "timestamp": status.timestamp,
    })
}

fn message_to_json(msg: &a2a::protocol::types::Message) -> Value {
    use a2a::protocol::types::Role;
    json!({
        "role": match msg.role {
            Role::User => "user",
            Role::Agent => "agent",
        },
        "parts": msg.parts.iter().map(part_to_json).collect::<Vec<_>>(),
        "message_id": msg.message_id,
        "task_id": msg.task_id,
        "context_id": msg.context_id,
    })
}

fn part_to_json(part: &a2a::protocol::types::Part) -> Value {
    use a2a::protocol::types::Part;
    match part {
        Part::Text(text_part) => json!({
            "type": "text",
            "text": text_part.text,
        }),
        Part::File(file_part) => json!({
            "type": "file",
            "file": {
                "name": file_part.file.name,
                "mime_type": file_part.file.mime_type,
                "uri": file_part.file.uri,
                "has_bytes": file_part.file.bytes.is_some(),
            },
        }),
        Part::Data(data_part) => json!({
            "type": "data",
            "data": data_part.data,
            "mime_type": data_part.mime_type,
        }),
    }
}

fn artifact_to_json(artifact: &a2a::protocol::types::Artifact) -> Value {
    json!({
        "artifact_id": artifact.artifact_id,
        "name": artifact.name,
        "description": artifact.description,
        "parts": artifact.parts.iter().map(part_to_json).collect::<Vec<_>>(),
    })
}
