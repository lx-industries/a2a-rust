//! Server integration tests for the A2A WASM component.
//!
//! These tests run the WASM component as an HTTP server and test it
//! with the Python A2A SDK client.
//!
//! Run with: `cargo test -p a2a-wasm-component --test server_integration_test`
//!
//! Prerequisites:
//! - Build the WASM component: `cargo build -p a2a-wasm-component --target wasm32-wasip2 --release`
//! - Install Python dependencies: `cd tests/fixtures/wasm_server_tests && uv sync`

mod common;

use common::wasm_server::WasmServer;
use std::process::Command;

const FIXTURES_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/wasm_server_tests"
);

/// Test the WASM server with Python A2A SDK client.
#[tokio::test]
async fn test_wasm_server_with_python_client() {
    // Start the WASM server
    let server = WasmServer::start().await;
    println!("WASM server started at {}", server.url);

    // Run Python tests
    let status = Command::new("uv")
        .args(["run", "pytest", "-v", "--tb=short"])
        .current_dir(FIXTURES_DIR)
        .env("WASM_SERVER_URL", &server.url)
        .status()
        .expect("Failed to run pytest - is uv installed?");

    assert!(status.success(), "Python tests failed");
}
