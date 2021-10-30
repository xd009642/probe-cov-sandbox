use cargo_tarpaulin::config::*;
use cargo_tarpaulin::source_analysis::*;
use cargo_tarpaulin::test_loader::*;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // This creates a tarpaulin configuration. It's needed for some of the tarpaulin functions
    // we're going to call for things like the file filter options the user can supply. But we
    // don't care about any of these so we'll make a default config and then push the project path
    // to it.
    let mut config = Config::default();
    config.manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test-project")
        .join("Cargo.toml");
    println!("Manifest: {}", config.manifest.display());

    // Runs tarpaulins source analysis. Here we get _uncoverable_ lines that should be ignored
    let analysis = SourceAnalysis::get_analysis(&config);

    // Now lets build the test binary we previously programmed onto the board. All the lines in
    // this example are ran and we're not using any tarpaulin filtering macros etc. So I don't care
    // about duplicating the rust flags tarpaulin would set _yet_. But we will in future and the
    // next paragraph will describe the work that has to be done for that (so feel free to skip
    // until we get there).
    //
    // In tarpaulin there's a module `src/cargo.rs` that handles setting up cargo to build the
    // project and working out where the generated artefacts are. For full integration we'll need
    // to figure out the appropriate build commands and _maybe_ if there is a target that it should be
    // built for - or something like an embed.toml which specifies the target. Depends if cargo
    // metadata still gives us everything we need. Also, doc tests are probably going to be _fun_.
    let x = Command::new("cargo")
        .args(&["build", "--example", "simple"])
        .current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-project"))
        .output()
        .unwrap();

    if !x.status.success() {
        panic!("Failed to build example");
    }

    // Hard coded but we could get cargo to help us if we really wanted (related to the future cargo 
    // build work mentioned above)
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test-project/target/thumbv7em-none-eabihf/debug/examples/simple");

    // get the lines we can get coverage for. We'll print this out and we will see all the lines
    // in our project and the addresses the instructions are at in the binary - so the points we
    // want to attach breakpoints. *However*, do we need the addresses here? Or can we simply use
    // probe-rs to attach the breakpoints given the file and line number? If we can that simplifies
    // it greatly and we can just get probe-rs to do it based on line location and any it can't set
    // we filter out as false-positive lines
    let tracemap = generate_tracemap(&output, &analysis.lines, &config).unwrap();

    println!("Tracemap?\n{:#?}", tracemap);

    // TODO Now the first step for the embedded coverage is to program the board. Can we just use
    // probe-rs to do this or do we need to rely on something like cargo-flash/cargo-embed? We
    // should probably detect if the user has cargo embed or cargo flash and their settings so we
    // program things correctly. Write some code to program the board (it'll also have to start
    // halted):


    // TODO Now ideally we go to tarpaulin and look at how to implement a statemachine in
    // `src/statemachine/`. There's a trait we can implement for StateData we'll want to create
    // some struct that holds any context we need for running the coverage (there'll be some sort
    // of probe-rs struct I imagine for the connection to the board that lets you send commands to
    // it. Looking at `instrumented.rs` is the most minimal statemachine possible - it just starts
    // the test and lets it run to end without adding any breakpoints. linux.rs has all the ptrace
    // faff so is much larger and more complicated but more realistic to what we'll do here. But
    // you can start with instrumented.rs as a base to just to run our example to the end. Then up the
    // complexity by adding breakpoints and doing the start/signal/collect process to collect
    // coverage
}
