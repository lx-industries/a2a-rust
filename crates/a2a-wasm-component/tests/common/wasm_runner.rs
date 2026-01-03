//! WASM component runner for integration tests.
//!
//! This module provides a test harness that loads the compiled A2A WASM component
//! and calls its exported functions using wasmtime's component model.

use serde_json::{json, Value};
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

/// WASM component runner for testing A2A client operations.
///
/// This struct handles:
/// - Loading and instantiating the WASM component
/// - Setting up WASI and WASI-HTTP contexts
/// - Calling the component's exported functions
pub struct WasmRunner {
    store: Store<TestState>,
    bindings: A2aComponent,
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

        // Build WASI context with environment access
        let wasi = WasiCtxBuilder::new().inherit_env().build();

        let state = TestState {
            wasi,
            http: WasiHttpCtx::new(),
            table: ResourceTable::new(),
        };

        let mut store = Store::new(&engine, state);

        // Instantiate the component and get typed bindings
        let bindings = A2aComponent::instantiate_async(&mut store, &component, &linker)
            .await
            .expect("Failed to instantiate component");

        Self { store, bindings }
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
        use exports::a2a::protocol::client::{
            Message, MessageSendConfig, MessageSendParams, Part, Role, TextPart,
        };

        // Build the message parameters
        let params = MessageSendParams {
            message: Message {
                role: Role::User,
                parts: vec![Part::Text(TextPart {
                    text: message_text.to_string(),
                })],
                message_id: None,
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
        let client = self.bindings.a2a_protocol_client();
        let result = client
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
        let client = self.bindings.a2a_protocol_client();
        let result = client
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
        let client = self.bindings.a2a_protocol_client();
        let result = client
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

fn send_response_to_json(response: &exports::a2a::protocol::client::SendResponse) -> Value {
    use exports::a2a::protocol::client::SendResponse;
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

fn task_to_json(task: &exports::a2a::protocol::client::Task) -> Value {
    json!({
        "id": task.id,
        "context_id": task.context_id,
        "status": task_status_to_json(&task.status),
        "history": task.history.as_ref().map(|h| h.iter().map(message_to_json).collect::<Vec<_>>()),
        "artifacts": task.artifacts.as_ref().map(|a| a.iter().map(artifact_to_json).collect::<Vec<_>>()),
    })
}

fn task_status_to_json(status: &exports::a2a::protocol::client::TaskStatus) -> Value {
    use exports::a2a::protocol::client::TaskState;
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

fn message_to_json(msg: &exports::a2a::protocol::client::Message) -> Value {
    use exports::a2a::protocol::client::Role;
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

fn part_to_json(part: &exports::a2a::protocol::client::Part) -> Value {
    use exports::a2a::protocol::client::Part;
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

fn artifact_to_json(artifact: &exports::a2a::protocol::client::Artifact) -> Value {
    json!({
        "artifact_id": artifact.artifact_id,
        "name": artifact.name,
        "description": artifact.description,
        "parts": artifact.parts.iter().map(part_to_json).collect::<Vec<_>>(),
    })
}
