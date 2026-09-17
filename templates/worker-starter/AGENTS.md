# AGENTS.md — worker-starter

Scaffolded with `create-rust-app` from the `worker-starter` template.

## Commands

```sh
cargo run            # worker pool + demo scheduler (Ctrl-C to stop)
cargo test           # unit + end-to-end queue tests
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Conventions

- One job per module under `src/jobs/<feature>.rs`, exposing its payload
  struct and an `execute` function.
- Declare new jobs in `src/jobs/mod.rs::Job` and dispatch them in
  `src/jobs::dispatch`; copy `src/jobs/_feature_template.rs` to start.
- Producers use `queue::pool(workers)` + `Producer::submit` (awaitable) or
  `Producer::enqueue` (fire and forget); workers run `Consumer::run`.
- `Config::from_env()` is the only runtime configuration source; add new
  settings there with defaults and document them in `.env.example`.
