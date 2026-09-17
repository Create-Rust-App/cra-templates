# Project Structure — axum-starter

```text
.
├── Cargo.toml            # crate manifest (axum, tokio, serde, tower-http, tracing)
├── rust-toolchain.toml   # pinned toolchain (1.88.0)
├── cra.config.json       # scaffold-time options (apiPrefix)
├── .env.example          # documented runtime defaults
├── src/
│   ├── main.rs           # tracing setup + serve loop
│   ├── lib.rs            # module declarations
│   ├── config.rs         # Config::from_env with defaults
│   ├── app.rs            # create_app: routes, state, prefix nesting
│   └── routes/
│       ├── mod.rs            # feature module registry
│       ├── health.rs         # GET {prefix}/healthz
│       └── _feature_template.rs  # copy to start a new feature
├── tests/
│   └── test_health.rs    # integration tests via tower oneshot
└── docs/                 # API, configuration, structure, testing, deployment
```
