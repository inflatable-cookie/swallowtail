#!/usr/bin/env python3
"""Fail when task indexes disagree with Status frontmatter.

Task model (g05.038): ``docs/roadmaps/gNN/NNN-<slug>.md`` files are the sole
executable planning unit. The generation README lists each task once under
``### Planned`` / ``### Ready`` / ``### Blocked`` / ``### Stopped`` /
``### Completed`` beneath ``## Tasks``. There is no nested card level: any
``batch-cards/`` directory or link, ``## Batch Cards`` section, ``Milestone:``
pointer, ``execute card`` verb, card-budget table, allowed-runway table, or
``First milestone`` / ``Next milestone`` column in a current planning surface
is a migration defect.
"""

from __future__ import annotations

import argparse
import re
from collections import defaultdict
from pathlib import Path

SCRIPT_ROOT = Path(__file__).resolve().parent.parent
ROOT = SCRIPT_ROOT
GENERATION_INDEX = ROOT / "docs/roadmaps/generation-index.md"

# Longer aliases first so ``evidence stop`` wins over a later ``stopped``.
RECOGNISED_TOKENS = (
    "evidence stop",
    "identity stop",
    "stopped",
    "completed",
    "complete",
    "blocked",
    "planned",
    "ready",
    "done",
)
STATUS_TOKENS = {
    "planned",
    "ready",
    "blocked",
    "stopped",
    "complete",
    "completed",
    "done",
}


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
        description="Reject roadmap task Status drift and nested dispatch."
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=SCRIPT_ROOT,
        help="repository root to scan (hermetic tests; default: this checkout)",
    )
    return parser.parse_args(argv)


STATUS_RE = re.compile(r"^Status:\s*(?P<raw>.+)$", re.MULTILINE)
LINK_RE = re.compile(
    r"^- \[.*?\]\(\.?/?(?P<file>\d{3}-[^)\s]+\.md)\)(?:\s*—\s*(?P<ann>.*))?$",
    re.MULTILINE,
)
SECTION_RE = re.compile(
    r"^#{2,3} (?P<title>Planned|Ready|Blocked|Stopped|Completed)\s*$",
    re.MULTILINE,
)
TASK_READY_PROSE_RE = re.compile(
    r"tasks?\s+(?P<ids>(?:\d{3}(?:\s*[-–,]\s*\d{3})*)+)\s+(?:is|are)\s+ready",
    re.IGNORECASE,
)
STOPPED_LIST_RE = re.compile(
    r"honest evidence\s+stops at\s+(?P<ids>[\d,\s]+(?:and\s+\d+)?)",
    re.IGNORECASE,
)
COMPLETED_COUNT_RE = re.compile(r"(?P<count>\d+)\s+completed tasks?", re.IGNORECASE)
READY_TASK_RE = re.compile(
    r"(?:one ready task at|ready\s+tasks?\s+at)\s+(?P<ids>\d{3}(?:\s*,\s*\d{3})*(?:\s*,?\s*and\s+\d{3})?)",
    re.IGNORECASE,
)
PLANNED_TASK_RE = re.compile(
    r"(?:one planned task at|planned\s+tasks?\s+at)\s+(?P<ids>\d{3}(?:\s*,\s*\d{3})*(?:\s*,?\s*and\s+\d{3})?)",
    re.IGNORECASE,
)
NO_PLANNED_RE = re.compile(r"\bno planned tasks\b", re.IGNORECASE)

SECTION_BUCKET = {
    "Planned": "planned",
    "Ready": "ready",
    "Blocked": "blocked",
    "Stopped": "stopped",
    "Completed": "complete",
}
ANNOTATION_ALLOWED = {
    "planned": {"planned"},
    "ready": {"ready"},
    "blocked": {"blocked"},
    "complete": {"complete", "completed", "done", "evidence stop", "identity stop"},
    "stopped": {"stopped"},
}

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
# migration task itself (it specifies the removal) and the grammar that
# defines the rejection rule.
LEGACY_SCAN_EXEMPT = frozenset(
    {
        "docs/roadmaps/g05/038-flattened-task-switchover.md",
        "docs/roadmaps/status-grammar.md",
    }
)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing {path.relative_to(ROOT)}", at=(path, 1))
    return path.read_text(encoding="utf-8")


def first_recognised_token(raw: str) -> str | None:
    primary = re.split(r"[;\n]", raw, maxsplit=1)[0].strip().lower()
    for candidate in RECOGNISED_TOKENS:
        if re.match(rf"{re.escape(candidate)}\b", primary):
            return candidate
    return None


