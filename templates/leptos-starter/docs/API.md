# Route reference

| Method | Path | Response |
|--------|------|----------|
| `GET` | `/` | SSR home page (counter widget) |
| `GET` | `/api/healthz` | `{"status": "ok"}` |
| `POST` | `/api/count` | Counter widget fragment (`{"count": N}` in) |
| any | unknown | SSR 404 page |

The counter button posts `{"count": N}` as JSON; the endpoint answers with
the widget re-rendered at `N + 1`, which htmx swaps into `#counter`.
