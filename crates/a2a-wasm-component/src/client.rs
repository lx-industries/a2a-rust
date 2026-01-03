//! Client interface implementation.

use crate::exports::a2a::protocol::client::{Error, MessageSendParams, SendResponse, Task};

pub fn send_message(
    _agent_url: String,
    _params: MessageSendParams,
) -> Result<SendResponse, Error> {
    Err(Error {
        code: -32601,
        message: "Not implemented".to_string(),
    })
}

pub fn get_task(
    _agent_url: String,
    _id: String,
    _history_length: Option<u32>,
) -> Result<Option<Task>, Error> {
    Err(Error {
        code: -32601,
        message: "Not implemented".to_string(),
    })
}

pub fn cancel_task(
    _agent_url: String,
    _id: String,
) -> Result<Option<Task>, Error> {
    Err(Error {
        code: -32601,
        message: "Not implemented".to_string(),
    })
}
