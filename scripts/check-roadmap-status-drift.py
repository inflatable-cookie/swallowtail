#!/usr/bin/env python3
"""Fail when roadmap/batch-card indexes disagree with Status frontmatter."""

from __future__ import annotations

import re
import sys
from collections import defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
BATCH_DIR = ROOT / "docs/roadmaps/g04/batch-cards"
BATCH_INDEX = BATCH_DIR / "README.md"
MILESTONE_DIR = ROOT / "docs/roadmaps/g04"
MILESTONE_INDEX = MILESTONE_DIR / "README.md"
GENERATION_INDEX = ROOT / "docs/roadmaps/generation-index.md"

STATUS_RE = re.compile(r"^Status:\s*(?P<raw>.+)$", re.MULTILINE)
LINK_RE = re.compile(
    r"^- \[.*?\]\(\.?/?(?P<file>\d{3}-[^)\s]+\.md)\)(?:\s*—\s*(?P<ann>.*))?$",
    re.MULTILINE,
)
SECTION_RE = re.compile(r"^## (?P<title>Planned|Ready|Blocked|Completed)\s*$", re.MULTILINE)
CARD_READY_PROSE_RE = re.compile(
    r"cards?\s+(?P<ids>(?:\d{3}(?:\s*[-–,]\s*\d{3})*)+)\s+are\s+ready",
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
    "Completed": "complete",
}

ANNOTATION_ALLOWED = {
    "planned": {"planned"},
    "ready": {"ready"},
    "blocked": {"blocked"},
    "complete": {"complete", "completed", "done", "evidence stop", "identity stop"},
    "stopped": {"stopped"},
}


