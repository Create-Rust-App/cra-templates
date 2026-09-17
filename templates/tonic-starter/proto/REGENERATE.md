# Regenerating the Protobuf bindings

`src/proto/hello.rs` and `src/proto/hello_descriptor.bin` are generated
from `proto/hello.proto` and checked in, so `cargo build` and `cargo test`
never need `protoc`. Regenerate only when the contract changes.

## Prerequisites

- `protoc` (any recent 3.x/4.x/5.x/6.x release)
- Rust toolchain with the `tonic-build` version below

## Steps

1. Create a scratch crate (never in this repo):

   ```toml
   [build-dependencies]
   tonic-build = "=0.12.3"
   ```

   ```rust
   // build.rs
   fn main() {
       tonic_build::configure()
           .file_descriptor_set_path("src/hello_descriptor.bin")
           .compile_protos(&["proto/hello.proto"], &["proto"])
           .unwrap();
   }
   ```

2. Copy `proto/hello.proto` into the scratch crate, `cargo build`, and copy
   back `OUT_DIR/hello.rs` → `src/proto/hello.rs` plus the descriptor set →
   `src/proto/hello_descriptor.bin`.
3. Run the gates: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`.
