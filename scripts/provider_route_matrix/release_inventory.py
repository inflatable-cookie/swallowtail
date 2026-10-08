"""Compatibility rules for working route inventories against the last release."""

from __future__ import annotations


def _version_key(version: str) -> tuple[int, int, int]:
    parts = version.split(".")
    if len(parts) != 3 or not all(part.isdigit() for part in parts):
        raise ValueError(f"invalid route inventory version: {version}")
    return tuple(int(part) for part in parts)  # type: ignore[return-value]


def validate_non_decreasing_routes(
    immutable_routes: set[str], current_routes: set[str]
) -> set[str]:
    """Return candidate additions while rejecting empty or shrinking inventories."""
    if not current_routes:
        raise ValueError("current route inventory is empty")
    removed = immutable_routes - current_routes
    if removed:
        raise ValueError(f"current route inventory removes frozen routes: {sorted(removed)}")
    return current_routes - immutable_routes


def validate_behavior_ledger_snapshot(
    ledger_routes: set[str],
    historical_routes: set[str],
    current_routes: set[str],
    snapshot_inventories: dict[str, set[str]],
    previous_version: str,
) -> str:
    """Identify the frozen baseline represented by a historical behavior ledger."""
    if not ledger_routes:
        raise ValueError("route behavior ledger is empty")
    if not historical_routes <= ledger_routes:
        raise ValueError(
            f"route behavior ledger omits historical routes: {sorted(historical_routes - ledger_routes)}"
        )
    if not ledger_routes <= current_routes:
        raise ValueError(
            f"route behavior ledger names routes absent from current source: {sorted(ledger_routes - current_routes)}"
        )
    previous_key = _version_key(previous_version)
    matching_versions = sorted(
        (
            version
            for version, routes in snapshot_inventories.items()
            if _version_key(version) <= previous_key and routes == ledger_routes
        ),
        key=_version_key,
    )
    if not matching_versions:
        raise ValueError("route behavior ledger does not match a frozen prior inventory")
    return matching_versions[-1]
