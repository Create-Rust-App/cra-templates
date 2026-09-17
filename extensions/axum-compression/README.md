# Axum Compression

Feature extension: gzip response compression for the Axum service,
registered through the template's `CUSTOM_LAYERS` extension point (no file
forks).

## What it adds

- `tower-http` with the `compression-gzip` feature (merged into
  `[dependencies]`; the template's existing features are kept via feature
  union)
- `src/compression.rs` — gzip `CompressionLayer` via the `CUSTOM_LAYERS`
  registration
- `src/lib.rs.append` — `pub mod compression;` declaration merged into the
  library
- `tests/test_compression.rs` — gzip served when accepted, identity
  otherwise

## Compatibility

Applies to `axum-backend` templates.
