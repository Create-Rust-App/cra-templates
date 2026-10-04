//! Browser WASM library starter.
//!
//! Pure logic lives in plain Rust functions (native-testable); thin
//! `#[wasm_bindgen]` wrappers expose them to JavaScript/TypeScript.
//! Copy `src/_feature_template.rs` to add a feature, declare it here,
//! and cover it in `tests/test_greet.rs` (native) plus `tests/web.rs`
//! (browser, via `wasm-pack test`).

use wasm_bindgen::prelude::*;

pub mod _feature_template;

/// Pure greeting logic — runs on any target, covered by native tests.
///
/// ```rust
/// assert_eq!(wasm_starter::greet_logic("World"), "Hello, World!");
/// ```
#[must_use]
pub fn greet_logic(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return "Hello, stranger!".to_string();
    }
    format!("Hello, {trimmed}!")
}

/// Pure addition logic — native-testable core of the `add` export.
///
/// ```rust
/// assert_eq!(wasm_starter::add_logic(2, 3), 5);
/// ```
#[must_use]
pub fn add_logic(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

/// Greet by name. Called from JS/TS as `greet(name)`.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    greet_logic(name)
}

/// Add two integers. Called from JS/TS as `add(a, b)`.
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    add_logic(a, b)
}

/// Install a readable panic hook in the browser. Runs on module init.
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::{add_logic, greet_logic};

    #[test]
    fn greets_by_name() {
        assert_eq!(greet_logic("World"), "Hello, World!");
    }

    #[test]
    fn greets_stranger_on_blank() {
        assert_eq!(greet_logic("   "), "Hello, stranger!");
    }

    #[test]
    fn adds_integers() {
        assert_eq!(add_logic(2, 3), 5);
    }
}
