#!/usr/bin/env python3
"""Validate the provider-solution observable-activity inventory."""

from __future__ import annotations

import csv
import json
import re
import sys
from collections import Counter
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO_ROOT / "scripts"))
from provider_route_matrix.route_inventory import (  # noqa: E402
    production_routes as inventory_production_routes,
)
MATRIX = REPO_ROOT / "docs/guides/provider-solution-activity-matrix.csv"
ROUTE_MATRIX = REPO_ROOT / "docs/guides/provider-route-matrix.md"
HARNESS_INVENTORY = (
    REPO_ROOT
    / "crates/swallowtail-testkit/tests/fixtures/provider-wide-harness-activity.json"
)
DIRECT_INVENTORY = (
    REPO_ROOT
    / "crates/swallowtail-testkit/tests/fixtures/direct-activity-applicability.json"
)

HEADERS = [
    "provider",
    "solution",
    "route_scope",
    "route_id",
    "operation_shape",
    "activity_profile",
    "assistant_intermediate",
    "assistant_final",
    "reasoning_summary",
    "plan",
    "provider_tool_lifecycle",
    "consumer_tool_lifecycle",
    "tool_display_input",
    "tool_display_output",
    "tool_correlation",
    "command_output",
    "file_changes",
    "external_search",
    "image_view",
    "tasks",
    "hooks",
    "subagents",
    "subagent_observation",
    "subagent_parentage",
    "child_activity_attribution",
    "provider_collaboration_actions",
    "operator_control",
    "lifecycle_fidelity",
    "disclosure_fidelity",
    "unknown_event_posture",
    "prepared_entry",
    "conformance_test",
    "evidence_ref",
]

CONTENT_COLUMNS = {
    "assistant_intermediate",
    "assistant_final",
    "reasoning_summary",
    "plan",
    "tool_display_input",
    "tool_display_output",
    "command_output",
    "file_changes",
    "external_search",
    "image_view",
    "tasks",
    "hooks",
    "subagents",
}
LIFECYCLE_COLUMNS = {
    "provider_tool_lifecycle",
    "consumer_tool_lifecycle",
    "lifecycle_fidelity",
}
TOPOLOGY_COLUMNS = {
    "subagent_observation",
    "subagent_parentage",
    "child_activity_attribution",
    "provider_collaboration_actions",
    "operator_control",
}

ALLOWED = {
    "route_scope": {"production", "auxiliary"},
    "activity_profile": {"available", "not-applicable"},
    **{
        column: {
            "provider-display",
            "adapter-summary",
            "identity-lifecycle",
            "profile-dependent",
            "unavailable",
            "not-applicable",
        }
        for column in CONTENT_COLUMNS
    },
    **{
        column: {
            "complete",
            "update-completion",
            "completion-only",
            "mixed-by-kind",
            "profile-dependent",
            "unavailable",
            "not-applicable",
        }
        for column in LIFECYCLE_COLUMNS
    },
    "tool_correlation": {
        "provider-item",
        "provider-request",
        "consumer-callback",
        "direct-tool-call",
        "operation-local",
        "profile-dependent",
        "unavailable",
        "not-applicable",
    },
    "disclosure_fidelity": {
        "provider-display",
        "adapter-summary",
        "identity-lifecycle",
        "mixed-by-kind",
        "profile-dependent",
        "not-applicable",
    },
    "unknown_event_posture": {
        "preserve-namespaced",
        "fail-closed",
        "profile-dependent",
        "not-applicable",
    },
    "subagent_observation": {
        "identity-lifecycle",
        "parent-and-metadata",
        "attributed-activity",
        "profile-dependent",
        "unavailable",
        "not-applicable",
    },
    "subagent_parentage": {
        "unknown",
        "operation",
        "nested-and-operation",
        "profile-dependent",
        "unavailable",
        "not-applicable",
    },
    "child_activity_attribution": {
        "available",
        "profile-dependent",
        "unavailable",
        "not-applicable",
    },
    "provider_collaboration_actions": {
        "spawn-send-resume-wait-close",
        "profile-dependent",
        "unavailable",
        "not-applicable",
    },
    "operator_control": {
        "unavailable",
        "not-applicable",
    },
}

PROFILE_COLUMNS = (
    CONTENT_COLUMNS
    | LIFECYCLE_COLUMNS
    | TOPOLOGY_COLUMNS
    | {"tool_correlation", "disclosure_fidelity", "unknown_event_posture"}
)

