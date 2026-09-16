# Axum Starter

Production-ready [Axum](https://github.com/tokio-rs/axum) HTTP API starter with
a feature-based module layout, Tokio runtime, `tracing` observability, and
`cargo fmt` / `clippy` / `test` quality gates.

## Run

```sh
cargo run
curl localhost:8080/ping
curl localhost:8080/api/v1/healthz
```

Configure with environment variables (see `.env.example`):

| Variable | Default | Purpose |
|----------|---------|---------|
| `HOST` | `0.0.0.0` | Interface to bind |
| `PORT` | `8080` | TCP port |
| `API_PREFIX` | `/api/v1` | Prefix for versioned routes |
| `RUST_LOG` | `axum_starter=debug,tower_http=debug` | Log filter |

## Quality gates

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Docs

- [docs/README.md](docs/README.md) — index
- [docs/API.md](docs/API.md) — endpoint reference
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — configuration
- [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) — layout
- [docs/TESTING_GUIDE.md](docs/TESTING_GUIDE.md) — testing
- [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) — deployment
