use lib_flutter_rust_bridge_codegen::codegen;
use lib_flutter_rust_bridge_codegen::codegen::Config;
use std::process::Command;

fn main() -> anyhow::Result<()> {
    // Only regenerate bindings when API code changes (accelerates builds)
    println!("cargo:rerun-if-changed=src/api");

    // Check if Flutter/Dart is available before trying to generate bindings
    // This allows building the Rust library without Flutter when bindings are already generated
    let flutter_available = Command::new("flutter")
        .arg("--version")
        .output()
        .is_ok();

    if !flutter_available {
        // Skip binding generation if Flutter is not available
        // Bindings should already exist from a previous generation
        eprintln!("Warning: Flutter not available, skipping binding generation");
        eprintln!("         Using existing Dart bindings (if any)");
        return Ok(());
    }

    // Auto-generate Dart/Flutter bindings for Rust FFI
    // This runs during cargo build and regenerates bindings when Rust code changes
    codegen::generate(
        Config::from_config_file("flutter_rust_bridge.yaml")?.unwrap(),
        Default::default(),
    )
}
