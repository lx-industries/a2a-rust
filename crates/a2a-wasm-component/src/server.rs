//! HTTP handler implementation for A2A server.
//!
//! Exports wasi:http/incoming-handler to handle incoming A2A requests.

use crate::exports::wasi::http::incoming_handler::Guest;
use crate::jsonrpc::{self, Response};
use crate::wasi::http::types::{
    Headers, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

/// Implement the wasi:http/incoming-handler interface.
impl Guest for crate::Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let result = handle_request(request);

        match result {
            Ok((status, content_type, body)) => {
                send_response(response_out, status, content_type, body);
            }
            Err((status, message)) => {
                send_response(response_out, status, "text/plain", message.into_bytes());
            }
        }
    }
}

fn handle_request(request: IncomingRequest) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    let method = request.method();
    let path = request.path_with_query().unwrap_or_default();

    match (method, path.as_str()) {
        (crate::wasi::http::types::Method::Get, "/.well-known/agent.json") => handle_agent_card(),
        (crate::wasi::http::types::Method::Post, "/" | "") => {
            let body = read_request_body(&request)?;
            handle_jsonrpc(&body)
        }
        (crate::wasi::http::types::Method::Options, _) => {
            // CORS preflight
            Ok((204, "text/plain", vec![]))
        }
        _ => Err((404, "Not Found".to_string())),
    }
}

fn handle_agent_card() -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::a2a::protocol::agent;

    match agent::get_agent_card() {
        Ok(card_json) => Ok((200, "application/json", card_json.into_bytes())),
        Err(e) => {
            let response = Response::internal_error(serde_json::Value::Null, e.message);
            let body = serde_json::to_vec(&response).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
}

fn handle_jsonrpc(body: &[u8]) -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    // Parse JSON-RPC request
    let request: jsonrpc::Request = match serde_json::from_slice(body) {
        Ok(r) => r,
        Err(e) => {
            let response = Response::error(
                serde_json::Value::Null,
                jsonrpc::PARSE_ERROR,
                format!("Parse error: {e}"),
            );
            let body = serde_json::to_vec(&response).unwrap_or_default();
            return Ok((400, "application/json", body));
        }
    };

    // Route to handler
    let response = match request.method.as_str() {
        "message/send" => handle_message_send(&request),
        "tasks/get" => handle_tasks_get(&request),
        "tasks/cancel" => handle_tasks_cancel(&request),
        "message/stream" | "tasks/resubscribe" => {
            // Streaming not yet implemented
            Response::error(request.id.clone(), -32601, "Streaming not implemented")
        }
        _ => Response::method_not_found(request.id.clone()),
    };

    let body = serde_json::to_vec(&response).unwrap_or_default();
    Ok((200, "application/json", body))
}

fn handle_message_send(request: &jsonrpc::Request) -> Response {
    use crate::a2a::protocol::agent;
    use crate::convert;

    // Parse params
    let params: a2a_types::MessageSendParams = match serde_json::from_value(request.params.clone())
    {
        Ok(p) => p,
        Err(e) => return Response::invalid_params(request.id.clone(), e.to_string()),
    };

    // Convert to WIT types
    let wit_params = match convert::message_send_params_to_wit(&params) {
        Ok(p) => p,
        Err(e) => return Response::invalid_params(request.id.clone(), e),
    };

    // Call agent
    match agent::on_message(&wit_params) {
        Ok(response) => {
            let a2a_response = convert::send_response_from_wit(&response);
            Response::success(request.id.clone(), a2a_response)
        }
        Err(e) => Response::error(request.id.clone(), e.code, e.message),
    }
}

fn handle_tasks_get(request: &jsonrpc::Request) -> Response {
    use crate::a2a::protocol::agent;
    use crate::convert;

    #[derive(serde::Deserialize)]
    struct Params {
        id: String,
        #[serde(rename = "historyLength")]
        history_length: Option<u32>,
    }

    let params: Params = match serde_json::from_value(request.params.clone()) {
        Ok(p) => p,
        Err(e) => return Response::invalid_params(request.id.clone(), e.to_string()),
    };

    match agent::on_get_task(&params.id, params.history_length) {
        Ok(Some(task)) => {
            let a2a_task = convert::task_from_wit(&task);
            Response::success(request.id.clone(), a2a_task)
        }
        Ok(None) => Response::success(request.id.clone(), serde_json::Value::Null),
        Err(e) => Response::error(request.id.clone(), e.code, e.message),
    }
}

fn handle_tasks_cancel(request: &jsonrpc::Request) -> Response {
    use crate::a2a::protocol::agent;
    use crate::convert;

    #[derive(serde::Deserialize)]
    struct Params {
        id: String,
    }

    let params: Params = match serde_json::from_value(request.params.clone()) {
        Ok(p) => p,
        Err(e) => return Response::invalid_params(request.id.clone(), e.to_string()),
    };

    match agent::on_cancel_task(&params.id) {
        Ok(Some(task)) => {
            let a2a_task = convert::task_from_wit(&task);
            Response::success(request.id.clone(), a2a_task)
        }
        Ok(None) => Response::success(request.id.clone(), serde_json::Value::Null),
        Err(e) => Response::error(request.id.clone(), e.code, e.message),
    }
}

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
                data.extend_from_slice(&chunk);
            }
            Err(crate::wasi::io::streams::StreamError::Closed) => break,
            Err(_) => return Err((500, "Failed to read body".to_string())),
        }
    }

    Ok(data)
}

fn send_response(response_out: ResponseOutparam, status: u16, content_type: &str, body: Vec<u8>) {
    let headers = Headers::new();
    headers
        .set("Content-Type", &[content_type.as_bytes().to_vec()])
        .ok();
    headers
        .set("Content-Length", &[body.len().to_string().into_bytes()])
        .ok();

    let response = OutgoingResponse::new(headers);
    response.set_status_code(status).ok();

    let outgoing_body = response.body().unwrap();
    ResponseOutparam::set(response_out, Ok(response));

    // Write body
    let stream = outgoing_body.write().unwrap();
    stream.blocking_write_and_flush(&body).ok();
    drop(stream);

    OutgoingBody::finish(outgoing_body, None).ok();
}
