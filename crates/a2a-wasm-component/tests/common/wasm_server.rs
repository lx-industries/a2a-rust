//! WASM HTTP server for integration tests.
//!
//! This module provides a test harness that runs the WASM component as an HTTP server,
//! allowing external clients (like Python A2A SDK) to test against it.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::body::HyperOutgoingBody;
use wasmtime_wasi_http::io::TokioIo;
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

// Generate bindings for the a2a:protocol world from the WIT file.
// Use `with` to reuse wasmtime_wasi_http's types instead of generating new ones.
wasmtime::component::bindgen!({
    world: "a2a-component",
    path: "wit",
    async: true,
    with: {
        "wasi:http/types": wasmtime_wasi_http::bindings::http::types,
        "wasi:io/error": wasmtime_wasi_http::bindings::io::error,
        "wasi:io/streams": wasmtime_wasi_http::bindings::io::streams,
        "wasi:io/poll": wasmtime_wasi_http::bindings::io::poll,
    },
});

// Use types from this module's bindgen
use a2a::protocol::agent::Host as AgentHost;
use a2a::protocol::types::{
    Error as A2aError, Message, MessageSendParams, Part, Role, SendResponse, Task, TaskState,
    TaskStatus, TextPart,
};

/// Task storage for the mock agent.
///
/// Maintains task state across HTTP requests for the WASM server tests.
#[derive(Default)]
pub struct TaskStore {
    tasks: HashMap<String, Task>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    pub fn create_task(&mut self, message: &Message) -> Task {
        let id = uuid::Uuid::new_v4().to_string();
        let context_id = message
            .context_id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let task = Task {
            id: id.clone(),
            context_id,
            status: TaskStatus {
                state: TaskState::Completed,
                message: Some(Message {
                    role: Role::Agent,
                    parts: vec![Part::Text(TextPart {
                        text: "Hello World".to_string(),
                    })],
                    message_id: Some(uuid::Uuid::new_v4().to_string()),
                    task_id: Some(id.clone()),
                    context_id: None,
                }),
                timestamp: Some(chrono::Utc::now().to_rfc3339()),
            },
            history: None,
            artifacts: None,
        };

        self.tasks.insert(id, task.clone());
        task
    }

    pub fn get_task(&self, id: &str) -> Option<Task> {
        self.tasks.get(id).cloned()
    }

    pub fn cancel_task(&mut self, id: &str) -> Option<Task> {
        self.tasks.get_mut(id).map(|task| {
            task.status.state = TaskState::Canceled;
            task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
            task.clone()
        })
    }
}

/// Path to the compiled WASM component.
const WASM_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../target/wasm32-wasip2/release/a2a_wasm_component.wasm"
);

/// Server state shared across requests
struct ServerState {
    engine: Engine,
    component: Component,
    linker: Linker<ServerClientState>,
    task_store: Arc<Mutex<TaskStore>>,
    port: u16,
}

/// Per-request state
struct ServerClientState {
    wasi: WasiCtx,
    http: WasiHttpCtx,
    table: ResourceTable,
    task_store: Arc<Mutex<TaskStore>>,
    port: u16,
}

impl WasiView for ServerClientState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiHttpView for ServerClientState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl AgentHost for ServerClientState {
    async fn get_agent_card(&mut self, _tenant: Option<String>) -> Result<String, A2aError> {
        let port = self.port;
        Ok(format!(
            r#"{{"name":"test-wasm-agent","description":"Test WASM agent","url":"http://localhost:{}","version":"1.0.0","capabilities":{{}},"defaultInputModes":["text"],"defaultOutputModes":["text"],"skills":[]}}"#,
            port
        ))
    }

    async fn on_message(
        &mut self,
        _tenant: Option<String>,
        params: MessageSendParams,
    ) -> Result<SendResponse, A2aError> {
        let task = self.task_store.lock().unwrap().create_task(&params.message);
        Ok(SendResponse::Task(task))
    }

    async fn on_get_task(
        &mut self,
        _tenant: Option<String>,
        name: String,
        _history_length: Option<u32>,
    ) -> Result<Option<Task>, A2aError> {
        // name is resource name in format "tasks/{task_id}", extract the task_id
        let task_id = name.strip_prefix("tasks/").unwrap_or(&name);
        Ok(self.task_store.lock().unwrap().get_task(task_id))
    }

    async fn on_cancel_task(
        &mut self,
        _tenant: Option<String>,
        name: String,
    ) -> Result<Option<Task>, A2aError> {
        // name is resource name in format "tasks/{task_id}", extract the task_id
        let task_id = name.strip_prefix("tasks/").unwrap_or(&name);
        Ok(self.task_store.lock().unwrap().cancel_task(task_id))
    }
}

