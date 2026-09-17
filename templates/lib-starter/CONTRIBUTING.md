# Contributing

Thanks for contributing! English is canonical.

## Workflow

1. Add the feature under `src/<feature>.rs` with unit tests and doc examples.
2. Declare it in `src/lib.rs` and cover it in `tests/test_lib.rs`.
3. Run the gates: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`, `cargo publish --dry-run`.

## Rules

- No new required dependencies without discussion (keep the tree lean).
- Follow semver: breaking changes bump minor while `0.x`.
- Use conventional commits (`feat:`, `fix:`, `docs:`, `chore:`).