POSITIVE_TOPOLOGY = {
    ("antigravity.headless", "interactive-session"): {
        "subagent_observation": "identity-lifecycle",
        "subagent_parentage": "operation",
        "child_activity_attribution": "unavailable",
        "provider_collaboration_actions": "unavailable",
        "operator_control": "unavailable",
    },
    ("antigravity.headless", "structured-run"): {
        "subagent_observation": "identity-lifecycle",
        "subagent_parentage": "operation",
        "child_activity_attribution": "unavailable",
        "provider_collaboration_actions": "unavailable",
        "operator_control": "unavailable",
    },
    ("codex.app-server", "interactive-session"): {
        "subagent_observation": "profile-dependent",
        "subagent_parentage": "profile-dependent",
        "child_activity_attribution": "profile-dependent",
        "provider_collaboration_actions": "profile-dependent",
        "operator_control": "unavailable",
    },
    ("codex.exec", "structured-run"): {
        "subagent_observation": "profile-dependent",
        "subagent_parentage": "profile-dependent",
        "child_activity_attribution": "unavailable",
        "provider_collaboration_actions": "profile-dependent",
        "operator_control": "unavailable",
    },
    ("kimi-code.local-server", "interactive-session"): {
        "subagent_observation": "parent-and-metadata",
        "subagent_parentage": "operation",
        "child_activity_attribution": "unavailable",
        "provider_collaboration_actions": "unavailable",
        "operator_control": "unavailable",
    },
    ("kimi-code.local-server", "structured-run"): {
        "subagent_observation": "parent-and-metadata",
        "subagent_parentage": "operation",
        "child_activity_attribution": "unavailable",
        "provider_collaboration_actions": "unavailable",
        "operator_control": "unavailable",
    },
}


def load_json(path: Path) -> dict:
    with path.open(encoding="utf-8") as source:
        return json.load(source)


def expected_operations() -> dict[tuple[str, str], str]:
    expected: dict[tuple[str, str], str] = {}
    harness = load_json(HARNESS_INVENTORY)
    for route in harness["routes"]:
        for profile in route["prepared_profiles"]:
            expected[(route["id"], profile["operation"])] = profile["availability"]
        for operation in route["not_applicable_operations"]:
            expected[(route["id"], operation)] = "not-applicable"

    direct = load_json(DIRECT_INVENTORY)
    for route in direct["routes"]:
        for profile in route["ordinary_profiles"]:
            expected[(route["id"], profile["operation"])] = profile["applicability"]
        for operation in route["non_activity_operations"]:
            expected[(route["id"], operation["operation"])] = operation["applicability"]
    for catalogue in direct["auxiliary_catalogues"]:
        expected[(catalogue["id"], "model-catalogue")] = catalogue["applicability"]
    return expected


def production_routes() -> set[str]:
    routes = set(inventory_production_routes())
    source = ROUTE_MATRIX.read_text(encoding="utf-8")
    pre_lifecycle = source.split(
        "<!-- provider-session-lifecycle-matrix:start -->", maxsplit=1
    )[0]
    documented = set(re.findall(r"^\| `([^`]+)` \|", pre_lifecycle, flags=re.MULTILINE))
    if documented != routes:
        raise SystemExit(
            "provider route matrix route identities differ from the feature matrix"
        )
    return routes


def referenced_path(value: str, *, row_key: tuple[str, str], column: str) -> Path:
    relative = value.split("#", maxsplit=1)[0]
    if not relative:
        raise SystemExit(f"{row_key} has an empty {column}")
    path = REPO_ROOT / relative
    if not path.is_file():
        raise SystemExit(f"{row_key} {column} does not exist: {relative}")
    return path


