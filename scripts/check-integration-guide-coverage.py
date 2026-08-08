#!/usr/bin/env python3
"""Validate Contract 052 route, feature, guide, and example coverage."""

from __future__ import annotations

import csv
import re
import sys
from collections import Counter
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO_ROOT / "scripts"))
from provider_route_matrix.route_inventory import (  # noqa: E402
    production_routes as inventory_production_routes,
)
GUIDES_DIR = REPO_ROOT / "docs/guides"
GUIDE_MAP = GUIDES_DIR / "integration-guide-map.md"
GUIDE_INDEX = GUIDES_DIR / "README.md"
ROUTE_MATRIX = GUIDES_DIR / "provider-route-matrix.md"
FEATURE_MATRIX = GUIDES_DIR / "provider-solution-feature-matrix.csv"

FEATURE_METADATA_HEADERS = [
    "provider",
    "solution",
    "route_id",
    "adapter_crate",
    "driver_id",
    "execution_layer",
    "runtime_posture",
    "runtime_ownership",
    "transport",
    "access_posture",
    "version_axis",
    "guaranteed_version_posture",
    "unverified_newer_allowed",
]

PORTABLE_FEATURES = {
    "configured_provider_instance_catalogue",
    "observable_activity",
    "plan_mode",
    "task_lists",
    "subagent_topology",
    "subagent_control",
    "working_state_restoration",
    "failure_classification",
    "operator_validation",
}

LINK_PATTERN = re.compile(r"\[[^\]]+\]\(([^)]+)\)")
TOKEN_PATTERN = re.compile(r"`([^`]+)`")
ROUTE_PATTERN = re.compile(r"^\| `([^`]+)` \|", re.MULTILINE)


class CoverageFailure(Exception):
    pass


def table_rows(document: str, heading: str, columns: int) -> list[list[str]]:
    marker = f"## {heading}"
    try:
        section = document.split(marker, 1)[1]
    except IndexError as error:
        raise CoverageFailure(f"guide map lacks {marker}") from error
    section = section.split("\n## ", 1)[0]
    rows: list[list[str]] = []
    for line in section.splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if not cells or cells[0] in {"Route", "Feature family"}:
            continue
        if all(set(cell) <= {"-", ":"} for cell in cells):
            continue
        if len(cells) != columns:
            raise CoverageFailure(
                f"{heading} row has {len(cells)} columns instead of {columns}: {line}"
            )
        rows.append(cells)
    return rows


def links(cell: str) -> list[str]:
    return LINK_PATTERN.findall(cell)


def resolve_link(target: str) -> Path:
    if "://" in target or target.startswith("#"):
        raise CoverageFailure(f"coverage owner must be a local file: {target}")
    return (GUIDE_MAP.parent / target.split("#", 1)[0]).resolve()


def require_file(target: str, expected_root: Path, suffix: str, role: str) -> Path:
    path = resolve_link(target)
    if not path.is_file():
        raise CoverageFailure(f"{role} does not exist: {target}")
    try:
        path.relative_to(expected_root.resolve())
    except ValueError as error:
        raise CoverageFailure(f"{role} escapes {expected_root.relative_to(REPO_ROOT)}: {target}") from error
    if path.suffix != suffix:
        raise CoverageFailure(f"{role} must end in {suffix}: {target}")
    return path


def production_routes() -> list[str]:
    routes = inventory_production_routes()
    route_document = ROUTE_MATRIX.read_text()
    ordinary_routes = route_document.split(
        "<!-- provider-session-lifecycle-matrix:start -->", 1
    )[0]
    documented = ROUTE_PATTERN.findall(ordinary_routes)
    if set(documented) != set(routes) or len(documented) != len(routes):
        missing = sorted(set(routes) - set(documented))
        unexpected = sorted(set(documented) - set(routes))
        raise CoverageFailure(
            "provider route matrix routes differ from the feature matrix: "
            f"missing={missing} unexpected={unexpected}"
        )
    return routes


def portable_feature_inventory() -> set[str]:
    with FEATURE_MATRIX.open(newline="") as matrix:
        headers = next(csv.reader(matrix))
    if headers[: len(FEATURE_METADATA_HEADERS)] != FEATURE_METADATA_HEADERS:
        raise CoverageFailure("provider solution feature metadata headers changed")
    if not headers or headers[-1] != "notes":
        raise CoverageFailure("provider solution feature matrix must end with notes")
    feature_headers = headers[len(FEATURE_METADATA_HEADERS) : -1]
    if len(feature_headers) != 34:
        raise CoverageFailure(
            f"provider solution feature matrix contains {len(feature_headers)} feature columns instead of 34"
        )
    duplicates = sorted(
        feature for feature, count in Counter(feature_headers).items() if count > 1
    )
    if duplicates:
        raise CoverageFailure(f"provider solution feature matrix has duplicate features: {duplicates}")
    return set(feature_headers) | PORTABLE_FEATURES


