#!/usr/bin/env python3
"""Validate source-install copy against the package and route inventories."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
REPOSITORY = "https://github.com/inflatable-cookie/swallowtail"
TAG = "v0.1.1"


def fail(message: str) -> None:
    print(f"consumer front-door check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def read(relative: str) -> str:
    path = ROOT / relative
    if not path.is_file():
        fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def section(document: str, start: str, end: str) -> str:
    pattern = rf"^{re.escape(start)}\n(?P<body>.*?)(?=^{re.escape(end)}\n)"
    match = re.search(pattern, document, re.MULTILINE | re.DOTALL)
    if match is None:
        fail(f"missing section boundary {start!r} -> {end!r}")
    return match.group("body")


readme = read("README.md")
release = read("docs/releases/0.1.1.md")
changelog = read("CHANGELOG.md")
matrix = read("docs/guides/provider-route-matrix.md")

for required in ("SECURITY.md", "SUPPORT.md", "CONTRIBUTING.md", "LICENSE"):
    read(required)

if len(readme.splitlines()) > 240:
    fail("README.md exceeds the 240-line consumer-front-door bound")

install_match = re.search(
    r"<!-- source-install:start -->\s*```toml\n(?P<toml>.*?)```\s*"
    r"<!-- source-install:end -->",
    readme,
    re.DOTALL,
)
if install_match is None:
    fail("README.md has no marked source-install TOML example")

expected_dependencies = {
    "swallowtail-core",
    "swallowtail-runtime",
    "swallowtail-host-local",
    "swallowtail-adapter-codex",
}
install_lines = [line for line in install_match.group("toml").splitlines() if line]
if not install_lines or install_lines.pop(0) != "[dependencies]":
    fail("source-install example does not start with a dependencies table")
dependency_pattern = re.compile(
    rf'^(swallowtail-[a-z0-9-]+) = \{{ git = "{re.escape(REPOSITORY)}", '
    rf'tag = "{re.escape(TAG)}" \}}$'
)
dependencies = {}
for line in install_lines:
    match = dependency_pattern.fullmatch(line)
    if match is None:
        fail(f"source-install dependency is not an exact Git-tag pin: {line!r}")
    dependencies[match.group(1)] = line
if set(dependencies) != expected_dependencies:
    fail("source-install example does not contain the expected direct package set")

expected_packages = set(
    read("release-baselines/public-api-0.1.0/packages.txt").splitlines()
)
release_package_section = section(release, "## Package Set", "## Production Routes")
documented_packages = set(re.findall(r"`(swallowtail-[a-z0-9-]+)`", release_package_section))
if documented_packages != expected_packages:
    missing = sorted(expected_packages - documented_packages)
    extra = sorted(documented_packages - expected_packages)
    fail(f"release package inventory drifted; missing={missing}, extra={extra}")

current_routes = set(
    re.findall(
        r"^\| `([^`]+)` \| `swallowtail-adapter-[^`]+`;",
        matrix,
        re.MULTILINE,
    )
)
expected_routes = set(read("release-baselines/production-routes-0.1.1.txt").splitlines())
if not expected_routes <= current_routes or "muse-code.headless" not in current_routes:
    fail("current route inventory lost tagged routes or the additive Muse route")
release_route_section = section(release, "## Production Routes", "## Highlights")
documented_routes = set(re.findall(r"^- `([^`]+)`$", release_route_section, re.MULTILINE))
if documented_routes != expected_routes:
    missing = sorted(expected_routes - documented_routes)
    extra = sorted(documented_routes - expected_routes)
    fail(f"release route inventory drifted; missing={missing}, extra={extra}")

for relative, document in (
    ("README.md", readme),
    ("docs/releases/0.1.1.md", release),
):
    if REPOSITORY not in document or TAG not in document:
        fail(f"{relative} omits the canonical repository or exact tag")

if "docs/releases/0.1.1.md" not in changelog:
    fail("CHANGELOG.md does not link to the current release notes")
if "security/advisories/new" not in read("SECURITY.md"):
    fail("SECURITY.md does not name the private reporting path")

print(
    "consumer front door passed: "
    f"{len(expected_packages)} tagged packages, {len(expected_routes)} tagged routes, "
    f"{len(current_routes)} current routes, exact source tag"
)
