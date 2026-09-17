# Tonic Health

Feature extension: standard gRPC health probing for the tonic service,
registered through the template's `CUSTOM_SERVICES` extension point (no file
forks).

## What it adds

- `tonic-health` v0.12 (merged into `[dependencies]`)
- `src/health.rs` — serves `grpc.health.v1.Health` with the Greeter marked
  `SERVING` (via the `CUSTOM_SERVICES` registration), plus a shared
  `reporter()` handle for tests
- `src/lib.rs.append` — `pub mod health;` declaration merged into the
  library
- `tests/test_health.rs` — a real client observes `SERVING` for
  `hello.Greeter` on an ephemeral server

## Compatibility

Applies to `tonic-grpc` templates. Requires the `CUSTOM_SERVICES` hook in
`src/server.rs` (present in `tonic-starter` since the hook landed).
