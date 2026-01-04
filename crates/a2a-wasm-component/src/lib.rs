// crates/a2a-wasm-component/src/lib.rs
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

    fn on_cancel_task(id: String) -> Result<Option<server_exports::Task>, server_exports::Error> {
        server::on_cancel_task(id)
    }
}
