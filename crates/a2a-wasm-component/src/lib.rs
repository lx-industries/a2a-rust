// crates/a2a-wasm-component/src/lib.rs
//! A2A WASM component with HTTP server and client interfaces.
//!
//! This component exports:
//! - `wasi:http/incoming-handler` - HTTP server for A2A requests
//! - `a2a:protocol/client` - Client interface for calling other agents
//!
//! And imports:
//! - `wasi:http/outgoing-handler` - For client HTTP requests
//! - `a2a:protocol/agent` - Host-provided agent logic
//!
//! # Architecture
//!
//! ```text
//! External A2A Client
//!     │ HTTP
//!     ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │ WASM Host Runtime (e.g., Wassette)                      │
//! │   • TCP/TLS termination                                 │
//! │   • Provides agent interface implementation             │
//! └─────────────────────────┬───────────────────────────────┘
//!                           │
//! ┌─────────────────────────▼───────────────────────────────┐
//! │ a2a-wasm-component                                      │
//! │   export wasi:http/incoming-handler                     │
//! │   import agent { get-agent-card, on-message, ... }      │
//! │   export client { send-message, get-task, cancel-task } │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! # HTTP Endpoints
//!
//! - `GET /.well-known/agent.json` - Agent card discovery
//! - `POST /` - JSON-RPC (message/send, tasks/get, tasks/cancel)
//!
//! # Limitations
//!
//! - Only `TextPart` is supported; `FilePart` and `DataPart` return errors
//! - Streaming (message/stream) is not yet implemented
//! - Metadata fields are not supported (deferred)

mod client;
mod convert;
mod jsonrpc;
mod server;

// Generate WIT bindings
wit_bindgen::generate!({
    world: "a2a-component",
    path: "wit",
    generate_all,
});

use exports::a2a::protocol::client as client_exports;

/// Component struct that implements client and HTTP handler interfaces.
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

// HTTP handler implementation is in server.rs
// The wasi:http/incoming-handler export is implemented there.
