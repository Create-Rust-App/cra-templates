# Leptos Starter

Fullstack [Leptos](https://github.com/leptos-rs/leptos) starter: server-side
rendered pages with htmx interactivity on an Axum + Tokio runtime, `tracing`
observability, and `cargo fmt` / `clippy` / `test` quality gates.

## Run

```sh
cargo run
open http://localhost:3000/
```

- `GET /` — server-rendered home with an interactive counter
- `GET /api/healthz` — JSON health probe
- `POST /api/count` — htmx fragment endpoint (increments the counter)

Configure with environment variables (see `.env.example`):

| Variable | Default | Purpose |
|----------|---------|---------|
| `HOST` | `0.0.0.0` | Interface to bind |
| `PORT` | `3000` | TCP port |
| `RUST_LOG` | `leptos_starter=debug` | Log filter |

## Interactivity without WASM

Widgets render on the server; the counter button posts to `/api/count` and
htmx swaps the refreshed fragment in place. To upgrade to hydrated islands,
install [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) and follow
`docs/DEPLOYMENT.md`.

## Quality gates

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Docs

- [docs/README.md](docs/README.md) — index
- [docs/API.md](docs/API.md) — route reference
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — configuration
- [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) — layout
- [docs/TESTING_GUIDE.md](docs/TESTING_GUIDE.md) — testing
- [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) — deployment
