#!/usr/bin/env python3
"""Validate feature-matrix cross ownership and print the pinned-route backlog."""

from __future__ import annotations

import argparse
import csv
import json
import re
from pathlib import Path


CLASSIFICATION_COLUMNS = ("cross_kind", "cross_ref")
ALLOWED_KINDS = {"provider_limitation", "producer_gap"}
PINNED_CONSUMERS = (
    ("Bovine Claude", "claude-agent.sdk"),
    ("Bovine Codex", "codex.app-server"),
    ("Bovine Grok", "grok-build.acp"),
    ("Nucleus", "codex.app-server"),
)
ROUTE_SPLIT = re.compile(r"\s*(?:;|\+)\s*")
CARD_STATUS = re.compile(r"^Status:\s*(.+)$", re.MULTILINE)


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


def route_guide_evidence(root: Path, ref: str) -> None:
    path = root / ref
    if not path.is_file():
        fail(f"provider_limitation reference does not exist: {ref}")
    try:
        relative = path.relative_to(root)
    except ValueError:
        fail(f"provider_limitation reference escapes the repository: {ref}")
    if relative.parts[:2] not in {
        ("docs", "guides"),
        ("docs", "research"),
        ("docs", "contracts"),
    }:
        fail(
            "provider_limitation reference must be frozen docs/research, "
            f"docs/contracts, or docs/guides evidence: {ref}"
        )


def producer_card(root: Path, ref: str) -> None:
    path = root / ref
    if not path.is_file() or path.parent != root / "docs/roadmaps/g05/batch-cards":
        fail(f"producer_gap reference must be an existing g05 batch card: {ref}")
    match = CARD_STATUS.search(path.read_text(encoding="utf-8"))
    if match is None:
        fail(f"producer_gap card lacks a Status line: {ref}")
    status = match.group(1).strip().casefold()
    if status.startswith("complete") or status.startswith("completed"):
        fail(f"producer_gap references a complete card: {ref}")


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
                route_guide_evidence(root, ref)
            else:
                producer_card(root, ref)
            if row[feature].strip().casefold() == "withheld" and kind != "producer_gap":
                fail(f"{route} {feature} is withheld but is not a producer_gap")
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
        for route in ROUTE_SPLIT.split(row["route_id"]):
            consumers = route_to_consumers.get(route)
            if consumers is None:
                continue
            for consumer in consumers:
                for feature in features:
                    if kinds.get(feature) != "producer_gap":
                        continue
                    result.append((consumer, route, feature, refs[feature], row["notes"]))
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
        print("consumer | route | feature | producer card")
        print("--- | --- | --- | ---")
        for consumer, route, feature, ref, _notes in gaps:
            print(f"{consumer} | `{route}` | `{feature}` | `{ref}`")
    else:
        print(
            f"feature-matrix cross classification passed: {len(rows)} solution rows, "
            f"{len(features)} features, {cells} unavailable cells, "
            f"{len(gaps)} pinned producer gaps"
        )


if __name__ == "__main__":
    main()
