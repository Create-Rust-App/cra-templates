# AGENTS.md — lib-starter

Scaffolded with `create-rust-app` from the `lib-starter` template.

## Commands

```sh
cargo run -- "Some Name"        # demo binary over the public API
cargo test                       # unit + integration tests
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
cargo publish --dry-run          # pre-publish check
```

## Conventions

- One feature per module (`src/<feature>.rs`) with unit tests; declare it
  in `src/lib.rs` and cover the public surface in `tests/test_lib.rs`.
- Copy `src/_feature_template.rs` to start a new feature.
- Public APIs carry doc comments with runnable examples (`cargo test`
  executes them as doctests).
- `src/main.rs` is a thin demo over the public API, never business logic.
