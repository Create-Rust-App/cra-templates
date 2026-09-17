# AGENTS.md — tonic-starter

Scaffolded with `create-rust-app` from the `tonic-starter` template.

## Commands

```sh
cargo run            # serve Greeter on $HOST:$PORT (defaults 0.0.0.0:50051)
cargo test           # unit + end-to-end gRPC tests
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Conventions

- `proto/*.proto` files are the contract source of truth; bindings under
  `src/proto/` are checked-in generator outputs (see `proto/REGENERATE.md`).
- One service implementation per concern in `src/service.rs` (split into
  modules as it grows); register servers in `src/main.rs`.
- Add RPCs to the `.proto` first, regenerate, then implement and cover them
  in `tests/test_grpc.rs` against an ephemeral port.
- `Config::from_env()` is the only runtime configuration source; add new
  settings there with defaults and document them in `.env.example`.
