//! Copy-paste starter for a new WASM feature.
//!
//! 1. Copy this file to `src/<feature>.rs`.
//! 2. Implement the pure logic as a plain function (native-testable).
//! 3. Expose a thin `#[wasm_bindgen]` wrapper for JS/TS.
//! 4. Declare the module in `src/lib.rs` and cover it in
//!    `tests/test_greet.rs` (native) and `tests/web.rs` (browser).

use wasm_bindgen::prelude::*;

/// Pure logic — runs on any target.
///
/// ```rust
/// assert_eq!(wasm_starter::_feature_template::hello_logic(), "hello");
/// ```
#[must_use]
pub fn hello_logic() -> &'static str {
    "hello"
}

/// Thin WASM export — no logic here, just delegate.
#[wasm_bindgen]
pub fn hello() -> String {
    hello_logic().to_string()
}

#[cfg(test)]
mod tests {
    use super::hello_logic;

    #[test]
    fn returns_hello() {
        assert_eq!(hello_logic(), "hello");
    }
}
