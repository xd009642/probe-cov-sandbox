use cargo_tarpaulin::config::*;
use cargo_tarpaulin::source_analysis::*;
use std::path::PathBuf;

fn main() {
    let mut config = Config::default();
    config.manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test-project")
        .join("Cargo.toml");
    println!("Manifest: {}", config.manifest.display());
}
