# AGENTS.md — wasm-starter

Scaffolded with `create-rust-app` from the `wasm-starter` template.

## Commands

```sh
cargo run -- "World"               # native demo over the public API
cargo test                         # native unit + integration tests
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
wasm-pack build --target web --out-dir pkg   # browser package + TS bindings
wasm-pack test --headless --chrome           # browser tests (tests/web.rs)
cd www && npm install && npm run dev         # Vite demo over pkg/
```

## Conventions

- Pure logic lives in plain functions (`src/<feature>.rs`) with unit tests;
  thin `#[wasm_bindgen]` wrappers expose them to JS/TS.
- Declare new modules in `src/lib.rs`; cover the native surface in
  `tests/test_greet.rs` and the browser surface in `tests/web.rs`.
- Copy `src/_feature_template.rs` to start a new feature.
- Public APIs carry doc comments with runnable examples (`cargo test`
  executes them as doctests).
- `src/main.rs` is a thin native demo over the pure logic, never WASM glue.
- `pkg/` is generated build output — never edit it by hand.
