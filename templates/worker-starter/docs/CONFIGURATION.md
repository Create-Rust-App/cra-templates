# Configuration

Environment variables (see `.env.example`):

| Variable | Default | Purpose |
|----------|---------|---------|
| `WORKER_COUNT` | `4` | Worker tasks in the pool |
| `TICK_SECS` | `30` | Seconds between demo scheduler ticks |
| `RUST_LOG` | `worker_starter=debug` | `tracing` filter directives |
