//! Integration tests for the A2A WASM component.
//!
//! These tests run against a real Python A2A helloworld server and verify
//! that the WASM component correctly implements the A2A protocol.
//!
//! Run with: `cargo test -p a2a-wasm-component --test integration_test -- --test-threads=1 --nocapture`
//!
//! Note: The helloworld server returns a "message" response directly, not a "task".

mod common;

use common::server::TestServer;
use common::wasm_runner::WasmRunner;
use insta::assert_json_snapshot;

// ============ Core Operations (5 tests) ============

/// Test that sending a text message returns a response.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn send_message_text_returns_response() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    let result = runner.send_message(&server.url, "Hello").await;

    assert!(result.is_ok(), "send_message should succeed: {:?}", result);

    let mut settings = insta::Settings::clone_current();
    settings.set_filters(vec![
        (r#""id": "[^"]+""#, r#""id": "[TASK_ID]""#),
        (r#""context_id": "[^"]+""#, r#""context_id": "[CTX_ID]""#),
        (r#""message_id": "[^"]+""#, r#""message_id": "[MSG_ID]""#),
        (r#""timestamp": "[^"]+""#, r#""timestamp": "[TS]""#),
    ]);
    settings.bind(|| {
        assert_json_snapshot!("send_message_response", result.unwrap());
    });
}

/// Test that sending a message with blocking config works.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn send_message_with_blocking_config() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    // The WasmRunner already sets blocking: Some(true) by default
    let result = runner.send_message(&server.url, "Test blocking").await;

    assert!(
        result.is_ok(),
        "send_message with blocking should succeed: {:?}",
        result
    );
    let response = result.unwrap();

    // The helloworld server returns a message response directly
    // Verify we get either a task or message response
    let response_type = response["type"].as_str();
    assert!(
        response_type == Some("task") || response_type == Some("message"),
        "Response should be a task or message, got: {:?}",
        response_type
    );
}

/// Test retrieving an existing task (if server returns tasks).
/// Note: The helloworld server returns messages, not tasks, so this test
/// may need adjustment based on actual server behavior.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn get_task_existing() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    // First, send a message to possibly create a task
    let send_result = runner.send_message(&server.url, "Create task").await;
    assert!(send_result.is_ok(), "send_message should succeed");
    let send_response = send_result.unwrap();

    // If the response is a task, try to retrieve it
    if send_response["type"] == "task" {
        let task_id = send_response["task"]["id"]
            .as_str()
            .expect("Task should have an id");

        // Now retrieve the task
        let get_result = runner.get_task(&server.url, task_id, None).await;

        assert!(
            get_result.is_ok(),
            "get_task should succeed: {:?}",
            get_result
        );
        let task = get_result.unwrap();
        assert!(task.is_some(), "Task should exist");

        let mut settings = insta::Settings::clone_current();
        settings.set_filters(vec![
            (r#""id": "[^"]+""#, r#""id": "[TASK_ID]""#),
            (r#""context_id": "[^"]+""#, r#""context_id": "[CTX_ID]""#),
            (r#""message_id": "[^"]+""#, r#""message_id": "[MSG_ID]""#),
            (r#""timestamp": "[^"]+""#, r#""timestamp": "[TS]""#),
        ]);
        settings.bind(|| {
            assert_json_snapshot!("get_task_response", task.unwrap());
        });
    } else {
        // Server returned a message, not a task - this is valid A2A behavior
        // The message response doesn't create a persistent task
        assert_eq!(
            send_response["type"], "message",
            "Expected message response"
        );
    }
}

/// Test retrieving a task with history_length parameter.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn get_task_with_history_length() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    // First, send a message
    let send_result = runner.send_message(&server.url, "Create for history").await;
    assert!(send_result.is_ok());
    let send_response = send_result.unwrap();

    // Only test task retrieval if we got a task response
    if send_response["type"] == "task" {
        let task_id = send_response["task"]["id"]
            .as_str()
            .expect("Task should have an id");

        // Get task with history_length = 1
        let get_result = runner.get_task(&server.url, task_id, Some(1)).await;

        assert!(
            get_result.is_ok(),
            "get_task with history_length should succeed: {:?}",
            get_result
        );
        let task = get_result.unwrap();
        assert!(task.is_some(), "Task should exist");

        // Verify the task was returned (history length validation depends on server)
        let task_json = task.unwrap();
        assert!(task_json["id"].is_string(), "Task should have an id");
    }
}

/// Test that canceling a non-cancelable task returns an error.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn cancel_task_not_cancelable() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    // First, send a message
    let send_result = runner.send_message(&server.url, "Create for cancel").await;
    assert!(send_result.is_ok());
    let send_response = send_result.unwrap();

    // Only test cancellation if we got a task response
    if send_response["type"] == "task" {
        let task_id = send_response["task"]["id"]
            .as_str()
            .expect("Task should have an id");

        // Try to cancel it - the helloworld agent doesn't support cancellation
        let cancel_result = runner.cancel_task(&server.url, task_id).await;

        // Should return an error: TaskNotCancelableError (-32002)
        assert!(
            cancel_result.is_err(),
            "cancel_task on non-cancelable should fail"
        );
        let error_msg = cancel_result.unwrap_err();
        assert!(
            error_msg.contains("-32002") || error_msg.contains("not cancelable"),
            "Error should indicate task not cancelable: {}",
            error_msg
        );
    }
}

// ============ Error Handling (4 tests) ============

/// Test that getting a non-existent task returns None or error.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn get_task_not_found() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    // Use a fake UUID that doesn't exist
    let fake_id = "00000000-0000-0000-0000-000000000000";
    let result = runner.get_task(&server.url, fake_id, None).await;

    // TaskNotFoundError (-32001) could be mapped to Ok(None) or Err
    match result {
        Ok(task) => {
            assert!(task.is_none(), "Non-existent task should return None");
        }
        Err(e) => {
            // TaskNotFoundError is also acceptable
            assert!(
                e.contains("-32001") || e.contains("not found") || e.contains("Task"),
                "Error should indicate task not found: {}",
                e
            );
        }
    }
}

/// Test that canceling a non-existent task returns appropriate error/None.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn cancel_task_not_found() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    // Use a fake UUID that doesn't exist
    let fake_id = "00000000-0000-0000-0000-000000000000";
    let result = runner.cancel_task(&server.url, fake_id).await;

    // Depending on server behavior, this could be Ok(None) or an error
    // The helloworld server may return TaskNotFoundError (-32001)
    match result {
        Ok(task) => {
            assert!(task.is_none(), "Non-existent task should return None");
        }
        Err(e) => {
            // TaskNotFoundError is also acceptable
            assert!(
                e.contains("-32001") || e.contains("not found") || e.contains("Task"),
                "Error should indicate task not found: {}",
                e
            );
        }
    }
}

/// Test that an invalid URL returns a transport error.
#[tokio::test]
async fn invalid_url_transport_error() {
    // No server started - using invalid URL
    let mut runner = WasmRunner::new().await;

    let result = runner.send_message("not-a-valid-url", "Test").await;

    assert!(result.is_err(), "Invalid URL should fail");
    let error_msg = result.unwrap_err();
    // Could be various error messages depending on the transport layer
    assert!(
        !error_msg.is_empty(),
        "Error message should not be empty: {}",
        error_msg
    );
}

/// Test that connection refused returns an error.
#[tokio::test]
async fn connection_refused_error() {
    // No server started - connect to a port that's not listening
    let mut runner = WasmRunner::new().await;

    // Use a high port unlikely to be in use
    let result = runner
        .send_message("http://127.0.0.1:59999", "Test")
        .await;

    assert!(result.is_err(), "Connection refused should fail");
    let error_msg = result.unwrap_err();
    assert!(
        !error_msg.is_empty(),
        "Error message should not be empty: {}",
        error_msg
    );
}

// ============ Response Validation (5 tests) ============

/// Test that the response has the expected structure.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn response_has_valid_structure() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    let result = runner.send_message(&server.url, "Test").await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // Response should have a type field
    let response_type = response["type"].as_str();
    assert!(
        response_type == Some("task") || response_type == Some("message"),
        "Response should have type 'task' or 'message'"
    );

    // If it's a task, verify structure
    if response_type == Some("task") {
        let task = &response["task"];
        assert!(task["id"].is_string(), "Task should have string id");
        assert!(
            task["context_id"].is_string(),
            "Task should have string context_id"
        );
    }

    // If it's a message, verify structure
    if response_type == Some("message") {
        let message = &response["message"];
        let role = message["role"].as_str();
        assert!(
            role == Some("agent") || role == Some("user"),
            "Message should have valid role"
        );
        assert!(message["parts"].is_array(), "Message should have parts");
    }
}

