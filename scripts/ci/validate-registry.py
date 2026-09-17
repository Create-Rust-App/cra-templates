#!/usr/bin/env python3
"""L0 integrity: registry paths, naming law, and template quality bar."""

from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from registry import (  # noqa: E402
    ALL_TEMPLATE_TYPES,
    CANONICAL_TEMPLATE_BY_TYPE,
    REPO_ROOT,
    SCHEMA_JSON,
    TEMPLATES_JSON,
    as_types,
    extension_dir,
    load_registry,
    template_dir,
)

try:
    import jsonschema
except ImportError:
    jsonschema = None  # type: ignore

# Folder prefix required for stack-bound extensions (single type).
STACK_PREFIX_BY_TYPE: dict[str, str] = {
    "axum-backend": "axum",
    "cli": "cli",
    "leptos-fullstack": "leptos",
    "tonic-grpc": "tonic",
}

REQUIRED_TEMPLATE_FILES = (
    "Cargo.toml",
    "src/main.rs",
    "rust-toolchain.toml",
    "cra.config.json",
    "README.md",
    "AGENTS.md",
    "CONTRIBUTING.md",
    ".env.example",
)

REQUIRED_TEMPLATE_DOCS = (
    "docs/README.md",
    "docs/PROJECT_STRUCTURE.md",
    "docs/CONFIGURATION.md",
    "docs/TESTING_GUIDE.md",
    "docs/DEPLOYMENT.md",
    "docs/API.md",
)


def validate_schema() -> list[str]:
    errors: list[str] = []
    if jsonschema is None:
        print("WARN: jsonschema not installed — skipping schema validation")
        return errors
    schema = json.loads(SCHEMA_JSON.read_text(encoding="utf-8"))
    data = json.loads(TEMPLATES_JSON.read_text(encoding="utf-8"))
    validator = jsonschema.Draft7Validator(schema)
    for err in sorted(validator.iter_errors(data), key=lambda e: list(e.path)):
        path = ".".join(str(p) for p in err.path) or "(root)"
        errors.append(f"schema: {path}: {err.message}")
    return errors


def _has_tests(path: Path) -> bool:
    tests = path / "tests"
    if tests.is_dir():
        return any(tests.glob("test_*.rs"))
    return False


def validate_extension_folder_name(directory: str, types: list[str], slug: str) -> list[str]:
    errors: list[str] = []
    if len(types) == 1 and types[0] in STACK_PREFIX_BY_TYPE:
        prefix = STACK_PREFIX_BY_TYPE[types[0]] + "-"
        if directory.startswith("all-"):
            return errors  # cross-cutting alias, checked against ALL types below
        if not directory.startswith(prefix):
            errors.append(
                f"extension {slug}: folder `{directory}` must start with `{prefix}` "
                f"for type `{types[0]}`"
            )
    return errors


def main() -> int:
    registry = load_registry()
    errors: list[str] = validate_schema()

    categories = {c.get("slug") for c in registry.get("categories", [])}
    seen_slugs: set[str] = set()

    for kind, entries in (("template", registry.get("templates", [])),
                          ("extension", registry.get("extensions", []))):
        for entry in entries:
            slug = entry.get("slug", "")
            name = f"{kind} {slug or entry.get('name', '?')}"
            if slug in seen_slugs:
                errors.append(f"{name}: slug `{slug}` is not globally unique")
            seen_slugs.add(slug)

            for field in ("name", "slug", "description", "url", "type", "category", "labels"):
                if not entry.get(field):
                    errors.append(f"{name}: missing required field `{field}`")

            directory = template_dir(entry) if kind == "template" else extension_dir(entry)
            if not directory:
                errors.append(f"{name}: cannot resolve directory from url `{entry.get('url')}`")
                continue
            if directory != slug:
                errors.append(
                    f"{name}: directory `{directory}` must match slug `{slug}`"
                )
            on_disk = REPO_ROOT / ("templates" if kind == "template" else "extensions") / directory
            if not on_disk.is_dir():
                errors.append(f"{name}: on-disk directory `{on_disk}` missing")
                continue

            if entry.get("category") not in categories:
                errors.append(f"{name}: unknown category `{entry.get('category')}`")

            types = as_types(entry.get("type"))
            for type_name in types:
                if type_name not in ALL_TEMPLATE_TYPES:
                    errors.append(f"{name}: unknown type `{type_name}`")

            if kind == "template":
                if len(types) != 1:
                    errors.append(f"{name}: template must declare exactly one `type`")
                for required in REQUIRED_TEMPLATE_FILES + REQUIRED_TEMPLATE_DOCS:
                    if not (on_disk / required).is_file():
                        errors.append(f"{name}: missing required file `{required}`")
                if not _has_tests(on_disk):
                    errors.append(f"{name}: missing `tests/test_*.rs`")
            else:
                errors.extend(validate_extension_folder_name(directory, types, slug))
                if not (on_disk / "README.md").is_file():
                    errors.append(f"{name}: missing required file `README.md`")
                template_overlay = on_disk / "template"
                if template_overlay.is_dir() and not any(template_overlay.rglob("*")):
                    errors.append(f"{name}: empty `template/` overlay")
                if directory.startswith("all-") and set(types) != set(ALL_TEMPLATE_TYPES):
                    errors.append(
                        f"{name}: `all-` extension must cover all template types"
                    )
                for incompatible in entry.get("incompatibleWith", []):
                    if incompatible not in seen_slugs and incompatible not in [
                        e.get("slug") for e in registry.get("extensions", [])
                    ]:
                        errors.append(
                            f"{name}: unknown incompatibleWith `{incompatible}`"
                        )

    for type_name, canonical in CANONICAL_TEMPLATE_BY_TYPE.items():
        if not (REPO_ROOT / "templates" / canonical).is_dir():
            errors.append(
                f"canonical template for type `{type_name}` missing: `templates/{canonical}`"
            )

    if errors:
        print(f"{len(errors)} registry error(s):")
        for error in errors:
            print(f"  - {error}")
        return 1
    print("Registry OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
