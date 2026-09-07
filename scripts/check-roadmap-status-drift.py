#!/usr/bin/env python3
"""Fail when roadmap/batch-card indexes disagree with Status frontmatter.

Grammar (this file is authority for the live parse; docs/roadmaps/status-grammar.md
is the human copy):

- First recognised token wins. A Status line or index annotation is split on
  the first ``;`` or newline. Only the first field is parsed. Later fields are
  free-form detail and cannot change the bucket, even when they contain words
  such as ``stopped`` or ``blocked``.
- Recognised Status tokens, matched at the start of that first field:
  ``planned``, ``ready``, ``blocked``, ``stopped``, ``complete``,
  ``completed``, ``done``. Those collapse to buckets planned, ready, blocked,
  stopped, and complete.
- Index annotations accept the same tokens plus complete aliases
  ``evidence stop`` and ``identity stop``, still only as the first field.
- Batch-card indexes are matched as markdown list entries
  ``- [title](./NNN-file.md)`` (optional ``./``) under ``## Planned``,
  ``## Ready``, ``## Blocked``, ``## Stopped``, or ``## Completed``. One
  entry per card. The section heading is the index bucket; the optional
  ``—`` annotation primary must belong to that bucket. ``stopped`` Status
  maps only to ``## Stopped``.
- Every failure names ``path:line`` of the line to fix.

Hermetic tests pass ``--root`` pointing at a throwaway tree. Ambient
environment variables cannot retarget this checker.

Accepted Status buckets and generation-index census phrases are also
documented in docs/roadmaps/status-grammar.md. Live census regexes:

- completed: ``N completed milestones``
- stops: ``honest evidence stops at …`` or ``no honest evidence stops``
- ready: ``one ready milestone at`` / ``ready milestone(s) at``
"""

from __future__ import annotations

import argparse
import re
import sys
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
    print(
        f"roadmap status drift check failed: {rel}:{at[1]}: {message}",
        file=sys.stderr,
    )
    raise SystemExit(1)


def active_generation_id() -> str:
    document = GENERATION_INDEX.read_text(encoding="utf-8")
    matches = list(
        re.finditer(
            r"^\| `(?P<generation>g\d{2})` \| active \|",
            document,
            re.MULTILINE,
        )
    )
    if len(matches) != 1:
        line = line_at(document, matches[0].start()) if matches else 1
        fail(
            "generation index must name exactly one active generation",
            at=(GENERATION_INDEX, line),
        )
    return matches[0].group("generation")


ACTIVE_GENERATION = ""
BATCH_DIR = ROOT
BATCH_INDEX = ROOT
MILESTONE_DIR = ROOT
MILESTONE_INDEX = ROOT


def bind_paths(root: Path) -> None:
    global ROOT, GENERATION_INDEX, ACTIVE_GENERATION
    global BATCH_DIR, BATCH_INDEX, MILESTONE_DIR, MILESTONE_INDEX
    ROOT = root.resolve()
    GENERATION_INDEX = ROOT / "docs/roadmaps/generation-index.md"
    ACTIVE_GENERATION = active_generation_id()
    BATCH_DIR = ROOT / f"docs/roadmaps/{ACTIVE_GENERATION}/batch-cards"
    BATCH_INDEX = BATCH_DIR / "README.md"
    MILESTONE_DIR = ROOT / f"docs/roadmaps/{ACTIVE_GENERATION}"
    MILESTONE_INDEX = MILESTONE_DIR / "README.md"


def parse_args(argv: list[str] | None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Reject roadmap and batch-card Status drift."
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
    r"^## (?P<title>Planned|Ready|Blocked|Stopped|Completed)\s*$",
    re.MULTILINE,
)
CARD_READY_PROSE_RE = re.compile(
    r"cards?\s+(?P<ids>(?:\d{3}(?:\s*[-–,]\s*\d{3})*)+)\s+(?:is|are)\s+ready",
    re.IGNORECASE,
)
STOPPED_LIST_RE = re.compile(
    r"honest evidence\s+stops at\s+(?P<ids>[\d,\s]+(?:and\s+\d+)?)",
    re.IGNORECASE,
)
COMPLETED_COUNT_RE = re.compile(r"(?P<count>\d+)\s+completed milestones", re.IGNORECASE)
READY_MILESTONE_RE = re.compile(
    r"(?:one ready milestone at|ready milestones? at)\s+(?P<ids>\d{3}(?:\s*,\s*\d{3})*)",
    re.IGNORECASE,
)

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


