#!/usr/bin/env python3
"""Download frozen public Copilot CLI artifacts into a fresh temporary tree."""

from __future__ import annotations

import argparse
import base64
import hashlib
import json
import os
import tarfile
import tempfile
import urllib.request
from pathlib import Path, PurePosixPath
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
INVENTORY = (
    ROOT
    / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/artifact-inventory.json"
)
VERSIONS = ("1.0.80", "1.0.81", "1.0.93")


def inventory_packages(data: dict[str, Any]) -> dict[tuple[str, str], dict[str, Any]]:
    return {
        (package["name"], version["version"]): version
        for package in data["packages"]
        for version in package["versions"]
    }


def download(url: str, target: Path) -> tuple[str, str]:
    digest256 = hashlib.sha256()
    digest512 = hashlib.sha512()
    request = urllib.request.Request(
        url,
        headers={"User-Agent": "swallowtail-copilot-acp-offline-proof"},
    )
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    with opener.open(request, timeout=60) as response, target.open("wb") as output:
        while True:
            chunk = response.read(1024 * 1024)
            if not chunk:
                break
            output.write(chunk)
            digest256.update(chunk)
            digest512.update(chunk)
        output.flush()
        os.fsync(output.fileno())
    return digest256.hexdigest(), "sha512-" + base64.b64encode(digest512.digest()).decode()


def extract_and_check(
    archive: Path,
    destination: Path,
    expected: dict[str, Any],
) -> None:
    found: list[dict[str, Any]] = []
    names: set[str] = set()
    destination.mkdir(parents=True, exist_ok=False)
    with tarfile.open(archive, "r:gz") as source:
        members = source.getmembers()
        if len(members) != expected["dist"]["fileCount"]:
            raise ValueError("package tarball file count differs from frozen identity")
        for member in members:
            path = PurePosixPath(member.name)
            if (
                path.is_absolute()
                or ".." in path.parts
                or "\\" in member.name
                or not member.name.startswith("package/")
                or member.name in names
                or not member.isfile()
            ):
                raise ValueError(f"package tarball contains an unapproved entry: {member.name!r}")
            names.add(member.name)
            target = destination.joinpath(*path.parts[1:])
            target.parent.mkdir(parents=True, exist_ok=True)
            contents = source.extractfile(member)
            if contents is None:
                raise ValueError(f"package tarball has no content for {member.name!r}")
            digest = hashlib.sha256()
            size = 0
            with contents, target.open("wb") as output:
                while True:
                    chunk = contents.read(1024 * 1024)
                    if not chunk:
                        break
                    size += len(chunk)
                    digest.update(chunk)
                    output.write(chunk)
            if size != member.size:
                raise ValueError(f"package tarball size mismatch for {member.name!r}")
            os.chmod(target, member.mode)
            found.append(
                {
                    "path": member.name,
                    "kind": "file",
                    "size": size,
                    "mode": f"{member.mode:04o}",
                    "sha256": digest.hexdigest(),
                }
            )
    found.sort(key=lambda item: item["path"])
    if found != expected["files"]:
        raise ValueError("package file inventory differs from the frozen identity")
    if sum(item["size"] for item in found) != expected["dist"]["unpackedSize"]:
        raise ValueError("package unpacked size differs from the frozen identity")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--parent",
        type=Path,
        help="existing temp directory to contain a new unique staging directory",
    )
    args = parser.parse_args()
    inventory = json.loads(INVENTORY.read_text())
    packages = inventory_packages(inventory)
    parent = args.parent.resolve(strict=True) if args.parent else None
    if parent and not parent.is_relative_to(Path(tempfile.gettempdir()).resolve()):
        raise ValueError("staging parent must be inside the system temp directory")
    scratch = Path(
        tempfile.mkdtemp(prefix="copilot-acp-offline-artifacts-", dir=parent)
    ).resolve()
    downloads = scratch / "downloads"
    wrapper_root = scratch / "wrapper"
    artifact_root = scratch / "platform"
    for directory in (downloads, wrapper_root, artifact_root):
        directory.mkdir()

    for name, output_root in (
        ("@github/copilot", wrapper_root),
        ("@github/copilot-darwin-arm64", artifact_root),
    ):
        for version in VERSIONS:
            expected = packages[(name, version)]
            tarball = expected["dist"]["tarball"]
            archive = downloads / f"{name.rsplit('/', 1)[-1]}-{version}.tgz"
            sha256, integrity = download(tarball, archive)
            if sha256 != expected["archive_sha256"]:
                raise ValueError(f"downloaded archive SHA-256 mismatch for {name}@{version}")
            if integrity != expected["dist"]["integrity"]:
                raise ValueError(f"downloaded npm integrity mismatch for {name}@{version}")
            extract_and_check(archive, output_root / version, expected)

    print(
        json.dumps(
            {
                "scratch_root": str(scratch),
                "artifact_root": str(artifact_root),
                "wrapper_root": str(wrapper_root),
                "versions": list(VERSIONS),
                "network_use": "public pinned npm tarballs only; artifact execution remains network-denied",
                "installed": False,
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
