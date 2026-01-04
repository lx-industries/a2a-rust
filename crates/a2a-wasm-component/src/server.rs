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
