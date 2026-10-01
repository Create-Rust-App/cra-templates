# API

## `greet(name: &str) -> String` (WASM + native `greet_logic`)

Returns `Hello, <name>!`, or `Hello, stranger!` for blank input.

```rust
assert_eq!(wasm_starter::greet_logic("World"), "Hello, World!");
```

```js
import init, { greet } from "../pkg/wasm_starter.js";
await init();
greet("World"); // "Hello, World!"
```

## `add(a: i32, b: i32) -> i32` (WASM + native `add_logic`)

Wrapping integer addition.

```rust
assert_eq!(wasm_starter::add_logic(2, 3), 5);
```

```js
import init, { add } from "../pkg/wasm_starter.js";
await init();
add(2, 3); // 5
```
