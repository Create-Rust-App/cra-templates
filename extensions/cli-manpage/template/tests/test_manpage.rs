//! Man page end-to-end test: the `gen-man` example renders a valid page
//! covering the CLI surface.
//!
//! The example binary is located like `test_cli.rs` locates the main
//! binary: under `target/<profile>/examples`.

use std::path::PathBuf;
use std::process::Command;

/// Locate the built `gen-man` example next to the target directory.
fn example() -> PathBuf {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(profile)
        .join("examples")
        .join(format!("gen-man{}", std::env::consts::EXE_SUFFIX));
    assert!(path.is_file(), "example built at {}", path.display());
    path
}

#[test]
fn example_renders_man_page() {
    let output = Command::new(example()).output().expect("run gen-man");
    assert!(output.status.success(), "gen-man exits zero");
    let page = String::from_utf8(output.stdout).expect("page is UTF-8");
    assert!(page.contains(".TH"), "roff title macro");
    assert!(page.contains("greet"), "greet subcommand documented");
    assert!(page.contains("completions"), "completions documented");
}
