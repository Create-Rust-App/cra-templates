# Configuration

No runtime configuration — the library reads no env vars (see
`.env.example`).

Build configuration:

- `rust-toolchain.toml` pins Rust `1.88.0` with the
  `wasm32-unknown-unknown` target plus `rustfmt`/`clippy`.
- `Cargo.toml` sets `[lib] crate-type = ["cdylib", "rlib"]` so the crate
  builds both as a native rlib (tests, demo binary) and as a WASM cdylib.
- `wasm-pack build --target web --out-dir pkg` generates `pkg/` with the
  `.wasm` binary plus TypeScript bindings (gitignored build output).
- `www/` is a Vite app that imports from `../pkg/`.

Why no `wee_alloc`: the allocator crate is unmaintained and unnecessary for
a starter — the default allocator keeps the build lean and avoids a
deprecated dependency. Document here if you outgrow it.
