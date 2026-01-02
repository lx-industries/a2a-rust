#!/bin/bash
# scripts/test-cross-impl.sh

set -e

echo "=== A2A Cross-Implementation Tests ==="

# Check for JS SDK
if [ ! -d "../a2a-js" ]; then
    echo "JS SDK not found. Clone it first:"
    echo "  git clone https://github.com/a2aproject/a2a-js ../a2a-js"
    exit 1
fi

# Start JS SUT Agent
echo "Starting JS SUT Agent..."
cd ../a2a-js
npm run tck:sut-agent &
SUT_PID=$!
sleep 3

# Run Rust client tests
echo "Running Rust client integration tests..."
cd ../a2a-rust
cargo test --test client_integration || true

# Cleanup
kill $SUT_PID 2>/dev/null || true

echo "=== Tests Complete ==="
