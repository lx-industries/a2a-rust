# Proto Migration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Migrate a2a-types from typify/JSON schema to prost/protobuf code generation.

**Architecture:** Replace build.rs typify codegen with prost-build + protox. Generated types get serde derives for JSON compatibility. Consuming crates update to new type structure.

**Tech Stack:** prost 0.14, prost-build 0.14, protox 0.9, serde

---

## Task 1: Vendor Google API Protos

**Files:**
- Create: `crates/a2a-types/proto/google/api/annotations.proto`
- Create: `crates/a2a-types/proto/google/api/client.proto`
- Create: `crates/a2a-types/proto/google/api/field_behavior.proto`
- Create: `crates/a2a-types/proto/google/api/http.proto`
- Create: `crates/a2a-types/proto/google/api/launch_stage.proto`

**Step 1: Create directory and copy protos**

```bash
mkdir -p crates/a2a-types/proto/google/api
cp /home/jmlx/Downloads/googleapis/google/api/annotations.proto crates/a2a-types/proto/google/api/
cp /home/jmlx/Downloads/googleapis/google/api/client.proto crates/a2a-types/proto/google/api/
cp /home/jmlx/Downloads/googleapis/google/api/field_behavior.proto crates/a2a-types/proto/google/api/
cp /home/jmlx/Downloads/googleapis/google/api/http.proto crates/a2a-types/proto/google/api/
cp /home/jmlx/Downloads/googleapis/google/api/launch_stage.proto crates/a2a-types/proto/google/api/
```

**Step 2: Verify files exist**

```bash
ls -la crates/a2a-types/proto/google/api/
```

Expected: 5 proto files listed.

**Step 3: Commit**

```bash
git add crates/a2a-types/proto/google/
git commit -m "chore(a2a-types): vendor Google API protos for prost build"
```

---

## Task 2: Update a2a-types Cargo.toml

**Files:**
- Modify: `crates/a2a-types/Cargo.toml`

**Step 1: Update dependencies**

Replace contents with:

```toml
[package]
name = "a2a-types"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
prost = "0.14"
prost-types = "0.14"
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true

[build-dependencies]
prost-build = "0.14"
protox = "0.9"
```

**Step 2: Verify syntax**

```bash
cargo metadata --manifest-path crates/a2a-types/Cargo.toml --no-deps --format-version 1 | head -5
```

Expected: Valid JSON output (no parse errors).

**Step 3: Commit**

```bash
git add crates/a2a-types/Cargo.toml
git commit -m "chore(a2a-types): switch dependencies from typify to prost"
```

---

## Task 3: Create Generated Module Structure

**Files:**
- Create: `crates/a2a-types/src/generated/mod.rs`
- Create: `crates/a2a-types/src/generated/a2a/mod.rs`

**Step 1: Create directories**

```bash
mkdir -p crates/a2a-types/src/generated/a2a
```

**Step 2: Write generated/mod.rs**

```rust
//! Generated protobuf types - do not edit manually.

#[allow(clippy::derive_partial_eq_without_eq)]
#[allow(clippy::large_enum_variant)]
#[allow(clippy::doc_lazy_continuation)]
pub mod a2a;
```

**Step 3: Write generated/a2a/mod.rs**

```rust
pub mod v1;
```

**Step 4: Create placeholder v1.rs**

```rust
// Placeholder - will be replaced by prost-build output
```

**Step 5: Commit**

```bash
git add crates/a2a-types/src/generated/
git commit -m "chore(a2a-types): create generated module structure"
```

---

## Task 4: Write New build.rs

**Files:**
- Modify: `crates/a2a-types/build.rs`

**Step 1: Replace build.rs contents**

```rust
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/");

    let proto_files = &["proto/a2a.proto"];
    let include_dirs = &["proto/"];

    // Parse protos with protox (pure-Rust compiler)
    let file_descriptors = protox::compile(proto_files, include_dirs)?;

    // Configure prost-build
    let mut config = prost_build::Config::new();

    // Add serde derives for JSON compatibility
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");

    // Output to src/generated/a2a/
    let out_dir = PathBuf::from("src/generated/a2a");
    fs::create_dir_all(&out_dir)?;
    config.out_dir(&out_dir);

    // Generate from file descriptors
    config.compile_fds(file_descriptors)?;

    // Rename a2a.v1.rs to v1.rs
    let generated = out_dir.join("a2a.v1.rs");
    let target = out_dir.join("v1.rs");
    if generated.exists() {
        fs::rename(&generated, &target)?;
    }

    Ok(())
}
```

