# Axum OpenAPI

Feature extension: served OpenAPI 3.1 document plus Redoc UI for the Axum
service, registered through the template's `CUSTOM_ROUTERS` extension point
(no file forks).

## What it adds

- `utoipa` v5 and `utoipa-redoc` v5 (merged into `[dependencies]`)
- `src/openapi.rs` — `ApiDoc` document, `GET /openapi.json` serving the
  spec, and the Redoc UI under `/docs`, registered on `CUSTOM_ROUTERS` so
  both merge under the API prefix (`/api/v1/openapi.json`, `/api/v1/docs`)
- `src/lib.rs.append` — `pub mod openapi;` declaration merged into the library
- `tests/test_openapi.rs` — the document parses with the expected endpoints,
  and the Redoc UI renders

## Compatibility

Applies to `axum-backend` templates. The documented paths assume the default
`/api/v1` prefix; serving still works under a custom `API_PREFIX`.
