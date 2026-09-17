# Axum Timeout

Feature extension: fail-fast request timeouts for the Axum service,
registered through the template's `CUSTOM_LAYERS` extension point (no file
forks).

## What it adds

- `tower-http` with the `timeout` feature (merged into `[dependencies]`;
  the template's existing features are kept via feature union)
- `src/timeout.rs` — 10s [`TIMEOUT`] applied to every route via the
  `CUSTOM_LAYERS` registration; slow handlers answer `408 Request Timeout`
- `src/lib.rs.append` — `pub mod timeout;` declaration merged into the
  library
- `tests/test_timeout.rs` — a slow handler times out with a short test
  timeout, and fast routes pass through the composed service untouched

## Compatibility

Applies to `axum-backend` templates.