**Step 2: Run build to generate types**

```bash
cd crates/a2a-types && cargo build 2>&1
```

Expected: Build succeeds, `src/generated/a2a/v1.rs` is created.

**Step 3: Verify generated file exists**

```bash
ls -la crates/a2a-types/src/generated/a2a/v1.rs
head -50 crates/a2a-types/src/generated/a2a/v1.rs
```

Expected: File exists with prost-generated structs.

**Step 4: Commit**

```bash
git add crates/a2a-types/build.rs crates/a2a-types/src/generated/
git commit -m "feat(a2a-types): implement prost-build code generation"
```

---

## Task 5: Update lib.rs to Use Generated Types

**Files:**
- Modify: `crates/a2a-types/src/lib.rs`

**Step 1: Update lib.rs**

```rust
//! A2A Protocol type definitions.

pub mod error;
mod generated;

// Re-export all generated types at the crate root
pub use generated::a2a::v1::*;

use serde::{Deserialize, Serialize};

/// Strongly-typed task identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskId(pub String);

impl TaskId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for TaskId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for TaskId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Protocol binding type for client/server communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binding {
    JsonRpc,
    Rest,
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Binding::JsonRpc => write!(f, "JSONRPC"),
            Binding::Rest => write!(f, "HTTP+JSON"),
        }
    }
}

/// Protocol binding as declared in Agent Card (parses proto's string field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolBinding {
    #[serde(rename = "JSONRPC")]
    JsonRpc,
    #[serde(rename = "HTTP+JSON")]
    Rest,
    #[serde(rename = "GRPC")]
    Grpc,
}

impl From<ProtocolBinding> for Option<Binding> {
    fn from(pb: ProtocolBinding) -> Self {
        match pb {
            ProtocolBinding::JsonRpc => Some(Binding::JsonRpc),
            ProtocolBinding::Rest => Some(Binding::Rest),
            ProtocolBinding::Grpc => None, // Not supported
        }
    }
}

/// Parse a protocol binding string from the proto.
impl ProtocolBinding {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "JSONRPC" => Some(Self::JsonRpc),
            "HTTP+JSON" => Some(Self::Rest),
            "GRPC" => Some(Self::Grpc),
            _ => None,
        }
    }
}
```

**Step 2: Verify build**

```bash
cargo build -p a2a-types 2>&1
```

Expected: Build succeeds.

**Step 3: Commit**

```bash
git add crates/a2a-types/src/lib.rs
git commit -m "feat(a2a-types): update lib.rs to export prost types"
```

---

## Task 6: Delete Old Schema and Files

**Files:**
- Delete: `crates/a2a-types/schema/a2a.json`
- Delete: `crates/a2a-types/schema/` (directory)

**Step 1: Remove schema directory**

```bash
rm -rf crates/a2a-types/schema/
```

**Step 2: Verify removal**

```bash
ls crates/a2a-types/
```

Expected: No `schema/` directory.

**Step 3: Commit**

```bash
git add -A crates/a2a-types/schema/
git commit -m "chore(a2a-types): remove old JSON schema"
```

---

## Task 7: Fix a2a-client Compilation

**Files:**
- Modify: `crates/a2a-client/src/binding.rs`
- Modify: `crates/a2a-client/src/lib.rs`
- Modify: `crates/a2a-client/src/error.rs`
- Modify: `crates/a2a-client/src/rest.rs`
- Modify: `crates/a2a-client/src/builder.rs`

**Step 1: Attempt build to see errors**

```bash
cargo build -p a2a-client 2>&1 | head -100
```

Expected: Compilation errors showing what needs fixing.

**Step 2: Fix binding.rs**

The `AgentCard` now uses `supported_interfaces` (Vec<AgentInterface>) instead of `url` + `additional_interfaces`. The `AgentInterface.protocol_binding` is now a String, not an enum.

Update `extract_interfaces` function:

