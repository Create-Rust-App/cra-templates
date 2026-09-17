# Testing guide

- Unit tests live next to the code (shell, config, fragments).
- `tests/test_pages.rs` drives the router end to end with `tower::oneshot`:
  page HTML, JSON health, fragment increments, and the 404 fallback.
- Rendering in tests goes through `leptos::ssr::render_to_string`, mirroring
  the route handlers.
- Run everything with `cargo test`.
