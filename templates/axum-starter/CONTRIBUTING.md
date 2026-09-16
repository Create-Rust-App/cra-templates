# Contributing — axum-starter project

## Development loop

```sh
cargo run
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
```

## Adding a feature

1. Copy `src/routes/_feature_template.rs` to `src/routes/<feature>.rs`
2. Declare it in `src/routes/mod.rs`
3. Merge `router()` in `src/app.rs::create_app`
4. Add integration tests under `tests/`
5. Document endpoints in `docs/API.md`

Use conventional commits (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`).
