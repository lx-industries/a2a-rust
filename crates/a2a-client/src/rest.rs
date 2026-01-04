// crates/a2a-client/src/rest.rs
//! REST binding implementation.

use a2a_transport::HttpRequest;
use a2a_types::TaskId;

/// Build REST endpoint URL.
pub fn endpoint(base_url: &str, path: &str) -> String {
    format!("{}{}", base_url.trim_end_matches('/'), path)
}

/// POST /v1/message:send
pub fn send_message_request(base_url: &str, body: Vec<u8>) -> HttpRequest {
    HttpRequest::post(endpoint(base_url, "/v1/message:send"), body)
        .with_header("Content-Type", "application/json")
        .with_header("Accept", "application/json")
}

/// GET /v1/tasks/{id}
pub fn get_task_request(base_url: &str, task_id: &TaskId) -> HttpRequest {
    HttpRequest::get(endpoint(
        base_url,
        &format!("/v1/tasks/{}", task_id.as_str()),
    ))
    .with_header("Accept", "application/json")
}

/// GET /v1/tasks/{id}?historyLength={n}
pub fn get_task_with_history_request(
    base_url: &str,
    task_id: &TaskId,
    history_length: u32,
) -> HttpRequest {
    HttpRequest::get(endpoint(
        base_url,
        &format!(
            "/v1/tasks/{}?historyLength={}",
            task_id.as_str(),
            history_length
        ),
    ))
    .with_header("Accept", "application/json")
}

/// POST /v1/tasks/{id}:cancel
pub fn cancel_task_request(base_url: &str, task_id: &TaskId) -> HttpRequest {
    HttpRequest::post(
        endpoint(base_url, &format!("/v1/tasks/{}:cancel", task_id.as_str())),
        vec![],
    )
    .with_header("Accept", "application/json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_with_trailing_slash() {
        assert_eq!(
            endpoint("https://example.com/", "/v1/message:send"),
            "https://example.com/v1/message:send"
        );
    }

    #[test]
    fn test_endpoint_without_trailing_slash() {
        assert_eq!(
            endpoint("https://example.com", "/v1/message:send"),
            "https://example.com/v1/message:send"
        );
    }

    #[test]
    fn test_get_task_request() {
        let req = get_task_request("https://example.com", &TaskId::new("task-123"));
        assert!(req.url.contains("/v1/tasks/task-123"));
    }

    #[test]
    fn test_cancel_task_request() {
        let req = cancel_task_request("https://example.com", &TaskId::new("task-456"));
        assert!(req.url.contains("/v1/tasks/task-456:cancel"));
    }
}
