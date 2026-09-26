#!/usr/bin/env python3
"""Validate feature-matrix cross ownership and print the pinned-route backlog."""

from __future__ import annotations

import argparse
import csv
import json
import re
from pathlib import Path


CLASSIFICATION_COLUMNS = ("cross_kind", "cross_ref")
ALLOWED_KINDS = {"provider_limitation", "producer_gap", "evidence_pending"}
PINNED_CONSUMERS = (
    ("Bovine Claude", "claude-agent.sdk"),
    ("Bovine Codex", "codex.app-server"),
    ("Bovine Grok", "grok-build.acp"),
    ("Nucleus", "codex.app-server"),
)
ROUTE_SPLIT = re.compile(r"\s*(?:;|\+)\s*")
REASON_MARKER = "Card129 producer-gap reasons:"
PLAN = Path("docs/plan.md")
PLAN_REF = re.compile(r"^plan:(?P<key>[a-z0-9][a-z0-9-]*)$")
PLAN_LIVE_SECTIONS = {"## Now", "## Next"}
QUESTIONS = Path("docs/knowledge/questions.md")
QUESTION_REF = re.compile(r"^docs/knowledge/questions\.md#(?P<id>q-[0-9]{3})$")
QUESTION_HEADING = re.compile(r"^## (?P<id>Q-[0-9]{3})\b")
GATE_SCOPE_MARKER = "Evidence gate scope:"
EVIDENCE_LEDGER = Path("docs/research/290-feature-matrix-cross-evidence.tsv")
EVIDENCE_DOC = Path("docs/research/290-feature-matrix-cross-evidence.md")
LINE_REF = re.compile(r"^(?P<path>[^#]+)#L(?P<line>[1-9][0-9]*)$")


def fail(message: str) -> None:
    raise SystemExit(f"feature matrix cross classification: {message}")


def parse_map(raw: str, row_route: str, column: str) -> dict[str, str]:
    try:
        value = json.loads(raw)
    except json.JSONDecodeError as error:
        fail(f"{row_route} has invalid {column} JSON: {error}")
    if not isinstance(value, dict) or not all(
        isinstance(key, str) and isinstance(item, str) for key, item in value.items()
    ):
        fail(f"{row_route} {column} must be a JSON object of strings")
    return value


def feature_columns(headers: list[str]) -> list[str]:
    try:
        first_cross = headers.index(CLASSIFICATION_COLUMNS[0])
        second_cross = headers.index(CLASSIFICATION_COLUMNS[1])
        notes = headers.index("notes")
    except ValueError as error:
        fail(f"missing classification or notes column: {error}")
    if second_cross != first_cross + 1 or notes != second_cross + 1:
        fail("cross_kind and cross_ref must be adjacent immediately before notes")
    return headers[13:first_cross]


def anchored_line(root: Path, ref: str, label: str) -> tuple[Path, int, list[str]]:
    match = LINE_REF.fullmatch(ref)
    if match is None:
        fail(f"{label} must use an anchored #L<line> citation: {ref}")
    path = root / match.group("path")
    line_number = int(match.group("line"))
    if not path.is_file():
        fail(f"{label} reference does not exist: {ref}")
    try:
        relative = path.relative_to(root)
    except ValueError:
        fail(f"{label} reference escapes the repository: {ref}")
    if relative.parts[:2] not in {("docs", "research"), ("docs", "contracts")}:
        fail(f"{label} reference must cite frozen docs/research or docs/knowledge/contracts: {ref}")
    lines = path.read_text(encoding="utf-8").splitlines()
    if line_number > len(lines):
        fail(f"{label} citation is past EOF: {ref}")
    return path, line_number, lines


def evidence_ledger(root: Path, row_route: str, feature: str, ref: str) -> None:
    evidence_doc = root / EVIDENCE_DOC
    if not evidence_doc.is_file():
        fail(f"frozen evidence document is missing: {EVIDENCE_DOC}")
    if not any(line.startswith("Status: complete") for line in evidence_doc.read_text(encoding="utf-8").splitlines()):
        fail(f"frozen evidence document is not complete: {EVIDENCE_DOC}")
    path, line_number, lines = anchored_line(root, ref, "provider_limitation")
    if path != root / EVIDENCE_LEDGER:
        fail(
            "provider_limitation must cite the Card129 frozen evidence ledger, "
            f"not {path.relative_to(root)}"
        )
    if line_number == 1:
        fail(f"provider_limitation cannot cite the evidence-ledger header: {ref}")
    fields = lines[line_number - 1].split("\t")
    if len(fields) != 5:
        fail(f"provider_limitation evidence row is malformed: {ref}")
    evidence_route, evidence_feature, evidence_kind, basis, finding = fields
    if (evidence_route, evidence_feature, evidence_kind) != (
        row_route,
        feature,
        "provider_limitation",
    ):
        fail(f"provider_limitation citation does not match {row_route} {feature}: {ref}")
    if not finding.startswith("qualified route unavailable:"):
        fail(f"provider_limitation evidence lacks an explicit unavailable finding: {ref}")
    _, basis_line, basis_lines = anchored_line(root, basis, "provider_limitation basis")
    basis_fields = basis_lines[basis_line - 1].split("\t")
    if not basis_fields or basis_fields[0].strip("`") not in ROUTE_SPLIT.split(row_route):
        fail(f"provider_limitation basis does not name {row_route}: {ref}")


