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

    // Skip serde for fields using prost_types::Struct (not serde-compatible)
    // These need custom serialization handling at the application layer
    config.field_attribute("a2a.v1.Task.metadata", "#[serde(skip)]");
    config.field_attribute("a2a.v1.Part.metadata", "#[serde(skip)]");
    config.field_attribute("a2a.v1.DataPart.data", "#[serde(skip)]");
    config.field_attribute("a2a.v1.Message.metadata", "#[serde(skip)]");
    config.field_attribute("a2a.v1.Artifact.metadata", "#[serde(skip)]");
    config.field_attribute("a2a.v1.TaskStatusUpdateEvent.metadata", "#[serde(skip)]");
    config.field_attribute("a2a.v1.TaskArtifactUpdateEvent.metadata", "#[serde(skip)]");
    config.field_attribute("a2a.v1.AgentExtension.params", "#[serde(skip)]");
    config.field_attribute("a2a.v1.AgentCardSignature.header", "#[serde(skip)]");
    config.field_attribute("a2a.v1.SendMessageRequest.metadata", "#[serde(skip)]");

    // Skip serde for fields using prost_types::Timestamp (not serde-compatible)
    config.field_attribute("a2a.v1.TaskStatus.timestamp", "#[serde(skip)]");

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
