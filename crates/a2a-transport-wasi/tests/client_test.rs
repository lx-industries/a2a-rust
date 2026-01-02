// crates/a2a-transport-wasi/tests/client_test.rs
//! Integration tests for WASI HTTP client.
//!
//! These tests require a WASI runtime (like wasmtime) to execute.
//! They are marked as ignored by default for regular cargo test.

#![cfg(target_arch = "wasm32")]

use a2a_transport::HttpClient;
use a2a_transport_wasi::WasiHttpClient;

#[test]
#[ignore = "requires WASI runtime"]
fn test_client_creation() {
    let _client = WasiHttpClient::new();
}
