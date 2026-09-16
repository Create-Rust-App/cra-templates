# AGENTS.md — axum-starter

Scaffolded with `create-rust-app` from the `axum-starter` template.

## Commands

```sh
cargo run          # serve on $HOST:$PORT (defaults 0.0.0.0:8080)
cargo test         # unit + integration tests
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Conventions

- One feature per module under `src/routes/<feature>.rs`, exposing `router()`.
- Merge new feature routers in `src/app.rs::create_app`.
- Shared request/response shapes go next to their feature module.
- Copy `src/routes/_feature_template.rs` to start a new feature.
- `Config::from_env()` is the only runtime configuration source; add new
  settings there with defaults and document them in `.env.example`.
