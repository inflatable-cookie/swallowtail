#!/usr/bin/env python3
"""Fail when the flattened task index drifts from the task files.

Task model (g05.038): ``docs/roadmaps/gNN/NNN-<slug>.md`` files are the sole
executable planning unit. Since g05.057, terminal task state is
lifecycle-owned in ``.northstar/lifecycle/v1/``: task files carry no
hand-maintained ``Status:`` line, the generation README's ``## Tasks``
section is a flat link registry with no status buckets, and the generation
index carries no hand-maintained status census. The retired status-bucket
model is a migration defect, like any nested card level.

Remaining drift checks: exactly one active generation is declared, every
task file is indexed exactly once under ``## Tasks``, every indexed link
resolves to a task file, no status buckets return under ``## Tasks``, and no
nested dispatch level (``batch-cards/``) is reintroduced.
"""

from __future__ import annotations

import argparse
import re
from collections import defaultdict
from pathlib import Path

SCRIPT_ROOT = Path(__file__).resolve().parent.parent
ROOT = SCRIPT_ROOT
GENERATION_INDEX = ROOT / "docs/roadmaps/generation-index.md"

LINK_RE = re.compile(
    r"^- \[.*?\]\(\.?/?(?P<file>\d{3}-[^)\s]+\.md)\)(?:\s*—\s*(?P<ann>.*))?$",
    re.MULTILINE,
)
TASKS_HEADING_RE = re.compile(r"^## Tasks\s*$", re.MULTILINE)
ANY_H2_RE = re.compile(r"^## .+$", re.MULTILINE)
# Status buckets under ``## Tasks`` duplicated lifecycle-owned state; they
# are rejected outright rather than validated.
RETIRED_BUCKET_RE = re.compile(
    r"^#{3,}\s+(?:Planned|Ready|Blocked|Stopped|Completed)\s*$",
    re.MULTILINE,
)


def line_at(document: str, offset: int) -> int:
    return document.count("\n", 0, offset) + 1


def fail(message: str, *, at: tuple[Path, int]) -> None:
    rel = at[0].relative_to(ROOT) if at[0].is_absolute() else at[0]
    print(f"roadmap status drift check failed: {message} ({rel}:{at[1]})")
    raise SystemExit(1)


def active_generation_id() -> str:
    document = GENERATION_INDEX.read_text(encoding="utf-8")
    matches = re.findall(
        r"^\| `(g\d{2})` \| active \|", document, re.MULTILINE
    )
    if len(matches) != 1:
        fail(
            "generation-index must name exactly one active generation",
            at=(GENERATION_INDEX, 1),
        )
    return matches[0]


ACTIVE_GENERATION = ""
TASK_DIR = ROOT
TASK_INDEX = ROOT


def bind_paths(root: Path) -> None:
    global ROOT, GENERATION_INDEX, ACTIVE_GENERATION
    global TASK_DIR, TASK_INDEX
    ROOT = root.resolve()
    GENERATION_INDEX = ROOT / "docs/roadmaps/generation-index.md"
    ACTIVE_GENERATION = active_generation_id()
    TASK_DIR = ROOT / f"docs/roadmaps/{ACTIVE_GENERATION}"
    TASK_INDEX = TASK_DIR / "README.md"
    legacy = TASK_DIR / "batch-cards"
    if legacy.is_dir():
        fail(
            f"legacy nested dispatch level remains: {legacy.relative_to(ROOT)}",
            at=(TASK_INDEX, 1),
        )


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Reject flattened task-index drift and nested dispatch."
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=SCRIPT_ROOT,
        help="repository root to scan (hermetic tests; default: this checkout)",
    )
    return parser.parse_args(argv)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing {path.relative_to(ROOT)}", at=(path, 1))
    return path.read_text(encoding="utf-8")


