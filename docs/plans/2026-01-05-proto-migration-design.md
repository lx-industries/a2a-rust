# Migrate a2a-types from JSON Schema to Protobuf/Prost

## Overview

Replace typify-based code generation from `schema/a2a.json` with prost-based generation from `proto/a2a.proto`.

The proto file is the single source of truth. No backward compatibility with the JSON schema wire format is required.

## Key Decisions

- **Types only** — No gRPC service generation, just message structs
- **Prost + serde** — Add serde derives for JSON-RPC compatibility
- **camelCase JSON** — Use `#[serde(rename_all = "camelCase")]` for A2A wire format
- **Proto3 semantics** — Bare fields are required, `optional` keyword produces `Option<T>`
- **Pure-Rust build** — Use protox (no system protoc dependency)
- **Vendor Google API protos** — Copy required protos from googleapis
- **Commit generated code** — Output to `src/generated/a2a/v1.rs` for IDE support

## Dependencies

**crates/a2a-types/Cargo.toml:**

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

**Remove:** `typify`, `schemars`

## File Structure

```
crates/a2a-types/
├── Cargo.toml
├── build.rs
├── proto/
│   ├── a2a.proto
│   └── google/
│       └── api/
│           ├── annotations.proto
│           ├── client.proto
│           ├── field_behavior.proto
│           ├── http.proto
│           └── launch_stage.proto
├── src/
│   ├── lib.rs
│   ├── error.rs
│   └── generated/
│       ├── mod.rs
│       └── a2a/
│           ├── mod.rs
│           └── v1.rs
```

**Removed:** `schema/a2a.json`

## Build Script

**build.rs:**

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

## Module Structure

**src/generated/mod.rs:**

```rust
//! Generated protobuf types - do not edit manually.

#[allow(clippy::derive_partial_eq_without_eq)]
#[allow(clippy::large_enum_variant)]
pub mod a2a;
```

**src/generated/a2a/mod.rs:**

```rust
pub mod v1;
```

**src/lib.rs:**

```rust
//! A2A Protocol type definitions.

pub mod error;
mod generated;

pub use generated::a2a::v1::*;

// Keep custom helper types:
// - TaskId (strongly-typed wrapper)
// - Binding (supported protocol bindings)
// - ProtocolBinding (parses proto's string field)
```

## Custom Types to Keep

These Rust-specific helpers remain in `lib.rs`:

- `TaskId` — Strongly-typed wrapper around String for task identifiers
- `Binding` — Enum for supported protocol bindings (JsonRpc, Rest)
- `ProtocolBinding` — Enum for parsing the proto's `protocol_binding` string field

## Custom Types to Remove

- `AgentInterface` — Now generated from proto

## Breaking Changes

This migration changes the wire format. Consuming crates must update.

**Enum naming:**
```rust
// Before
TaskState::Submitted, Role::User

// After
TaskState::TaskStateSubmitted, Role::RoleUser
```

**Part structure:**
```rust
// Before: discriminated with "kind" field
Part::Text(TextPart { kind: "text", text, metadata })

// After: oneof without discriminator
Part { part: Some(part::Part::Text(text)), metadata }
```

**FilePart:**
```rust
// Before
FileContent { uri, bytes, mime_type, name }

// After
FilePart { file: Some(file_part::File::FileWithUri(uri)), media_type, name }
```

**AgentCard:**
```rust
// Before
agent_card.url

// After
agent_card.supported_interfaces[0].url
```

## Migration Steps

### 1. Update a2a-types crate

1. Update `Cargo.toml` with prost dependencies, remove typify/schemars
2. Copy Google API protos to `proto/google/api/`
3. Write new `build.rs` with prost-build + protox
4. Create `src/generated/` module structure
5. Run `cargo build` to generate types
6. Update `src/lib.rs` to re-export generated types
7. Keep `TaskId`, `Binding`, `ProtocolBinding` helpers
8. Remove custom `AgentInterface`
9. Delete `schema/a2a.json`

### 2. Fix consuming crates

- Update imports and type usage
- Adapt to new enum naming
- Adapt to new `Part` oneof structure
- Update `AgentCard` field access
- Fix any other type mismatches

### 3. Update integration tests

Integration tests may need updates due to wire format changes. If tests fail:

1. Reference the A2A Python SDK integration tests as the canonical test cases
2. Vendor relevant test fixtures from the Python SDK if needed
3. Update Rust test assertions to match the new proto-based wire format

Source: https://github.com/google/a2a-python

### 4. Verify

- Run `cargo build --workspace`
- Run `cargo test --workspace`
- Fix all failures

## Vendored Google API Protos

Copy from `/home/jmlx/Downloads/googleapis/google/api/`:

- `annotations.proto`
- `client.proto`
- `field_behavior.proto`
- `http.proto`
- `launch_stage.proto`

The `google/protobuf/*` protos are bundled with protox.
