# Wasm Starter

Browser WASM library starter: `wasm-bindgen` exports, TypeScript bindings
generated with `wasm-pack`, Vite demo consumer, and headless browser tests.

## Use

```rust
use wasm_starter::{add_logic, greet_logic};

let greeting = greet_logic("World"); // "Hello, World!"
let sum = add_logic(2, 3); // 5
```

From JavaScript/TypeScript (after `wasm-pack build`):

```js
import init, { greet, add } from "../pkg/wasm_starter.js";

await init();
console.log(greet("World")); // "Hello, World!"
console.log(add(2, 3)); // 5
```

Run the native demo binary (no browser needed):

```sh
cargo run -- "World"
```

Build the browser package:

```sh
wasm-pack build --target web --out-dir pkg
```

Run the Vite demo:

```sh
cd www && npm install && npm run dev
```

## Quality gates

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
wasm-pack build --target web
wasm-pack test --headless --chrome
```

## Publishing

The `pkg/` output of `wasm-pack build` is an npm package: bump `version`
in `Cargo.toml`, rebuild, then publish `pkg/` with `npm publish` (see
`docs/DEPLOYMENT.md`). The Rust crate itself keeps publish-ready metadata
(`description`, `license`, `readme`, `repository`, `keywords`, `categories`).

## Docs

- [docs/README.md](docs/README.md) — index
- [docs/API.md](docs/API.md) — API reference
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — configuration
- [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) — layout
- [docs/TESTING_GUIDE.md](docs/TESTING_GUIDE.md) — testing
- [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) — browser build + npm publish
