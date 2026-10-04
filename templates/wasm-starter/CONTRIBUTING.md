# Contributing

Thanks for contributing! English is canonical.

## Workflow

1. Add the feature under `src/<feature>.rs` with unit tests and doc examples.
2. Declare it in `src/lib.rs`; cover native behavior in `tests/test_greet.rs`
   and browser behavior in `tests/web.rs`.
3. Run the gates: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`, `wasm-pack build --target web`,
   `wasm-pack test --headless --chrome`.

## Rules

- Keep the WASM glue thin: logic stays in plain functions, `#[wasm_bindgen]`
  wrappers only delegate.
- No new required dependencies without discussion (keep the tree lean).
- Follow semver: breaking changes bump minor while `0.x`.
- Use conventional commits (`feat:`, `fix:`, `docs:`, `chore:`).