def check_batch_cards() -> None:
    document = read(BATCH_INDEX)
    sections = list(SECTION_RE.finditer(document))
    if not sections:
        fail(
            "batch-card index has no Planned/Ready/Blocked/Stopped/Completed sections",
            at=(BATCH_INDEX, 1),
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

    card_files = sorted(
        path for path in BATCH_DIR.glob("*.md") if path.name != "README.md"
    )
    for path in card_files:
        expected, status_line = frontmatter_status(path)
        entries = indexed.get(path.name, [])
        if not entries:
            fail(
                f"batch card {path.name} is not indexed in {BATCH_INDEX.relative_to(ROOT)}",
                at=(path, status_line),
            )
        if len(entries) > 1:
            places = ", ".join(f"{bucket} at line {line}" for bucket, _, line in entries)
            fail(
                f"batch card {path.name} is indexed more than once ({places})",
                at=(BATCH_INDEX, entries[0][2]),
            )
        section_bucket, annotation, index_line = entries[0]
        if section_bucket != expected:
            fail(
                f"batch card {path.name} Status bucket is {expected!r} but index lists it under {section_bucket!r}",
                at=(BATCH_INDEX, index_line),
            )
        primary = annotation_primary(annotation)
        if primary is not None:
            allowed = ANNOTATION_ALLOWED[expected]
            if primary not in allowed:
                fail(
                    f"batch card {path.name} annotation primary {primary!r} does not match Status bucket {expected!r}",
                    at=(BATCH_INDEX, index_line),
                )

    for name, entries in sorted(indexed.items()):
        if not (BATCH_DIR / name).is_file():
            fail(
                f"batch-card index links missing file {name}",
                at=(BATCH_INDEX, entries[0][2]),
            )


def check_milestones() -> None:
    document = read(MILESTONE_INDEX)
    milestone_files = {
        path.name: path
        for path in MILESTONE_DIR.glob("*.md")
        if path.name != "README.md" and re.match(r"^\d{3}-", path.name)
    }
    for link in LINK_RE.finditer(document):
        name = link.group("file")
        path = milestone_files.get(name)
        if path is None:
            continue
        expected, _status_line = frontmatter_status(path)
        primary = annotation_primary(link.group("ann"))
        if primary is None:
            continue
        allowed = ANNOTATION_ALLOWED[expected]
        if primary not in allowed:
            fail(
                f"milestone {name} annotation primary {primary!r} does not match Status bucket {expected!r}",
                at=(MILESTONE_INDEX, line_at(document, link.start())),
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
    for path in MILESTONE_DIR.glob("*.md"):
        if path.name == "README.md" or not re.match(r"^\d{3}-", path.name):
            continue
        number = path.name[:3]
        bucket, _status_line = frontmatter_status(path)
        buckets[bucket].add(number)

    for match in CARD_READY_PROSE_RE.finditer(document):
        line = line_at(document, match.start())
        for number in parse_id_list(match.group("ids"), at=(GENERATION_INDEX, line)):
            path = next(BATCH_DIR.glob(f"{number}-*.md"), None)
            if path is None:
                fail(
                    f"generation-index claims card {number} is ready but the card file is missing",
                    at=(GENERATION_INDEX, line),
                )
            actual, _status_line = frontmatter_status(path)
            if actual != "ready":
                fail(
                    f"generation-index claims card {number} is ready but Status bucket is {actual!r}",
                    at=(GENERATION_INDEX, line),
                )

    census, census_line = active_generation_census(document)
    ready_claimed: set[str] = set()
    for match in READY_MILESTONE_RE.finditer(census):
        ready_line = census_line + line_at(census, match.start()) - 1
        ready_claimed.update(parse_id_list(match.group("ids"), at=(GENERATION_INDEX, ready_line)))
    if ready_claimed != buckets["ready"]:
        fail(
            "generation-index ready milestone set "
            f"{sorted(ready_claimed)} disagrees with frontmatter {sorted(buckets['ready'])}",
            at=(GENERATION_INDEX, census_line),
        )

    completed_match = COMPLETED_COUNT_RE.search(census)
    if completed_match is None:
        fail(
            f"generation-index {ACTIVE_GENERATION} census omits completed milestone count",
            at=(GENERATION_INDEX, census_line),
        )
    claimed = int(completed_match.group("count"))
    actual = len(buckets["complete"])
    if claimed != actual:
        fail(
            f"generation-index claims {claimed} completed milestones but frontmatter has {actual}",
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


def main(argv: list[str] | None = None) -> None:
    args = parse_args(argv)
    bind_paths(args.root)
    check_batch_cards()
    check_milestones()
    check_generation_index()
    print("roadmap status drift check passed")


if __name__ == "__main__":
    main()
