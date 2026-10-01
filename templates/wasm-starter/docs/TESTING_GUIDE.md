# Testing guide

Two layers:

1. Native (`cargo test`): unit tests in `src/` (including doctests) plus
   `tests/test_greet.rs` over the pure logic. Runs everywhere, including L1
   CI — no browser needed.
2. Browser (`wasm-pack test --headless --chrome`): `tests/web.rs` with
   `wasm_bindgen_test`, configured with `run_in_browser`. The file starts
   with `#![cfg(target_arch = "wasm32")]` so native `cargo test` ignores it.

```sh
cargo test
wasm-pack test --headless --chrome
```

Manual: `cd www && npm install && npm run dev`, open the demo, check the
console output calls one exported fn.
