#!/usr/bin/env python3
"""Validate source-install copy against the package and route inventories."""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

# Keep route-inventory imports from writing __pycache__ under scripts/.
sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "scripts"))
from provider_route_matrix.route_inventory import (  # noqa: E402
    production_routes as inventory_production_routes,
)
REPOSITORY = "https://github.com/inflatable-cookie/swallowtail"


def fail(message: str) -> None:
    print(f"consumer front-door check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_version_identity() -> tuple[str, str]:
    completed = subprocess.run(
        ["bash", str(ROOT / "scripts/release-version-identity.sh")],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        fail(completed.stderr.strip() or "release-version-identity.sh failed")
    values = {}
    for line in completed.stdout.splitlines():
        key, _, value = line.partition("=")
        if key and value:
            values[key] = value
    current = values.get("current")
    previous = values.get("previous")
    if not current or not previous:
        fail("release-version-identity.sh did not report current and previous")
    return current, previous


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


current_version, previous_version = load_version_identity()
release_tag = f"v{current_version}"
previous_tag = f"v{previous_version}"
release_relative = f"docs/releases/{current_version}.md"
packages_relative = f"release-baselines/public-api-{current_version}/packages.txt"
routes_relative = f"release-baselines/production-routes-{current_version}.txt"

readme = read("README.md")
release = read(release_relative)
changelog = read("CHANGELOG.md")

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
    rf'tag = "{re.escape(release_tag)}" \}}$'
)
dependencies = {}
for line in install_lines:
    match = dependency_pattern.fullmatch(line)
    if match is None:
        fail(f"source-install dependency is not an exact Git-tag pin: {line!r}")
    dependencies[match.group(1)] = line
if set(dependencies) != expected_dependencies:
    fail("source-install example does not contain the expected direct package set")

expected_packages = set(read(packages_relative).splitlines())
release_package_section = section(release, "## Package Set", "## Production Routes")
documented_packages = set(re.findall(r"`(swallowtail-[a-z0-9-]+)`", release_package_section))
if not documented_packages:
    if (
        f"The 40-package set is unchanged from `{previous_tag}`." not in release_package_section
        or f"packages listed in the `{previous_tag}` release note." not in release_package_section
    ):
        fail("release package section has no package inventory or unchanged-set evidence")
    prior_release = read(f"docs/releases/{previous_version}.md")
    prior_package_section = section(
        prior_release, "## Package Set", "## Production Routes"
    )
    documented_packages = set(
        re.findall(r"`(swallowtail-[a-z0-9-]+)`", prior_package_section)
    )
if documented_packages != expected_packages:
    missing = sorted(expected_packages - documented_packages)
    extra = sorted(documented_packages - expected_packages)
    fail(f"release package inventory drifted; missing={missing}, extra={extra}")

current_routes = set(inventory_production_routes())
expected_routes = set(read(routes_relative).splitlines())
if current_routes != expected_routes:
    missing = sorted(expected_routes - current_routes)
    extra = sorted(current_routes - expected_routes)
    fail(f"current source route inventory drifted; missing={missing}, extra={extra}")
release_route_section = section(release, "## Production Routes", "## Highlights")
documented_routes = set(re.findall(r"^- `([^`]+)`$", release_route_section, re.MULTILINE))
if not documented_routes:
    if (
        f"The 49-route candidate inventory is unchanged from `{previous_tag}`, including"
        not in release_route_section
        or "Research 286 confirms that no route" not in release_route_section
        or "was renamed or removed." not in release_route_section
    ):
        fail("release route section has no route inventory or unchanged-set evidence")
    prior_release = read(f"docs/releases/{previous_version}.md")
    prior_route_section = section(
        prior_release, "## Production Routes", "## Highlights"
    )
    documented_routes = set(
        re.findall(r"^- `([^`]+)`$", prior_route_section, re.MULTILINE)
    )
if documented_routes != expected_routes:
    missing = sorted(expected_routes - documented_routes)
    extra = sorted(documented_routes - expected_routes)
    fail(f"release route inventory drifted; missing={missing}, extra={extra}")

for relative, document in (
    ("README.md", readme),
    (release_relative, release),
):
    if REPOSITORY not in document or release_tag not in document:
        fail(f"{relative} omits the canonical repository or exact release tag")

if release_relative not in changelog and not re.search(
    rf"^## \[{re.escape(current_version)}\](?: - .*)?$", changelog, re.MULTILINE
):
    fail(f"CHANGELOG.md does not reference the {current_version} release notes")
if "security/advisories/new" not in read("SECURITY.md"):
    fail("SECURITY.md does not name the private reporting path")

print(
    "consumer front door passed: "
    f"{len(expected_packages)} release packages, {len(expected_routes)} release routes, "
    "exact source tag"
)
