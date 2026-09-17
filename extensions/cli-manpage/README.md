# CLI Manpage

Feature extension: generated man pages for CLI starters. Needs engine
`create-rust-app` 0.4.0+, which merges `[[example]]` target sections
(earlier releases ignore them).

## What it adds

- `clap_mangen` v0.2 (merged into `[dev-dependencies]`)
- `[[example]]` target `gen-man` (merged by target name)
- `examples/gen-man.rs` — renders the man page from the clap definition:
  `cargo run --example gen-man > <name>.1`
- `tests/test_manpage.rs` — the example renders a page covering the CLI
  surface

## Compatibility

Applies to `cli` templates exposing a `Cli::command()` factory (i.e.
`cli-starter`).
