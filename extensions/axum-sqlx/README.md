# Axum SQLx

Feature extension: SQLite persistence via SQLx for the Axum service, with a
demo todo CRUD API registered through the template's `CUSTOM_ROUTERS`
extension point (no file forks).

## What it adds

- `sqlx` v8 with `sqlite`, `runtime-tokio`, and `migrate`
  (merged into `[dependencies]`; SQLite is built from bundled sources, so
  scaffolded projects need no system library)
- `migrations/0001_todos.sql` — embedded via `sqlx::migrate!`, applied on
  first pool use
- `src/db.rs` — shared `OnceLock` pool from `DATABASE_URL`
  (defaults to `sqlite:data/app.db?mode=rwc`), creating parent directories
- `src/todos.rs` — `Todo` / `NewTodo` types plus `create` / `get` / `list` /
  `delete` queries
- `src/todo_routes.rs` — `GET /todos`, `POST /todos`, `GET /todos/:id`,
  `DELETE /todos/:id` under the API prefix, with 404/422/500 mapping
- `src/lib.rs.append` — module declarations merged into the library
- `tests/test_sqlx.rs` — full CRUD flow against an in-memory database

## Configuration

| Variable       | Default                      | Description              |
| -------------- | ---------------------------- | ------------------------ |
| `DATABASE_URL` | `sqlite:data/app.db?mode=rwc`| SQLite connection string |

## Compatibility

Applies to `axum-backend` templates.
