# Axum Rate Limit

Feature extension: per-key rate limiting for the Axum service, registered
through the template's `CUSTOM_LAYERS` extension point (no file forks).

## What it adds

- `tower_governor` v0.5 (merged into `[dependencies]`; the 0.6+ line targets
  axum 0.8, while `axum-starter` currently uses axum 0.7)
- `src/rate_limit.rs` — quotas keyed by the `x-api-key` header (callers
  without one share a single anonymous bucket): a burst of 2 requests,
  replenished at 2 per second, via the `CUSTOM_LAYERS` registration.
  Over-quota callers get `429 Too Many Requests`.
- `src/lib.rs.append` — `pub mod rate_limit;` declaration merged into the
  library
- `tests/test_ratelimit.rs` — burst cap, per-key isolation, and the shared
  anonymous bucket

## Compatibility

Applies to `axum-backend` templates.

Note: this extension is intentionally excluded from the
`axum-starter-extensions` L3 profile. Quotas are process-global, so the
combined hammering of every suite would trip them; each suite passes
standalone and in smaller compositions.
