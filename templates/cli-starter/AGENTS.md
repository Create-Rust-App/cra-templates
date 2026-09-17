# AGENTS.md — cli-starter

Scaffolded with `create-rust-app` from the `cli-starter` template.

## Commands

```sh
cargo run -- greet --name Ferris   # print a greeting
cargo run -- completions bash      # shell completions to stdout
cargo test                         # unit + end-to-end tests
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Conventions

- One subcommand per module under `src/commands/<feature>.rs`, exposing its
  `Args` struct and a `run` function.
- Declare new modules in `src/commands/mod.rs` and add a variant in
  `src/cli.rs::Commands`; dispatch in `src/commands/mod.rs::run`.
- Copy `src/commands/_feature_template.rs` to start a new subcommand.
- `Config::from_env()` is the only runtime configuration source; add new
  settings there with defaults and document them in `.env.example`.
- Every subcommand works non-interactively with flag-only input and prints
  copy-pasteable examples in its `--help`.
