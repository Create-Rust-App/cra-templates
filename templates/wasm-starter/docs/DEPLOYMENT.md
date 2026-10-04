# Deployment

## Browser package

```sh
wasm-pack build --target web --out-dir pkg
```

This writes `pkg/` (npm package: `.wasm`, JS glue, `.d.ts`). Never edit
`pkg/` by hand; it is gitignored build output.

## Publish to npm

1. Bump `version` in `Cargo.toml`.
2. Rebuild with `wasm-pack build --target web --out-dir pkg`.
3. `cd pkg && npm publish` (dry-run first with `--dry-run`).

## Publish the Rust crate (optional)

The manifest ships publish-ready metadata. For a native release:

```sh
cargo publish --dry-run
cargo publish
```

## Vite demo

The `www/` demo is not published — it consumes the local `pkg/` output.
Deploy it like any static Vite build (`npm run build`, serve `dist/`).