/// WASM HTTP server for testing.
pub struct WasmServer {
    shutdown_tx: Option<oneshot::Sender<()>>,
    handle: tokio::task::JoinHandle<()>,
    pub url: String,
}

impl WasmServer {
    /// Start the WASM server on a dynamic port.
    pub async fn start() -> Self {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (ready_tx, ready_rx) = oneshot::channel::<u16>();

        let handle = tokio::spawn(async move {
            run_server(shutdown_rx, ready_tx).await;
        });

        // Wait for server to be ready and get the assigned port
        let port = ready_rx.await.expect("Server failed to start");

        Self {
            shutdown_tx: Some(shutdown_tx),
            handle,
            url: format!("http://localhost:{}", port),
        }
    }
}

impl Drop for WasmServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        self.handle.abort();
    }
}

async fn run_server(mut shutdown_rx: oneshot::Receiver<()>, ready_tx: oneshot::Sender<u16>) {
    // Setup wasmtime engine and component
    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    let engine = Engine::new(&config).expect("Failed to create engine");

    let component =
        Component::from_file(&engine, WASM_PATH).expect("Failed to load WASM component");

    // Setup linker with all required interfaces
    let mut linker = Linker::<ServerClientState>::new(&engine);
    wasmtime_wasi::add_to_linker_async(&mut linker).expect("Failed to add WASI");
    wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)
        .expect("Failed to add WASI HTTP");
    a2a::protocol::agent::add_to_linker(&mut linker, |state| state)
        .expect("Failed to add agent interface");

    let task_store = Arc::new(Mutex::new(TaskStore::new()));

    // Bind to port 0 (OS assigns free port)
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    let port = listener
        .local_addr()
        .expect("Failed to get local addr")
        .port();

    let server_state = Arc::new(ServerState {
        engine,
        component,
        linker,
        task_store,
        port,
    });

    // Signal that we're ready with the assigned port
    let _ = ready_tx.send(port);

    loop {
        tokio::select! {
            _ = &mut shutdown_rx => {
                break;
            }
            result = listener.accept() => {
                match result {
                    Ok((stream, _addr)) => {
                        let server_state = server_state.clone();
                        tokio::spawn(async move {
                            let service = service_fn(move |req| {
                                let server_state = server_state.clone();
                                async move {
                                    handle_request(server_state, req).await
                                }
                            });

                            if let Err(e) = http1::Builder::new()
                                .keep_alive(true)
                                .serve_connection(TokioIo::new(stream), service)
                                .await
                            {
                                eprintln!("Error serving connection: {e}");
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {e}");
                    }
                }
            }
        }
    }
}

async fn handle_request(
    server_state: Arc<ServerState>,
    req: Request<Incoming>,
) -> Result<Response<HyperOutgoingBody>, hyper::Error> {
    // Fresh state per request
    let wasi = WasiCtxBuilder::new().inherit_env().build();
    let state = ServerClientState {
        wasi,
        http: WasiHttpCtx::new(),
        table: ResourceTable::new(),
        task_store: server_state.task_store.clone(),
        port: server_state.port,
    };

    let mut store = Store::new(&server_state.engine, state);

    // Convert hyper request to WASI HTTP types
    let (sender, receiver) = tokio::sync::oneshot::channel();
    let incoming_req = store
        .data_mut()
        .new_incoming_request(Scheme::Http, req)
        .expect("Failed to create incoming request");
    let out = store
        .data_mut()
        .new_response_outparam(sender)
        .expect("Failed to create response outparam");

    // Instantiate the component using our own bindgen's A2aComponent
    let bindings =
        A2aComponent::instantiate_async(&mut store, &server_state.component, &server_state.linker)
            .await
            .expect("Failed to instantiate component");

    // Spawn the handler in a task
    let handle_task = tokio::spawn(async move {
        bindings
            .wasi_http_incoming_handler()
            .call_handle(store, incoming_req, out)
            .await
    });

    // Response comes back through the oneshot channel
    match receiver.await {
        Ok(Ok(resp)) => Ok(resp),
        Ok(Err(e)) => {
            eprintln!("Response error: {e:?}");
            Ok(Response::builder()
                .status(500)
                .body(HyperOutgoingBody::default())
                .unwrap())
        }
        Err(_) => {
            // Receiver dropped - check task result
            match handle_task.await {
                Ok(Ok(())) => Ok(Response::builder()
                    .status(500)
                    .body(HyperOutgoingBody::default())
                    .unwrap()),
                Ok(Err(e)) => {
                    eprintln!("Handler error: {e}");
                    Ok(Response::builder()
                        .status(500)
                        .body(HyperOutgoingBody::default())
                        .unwrap())
                }
                Err(e) => {
                    eprintln!("Task join error: {e}");
                    Ok(Response::builder()
                        .status(500)
                        .body(HyperOutgoingBody::default())
                        .unwrap())
                }
            }
        }
    }
}
