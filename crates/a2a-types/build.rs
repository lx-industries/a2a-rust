use protox::prost::Message;
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
