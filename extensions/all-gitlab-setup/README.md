# GitLab Setup

Cross-cutting extension: GitLab CI for any scaffolded project. Vendor-neutral
counterpart to `all-github-setup` — pick the forge your project lives on.

## What it adds

- `.gitlab-ci.yml` — `cargo fmt --check` and `cargo clippy` with
  `-D warnings` in `lint`, plus `cargo test` in `test`, on the
  `rust:1.88-bookworm` image (tracks the bank toolchain pin)

## Compatibility

Applies to every template type.
