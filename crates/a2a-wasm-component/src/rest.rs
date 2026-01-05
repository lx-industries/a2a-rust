//! REST binding implementation for A2A server.
//!
//! Handles `/v1/*` paths per A2A HTTP+JSON/REST binding spec.

use crate::convert;
use crate::wasi::http::types::{IncomingRequest, Method};
use serde::Serialize;

/// Handle a REST request.
pub fn handle(
    method: Method,
    path: &str,
    request: &IncomingRequest,
) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    match (method, path) {
        // POST /v1/message:send
        (Method::Post, "/v1/message:send") => handle_send_message(request),

        // GET /v1/tasks/{id} or GET /v1/tasks/{id}?historyLength=N
        (Method::Get, p) if is_task_get(p) => {
            let (task_id, history_length) = parse_task_get(p)?;
            handle_get_task(&task_id, history_length)
        }

        // POST /v1/tasks/{id}:cancel
        (Method::Post, p) if p.starts_with("/v1/tasks/") && p.ends_with(":cancel") => {
            let task_id = extract_task_id_before_action(p)?;
            handle_cancel_task(&task_id)
        }

        // GET /v1/agentCard (extended, authenticated)
        (Method::Get, "/v1/agentCard") => handle_extended_agent_card(),

        _ => Err((404, "Not Found".to_string())),
    }
}

fn is_task_get(path: &str) -> bool {
    path.starts_with("/v1/tasks/") && !path.ends_with(":cancel")
}

fn parse_task_get(path: &str) -> Result<(String, Option<u32>), (u16, String)> {
    // Path format: /v1/tasks/{id} or /v1/tasks/{id}?historyLength=N
    let path = path.strip_prefix("/v1/tasks/").unwrap_or(path);

    let (task_id, query) = match path.split_once('?') {
        Some((id, q)) => (id, Some(q)),
        None => (path, None),
    };

    if task_id.is_empty() {
        return Err((400, "Missing task ID".to_string()));
    }

    let history_length = query
        .and_then(|q| {
            q.split('&')
                .find_map(|param| param.strip_prefix("historyLength="))
        })
        .and_then(|v| v.parse().ok());

    Ok((task_id.to_string(), history_length))
}

fn extract_task_id_before_action(path: &str) -> Result<String, (u16, String)> {
    // Path format: /v1/tasks/{id}:cancel
    let path = path
        .strip_prefix("/v1/tasks/")
        .ok_or((400, "Invalid path".to_string()))?;

    let task_id = path
        .strip_suffix(":cancel")
        .ok_or((400, "Invalid path".to_string()))?;

    if task_id.is_empty() {
        return Err((400, "Missing task ID".to_string()));
    }

    Ok(task_id.to_string())
}

fn handle_send_message(
    request: &IncomingRequest,
) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::a2a::protocol::agent;

    let body = read_request_body(request)?;

    // In prost-generated types, MessageSendParams is now SendMessageRequest
    let params: a2a_types::SendMessageRequest =
        serde_json::from_slice(&body).map_err(|e| (400, format!("Invalid JSON: {e}")))?;

    let wit_params = convert::message_send_params_to_wit(&params)
        .map_err(|e| (400, format!("Invalid params: {e}")))?;

    // Extract tenant from params (empty string means no tenant)
    let tenant = if params.tenant.is_empty() {
        None
    } else {
        Some(params.tenant.as_str())
    };

    match agent::on_message(tenant, &wit_params) {
        Ok(response) => {
            let a2a_response = convert::send_response_from_wit(&response);
            let body = serde_json::to_vec(&a2a_response).unwrap_or_default();
            Ok((200, "application/json", body))
        }
        Err(e) => {
            let body = serde_json::to_vec(&ErrorResponse { error: e.message }).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
}

fn handle_get_task(
    task_id: &str,
    history_length: Option<u32>,
) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::a2a::protocol::agent;

    // Construct resource name from task ID per A2A spec
    let name = format!("tasks/{task_id}");

    // TODO: Extract tenant from request headers or path for multi-tenancy
    match agent::on_get_task(None, &name, history_length) {
        Ok(Some(task)) => {
            let a2a_task = convert::task_from_wit(&task);
            let body = serde_json::to_vec(&a2a_task).unwrap_or_default();
            Ok((200, "application/json", body))
        }
        Ok(None) => {
            let body = serde_json::to_vec(&ErrorResponse {
                error: "Task not found".to_string(),
            })
            .unwrap_or_default();
            Ok((404, "application/json", body))
        }
        Err(e) => {
            let body = serde_json::to_vec(&ErrorResponse { error: e.message }).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
}

fn handle_cancel_task(task_id: &str) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::a2a::protocol::agent;

    // Construct resource name from task ID per A2A spec
    let name = format!("tasks/{task_id}");

    // TODO: Extract tenant from request headers or path for multi-tenancy
    match agent::on_cancel_task(None, &name) {
        Ok(Some(task)) => {
            let a2a_task = convert::task_from_wit(&task);
            let body = serde_json::to_vec(&a2a_task).unwrap_or_default();
            Ok((200, "application/json", body))
        }
        Ok(None) => {
            let body = serde_json::to_vec(&ErrorResponse {
                error: "Task not found".to_string(),
            })
            .unwrap_or_default();
            Ok((404, "application/json", body))
        }
        Err(e) => {
            let body = serde_json::to_vec(&ErrorResponse { error: e.message }).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
}

fn handle_extended_agent_card() -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::a2a::protocol::agent;

    // For now, return the same card as the public one
    // In the future, this could return additional authenticated details
    // TODO: Extract tenant from request headers or path for multi-tenancy
    match agent::get_agent_card(None) {
        Ok(card_json) => Ok((200, "application/json", card_json.into_bytes())),
        Err(e) => {
            let body = serde_json::to_vec(&ErrorResponse { error: e.message }).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
}

/// Maximum request body size (1 MB).
const MAX_BODY_SIZE: usize = 1024 * 1024;

fn read_request_body(request: &IncomingRequest) -> Result<Vec<u8>, (u16, String)> {
    let body = request
        .consume()
        .map_err(|_| (500, "Failed to consume request body".to_string()))?;

    let stream = body
        .stream()
        .map_err(|_| (500, "Failed to get body stream".to_string()))?;

    let mut data = Vec::new();
    loop {
        match stream.blocking_read(64 * 1024) {
            Ok(chunk) => {
                if chunk.is_empty() {
                    break;
                }
                if data.len() + chunk.len() > MAX_BODY_SIZE {
                    return Err((413, "Request body too large".to_string()));
                }
                data.extend_from_slice(&chunk);
            }
            Err(crate::wasi::io::streams::StreamError::Closed) => break,
            Err(_) => return Err((500, "Failed to read body".to_string())),
        }
    }

    Ok(data)
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
