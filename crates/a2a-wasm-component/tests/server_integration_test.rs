//! Server integration tests for the A2A WASM component.
//!
//! These tests run the WASM component as an HTTP server and test it
//! with the Python A2A SDK client via individual scenario scripts.
//!
//! Run with: `cargo test -p a2a-wasm-component --test server_integration_test`
//!
//! Prerequisites:
//! - Build the WASM component: `cargo build -p a2a-wasm-component --target wasm32-wasip2 --release`
//! - Install Python dependencies: `cd tests/fixtures/wasm_server_tests && uv sync`

mod common;

use common::wasm_server::WasmServer;
use std::process::Command;
use test_case::test_case;

const SCENARIOS_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/wasm_server_tests/scenarios"
);

#[test_case("agent_card_discovery" ; "agent_card_discovery")]
#[test_case("send_message_success" ; "send_message_success")]
#[test_case("send_message_creates_task" ; "send_message_creates_task")]
#[test_case("get_task_not_found" ; "get_task_not_found")]
#[test_case("get_task_after_send" ; "get_task_after_send")]
#[test_case("cancel_task_not_found" ; "cancel_task_not_found")]
#[test_case("cancel_task_success" ; "cancel_task_success")]
#[test_case("json_rpc_invalid_method" ; "json_rpc_invalid_method")]
#[test_case("journey_basic_flow" ; "journey_basic_flow")]
#[test_case("journey_error_handling" ; "journey_error_handling")]
fn test_scenario(scenario: &str) {
    // Start the WASM server using tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    let server = rt.block_on(WasmServer::start());

    // Run Python scenario script
    let script_path = format!("{}/{}.py", SCENARIOS_DIR, scenario);
    let output = Command::new("uv")
        .args(["run", "python", &script_path])
        .current_dir(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/wasm_server_tests"
        ))
        .env("WASM_SERVER_URL", &server.url)
        .output()
        .expect("Failed to run scenario - is uv installed?");

    if !output.status.success() {
        panic!(
            "Scenario {} failed:\nstderr: {}",
            scenario,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Parse NDJSON stdout into JSON array
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in stdout");
    let steps: Vec<serde_json::Value> = stdout
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).expect("Invalid JSON in stdout"))
        .collect();

    // Snapshot with redactions for dynamic values
    insta::with_settings!({
        filters => vec![
            (r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", "[UUID]"),
            (r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}", "[TIMESTAMP]"),
        ]
    }, {
        insta::assert_json_snapshot!(scenario, steps);
    });
}
