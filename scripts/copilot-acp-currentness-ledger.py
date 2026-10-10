#!/usr/bin/env python3
"""Validate the frozen Copilot ACP package and hop identity ledger."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
LEDGER = (
    ROOT
    / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-currentness-1.0.95/artifact-hop-ledger.json"
)
PREVIOUS = (
    ROOT
    / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/artifact-inventory.json"
)
EXPECTED_LEDGER_SHA256 = "66aaa0480237235fc7ee526ece213b800f2f57724e26da836956841d1d97c4e6"
VERSIONS = tuple(f"1.0.{minor}" for minor in range(80, 96))
PACKAGES = ("@github/copilot", "@github/copilot-darwin-arm64")
CLASSIFICATIONS = {
    "distribution-identity-metadata",
    "unmapped-documentation-or-asset",
    "provider-owned-content-unmapped",
    "runtime-dependency-metadata-unresolved",
    "type-declaration-unmapped",
    "wrapper-launch-code",
    "opaque-cli-executable",
    "cli-application-code-unresolved",
    "runtime-code-or-component-unresolved",
    "runtime-data-or-provider-internal-unresolved",
}


def fail(message: str) -> None:
    raise SystemExit(message)


def manifest_sha256(files: list[dict[str, Any]]) -> str:
    body = "".join(
        f"{item['path']}\0{item['kind']}\0{item['size']}\0{item['mode']}\0{item['sha256']}\n"
        for item in files
    ).encode()
    return hashlib.sha256(body).hexdigest()


def index_packages(ledger: dict[str, Any]) -> dict[str, dict[str, dict[str, Any]]]:
    return {
        package["name"]: {version["version"]: version for version in package["versions"]}
        for package in ledger["packages"]
    }


def validate() -> dict[str, Any]:
    raw = LEDGER.read_bytes()
    if hashlib.sha256(raw).hexdigest() != EXPECTED_LEDGER_SHA256:
        fail("Copilot currentness ledger changed without an audited fixture update")
    ledger = json.loads(raw)
    if ledger.get("schema") != "copilot-cli-acp-currentness-artifact-ledger.v1":
        fail("unexpected currentness ledger schema")
    if ledger.get("official_stable") != "1.0.95" or ledger.get("official_prerelease") != "1.0.96-2":
        fail("official currentness point changed")
    if ledger.get("baseline") != "1.0.80" or ledger.get("platform") != "darwin-arm64":
        fail("currentness axis or baseline changed")
    if ledger.get("claim_at_observation") != {
        "qualified": "1.0.80",
        "posture": "QualifiedOnly",
        "claim_change": "none",
    }:
        fail("the frozen record must preserve the 1.0.80 claim")
    if ledger.get("published_stable_hops") != list(VERSIONS[1:]):
        fail("published stable hop set changed")

    packages = index_packages(ledger)
    if tuple(sorted(packages)) != tuple(sorted(PACKAGES)):
        fail("package identity set changed")
    if len(ledger["packages"]) != len(PACKAGES):
        fail("duplicate package identity")

    for name in PACKAGES:
        by_version = packages[name]
        if tuple(by_version) != VERSIONS:
            fail(f"version set changed for {name}")
        for version in VERSIONS:
            record = by_version[version]
            dist = record["dist"]
            files = record["files"]
            paths = [entry["path"] for entry in files]
            if paths != sorted(paths) or len(paths) != len(set(paths)):
                fail(f"file paths are not an exact sorted set for {name}@{version}")
            if len(files) != dist["fileCount"] or sum(item["size"] for item in files) != dist["unpackedSize"]:
                fail(f"file count or unpacked size changed for {name}@{version}")
            if manifest_sha256(files) != record["inventory_sha256"]:
                fail(f"complete tree digest mismatch for {name}@{version}")
            if len(record["archive_sha256"]) != 64 or not record["dist"]["integrity"].startswith("sha512-"):
                fail(f"artifact digest missing for {name}@{version}")
            if not dist.get("tarball", "").startswith("https://registry.npmjs.org/"):
                fail(f"non-official artifact source for {name}@{version}")

    previous = json.loads(PREVIOUS.read_text())
    old_packages = {
        (package["name"], version["version"]): version
        for package in previous["packages"]
        for version in package["versions"]
    }
    for name in PACKAGES:
        for version in ("1.0.80", "1.0.81", "1.0.93"):
            old = old_packages[(name, version)]
            new = packages[name][version]
            for field in ("archive_sha256", "inventory_sha256", "archive_bytes"):
                if old[field] != new[field]:
                    fail(f"accepted identity changed for {name}@{version}: {field}")

    versions_index = {name: packages[name] for name in PACKAGES}
    expected_hops = []
    for left, right in zip(VERSIONS, VERSIONS[1:]):
        hop = next((item for item in ledger["hops"] if item["from"] == left and item["to"] == right), None)
        if hop is None:
            fail(f"missing hop {left} to {right}")
        unresolved = False
        if set(hop["packages"]) != set(PACKAGES):
            fail(f"package delta set changed for {left} to {right}")
        for name in PACKAGES:
            before = {item["path"]: item for item in versions_index[name][left]["files"]}
            after = {item["path"]: item for item in versions_index[name][right]["files"]}
            delta = hop["packages"][name]
            added = sorted(after.keys() - before.keys())
            removed = sorted(before.keys() - after.keys())
            changed = sorted(path for path in before.keys() & after.keys() if before[path]["sha256"] != after[path]["sha256"])
            identical = sorted(path for path in before.keys() & after.keys() if before[path]["sha256"] == after[path]["sha256"])
            if delta["added"] != added or delta["removed"] != removed or delta["changed"] != changed or delta["identical"] != identical:
                fail(f"per-hop file set drift for {name} {left} to {right}")
            classified = delta["changed_classification"]
            expected_paths = sorted(set(added + removed + changed))
            if [row["path"] for row in classified] != expected_paths:
                fail(f"changed-file classifications incomplete for {name} {left} to {right}")
            for row in classified:
                if row["class"] not in CLASSIFICATIONS or not row.get("selected_surface"):
                    fail(f"missing selected-surface classification for {row['path']}")
                if row["class"].endswith("unresolved") or row["class"] == "opaque-cli-executable":
                    unresolved = True
        expected_status = "unresolved-runtime-delta" if unresolved else "selected-runtime-byte-identity-with-metadata-or-doc-changes"
        if hop["selected_surface_status"] != expected_status:
            fail(f"selected-surface status mismatch for {left} to {right}")
        expected_hops.append({"from": left, "to": right, "status": expected_status})

    if len(ledger["hops"]) != len(VERSIONS) - 1:
        fail("hop ledger has extra or missing entries")
    wrapper_loader = [packages["@github/copilot"][v]["files"] for v in VERSIONS]
    loader_hashes = [next(item["sha256"] for item in files if item["path"] == "package/npm-loader.js") for files in wrapper_loader]
    if len(set(loader_hashes)) != 1:
        fail("wrapper loader changed across the selected package window")
    if not all(item["status"] == "unresolved-runtime-delta" for item in expected_hops):
        fail("expected every published hop to retain an unresolved runtime gate")
    return {
        "status": "passed",
        "ledger_sha256": EXPECTED_LEDGER_SHA256,
        "official_stable": ledger["official_stable"],
        "published_hops": len(expected_hops),
        "complete_package_trees": len(PACKAGES) * len(VERSIONS),
        "wrapper_loader_byte_identical": True,
        "runtime_hops_unresolved": len(expected_hops),
        "claim_change": "none",
    }


if __name__ == "__main__":
    print(json.dumps(validate(), sort_keys=True))
