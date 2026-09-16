# CRA Templates

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust Version](https://img.shields.io/badge/rust-1.82+-orange.svg)

[![CI Integrity (L0)](https://github.com/Create-Rust-App/cra-templates/actions/workflows/ci-integrity.yml/badge.svg?branch=main)](https://github.com/Create-Rust-App/cra-templates/actions/workflows/ci-integrity.yml)
[![CI Templates (L1)](https://github.com/Create-Rust-App/cra-templates/actions/workflows/ci-templates.yml/badge.svg?branch=main)](https://github.com/Create-Rust-App/cra-templates/actions/workflows/ci-templates.yml)
[![CI Extensions (L2)](https://github.com/Create-Rust-App/cra-templates/actions/workflows/ci-extensions.yml/badge.svg?branch=main)](https://github.com/Create-Rust-App/cra-templates/actions/workflows/ci-extensions.yml)
[![CI Profiles (L3)](https://github.com/Create-Rust-App/cra-templates/actions/workflows/ci-profiles.yml/badge.svg?branch=main)](https://github.com/Create-Rust-App/cra-templates/actions/workflows/ci-profiles.yml)

Official templates and extensions for `create-rust-app`.

## Quick start

Run the starter directly with Cargo:

```sh
cd templates/axum-starter
cargo run
# listening on 0.0.0.0:8080 — try:
curl localhost:8080/ping
curl localhost:8080/api/v1/healthz
```

For local development against this checkout, point the CLI at a `file://`
catalog path once `create-rust-app` is available:

```sh
create-rust-app my-api \
  --template "file://$PWD?subdir=templates/axum-starter" \
  --no-interactive
```

## Available templates

| Template | Type | Use case |
|----------|------|----------|
| [axum-starter](./templates/axum-starter) | `axum-backend` | Production-ready Axum HTTP API with feature-based modules, Tokio, tracing, `cargo fmt`/`clippy`/`test` |

## Available extensions

| Extension | Applies to | Use case |
|-----------|------------|----------|
| [all-github-setup](./extensions/all-github-setup) | all templates | GitHub Actions CI (fmt, clippy, test) for the scaffolded project |
| [axum-docker](./extensions/axum-docker) | `axum-backend` | Multi-stage Dockerfile for the Axum service |

## Repository layout

- `templates.json` — single registry of all templates, extensions, and categories.
- `templates.schema.json` — JSON Schema for the registry.
- `templates/<slug>/` — one scaffoldable starter per directory.
- `extensions/<slug>/` — overlays merged onto a scaffolded template.
- `ci/profiles/` — curated template+extension combinations exercised by L3 CI.
- `scripts/ci/` — registry validation and matrix generation used by all CI tiers.
- `docs/` — [authoring](docs/AUTHORING.md), [architecture](docs/ARCHITECTURE.md), [testing](docs/TESTING.md).

See [CONTRIBUTING.md](CONTRIBUTING.md) to add a template or extension.
