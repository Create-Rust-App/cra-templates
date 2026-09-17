# Worker Starter

Tokio background worker starter: typed job queue, worker pool with
round-robin dispatch, demo scheduler, graceful shutdown, and `cargo fmt` /
`clippy` / `test` quality gates.

## Run

```sh
cargo run
```

Workers greet every `TICK_SECS` until Ctrl-C. Configure with environment
variables (see `.env.example`):

| Variable | Default | Purpose |
|----------|---------|---------|
| `WORKER_COUNT` | `4` | Worker tasks in the pool |
| `TICK_SECS` | `30` | Seconds between demo scheduler ticks |
| `RUST_LOG` | `worker_starter=debug` | Log filter |

## Durable queues

The built-in queue is in-process. For Redis/Postgres-backed durable queues,
apply the task-queue extension (apalis).

## Quality gates

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Docs

- [docs/README.md](docs/README.md) — index
- [docs/API.md](docs/API.md) — job reference
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — configuration
- [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) — layout
- [docs/TESTING_GUIDE.md](docs/TESTING_GUIDE.md) — testing
- [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) — deployment
