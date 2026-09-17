# AGENTS.md — leptos-starter

Scaffolded with `create-rust-app` from the `leptos-starter` template.

## Commands

```sh
cargo run            # serve on $HOST:$PORT (defaults 0.0.0.0:3000)
cargo test           # unit + integration tests
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Conventions

- One page per module under `src/pages/<feature>.rs`, exposing a `#[component]`.
- One widget per module under `src/components/<feature>.rs`; fragments that
  answer htmx requests live next to their widget usage.
- API modules under `src/api/<feature>.rs` serve JSON or HTML fragments.
- Render through `leptos::ssr::render_to_string` (it owns the reactive
  runtime); never call `.render_to_string()` on a view outside it.
- Copy `src/pages/home.rs` to start a new page; route it in `src/main.rs`.
- `Config::from_env()` is the only runtime configuration source; add new
  settings there with defaults and document them in `.env.example`.
