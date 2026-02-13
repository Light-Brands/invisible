use lib_flutter_rust_bridge_codegen::codegen;
use lib_flutter_rust_bridge_codegen::codegen::Config;

fn main() -> anyhow::Result<()> {
    // NOTE: Uncomment the line below to only regenerate bindings on api directory changes
    // This accelerates builds but requires manual regeneration for dependency changes
    // println!("cargo:rerun-if-changed=src/api");

    // Auto-generate Dart/Flutter bindings for Rust FFI
    // This runs during cargo build and regenerates bindings when Rust code changes
    codegen::generate(
        Config::from_config_file("flutter_rust_bridge.yaml")?.unwrap(),
        Default::default(),
    )
}
