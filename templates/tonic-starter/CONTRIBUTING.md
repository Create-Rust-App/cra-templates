# Contributing

Thanks for contributing! English is canonical.

## Workflow

1. Extend `proto/*.proto`, regenerate per `proto/REGENERATE.md`.
2. Implement the RPC in `src/service.rs` with a unit test.
3. Cover it end to end in `tests/test_grpc.rs` (ephemeral port pattern).
4. Run the gates: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`.

## Rules

- Never hand-edit generated files under `src/proto/`.
- Keep the reflection descriptor set in sync with the bindings.
- Use conventional commits (`feat:`, `fix:`, `docs:`, `chore:`).