/// Test that the response status is correct (for task responses).
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn response_task_status_correct() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    let result = runner.send_message(&server.url, "Test").await;

    assert!(result.is_ok());
    let response = result.unwrap();

    if response["type"] == "task" {
        let status = &response["task"]["status"];
        assert!(status.is_object(), "Status should be an object");

        // In blocking mode, the task should be completed
        let state = status["state"].as_str();
        assert!(
            state == Some("completed") || state == Some("working"),
            "Task state should be 'completed' or 'working', got: {:?}",
            state
        );
    }
}

/// Test that the response message role is agent.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn response_message_role_agent() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    let result = runner.send_message(&server.url, "Test").await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // For message responses, check role
    if response["type"] == "message" {
        let role = response["message"]["role"].as_str();
        assert_eq!(role, Some("agent"), "Message role should be 'agent'");
    }

    // For task responses, check the status message role
    if response["type"] == "task" {
        let status_message = &response["task"]["status"]["message"];
        if !status_message.is_null() {
            let role = status_message["role"].as_str();
            assert_eq!(
                role,
                Some("agent"),
                "Status message role should be 'agent'"
            );
        }
    }
}

/// Test that the response part is of type text.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn response_part_is_text() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    let result = runner.send_message(&server.url, "Test").await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // For message responses
    if response["type"] == "message" {
        let parts = response["message"]["parts"].as_array();
        if let Some(parts_arr) = parts {
            assert!(!parts_arr.is_empty(), "Message should have parts");
            let first_part = &parts_arr[0];
            assert_eq!(
                first_part["type"], "text",
                "First part should be text type"
            );
        }
    }

    // For task responses
    if response["type"] == "task" {
        let status_message = &response["task"]["status"]["message"];
        if !status_message.is_null() {
            let parts = status_message["parts"].as_array();
            if let Some(parts_arr) = parts {
                assert!(!parts_arr.is_empty(), "Message should have parts");
                let first_part = &parts_arr[0];
                assert_eq!(
                    first_part["type"], "text",
                    "First part should be text type"
                );
            }
        }
    }
}

