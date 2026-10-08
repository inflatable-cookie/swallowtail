#!/usr/bin/env python3
"""Exercise patch-equal, additive, empty, and removal route transitions."""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from provider_route_matrix.release_inventory import (  # noqa: E402
    validate_behavior_ledger_snapshot,
    validate_non_decreasing_routes,
)


def must_refuse(previous: set[str], current: set[str], message: str) -> None:
    try:
        validate_non_decreasing_routes(previous, current)
    except ValueError as error:
        if message not in str(error):
            raise AssertionError(f"expected refusal containing {message!r}, found {error!r}")
    else:
        raise AssertionError(f"expected refusal containing {message!r}")


frozen = {"route.a", "route.b"}
assert validate_non_decreasing_routes(frozen, set(frozen)) == set()
assert validate_non_decreasing_routes(frozen, frozen | {"route.c"}) == {"route.c"}
must_refuse(frozen, set(), "current route inventory is empty")
must_refuse(frozen, {"route.a"}, "removes frozen routes")

assert (
    validate_behavior_ledger_snapshot(
        {"route.a", "route.b"},
        {"route.a"},
        {"route.a", "route.b", "route.c"},
        {
            "0.5.0": {"route.a", "route.b"},
            "0.5.1": {"route.a", "route.b", "route.c"},
        },
        "0.5.1",
    )
    == "0.5.0"
)
try:
    validate_behavior_ledger_snapshot(
        {"route.a", "route.b"},
        {"route.a"},
        {"route.a", "route.b", "route.c"},
        {"0.5.1": {"route.a", "route.b", "route.c"}},
        "0.5.1",
    )
except ValueError as error:
    assert "does not match a frozen prior inventory" in str(error)
else:
    raise AssertionError("expected a ledger without a matching frozen snapshot to be refused")
print("release route inventory transition fixtures passed")
