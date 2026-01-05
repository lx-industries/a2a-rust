// Allow dead_code in test utilities: Rust's dead code analysis produces false positives
// for code used through wasmtime's bindgen!() macro, indirect trait method calls,
// and constants composed via concat!(). These modules are actively used by tests.
#[allow(dead_code)]
pub mod server;
#[allow(dead_code)]
pub mod wasm_runner;
#[allow(dead_code)]
pub mod wasm_server;