def indexed_guides() -> set[Path]:
    indexed: set[Path] = set()
    for target in links(GUIDE_INDEX.read_text()):
        if "://" not in target and not target.startswith("#"):
            indexed.add((GUIDE_INDEX.parent / target.split("#", 1)[0]).resolve())
    return indexed


def validate_route_guides(
    rows: list[list[str]], expected_routes: list[str], guide_index: set[Path]
) -> tuple[int, int]:
    mapped_routes: list[str] = []
    guides: set[Path] = set()
    examples: set[Path] = set()
    for route_cell, guide_cell, example_cell, coverage in rows:
        match = re.fullmatch(r"`([^`]+)`", route_cell)
        if match is None:
            raise CoverageFailure(f"route guide row lacks one exact route id: {route_cell}")
        route = match.group(1)
        mapped_routes.append(route)
        if coverage != "complete":
            raise CoverageFailure(f"route guide is not complete: {route} ({coverage})")

        guide_targets = links(guide_cell)
        example_targets = links(example_cell)
        if len(guide_targets) != 1:
            raise CoverageFailure(f"route must have one canonical guide: {route}")
        if len(example_targets) != 1:
            raise CoverageFailure(f"route must have one normal-path example: {route}")

        guide = require_file(guide_targets[0], GUIDES_DIR, ".md", "route guide")
        example = require_file(
            example_targets[0], REPO_ROOT / "crates", ".rs", "route example"
        )
        if "examples" not in example.parts:
            raise CoverageFailure(f"route example is not under an examples directory: {route}")
        if guide not in guide_index:
            raise CoverageFailure(f"route guide is missing from docs/guides/README.md: {route}")
        guides.add(guide)
        examples.add(example)

    expected = set(expected_routes)
    actual = set(mapped_routes)
    duplicates = sorted(
        route for route, count in Counter(mapped_routes).items() if count > 1
    )
    if duplicates:
        raise CoverageFailure(f"integration guide map contains duplicate routes: {duplicates}")
    if expected != actual:
        missing = sorted(expected - actual)
        unexpected = sorted(actual - expected)
        raise CoverageFailure(
            f"integration guide route coverage differs: missing={missing} unexpected={unexpected}"
        )
    return len(guides), len(examples)


def validate_feature_guides(
    rows: list[list[str]], expected_features: set[str], guide_index: set[Path]
) -> tuple[int, int]:
    families: list[str] = []
    feature_occurrences: Counter[str] = Counter()
    guides: set[Path] = set()
    for family, surfaces, guide_cell, coverage in rows:
        if not family:
            raise CoverageFailure("feature guide row has an empty family")
        families.append(family)
        if coverage != "complete":
            raise CoverageFailure(f"feature guide is not complete: {family} ({coverage})")
        tokens = TOKEN_PATTERN.findall(surfaces)
        if not tokens:
            raise CoverageFailure(f"feature family has no machine-readable surfaces: {family}")
        feature_occurrences.update(tokens)

        guide_targets = links(guide_cell)
        if not guide_targets:
            raise CoverageFailure(f"feature family has no canonical guide: {family}")
        for target in guide_targets:
            guide = require_file(target, GUIDES_DIR, ".md", "feature guide")
            if guide not in guide_index:
                raise CoverageFailure(
                    f"feature guide is missing from docs/guides/README.md: {target}"
                )
            guides.add(guide)

    duplicate_families = sorted(
        family for family, count in Counter(families).items() if count > 1
    )
    if duplicate_families:
        raise CoverageFailure(f"duplicate feature guide families: {duplicate_families}")
    duplicate_features = sorted(
        feature for feature, count in feature_occurrences.items() if count > 1
    )
    if duplicate_features:
        raise CoverageFailure(f"features have multiple canonical owners: {duplicate_features}")
    actual = set(feature_occurrences)
    if expected_features != actual:
        missing = sorted(expected_features - actual)
        unexpected = sorted(actual - expected_features)
        raise CoverageFailure(
            f"integration guide feature coverage differs: missing={missing} unexpected={unexpected}"
        )
    return len(families), len(guides)


def main() -> int:
    try:
        guide_document = GUIDE_MAP.read_text()
        route_rows = table_rows(guide_document, "Route Guides", 4)
        feature_rows = table_rows(guide_document, "Feature Guide Families", 4)
        guide_index = indexed_guides()
        route_guides, route_examples = validate_route_guides(
            route_rows, production_routes(), guide_index
        )
        feature_families, feature_guides = validate_feature_guides(
            feature_rows, portable_feature_inventory(), guide_index
        )
    except (CoverageFailure, OSError, csv.Error) as error:
        print(f"integration guide coverage failed: {error}", file=sys.stderr)
        return 1

    print(
        "integration guide coverage passed: "
        f"{len(route_rows)} routes, {route_guides} route guides, "
        f"{route_examples} examples, {feature_families} feature families, "
        f"{feature_guides} feature guides, "
        f"{len(portable_feature_inventory())} portable features"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
