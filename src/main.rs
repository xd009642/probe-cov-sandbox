use cargo_tarpaulin::config::*;
use cargo_tarpaulin::source_analysis::*;
use cargo_tarpaulin::test_loader::*;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let mut config = Config::default();
    config.manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test-project")
        .join("Cargo.toml");
    println!("Manifest: {}", config.manifest.display());

    let analysis = SourceAnalysis::get_analysis(&config);

    // TODO Build the test binary
    let x = Command::new("cargo")
        .args(&["build", "--example", "simple"])
        .current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-project"))
        .output()
        .unwrap();

    if !x.status.success() {
        panic!("Failed to build example");
    }

    // Hard coded but we could get cargo to help us if we really wanted
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test-project/target/thumbv7em-none-eabihf/debug/examples/simple");

    // get the lines we can get coverage for
    let tracemap = generate_tracemap(&output, &analysis.lines, &config).unwrap();

    println!("Tracemap?\n{:#?}", tracemap);

    // TODO Program board with test binary

    // TODO Use probe-rs to collect coverage info
}
