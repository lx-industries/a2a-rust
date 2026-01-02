# A2A Rust Integration Tests

## Cross-Implementation Testing

Tests are run against other A2A implementations (JS, Python) rather than mocks.

### Prerequisites

1. Clone JS SDK: `git clone https://github.com/a2aproject/a2a-js ../a2a-js`
2. Clone Python SDK: `git clone https://github.com/a2aproject/a2a-python ../a2a-python`

### Running Tests

#### Test Rust Client against JS SUT Agent

```bash
cd ../a2a-js
npm install
npm run tck:sut-agent &  # Starts on port 41241

cd ../a2a-rust
cargo test --test client_integration
```

#### Test Rust Server with Python Client

```bash
cargo run --example echo-agent &  # Start Rust server

cd ../a2a-python
pytest tests/integration/test_client_server_integration.py
```