def main() -> None:
    with MATRIX.open(newline="", encoding="utf-8") as source:
        reader = csv.DictReader(source)
        if reader.fieldnames != HEADERS:
            raise SystemExit(
                "provider activity matrix headers changed: "
                f"expected {HEADERS}, got {reader.fieldnames}"
            )
        rows = list(reader)

    expected = expected_operations()
    actual = {
        (row["route_id"], row["operation_shape"]): row["activity_profile"]
        for row in rows
    }
    if len(actual) != len(rows):
        raise SystemExit("provider activity matrix contains duplicate route operations")
    if actual != expected:
        missing = sorted(set(expected) - set(actual))
        extra = sorted(set(actual) - set(expected))
        changed = sorted(
            key
            for key in set(actual) & set(expected)
            if actual[key] != expected[key]
        )
        raise SystemExit(
            "provider activity operation inventory changed: "
            f"missing={missing}, extra={extra}, changed={changed}"
        )

    if len(rows) != 69:
        raise SystemExit("provider activity matrix must contain exactly 69 rows")
    counts = Counter(row["activity_profile"] for row in rows)
    if counts != Counter({"available": 43, "not-applicable": 26}):
        raise SystemExit(
            f"provider activity dispositions changed: {dict(counts)}"
        )

    order = [
        (
            row["provider"].casefold(),
            row["solution"].casefold(),
            row["route_id"],
            row["operation_shape"],
        )
        for row in rows
    ]
    if order != sorted(order):
        raise SystemExit(
            "provider activity matrix must be sorted by provider, solution, route, operation"
        )

    production = production_routes()
    direct = load_json(DIRECT_INVENTORY)
    auxiliary = {entry["id"] for entry in direct["auxiliary_catalogues"]}
    routes = {row["route_id"] for row in rows}
    if routes != production | auxiliary:
        raise SystemExit(
            "provider activity route identities changed: "
            f"missing={sorted((production | auxiliary) - routes)}, "
            f"extra={sorted(routes - (production | auxiliary))}"
        )

    for row in rows:
        key = (row["route_id"], row["operation_shape"])
        expected_scope = "production" if row["route_id"] in production else "auxiliary"
        if row["route_scope"] != expected_scope:
            raise SystemExit(
                f"{key} has route_scope={row['route_scope']}, expected {expected_scope}"
            )

        for column, values in ALLOWED.items():
            if row[column] not in values:
                raise SystemExit(
                    f"{key} has unsupported {column} value: {row[column]}"
                )

        prepared_path = referenced_path(
            row["prepared_entry"], row_key=key, column="prepared_entry"
        )
        referenced_path(
            row["conformance_test"], row_key=key, column="conformance_test"
        )
        evidence_path = referenced_path(
            row["evidence_ref"], row_key=key, column="evidence_ref"
        )
        if (
            not row["prepared_entry"].startswith("crates/swallowtail-adapter-")
            or "/src/" not in row["prepared_entry"]
            or "prepare" not in row["prepared_entry"].split("#", maxsplit=1)[-1].casefold()
        ):
            raise SystemExit(f"{key} does not link a public prepared entry point")
        if "protocol" in prepared_path.parts or "fixtures" in prepared_path.parts:
            raise SystemExit(f"{key} prepared path points at native parsing evidence")
        if evidence_path not in {HARNESS_INVENTORY, DIRECT_INVENTORY}:
            raise SystemExit(f"{key} does not link the canonical activity inventory")
        if f"#{row['route_id']}" not in row["evidence_ref"]:
            raise SystemExit(f"{key} evidence does not name its exact route identity")

        values = {row[column] for column in PROFILE_COLUMNS}
        if row["activity_profile"] == "not-applicable":
            if values != {"not-applicable"}:
                raise SystemExit(
                    f"{key} must mark every activity dimension not-applicable"
                )
        elif "not-applicable" in values:
            raise SystemExit(
                f"{key} is available and cannot use not-applicable feature cells"
            )

        if any(row[column] == "unavailable" for column in PROFILE_COLUMNS):
            if not row["evidence_ref"]:
                raise SystemExit(f"{key} has an unavailable cell without exact evidence")

        expected_topology = POSITIVE_TOPOLOGY.get(key)
        if expected_topology is None:
            disposition = (
                "not-applicable"
                if row["activity_profile"] == "not-applicable"
                else "unavailable"
            )
            expected_topology = {
                column: disposition for column in TOPOLOGY_COLUMNS
            }
        actual_topology = {
            column: row[column] for column in TOPOLOGY_COLUMNS
        }
        if actual_topology != expected_topology:
            raise SystemExit(
                f"{key} child-topology truth changed: "
                f"expected={expected_topology}, actual={actual_topology}"
            )

    print(
        "provider activity matrix passed: "
        "69 operations, 43 available, 26 not-applicable, "
        "35 production routes, 4 auxiliary catalogues, "
        "6 topology-capable operations, 0 operator-control operations"
    )


if __name__ == "__main__":
    main()