def fail(message: str) -> None:
    print(f"roadmap status drift check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def status_bucket(raw: str) -> str | None:
    primary = re.split(r"[;\n]", raw, maxsplit=1)[0].strip().lower()
    match = re.match(
        r"(planned|ready|blocked|stopped|complete(?:d)?|done)\b",
        primary,
    )
    if match is None:
        return None
    token = match.group(1)
    if token in {"complete", "completed", "done"}:
        return "complete"
    if token == "stopped":
        return "stopped"
    return token


def frontmatter_status(path: Path) -> str:
    match = STATUS_RE.search(read(path))
    if match is None:
        fail(f"{path.relative_to(ROOT)} has no Status line")
    bucket = status_bucket(match.group("raw"))
    if bucket is None:
        fail(f"{path.relative_to(ROOT)} has unrecognized Status {match.group('raw')!r}")
    return bucket


def annotation_primary(annotation: str | None) -> str | None:
    if annotation is None:
        return None
    lowered = annotation.strip().lower()
    for candidate in (
        "evidence stop",
        "identity stop",
        "stopped",
        "completed",
        "complete",
        "blocked",
        "planned",
        "ready",
        "done",
    ):
        if re.search(rf"\b{re.escape(candidate)}\b", lowered):
            return candidate
    return None


def parse_id_list(text: str) -> set[str]:
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
                fail(f"inverted id range {chunk!r}")
            ids.update(f"{value:03d}" for value in range(start, end + 1))
            continue
        single = re.fullmatch(r"\d{3}", chunk)
        if single:
            ids.add(chunk)
            continue
        fail(f"unparseable id list fragment {chunk!r}")
    return ids


def check_batch_cards() -> None:
    document = read(BATCH_INDEX)
    sections = list(SECTION_RE.finditer(document))
    if not sections:
        fail("batch-card index has no Planned/Ready/Blocked/Completed sections")

    indexed: dict[str, list[tuple[str, str | None]]] = defaultdict(list)
    for index, match in enumerate(sections):
        start = match.end()
        end = sections[index + 1].start() if index + 1 < len(sections) else len(document)
        body = document[start:end]
        section_name = match.group("title")
        bucket = SECTION_BUCKET[section_name]
        for link in LINK_RE.finditer(body):
            indexed[link.group("file")].append((bucket, link.group("ann")))

    card_files = sorted(
        path for path in BATCH_DIR.glob("*.md") if path.name != "README.md"
    )
    for path in card_files:
        expected = frontmatter_status(path)
        entries = indexed.get(path.name, [])
        if not entries:
            fail(f"batch card {path.name} is not indexed in {BATCH_INDEX.relative_to(ROOT)}")
        if len(entries) > 1:
            places = ", ".join(bucket for bucket, _ in entries)
            fail(f"batch card {path.name} is indexed more than once ({places})")
        section_bucket, annotation = entries[0]
        if section_bucket != expected:
            fail(
                f"batch card {path.name} Status bucket is {expected!r} but index lists it under {section_bucket!r}"
            )
        primary = annotation_primary(annotation)
        if primary is not None:
            allowed = ANNOTATION_ALLOWED[expected]
            if primary not in allowed:
                fail(
                    f"batch card {path.name} annotation primary {primary!r} does not match Status bucket {expected!r}"
                )

    for name in sorted(indexed):
        if not (BATCH_DIR / name).is_file():
            fail(f"batch-card index links missing file {name}")


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
        expected = frontmatter_status(path)
        primary = annotation_primary(link.group("ann"))
        if primary is None:
            continue
        allowed = ANNOTATION_ALLOWED[expected]
        if primary not in allowed:
            fail(
                f"milestone {name} annotation primary {primary!r} does not match Status bucket {expected!r}"
            )


def active_generation_census(document: str) -> str:
    match = re.search(
        r"^g04 now has .+?(?=^g04\.|^## |\Z)",
        document,
        re.MULTILINE | re.DOTALL,
    )
    if match is None:
        fail("generation-index is missing the active g04 census paragraph")
    return match.group(0)


def check_generation_index() -> None:
    document = read(GENERATION_INDEX)
    buckets: dict[str, set[str]] = defaultdict(set)
    for path in MILESTONE_DIR.glob("*.md"):
        if path.name == "README.md" or not re.match(r"^\d{3}-", path.name):
            continue
        number = path.name[:3]
        buckets[frontmatter_status(path)].add(number)

    for match in CARD_READY_PROSE_RE.finditer(document):
        for number in parse_id_list(match.group("ids")):
            path = next(BATCH_DIR.glob(f"{number}-*.md"), None)
            if path is None:
                fail(f"generation-index claims card {number} is ready but the card file is missing")
            actual = frontmatter_status(path)
            if actual != "ready":
                fail(
                    f"generation-index claims card {number} is ready but Status bucket is {actual!r}"
                )

    census = active_generation_census(document)
    ready_claimed: set[str] = set()
    for match in READY_MILESTONE_RE.finditer(census):
        ready_claimed.update(parse_id_list(match.group("ids")))
    if ready_claimed != buckets["ready"]:
        fail(
            "generation-index ready milestone set "
            f"{sorted(ready_claimed)} disagrees with frontmatter {sorted(buckets['ready'])}"
        )

    completed_match = COMPLETED_COUNT_RE.search(census)
    if completed_match is None:
        fail("generation-index g04 census omits completed milestone count")
    claimed = int(completed_match.group("count"))
    actual = len(buckets["complete"])
    if claimed != actual:
        fail(
            f"generation-index claims {claimed} completed milestones but frontmatter has {actual}"
        )

    stopped_match = STOPPED_LIST_RE.search(census)
    if stopped_match is None:
        fail("generation-index g04 census omits honest evidence stop list")
    claimed_ids = parse_id_list(stopped_match.group("ids"))
    actual_stopped = buckets["stopped"]
    if claimed_ids != actual_stopped:
        fail(
            "generation-index honest evidence stops "
            f"{sorted(claimed_ids)} disagree with frontmatter {sorted(actual_stopped)}"
        )


def main() -> None:
    check_batch_cards()
    check_milestones()
    check_generation_index()
    print("roadmap status drift check passed")


if __name__ == "__main__":
    main()
