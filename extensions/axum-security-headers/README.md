# Axum Security Headers

Feature extension: baseline response hardening for the Axum service,
registered through the template's `CUSTOM_LAYERS` extension point (no file
forks).

## What it adds

- `tower-http` with the `set-header` feature (merged into `[dependencies]`;
  the template's existing features are kept via feature union)
- `src/security_headers.rs` — `SECURITY_HEADERS` (`nosniff`, `DENY`,
  strict referrer policy, same-origin opener policy) stamped on every
  response via the `CUSTOM_LAYERS` registration
- `src/lib.rs.append` — `pub mod security_headers;` declaration merged into
  the library
- `tests/test_security_headers.rs` — every route carries the full set with
  the documented values

## Compatibility

Applies to `axum-backend` templates.
