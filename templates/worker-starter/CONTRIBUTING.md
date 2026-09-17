# Contributing

Thanks for contributing! English is canonical.

## Workflow

1. Add the job under `src/jobs/<feature>.rs` with a unit test.
2. Wire `Job`, `dispatch`, and cover it in `tests/test_queue.rs`.
3. Run the gates: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`.

## Rules

- Jobs stay serializable (serde payloads) so brokers can adopt them.
- Keep handlers total: return `anyhow::Result`, never panic.
- Use conventional commits (`feat:`, `fix:`, `docs:`, `chore:`).