def status_bucket(raw: str) -> str | None:
    token = first_recognised_token(raw)
    if token is None or token not in STATUS_TOKENS:
        return None
    if token in {"complete", "completed", "done"}:
        return "complete"
    if token == "stopped":
        return "stopped"
    return token


def frontmatter_status(path: Path) -> tuple[str, int]:
    document = read(path)
    match = STATUS_RE.search(document)
    if match is None:
        fail("has no Status line", at=(path, 1))
    line = line_at(document, match.start())
    bucket = status_bucket(match.group("raw"))
    if bucket is None:
        fail(f"unrecognized Status {match.group('raw')!r}", at=(path, line))
    return bucket, line


def annotation_primary(annotation: str | None) -> str | None:
    if annotation is None:
        return None
    token = first_recognised_token(annotation)
    if token is None:
        return None
    return token


def parse_id_list(text: str, *, at: tuple[Path, int]) -> set[str]:
    ids: set[str] = set()
    for chunk in re.split(r",|\band\b", text):
        chunk = chunk.strip()
        if not chunk:
            continue
        range_match = re.fullmatch(r"(\d{3})\s*[-–]\s*(\d{3})", chunk)
        if range_match:
            start = int(range_match.group(1))
            end = int(range_match.group(2))
            if end < start:
                fail(f"inverted id range {chunk!r}", at=at)
            ids.update(f"{value:03d}" for value in range(start, end + 1))
            continue
        single = re.fullmatch(r"\d{3}", chunk)
        if single:
            ids.add(chunk)
            continue
        fail(f"unparseable id list fragment {chunk!r}", at=at)
    return ids


def check_tasks() -> None:
    document = read(TASK_INDEX)
    sections = list(SECTION_RE.finditer(document))
    if not sections:
        fail(
            "task index has no Planned/Ready/Blocked/Stopped/Completed sections",
            at=(TASK_INDEX, 1),
        )

    indexed: dict[str, list[tuple[str, str | None, int]]] = defaultdict(list)
    for index, match in enumerate(sections):
        start = match.end()
        end = sections[index + 1].start() if index + 1 < len(sections) else len(document)
        body = document[start:end]
        section_name = match.group("title")
        bucket = SECTION_BUCKET[section_name]
        for link in LINK_RE.finditer(body):
            line = line_at(document, start + link.start())
            indexed[link.group("file")].append((bucket, link.group("ann"), line))

    task_files = sorted(
        path for path in TASK_DIR.glob("*.md") if path.name != "README.md"
    )
    for path in task_files:
        expected, status_line = frontmatter_status(path)
        entries = indexed.get(path.name, [])
        if not entries:
            fail(
                f"task {path.name} is not indexed in {TASK_INDEX.relative_to(ROOT)}",
                at=(path, status_line),
            )
        if len(entries) > 1:
            places = ", ".join(f"{bucket} at line {line}" for bucket, _, line in entries)
            fail(
                f"task {path.name} is indexed more than once ({places})",
                at=(TASK_INDEX, entries[0][2]),
            )
        section_bucket, annotation, index_line = entries[0]
        if section_bucket != expected:
            fail(
                f"task {path.name} Status bucket is {expected!r} but index lists it under {section_bucket!r}",
                at=(TASK_INDEX, index_line),
            )
        primary = annotation_primary(annotation)
        if primary is not None:
            allowed = ANNOTATION_ALLOWED[expected]
            if primary not in allowed:
                fail(
                    f"task {path.name} annotation primary {primary!r} does not match Status bucket {expected!r}",
                    at=(TASK_INDEX, index_line),
                )

    for name, entries in sorted(indexed.items()):
        if not (TASK_DIR / name).is_file():
            fail(
                f"task index links missing file {name}",
                at=(TASK_INDEX, entries[0][2]),
            )


def check_task_annotations() -> None:
    document = read(TASK_INDEX)
    task_files = {
        path.name: path
        for path in TASK_DIR.glob("*.md")
        if path.name != "README.md" and re.match(r"^\d{3}-", path.name)
    }
    for link in LINK_RE.finditer(document):
        name = link.group("file")
        path = task_files.get(name)
        if path is None:
            continue
        expected, _status_line = frontmatter_status(path)
        primary = annotation_primary(link.group("ann"))
        if primary is None:
            continue
        allowed = ANNOTATION_ALLOWED[expected]
        if primary not in allowed:
            fail(
                f"task {name} annotation primary {primary!r} does not match Status bucket {expected!r}",
                at=(TASK_INDEX, line_at(document, link.start())),
            )


