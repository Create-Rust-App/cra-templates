//! Integration tests for the public library API (native, no browser needed).

use wasm_starter::{add_logic, greet_logic};

#[test]
fn public_greet() {
    assert_eq!(greet_logic("World"), "Hello, World!");
}

#[test]
fn public_greet_stranger_on_blank() {
    assert_eq!(greet_logic("   "), "Hello, stranger!");
}

#[test]
fn public_add() {
    assert_eq!(add_logic(2, 3), 5);
}
