//! End-to-end tests for the CLI surface (help, greet, completions).
//!
//! The binary is located via `CARGO_PKG_NAME` (resolved at compile time), so
//! these tests keep working after the project is renamed by scaffolding.

use std::path::PathBuf;
use std::process::Command;

/// Locate the built binary next to the test executable's target directory.
fn binary() -> PathBuf {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let name = format!("{}{}", env!("CARGO_PKG_NAME"), std::env::consts::EXE_SUFFIX);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(profile)
        .join(&name);
    assert!(path.is_file(), "test binary built at {}", path.display());
    path
}

fn run(args: &[&str]) -> (bool, String) {
    let output = Command::new(binary())
        .args(args)
        .output()
        .expect("run binary");
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    (output.status.success(), combined)
}

#[test]
fn help_lists_subcommands() {
    let (ok, out) = run(&["--help"]);
    assert!(ok, "help exits zero, got: {out}");
    assert!(out.contains("greet"), "help lists greet, got: {out}");
    assert!(
        out.contains("completions"),
        "help lists completions, got: {out}"
    );
}

#[test]
fn bare_invocation_greets_default_name() {
    let (ok, out) = run(&[]);
    assert!(ok, "bare run exits zero, got: {out}");
    assert!(
        out.contains("Hello, world!"),
        "default greeting, got: {out}"
    );
}

#[test]
fn greet_uses_name_flag() {
    let (ok, out) = run(&["greet", "--name", "Ferris"]);
    assert!(ok, "greet exits zero, got: {out}");
    assert!(out.contains("Hello, Ferris!"), "named greeting, got: {out}");
}

#[test]
fn greet_shout_uppercases() {
    let (ok, out) = run(&["greet", "--name", "Ferris", "--shout"]);
    assert!(ok, "shout exits zero, got: {out}");
    assert!(
        out.contains("HELLO, FERRIS!"),
        "shouted greeting, got: {out}"
    );
}

#[test]
fn completions_emit_shell_script() {
    let (ok, out) = run(&["completions", "bash"]);
    assert!(ok, "completions exits zero, got: {out}");
    assert!(
        out.contains(env!("CARGO_PKG_NAME")),
        "completions mention the binary, got: {out}"
    );
}
