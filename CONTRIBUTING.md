# Contributing

Thanks for contributing! This repo is the template and extension bank for
`create-rust-app`.

For a full explanation of how templates, extensions, and the file system work,
read [docs/AUTHORING.md](./docs/AUTHORING.md). English is canonical.

## Adding an extension

1. Create `extensions/<your-slug>/`
2. Add files to copy into the generated project (use `template/` if you only
   want a subset copied)
3. Register it in `templates.json` under `"extensions"`:

```json
{
  "name": "My Extension",
  "slug": "my-extension",
  "description": "Adds X to your project",
  "url": "https://github.com/Create-Rust-App/cra-templates?subdir=extensions/my-extension",
  "type": ["axum-backend"],
  "category": "ci",
  "labels": ["Tooling"]
}
```

Naming law: stack-bound extensions (single `type`) must use the
`<stack>-` prefix (`axum-docker` for `axum-backend`). Cross-cutting
extensions that apply to every template use the `all-` prefix
(`all-github-setup`).

## Adding a template

1. Create `templates/<your-slug>/` with a `Cargo.toml`, `src/main.rs`,
   `rust-toolchain.toml`, and the required docs (see below)
2. Add `cra.config.json` for interactive options when needed
3. Register it in `templates.json` under `"templates"` (same fields as extensions)

Required template files (enforced by `scripts/ci/validate-registry.py`):

- `Cargo.toml`, `src/main.rs`, `rust-toolchain.toml`
- `README.md`, `AGENTS.md`, `CONTRIBUTING.md`
- `docs/README.md`, `docs/PROJECT_STRUCTURE.md`, `docs/CONFIGURATION.md`,
  `docs/TESTING_GUIDE.md`, `docs/DEPLOYMENT.md`, `docs/API.md`
- `.env.example`, `tests/` with at least one `test_*.rs`

## Commit messages

Use [conventional commits](https://www.conventionalcommits.org/): `feat:`,
`fix:`, `docs:`, `chore:`, `refactor:`.

## PR checklist

- [ ] Directory name matches the `slug` in `templates.json`
- [ ] `url` points to the correct path (`?subdir=templates/<slug>` or
      `?subdir=extensions/<slug>`)
- [ ] `slug` is globally unique across templates and extensions
- [ ] All required fields present: `name`, `slug`, `description`, `url`,
      `type`, `category`, `labels`
- [ ] `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and
      `cargo test` pass inside the template directory