def producer_gap_reasons(notes: str, row_route: str) -> dict[str, str]:
    if REASON_MARKER not in notes:
        return {}
    raw = notes.split(REASON_MARKER, 1)[1].strip().rstrip(".")
    if not raw:
        fail(f"{row_route} has an empty producer-gap reason marker")
    reasons: dict[str, str] = {}
    for entry in raw.split(" || "):
        feature, separator, reason = entry.partition("=")
        if not separator or not feature.strip() or not reason.strip():
            fail(f"{row_route} has a malformed producer-gap reason: {entry!r}")
        if feature.strip() in reasons:
            fail(f"{row_route} repeats a producer-gap reason: {feature.strip()}")
        reasons[feature.strip()] = reason.strip()
    return reasons


def producer_task(root: Path, ref: str) -> None:
    """Guard the producer_gap cross kind: the gap must be a planned outcome.

    The reference names a ``docs/plan.md`` item by its lane key. The item must
    sit under ``## Now`` or ``## Next``; a deferred or missing item would leave
    the gap with no owner.
    """
    match = PLAN_REF.fullmatch(ref)
    if match is None:
        fail(f"producer_gap reference must be plan:<key>: {ref}")
    plan = root / PLAN
    if not plan.is_file():
        fail(f"missing {PLAN}")
    marker = re.compile(rf"\(lane\s+`{re.escape(match.group('key'))}`\)")
    sections: dict[str, list[str]] = {}
    section = ""
    for line in plan.read_text(encoding="utf-8").splitlines():
        if line.startswith("## "):
            section = line.strip()
        sections.setdefault(section, []).append(line.strip())
    for heading, lines in sections.items():
        if marker.search(" ".join(lines)):
            if heading in PLAN_LIVE_SECTIONS:
                return
            fail(f"producer_gap plan item is not under Now or Next: {ref}")
    fail(f"producer_gap reference names no docs/plan.md item: {ref}")


def evidence_packet(root: Path, ref: str, row_route: str, feature: str) -> None:
    """Guard the evidence_pending cross kind per the Feature Matrix Rule.

    The reference must be an open question in ``docs/knowledge/questions.md``
    — never a plan item, because finishing the work closes its gate. The
    question must name the owner who runs the gate, state the decision tree
    converting each outcome into ``producer_gap`` or ``provider_limitation``,
    and list the cells it investigates; evidence pending is unavailable to
    any cell no open question covers.
    """
    match = QUESTION_REF.fullmatch(ref)
    questions = root / QUESTIONS
    if match is None or not questions.is_file():
        fail(f"evidence_pending must reference an open docs/knowledge/questions.md entry: {ref}")
    wanted = match.group("id").upper()
    section: list[str] | None = None
    for line in questions.read_text(encoding="utf-8").splitlines():
        heading = QUESTION_HEADING.match(line)
        if heading or line.startswith("## "):
            if section is not None:
                break
            if heading and heading.group("id") == wanted:
                section = []
            continue
        if section is not None:
            section.append(line)
    if section is None:
        fail(f"evidence_pending question does not exist: {ref}")
    text = "\n".join(section)
    if re.search(r"(?m)^Status:\s*open\s*$", text) is None:
        fail(f"evidence_pending must reference an open question: {ref}")
    lowered = text.casefold()
    if "owner" not in lowered:
        fail(f"evidence_pending question must name the owner who runs the gate: {ref}")
    if "producer_gap" not in lowered or "provider_limitation" not in lowered:
        fail(
            "evidence_pending question must state the outcome decision tree into "
            f"producer_gap or provider_limitation: {ref}"
        )
    scoped_cells: set[tuple[str, str]] = set()
    in_scope = False
    for line in section:
        if line.strip() == GATE_SCOPE_MARKER:
            in_scope = True
        elif in_scope and line.startswith("- "):
            tokens = line[2:].replace("`", "").split()
            if len(tokens) >= 2:
                scoped_cells.add((tokens[0], tokens[1]))
        elif in_scope and line.strip():
            in_scope = False
    if not any(
        (route_token, feature) in scoped_cells
        for route_token in ROUTE_SPLIT.split(row_route)
    ):
        fail(
            f"evidence_pending cell {row_route} {feature} is not in the "
            f"question gate scope: {ref}"
        )