/// Test that the response text content is "Hello World".
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn response_text_content_hello_world() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    let result = runner.send_message(&server.url, "Test").await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // The helloworld server returns "Hello World" for any message
    let text_content = if response["type"] == "message" {
        response["message"]["parts"][0]["text"].as_str()
    } else if response["type"] == "task" {
        response["task"]["status"]["message"]["parts"][0]["text"].as_str()
    } else {
        None
    };

    assert_eq!(
        text_content,
        Some("Hello World"),
        "Text content should be 'Hello World'"
    );
}

// ============ Additional Tests ============

/// Test that multiple sequential messages work correctly.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn multiple_messages_sequential() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    // Send multiple messages
    for i in 1..=3 {
        let result = runner
            .send_message(&server.url, &format!("Message {}", i))
            .await;
        assert!(
            result.is_ok(),
            "Message {} should succeed: {:?}",
            i,
            result
        );
    }
}

/// Test that response message parts array contains at least one element.
#[tokio::test]
#[ignore = "wasmtime-wasi-http response body issue - see integration test README"]
async fn response_parts_not_empty() {
    let server = TestServer::start();
    let mut runner = WasmRunner::new().await;

    let result = runner.send_message(&server.url, "Test").await;

    assert!(result.is_ok());
    let response = result.unwrap();

    // Check that parts are not empty
    if response["type"] == "message" {
        let parts = response["message"]["parts"].as_array();
        assert!(
            parts.is_some() && !parts.unwrap().is_empty(),
            "Message should have non-empty parts"
        );
    }

    if response["type"] == "task" {
        let status_message = &response["task"]["status"]["message"];
        if !status_message.is_null() {
            let parts = status_message["parts"].as_array();
            assert!(
                parts.is_some() && !parts.unwrap().is_empty(),
                "Status message should have non-empty parts"
            );
        }
    }
}
