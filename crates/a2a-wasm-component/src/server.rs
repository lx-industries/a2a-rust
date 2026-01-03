//! Server interface implementation (stub).

use crate::exports::a2a::protocol::server::{Error, MessageSendParams, SendResponse, Task};

pub fn on_message(_params: MessageSendParams) -> Result<SendResponse, Error> {
    Err(Error {
        code: -32601,
        message: "Server not implemented".to_string(),
    })
}

pub fn on_get_task(
    _id: String,
    _history_length: Option<u32>,
) -> Result<Option<Task>, Error> {
    Err(Error {
        code: -32601,
        message: "Server not implemented".to_string(),
    })
}

pub fn on_cancel_task(_id: String) -> Result<Option<Task>, Error> {
    Err(Error {
        code: -32601,
        message: "Server not implemented".to_string(),
    })
}
