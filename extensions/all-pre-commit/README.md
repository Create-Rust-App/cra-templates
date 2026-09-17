# Pre-commit

Cross-cutting extension: pre-commit hooks mirroring the CI quality gates
for any scaffolded project.

## What it adds

- `.pre-commit-config.yaml` — `cargo fmt` and `cargo clippy --all-targets
  -- -D warnings` via `pre-commit-rust`

Install the hooks with `pip install pre-commit && pre-commit install`.

## Compatibility

Applies to every template type.
