# pbjson Migration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace manual serde derives with pbjson-build for proto3 JSON-compliant serialization, fixing "missing field" errors when deserializing A2A server responses.

**Architecture:** pbjson-build generates proto3 JSON-compliant serde implementations as `impl` blocks in a separate `.serde.rs` file, included via `include!()` macro (the documented pattern). This handles empty repeated fields, enum name serialization, and well-known types correctly.

**Tech Stack:** pbjson-build 0.9, pbjson-types 0.9, pbjson 0.9, protox 0.9, prost 0.14

---

## Task 1: Reset to Before prost-wkt-types

**Files:** All changes from prost-wkt-types commits reverted

The prost-wkt-types approach was incorrect - it only solved Struct/Timestamp serialization but not the proto3 JSON format compliance (empty repeated fields, etc.).

**Step 1: Reset to commit 88eafec**

```bash
git reset --hard 88eafec
```

This is the last commit from the proto migration before prost-wkt-types was attempted.

**Step 2: Verify state**

```bash
git log --oneline -3
```

Expected:
```
88eafec fix(a2a-wasm-component): update for prost-generated types
dc48fac fix(a2a-client): update for prost-generated types
750da35 chore(a2a-types): remove old JSON schema
```

**Step 3: Verify build.rs has serde(skip) attributes**

```bash
grep -c "serde(skip)" crates/a2a-types/build.rs
```

Expected: 11 (the original workaround for prost_types fields)

---

## Task 2: Update a2a-types Dependencies

**Files:**
- Modify: `crates/a2a-types/Cargo.toml`

**Step 1: Read current Cargo.toml**

```bash
cat crates/a2a-types/Cargo.toml
```

**Step 2: Update dependencies**

Replace contents with:

```toml
[package]
name = "a2a-types"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
prost = "0.14"
pbjson = "0.9"
pbjson-types = "0.9"
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true

[build-dependencies]
prost-build = "0.14"
pbjson-build = "0.9"
protox = "0.9"
```

Key changes:
- Removed: `prost-types` (was used for Struct/Timestamp)
- Added: `pbjson`, `pbjson-types`, `pbjson-build`

**Step 3: Verify syntax**

```bash
cargo metadata --manifest-path crates/a2a-types/Cargo.toml --no-deps --format-version 1 | head -3
```

Expected: Valid JSON output.

**Step 4: Commit**

```bash
git add crates/a2a-types/Cargo.toml
git commit -m "chore(a2a-types): switch to pbjson for proto3 JSON compliance"
```

---

## Task 3: Rewrite build.rs for pbjson-build

**Files:**
- Modify: `crates/a2a-types/build.rs`

**Step 1: Replace build.rs contents**

```rust
use prost::Message;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/");

    let proto_files = &["proto/a2a.proto"];
    let include_dirs = &["proto/"];

    // Parse protos with protox (pure-Rust compiler)
    let file_descriptors = protox::compile(proto_files, include_dirs)?;

    // Output directory for generated code
    let out_dir = PathBuf::from("src/generated/a2a");
    fs::create_dir_all(&out_dir)?;

    // Configure prost-build
    // NOTE: No serde derives here - pbjson-build handles serialization
    let mut config = prost_build::Config::new();

    // Use pbjson-types for Well-Known Types (Struct, Timestamp, etc.)
    config.extern_path(".google.protobuf.Struct", "::pbjson_types::Struct");
    config.extern_path(".google.protobuf.Timestamp", "::pbjson_types::Timestamp");
    config.extern_path(".google.protobuf.Value", "::pbjson_types::Value");
    config.extern_path(".google.protobuf.ListValue", "::pbjson_types::ListValue");
    config.extern_path(".google.protobuf.NullValue", "::pbjson_types::NullValue");

    config.out_dir(&out_dir);

    // Generate prost types from file descriptors
    config.compile_fds(file_descriptors.clone())?;

    // Rename a2a.v1.rs to v1.rs
    let generated = out_dir.join("a2a.v1.rs");
    let target = out_dir.join("v1.rs");
    if generated.exists() {
        fs::rename(&generated, &target)?;
    }

    // Serialize file descriptors for pbjson-build
    let descriptor_bytes = file_descriptors.encode_to_vec();

    // Generate pbjson serde implementations (proto3 JSON compliant)
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_bytes)?
        .out_dir(&out_dir)
        .build(&[".a2a.v1"])?;

    // Rename a2a.v1.serde.rs to v1.serde.rs
    let serde_generated = out_dir.join("a2a.v1.serde.rs");
    let serde_target = out_dir.join("v1.serde.rs");
    if serde_generated.exists() {
        fs::rename(&serde_generated, &serde_target)?;
    }

    Ok(())
}
```

