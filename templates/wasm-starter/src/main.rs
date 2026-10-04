//! Demo binary exercising the public library API end to end.
//!
//! Runs natively (no browser needed); the browser path is covered by
//! `tests/web.rs` under `wasm-pack test`.

use wasm_starter::{add_logic, greet_logic};

fn main() {
    let name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "World".to_string());
    println!("{}", greet_logic(&name));
    println!("2 + 3 = {}", add_logic(2, 3));
}
