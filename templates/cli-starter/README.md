# CLI Starter

Production-ready [clap](https://github.com/clap-rs/clap) CLI starter with a
feature-based subcommand layout, `tracing` observability, shell completions,
and `cargo fmt` / `clippy` / `test` quality gates.

## Run

```sh
cargo run -- greet --name Ferris
cargo run -- completions bash
```

Configure with environment variables (see `.env.example`):

| Variable | Default | Purpose |
|----------|---------|---------|
| `CLI_DEFAULT_NAME` | `world` | Name greeted when `--name` is omitted |
| `RUST_LOG` | `cli_starter=debug` | Log filter |

## Quality gates

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Docs

- [docs/README.md](docs/README.md) — index
- [docs/API.md](docs/API.md) — command reference
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — configuration
- [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) — layout
- [docs/TESTING_GUIDE.md](docs/TESTING_GUIDE.md) — testing
- [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) — distribution
