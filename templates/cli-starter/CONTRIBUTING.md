# Contributing

Thanks for contributing! English is canonical.

## Workflow

1. Add the subcommand under `src/commands/<feature>.rs` with unit tests.
2. Wire `mod.rs`, `cli.rs`, and `run` dispatch.
3. Add end-to-end coverage in `tests/test_cli.rs`.
4. Run the gates: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`.

## Rules

- Subcommands stay non-interactive: flags only, no prompts.
- Keep `--help` examples runnable.
- Use conventional commits (`feat:`, `fix:`, `docs:`, `chore:`).
