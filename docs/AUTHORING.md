# Authoring — cra-templates

How templates, extensions, and the registry fit together. English is canonical.

## Directory layout

```text
templates.json            # registry: categories, templates, extensions
templates.schema.json     # registry schema
templates/<slug>/         # one scaffoldable starter per directory
extensions/<slug>/        # overlays merged onto a scaffolded template
extensions/<slug>/template/  # overlay root (only this subtree is copied)
ci/profiles/*.json        # curated template+extension combos for L3 CI
scripts/ci/               # validate-registry.py, generate-matrix.py, registry.py
```

## Registry entry

```json
{
  "name": "Axum Starter",
  "slug": "axum-starter",
  "description": "Production-ready Axum HTTP API",
  "url": "https://github.com/Create-Rust-App/cra-templates?subdir=templates/axum-starter",
  "type": "axum-backend",
  "category": "backend-applications",
  "labels": ["Rust", "Axum", "Tokio", "API"]
}
```

- Templates declare exactly one `type`; extensions list one or more.
- `url` uses the `?subdir=` form and its directory must equal `slug`.
- `category` must exist in `categories`; `type` must be a known template type.

## Templates

A template is a complete, buildable Rust crate: `Cargo.toml`, `src/main.rs`,
`rust-toolchain.toml` pinned to the bank toolchain, `cra.config.json` when
scaffold-time options exist, plus `README.md`, `AGENTS.md`, `CONTRIBUTING.md`,
`.env.example`, `tests/test_*.rs`, and `docs/` (README, PROJECT_STRUCTURE,
CONFIGURATION, TESTING_GUIDE, DEPLOYMENT, API).

`cra.config.json` declares interactive options:

```json
{
  "name": "axum-starter",
  "customOptions": [
    { "key": "apiPrefix", "type": "string", "message": "API URL prefix (e.g. /api/v1)", "default": "/api/v1" }
  ]
}
```

## Extensions

An extension is an overlay. When `extensions/<slug>/template/` exists, only
that subtree is copied into the scaffolded project; later layers override
earlier ones. Keep author docs (`README.md`) at the extension root so they
are never merged into user projects.

Naming law (enforced by `validate-registry.py`):

- Stack-bound extensions (single `type`) use the `<stack>-` prefix:
  `axum-docker` for `axum-backend`.
- Cross-cutting extensions use the `all-` prefix and must cover every
  template type: `all-github-setup`.

The engine (`create-rust-app` 0.3.0+) additionally honors two overlay
merge semantics when copying `template/` content:

- `<name>.append` fragments append to the same-named project file
  (`router.rs.append` extends `router.rs`; a missing target is created).
  Fragments apply after every plain copy in the same overlay pass, so file
  order is deterministic. Use fragments to register routers, declare
  modules, add env keys, and index docs without forking template files.
- An overlay `Cargo.toml` merges `[dependencies]`, `[dev-dependencies]`,
  and `[build-dependencies]` into the project manifest. Missing entries are
  added and identical entries skipped; a conflicting requirement for the
  same dependency fails the scaffold naming both specs. Every other overlay
  section is ignored by contract — extension manifests must carry only
  dependency tables, never `[package]` or targets.