def active_generation_census(document: str) -> tuple[str, int]:
    match = re.search(
        rf"^{re.escape(ACTIVE_GENERATION)} (?:now )?has .+?(?=^{re.escape(ACTIVE_GENERATION)}\.|^## |\Z)",
        document,
        re.MULTILINE | re.DOTALL,
    )
    if match is None:
        fail(
            f"generation-index is missing the active {ACTIVE_GENERATION} census paragraph",
            at=(GENERATION_INDEX, 1),
        )
    return match.group(0), line_at(document, match.start())


def check_generation_index() -> None:
    document = read(GENERATION_INDEX)
    buckets: dict[str, set[str]] = defaultdict(set)
    for path in TASK_DIR.glob("*.md"):
        if path.name == "README.md" or not re.match(r"^\d{3}-", path.name):
            continue
        number = path.name[:3]
        bucket, _status_line = frontmatter_status(path)
        buckets[bucket].add(number)

    for match in TASK_READY_PROSE_RE.finditer(document):
        line = line_at(document, match.start())
        for number in parse_id_list(match.group("ids"), at=(GENERATION_INDEX, line)):
            path = next(TASK_DIR.glob(f"{number}-*.md"), None)
            if path is None:
                fail(
                    f"generation-index claims task {number} is ready but the task file is missing",
                    at=(GENERATION_INDEX, line),
                )
            actual, _status_line = frontmatter_status(path)
            if actual != "ready":
                fail(
                    f"generation-index claims task {number} is ready but Status bucket is {actual!r}",
                    at=(GENERATION_INDEX, line),
                )

    census, census_line = active_generation_census(document)
    ready_claimed: set[str] = set()
    for match in READY_TASK_RE.finditer(census):
        ready_line = census_line + line_at(census, match.start()) - 1
        ready_claimed.update(parse_id_list(match.group("ids"), at=(GENERATION_INDEX, ready_line)))
    if ready_claimed != buckets["ready"]:
        fail(
            "generation-index ready task set "
            f"{sorted(ready_claimed)} disagrees with frontmatter {sorted(buckets['ready'])}",
            at=(GENERATION_INDEX, census_line),
        )

    planned_claimed: set[str] = set()
    for match in PLANNED_TASK_RE.finditer(census):
        planned_line = census_line + line_at(census, match.start()) - 1
        planned_claimed.update(parse_id_list(match.group("ids"), at=(GENERATION_INDEX, planned_line)))
    if not PLANNED_TASK_RE.search(census) and not NO_PLANNED_RE.search(census):
        fail(
            f"generation-index {ACTIVE_GENERATION} census omits planned task disposition",
            at=(GENERATION_INDEX, census_line),
        )
    if planned_claimed != buckets["planned"]:
        fail(
            "generation-index planned task set "
            f"{sorted(planned_claimed)} disagrees with frontmatter {sorted(buckets['planned'])}",
            at=(GENERATION_INDEX, census_line),
        )

    completed_match = COMPLETED_COUNT_RE.search(census)
    if completed_match is None:
        fail(
            f"generation-index {ACTIVE_GENERATION} census omits completed task count",
            at=(GENERATION_INDEX, census_line),
        )
    claimed = int(completed_match.group("count"))
    actual = len(buckets["complete"])
    if claimed != actual:
        fail(
            f"generation-index claims {claimed} completed tasks but frontmatter has {actual}",
            at=(
                GENERATION_INDEX,
                census_line + line_at(census, completed_match.start()) - 1,
            ),
        )

    stopped_match = STOPPED_LIST_RE.search(census)
    if stopped_match is None:
        if not re.search(r"\bno honest evidence stops\b", census, re.IGNORECASE):
            fail(
                f"generation-index {ACTIVE_GENERATION} census omits honest evidence stop disposition",
                at=(GENERATION_INDEX, census_line),
            )
        claimed_ids: set[str] = set()
        stopped_line = census_line
    else:
        stopped_line = census_line + line_at(census, stopped_match.start()) - 1
        claimed_ids = parse_id_list(stopped_match.group("ids"), at=(GENERATION_INDEX, stopped_line))
    actual_stopped = buckets["stopped"]
    if claimed_ids != actual_stopped:
        fail(
            "generation-index honest evidence stops "
            f"{sorted(claimed_ids)} disagree with frontmatter {sorted(actual_stopped)}",
            at=(GENERATION_INDEX, stopped_line),
        )


def legacy_scan_roots() -> list[Path]:
    roots = [x for x in (ROOT / "docs" / "roadmaps").rglob("*.md")]
    roots.append(ROOT / "docs" / "contracts" / "001-working-rules.md")
    roots.append(ROOT / "AGENTS.md")
    return sorted(set(roots))


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
    check_task_annotations()
    check_generation_index()
    check_no_legacy_dispatch()
    print("roadmap status drift check passed")


if __name__ == "__main__":
    main()
