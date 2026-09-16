# AGENTS.md

This repo is the template and extension bank for
[`create-rust-app`](https://github.com/Create-Rust-App/create-rust-app).

## Key concepts

- **`templates.json`** — single registry of all templates, extensions, and
  categories. Every entry needs `name`, `slug`, `description`, `url`, `type`,
  `category`, and `labels`. Slugs are globally unique across templates and
  extensions.
- **`type`** — links extensions to templates. A template declares exactly one
  type (e.g. `axum-backend`); an extension lists one or more template types in
  `type`. The `all-` prefix convention marks cross-cutting extensions whose
  `type` covers every template type.
- **`Cargo.toml`** — lives in the template root and defines the scaffolded
  crate. Templates must build with the pinned toolchain in
  `rust-toolchain.toml` (currently 1.82.0).
- **`template/` subdirectory** — for extensions, the overlay root. When
  present, the engine copies from `extensions/<slug>/template/` instead of the
  extension root. Keep `README.md` at `extensions/<slug>/README.md` (not inside
  `template/`) so author docs are not merged into user projects.
- **`.template` and `.append` semantics** — the engine supports `.template`
  (rendered per file) and `.append` (append to existing target) with copy-only
  merge where later layers override earlier ones.
- **`file://` catalog URL** — for local testing:
  `file://$PWD?subdir=templates/axum-starter` and
  `file://$PWD?subdir=extensions/all-github-setup`. The `?subdir=` segment
  must match the on-disk `templates/<slug>` or `extensions/<slug>` directory.

## How to test

```sh
# Validate registry (L0)
python3 scripts/ci/validate-registry.py

# Validate curated CI profiles
python3 scripts/ci/generate-matrix.py --layer validate-profiles

# Template quality gates (run inside each template dir)
cd templates/axum-starter
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

See [docs/TESTING.md](./docs/TESTING.md) for more examples and CI details.

## Docs

| File | Contents |
|---|---|
| [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) | System overview, template vs extension, merge model |
| [docs/AUTHORING.md](./docs/AUTHORING.md) | Directory layout, `Cargo.toml`, extensions, `type` |
| [docs/TESTING.md](./docs/TESTING.md) | Local testing commands and CI layers |
| [CONTRIBUTING.md](./CONTRIBUTING.md) | How to add templates and extensions |
