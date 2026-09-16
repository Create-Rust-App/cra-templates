#!/usr/bin/env python3
"""Generate GitHub Actions matrices for layered CI."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from registry import (  # noqa: E402
    REPO_ROOT,
    as_types,
    assert_profile_valid,
    extension_dir,
    load_profiles,
    load_registry,
    template_dir,
)


def changed_paths(base_ref: str) -> list[str]:
    try:
        out = subprocess.check_output(
            ["git", "diff", "--name-only", f"{base_ref}...HEAD"],
            cwd=REPO_ROOT,
            text=True,
        )
        return [line.strip() for line in out.splitlines() if line.strip()]
    except subprocess.CalledProcessError:
        # Fail closed: treat as "everything changed" so PR CI does not go green
        # with an empty matrix when git history is shallow/broken.
        print(
            f"warning: git diff {base_ref}...HEAD failed — forcing full matrix",
            file=sys.stderr,
        )
        return ["templates.json"]


def force_full_matrix(changed: list[str]) -> bool:
    triggers = (
        "templates.json",
        "templates.schema.json",
        "scripts/ci/",
        "ci/profiles/",
        ".github/workflows/",
    )
    return any(path == trigger or path.startswith(trigger) for path in changed for trigger in triggers)


def touched_templates(changed: list[str]) -> set[str]:
    touched: set[str] = set()
    for path in changed:
        parts = Path(path).parts
        if len(parts) >= 2 and parts[0] == "templates":
            touched.add(parts[1])
    return touched


def touched_extensions(changed: list[str]) -> set[str]:
    touched: set[str] = set()
    for path in changed:
        parts = Path(path).parts
        if len(parts) >= 2 and parts[0] == "extensions":
            touched.add(parts[1])
    return touched


def cmd_templates(base_ref: str) -> int:
    registry = load_registry()
    changed = changed_paths(base_ref)
    if force_full_matrix(changed):
        cells = [
            {"id": t["slug"], "dir": f"templates/{template_dir(t)}"}
            for t in registry.get("templates", [])
        ]
    else:
        touched = touched_templates(changed)
        cells = [
            {"id": t["slug"], "dir": f"templates/{template_dir(t)}"}
            for t in registry.get("templates", [])
            if template_dir(t) in touched
        ]
    return emit_matrix(cells)


def cmd_extensions(base_ref: str) -> int:
    registry = load_registry()
    changed = changed_paths(base_ref)
    templates = {template_dir(t): t for t in registry.get("templates", [])}
    cells: list[dict[str, str]] = []
    full = force_full_matrix(changed)
    touched_t = touched_templates(changed)
    touched_e = touched_extensions(changed)
    for extension in registry.get("extensions", []):
        ext_dir = extension_dir(extension)
        if not (full or (ext_dir in touched_e)):
            # Also cover extensions when a compatible template changed.
            ext_types = set(as_types(extension.get("type")))
            impacted = any(
                templates.get(d, {}).get("slug") in touched_t or d in touched_t
                for d in templates
                if set(as_types(templates[d].get("type"))) & ext_types
            )
            if not impacted:
                continue
        for slug, template in templates.items():
            if set(as_types(template.get("type"))) & set(as_types(extension.get("type"))):
                cells.append(
                    {
                        "id": f"{slug}+{extension.get('slug')}",
                        "template_dir": f"templates/{slug}",
                        "extension_dir": f"extensions/{ext_dir}",
                    }
                )
    return emit_matrix(cells)


def cmd_profiles() -> int:
    registry = load_registry()
    cells = [
        {"id": name, "profile": f"ci/profiles/{name}.json"}
        for name, _ in load_profiles()
    ]
    errors = []
    for name, profile in load_profiles():
        errors.extend(assert_profile_valid(name, profile, registry))
    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1
    return emit_matrix(cells)


def cmd_validate_profiles() -> int:
    registry = load_registry()
    profiles = load_profiles()
    if not profiles:
        print("error: no CI profiles found in ci/profiles/", file=sys.stderr)
        return 1
    errors: list[str] = []
    for name, profile in profiles:
        errors.extend(assert_profile_valid(name, profile, registry))
    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1
    print(f"Profiles OK ({len(profiles)} profiles)")
    return 0


def emit_matrix(cells: list[dict[str, str]]) -> int:
    matrix = {"include": cells}
    output = os.environ.get("GITHUB_OUTPUT")
    payload = json.dumps(matrix)
    if output:
        with open(output, "a", encoding="utf-8") as handle:
            handle.write(f"matrix={payload}\n")
            handle.write(f"count={len(cells)}\n")
    else:
        print(payload)
    print(f"matrix cells: {len(cells)}", file=sys.stderr)
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--layer",
        required=True,
        choices=["templates", "extensions", "profiles", "validate-profiles"],
    )
    parser.add_argument("--base-ref", default="origin/main")
    args = parser.parse_args()
    if args.layer == "templates":
        return cmd_templates(args.base_ref)
    if args.layer == "extensions":
        return cmd_extensions(args.base_ref)
    if args.layer == "profiles":
        return cmd_profiles()
    return cmd_validate_profiles()


if __name__ == "__main__":
    raise SystemExit(main())