Key changes from original:
- Removed all `type_attribute` serde derive calls
- Removed all `field_attribute` serde(skip) calls
- Added `pbjson_build::Builder` to generate serde implementations
- Uses `encode_to_vec()` to serialize descriptors for pbjson

**Step 2: Commit**

```bash
git add crates/a2a-types/build.rs
git commit -m "feat(a2a-types): use pbjson-build for proto3 JSON serde"
```

---

## Task 4: Update Module to Include Serde Implementations

**Files:**
- Modify: `crates/a2a-types/src/generated/a2a/mod.rs`

**Step 1: Read current mod.rs**

```bash
cat crates/a2a-types/src/generated/a2a/mod.rs
```

**Step 2: Update mod.rs**

pbjson-build generates `impl Serialize/Deserialize` blocks (not a module), so `include!()` is the documented pattern:

```rust
mod v1;
pub use v1::*;

// pbjson-generated serde implementations (proto3 JSON compliant)
// This file contains `impl Serialize` and `impl Deserialize` blocks
include!("v1.serde.rs");
```

**Step 3: Commit**

```bash
git add crates/a2a-types/src/generated/a2a/mod.rs
git commit -m "feat(a2a-types): include pbjson serde implementations"
```

---

## Task 5: Regenerate Types and Verify

**Files:**
- Modify: `crates/a2a-types/src/generated/a2a/v1.rs` (regenerated)
- Create: `crates/a2a-types/src/generated/a2a/v1.serde.rs` (new)

**Step 1: Clean and rebuild**

```bash
cargo clean -p a2a-types
cargo build -p a2a-types 2>&1
```

Expected: Build succeeds.

**Step 2: Verify v1.rs uses pbjson_types**

```bash
grep -c "pbjson_types::" crates/a2a-types/src/generated/a2a/v1.rs
```

Expected: > 0 (references to Struct, Timestamp, etc.)

**Step 3: Verify v1.rs has NO serde derives**

```bash
grep -c "#\[derive.*serde" crates/a2a-types/src/generated/a2a/v1.rs
```

Expected: 0 (pbjson handles serde, not derives)

**Step 4: Verify v1.serde.rs exists and has implementations**

```bash
head -30 crates/a2a-types/src/generated/a2a/v1.serde.rs
```

Expected: File contains `impl serde::Serialize for ...` blocks.

**Step 5: Commit regenerated files**

```bash
git add crates/a2a-types/src/generated/
git commit -m "feat(a2a-types): regenerate with pbjson serde"
```

---

## Task 6: Update a2a-wasm-component Dependencies

**Files:**
- Modify: `crates/a2a-wasm-component/Cargo.toml`

**Step 1: Check current dependency**

```bash
grep "prost-types" crates/a2a-wasm-component/Cargo.toml
```

**Step 2: Replace prost-types with pbjson-types**

Change:
```toml
prost-types = "0.14"
```

To:
```toml
pbjson-types = "0.9"
```

**Step 3: Commit**

```bash
git add crates/a2a-wasm-component/Cargo.toml
git commit -m "chore(a2a-wasm-component): switch to pbjson-types"
```

---

