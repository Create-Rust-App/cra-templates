# Axum Docker

Stack-bound extension for `axum-backend` templates: multi-stage Docker
build and a Compose stack for the Axum service.

## What it adds

- `Dockerfile` — `cargo chef`-free two-stage build pinned to `rust:1.82`,
  minimal Debian runtime, non-root user
- `compose.yaml` — service with `PORT`/`API_PREFIX` wiring and a `/ping`
  healthcheck

## Compatibility

Applies to `axum-backend` templates only.
