//! Browser tests — run with `wasm-pack test --headless --chrome`.
//!
//! Gated to `wasm32` so plain `cargo test` (native, L1 CI) ignores this file.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use wasm_starter::{add, greet};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn browser_greet() {
    assert_eq!(greet("World"), "Hello, World!");
}

#[wasm_bindgen_test]
fn browser_add() {
    assert_eq!(add(2, 3), 5);
}
