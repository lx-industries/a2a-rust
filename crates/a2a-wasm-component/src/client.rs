//! Client interface implementation.
//!
//! This module implements the A2A client interface for WASM components,
//! allowing them to send messages to and query tasks from A2A agents.

use crate::convert;
use crate::exports::a2a::protocol::client::{Error, MessageSendParams, SendResponse, Task};

use a2a_client::Client;
use a2a_transport_wasi::WasiHttpClient;

/// Error codes for client operations.
const ERROR_CODE_TRANSPORT: i32 = -32000;
const ERROR_CODE_CONVERSION: i32 = -32001;
/// Error code returned by server when a task is not found.
/// Note: This uses the same value as ERROR_CODE_CONVERSION (-32001) because
/// the A2A specification uses -32001 for both conversion errors and task-not-found.
const ERROR_CODE_TASK_NOT_FOUND: i32 = -32001;

/// Maps an a2a_client error to a WIT Error.
fn map_client_error(e: a2a_client::Error) -> Error {
    match e {
        a2a_client::Error::Agent {
            message,
            source: a2a_client::ProtocolError::JsonRpc { code, .. },
        } => Error {
            code: code.code(),
            message,
        },
        a2a_client::Error::Transport(msg) => Error {
            code: ERROR_CODE_TRANSPORT,
            message: msg,
        },
        other => Error {
            code: ERROR_CODE_TRANSPORT,
            message: other.to_string(),
        },
    }
}

/// Simple blocking executor for async operations.
///
/// Since WASI pollables handle the actual blocking internally,
/// a simple poll loop works for converting async to sync.
fn block_on<F: std::future::Future>(future: F) -> F::Output {
    use std::pin::pin;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    fn noop_clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    fn noop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(noop_clone, noop, noop, noop);
    let raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);
    // SAFETY: The vtable functions are valid (all no-ops) and the waker is used
    // only within this function's scope, so it cannot outlive the vtable.
    let waker = unsafe { Waker::from_raw(raw_waker) };
    let mut cx = Context::from_waker(&waker);

    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(result) => return result,
            Poll::Pending => {}
        }
    }
}

/// Send a message to an A2A agent.
///
/// Creates an HTTP client and A2A client, converts the WIT params to a2a-types,
/// makes the JSON-RPC call, and converts the response back to WIT types.
pub fn send_message(agent_url: String, params: MessageSendParams) -> Result<SendResponse, Error> {
    // Convert WIT params to a2a-types
    let a2a_params = convert::message_send_params_to_a2a(params).map_err(|e| Error {
        code: ERROR_CODE_CONVERSION,
        message: e,
    })?;

    // Create HTTP client and A2A client (discovery is async)
    let http_client = WasiHttpClient::new();
    let client = block_on(Client::connect(http_client, &agent_url)).map_err(map_client_error)?;

    // Make the RPC call
    let result: Result<a2a_types::SendMessageResponse, _> =
        block_on(client.rpc("message/send", &a2a_params));

    match result {
        Ok(response) => {
            // Convert response back to WIT types
            convert::send_response_from_a2a(&response).map_err(|e| Error {
                code: ERROR_CODE_CONVERSION,
                message: e,
            })
        }
        Err(e) => Err(map_client_error(e)),
    }
}

/// Get a task from an A2A agent.
///
/// Returns `Ok(None)` if the task is not found (error code -32001).
pub fn get_task(
    agent_url: String,
    id: String,
    history_length: Option<u32>,
) -> Result<Option<Task>, Error> {
    // Create the query params
    let params = a2a_types::TaskQueryParams {
        id,
        history_length: history_length.map(|v| v as u64),
        metadata: Default::default(),
    };

    // Create HTTP client and A2A client (discovery is async)
    let http_client = WasiHttpClient::new();
    let client = block_on(Client::connect(http_client, &agent_url)).map_err(map_client_error)?;

    // Make the RPC call
    let result: Result<a2a_types::Task, _> = block_on(client.rpc("tasks/get", &params));

    match result {
        Ok(task) => {
            // Convert task to WIT type
            let wit_task = convert::task_from_a2a(&task).map_err(|e| Error {
                code: ERROR_CODE_CONVERSION,
                message: e,
            })?;
            Ok(Some(wit_task))
        }
        Err(ref e)
            if matches!(
                e,
                a2a_client::Error::Agent {
                    source: a2a_client::ProtocolError::JsonRpc { code, .. },
                    ..
                } if code.code() == ERROR_CODE_TASK_NOT_FOUND
            ) =>
        {
            // Task not found
            Ok(None)
        }
        Err(e) => Err(map_client_error(e)),
    }
}

/// Cancel a task on an A2A agent.
///
/// Returns `Ok(None)` if the task is not found (error code -32001).
pub fn cancel_task(agent_url: String, id: String) -> Result<Option<Task>, Error> {
    // Create the cancel params
    let params = a2a_types::TaskIdParams {
        id,
        metadata: Default::default(),
    };

    // Create HTTP client and A2A client (discovery is async)
    let http_client = WasiHttpClient::new();
    let client = block_on(Client::connect(http_client, &agent_url)).map_err(map_client_error)?;

    // Make the RPC call
    let result: Result<a2a_types::Task, _> = block_on(client.rpc("tasks/cancel", &params));

    match result {
        Ok(task) => {
            // Convert task to WIT type
            let wit_task = convert::task_from_a2a(&task).map_err(|e| Error {
                code: ERROR_CODE_CONVERSION,
                message: e,
            })?;
            Ok(Some(wit_task))
        }
        Err(ref e)
            if matches!(
                e,
                a2a_client::Error::Agent {
                    source: a2a_client::ProtocolError::JsonRpc { code, .. },
                    ..
                } if code.code() == ERROR_CODE_TASK_NOT_FOUND
            ) =>
        {
            // Task not found
            Ok(None)
        }
        Err(e) => Err(map_client_error(e)),
    }
}
