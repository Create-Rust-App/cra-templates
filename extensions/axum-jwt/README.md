# Axum JWT

Feature extension: HS256 JWT authentication for the Axum service, registered
through the template's `CUSTOM_ROUTERS` extension point (no file forks).

## What it adds

- `jsonwebtoken` v9 (merged into `[dependencies]`)
- `src/auth.rs` — `Claims`, `AuthConfig::from_env()`, `create_token` /
  `verify_token` helpers, and the `AuthUser` extractor (rejects requests
  without a valid `Authorization: Bearer <token>` header with `401`)
- `src/auth_routes.rs` — demo protected `GET /me` route echoing the token
  subject, registered on `CUSTOM_ROUTERS` so it merges under the API prefix
- `src/lib.rs.append` — `pub mod auth;` and `pub mod auth_routes;`
  declarations merged into the library
- `tests/test_jwt.rs` — token round-trip plus protected-route cases
  (missing token, bad token, valid token)

## Configuration

| Variable      | Default              | Description                        |
| ------------- | -------------------- | ---------------------------------- |
| `JWT_SECRET`  | `dev-only-change-me` | HMAC secret for signing/verifying  |
| `JWT_TTL_SECS`| `3600`               | Token time-to-live in seconds      |

Set a strong `JWT_SECRET` before deploying.

## Compatibility

Applies to `axum-backend` templates.