## Task 7: Update convert.rs for pbjson-types

**Files:**
- Modify: `crates/a2a-wasm-component/src/convert.rs`

**Step 1: Find prost_types usage**

```bash
grep -n "prost_types" crates/a2a-wasm-component/src/convert.rs
```

**Step 2: Replace prost_types::Timestamp with pbjson_types::Timestamp**

Update the Timestamp creation code (around line 370):

From:
```rust
prost_types::Timestamp {
    seconds: dt_utc.timestamp(),
    nanos: dt_utc.timestamp_subsec_nanos() as i32,
}
```

To:
```rust
pbjson_types::Timestamp {
    seconds: dt_utc.timestamp(),
    nanos: dt_utc.timestamp_subsec_nanos() as i32,
}
```

Also update any comments referencing prost_types.

**Step 3: Verify no prost_types references remain**

```bash
grep -c "prost_types" crates/a2a-wasm-component/src/convert.rs
```

Expected: 0

**Step 4: Commit**

```bash
git add crates/a2a-wasm-component/src/convert.rs
git commit -m "fix(a2a-wasm-component): use pbjson_types::Timestamp"
```

---

## Task 8: Build and Test

**Files:** None (verification only)

**Step 1: Build workspace (excluding WASM for native)**

```bash
cargo build --workspace --exclude a2a-wasm-component 2>&1
```

Expected: Build succeeds.

**Step 2: Build WASM target**

```bash
cargo build -p a2a-wasm-component --target wasm32-wasip2 2>&1
```

Expected: Build succeeds.

**Step 3: Run unit tests**

```bash
cargo test --workspace --lib 2>&1
```

Expected: All unit tests pass.

**Step 4: Build WASM release for integration tests**

```bash
cargo build -p a2a-wasm-component --target wasm32-wasip2 --release 2>&1
```

**Step 5: Run integration tests**

```bash
cargo test -p a2a-wasm-component --test integration_test 2>&1
```

Expected: Integration tests pass (no more "missing field `extensions`" errors).

---

## Task 9: Update Snapshots (if needed)

**Files:**
- Modify: `crates/a2a-wasm-component/tests/snapshots/*.snap` (if changed)

pbjson may produce slightly different JSON format (field ordering, enum names as strings).

**Step 1: If integration tests fail on snapshots, review changes**

```bash
cargo insta review
```

Accept snapshots if they represent correct proto3 JSON format.

**Step 2: Commit if snapshots changed**

```bash
git add crates/a2a-wasm-component/tests/snapshots/
git commit -m "test: update snapshots for pbjson format"
```

---

## Task 10: Run Clippy and Final Verification

**Files:** None (verification only)

**Step 1: Run clippy**

```bash
cargo clippy --workspace --exclude a2a-wasm-component 2>&1
```

Expected: No errors.

**Step 2: Clean build**

```bash
cargo clean
cargo build --workspace --exclude a2a-wasm-component 2>&1
```

Expected: Build succeeds.

---

## Reference: Why pbjson-build Uses include!()

The `.serde.rs` file contains `impl` blocks, not module definitions:

```rust
// v1.serde.rs contains:
impl serde::Serialize for Task { ... }
impl<'de> serde::Deserialize<'de> for Task { ... }
// etc.
```

These are trait implementations for types defined in `v1.rs`, so they must be in the same module scope. The `include!()` macro textually inserts the file contents, making the impls available. This is the [documented pattern](https://docs.rs/pbjson-build/latest/pbjson_build/) for pbjson-build.

---

## Reference: pbjson vs Manual Serde Derives

| Feature | Manual `#[derive(Serialize)]` | pbjson-build |
|---------|------------------------------|--------------|
| Empty repeated fields | Error on missing | `[]` default (proto3 compliant) |
| Enum serialization | i32 numbers | String names |
| Well-known types | Requires prost-wkt-types | Built-in via pbjson-types |
| Proto3 JSON spec | Partial | Full compliance |