```rust
// crates/a2a-client/src/binding.rs
//! Protocol binding selection and configuration.

use a2a_types::{AgentCard, AgentInterface, Binding, ProtocolBinding};

/// Selected binding for client communication.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectedBinding {
    JsonRpc { url: String },
    Rest { url: String },
}

impl SelectedBinding {
    pub fn binding(&self) -> Binding {
        match self {
            Self::JsonRpc { .. } => Binding::JsonRpc,
            Self::Rest { .. } => Binding::Rest,
        }
    }

    pub fn url(&self) -> &str {
        match self {
            Self::JsonRpc { url } | Self::Rest { url } => url,
        }
    }
}

/// Default binding preference order.
pub const DEFAULT_PREFERENCE: &[Binding] = &[Binding::JsonRpc, Binding::Rest];

fn protocol_string_to_binding(protocol: &str) -> Option<Binding> {
    match protocol {
        "JSONRPC" => Some(Binding::JsonRpc),
        "HTTP+JSON" => Some(Binding::Rest),
        _ => None, // GRPC and others not supported
    }
}

/// Extract all available interfaces from an agent card.
pub fn extract_interfaces(card: &AgentCard) -> Vec<(String, Binding)> {
    let mut interfaces = Vec::new();

    for iface in &card.supported_interfaces {
        if let Some(binding) = protocol_string_to_binding(&iface.protocol_binding) {
            interfaces.push((iface.url.clone(), binding));
        }
    }

    interfaces
}

/// Select a binding from available interfaces based on preference order.
pub fn select_binding(
    interfaces: &[(String, Binding)],
    preference: &[Binding],
) -> Option<SelectedBinding> {
    for preferred in preference {
        for (url, binding) in interfaces {
            if binding == preferred {
                return match binding {
                    Binding::JsonRpc => Some(SelectedBinding::JsonRpc { url: url.clone() }),
                    Binding::Rest => Some(SelectedBinding::Rest { url: url.clone() }),
                };
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_binding_prefers_jsonrpc() {
        let interfaces = vec![
            ("https://example.com/v1".to_string(), Binding::Rest),
            ("https://example.com/".to_string(), Binding::JsonRpc),
        ];
        let result = select_binding(&interfaces, &[Binding::JsonRpc, Binding::Rest]);
        assert_eq!(
            result,
            Some(SelectedBinding::JsonRpc {
                url: "https://example.com/".to_string()
            })
        );
    }

    #[test]
    fn test_select_binding_respects_preference() {
        let interfaces = vec![
            ("https://example.com/v1".to_string(), Binding::Rest),
            ("https://example.com/".to_string(), Binding::JsonRpc),
        ];
        let result = select_binding(&interfaces, &[Binding::Rest, Binding::JsonRpc]);
        assert_eq!(
            result,
            Some(SelectedBinding::Rest {
                url: "https://example.com/v1".to_string()
            })
        );
    }

    #[test]
    fn test_select_binding_no_match() {
        let interfaces: Vec<(String, Binding)> = vec![];
        let result = select_binding(&interfaces, &[Binding::JsonRpc]);
        assert_eq!(result, None);
    }
}
```

**Step 3: Update other files as needed based on compiler errors**

The changes needed will depend on exact proto-generated types. Common fixes:
- `MessageSendParams` → `SendMessageRequest` (check proto)
- `SendMessageResponse` payload access via oneof
- Field name changes (snake_case in Rust)

**Step 4: Build until clean**

```bash
cargo build -p a2a-client 2>&1
```

Expected: Build succeeds.

**Step 5: Commit**

```bash
git add crates/a2a-client/
git commit -m "fix(a2a-client): update for prost-generated types"
```

---

## Task 8: Fix a2a-wasm-component Compilation

**Files:**
- Modify: `crates/a2a-wasm-component/src/convert.rs`

**Step 1: Attempt build to see errors**

```bash
cargo build -p a2a-wasm-component 2>&1 | head -100
```

Expected: Compilation errors in convert.rs.

**Step 2: Update convert.rs for new type structure**

Key changes:
- `a2a_types::Role::User` → `a2a_types::Role::RoleUser`
- `a2a_types::TaskState::Submitted` → `a2a_types::TaskState::TaskStateSubmitted`
- `a2a_types::Part::TextPart(...)` → `a2a_types::Part { part: Some(a2a_types::part::Part::Text(...)), metadata }`
- Remove `kind` field from parts (no longer exists)
- `TextPart` is now just a string in the oneof

The convert.rs file has extensive conversions. Update all match arms.

