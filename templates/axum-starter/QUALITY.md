# QUALITY.md — axum-starter

Quality bar for this template (checked by bank CI and locally).

## Gates

- `cargo fmt --all -- --check` — clean
- `cargo clippy --all-targets -- -D warnings` — zero warnings
- `cargo test` — all green (unit + integration)

## Coverage

- Every route has an integration test in `tests/`
- Every config field has a default covered by a unit test
- Docs index (`docs/README.md`), structure, configuration, testing guide,
  deployment, and API reference all present and current
