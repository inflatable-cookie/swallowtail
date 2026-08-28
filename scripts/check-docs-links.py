#!/usr/bin/env python3
"""Check front-door Markdown links plus research and lane-log bodies.

Keeps `qa:docs:links` bounded: root/release front doors plus the indexed
research and logs corpora. Does not walk roadmap child indexes or the rest of
`docs/`.
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent

FRONT_DOOR = [
    "README.md",
    "AGENTS.md",
    "CHANGELOG.md",
    "CONTRIBUTING.md",
    "SECURITY.md",
    "SUPPORT.md",
    "docs/README.md",
    "docs/releases/README.md",
    "docs/releases/0.1.0.md",
    "docs/releases/0.1.1.md",
    "docs/releases/0.2.0.md",
    "docs/releases/0.3.0.md",
    "docs/releases/0.3.1.md",
    "docs/releases/0.3.2.md",
    "docs/releases/0.3.3.md",
]

CORPUS_DIRS = (
    ROOT / "docs/research",
    ROOT / "docs/logs",
)


def fail(message: str) -> None:
    print(f"docs link check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def corpus_markdown() -> list[str]:
    files: list[Path] = []
    for directory in CORPUS_DIRS:
        if not directory.is_dir():
            fail(f"missing corpus directory {directory.relative_to(ROOT)}")
        files.extend(path for path in directory.rglob("*.md") if path.is_file())
    return sorted(path.relative_to(ROOT).as_posix() for path in files)


def main() -> None:
    ordered: list[str] = []
    seen: set[str] = set()
    for relative in [*FRONT_DOOR, *corpus_markdown()]:
        if relative in seen:
            continue
        path = ROOT / relative
        if not path.is_file():
            fail(f"missing {relative}")
        seen.add(relative)
        ordered.append(relative)

    front_door_count = sum(1 for path in FRONT_DOOR if path in seen)
    corpus_count = len(ordered) - front_door_count
    print(
        f"docs link check: {front_door_count} front-door + "
        f"{corpus_count} research/log Markdown files"
    )
    proc = subprocess.run(
        ["effigy", "docs", "check", "links", *ordered],
        cwd=ROOT,
        check=False,
    )
    raise SystemExit(proc.returncode)


if __name__ == "__main__":
    main()