**Step 3: Build until clean**

```bash
cargo build -p a2a-wasm-component 2>&1
```

Expected: Build succeeds.

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/
git commit -m "fix(a2a-wasm-component): update for prost-generated types"
```

---

## Task 9: Fix a2a-server Compilation (if needed)

**Files:**
- Modify: `crates/a2a-server/src/*.rs` (as needed)

**Step 1: Attempt build**

```bash
cargo build -p a2a-server 2>&1 | head -100
```

**Step 2: Fix any errors**

Follow same patterns as previous tasks.

**Step 3: Commit**

```bash
git add crates/a2a-server/
git commit -m "fix(a2a-server): update for prost-generated types"
```

---

## Task 10: Run All Tests

**Files:** None (verification only)

**Step 1: Build entire workspace**

```bash
cargo build --workspace 2>&1
```

Expected: All crates build successfully.

**Step 2: Run unit tests**

```bash
cargo test --workspace 2>&1
```

Expected: Tests pass or fail with clear reasons.

**Step 3: Fix failing tests**

Update test assertions for new type structure:
- Enum variant names changed
- Part structure changed
- JSON field names may differ

**Step 4: Commit test fixes**

```bash
git add -A
git commit -m "test: update tests for prost-generated types"
```

---

## Task 11: Update Integration Tests

**Files:**
- Modify: `crates/a2a-wasm-component/tests/integration_test.rs`
- Modify: `crates/a2a-wasm-component/tests/common/*.rs`

**Step 1: Run integration tests**

```bash
cargo test -p a2a-wasm-component --test integration_test 2>&1
```

Expected: Tests may fail due to wire format changes.

**Step 2: Check A2A Python SDK for reference**

If integration tests fail due to wire format mismatch:

```bash
# Clone if needed
git clone https://github.com/google/a2a-python /tmp/a2a-python
# Look at test fixtures
ls /tmp/a2a-python/tests/
```

**Step 3: Update test fixtures/assertions**

Match the wire format expected by the proto-based implementation.

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/tests/
git commit -m "test: update integration tests for proto wire format"
```

---

## Task 12: Final Verification

**Step 1: Clean build**

```bash
cargo clean
cargo build --workspace 2>&1
```

Expected: Clean build succeeds.

**Step 2: Run all tests**

```bash
cargo test --workspace 2>&1
```

Expected: All tests pass.

**Step 3: Run clippy**

```bash
cargo clippy --workspace 2>&1
```

Expected: No errors (warnings acceptable).

**Step 4: Final commit if needed**

```bash
git status
# If any uncommitted changes:
git add -A
git commit -m "chore: final cleanup for proto migration"
```

---

## Reference: Type Mapping

| Old (typify) | New (prost) |
|--------------|-------------|
| `Role::User` | `Role::RoleUser` |
| `Role::Agent` | `Role::RoleAgent` |
| `TaskState::Submitted` | `TaskState::TaskStateSubmitted` |
| `TaskState::Working` | `TaskState::TaskStateWorking` |
| `TaskState::InputRequired` | `TaskState::TaskStateInputRequired` |
| `TaskState::Completed` | `TaskState::TaskStateCompleted` |
| `TaskState::Failed` | `TaskState::TaskStateFailed` |
| `TaskState::Canceled` | `TaskState::TaskStateCancelled` |
| `TaskState::Rejected` | `TaskState::TaskStateRejected` |
| `TaskState::AuthRequired` | `TaskState::TaskStateAuthRequired` |
| `TaskState::Unknown` | `TaskState::TaskStateUnspecified` |
| `Part::TextPart(TextPart { kind, text, metadata })` | `Part { part: Some(part::Part::Text(text)), metadata }` |
| `Part::FilePart(...)` | `Part { part: Some(part::Part::File(FilePart {...})), metadata }` |
| `Part::DataPart(...)` | `Part { part: Some(part::Part::Data(DataPart {...})), metadata }` |
| `AgentCard.url` | `AgentCard.supported_interfaces[0].url` |
| `AgentCard.additional_interfaces` | Deprecated, use `supported_interfaces` |
| `TransportProtocol::Jsonrpc` | String `"JSONRPC"` in `AgentInterface.protocol_binding` |
| `MessageSendParams` | `SendMessageRequest` |
| `MessageSendConfiguration` | `SendMessageConfiguration` |
