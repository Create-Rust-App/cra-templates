# Contributing

Thanks for contributing! English is canonical.

## Workflow

1. Add the page under `src/pages/<feature>.rs` (or widget under
   `src/components/`), with unit tests for fragments.
2. Route it in `src/main.rs` and cover it in `tests/test_pages.rs`.
3. Run the gates: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`.

## Rules

- All rendering goes through `leptos::ssr::render_to_string`.
- Keep interactive widgets htmx-driven (no WASM build step in CI).
- Use conventional commits (`feat:`, `fix:`, `docs:`, `chore:`).
