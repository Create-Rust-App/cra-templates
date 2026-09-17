# Axum OpenTelemetry

Feature extension: OTLP/gRPC trace export for the Axum service, registered
through the template's `CUSTOM_TRACING` extension point (no file forks).

## What it adds

- `opentelemetry` + `opentelemetry-otlp` (gRPC) + `opentelemetry_sdk` v0.28
  and `tracing-opentelemetry` v0.29 (merged into `[dependencies]`)
- `src/otel.rs` — builds the export layer from the environment, or an
  inert no-op without an endpoint, via the `CUSTOM_TRACING` registration
- `src/lib.rs.append` — `pub mod otel;` declaration merged into the library
- `tests/test_otel.rs` — the layer builds with and without an endpoint,
  registers exactly once, and the composed subscriber installs

## Configuration

| Variable                       | Default          | Description                          |
| ------------------------------ | ---------------- | ------------------------------------ |
| `OTEL_EXPORTER_OTLP_ENDPOINT`  | (unset)          | OTLP/gRPC collector endpoint         |
| `OTEL_SERVICE_NAME`            | `axum-starter`   | Service name reported to the collector |

Requires the `CUSTOM_TRACING` hook (`src/telemetry.rs`, present in
`axum-starter` since the hook landed).

## Compatibility

Applies to `axum-backend` templates.
