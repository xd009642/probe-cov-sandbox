use cargo_tarpaulin::source_analysis::*;
use cargo_tarpaulin::config::*;
use std::path::PathBuf;

fn main() {
    let mut config = Config::default();
    config.manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-project").join("Cargo.toml");
    println!("Manifest: {}", config.manifest.display());

    let analysis = SourceAnalysis::get_analysis(&config);

    // TODO Build the test binary 
    
    // TODO get the lines we can get coverage for 

    // TODO Program board with test binary

    // TODO Use probe-rs to collect coverage info
}