def tasks_section(document: str) -> tuple[str, int]:
    heading = TASKS_HEADING_RE.search(document)
    if heading is None:
        fail("task index has no `## Tasks` section", at=(TASK_INDEX, 1))
    start = heading.end()
    next_h2 = ANY_H2_RE.search(document, start)
    end = next_h2.start() if next_h2 else len(document)
    return document[start:end], line_at(document, heading.start())


def check_tasks() -> None:
    document = read(TASK_INDEX)
    section, section_line = tasks_section(document)

    bucket = RETIRED_BUCKET_RE.search(section)
    if bucket is not None:
        fail(
            "retired status bucket under `## Tasks`; task state is lifecycle-owned",
            at=(TASK_INDEX, section_line + line_at(section, bucket.start()) - 1),
        )

    indexed: dict[str, list[int]] = defaultdict(list)
    for link in LINK_RE.finditer(section):
        indexed[link.group("file")].append(line_at(section, link.start()))

    for name, lines in sorted(indexed.items()):
        if len(lines) > 1:
            places = ", ".join(str(line) for line in lines)
            fail(
                f"task {name} is indexed more than once (lines {places})",
                at=(TASK_INDEX, section_line + lines[0] - 1),
            )
        if not (TASK_DIR / name).is_file():
            fail(
                f"task index links missing file {name}",
                at=(TASK_INDEX, section_line + lines[0] - 1),
            )

    for path in sorted(TASK_DIR.glob("*.md")):
        if path.name == "README.md":
            continue
        if path.name not in indexed:
            fail(
                f"task {path.name} is not indexed under `## Tasks`",
                at=(path, 1),
            )


def legacy_scan_roots() -> list[Path]:
    roots = [x for x in (ROOT / "docs" / "roadmaps").rglob("*.md")]
    roots.append(ROOT / "docs" / "contracts" / "001-working-rules.md")
    roots.append(ROOT / "AGENTS.md")
    return sorted(set(roots))


# Nested-dispatch structures rejected in current planning surfaces.
LEGACY_LINK_RE = re.compile(r"\]\([^)]*batch-cards/")
LEGACY_PATTERNS = (
    re.compile(r"^## Batch Cards\s*$", re.MULTILINE),
    re.compile(r"^Milestone:\s*`", re.MULTILINE),
    re.compile(r"execute card", re.IGNORECASE),
    re.compile(r"Remaining card budget"),
    re.compile(r"Allowed runway"),
    re.compile(r"[Ff]irst milestone"),
    re.compile(r"[Nn]ext milestone"),
    re.compile(r"Ready cards, in order"),
    re.compile(r"^\| Card \|", re.MULTILINE),
)
# Surfaces that may name retired structures without dispatching them: the
# grammar that defines the rejection rule. Compacted migration tasks live in
# docs/roadmaps/archive/, which the scan skips wholesale.
LEGACY_SCAN_EXEMPT = frozenset(
    {
        "docs/roadmaps/status-grammar.md",
    }
)


def check_no_legacy_dispatch() -> None:
    for path in legacy_scan_roots():
        if not path.is_file():
            continue
        try:
            relative = path.relative_to(ROOT).as_posix()
        except ValueError:
            continue
        if relative in LEGACY_SCAN_EXEMPT:
            continue
        if "/archive/" in relative:
            continue
        document = path.read_text(encoding="utf-8")
        if LEGACY_LINK_RE.search(document):
            fail("nested batch-cards/ link remains", at=(path, 1))
        for pattern in LEGACY_PATTERNS:
            match = pattern.search(document)
            if match is not None:
                fail(
                    f"legacy dispatch structure remains: {match.group(0).strip()!r}",
                    at=(path, line_at(document, match.start())),
                )


def main(argv: list[str] | None = None) -> None:
    args = parse_args(argv)
    bind_paths(args.root)
    check_tasks()
    check_no_legacy_dispatch()
    print("roadmap status drift check passed")


if __name__ == "__main__":
    main()
