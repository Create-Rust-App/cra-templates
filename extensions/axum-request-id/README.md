# Axum Request ID

Feature extension: unique request IDs for the Axum service, registered
through the template's `CUSTOM_LAYERS` extension point (no file forks).

## What it adds

- `tower-http` with the `request-id` feature (merged into `[dependencies]`;
  the template's existing features are kept via feature union)
- `src/request_id.rs` — stamps a UUID `x-request-id` on every request and
  propagates it back on the response, via the `CUSTOM_LAYERS` registration
- `src/lib.rs.append` — `pub mod request_id;` declaration merged into the
  library
- `tests/test_request_id.rs` — every response carries a unique id

## Compatibility

Applies to `axum-backend` templates.
