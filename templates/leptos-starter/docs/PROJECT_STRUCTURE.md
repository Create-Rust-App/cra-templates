# Project structure

```text
src/
  main.rs                 # tracing setup + serve loop
  lib.rs                  # module root (api, app, components, config, pages, site)
  app.rs                  # shell(): full HTML document wrapper
  site.rs                 # create_app(): Axum routes + extension hook
  config.rs               # Config::from_env with defaults
  components/
    mod.rs                # widget registry
    counter.rs            # htmx counter widget
  pages/
    mod.rs                # page registry
    home.rs               # home page
    not_found.rs          # 404 page
  api/
    mod.rs                # endpoint registry
    health.rs             # GET /api/healthz
    count.rs              # POST /api/count fragment endpoint
tests/
  test_pages.rs           # end-to-end page and endpoint tests
```
