# Architecture — cra-templates

## System overview

`cra-templates` is the template and extension bank for `create-rust-app`.
The CLI reads `templates.json`, scaffolds a base template directory, then
merges selected extension overlays on top.

## Template vs extension

- A **template** is a complete crate that builds and runs on its own
  (`axum-starter`).
- An **extension** is a partial file tree merged over the scaffold
  (`all-github-setup` adds CI files; `axum-docker` adds container files).

## Merge model

Copy-only merge: later layers override earlier ones file by file. Two file
suffixes change the behavior:

- `.template` — rendered per file with scaffold-time options (e.g. values
  from `cra.config.json`).
- `.append` — content is appended to the existing target file instead of
  replacing it (e.g. `docs/README.md.append`).

`compatibleWith`/`incompatibleWith` on registry entries constrain which
extensions can combine; `type` links extensions to the template types they
apply to.

## CI layers

| Layer | Workflow | Purpose |
|-------|----------|---------|
| L0 | `ci-integrity.yml` | Registry schema + on-disk paths, profile validity |
| L1 | `ci-templates.yml` | Per-template `cargo fmt`, `clippy`, `test` |
| L2 | `ci-extensions.yml` | Extension overlay integrity per template+extension cell |
| L3 | `ci-profiles.yml` | Curated `ci/profiles/*.json` combos with profile env |
