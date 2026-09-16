# Configuration — axum-starter

All configuration comes from the environment via `Config::from_env()`.
Copy `.env.example` to `.env` for local development.

| Variable | Default | Purpose |
|----------|---------|---------|
| `HOST` | `0.0.0.0` | Interface to bind |
| `PORT` | `8080` | TCP port to listen on |
| `API_PREFIX` | `/api/v1` | Prefix for versioned API routes |
| `RUST_LOG` | `axum_starter=debug,tower_http=debug` | `tracing` log filter |

The scaffold-time `apiPrefix` option (`cra.config.json`) sets the initial
`API_PREFIX` default for generated projects.