def load_matrix(root: Path, matrix: Path) -> tuple[list[dict[str, str]], list[str]]:
    with matrix.open(newline="", encoding="utf-8") as source:
        reader = csv.DictReader(source)
        headers = reader.fieldnames or []
        features = feature_columns(headers)
        rows = list(reader)
    if not rows:
        fail("matrix has no rows")
    for row in rows:
        if row.get(None):
            fail(f"{row['route_id']} has more CSV fields than the header")
        route = row["route_id"]
        kinds = parse_map(row["cross_kind"], route, "cross_kind")
        refs = parse_map(row["cross_ref"], route, "cross_ref")
        if set(kinds) != set(refs):
            fail(f"{route} cross_kind and cross_ref keys differ")
        unavailable = {
            feature
            for feature in features
            if row[feature].strip().casefold() in {"no", "withheld"}
        }
        if set(kinds) != unavailable:
            missing = sorted(unavailable - set(kinds))
            extra = sorted(set(kinds) - unavailable)
            fail(f"{route} cross keys mismatch: missing={missing} extra={extra}")
        for feature, kind in kinds.items():
            if kind not in ALLOWED_KINDS:
                fail(f"{route} {feature} has invalid cross_kind {kind!r}")
            ref = refs[feature]
            if not ref:
                fail(f"{route} {feature} has an empty cross_ref")
            if kind == "provider_limitation":
                evidence_ledger(root, route, feature, ref)
            elif kind == "producer_gap":
                producer_task(root, ref)
            else:
                evidence_packet(root, ref, route, feature)
            if row[feature].strip().casefold() == "withheld" and kind != "producer_gap":
                fail(f"{route} {feature} is withheld but is not a producer_gap")
        reasons = producer_gap_reasons(row["notes"], route)
        producer_features = {feature for feature, kind in kinds.items() if kind == "producer_gap"}
        if set(reasons) != producer_features:
            fail(
                f"{route} producer-gap reasons mismatch: "
                f"missing={sorted(producer_features - set(reasons))} "
                f"extra={sorted(set(reasons) - producer_features)}"
            )
    return rows, features


def backlog(rows: list[dict[str, str]], features: list[str]) -> list[tuple[str, str, str, str, str]]:
    route_to_consumers: dict[str, list[str]] = {}
    consumer_order = {consumer: index for index, (consumer, _route) in enumerate(PINNED_CONSUMERS)}
    for consumer, route in PINNED_CONSUMERS:
        route_to_consumers.setdefault(route, []).append(consumer)
    result: list[tuple[str, str, str, str, str]] = []
    for row in rows:
        kinds = json.loads(row["cross_kind"])
        refs = json.loads(row["cross_ref"])
        reasons = producer_gap_reasons(row["notes"], row["route_id"])
        for route in ROUTE_SPLIT.split(row["route_id"]):
            consumers = route_to_consumers.get(route)
            if consumers is None:
                continue
            for consumer in consumers:
                for feature in features:
                    if kinds.get(feature) != "producer_gap":
                        continue
                    result.append((consumer, route, feature, refs[feature], reasons[feature]))
    result.sort(key=lambda item: (consumer_order[item[0]], item[1], item[2], item[3]))
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("matrix", type=Path)
    parser.add_argument("--backlog", action="store_true")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    rows, features = load_matrix(root, args.matrix)
    cells = sum(
        1
        for row in rows
        for feature in features
        if row[feature].strip().casefold() in {"no", "withheld"}
    )
    gaps = backlog(rows, features)
    if args.backlog:
        print("consumer | route | feature | plan item | reason")
        print("--- | --- | --- | --- | ---")
        for consumer, route, feature, ref, reason in gaps:
            print(f"{consumer} | `{route}` | `{feature}` | `{ref}` | {reason}")
    else:
        print(
            f"feature-matrix cross classification passed: {len(rows)} solution rows, "
            f"{len(features)} features, {cells} unavailable cells, "
            f"{len(gaps)} pinned producer gaps"
        )


if __name__ == "__main__":
    main()
