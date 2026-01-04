//! HTTP handler implementation for A2A server.
//!
//! Routes requests to appropriate protocol handlers:
//! - JSON-RPC: `POST /`
//! - REST: `/v1/*` paths
//! - Agent card: `GET /.well-known/agent-card.json`

use crate::exports::wasi::http::incoming_handler::Guest;
use crate::jsonrpc;
use crate::wasi::http::types::{
    Headers, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

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

    // Agent card discovery (new spec-compliant path)
    if path == "/.well-known/agent-card.json"
        && matches!(method, crate::wasi::http::types::Method::Get)
    {
        return handle_agent_card();
    }

    // REST binding: /v1/* paths (placeholder for Task 9)
    if path.starts_with("/v1/") {
        return Err((501, "REST binding not yet implemented".to_string()));
    }

    // JSON-RPC binding: POST /
    if (path == "/" || path.is_empty())
        && matches!(method, crate::wasi::http::types::Method::Post)
    {
        return jsonrpc::handle(&request);
    }

    // CORS preflight
    if matches!(method, crate::wasi::http::types::Method::Options) {
        return Ok((204, "text/plain", vec![]));
    }

    Err((404, "Not Found".to_string()))
}

fn handle_agent_card() -> Result<(u16, &'static str, Vec<u8>), (u16, String)> {
    use crate::a2a::protocol::agent;

    match agent::get_agent_card() {
        Ok(card_json) => Ok((200, "application/json", card_json.into_bytes())),
        Err(e) => {
            let response = jsonrpc::Response::internal_error(serde_json::Value::Null, e.message);
            let body = serde_json::to_vec(&response).unwrap_or_default();
            Ok((500, "application/json", body))
        }
    }
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

    let stream = outgoing_body.write().unwrap();
    stream.blocking_write_and_flush(&body).ok();
    drop(stream);

    OutgoingBody::finish(outgoing_body, None).ok();
}
