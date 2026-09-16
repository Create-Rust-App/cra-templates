# GitHub Setup

Cross-cutting extension: GitHub Actions CI plus Dependabot for any
scaffolded project.

## What it adds

- `.github/workflows/ci.yml` — `cargo fmt --check`, `cargo clippy` with
  `-D warnings`, and `cargo test` on pushes and pull requests
- `.github/dependabot.yml` — weekly Cargo dependency updates

## Compatibility

Applies to every template type.
