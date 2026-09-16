# Testing — cra-templates

## Local commands

```sh
# L0: registry + profiles
python3 scripts/ci/validate-registry.py
python3 scripts/ci/generate-matrix.py --layer validate-profiles

# L1: template gates (run inside each template dir)
cd templates/axum-starter
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test

# Matrix previews (print JSON when GITHUB_OUTPUT is unset)
python3 scripts/ci/generate-matrix.py --layer templates --base-ref origin/main
python3 scripts/ci/generate-matrix.py --layer extensions --base-ref origin/main
python3 scripts/ci/generate-matrix.py --layer profiles
```

## Scaffold smoke test (once the CLI is available)

```sh
create-rust-app my-api \
  --template "file://$PWD?subdir=templates/axum-starter" \
  --addons "file://$PWD?subdir=extensions/all-github-setup" \
  --no-interactive
cd my-api && cargo test
```

## CI details

L1–L3 jobs install the pinned toolchain (`rust-toolchain.toml`, 1.82.0)
via `dtolnay/rust-toolchain` and cache Cargo with `Swatinem/rust-cache`.
L3 exports each profile's `env` block before running `cargo test`, so
profiles like a custom `API_PREFIX` are exercised, not just parsed.
