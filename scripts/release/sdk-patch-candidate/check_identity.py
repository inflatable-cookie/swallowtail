#!/usr/bin/env python3
"""Check the exact released-line SDK patch candidate identity."""

from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
MANIFEST = json.loads((SCRIPT_DIR / "manifest.json").read_text(encoding="utf-8"))


def fail(message: str) -> None:
    raise SystemExit(f"SDK patch candidate identity failed: {message}")


def check(root: Path) -> None:
    version = MANIFEST["candidate_version"]
    cargo_path = root / "Cargo.toml"
    try:
        workspace_text = cargo_path.read_text(encoding="utf-8")
    except OSError as error:
        fail(f"workspace manifest is unavailable: {error}")

    def toml_section(contents: str, name: str) -> str:
        header = re.search(rf"(?m)^\[{re.escape(name)}\]\s*$", contents)
        if header is None:
            return ""
        remainder = contents[header.end() :]
        next_header = re.search(r"(?m)^\[", remainder)
        return remainder if next_header is None else remainder[: next_header.start()]

    package = toml_section(workspace_text, "workspace.package")
    workspace_version = re.search(r'(?m)^\s*version\s*=\s*"([^"]+)"\s*$', package)
    if workspace_version is None or workspace_version.group(1) != version:
        fail(f"workspace package version must be {version}")

    dependencies = toml_section(workspace_text, "workspace.dependencies")
    for line in dependencies.splitlines():
        match = re.match(r'\s*(swallowtail-[\w-]+)\s*=\s*\{(.*)\}\s*$', line)
        if match is None or "path" not in match.group(2):
            continue
        dependency_version = re.search(r'version\s*=\s*"([^"]+)"', match.group(2))
        if dependency_version is None or dependency_version.group(1) != version:
            fail(f"internal workspace dependency {match.group(1)} must be {version}")

    adapter_path = root / "crates/swallowtail-adapter-claude-agent/Cargo.toml"
    try:
        adapter_text = adapter_path.read_text(encoding="utf-8")
    except OSError as error:
        fail(f"Claude adapter manifest is unavailable: {error}")
    adapter_package = toml_section(adapter_text, "package")
    if not re.search(r"(?m)^\s*version\.workspace\s*=\s*true\s*$", adapter_package):
        fail("Claude adapter package must inherit the workspace version")

    sdk_path = root / "crates/swallowtail-adapter-claude-agent/src/sdk.rs"
    try:
        sdk = sdk_path.read_text(encoding="utf-8")
    except OSError as error:
        fail(f"Claude SDK identity source is unavailable: {error}")
    for expected in (
        'CLAUDE_AGENT_SDK_VERSION: &str = "0.3.259"',
        'CLAUDE_AGENT_SDK_NATIVE_VERSION: &str = "2.1.259"',
        'CLAUDE_AGENT_SDK_NODE_RUNTIME: &str = "22.23.2"',
    ):
        if expected not in sdk:
            fail(f"released SDK identity changed or disappeared: {expected}")

    asset_path = root / "crates/swallowtail-adapter-claude-agent/src/sdk/asset.rs"
    try:
        asset = asset_path.read_text(encoding="utf-8")
    except OSError as error:
        fail(f"sidecar asset declaration is unavailable: {error}")
    include = re.compile(
        r'include_str!\(\s*"../../sidecar/claude-agent-sdk-sidecar\.mjs"\s*\)'
    )
    source_tag = re.compile(
        r'concat!\(\s*"swallowtail-claude-agent-sdk-sidecar@",\s*'
        r'env!\("CARGO_PKG_VERSION"\)\s*\)'
    )
    if not include.search(asset):
        fail("embedded sidecar source path changed or disappeared")
    if not source_tag.search(asset):
        fail("sidecar source tag must derive from CARGO_PKG_VERSION")

    sidecar_path = root / "crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs"
    try:
        sidecar = sidecar_path.read_bytes()
    except OSError as error:
        fail(f"sidecar source bytes are unavailable: {error}")
    actual_digest = hashlib.sha256(sidecar).hexdigest()
    expected_digest = MANIFEST["sidecar_source_sha256"]
    if actual_digest != expected_digest:
        fail(
            "sidecar source bytes differ from the reviewed candidate asset: "
            f"expected {expected_digest}, found {actual_digest}"
        )


if len(sys.argv) != 2:
    raise SystemExit("usage: check_identity.py CANDIDATE_ROOT")

check(Path(sys.argv[1]).resolve())
print("candidate version, SDK pins, source tag, and embedded sidecar source identity passed")
