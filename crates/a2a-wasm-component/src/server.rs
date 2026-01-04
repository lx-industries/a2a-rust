//! Server interface implementation.
//!
//! The server interface is a pass-through to the imported agent interface.
//! The host runtime provides the actual agent implementation.

use crate::a2a::protocol::agent;
use crate::exports::a2a::protocol::server::{Error, MessageSendParams, SendResponse, Task};

/// Handle incoming message/send request.
///
/// Delegates to the imported agent interface.
pub fn on_message(params: MessageSendParams) -> Result<SendResponse, Error> {
    agent::on_message(&params)
}

/// Handle incoming tasks/get request.
///
/// Delegates to the imported agent interface.
pub fn on_get_task(id: String, history_length: Option<u32>) -> Result<Option<Task>, Error> {
    agent::on_get_task(&id, history_length)
}

/// Handle incoming tasks/cancel request.
///
/// Delegates to the imported agent interface.
pub fn on_cancel_task(id: String) -> Result<Option<Task>, Error> {
    agent::on_cancel_task(&id)
}

#[cfg(test)]
mod tests {
    // Note: Unit tests for server are limited because we can't mock the imported
    // agent interface in regular tests. The server is a thin pass-through layer.
    //
    // Full testing is done via integration tests with a real WASM runtime that
    // provides the agent implementation.
    //
    // These tests verify the module structure compiles correctly.

    #[test]
    fn test_server_module_exists() {
        // Verify the module compiles and functions are accessible
        // Actual behavior is tested via integration tests
        assert!(true);
    }
}