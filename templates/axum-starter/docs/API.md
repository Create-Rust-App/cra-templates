# API Reference — axum-starter

Base URL defaults to `http://localhost:8080`. Versioned routes live under
`{API_PREFIX}` (default `/api/v1`).

## `GET /ping`

Minimal health probe for load balancers. Not versioned, excluded from any
future OpenAPI document.

Response `200`:

```json
{ "status": "ok" }
```

## `GET /`

Root endpoint with navigation hints.

Response `200`:

```json
{ "message": "Welcome to axum-starter", "health": "/api/v1/healthz" }
```

## `GET {API_PREFIX}/healthz`

Service health endpoint.

Response `200`:

```json
{ "status": "ok" }
```
