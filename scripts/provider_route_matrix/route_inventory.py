"""Authoritative production route inventory derived from the feature matrix.

The feature-matrix CSV is the single source for route identity. Every
consumer of the 34-route inventory derives from this module: the shell
route-matrix check, the integration-guide and activity-matrix parsers, and
the consumer front door. Provider-session lifecycle posture is per-route
data that must be supplied explicitly; missing entries fail loudly.
"""

from __future__ import annotations

import csv
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
FEATURE_MATRIX = REPO / "docs" / "guides" / "provider-solution-feature-matrix.csv"
EXPECTED_ROUTE_COUNT = 34

# Provider-session lifecycle posture per route:
# (persistent-session posture, management binding, archive, restore, delete,
# deletion strength). Data, not inventory: a new route must gain an explicit
# entry here or the check fails.
LIFECYCLE_POSTURES: dict[str, tuple[str, str, str, str, str, str]] = {
    "antigravity.catalogue": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "antigravity.headless": ("unsupported", "no", "unsupported", "unsupported", "unsupported", "unsupported"),
    "alibaba.conversations": ("supported", "yes", "unsupported", "unsupported", "supported", "ProviderDataDeleted"),
    "anthropic.managed-agent": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "anthropic.messages": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "bedrock.catalogue": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "bedrock.runtime": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "claude-agent.acp": ("supported", "yes", "unsupported", "unsupported", "supported", "ProviderDataDeleted"),
    "claude-code.headless": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "codex.app-server": ("supported", "yes", "supported", "supported", "supported", "ProviderHardDeleted"),
    "codex.exec": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "cursor-agent.acp": ("unsupported", "no", "unsupported", "unsupported", "unsupported", "unsupported"),
    "cursor-agent.catalogue": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "cursor-agent.headless": ("unsupported", "no", "unsupported", "unsupported", "unsupported", "unsupported"),
    "deepseek.continuation": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "gemini-cli.acp": ("unsupported", "no", "unsupported", "unsupported", "unsupported", "unsupported"),
    "gemini-cli.headless": ("unsupported", "no", "unsupported", "unsupported", "unsupported", "unsupported"),
    "gemini.live": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "grok-build.acp": ("unsupported", "no", "unsupported", "unsupported", "unsupported", "unsupported"),
    "kimi-code.acp": ("unsupported", "no", "unsupported", "unsupported", "unsupported", "unsupported"),
    "kimi-code.headless": ("unsupported", "no", "unsupported", "unsupported", "unsupported", "unsupported"),
    "kimi-code.local-server": ("supported", "yes", "supported", "supported", "unsupported", "unsupported"),
    "kimi-platform.chat": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "llama-cpp.attached": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "llama-cpp.owned": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "muse-code.headless": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "ollama.attached": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "openai.background": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "openai.realtime": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "opencode.http": ("supported", "yes", "unsupported", "unsupported", "supported", "ProviderDataDeleted"),
    "oh-my-pi.rpc": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "pi.rpc": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "qwen.headless": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
    "xai.responses-websocket": ("not-applicable", "no", "not-applicable", "not-applicable", "not-applicable", "not-applicable"),
}


def production_routes() -> list[str]:
    """Returns the 34 production route identities from the feature matrix."""
    with FEATURE_MATRIX.open(newline="", encoding="utf-8") as source:
        rows = list(csv.DictReader(source))
    routes = [
        route
        for row in rows
        for route in re.split(r"\s*(?:;|\+)\s*", row["route_id"])
    ]
    if len(routes) != EXPECTED_ROUTE_COUNT or len(set(routes)) != EXPECTED_ROUTE_COUNT:
        raise SystemExit(
            "provider solution feature matrix must cover "
            f"{EXPECTED_ROUTE_COUNT} unique route identities, found {len(routes)}"
        )
    return routes


def lifecycle_posture_rows() -> list[str]:
    """Returns sorted `route|posture|...` rows for every production route."""
    rows = []
    for route in production_routes():
        posture = LIFECYCLE_POSTURES.get(route)
        if posture is None:
            raise SystemExit(
                f"provider session lifecycle posture is missing for route: {route}"
            )
        rows.append("|".join((route, *posture)))
    return sorted(rows)


if __name__ == "__main__":
    if "--lifecycle-postures" in sys.argv:
        for row in lifecycle_posture_rows():
            print(row)
    else:
        for route in production_routes():
            print(route)
