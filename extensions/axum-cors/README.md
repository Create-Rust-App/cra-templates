# Axum CORS

Feature extension: permissive Cross-Origin Resource Sharing for the Axum
service, registered through the template's `CUSTOM_LAYERS` extension point
(no file forks).

## What it adds

- `tower-http` with the `cors` feature (merged into `[dependencies]`; the
  template's existing `trace` feature is kept via feature union)
- `src/cors.rs` — `CorsLayer` allowing any origin/method/header, plus the
  `CUSTOM_LAYERS` registration
- `src/lib.rs.append` — `pub mod cors;` declaration merged into the library
- `tests/test_cors.rs` — cross-origin responses carry
  `access-control-allow-origin: *`

## Production hardening

Restrict `allow_origin`, `allow_methods`, and `allow_headers` in
`src/cors.rs` to the origins you serve before deploying.

## Compatibility

Applies to `axum-backend` templates.
