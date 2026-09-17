# CRA Templates

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust Version](https://img.shields.io/badge/rust-1.88+-orange.svg)

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
| [axum-starter](./templates/axum-starter) | `axum-backend` | Production-ready Axum HTTP API with feature-based modules, Tokio, tracing, and cargo fmt/clippy/test gates |
| [cli-starter](./templates/cli-starter) | `cli` | Production-ready clap CLI with feature-based subcommands, tracing, shell completions, and cargo fmt/clippy/test gates |
| [leptos-starter](./templates/leptos-starter) | `leptos-fullstack` | Leptos fullstack starter: SSR pages with htmx interactivity on Axum, tracing, and cargo fmt/clippy/test gates |
| [worker-starter](./templates/worker-starter) | `worker` | Tokio background worker starter: typed job queue, worker pool, demo scheduler, graceful shutdown, and cargo fmt/clippy/test gates |
| [lib-starter](./templates/lib-starter) | `library` | Publishable Rust library starter: feature modules, doc examples, demo binary, and cargo fmt/clippy/test gates |
| [tonic-starter](./templates/tonic-starter) | `tonic-grpc` | tonic gRPC microservice starter: Protobuf contract, checked-in bindings, reflection, graceful shutdown, and cargo fmt/clippy/test gates |

## Available extensions

| Extension | Applies to | Use case |
|-----------|------------|----------|
| [all-github-setup](./extensions/all-github-setup) | all templates | GitHub Actions CI (fmt, clippy, test) for the scaffolded project |
| [axum-docker](./extensions/axum-docker) | `axum-backend` | Multi-stage Docker build and Compose stack for the Axum service |
| [all-devcontainer](./extensions/all-devcontainer) | all templates | Reproducible VS Code dev container (Rust image, rust-analyzer) for any scaffolded project |
| [all-pre-commit](./extensions/all-pre-commit) | all templates | Pre-commit hooks mirroring CI gates (cargo fmt, clippy -D warnings) |
| [axum-cors](./extensions/axum-cors) | `axum-backend` | Permissive CORS layer for the Axum service, registered without file forks |
| [axum-jwt](./extensions/axum-jwt) | `axum-backend` | HS256 JWT authentication with a demo protected route for the Axum service |
| [axum-openapi](./extensions/axum-openapi) | `axum-backend` | Served OpenAPI 3.1 document plus Redoc UI for the Axum service |
| [axum-sqlx](./extensions/axum-sqlx) | `axum-backend` | SQLite persistence with embedded migrations and a todo CRUD API for the Axum service |
| [axum-compression](./extensions/axum-compression) | `axum-backend` | Gzip response compression for the Axum service |
| [axum-timeout](./extensions/axum-timeout) | `axum-backend` | Fail-fast 10s request timeouts for the Axum service |
| [axum-request-id](./extensions/axum-request-id) | `axum-backend` | Unique x-request-id on every response for log correlation |
| [axum-ratelimit](./extensions/axum-ratelimit) | `axum-backend` | Per-key rate limiting with 429 responses for the Axum service |
| [all-gitlab-setup](./extensions/all-gitlab-setup) | all templates | GitLab CI pipeline for any scaffolded project |
| [tonic-health](./extensions/tonic-health) | `tonic-grpc` | Standard gRPC health probing with the Greeter marked SERVING |

## Repository layout

- `templates.json` — single registry of all templates, extensions, and categories.
- `templates.schema.json` — JSON Schema for the registry.
- `templates/<slug>/` — one scaffoldable starter per directory.
- `extensions/<slug>/` — overlays merged onto a scaffolded template.
- `ci/profiles/` — curated template+extension combinations exercised by L3 CI.
- `scripts/ci/` — registry validation and matrix generation used by all CI tiers.
- `docs/` — [authoring](docs/AUTHORING.md), [architecture](docs/ARCHITECTURE.md), [testing](docs/TESTING.md).

See [CONTRIBUTING.md](CONTRIBUTING.md) to add a template or extension.
