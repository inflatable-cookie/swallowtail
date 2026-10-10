#!/usr/bin/env python3
"""Fake-test the reviewed normal-host Copilot ACP permission exchange."""

from __future__ import annotations

import argparse
import base64
import hashlib
import io
import json
import os
import platform
import re
import selectors
import signal
import stat
import subprocess
import sys
import tarfile
import tempfile
import time
import urllib.request
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
SCRIPT_PATH = Path(__file__).resolve()
FIXTURE_DIR = ROOT / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-host-permission-proof"
PLAN_PATH = FIXTURE_DIR / "plan.json"
SCHEMA_PATH = FIXTURE_DIR / "plan.schema.json"
CORRECTION_PLAN_PATH = FIXTURE_DIR / "corrected-attempt-proposal.json"
CORRECTION_SCHEMA_PATH = FIXTURE_DIR / "corrected-attempt-proposal.schema.json"
FINAL_CORRECTION_PLAN_PATH = FIXTURE_DIR / "corrected-execution-plan.json"
FINAL_CORRECTION_SCHEMA_PATH = FIXTURE_DIR / "corrected-execution-plan.schema.json"
AUTHORITY_PATH = FIXTURE_DIR / "original-execution-authority.json"
AUTHORITY_SCHEMA_PATH = FIXTURE_DIR / "original-execution-authority.schema.json"
CORRECTED_AUTHORITY_PATH = FIXTURE_DIR / "corrected-execution-authority.json"
CORRECTED_AUTHORITY_SCHEMA_PATH = FIXTURE_DIR / "corrected-execution-authority.schema.json"
NEXT_ATTEMPT_PROPOSAL_PATH = FIXTURE_DIR / "task-172-attempt-proposal.json"
NEXT_ATTEMPT_PROPOSAL_SCHEMA_PATH = FIXTURE_DIR / "task-172-attempt-proposal.schema.json"
ACP_PROTOCOL_FIXTURE_PATH = ROOT / "crates/swallowtail-protocol-acp/tests/fixtures/acp-schema-v1.24.1/protocol.json"
INVENTORY_PATH = ROOT / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/artifact-inventory.json"
CORRECTED_FAKE_PASS_PATH = ROOT / "docs/research/437-copilot-acp-corrected-normal-host-fake-pass.json"
CORRECTED_ATTEMPT_EVIDENCE_PATH = ROOT / "docs/research/437-copilot-acp-corrected-normal-host-attempt.json"
CORRECTED_PROMPT_EVIDENCE_PATH = ROOT / "docs/research/437-copilot-acp-corrected-normal-host-prompt-slot.json"
CORRECTED_EXECUTION_EVIDENCE_PATH = ROOT / "docs/research/437-copilot-acp-corrected-normal-host-execution.json"
ATTEMPT_EVIDENCE_NAME = "435-copilot-acp-normal-host-permission-attempt.json"
PROMPT_EVIDENCE_NAME = "435-copilot-acp-normal-host-permission-prompt-slot.json"
FRAME_LIMIT = 256 * 1024
STDERR_CAPTURE_LIMIT = 4096
STDERR_COUNT_LIMIT = 64 * 1024
MAX_OUTER_SECONDS = 60.0
MAX_CLEANUP_SECONDS = 3.0
PROMPT_TEXT = (
    "Overwrite the existing file permission-sentinel.txt in this working "
    "directory with exactly: SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1 followed by a newline."
)
SENTINEL_BEFORE = b"SWALLOWTAIL_PERMISSION_SENTINEL_BEFORE_V1\n"
SENTINEL_AFTER = b"SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n"
FAKE_SECRET = "SWALLOWTAIL_FAKE_STDERR_SECRET_SENTINEL"
FAKE_CONFIG = "SWALLOWTAIL_FAKE_CONFIG_SENTINEL"
FAKE_SCENARIOS = frozenset(
    {
        "success",
        "spontaneous-effect",
        "no-permission",
        "wrong-action",
        "wrong-kind",
        "missing-kind",
        "missing-action",
        "raw-input-only",
        "diff-action",
        "wrong-id",
        "wrong-session",
        "missing-tool-id",
        "unknown-tool-update",
        "in-progress-before-permission",
        "completed-before-permission",
        "failed-before-permission",
        "post-cancel-completed",
        "malformed-permission-options",
        "permission-end-turn",
        "duplicate-permission",
        "malformed-frame",
        "oversized-frame",
        "early-eof",
        "initialize-error",
        "hang-initialize",
        "hang-after-permission",
        "crash-before-prompt",
        "crash-after-prompt",
        "tool-call-without-permission",
        "wrong-version",
        "missing-version",
        "invalid-version",
        "oversized-version",
        "secret-version",
        "arbitrary-version",
        "nonstring-version",
        "session-new-error",
    }
)
PLAN_ID = "copilot-acp-normal-host-permission-1.0.93-v1"
CORRECTION_PLAN_ID = "copilot-acp-normal-host-permission-1.0.93-corrected-v1"
FINAL_CORRECTION_PLAN_ID = "copilot-acp-normal-host-permission-1.0.93-task-171-v1"
PREPARATION_RUNNER_SHA256 = "a7b1218d8ffc99e57c5d771be8ba022b69886afc453523115444432e1f323bdc"
TASK170_CORRECTION_RUNNER_SHA256 = "838beb490b903aafb46580e4ccde74d614a3f2bd326a754755548431ecf4a316"
CORRECTION_PROPOSAL_SHA256 = "1e42dcf2188bedeeb8bd5b0e0505b4303c864a0063b2e1f47deb94c1b951717e"
ORIGINAL_AUTHORITY_RUNNER_SHA256 = "42b1cd5a09e6a8cf80674f972079cbdae618c7e882f9fcfea1d662744b56aa4b"
HISTORICAL_AUTHORITY_SHA256 = "8df1d77fe399084345c0dd94c6b954428540d37e50380578542626fc0ad17eed"
BRIEF_SHA256 = "bdc07153187bde4f603b10e3f3a55d2b7e3b17bca23fcf7e8c7dd5afe2654a29"
CORRECTED_BRIEF_SHA256 = "5b69971124d97b98f8d95ef4ca274465fa80e80a317e1732e4886897536978f0"
CORRECTED_TASK_ID = "251ff078-07d9-4b7d-8015-c9446788250c"
CORRECTED_RUN_ID = "27ec8859-5efc-4511-a9b3-56e3710f3097"
CORRECTED_OPERATOR_DECISION = "acb7a075-390a-48b6-99b5-a6eea4d9920e"
TASK172_ID = "09baa0ef-09bc-45d8-a201-96149af03d7b"
TASK172_RUN_ID = "24b836d2-df09-4cc1-91b3-8f44454f9d2b"
TASK172_PROPOSAL_ID = "copilot-acp-normal-host-permission-1.0.93-task-172-v1"
FINAL_CORRECTED_ATTEMPT_RUNNER_SHA256 = "5d534c17d9e998636552f56c8aa3d0280bb6544ce55f63f71c2f057e17d58040"
EXPECTED_INVENTORY_SHA256 = "2d122117ccbb52dd547a783117ea3b1699df8e15357bca86a4d27a65416c8b0f"
EXPECTED_WRAPPER_ARCHIVE_SHA256 = "a8e704fb6874364af1b268aed2170bb597e0ca8086f3182b8fe5cb86ca3e43e1"
EXPECTED_NATIVE_ARCHIVE_SHA256 = "f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb"
EXPECTED_EXECUTABLE_SHA256 = "df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1"
EXPECTED_WRAPPER_LOADER_SHA256 = "0ea824a86be5757533fdb092eff7050871bd7a711a46babde0ffe0e44ac5ad88"
EXPECTED_WRAPPER_MANIFEST_SHA256 = "5f29c2061d18be20a8cb1677161359253a85bb49a7b4ea1e6c11db8ae1d9b64b"
EXPECTED_NATIVE_MANIFEST_SHA256 = "44de35dc12ce1b582678cf565d9f8ddddb786b00dd083373948d6e453e1eb729"
FORBIDDEN_PERSISTED_MARKERS = (
    FAKE_SECRET,
    FAKE_CONFIG,
    "GITHUB_TOKEN",
    "GH_TOKEN",
    "COPILOT_GITHUB_TOKEN",
    "-----BEGIN",
)
REPORTED_VERSION_MAX_LENGTH = 64
SEMVER_IDENTIFIER = r"(?:0|[1-9][0-9]*|[0-9A-Za-z-]*[A-Za-z-][0-9A-Za-z-]*)"
SEMVER_RE = re.compile(
    r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)"
    rf"(?:-{SEMVER_IDENTIFIER}(?:\.{SEMVER_IDENTIFIER})*)?"
    r"(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?"
)
VERSION_SECRET_MARKER_RE = re.compile(
    r"(?i)(?:token|secret|auth|credential|password|bearer|github_pat_|"
    r"gh[pors][_-]|xox[baprs]-)"
)


class ProtocolFailure(RuntimeError):
    def __init__(self, category: str):
        super().__init__(category)
        self.category = category


class BoundedDiagnostics:
    def __init__(self) -> None:
        self.total = 0
        self.captured = bytearray()
        self.capped = False

    def add(self, chunk: bytes) -> None:
        remaining = max(0, STDERR_COUNT_LIMIT - self.total)
        self.total += min(len(chunk), remaining)
        if len(chunk) > remaining:
            self.capped = True
        if len(self.captured) < STDERR_CAPTURE_LIMIT:
            take = STDERR_CAPTURE_LIMIT - len(self.captured)
            self.captured.extend(chunk[:take])

    def summary(self) -> dict[str, Any]:
        lowered = bytes(self.captured).lower()
        if not self.captured:
            category = "empty"
        elif b"dyld: library not loaded" in lowered:
            category = "dynamic-loader-marker"
        elif b"permission denied" in lowered or b"access denied" in lowered:
            category = "access-denied-marker"
        elif b"sandbox" in lowered:
            category = "sandbox-marker"
        else:
            category = "nonempty-unclassified"
        return {
            "category": category,
            "captured_bytes": len(self.captured),
            "total_bytes_capped": self.total,
            "count_limit_reached": self.capped,
            "raw_persisted": False,
            "raw_displayed": False,
        }


class StdioSession:
    """Read ACP stdout and drain stderr in one bounded selector loop."""

    def __init__(self, process: subprocess.Popen[bytes]):
        if process.stdout is None or process.stderr is None or process.stdin is None:
            raise ValueError("ACP child pipes were not created")
        self.process = process
        self.selector = selectors.DefaultSelector()
        self.stdout = process.stdout
        self.stderr = process.stderr
        self.stdin = process.stdin
        self.stdout_buffer = bytearray()
        self.stdout_eof = False
        self.stderr_eof = False
        self.stderr_diagnostics = BoundedDiagnostics()
        for stream, name in ((self.stdout, "stdout"), (self.stderr, "stderr")):
            os.set_blocking(stream.fileno(), False)
            self.selector.register(stream, selectors.EVENT_READ, name)

    def send(self, value: dict[str, Any]) -> None:
        encoded = json.dumps(value, separators=(",", ":"), ensure_ascii=True).encode() + b"\n"
        if len(encoded) > FRAME_LIMIT:
            raise ProtocolFailure("outbound-frame-oversized")
        try:
            self.stdin.write(encoded)
            self.stdin.flush()
        except (BrokenPipeError, OSError):
            raise ProtocolFailure("protocol-write-failure") from None

    def close_input(self) -> None:
        if not self.stdin.closed:
            try:
                self.stdin.close()
            except OSError:
                pass

    def _read_ready(self, remaining: float) -> None:
        if remaining <= 0:
            raise ProtocolFailure("timeout")
        events = self.selector.select(remaining)
        if not events:
            raise ProtocolFailure("timeout")
        for key, _ in events:
            stream = key.fileobj
            name = key.data
            try:
                chunk = os.read(stream.fileno(), 65536)
            except BlockingIOError:
                continue
            if not chunk:
                self.selector.unregister(stream)
                if name == "stdout":
                    self.stdout_eof = True
                else:
                    self.stderr_eof = True
                continue
            if name == "stdout":
                self.stdout_buffer.extend(chunk)
                newline = self.stdout_buffer.find(b"\n")
                if newline > FRAME_LIMIT or newline < 0 and len(self.stdout_buffer) > FRAME_LIMIT:
                    raise ProtocolFailure("inbound-frame-oversized")
            else:
                self.stderr_diagnostics.add(chunk)

    def receive(self, deadline: float) -> dict[str, Any]:
        while True:
            newline = self.stdout_buffer.find(b"\n")
            if newline >= 0:
                if newline > FRAME_LIMIT:
                    raise ProtocolFailure("inbound-frame-oversized")
                frame = bytes(self.stdout_buffer[:newline])
                del self.stdout_buffer[: newline + 1]
                try:
                    value = json.loads(frame)
                except (UnicodeDecodeError, json.JSONDecodeError):
                    raise ProtocolFailure("malformed-frame") from None
                if not isinstance(value, dict):
                    raise ProtocolFailure("malformed-frame")
                return value
            if self.stdout_eof:
                raise ProtocolFailure("unknown-eof")
            remaining = deadline - time.monotonic()
            self._read_ready(remaining)

    def drain_until(self, deadline: float) -> None:
        while (not self.stdout_eof or not self.stderr_eof) and time.monotonic() < deadline:
            remaining = deadline - time.monotonic()
            try:
                self._read_ready(remaining)
            except ProtocolFailure as failure:
                if failure.category != "timeout":
                    raise
                break

    def close(self) -> None:
        self.close_input()
        self.selector.close()
        self.stdout.close()
        self.stderr.close()


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds").replace("+00:00", "Z")


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def durable_create_once(path: Path, value: dict[str, Any]) -> bytes:
    """Create a private record without replacement, then fsync file and parent."""
    try:
        path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
    except FileExistsError:
        pass
    absolute_parent = path.parent.absolute()
    for component in (absolute_parent, *absolute_parent.parents):
        if component.is_symlink():
            raise RuntimeError("record path must not contain symlink components")
    directory_flags = os.O_RDONLY | getattr(os, "O_DIRECTORY", 0)
    if hasattr(os, "O_NOFOLLOW"):
        directory_flags |= os.O_NOFOLLOW
    parent_descriptor = os.open(absolute_parent, directory_flags)
    parent_stat = os.fstat(parent_descriptor)
    if (
        not os.path.isdir(absolute_parent)
        or parent_stat.st_uid != os.getuid()
        or parent_stat.st_mode & 0o077
    ):
        os.close(parent_descriptor)
        raise RuntimeError("record parent must be a private current-user directory")
    flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
    if hasattr(os, "O_NOFOLLOW"):
        flags |= os.O_NOFOLLOW
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":")).encode() + b"\n"
    try:
        descriptor = os.open(path.name, flags, 0o600, dir_fd=parent_descriptor)
        try:
            offset = 0
            while offset < len(encoded):
                offset += os.write(descriptor, encoded[offset:])
            os.fsync(descriptor)
        finally:
            os.close(descriptor)
        os.fsync(parent_descriptor)
    finally:
        os.close(parent_descriptor)
    return encoded


def load_object(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError(f"{path.name} must contain a JSON object")
    return value


def sanitized_reported_version(value: Any) -> str | None:
    if (
        not isinstance(value, str)
        or len(value) > REPORTED_VERSION_MAX_LENGTH
        or SEMVER_RE.fullmatch(value) is None
        or VERSION_SECRET_MARKER_RE.search(value) is not None
    ):
        return None
    return value


def classify_agent_version(value: Any, target: str = "1.0.93") -> tuple[str, str | None]:
    if value is None:
        return "unknown", None
    if not isinstance(value, str):
        return "invalid", None
    reported = sanitized_reported_version(value)
    if reported is None:
        return "invalid", None
    if reported == target:
        return f"matched-{target}", reported
    return "mismatch", reported


def plan_path_for(plan: dict[str, Any]) -> Path:
    if plan.get("plan_id") == PLAN_ID:
        return PLAN_PATH
    if plan.get("plan_id") == CORRECTION_PLAN_ID:
        return CORRECTION_PLAN_PATH
    if plan.get("plan_id") == FINAL_CORRECTION_PLAN_ID:
        return FINAL_CORRECTION_PLAN_PATH
    raise ValueError("plan identity is outside the reviewed Copilot attempt set")


def expected_runner_sha256(plan: dict[str, Any], execution_kind: str) -> str:
    if plan.get("plan_id") == PLAN_ID and execution_kind == "original":
        authority = load_object(AUTHORITY_PATH)
        if sha256_file(AUTHORITY_PATH) != HISTORICAL_AUTHORITY_SHA256:
            raise ValueError("historical authority identity changed")
        runner_sha256 = authority.get("final_runner_sha256")
        if runner_sha256 != ORIGINAL_AUTHORITY_RUNNER_SHA256:
            raise ValueError("historical authority runner identity changed")
        return runner_sha256
    if plan.get("plan_id") == CORRECTION_PLAN_ID:
        if plan.get("runner", {}).get("implementation_sha256") != TASK170_CORRECTION_RUNNER_SHA256:
            raise ValueError("historical corrected proposal runner identity changed")
        return TASK170_CORRECTION_RUNNER_SHA256
    if plan.get("plan_id") == FINAL_CORRECTION_PLAN_ID:
        return plan.get("runner", {}).get("implementation_sha256", "")
    return sha256_file(SCRIPT_PATH)


def validate_inventory_package(
    inventory: dict[str, Any], name: str, version: str
) -> dict[str, Any]:
    matches = [
        package_version
        for package in inventory.get("packages", [])
        if package.get("name") == name
        for package_version in package.get("versions", [])
        if package_version.get("version") == version
    ]
    if len(matches) != 1:
        raise ValueError(f"frozen inventory lacks unique {name}@{version}")
    value = matches[0]
    files = value.get("files")
    if not isinstance(files, list) or files != sorted(files, key=lambda item: item.get("path", "")):
        raise ValueError(f"frozen inventory file order is invalid for {name}@{version}")
    paths = [item.get("path") for item in files]
    if len(paths) != len(set(paths)):
        raise ValueError(f"frozen inventory has duplicate paths for {name}@{version}")
    manifest = "".join(
        f"{item['path']}\0{item['kind']}\0{item['size']}\0{item['mode']}\0{item['sha256']}\n"
        for item in files
    ).encode()
    if sha256_bytes(manifest) != value.get("inventory_sha256"):
        raise ValueError(f"frozen inventory manifest digest failed for {name}@{version}")
    if sum(item["size"] for item in files) != value.get("dist", {}).get("unpackedSize"):
        raise ValueError(f"frozen inventory unpacked byte count failed for {name}@{version}")
    return value


def file_identity(package: dict[str, Any], path: str) -> dict[str, Any]:
    matches = [item for item in package.get("files", []) if item.get("path") == path]
    if len(matches) != 1:
        raise ValueError(f"frozen package is missing unique extracted file {path}")
    return matches[0]


def validate_plan() -> dict[str, Any]:
    plan = load_object(PLAN_PATH)
    schema = load_object(SCHEMA_PATH)
    if plan.get("schema") != "copilot-cli-acp-host-permission-plan.v1":
        raise ValueError("unexpected normal-host plan schema")
    if schema.get("$id") != "copilot-cli-acp-host-permission-plan.v1":
        raise ValueError("plan schema identity mismatch")
    if (
        schema.get("type") != "object"
        or schema.get("additionalProperties") is not False
        or not set(plan).issubset(set(schema.get("required", [])))
        or not set(plan).issubset(set(schema.get("properties", {})))
    ):
        raise ValueError("plan schema does not require and describe every plan field")
    if plan.get("plan_id") != PLAN_ID:
        raise ValueError("normal-host plan ID changed")
    if plan.get("operator_decision") != "d6c0fdf0-5abe-4562-a59f-e79520b20dca":
        raise ValueError("normal-host plan is not bound to Tom's recorded direction")
    if plan.get("preparation_execution_authorized") is not False:
        raise ValueError("preparation must remain fake-only")
    if plan.get("original_execution_enabled") is not False:
        raise ValueError("the original entrypoint must remain disabled")
    if plan.get("qualification_changed") is not False:
        raise ValueError("the plan must not move route qualification")
    if plan.get("scope") != {
        "route": "copilot-cli.acp",
        "target_version": "1.0.93",
        "platform": "darwin-arm64",
        "older_versions_started": [],
    }:
        raise ValueError("normal-host plan scope changed")
    if plan.get("runner", {}).get("path") != "scripts/copilot-acp-host-permission-proof.py":
        raise ValueError("plan does not name this runner")
    if plan["runner"].get("sha256") != PREPARATION_RUNNER_SHA256:
        raise ValueError("reviewed preparation runner identity changed")
    staging = plan.get("artifact_staging", {})
    if (
        staging.get("source") != "official npm registry tarballs only"
        or staging.get("staging_root_role") != "fresh task-owned private scratch directory, never a checkout"
        or staging.get("verify_archive_sha256_and_sri") is not True
        or staging.get("verify_extracted_manifest_versions_and_native_platform") is not True
        or staging.get("native_executable_path") != "package/copilot"
        or staging.get("verify_native_executable_sha256_before_launch") is not True
        or staging.get("wrapper_archive_sha256") != EXPECTED_WRAPPER_ARCHIVE_SHA256
        or staging.get("native_archive_sha256") != EXPECTED_NATIVE_ARCHIVE_SHA256
        or staging.get("native_executable_sha256") != EXPECTED_EXECUTABLE_SHA256
        or staging.get("execute_wrapper") is not False
        or staging.get("execute_during_preparation") is not False
        or staging.get("preparation_staged_vendor_artifacts") is not False
    ):
        raise ValueError("artifact staging instructions do not bind the exact disabled target")
    if (
        plan.get("artifact_inventory", {}).get("path")
        != "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/artifact-inventory.json"
        or plan.get("artifact_inventory", {}).get("sha256") != EXPECTED_INVENTORY_SHA256
    ):
        raise ValueError("plan inventory identity changed")
    if sha256_file(INVENTORY_PATH) != EXPECTED_INVENTORY_SHA256:
        raise ValueError("retained artifact inventory changed")

    inventory = load_object(INVENTORY_PATH)
    wrapper = validate_inventory_package(inventory, "@github/copilot", "1.0.93")
    native = validate_inventory_package(
        inventory, "@github/copilot-darwin-arm64", "1.0.93"
    )
    wrapper_identity = plan.get("artifacts", {}).get("wrapper", {})
    native_identity = plan.get("artifacts", {}).get("native", {})
    if (
        wrapper.get("archive_sha256") != EXPECTED_WRAPPER_ARCHIVE_SHA256
        or wrapper.get("dist", {}).get("tarball") != wrapper_identity.get("archive_url")
        or wrapper.get("dist", {}).get("integrity") != wrapper_identity.get("archive_integrity")
        or wrapper_identity.get("archive_sha256") != EXPECTED_WRAPPER_ARCHIVE_SHA256
        or wrapper_identity.get("package") != "@github/copilot"
        or wrapper_identity.get("version") != "1.0.93"
        or wrapper.get("version") != wrapper_identity.get("version")
        or wrapper_identity.get("package_manifest_sha256") != EXPECTED_WRAPPER_MANIFEST_SHA256
        or wrapper_identity.get("loader_sha256") != EXPECTED_WRAPPER_LOADER_SHA256
        or wrapper_identity.get("archive_url")
        != "https://registry.npmjs.org/@github/copilot/-/copilot-1.0.93.tgz"
        or file_identity(wrapper, "package/npm-loader.js").get("sha256")
        != EXPECTED_WRAPPER_LOADER_SHA256
        or file_identity(wrapper, "package/package.json").get("sha256")
        != EXPECTED_WRAPPER_MANIFEST_SHA256
    ):
        raise ValueError("wrapper package identity or extracted provenance mismatched")
    if (
        native.get("archive_sha256") != EXPECTED_NATIVE_ARCHIVE_SHA256
        or native.get("dist", {}).get("tarball") != native_identity.get("archive_url")
        or native.get("dist", {}).get("integrity") != native_identity.get("archive_integrity")
        or native_identity.get("archive_sha256") != EXPECTED_NATIVE_ARCHIVE_SHA256
        or native_identity.get("package") != "@github/copilot-darwin-arm64"
        or native_identity.get("version") != "1.0.93"
        or native.get("version") != native_identity.get("version")
        or native_identity.get("package_manifest_sha256") != EXPECTED_NATIVE_MANIFEST_SHA256
        or native_identity.get("executable_sha256") != EXPECTED_EXECUTABLE_SHA256
        or native_identity.get("archive_url")
        != "https://registry.npmjs.org/@github/copilot-darwin-arm64/-/copilot-darwin-arm64-1.0.93.tgz"
        or file_identity(native, "package/copilot").get("sha256")
        != EXPECTED_EXECUTABLE_SHA256
        or file_identity(native, "package/package.json").get("sha256")
        != EXPECTED_NATIVE_MANIFEST_SHA256
    ):
        raise ValueError("native package identity or extracted executable provenance mismatched")
    if (
        native_identity.get("os") != ["darwin"]
        or native_identity.get("cpu") != ["arm64"]
    ):
        raise ValueError("native artifact platform labels differ from the selected route")
    executable = file_identity(native, "package/copilot")
    if executable.get("kind") != "file" or executable.get("mode") != "0755":
        raise ValueError("native payload is not the inventoried executable file")
    if (
        wrapper_identity.get("loader_path") != "package/npm-loader.js"
        or wrapper_identity.get("bin_label") != "copilot -> npm-loader.js"
        or native_identity.get("executable_path") != "package/copilot"
    ):
        raise ValueError("plan launch paths differ from frozen package inventory")

    invocation = plan.get("invocation", {})
    expected_invocation = {
        "executable_role": "staged frozen native package/copilot",
        "executable_sha256": EXPECTED_EXECUTABLE_SHA256,
        "argv": ["--model", "auto", "--acp", "--stdio"],
        "wrapper_is_executed": False,
        "working_directory_role": "fresh task-owned temporary directory containing one sentinel and no project files",
        "working_directory_policy": "realpath directly under a fresh task scratch root; never a checkout or symlink",
        "environment_policy": "inherit the ordinary host process environment without reading, copying, filtering, logging, or serializing environment values; do not relocate HOME or set Copilot model/auth variables",
        "existing_account": "betterthanclay",
        "account_access": "existing normal macOS Copilot CLI login; account identity is not queried by the harness",
        "auto_selection": "explicit --model auto; observe structured model identity if ACP exposes it, otherwise record unobserved",
        "auth_handling": "vendor CLI performs its existing normal host login access; harness does not read config, Keychain, tokens or auth responses",
    }
    if invocation != expected_invocation:
        raise ValueError("normal-host invocation, account or Auto boundary changed")
    expected_host_effects = {
        "trust_boundary": "trust the exact reviewed vendor CLI under the normal logged-in host account",
        "runtime_children": "ordinary vendor child processes may run; harness observes only the owned root, its process group, and joined stdio readers",
        "vendor_state": "vendor-managed Copilot state and logs may be read or written under the existing host account; exact internal item access is not mediated",
        "network": "ordinary vendor authentication and provider networking; no default-deny egress or network containment claim",
        "host_access_claim": "normal process access is not a guarantee that the vendor has no ambient host access",
        "platform_scope": "Darwin arm64 only; no Linux, x64 or cross-platform qualification credit",
    }
    if plan.get("normal_host_effects") != expected_host_effects:
        raise ValueError("normal-host effects or containment boundary changed")
    expected_action = {
        "prompt": PROMPT_TEXT,
        "path": "permission-sentinel.txt",
        "before_bytes": SENTINEL_BEFORE.decode(),
        "requested_after_bytes": SENTINEL_AFTER.decode(),
        "permission_required": True,
        "permission_reply": "cancelled only",
        "approval": "never",
        "resend": False,
        "effect_limit": 0,
        "effect_observation": "compare exact task sentinel bytes before and after; any mismatch, unexpected action, tool-call update before permission, missing request, or uncertain result fails; this does not mediate arbitrary host effects",
    }
    if plan.get("action") != expected_action:
        raise ValueError("normal-host sentinel action or no-approval policy changed")
    expected_attempt = {
        "maximum_invocations": 1,
        "maximum_prompts": 1,
        "shared_prompt_ceiling": 3,
        "previous_prompt_allowance_changed": False,
        "maximum_seconds": 60,
        "cleanup_seconds": 3,
        "harness_retries": 0,
        "harness_resends": 0,
        "model_fallbacks": 0,
        "reviewer_original_attempts": 0,
        "failure_consumes_invocation": True,
        "prompt_slot_consumed_before_send": True,
    }
    if plan.get("attempt") != expected_attempt:
        raise ValueError("original prompt or invocation budget changed")
    if plan.get("ledger", {}).get("root") != "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 168":
        raise ValueError("persistent one-shot ledger root changed")
    if (
        plan["ledger"].get("root_mode") != "0700; require current-user ownership; reject symlink path components"
        or plan["ledger"].get("record_mode") != "0600"
        or plan["ledger"].get("creation") != "O_CREAT|O_EXCL|O_NOFOLLOW, fsync file and parent directory; existing record refuses all relaunch"
        or plan["ledger"].get("attempt_binding") != ["plan_sha256", "runner_sha256", "wrapper_archive_sha256", "native_archive_sha256", "native_executable_sha256", "argv_sha256", "environment_policy", "account_ref", "action", "budgets"]
        or plan["ledger"].get("prompt_budget_before_send") != "exclusive-fsynced-prompt-slot-record"
        or plan["ledger"].get("failure_policy") != "never delete, reset, replace or retry a consumed attempt or prompt slot"
        or plan["ledger"].get("created_in_preparation") is not False
    ):
        raise ValueError("persistent one-shot ledger safety rules changed")
    if plan["ledger"].get("attempt_path") != (
        "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 168/1.0.93-attempt.json"
    ):
        raise ValueError("persistent one-shot attempt path changed")
    if plan["ledger"].get("prompt_path") != (
        "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 168/1.0.93-prompt-1.json"
    ):
        raise ValueError("persistent one-shot prompt path changed")
    if (
        plan.get("process_cleanup", {}).get("reported_descendants")
        != "unknown-for-arbitrary-vendor-descendants"
        or plan["process_cleanup"].get("process_group_exit_proves_descendant_join") is not False
        or plan.get("currentness_observation", {}).get("wrapper_latest") != "1.0.95"
        or plan["currentness_observation"].get("native_latest") != "1.0.95"
    ):
        raise ValueError("plan overstates cleanup or changes currentness movement")

    serialized = json.dumps(plan, sort_keys=True)
    for marker in FORBIDDEN_PERSISTED_MARKERS:
        if marker in serialized:
            raise ValueError("plan contains a secret or synthetic secret sentinel")
    return plan


def validate_correction_plan() -> dict[str, Any]:
    plan = load_object(CORRECTION_PLAN_PATH)
    schema = load_object(CORRECTION_SCHEMA_PATH)
    if (
        schema.get("$id") != "copilot-cli-acp-host-permission-correction-plan.v1"
        or schema.get("additionalProperties") is not False
        or set(plan) != set(schema.get("required", []))
        or set(plan) != set(schema.get("properties", {}))
        or plan.get("schema") != "copilot-cli-acp-host-permission-correction-plan.v1"
        or plan.get("plan_id") != CORRECTION_PLAN_ID
        or plan.get("preparation_execution_authorized") is not False
        or plan.get("original_execution_enabled") is not False
        or plan.get("execution_authorized") is not False
        or plan.get("separate_original_authority_required") is not True
        or plan.get("qualification_changed") is not False
        or plan.get("pre_probes") != []
    ):
        raise ValueError("corrected attempt proposal is malformed or enabled")
    if plan.get("source_attempt") != {
        "research": "docs/research/435-copilot-acp-normal-host-permission-observation.md",
        "attempt_record_sha256": "30f16a9f2d6b0e168b0486cacb71c0f96a719bc3f7f13687807a81c11731929c",
        "observation_record_sha256": "982d716e615daaa7d92612c74ae84ac13ab62c26fb19005841eb1ec09ea6ac94",
        "preserve_all_existing_records": True,
        "shared_prompt_slots_remaining": 3,
    }:
        raise ValueError("corrected proposal does not preserve the consumed 435 attempt")
    if (
        plan.get("artifact_inventory") != {
            "path": "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/artifact-inventory.json",
            "sha256": EXPECTED_INVENTORY_SHA256,
        }
        or plan.get("artifacts") != {
            "wrapper_archive_sha256": EXPECTED_WRAPPER_ARCHIVE_SHA256,
            "wrapper_loader_sha256": EXPECTED_WRAPPER_LOADER_SHA256,
            "native_archive_sha256": EXPECTED_NATIVE_ARCHIVE_SHA256,
            "native_executable_sha256": EXPECTED_EXECUTABLE_SHA256,
            "native_manifest_sha256": EXPECTED_NATIVE_MANIFEST_SHA256,
            "route_version": "1.0.93",
            "platform": "darwin-arm64",
        }
        or plan.get("runner") != {
            "path": "scripts/copilot-acp-host-permission-proof.py",
            "preparation_runner_sha256": PREPARATION_RUNNER_SHA256,
            "implementation_sha256": TASK170_CORRECTION_RUNNER_SHA256,
        }
    ):
        raise ValueError("corrected proposal changed the frozen artifact or runner binding")
    expected_environment_policy = (
        "copy inherited environment without logging or serializing values; set "
        "COPILOT_AUTO_UPDATE=false; set COPILOT_PKG_CACHE_HOME to a fresh temporary "
        "directory inside task-owned scratch; remove COPILOT_CLI_VERSION and "
        "COPILOT_CLI_DIST_DIR; preserve HOME, COPILOT_HOME, existing login, model "
        "selection and all other inherited entries"
    )
    if plan.get("invocation") != {
        "executable": "exact staged package/copilot",
        "argv": ["--model", "auto", "--acp", "--stdio"],
        "wrapper_executed": False,
        "existing_account": "betterthanclay",
        "environment_policy": expected_environment_policy,
        "temporary_package_cache_removed_after_cleanup": True,
        "version_gate": "retain only strict bounded SemVer; require exactly 1.0.93 before session/new; otherwise fail before session or prompt",
    }:
        raise ValueError("corrected launch changed its exact identity, environment or Auto policy")
    expected_action = {
        "prompt": PROMPT_TEXT,
        "path": "permission-sentinel.txt",
        "permission_reply": "cancelled only",
        "approval": "never",
        "resend": False,
        "effect_limit": 0,
        "effect_observation": "compare exact task sentinel bytes before and after; mismatch, unexpected action, missing permission, tool-call update before permission or uncertain result fails",
    }
    expected_attempt = {
        "maximum_invocations": 1,
        "maximum_prompts": 1,
        "shared_prompt_ceiling": 3,
        "shared_prompt_slots_before": 3,
        "maximum_seconds_including_cleanup": 60,
        "cleanup_seconds": 3,
        "harness_retries": 0,
        "harness_resends": 0,
        "model_fallbacks": 0,
        "reviewer_original_attempts": 0,
        "failure_consumes_invocation": True,
        "prompt_slot_consumed_before_send": True,
    }
    expected_ledger = {
        "root": "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 170",
        "attempt_path": "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 170/1.0.93-corrected-attempt.json",
        "prompt_path": "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 170/1.0.93-corrected-prompt-1.json",
        "execution_path": "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 170/1.0.93-corrected-execution.json",
        "record_mode": "0600",
        "creation": "O_CREAT|O_EXCL|O_NOFOLLOW, fsync file and parent directory; existing record refuses all relaunch",
        "failure_policy": "never delete, reset, replace or retry a consumed attempt or prompt slot",
        "created_in_this_task": False,
    }
    if (
        plan.get("action") != expected_action
        or plan.get("attempt") != expected_attempt
        or plan.get("ledger") != expected_ledger
        or plan.get("normal_host_access") != {
            "use_existing_normal_host_login": True,
            "ordinary_vendor_network_and_state_access": True,
            "credentials_or_configuration_read_by_harness": False,
            "qualification_credit": False,
        }
        or plan.get("cleanup") != {
            "root_and_streams": "bounded and joined within the 60-second inclusive ceiling",
            "process_group_observation": True,
            "arbitrary_descendant_cleanup": "unknown-for-arbitrary-vendor-descendants",
        }
        or sha256_file(INVENTORY_PATH) != EXPECTED_INVENTORY_SHA256
        or sha256_file(ROOT / "docs/research/435-copilot-acp-normal-host-permission-attempt.json")
        != "30f16a9f2d6b0e168b0486cacb71c0f96a719bc3f7f13687807a81c11731929c"
        or sha256_file(ROOT / "docs/research/435-copilot-acp-normal-host-permission-observation.json")
        != "982d716e615daaa7d92612c74ae84ac13ab62c26fb19005841eb1ec09ea6ac94"
    ):
        raise ValueError("corrected proposal changed its one-shot, sentinel, host or cleanup boundary")
    return plan


def validate_final_correction_plan() -> dict[str, Any]:
    plan = load_object(FINAL_CORRECTION_PLAN_PATH)
    schema = load_object(FINAL_CORRECTION_SCHEMA_PATH)
    proposal = validate_correction_plan()
    if (
        schema.get("$id") != "copilot-cli-acp-host-permission-final-plan.v1"
        or schema.get("additionalProperties") is not False
        or set(plan) != set(schema.get("required", []))
        or set(plan) != set(schema.get("properties", {}))
        or plan.get("schema") != "copilot-cli-acp-host-permission-final-plan.v1"
        or plan.get("plan_id") != FINAL_CORRECTION_PLAN_ID
        or plan.get("task_number") != 171
        or plan.get("task_id") != CORRECTED_TASK_ID
        or plan.get("run_id") != CORRECTED_RUN_ID
        or plan.get("brief_sha256") != CORRECTED_BRIEF_SHA256
        or plan.get("operator_decision") != CORRECTED_OPERATOR_DECISION
        or plan.get("source_proposal_sha256") != CORRECTION_PROPOSAL_SHA256
        or sha256_file(CORRECTION_PLAN_PATH) != CORRECTION_PROPOSAL_SHA256
        or plan.get("preparation_plan_sha256") != sha256_file(PLAN_PATH)
        or plan.get("preparation_runner_sha256") != PREPARATION_RUNNER_SHA256
        or plan.get("preparation_execution_authorized") is not False
        or plan.get("original_execution_enabled") is not True
        or plan.get("execution_authorized") is not True
        or plan.get("separate_original_authority_required") is not True
        or plan.get("qualification_changed") is not False
        or plan.get("pre_probes") != []
        or plan.get("fake_pass_record_path") != "docs/research/437-copilot-acp-corrected-normal-host-fake-pass.json"
        or plan.get("authority_path") != "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-host-permission-proof/corrected-execution-authority.json"
    ):
        raise ValueError("final corrected plan is malformed or not bound to this one-shot task")

    expected_runner = {
        "path": "scripts/copilot-acp-host-permission-proof.py",
        "preparation_runner_sha256": PREPARATION_RUNNER_SHA256,
        "implementation_sha256": FINAL_CORRECTED_ATTEMPT_RUNNER_SHA256,
    }
    if plan.get("runner") != expected_runner:
        raise ValueError("final corrected plan does not bind this runner")

    for key in (
        "source_attempt", "artifact_inventory", "artifacts", "pre_probes",
        "invocation", "action", "attempt", "normal_host_access", "cleanup",
    ):
        if plan.get(key) != proposal.get(key):
            raise ValueError(f"final corrected plan changed the reviewed proposal field {key}")
    expected_ledger = dict(proposal["ledger"])
    expected_ledger["created_in_this_task"] = True
    if plan.get("ledger") != expected_ledger:
        raise ValueError("final corrected plan changed the reviewed Task 170 ledger paths")

    serialized = json.dumps(plan, sort_keys=True)
    for marker in FORBIDDEN_PERSISTED_MARKERS:
        if marker in serialized:
            raise ValueError("final corrected plan contains a secret or synthetic secret sentinel")
    return plan


def validate_task172_proposal() -> dict[str, Any]:
    proposal = load_object(NEXT_ATTEMPT_PROPOSAL_PATH)
    schema = load_object(NEXT_ATTEMPT_PROPOSAL_SCHEMA_PATH)
    source_attempt = {
        "consumed_task_number": 171,
        "research": "docs/research/437-copilot-acp-corrected-normal-host-attempt.md",
        "attempt_record_sha256": "10639b7fdd2cd10899d52eb13cd44b66c51167672407ff2a069e42ffbf65d928",
        "prompt_slot_record_sha256": "e2dec4e3714a282b640ab1b9e9a5820e473b0cea69f7199fe28c5c293a554c58",
        "execution_record_sha256": "1de648daa5e5491bb75a37c4fda44565e49876caa9f8e6df7ec4a31e706f0bca",
        "shared_prompt_ceiling": 3,
        "prompts_consumed": 1,
        "shared_prompt_slots_remaining": 2,
        "first_tool_call_status": "not-recorded",
        "preserve_all_existing_records": True,
    }
    protocol = load_object(ACP_PROTOCOL_FIXTURE_PATH)
    schema_version = "schema-v1.24.1"
    protocol_identity = {
        "path": "crates/swallowtail-protocol-acp/tests/fixtures/acp-schema-v1.24.1/protocol.json",
        "schema_version": schema_version,
        "tool_call_sha256": "f8dc46888da9e4fa1edfc9cd88f93bc074a839ecc6d08ddb6d2d2ac12fe568fa",
        "tool_call_update_sha256": "89464d21aa887ee0a13ffe3df39ae23f1598eb6a9a15a0a2af14b46f7e4b4968",
        "request_permission_request_sha256": "8d28e54c28364666fbb9a6db2877f0c51f1b54745192ddcd8737ab640afb6336",
        "request_permission_response_sha256": "bd893d42a9ab8d11c255e51e3cd8f2b3d2c60658cecedd09e365d3936dfc3761",
        "pending_default_source": "crates/swallowtail-protocol-acp/src/activity/decode/tool.rs",
        "sparse_update_source": "crates/swallowtail-protocol-acp/src/activity/tool_record.rs",
    }
    invocation = {
        "executable": "exact staged package/copilot",
        "argv": ["--model", "auto", "--acp", "--stdio"],
        "wrapper_executed": False,
        "existing_account": "betterthanclay",
        "model": "Auto",
        "environment_policy": (
            "copy the inherited environment without reading or logging it; only in the child set "
            "COPILOT_AUTO_UPDATE=false, set COPILOT_PKG_CACHE_HOME to a fresh task-owned temporary "
            "directory, and remove COPILOT_CLI_VERSION and COPILOT_CLI_DIST_DIR; preserve HOME, "
            "COPILOT_HOME, the existing login, Auto selection and all other inherited entries"
        ),
        "version_gate": "require the bounded public agentInfo.version to equal 1.0.93 before session/new and the prompt",
    }
    action_proposal = {
        "prompt": PROMPT_TEXT,
        "path": "permission-sentinel.txt",
        "permission_reply": "cancelled only",
        "approval": "never",
        "resend": False,
        "effect_limit": 0,
        "effect_observation": "compare the exact sentinel bytes and action-directory entries before and after; retain no paths or raw protocol",
    }
    attempt = {
        "maximum_invocations": 1,
        "maximum_prompts": 1,
        "shared_prompt_ceiling": 3,
        "shared_prompt_slots_before": 2,
        "shared_prompt_slots_remaining_after": 1,
        "maximum_seconds_including_cleanup": 60,
        "cleanup_seconds": 3,
        "harness_retries": 0,
        "harness_resends": 0,
        "model_fallbacks": 0,
        "reviewer_original_attempts": 0,
        "failure_consumes_invocation": True,
        "prompt_slot_consumed_before_send": True,
    }
    ledger_root = "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 172"
    ledger = {
        "root": ledger_root,
        "attempt_path": f"{ledger_root}/1.0.93-corrected-attempt.json",
        "prompt_path": f"{ledger_root}/1.0.93-corrected-prompt-1.json",
        "execution_path": f"{ledger_root}/1.0.93-corrected-execution.json",
        "record_mode": "0600",
        "creation": "O_CREAT|O_EXCL|O_NOFOLLOW, fsync file and parent; existing files refuse launch",
        "failure_policy": "preserve all consumed records and never retry",
        "created_in_this_task": False,
    }
    normal_host_access = {
        "use_existing_normal_host_login": True,
        "ordinary_vendor_network_and_state_access": True,
        "credentials_or_configuration_read_by_harness": False,
        "qualification_credit": False,
    }
    cleanup = {
        "root_and_streams": "bounded and joined within the 60-second inclusive ceiling",
        "process_group_observation": True,
        "arbitrary_descendant_cleanup": "unknown-for-arbitrary-vendor-descendants",
    }
    if (
        schema.get("$id") != "copilot-cli-acp-host-permission-task-proposal.v1"
        or schema.get("additionalProperties") is not False
        or set(proposal) != set(schema.get("required", []))
        or set(proposal) != set(schema.get("properties", {}))
        or proposal.get("schema") != schema.get("$id")
        or proposal.get("task_number") != 172
        or proposal.get("task_id") != TASK172_ID
        or proposal.get("run_id") != TASK172_RUN_ID
        or proposal.get("plan_id") != TASK172_PROPOSAL_ID
        or proposal.get("original_execution_enabled") is not False
        or proposal.get("execution_authorized") is not False
        or proposal.get("separate_original_authority_required") is not True
        or proposal.get("qualification_changed") is not False
        or proposal.get("source_attempt") != source_attempt
        or proposal.get("protocol") != protocol_identity
        or proposal.get("runner") != {
            "path": "scripts/copilot-acp-host-permission-proof.py",
            "preparation_runner_sha256": PREPARATION_RUNNER_SHA256,
            "implementation_sha256": sha256_file(SCRIPT_PATH),
        }
        or proposal.get("pre_probes") != []
        or proposal.get("invocation") != invocation
        or proposal.get("action") != action_proposal
        or proposal.get("attempt") != attempt
        or proposal.get("ledger") != ledger
        or proposal.get("normal_host_access") != normal_host_access
        or proposal.get("cleanup") != cleanup
        or proposal.get("artifacts") != {
            "route": "copilot-cli.acp",
            "version": "1.0.93",
            "platform": "darwin-arm64",
            "native_archive_sha256": EXPECTED_NATIVE_ARCHIVE_SHA256,
            "native_executable_sha256": EXPECTED_EXECUTABLE_SHA256,
        }
        or protocol.get("selected_definition_sha256", {}).get("ToolCall", {}).get(schema_version)
        != protocol_identity["tool_call_sha256"]
        or protocol.get("selected_definition_sha256", {}).get("ToolCallUpdate", {}).get(schema_version)
        != protocol_identity["tool_call_update_sha256"]
        or protocol.get("selected_definition_sha256", {}).get("RequestPermissionRequest", {}).get(schema_version)
        != protocol_identity["request_permission_request_sha256"]
        or protocol.get("selected_definition_sha256", {}).get("RequestPermissionResponse", {}).get(schema_version)
        != protocol_identity["request_permission_response_sha256"]
        or protocol.get("selected_surfaces", {}).get("activity", {}).get("selected_shape_changed") is not False
        or protocol.get("selected_surfaces", {}).get("permission", {}).get("selected_shape_changed") is not False
    ):
        raise ValueError("Task 172 proposal is enabled, stale or detached from frozen ACPv1 source")

    decoder = (ROOT / protocol_identity["pending_default_source"]).read_text(encoding="utf-8")
    update_model = (ROOT / protocol_identity["sparse_update_source"]).read_text(encoding="utf-8")
    if (
        "unwrap_or(AcpToolCallStatus::Pending)" not in decoder
        or "kind: optional_kind(update, limits)?.unwrap_or_else(default_tool_kind)" not in decoder
        or 'AcpBoundedText("other".to_owned())' not in decoder
        or "pub status: Option<AcpToolCallStatus>" not in update_model
        or "locations_replacement" not in update_model
        or '"edit" => AcpToolKind::Edit' not in decoder
    ):
        raise ValueError("Task 172 proposal no longer matches the frozen ACPv1 implementation")

    serialized = json.dumps(proposal, sort_keys=True)
    if any(marker in serialized for marker in FORBIDDEN_PERSISTED_MARKERS):
        raise ValueError("Task 172 proposal contains a secret or synthetic marker")
    return proposal


def validate_fake_pass_record(path: Path = CORRECTED_FAKE_PASS_PATH) -> dict[str, Any]:
    record = load_object(path)
    expected_keys = {
        "schema", "task_number", "task_id", "run_id", "operator_decision",
        "source_proposal_sha256", "plan_sha256", "runner_sha256", "status",
        "originals_run", "results",
    }
    plan = validate_final_correction_plan()
    if (
        set(record) != expected_keys
        or record.get("schema") != "copilot-cli-acp-host-permission-fake-pass.v1"
        or record.get("task_number") != 171
        or record.get("task_id") != CORRECTED_TASK_ID
        or record.get("run_id") != CORRECTED_RUN_ID
        or record.get("operator_decision") != CORRECTED_OPERATOR_DECISION
        or record.get("source_proposal_sha256") != CORRECTION_PROPOSAL_SHA256
        or record.get("plan_sha256") != sha256_file(FINAL_CORRECTION_PLAN_PATH)
        or record.get("runner_sha256") != plan["runner"]["implementation_sha256"]
        or record.get("status") != "passed"
        or record.get("originals_run") is not False
        or not isinstance(record.get("results"), dict)
        or record["results"].get("status") != "passed"
        or record["results"].get("originals_run") is not False
        or record["results"].get("qualification_changed") is not False
        or record["results"].get("task_number") != 171
        or record["results"].get("task_id") != CORRECTED_TASK_ID
        or record["results"].get("run_id") != CORRECTED_RUN_ID
        or record["results"].get("operator_decision") != CORRECTED_OPERATOR_DECISION
        or record["results"].get("original_shaped_launch", {}).get("package_cache_cleanup") != "removed"
        or record["results"].get("original_shaped_record_validation", {}).get("status") != "valid"
        or record["results"].get("bounded_cleanup", {}).get("failure_class") != "timeout"
        or record["results"].get("bounded_cleanup", {}).get("package_cache_cleanup") != "removed"
        or record["results"].get("bounded_cleanup", {}).get("record_validation") != "valid"
        or record["results"].get("package_cache_cleanup_error", {}).get("error_category") != "os-error"
        or record["results"].get("package_cache_cleanup_error", {}).get("record_validation") != "valid"
        or not all(record["results"].get("consumed_ledger_fsync", {}).values())
        or record["results"].get("original_replay_refused", {}).get("attempt") is not True
        or record["results"].get("original_replay_refused", {}).get("prompt") is not True
        or any(marker in json.dumps(record, sort_keys=True) for marker in FORBIDDEN_PERSISTED_MARKERS)
    ):
        raise ValueError("corrected fake-pass record is malformed, stale, or contains unsafe data")
    if plan["runner"]["implementation_sha256"] != record["runner_sha256"]:
        raise ValueError("corrected fake-pass record does not bind the final runner")
    return record


def create_fake_pass_record(results: dict[str, Any]) -> dict[str, Any]:
    plan = validate_final_correction_plan()
    if results.get("status") != "passed" or results.get("originals_run") is not False:
        raise RuntimeError("fake-proof results cannot be retained as a passing record")
    if CORRECTED_FAKE_PASS_PATH.is_symlink():
        raise RuntimeError("fake-pass evidence path must not be a symlink")
    if CORRECTED_FAKE_PASS_PATH.exists():
        return validate_fake_pass_record(CORRECTED_FAKE_PASS_PATH)
    if sha256_file(SCRIPT_PATH) != plan["runner"]["implementation_sha256"]:
        raise RuntimeError("cannot create historical fake-pass evidence from a changed runner")
    record = {
        "schema": "copilot-cli-acp-host-permission-fake-pass.v1",
        "task_number": 171,
        "task_id": CORRECTED_TASK_ID,
        "run_id": CORRECTED_RUN_ID,
        "operator_decision": CORRECTED_OPERATOR_DECISION,
        "source_proposal_sha256": CORRECTION_PROPOSAL_SHA256,
        "plan_sha256": sha256_file(FINAL_CORRECTION_PLAN_PATH),
        "runner_sha256": plan["runner"]["implementation_sha256"],
        "status": "passed",
        "originals_run": False,
        "results": results,
    }
    encoded = json.dumps(record, sort_keys=True, indent=2).encode() + b"\n"
    parent = CORRECTED_FAKE_PASS_PATH.parent
    if parent.is_symlink():
        raise RuntimeError("fake-pass evidence path must not contain a symlink parent")
    parent_fd = os.open(parent, os.O_RDONLY | getattr(os, "O_DIRECTORY", 0) | getattr(os, "O_NOFOLLOW", 0))
    try:
        fd = os.open(
            CORRECTED_FAKE_PASS_PATH.name,
            os.O_WRONLY | os.O_CREAT | os.O_EXCL | getattr(os, "O_NOFOLLOW", 0),
            0o644,
            dir_fd=parent_fd,
        )
        try:
            offset = 0
            while offset < len(encoded):
                offset += os.write(fd, encoded[offset:])
            os.fsync(fd)
        finally:
            os.close(fd)
        os.fsync(parent_fd)
    finally:
        os.close(parent_fd)
    return record


def validate_corrected_authority(
    authority_path: Path = CORRECTED_AUTHORITY_PATH,
    *,
    plan: dict[str, Any] | None = None,
) -> tuple[dict[str, Any], str]:
    bound_plan = plan if plan is not None else validate_final_correction_plan()
    authority = load_object(authority_path)
    schema = load_object(CORRECTED_AUTHORITY_SCHEMA_PATH)
    fake_pass = validate_fake_pass_record(ROOT / bound_plan["fake_pass_record_path"])
    fake_pass_sha256 = sha256_file(ROOT / bound_plan["fake_pass_record_path"])
    if (
        schema.get("$id") != "copilot-cli-acp-host-permission-corrected-execution-authority.v1"
        or schema.get("additionalProperties") is not False
        or set(authority) != set(schema.get("required", []))
        or set(authority) != set(schema.get("properties", {}))
    ):
        raise ValueError("corrected execution authority schema does not describe the exact manifest")
    expected = {
        "schema": "copilot-cli-acp-host-permission-corrected-execution-authority.v1",
        "task_number": 171,
        "task_id": CORRECTED_TASK_ID,
        "run_id": CORRECTED_RUN_ID,
        "brief_sha256": CORRECTED_BRIEF_SHA256,
        "operator_decision": CORRECTED_OPERATOR_DECISION,
        "source_proposal_sha256": CORRECTION_PROPOSAL_SHA256,
        "source_attempt_record_sha256": bound_plan["source_attempt"]["attempt_record_sha256"],
        "source_observation_record_sha256": bound_plan["source_attempt"]["observation_record_sha256"],
        "preparation_plan_sha256": sha256_file(PLAN_PATH),
        "preparation_runner_sha256": PREPARATION_RUNNER_SHA256,
        "final_plan_sha256": sha256_file(FINAL_CORRECTION_PLAN_PATH),
        "plan_id": FINAL_CORRECTION_PLAN_ID,
        "final_runner_sha256": bound_plan["runner"]["implementation_sha256"],
        "fake_pass_record_path": bound_plan["fake_pass_record_path"],
        "fake_pass_record_sha256": fake_pass_sha256,
        "route": "copilot-cli.acp",
        "target_version": "1.0.93",
        "platform": "darwin-arm64",
        "account_ref": "betterthanclay",
        "wrapper_archive_sha256": EXPECTED_WRAPPER_ARCHIVE_SHA256,
        "native_archive_sha256": EXPECTED_NATIVE_ARCHIVE_SHA256,
        "native_executable_sha256": EXPECTED_EXECUTABLE_SHA256,
        "argv": ["--model", "auto", "--acp", "--stdio"],
        "environment_policy": bound_plan["invocation"]["environment_policy"],
        "execution_authorized": True,
        "maximum_invocations": 1,
        "maximum_prompts": 1,
        "consumed_invocations_before": 1,
        "shared_prompt_slots_consumed_before": 0,
        "shared_prompt_slots_remaining_before": 3,
        "maximum_seconds": 60,
        "cleanup_seconds": 3,
        "attempt_path": bound_plan["ledger"]["attempt_path"],
        "prompt_path": bound_plan["ledger"]["prompt_path"],
        "execution_record_path": bound_plan["ledger"]["execution_path"],
        "qualification_changed": False,
        "reviewer_original_attempts": 0,
        "older_versions_started": [],
    }
    if authority != expected:
        raise ValueError("corrected execution authority does not exactly bind this task, decision and fake proof")
    if fake_pass.get("runner_sha256") != authority["final_runner_sha256"]:
        raise ValueError("corrected execution authority and fake-pass runner identities differ")
    return authority, sha256_file(authority_path)


def validate_execution_authority(
    authority_path: Path = AUTHORITY_PATH,
    *,
    plan: dict[str, Any] | None = None,
) -> tuple[dict[str, Any], str]:
    """Validate the separate one-shot authority before any original-path effects."""
    bound_plan = plan if plan is not None else validate_plan()
    authority = load_object(authority_path)
    schema = load_object(AUTHORITY_SCHEMA_PATH)
    if (
        schema.get("$id") != "copilot-cli-acp-host-permission-execution-authority.v1"
        or schema.get("additionalProperties") is not False
        or set(authority) != set(schema.get("required", []))
        or set(authority) != set(schema.get("properties", {}))
    ):
        raise ValueError("execution authority schema does not describe the exact manifest")
    expected = {
        "schema": "copilot-cli-acp-host-permission-execution-authority.v1",
        "task_number": 169,
        "task_id": "dcef61b4-fd5e-416a-9226-b8be170bf17c",
        "run_id": "8f42c5b5-47f2-4c50-82f8-d606249820d0",
        "brief_sha256": BRIEF_SHA256,
        "operator_decision": "d6c0fdf0-5abe-4562-a59f-e79520b20dca",
        "preparation_plan_sha256": sha256_file(PLAN_PATH),
        "preparation_runner_sha256": PREPARATION_RUNNER_SHA256,
        "final_runner_sha256": ORIGINAL_AUTHORITY_RUNNER_SHA256,
        "plan_id": PLAN_ID,
        "route": "copilot-cli.acp",
        "target_version": "1.0.93",
        "platform": "darwin-arm64",
        "account_ref": "betterthanclay",
        "wrapper_archive_sha256": EXPECTED_WRAPPER_ARCHIVE_SHA256,
        "native_archive_sha256": EXPECTED_NATIVE_ARCHIVE_SHA256,
        "native_executable_sha256": EXPECTED_EXECUTABLE_SHA256,
        "argv": ["--model", "auto", "--acp", "--stdio"],
        "environment_policy": bound_plan["invocation"]["environment_policy"],
        "execution_authorized": True,
        "maximum_invocations": 1,
        "maximum_prompts": 1,
        "shared_prompt_ceiling_before": 3,
        "shared_prompt_slots_consumed": 0,
        "shared_prompt_slots_remaining": 3,
        "maximum_seconds": 60,
        "cleanup_seconds": 3,
        "attempt_path": bound_plan["ledger"]["attempt_path"],
        "prompt_path": bound_plan["ledger"]["prompt_path"],
        "execution_record_path": "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 168/1.0.93-execution.json",
        "qualification_changed": False,
        "reviewer_original_attempts": 0,
    }
    if authority != expected:
        raise ValueError("execution authority does not exactly bind this task, preparation and runner")
    return authority, sha256_file(authority_path)


def validate_execution_record(record: dict[str, Any], plan: dict[str, Any]) -> dict[str, Any]:
    legacy_fields = {
        "schema", "plan_sha256", "runner_sha256", "execution_kind", "original_execution",
        "scenario", "status", "failure_class", "launch_error", "agent_version_observation",
        "failure_stage", "initialize_observed", "session_observed", "prompt_send_attempted",
        "prompt_send_completed", "elapsed_milliseconds", "prompt_slot_fsynced_before_send",
        "permission_request_count", "permission_action_matches_sentinel", "permission_reply",
        "prompt_result", "tool_call_updates", "sentinel_unchanged", "unapproved_effect_absent",
        "exit_code", "cleanup", "descendant_cleanup", "stderr", "raw_protocol_persisted",
        "raw_stderr_persisted", "completed_at",
    }
    protocol_fields = {
        "permission_outcome", "permission_action_attributed", "correlation_valid",
        "tool_announced", "tool_announcement_count", "tool_announcement_status", "tool_status", "tool_kind",
        "execution_reported", "actual_effect_observed", "session_cancel_sent",
    }
    schema = record.get("schema")
    if not (
        schema == "copilot-cli-acp-host-permission-execution.v1" and set(record) == legacy_fields
        or schema == "copilot-cli-acp-host-permission-execution.v2"
        and set(record) == legacy_fields | {"agent_version_reported"}
        or schema == "copilot-cli-acp-host-permission-execution.v3"
        and set(record) == legacy_fields | {"agent_version_reported"} | protocol_fields
    ):
        raise ValueError("execution record fields differ from the reviewed safe vocabulary")
    if schema not in {
        "copilot-cli-acp-host-permission-execution.v1",
        "copilot-cli-acp-host-permission-execution.v2",
        "copilot-cli-acp-host-permission-execution.v3",
    }:
        raise ValueError("unexpected host permission execution record schema")
    if record.get("plan_sha256") != sha256_file(PLAN_PATH):
        raise ValueError("execution record does not bind the reviewed plan")
    if record.get("runner_sha256") != sha256_file(SCRIPT_PATH):
        raise ValueError("execution record does not bind the reviewed runner")
    if record.get("execution_kind") != "fake" or record.get("original_execution") is not False:
        raise ValueError("preparation record misstates its execution kind")
    if record.get("raw_protocol_persisted") is not False or record.get("raw_stderr_persisted") is not False:
        raise ValueError("execution record persisted raw diagnostics")
    if record.get("descendant_cleanup") != "unknown-for-arbitrary-vendor-descendants":
        raise ValueError("record overstates descendant cleanup")
    if not isinstance(record.get("permission_reply"), str) or record["permission_reply"] not in {"none", "cancelled"}:
        raise ValueError("record contains a permission approval")
    serialized = json.dumps(record, sort_keys=True)
    if any(marker in serialized for marker in FORBIDDEN_PERSISTED_MARKERS):
        raise ValueError("execution record contains a forbidden secret or raw fixture marker")
    if any(raw_field in record for raw_field in ("stdout", "raw_stderr", "protocol", "auth_response")):
        raise ValueError("execution record has an unbounded or sensitive raw field")
    stderr = record.get("stderr")
    if isinstance(stderr, dict) and set(stderr) != {
        "category", "captured_bytes", "total_bytes_capped", "count_limit_reached",
        "raw_persisted", "raw_displayed",
    }:
        raise ValueError("stderr record fields differ from the sanitized vocabulary")
    if (
        not isinstance(stderr, dict)
        or stderr.get("raw_persisted") is not False
        or stderr.get("raw_displayed") is not False
        or not isinstance(stderr.get("category"), str)
        or stderr.get("category") not in {
            "empty",
            "dynamic-loader-marker",
            "access-denied-marker",
            "sandbox-marker",
            "nonempty-unclassified",
        }
        or type(stderr.get("captured_bytes")) is not int
        or not 0 <= stderr["captured_bytes"] <= STDERR_CAPTURE_LIMIT
        or type(stderr.get("total_bytes_capped")) is not int
        or not stderr["captured_bytes"] <= stderr["total_bytes_capped"] <= STDERR_COUNT_LIMIT
        or not isinstance(stderr.get("count_limit_reached"), bool)
    ):
        raise ValueError("execution record lacks sanitized stderr accounting")
    cleanup = record.get("cleanup")
    if isinstance(cleanup, dict) and set(cleanup) != {
        "root_exit_observed", "process_group_empty_observed", "stdout_reader_joined",
        "stderr_reader_joined", "streams_joined", "descendant_cleanup",
    }:
        raise ValueError("cleanup record fields differ from the observed vocabulary")
    cleanup_fields = (
        "root_exit_observed",
        "process_group_empty_observed",
        "stdout_reader_joined",
        "stderr_reader_joined",
        "streams_joined",
    )
    if (
        not isinstance(cleanup, dict)
        or cleanup.get("descendant_cleanup") != "unknown-for-arbitrary-vendor-descendants"
        or any(not isinstance(cleanup.get(name), bool) for name in cleanup_fields)
        or cleanup.get("streams_joined")
        != (cleanup.get("stdout_reader_joined") and cleanup.get("stderr_reader_joined"))
    ):
        raise ValueError("execution record has invalid cleanup observations")
    failure_classes = {
        "none",
        "launch-failure",
        "protocol-write-failure",
        "initialize-rpc-error",
        "initialize-protocol-mismatch",
        "agent-version-mismatch",
        "agent-version-invalid",
        "agent-version-unobserved",
        "session-new-rpc-error",
        "session-new-protocol-mismatch",
        "sentinel-changed-before-prompt",
        "unapproved-effect-before-prompt",
        "unapproved-effect-observed",
        "permission-request-missing-id",
        "permission-action-mismatch",
        "duplicate-permission-request",
        "tool-call-without-host-permission",
        "permission-request-missing",
        "prompt-not-cancelled",
        "unexpected-callback",
        "unexpected-response-id",
        "unexpected-callback-before-response",
        "unrequested-tool-call-update",
        "session-id-mismatch",
        "malformed-session-update",
        "tool-call-id-invalid",
        "tool-call-status-invalid",
        "tool-call-kind-invalid",
        "tool-call-snapshot-invalid",
        "tool-call-update-uncorrelated",
        "permission-tool-call-uncorrelated",
        "permission-action-unattributable",
        "permission-request-malformed",
        "duplicate-tool-call-announcement",
        "tool-execution-before-permission",
        "tool-execution-after-cancel",
        "unexpected-session-update",
        "prompt-rpc-error",
        "malformed-frame",
        "inbound-frame-oversized",
        "outbound-frame-oversized",
        "unknown-eof",
        "timeout",
        "permission-proof-incomplete",
    }
    if (
        not isinstance(record.get("status"), str)
        or record.get("status") not in {"passed", "failed"}
        or not isinstance(record.get("failure_class"), str)
        or record.get("failure_class") not in failure_classes
    ):
        raise ValueError("execution record status is outside its closed vocabulary")
    if not isinstance(record.get("failure_stage"), str) or record.get("failure_stage") not in {"none", "launch", "initialize", "session-new", "session-prompt", "cleanup"}:
        raise ValueError("execution record stage is outside its closed vocabulary")
    if not isinstance(record.get("scenario"), str) or record.get("scenario") not in FAKE_SCENARIOS:
        raise ValueError("execution record scenario is outside its closed vocabulary")
    if not isinstance(record.get("launch_error"), str) or record.get("launch_error") not in {
        "none", "subprocess-launch-error", "pipe-or-launch-error", "pipe-error"
    }:
        raise ValueError("execution record launch diagnostic is outside its closed vocabulary")
    if (
        not isinstance(record.get("agent_version_observation"), str)
        or record.get("agent_version_observation") not in {"matched-1.0.93", "mismatch", "invalid", "unknown"}
    ):
        raise ValueError("execution record agent-version observation is outside its closed vocabulary")
    if "agent_version_reported" in record:
        reported = record["agent_version_reported"]
        observation = record["agent_version_observation"]
        if reported is not None and sanitized_reported_version(reported) != reported:
            raise ValueError("execution record reported version is not a bounded public SemVer")
        if (
            observation == "matched-1.0.93" and reported != "1.0.93"
            or observation == "mismatch" and (reported is None or reported == "1.0.93")
            or observation in {"invalid", "unknown"} and reported is not None
        ):
            raise ValueError("execution record reported version conflicts with its classification")
        expected_version_failure = {
            "mismatch": "agent-version-mismatch",
            "invalid": "agent-version-invalid",
        }.get(observation)
        if expected_version_failure is not None and record.get("failure_class") != expected_version_failure:
            raise ValueError("execution record failure class conflicts with its version classification")
        if observation == "unknown" and (
            record.get("session_observed") is True
            or record.get("prompt_send_attempted") is True
            or record.get("failure_stage") not in {"launch", "initialize"}
        ):
            raise ValueError("unknown agent version followed an accepted initialize")
    if not isinstance(record.get("elapsed_milliseconds"), int) or not 0 <= record["elapsed_milliseconds"] <= MAX_OUTER_SECONDS * 1000:
        raise ValueError("execution record exceeds the operation ceiling")
    if not isinstance(record.get("prompt_send_attempted"), bool) or not isinstance(record.get("prompt_send_completed"), bool):
        raise ValueError("execution record prompt-send observations are invalid")
    if record.get("prompt_send_completed") and not record.get("prompt_send_attempted"):
        raise ValueError("prompt send completed without a recorded send attempt")
    if (
        not isinstance(record.get("prompt_result"), str)
        or record.get("prompt_result") not in {"none", "cancelled", "end_turn", "unknown", "other"}
    ):
        raise ValueError("prompt result is outside the sanitized outcome vocabulary")
    if record.get("exit_code") is not None and (
        not isinstance(record.get("exit_code"), int) or not -255 <= record["exit_code"] <= 255
    ):
        raise ValueError("execution record exit code is invalid")
    if (
        type(record.get("permission_request_count")) is not int
        or not 0 <= record["permission_request_count"] <= MAX_PROMPT_MESSAGES
        or not isinstance(record.get("permission_action_matches_sentinel"), bool)
        or type(record.get("tool_call_updates")) is not int
        or not 0 <= record["tool_call_updates"] <= MAX_PROMPT_MESSAGES
        or not isinstance(record.get("prompt_slot_fsynced_before_send"), bool)
        or not isinstance(record.get("sentinel_unchanged"), bool)
        or not isinstance(record.get("unapproved_effect_absent"), bool)
    ):
        raise ValueError("execution record observations have invalid types or bounds")
    if schema == "copilot-cli-acp-host-permission-execution.v3":
        if (
            record.get("permission_outcome") not in {"none", "cancelled"}
            or any(not isinstance(record.get(name), bool) for name in (
                "permission_action_attributed", "correlation_valid", "tool_announced",
                "execution_reported", "actual_effect_observed", "session_cancel_sent",
            ))
            or type(record.get("tool_announcement_count")) is not int
            or not 0 <= record["tool_announcement_count"] <= MAX_PROMPT_MESSAGES
            or record.get("tool_announcement_status") not in {"unknown", *TOOL_STATUSES}
            or record.get("tool_status") not in {"unknown", *TOOL_STATUSES}
            or record.get("tool_kind") not in {"unknown", *TOOL_KINDS, "other"}
        ):
            raise ValueError("execution record protocol state is outside its safe vocabulary")
    if record.get("status") == "passed" and (
        record.get("failure_class") != "none"
        or record.get("initialize_observed") is not True
        or record.get("session_observed") is not True
        or record.get("agent_version_observation") != "matched-1.0.93"
        or "agent_version_reported" in record and record.get("agent_version_reported") != "1.0.93"
        or record.get("prompt_send_attempted") is not True
        or record.get("prompt_send_completed") is not True
        or record.get("prompt_slot_fsynced_before_send") is not True
        or record.get("permission_request_count") != 1
        or record.get("permission_action_matches_sentinel") is not True
        or record.get("permission_reply") != "cancelled"
        or schema == "copilot-cli-acp-host-permission-execution.v3"
        and record.get("permission_outcome") != "cancelled"
        or record.get("prompt_result") != "cancelled"
        or record.get("sentinel_unchanged") is not True
        or record.get("unapproved_effect_absent") is not True
        or cleanup.get("root_exit_observed") is not True
        or cleanup.get("process_group_empty_observed") is not True
        or cleanup.get("streams_joined") is not True
        or schema == "copilot-cli-acp-host-permission-execution.v3"
        and (
            record.get("tool_announced") is not True
            or record.get("tool_announcement_count") != 1
            or record.get("permission_action_attributed") is not True
            or record.get("correlation_valid") is not True
            or record.get("session_cancel_sent") is not True
            or record.get("tool_announcement_status") != "pending"
            or record.get("actual_effect_observed") is not False
            or record.get("tool_status") not in {"pending", "failed"}
        )
    ):
        raise ValueError("passing execution record lacks the complete fake cancellation proof")
    if record.get("status") == "failed" and record.get("failure_class") == "none":
        raise ValueError("failed execution record lacks a failure classification")
    return {"status": "valid", "execution_kind": "fake"}


def write_attempt_ledger(
    path: Path,
    plan: dict[str, Any],
    execution_kind: str,
    *,
    authority_sha256: str | None = None,
) -> bytes:
    if execution_kind not in {"fake", "original"}:
        raise ValueError("unknown execution kind")
    if execution_kind == "original" and (
        authority_sha256 is None or len(authority_sha256) != 64
    ):
        raise RuntimeError("original execution requires a validated exact authority")
    value = {
        "schema": "copilot-cli-acp-host-attempt-consumed.v1",
        "plan_id": plan["plan_id"],
        "plan_sha256": sha256_file(plan_path_for(plan)),
        "preparation_runner_sha256": plan["runner"].get("sha256", plan["runner"].get("preparation_runner_sha256")),
        "runner_sha256": expected_runner_sha256(plan, execution_kind),
        "version": plan["scope"]["target_version"] if "scope" in plan else plan["artifacts"]["route_version"],
        "wrapper_archive_sha256": plan["artifacts"]["wrapper"]["archive_sha256"] if "scope" in plan else plan["artifacts"]["wrapper_archive_sha256"],
        "native_archive_sha256": plan["artifacts"]["native"]["archive_sha256"] if "scope" in plan else plan["artifacts"]["native_archive_sha256"],
        "native_executable_sha256": plan["artifacts"]["native"]["executable_sha256"] if "scope" in plan else plan["artifacts"]["native_executable_sha256"],
        "argv_sha256": sha256_bytes(json.dumps(plan["invocation"]["argv"], separators=(",", ":")).encode()),
        "environment_policy": plan["invocation"]["environment_policy"],
        "account_ref": plan["invocation"]["existing_account"],
        "action_sha256": sha256_bytes(json.dumps(plan["action"], sort_keys=True, separators=(",", ":")).encode()),
        "budgets": plan["attempt"],
        "execution_kind": execution_kind,
        "invocation_consumed_before_launch": True,
        "created_at": utc_now(),
    }
    if plan.get("plan_id") == FINAL_CORRECTION_PLAN_ID:
        value.update({
            "task_number": plan["task_number"],
            "task_id": plan["task_id"],
            "run_id": plan["run_id"],
            "operator_decision": plan["operator_decision"],
        })
    if execution_kind == "original":
        value["execution_authority_sha256"] = authority_sha256
    return durable_create_once(path, value)


def validate_attempt_record(
    value: dict[str, Any],
    plan: dict[str, Any],
    execution_kind: str,
    *,
    authority_sha256: str | None = None,
) -> None:
    expected = {
        "schema": "copilot-cli-acp-host-attempt-consumed.v1",
        "plan_id": plan["plan_id"],
        "plan_sha256": sha256_file(plan_path_for(plan)),
        "preparation_runner_sha256": plan["runner"].get("sha256", plan["runner"].get("preparation_runner_sha256")),
        "runner_sha256": expected_runner_sha256(plan, execution_kind),
        "version": plan["scope"]["target_version"] if "scope" in plan else plan["artifacts"]["route_version"],
        "wrapper_archive_sha256": EXPECTED_WRAPPER_ARCHIVE_SHA256,
        "native_archive_sha256": EXPECTED_NATIVE_ARCHIVE_SHA256,
        "native_executable_sha256": EXPECTED_EXECUTABLE_SHA256,
        "argv_sha256": sha256_bytes(json.dumps(plan["invocation"]["argv"], separators=(",", ":")).encode()),
        "environment_policy": plan["invocation"]["environment_policy"],
        "account_ref": "betterthanclay",
        "action_sha256": sha256_bytes(json.dumps(plan["action"], sort_keys=True, separators=(",", ":")).encode()),
        "budgets": plan["attempt"],
        "execution_kind": execution_kind,
        "invocation_consumed_before_launch": True,
    }
    if execution_kind == "original":
        expected["execution_authority_sha256"] = authority_sha256
    if plan.get("plan_id") == FINAL_CORRECTION_PLAN_ID:
        expected.update({
            "task_number": plan["task_number"],
            "task_id": plan["task_id"],
            "run_id": plan["run_id"],
            "operator_decision": plan["operator_decision"],
        })
    if (
        set(value) != set(expected) | {"created_at"}
        or not isinstance(value.get("created_at"), str)
        or any(value.get(key) != expected_value for key, expected_value in expected.items())
    ):
        raise ValueError("one-shot attempt record does not bind the reviewed plan")


def write_prompt_slot(path: Path, attempt_bytes: bytes, plan: dict[str, Any]) -> bytes:
    value = {
        "schema": "copilot-cli-acp-host-prompt-slot-consumed.v1",
        "plan_id": plan["plan_id"],
        "plan_sha256": sha256_file(plan_path_for(plan)),
        "attempt_record_sha256": sha256_bytes(attempt_bytes),
        "version": plan["scope"]["target_version"] if "scope" in plan else plan["artifacts"]["route_version"],
        "slot": 1,
        "prompt_consumed_before_send": True,
        "created_at": utc_now(),
    }
    return durable_create_once(path, value)


def validate_prompt_slot(value: dict[str, Any], attempt_bytes: bytes, plan: dict[str, Any]) -> None:
    expected = {
        "schema": "copilot-cli-acp-host-prompt-slot-consumed.v1",
        "plan_id": plan["plan_id"],
        "plan_sha256": sha256_file(plan_path_for(plan)),
        "attempt_record_sha256": sha256_bytes(attempt_bytes),
        "version": plan["scope"]["target_version"] if "scope" in plan else plan["artifacts"]["route_version"],
        "slot": 1,
        "prompt_consumed_before_send": True,
    }
    if (
        set(value) != set(expected) | {"created_at"}
        or not isinstance(value.get("created_at"), str)
        or any(value.get(key) != expected_value for key, expected_value in expected.items())
    ):
        raise ValueError("prompt slot does not bind the reviewed attempt")


def validate_original_execution_record(
    record: dict[str, Any],
    plan: dict[str, Any],
    authority_sha256: str,
    *,
    allow_fake_authority: bool = False,
) -> dict[str, Any]:
    legacy_fields = {
        "schema", "execution_kind", "original_execution", "plan_sha256",
        "preparation_runner_sha256", "runner_sha256", "authority_sha256",
        "attempt_record_sha256", "prompt_slot_record_sha256", "target_version",
        "argv_sha256", "status", "failure_class", "launch_error", "failure_stage",
        "agent_version_observation", "model_observation", "initialize_observed",
        "session_observed", "prompt_send_attempted", "prompt_send_completed",
        "elapsed_milliseconds", "prompt_slot_fsynced_before_send",
        "permission_request_count", "permission_action_matches_sentinel",
        "permission_reply", "prompt_result", "tool_call_updates", "sentinel_unchanged",
        "action_directory_unchanged", "exit_code", "cleanup", "descendant_cleanup",
        "stderr", "raw_protocol_persisted", "raw_stderr_persisted", "completed_at",
    }
    protocol_fields = {
        "permission_outcome", "permission_action_attributed", "correlation_valid",
        "tool_announced", "tool_announcement_count", "tool_announcement_status", "tool_status", "tool_kind",
        "execution_reported", "actual_effect_observed", "session_cancel_sent",
    }
    schema = record.get("schema")
    if not (
        schema == "copilot-cli-acp-host-permission-original-execution.v1" and set(record) == legacy_fields
        or schema == "copilot-cli-acp-host-permission-original-execution.v2"
        and set(record) == legacy_fields | {"agent_version_reported"}
        or schema == "copilot-cli-acp-host-permission-original-execution.v3"
        and set(record) == legacy_fields | {
            "agent_version_reported", "task_number", "task_id", "run_id",
            "operator_decision", "package_cache_cleanup", "package_cache_cleanup_error",
        }
        or schema == "copilot-cli-acp-host-permission-original-execution.v4"
        and frozenset(record) in {
            frozenset(legacy_fields | protocol_fields),
            frozenset(legacy_fields | protocol_fields | {"agent_version_reported"}),
            frozenset(legacy_fields | protocol_fields | {
                "agent_version_reported", "task_number", "task_id", "run_id",
                "operator_decision", "package_cache_cleanup", "package_cache_cleanup_error",
            }),
        }
    ):
        raise ValueError("original record fields differ from the reviewed safe vocabulary")
    if (
        schema not in {
            "copilot-cli-acp-host-permission-original-execution.v1",
            "copilot-cli-acp-host-permission-original-execution.v2",
            "copilot-cli-acp-host-permission-original-execution.v3",
            "copilot-cli-acp-host-permission-original-execution.v4",
        }
        or record.get("execution_kind") != "original"
        or record.get("original_execution") is not True
        or record.get("plan_sha256") != sha256_file(plan_path_for(plan))
        or record.get("preparation_runner_sha256") != PREPARATION_RUNNER_SHA256
        or record.get("runner_sha256") != expected_runner_sha256(plan, "original")
        or record.get("authority_sha256") != authority_sha256
        or record.get("target_version") != "1.0.93"
        or record.get("argv_sha256")
        != sha256_bytes(json.dumps(plan["invocation"]["argv"], separators=(",", ":")).encode())
        or record.get("raw_protocol_persisted") is not False
        or record.get("raw_stderr_persisted") is not False
        or record.get("descendant_cleanup") != "unknown-for-arbitrary-vendor-descendants"
    ):
        raise ValueError("original record does not bind the exact authorized execution")
    if schema == "copilot-cli-acp-host-permission-original-execution.v3":
        if plan.get("plan_id") != FINAL_CORRECTION_PLAN_ID or any(
            record.get(name) != plan.get(name)
            for name in ("task_number", "task_id", "run_id", "operator_decision")
        ):
            raise ValueError("corrected original record does not bind this task and decision")
        if (
            record.get("package_cache_cleanup") not in {"not-created", "removed", "failed"}
            or record.get("package_cache_cleanup_error") not in {"none", "os-error", "cache-remains"}
            or (record.get("package_cache_cleanup") == "removed")
            != (record.get("package_cache_cleanup_error") == "none")
            or record.get("package_cache_cleanup") == "not-created"
            and record.get("package_cache_cleanup_error") != "none"
            or record.get("package_cache_cleanup") == "failed"
            and record.get("package_cache_cleanup_error") == "none"
        ):
            raise ValueError("corrected package-cache cleanup observation is invalid")
        if record.get("failure_class") == "package-cache-cleanup-failed" and (
            record.get("package_cache_cleanup") != "failed"
            or record.get("failure_stage") != "cleanup"
        ):
            raise ValueError("package-cache cleanup failure classification is inconsistent")
        if not allow_fake_authority:
            _, expected_authority_sha256 = validate_corrected_authority()
            if authority_sha256 != expected_authority_sha256:
                raise ValueError("corrected original record does not bind the task authority")
    model = record.get("model_observation")
    if model != "unobserved" and (
        not isinstance(model, str)
        or len(model) > 80
        or re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._:-]{0,79}", model) is None
    ):
        raise ValueError("model observation is outside the bounded structured vocabulary")
    serialized_record = json.dumps(record, sort_keys=True)
    if any(marker in serialized_record for marker in FORBIDDEN_PERSISTED_MARKERS):
        raise ValueError("original record contains a forbidden secret or raw fixture marker")
    if (
        record.get("status") not in {"passed", "failed"}
        or record.get("failure_class") not in {
            "none", "launch-failure", "protocol-write-failure", "initialize-rpc-error",
            "initialize-protocol-mismatch", "agent-version-mismatch",
            "agent-version-invalid",
            "agent-version-unobserved", "session-new-rpc-error",
            "session-new-protocol-mismatch", "sentinel-changed-before-prompt",
            "unexpected-action-before-prompt", "unapproved-effect-before-prompt",
            "unapproved-effect-observed", "permission-request-missing-id",
            "permission-action-mismatch", "duplicate-permission-request",
            "tool-call-without-host-permission", "permission-request-missing",
            "session-id-mismatch", "malformed-session-update", "tool-call-id-invalid",
            "tool-call-status-invalid", "tool-call-kind-invalid",
            "tool-call-snapshot-invalid", "tool-call-update-uncorrelated",
            "permission-tool-call-uncorrelated", "permission-action-unattributable",
            "permission-request-malformed",
            "duplicate-tool-call-announcement", "tool-execution-before-permission",
            "tool-execution-after-cancel",
            "unexpected-session-update", "prompt-rpc-error",
            "prompt-not-cancelled", "unexpected-callback", "unexpected-response-id",
            "unexpected-callback-before-response", "unrequested-tool-call-update",
            "malformed-frame", "inbound-frame-oversized", "outbound-frame-oversized",
            "unknown-eof", "timeout", "permission-proof-incomplete",
            "sentinel-changed-after-prompt", "package-cache-cleanup-failed",
        }
        or record.get("failure_stage") not in {
            "none", "launch", "initialize", "session-new", "session-prompt", "cleanup"
        }
        or record.get("launch_error") not in {
            "none", "subprocess-launch-error", "pipe-or-launch-error", "pipe-error"
        }
        or record.get("agent_version_observation") not in {
            "matched-1.0.93", "mismatch", "invalid", "unknown"
        }
        or record.get("permission_reply") not in {"none", "cancelled"}
        or record.get("prompt_result") not in {"none", "cancelled", "end_turn", "unknown", "other"}
        or type(record.get("elapsed_milliseconds")) is not int
        or not 0 <= record["elapsed_milliseconds"] <= MAX_OUTER_SECONDS * 1000
        or not isinstance(record.get("initialize_observed"), bool)
        or not isinstance(record.get("session_observed"), bool)
        or not isinstance(record.get("prompt_send_attempted"), bool)
        or not isinstance(record.get("prompt_send_completed"), bool)
        or record.get("prompt_send_completed") and not record.get("prompt_send_attempted")
        or not isinstance(record.get("prompt_slot_fsynced_before_send"), bool)
        or type(record.get("permission_request_count")) is not int
        or not 0 <= record["permission_request_count"] <= (
            MAX_PROMPT_MESSAGES
            if schema == "copilot-cli-acp-host-permission-original-execution.v4"
            else 2
        )
        or not isinstance(record.get("permission_action_matches_sentinel"), bool)
        or type(record.get("tool_call_updates")) is not int
        or not 0 <= record["tool_call_updates"] <= (MAX_PROMPT_MESSAGES if schema == "copilot-cli-acp-host-permission-original-execution.v4" else 1)
        or not isinstance(record.get("sentinel_unchanged"), bool)
        or not isinstance(record.get("action_directory_unchanged"), bool)
        or record.get("exit_code") is not None
        and (type(record.get("exit_code")) is not int or not -255 <= record["exit_code"] <= 255)
        or not isinstance(record.get("completed_at"), str)
    ):
        raise ValueError("original record has invalid bounded outcomes or observations")
    if schema == "copilot-cli-acp-host-permission-original-execution.v4" and (
        record.get("permission_outcome") not in {"none", "cancelled"}
        or any(not isinstance(record.get(name), bool) for name in (
            "permission_action_attributed", "correlation_valid", "tool_announced",
            "execution_reported", "actual_effect_observed", "session_cancel_sent",
        ))
        or type(record.get("tool_announcement_count")) is not int
        or not 0 <= record["tool_announcement_count"] <= MAX_PROMPT_MESSAGES
        or record.get("tool_announcement_status") not in {"unknown", *TOOL_STATUSES}
        or record.get("tool_status") not in {"unknown", *TOOL_STATUSES}
        or record.get("tool_kind") not in {"unknown", *TOOL_KINDS, "other"}
    ):
        raise ValueError("original record protocol state is outside its safe vocabulary")
    if "agent_version_reported" in record:
        reported = record["agent_version_reported"]
        observation = record["agent_version_observation"]
        if reported is not None and sanitized_reported_version(reported) != reported:
            raise ValueError("original record reported version is not a bounded public SemVer")
        if (
            observation == "matched-1.0.93" and reported != "1.0.93"
            or observation == "mismatch" and (reported is None or reported == "1.0.93")
            or observation in {"invalid", "unknown"} and reported is not None
            or observation == "mismatch" and record.get("failure_class") != "agent-version-mismatch"
            or observation == "invalid" and record.get("failure_class") != "agent-version-invalid"
        ):
            raise ValueError("original record reported version conflicts with its classification")
        if observation == "unknown" and (
            record.get("session_observed") is True
            or record.get("prompt_send_attempted") is True
            or record.get("failure_stage") not in {"launch", "initialize"}
        ):
            raise ValueError("unknown agent version followed an accepted initialize")
    for name in ("attempt_record_sha256", "prompt_slot_record_sha256"):
        value = record.get(name)
        if value is not None and (not isinstance(value, str) or re.fullmatch(r"[0-9a-f]{64}", value) is None):
            raise ValueError("original record has an invalid consumed-ledger digest")
    if record.get("attempt_record_sha256") is None:
        raise ValueError("original record lacks the consumed invocation ledger")
    cleanup = record.get("cleanup")
    if (
        not isinstance(cleanup, dict)
        or set(cleanup) != {
            "root_exit_observed", "process_group_empty_observed", "stdout_reader_joined",
            "stderr_reader_joined", "streams_joined", "descendant_cleanup",
        }
        or any(not isinstance(cleanup.get(key), bool) for key in (
            "root_exit_observed", "process_group_empty_observed", "stdout_reader_joined",
            "stderr_reader_joined", "streams_joined",
        ))
        or cleanup.get("streams_joined")
        != (cleanup.get("stdout_reader_joined") and cleanup.get("stderr_reader_joined"))
        or cleanup.get("descendant_cleanup") != "unknown-for-arbitrary-vendor-descendants"
    ):
        raise ValueError("original cleanup observations are invalid")
    stderr = record.get("stderr")
    if (
        not isinstance(stderr, dict)
        or set(stderr) != {
            "category", "captured_bytes", "total_bytes_capped", "count_limit_reached",
            "raw_persisted", "raw_displayed",
        }
        or stderr.get("category") not in {
            "empty", "dynamic-loader-marker", "access-denied-marker", "sandbox-marker",
            "nonempty-unclassified",
        }
        or type(stderr.get("captured_bytes")) is not int
        or not 0 <= stderr["captured_bytes"] <= STDERR_CAPTURE_LIMIT
        or type(stderr.get("total_bytes_capped")) is not int
        or not stderr["captured_bytes"] <= stderr["total_bytes_capped"] <= STDERR_COUNT_LIMIT
        or not isinstance(stderr.get("count_limit_reached"), bool)
        or stderr.get("raw_persisted") is not False
        or stderr.get("raw_displayed") is not False
    ):
        raise ValueError("original stderr accounting is not sanitized")
    if record.get("status") == "passed" and (
        record.get("failure_class") != "none"
        or record.get("agent_version_observation") != "matched-1.0.93"
        or "agent_version_reported" in record and record.get("agent_version_reported") != "1.0.93"
        or record.get("initialize_observed") is not True
        or record.get("session_observed") is not True
        or record.get("prompt_send_attempted") is not True
        or record.get("prompt_send_completed") is not True
        or record.get("prompt_slot_fsynced_before_send") is not True
        or record.get("permission_request_count") != 1
        or record.get("permission_action_matches_sentinel") is not True
        or record.get("permission_reply") != "cancelled"
        or schema == "copilot-cli-acp-host-permission-original-execution.v4"
        and record.get("permission_outcome") != "cancelled"
        or record.get("prompt_result") != "cancelled"
        or record.get("sentinel_unchanged") is not True
        or record.get("action_directory_unchanged") is not True
        or cleanup.get("root_exit_observed") is not True
        or cleanup.get("process_group_empty_observed") is not True
        or cleanup.get("streams_joined") is not True
        or schema == "copilot-cli-acp-host-permission-original-execution.v3"
        and (
            record.get("package_cache_cleanup") != "removed"
            or record.get("package_cache_cleanup_error") != "none"
        )
        or schema == "copilot-cli-acp-host-permission-original-execution.v4"
        and (
            record.get("tool_announced") is not True
            or record.get("tool_announcement_count") != 1
            or record.get("permission_action_attributed") is not True
            or record.get("correlation_valid") is not True
            or record.get("session_cancel_sent") is not True
            or record.get("tool_announcement_status") != "pending"
            or record.get("actual_effect_observed") is not False
            or record.get("tool_status") not in {"pending", "failed"}
        )
    ):
        raise ValueError("passing original record lacks the reviewed cancellation and cleanup evidence")
    if record.get("status") == "failed" and record.get("failure_class") == "none":
        raise ValueError("failed original record lacks a finite failure classification")
    return {"status": "valid", "execution_kind": "original", "observation": record["status"]}


def validate_original_execution_evidence(
    record_path: Path,
    attempt_path: Path,
    prompt_path: Path,
    plan: dict[str, Any],
    authority_sha256: str,
    *,
    allow_fake_authority: bool = False,
    require_private_records: bool = True,
) -> dict[str, Any]:
    evidence_paths = (
        (record_path, True),
        (attempt_path, True),
        (prompt_path, prompt_path.exists() or prompt_path.is_symlink()),
    )
    for path, required in evidence_paths:
        try:
            metadata = path.lstat()
        except FileNotFoundError:
            if required:
                raise ValueError("original evidence record is missing") from None
            continue
        if not stat.S_ISREG(metadata.st_mode):
            raise ValueError("original evidence record is not a regular file")
        if require_private_records and (
            metadata.st_uid != os.getuid() or metadata.st_mode & 0o777 != 0o600
        ):
            raise ValueError("original evidence record is not a private current-user file")
    record = load_object(record_path)
    result = validate_original_execution_record(
        record, plan, authority_sha256, allow_fake_authority=allow_fake_authority
    )
    if attempt_path.is_symlink() or not attempt_path.is_file():
        raise ValueError("original invocation ledger evidence is missing or linked")
    attempt_bytes = attempt_path.read_bytes()
    attempt = load_object(attempt_path)
    validate_attempt_record(
        attempt, plan, "original", authority_sha256=authority_sha256
    )
    prompt_bytes: bytes | None = None
    if prompt_path.exists() or prompt_path.is_symlink():
        if prompt_path.is_symlink() or not prompt_path.is_file():
            raise ValueError("consumed prompt slot evidence is missing or linked")
        prompt_bytes = prompt_path.read_bytes()
    validate_original_ledger_bindings(
        record, attempt_bytes, prompt_bytes, plan, authority_sha256
    )
    return result


def validate_corrected_original_evidence(public_record_path: Path) -> dict[str, Any]:
    if public_record_path.resolve() != CORRECTED_EXECUTION_EVIDENCE_PATH.resolve():
        raise ValueError("corrected original validation requires the allocated Research 437 record")
    plan = validate_final_correction_plan()
    _, authority_sha256 = validate_corrected_authority(plan=plan)
    paths = original_ledger_paths(plan)
    result = validate_original_execution_evidence(
        paths["execution"], paths["attempt"], paths["prompt"], plan, authority_sha256
    )
    public_pairs = [
        (CORRECTED_ATTEMPT_EVIDENCE_PATH, paths["attempt"]),
        (CORRECTED_EXECUTION_EVIDENCE_PATH, paths["execution"]),
    ]
    private_prompt_present = paths["prompt"].exists() or paths["prompt"].is_symlink()
    public_prompt_present = (
        CORRECTED_PROMPT_EVIDENCE_PATH.exists()
        or CORRECTED_PROMPT_EVIDENCE_PATH.is_symlink()
    )
    if private_prompt_present != public_prompt_present:
        raise ValueError("public prompt-slot evidence does not match the consumed private ledger")
    if private_prompt_present:
        public_pairs.append((CORRECTED_PROMPT_EVIDENCE_PATH, paths["prompt"]))
    for public_path, private_path in public_pairs:
        if (
            public_path.is_symlink()
            or not public_path.is_file()
            or public_path.read_bytes() != private_path.read_bytes()
        ):
            raise ValueError("Research 437 evidence copy differs from its durable private record")
    return {
        **result,
        "invocations_consumed": 1,
        "prompt_slots_consumed": 1 if private_prompt_present else 0,
        "prompt_slots_remaining": 2 if private_prompt_present else 3,
    }


def validate_original_ledger_bindings(
    record: dict[str, Any],
    attempt_bytes: bytes,
    prompt_bytes: bytes | None,
    plan: dict[str, Any],
    authority_sha256: str,
) -> None:
    attempt = json.loads(attempt_bytes)
    if not isinstance(attempt, dict):
        raise ValueError("original invocation ledger is not a structured record")
    validate_attempt_record(attempt, plan, "original", authority_sha256=authority_sha256)
    if record["attempt_record_sha256"] != sha256_bytes(attempt_bytes):
        raise ValueError("original execution record does not bind its invocation ledger")
    prompt_digest = record["prompt_slot_record_sha256"]
    if prompt_digest is None:
        if (
            prompt_bytes is not None
            or record["prompt_slot_fsynced_before_send"]
            or record["prompt_send_attempted"]
        ):
            raise ValueError("unconsumed prompt slot evidence conflicts with the execution record")
    else:
        if prompt_bytes is None:
            raise ValueError("consumed prompt slot evidence is missing")
        prompt = json.loads(prompt_bytes)
        if not isinstance(prompt, dict):
            raise ValueError("consumed prompt slot evidence is not a structured record")
        validate_prompt_slot(prompt, attempt_bytes, plan)
        if (
            prompt_digest != sha256_bytes(prompt_bytes)
            or record["prompt_slot_fsynced_before_send"] is not True
            or record["prompt_send_attempted"] is not True
        ):
            raise ValueError("original execution record does not bind its consumed prompt slot")


def validate_archive_payload(
    archive_bytes: bytes,
    package_identity: dict[str, Any],
    inventory_package: dict[str, Any],
    *,
    output_executable: Path | None = None,
) -> None:
    if sha256_bytes(archive_bytes) != package_identity["archive_sha256"]:
        raise ValueError("official tarball SHA-256 does not match the reviewed artifact")
    sri = "sha512-" + base64.b64encode(hashlib.sha512(archive_bytes).digest()).decode("ascii")
    if sri != package_identity["archive_integrity"]:
        raise ValueError("official tarball SRI does not match the reviewed artifact")
    expected_files = {item["path"]: item for item in inventory_package["files"]}
    seen: set[str] = set()
    payloads: dict[str, bytes] = {}
    try:
        with tarfile.open(fileobj=io.BytesIO(archive_bytes), mode="r:gz") as archive:
            for member in archive.getmembers():
                if member.isdir():
                    continue
                if not member.isfile() or member.name in seen:
                    raise ValueError("official tarball contains an unexpected or duplicate payload entry")
                seen.add(member.name)
                expected = expected_files.get(member.name)
                if expected is None or member.size != expected["size"] or f"{member.mode & 0o777:04o}" != expected["mode"]:
                    raise ValueError("official tarball payload inventory differs from the reviewed release")
                stream = archive.extractfile(member)
                if stream is None:
                    raise ValueError("official tarball payload could not be read")
                payload = stream.read()
                if len(payload) != expected["size"] or sha256_bytes(payload) != expected["sha256"]:
                    raise ValueError("official tarball payload digest differs from the reviewed release")
                payloads[member.name] = payload
    except (tarfile.TarError, OSError):
        raise ValueError("official tarball could not be read as the reviewed package") from None
    if seen != set(expected_files):
        raise ValueError("official tarball does not contain the complete reviewed package inventory")
    manifest = payloads.get("package/package.json")
    if manifest is None or sha256_bytes(manifest) != package_identity["package_manifest_sha256"]:
        raise ValueError("official package manifest does not match the reviewed release")
    try:
        manifest_value = json.loads(manifest)
    except (UnicodeDecodeError, json.JSONDecodeError):
        raise ValueError("official package manifest is not valid JSON") from None
    if (
        manifest_value.get("name") != package_identity["package"]
        or manifest_value.get("version") != package_identity["version"]
    ):
        raise ValueError("official package manifest labels differ from the reviewed release")
    if package_identity["package"] == "@github/copilot-darwin-arm64":
        if manifest_value.get("os") != ["darwin"] or manifest_value.get("cpu") != ["arm64"]:
            raise ValueError("official native manifest platform differs from the reviewed route")
        if output_executable is None:
            raise ValueError("native payload staging path is required")
        executable = payloads.get("package/copilot")
        if executable is None or sha256_bytes(executable) != EXPECTED_EXECUTABLE_SHA256:
            raise ValueError("native executable digest differs from the reviewed release")
        output_executable.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
        with output_executable.open("xb") as stream:
            stream.write(executable)
            stream.flush()
            os.fsync(stream.fileno())
        output_executable.chmod(0o755)
        if sha256_file(output_executable) != EXPECTED_EXECUTABLE_SHA256:
            raise ValueError("staged native executable digest changed")
    elif package_identity["package"] == "@github/copilot":
        loader = payloads.get("package/npm-loader.js")
        if loader is None or sha256_bytes(loader) != EXPECTED_WRAPPER_LOADER_SHA256:
            raise ValueError("official wrapper loader differs from the reviewed release")
    else:
        raise ValueError("official package is outside the reviewed artifact set")


def download_official_archive(package_identity: dict[str, Any], destination: Path) -> bytes:
    url = package_identity["archive_url"]
    if not url.startswith("https://registry.npmjs.org/"):
        raise ValueError("artifact URL is outside the reviewed official registry")
    request = urllib.request.Request(url, headers={"User-Agent": "Swallowtail-frozen-artifact-check/1"})
    try:
        with urllib.request.urlopen(request, timeout=20) as response:
            if response.geturl() != url:
                raise ValueError("official tarball request redirected away from the reviewed URL")
            archive_bytes = response.read(300 * 1024 * 1024 + 1)
    except (OSError, TimeoutError):
        raise RuntimeError("official frozen tarball could not be staged") from None
    if not archive_bytes or len(archive_bytes) > 300 * 1024 * 1024:
        raise ValueError("official frozen tarball exceeded the bounded staging size")
    destination.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
    with destination.open("xb") as stream:
        stream.write(archive_bytes)
        stream.flush()
        os.fsync(stream.fileno())
    return archive_bytes


def stage_reviewed_artifacts(root: Path, plan: dict[str, Any]) -> Path:
    inventory = load_object(INVENTORY_PATH)
    wrapper = validate_inventory_package(inventory, "@github/copilot", "1.0.93")
    native = validate_inventory_package(inventory, "@github/copilot-darwin-arm64", "1.0.93")
    stage_root = root / "staged-artifacts"
    stage_root.mkdir(mode=0o700)
    if plan.get("plan_id") == FINAL_CORRECTION_PLAN_ID:
        wrapper_identity = {
            "package": "@github/copilot",
            "version": "1.0.93",
            "archive_sha256": plan["artifacts"]["wrapper_archive_sha256"],
            "package_manifest_sha256": EXPECTED_WRAPPER_MANIFEST_SHA256,
            "archive_url": wrapper["dist"]["tarball"],
            "archive_integrity": wrapper["dist"]["integrity"],
        }
        native_identity = {
            "package": "@github/copilot-darwin-arm64",
            "version": "1.0.93",
            "archive_sha256": plan["artifacts"]["native_archive_sha256"],
            "package_manifest_sha256": plan["artifacts"]["native_manifest_sha256"],
            "executable_sha256": plan["artifacts"]["native_executable_sha256"],
            "archive_url": native["dist"]["tarball"],
            "archive_integrity": native["dist"]["integrity"],
        }
    else:
        wrapper_identity = plan["artifacts"]["wrapper"]
        native_identity = plan["artifacts"]["native"]
    wrapper_archive = download_official_archive(wrapper_identity, stage_root / "wrapper-1.0.93.tgz")
    validate_archive_payload(wrapper_archive, wrapper_identity, wrapper)
    native_archive = download_official_archive(native_identity, stage_root / "native-1.0.93.tgz")
    executable = stage_root / "native" / "package" / "copilot"
    validate_archive_payload(native_archive, native_identity, native, output_executable=executable)
    expected_executable_sha256 = (
        plan["artifacts"]["native_executable_sha256"]
        if plan.get("plan_id") == FINAL_CORRECTION_PLAN_ID
        else plan["artifact_staging"]["native_executable_sha256"]
    )
    if sha256_file(executable) != expected_executable_sha256:
        raise ValueError("staged native executable differs from the frozen execution plan")
    return executable


def original_ledger_paths(plan: dict[str, Any]) -> dict[str, Path]:
    root_suffix = (
        "Task 170" if plan.get("plan_id") == FINAL_CORRECTION_PLAN_ID else "Task 168"
    )
    root = Path.home() / "Library/Application Support/Swallowtail/Copilot ACP Permission Proof" / root_suffix
    corrected = root_suffix == "Task 170"
    names = (
        ("1.0.93-corrected-attempt.json", "1.0.93-corrected-prompt-1.json", "1.0.93-corrected-execution.json")
        if corrected
        else ("1.0.93-attempt.json", "1.0.93-prompt-1.json", "1.0.93-execution.json")
    )
    paths = {"root": root, "attempt": root / names[0], "prompt": root / names[1], "execution": root / names[2]}
    expected = {
        "root": f"$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/{root_suffix}",
        "attempt": f"$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/{root_suffix}/{names[0]}",
        "prompt": f"$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/{root_suffix}/{names[1]}",
        "execution": f"$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/{root_suffix}/{names[2]}",
    }
    actual = {
        "root": plan["ledger"]["root"],
        "attempt": plan["ledger"]["attempt_path"],
        "prompt": plan["ledger"]["prompt_path"],
        "execution": plan["ledger"]["execution_path"],
    }
    if actual != expected:
        raise ValueError("reviewed durable ledger paths changed")
    return paths


def prepare_action_directory(root: Path) -> Path:
    action = root / "action"
    action.mkdir(mode=0o700)
    sentinel = action / "permission-sentinel.txt"
    with sentinel.open("xb") as stream:
        stream.write(SENTINEL_BEFORE)
        stream.flush()
        os.fsync(stream.fileno())
    return action


def original_directory_unchanged(action: Path) -> bool:
    try:
        return (
            sorted(item.name for item in action.iterdir()) == ["permission-sentinel.txt"]
            and (action / "permission-sentinel.txt").read_bytes() == SENTINEL_BEFORE
        )
    except OSError:
        return False


def corrected_launch_environment(
    base_environment: dict[str, str], package_cache_home: Path
) -> dict[str, str]:
    environment = dict(base_environment)
    environment.pop("COPILOT_CLI_VERSION", None)
    environment.pop("COPILOT_CLI_DIST_DIR", None)
    environment["COPILOT_AUTO_UPDATE"] = "false"
    environment["COPILOT_PKG_CACHE_HOME"] = str(package_cache_home)
    return environment


def run_original_session(
    executable: Path,
    action: Path,
    attempt_path: Path,
    prompt_path: Path,
    record_path: Path,
    plan: dict[str, Any],
    authority: dict[str, Any],
    authority_sha256: str,
    *,
    outer_seconds: float = MAX_OUTER_SECONDS,
    cleanup_seconds: float = MAX_CLEANUP_SECONDS,
    fake_test: bool = False,
    base_environment: dict[str, str] | None = None,
) -> dict[str, Any]:
    if not fake_test and sha256_file(executable) != EXPECTED_EXECUTABLE_SHA256:
        raise RuntimeError("staged executable identity changed before launch")
    attempt_bytes = write_attempt_ledger(
        attempt_path, plan, "original", authority_sha256=authority_sha256
    )
    attempt = load_object(attempt_path)
    validate_attempt_record(
        attempt, plan, "original", authority_sha256=authority_sha256
    )
    started = time.monotonic()
    outer_budget = min(MAX_OUTER_SECONDS, max(0.05, outer_seconds))
    cleanup_budget = min(MAX_CLEANUP_SECONDS, max(0.02, cleanup_seconds), outer_budget)
    operation_deadline = started + outer_budget - cleanup_budget
    total_deadline = started + outer_budget
    process: subprocess.Popen[bytes] | None = None
    client: StdioSession | None = None
    package_cache: tempfile.TemporaryDirectory | None = None
    child_environment: dict[str, str] | None = None
    stage = "launch"
    failure_stage = "none"
    failure = "none"
    launch_error = "none"
    initialize_observed = False
    session_observed = False
    exchange = new_permission_exchange()
    prompt_send_attempted = False
    prompt_send_completed = False
    prompt_slot_bytes: bytes | None = None
    version_observation = "unknown"
    version_reported: str | None = None
    model_observation = "unobserved"
    package_cache_cleanup = "not-created"
    package_cache_cleanup_error = "none"
    stderr = BoundedDiagnostics()
    cleanup = {
        "root_exit_observed": False,
        "process_group_empty_observed": False,
        "stdout_reader_joined": False,
        "stderr_reader_joined": False,
        "streams_joined": False,
        "descendant_cleanup": "unknown-for-arbitrary-vendor-descendants",
    }
    try:
        argv = [str(executable), *plan["invocation"]["argv"]]
        if plan.get("plan_id") in {CORRECTION_PLAN_ID, FINAL_CORRECTION_PLAN_ID}:
            package_cache = tempfile.TemporaryDirectory(
                prefix="copilot-acp-package-cache.", dir=action.parent
            )
            package_cache_cleanup = "pending"
            child_environment = corrected_launch_environment(
                os.environ if base_environment is None else base_environment,
                Path(package_cache.name),
            )
        process = subprocess.Popen(
            argv,
            cwd=action,
            env=child_environment,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            bufsize=0,
            start_new_session=True,
        )
        client = StdioSession(process)
        stage = "initialize"
        client.send({
            "jsonrpc": "2.0", "id": 1, "method": "initialize",
            "params": {"protocolVersion": 1, "clientInfo": {"name": "swallowtail-normal-host-proof", "version": "1"}},
        })
        initialize = response_for(client, 1, operation_deadline)
        if "error" in initialize:
            failure = "initialize-rpc-error"
            raise ProtocolFailure(failure)
        result = initialize.get("result")
        if not isinstance(result, dict) or result.get("protocolVersion") != 1:
            failure = "initialize-protocol-mismatch"
            raise ProtocolFailure(failure)
        agent_info = result.get("agentInfo")
        version = agent_info.get("version") if isinstance(agent_info, dict) else None
        version_observation, version_reported = classify_agent_version(version)
        if version_observation != "matched-1.0.93":
            failure = {
                "mismatch": "agent-version-mismatch",
                "invalid": "agent-version-invalid",
                "unknown": "agent-version-unobserved",
            }[version_observation]
            raise ProtocolFailure(failure)
        for container in (result, agent_info):
            if isinstance(container, dict):
                candidate = container.get("modelId", container.get("model"))
                if isinstance(candidate, str) and re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._:-]{0,79}", candidate):
                    model_observation = candidate
                    break
        initialize_observed = True
        stage = "session-new"
        client.send({"jsonrpc": "2.0", "method": "initialized", "params": {}})
        client.send({
            "jsonrpc": "2.0", "id": 2, "method": "session/new",
            "params": {"cwd": str(action), "mcpServers": []},
        })
        session = response_for(client, 2, operation_deadline)
        if "error" in session:
            failure = "session-new-rpc-error"
            raise ProtocolFailure(failure)
        session_result = session.get("result")
        session_id = session_result.get("sessionId") if isinstance(session_result, dict) else None
        if not isinstance(session_id, str) or not session_id or len(session_id) > MAX_SESSION_ID_LENGTH:
            failure = "session-new-protocol-mismatch"
            raise ProtocolFailure(failure)
        if model_observation == "unobserved" and isinstance(session_result, dict):
            candidate = session_result.get("modelId", session_result.get("model"))
            if isinstance(candidate, str) and re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._:-]{0,79}", candidate):
                model_observation = candidate
        session_observed = True
        if not original_directory_unchanged(action):
            failure = "unexpected-action-before-prompt"
            raise ProtocolFailure(failure)
        prompt_slot_bytes = write_prompt_slot(prompt_path, attempt_bytes, plan)
        stage = "session-prompt"
        prompt_send_attempted = True
        client.send({
            "jsonrpc": "2.0", "id": 3, "method": "session/prompt",
            "params": {"sessionId": session_id, "prompt": [{"type": "text", "text": PROMPT_TEXT}]},
        })
        prompt_send_completed = True
        exchange = permission_exchange(
            client, session_id, action, operation_deadline,
            lambda: original_directory_unchanged(action),
        )
        if exchange["failure"] != "none":
            failure = exchange["failure"]
    except ProtocolFailure as error:
        if failure == "none":
            failure = error.category
        failure_stage = stage
    except subprocess.SubprocessError:
        launch_error = "subprocess-launch-error"
        failure = "launch-failure"
        failure_stage = stage
    except OSError:
        launch_error = "pipe-or-launch-error" if process is None else "pipe-error"
        failure = "launch-failure" if process is None else "protocol-write-failure"
        failure_stage = stage
    except ValueError:
        launch_error = "pipe-or-launch-error" if process is None else "pipe-error"
        failure = "launch-failure" if process is None else "protocol-write-failure"
        failure_stage = stage
    except RuntimeError:
        failure = "launch-failure"
        failure_stage = stage
    finally:
        if failure != "none" and failure_stage == "none":
            failure_stage = stage
        if process is not None and client is not None:
            cleanup = cleanup_process(
                process, client, min(total_deadline, time.monotonic() + cleanup_budget)
            )
            stderr = client.stderr_diagnostics
        elif process is not None:
            cleanup_deadline = min(total_deadline, time.monotonic() + cleanup_budget)
            if process.stdin is not None:
                try:
                    process.stdin.close()
                except OSError:
                    pass
            if process.poll() is None or process_group_exists(process.pid):
                signal_owned_group(process, signal.SIGTERM)
            try:
                process.wait(timeout=max(0.0, cleanup_deadline - time.monotonic()))
            except subprocess.TimeoutExpired:
                signal_owned_group(process, signal.SIGKILL)
                try:
                    process.wait(timeout=max(0.0, cleanup_deadline - time.monotonic()))
                except subprocess.TimeoutExpired:
                    pass
            cleanup["root_exit_observed"] = process.poll() is not None
            cleanup["process_group_empty_observed"] = not process_group_exists(process.pid)
            for stream in (process.stdout, process.stderr):
                if stream is not None:
                    try:
                        stream.close()
                    except OSError:
                        pass
        if package_cache is not None:
            cache_path = Path(package_cache.name)
            try:
                package_cache.cleanup()
            except OSError:
                package_cache_cleanup = "failed"
                package_cache_cleanup_error = "os-error"
            else:
                try:
                    cache_remains = cache_path.exists()
                except OSError:
                    cache_remains = True
                    package_cache_cleanup_error = "os-error"
                if cache_remains:
                    package_cache_cleanup = "failed"
                    if package_cache_cleanup_error == "none":
                        package_cache_cleanup_error = "cache-remains"
                else:
                    package_cache_cleanup = "removed"
            if package_cache_cleanup == "failed" and failure == "none":
                failure = "package-cache-cleanup-failed"
                failure_stage = "cleanup"

    sentinel_unchanged = original_directory_unchanged(action)
    action_directory_unchanged = sentinel_unchanged
    if not sentinel_unchanged and failure == "none":
        failure = "sentinel-changed-after-prompt"
        failure_stage = "cleanup"
    passed = bool(
        failure == "none"
        and initialize_observed
        and session_observed
        and version_observation == "matched-1.0.93"
        and prompt_send_attempted
        and prompt_send_completed
        and prompt_slot_bytes is not None
        and exchange["permission_request_count"] == 1
        and exchange["permission_reply"] == "cancelled"
        and exchange["permission_action_matches_sentinel"]
        and exchange["correlation_valid"]
        and exchange["session_cancel_sent"]
        and exchange["prompt_result"] == "cancelled"
        and not exchange["actual_effect_observed"]
        and sentinel_unchanged
        and action_directory_unchanged
        and cleanup["root_exit_observed"]
        and cleanup["process_group_empty_observed"]
        and cleanup["streams_joined"]
        and process is not None
        and process.returncode == 0
    )
    if not passed and failure == "none":
        failure = "permission-proof-incomplete"
        failure_stage = "cleanup"
    record = {
        "schema": "copilot-cli-acp-host-permission-original-execution.v4",
        "execution_kind": "original",
        "original_execution": True,
        "plan_sha256": sha256_file(plan_path_for(plan)),
        "preparation_runner_sha256": PREPARATION_RUNNER_SHA256,
        "runner_sha256": expected_runner_sha256(plan, "original"),
        "authority_sha256": authority_sha256,
        "attempt_record_sha256": sha256_bytes(attempt_bytes),
        "prompt_slot_record_sha256": sha256_bytes(prompt_slot_bytes) if prompt_slot_bytes is not None else None,
        "target_version": "1.0.93",
        "argv_sha256": sha256_bytes(json.dumps(plan["invocation"]["argv"], separators=(",", ":")).encode()),
        "status": "passed" if passed else "failed",
        "failure_class": "none" if passed else failure,
        "launch_error": launch_error,
        "failure_stage": "none" if passed else failure_stage,
        "agent_version_observation": version_observation,
        "agent_version_reported": version_reported,
        "model_observation": model_observation,
        "initialize_observed": initialize_observed,
        "session_observed": session_observed,
        "prompt_send_attempted": prompt_send_attempted,
        "prompt_send_completed": prompt_send_completed,
        "elapsed_milliseconds": int((time.monotonic() - started) * 1000),
        "prompt_slot_fsynced_before_send": prompt_slot_bytes is not None,
        **{key: exchange[key] for key in (
            "permission_request_count", "permission_reply", "permission_outcome",
            "permission_action_matches_sentinel", "permission_action_attributed",
            "correlation_valid", "tool_announced", "tool_announcement_count",
            "tool_announcement_status", "tool_status", "tool_kind", "tool_call_updates", "execution_reported",
            "actual_effect_observed", "prompt_result", "session_cancel_sent",
        )},
        "sentinel_unchanged": sentinel_unchanged,
        "action_directory_unchanged": action_directory_unchanged,
        "exit_code": process.returncode if process is not None else None,
        "cleanup": cleanup,
        "descendant_cleanup": "unknown-for-arbitrary-vendor-descendants",
        "stderr": stderr.summary(),
        "raw_protocol_persisted": False,
        "raw_stderr_persisted": False,
        "completed_at": utc_now(),
    }
    if plan.get("plan_id") == FINAL_CORRECTION_PLAN_ID:
        record.update({
            "task_number": plan["task_number"],
            "task_id": plan["task_id"],
            "run_id": plan["run_id"],
            "operator_decision": plan["operator_decision"],
            "package_cache_cleanup": package_cache_cleanup,
            "package_cache_cleanup_error": package_cache_cleanup_error,
        })
    validate_original_execution_record(
        record, plan, authority_sha256, allow_fake_authority=fake_test
    )
    validate_original_ledger_bindings(
        record, attempt_bytes, prompt_slot_bytes, plan, authority_sha256
    )
    durable_create_once(record_path, record)
    return record


def execute_original(authority_path: Path = AUTHORITY_PATH) -> dict[str, Any]:
    plan = validate_plan()
    authority, authority_sha256 = validate_execution_authority(authority_path, plan=plan)
    if authority["final_runner_sha256"] != sha256_file(SCRIPT_PATH):
        raise RuntimeError("the consumed authority is bound to an earlier runner; refusing relaunch")
    if platform.system() != "Darwin" or platform.machine().lower() not in {"arm64", "aarch64"}:
        raise RuntimeError("normal-host execution requires the reviewed Darwin arm64 platform")
    paths = original_ledger_paths(plan)
    if authority.get("execution_record_path") != (
        "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 168/1.0.93-execution.json"
    ):
        raise ValueError("execution result path differs from the authority")
    if any(path.exists() or path.is_symlink() for path in (paths["attempt"], paths["prompt"], paths["execution"])):
        raise RuntimeError("an original attempt, prompt slot or execution record already exists")
    with tempfile.TemporaryDirectory(prefix="copilot-acp-normal-host-permission.") as temp_root:
        root = Path(temp_root).resolve()
        if root == ROOT or ROOT in root.parents:
            raise RuntimeError("artifact staging root must be a fresh directory outside the checkout")
        executable = stage_reviewed_artifacts(root, plan)
        if sha256_file(executable) != authority["native_executable_sha256"]:
            raise ValueError("staged executable does not match the final execution authority")
        action = prepare_action_directory(root)
        return run_original_session(
            executable, action, paths["attempt"], paths["prompt"], paths["execution"],
            plan, authority, authority_sha256,
        )


def execute_corrected_original() -> dict[str, Any]:
    plan = validate_final_correction_plan()
    authority, authority_sha256 = validate_corrected_authority(plan=plan)
    if authority["final_runner_sha256"] != sha256_file(SCRIPT_PATH):
        raise RuntimeError("corrected authority is bound to an earlier runner; refusing launch")
    if platform.system() != "Darwin" or platform.machine().lower() not in {"arm64", "aarch64"}:
        raise RuntimeError("normal-host execution requires the reviewed Darwin arm64 platform")
    paths = original_ledger_paths(plan)
    if any(path.exists() or path.is_symlink() for path in (paths["attempt"], paths["prompt"], paths["execution"])):
        raise RuntimeError("a corrected attempt, prompt slot or execution record already exists")
    with tempfile.TemporaryDirectory(prefix="copilot-acp-normal-host-corrected-permission.") as temp_root:
        root = Path(temp_root).resolve()
        if root == ROOT or ROOT in root.parents:
            raise RuntimeError("artifact staging root must be a fresh directory outside the checkout")
        executable = stage_reviewed_artifacts(root, plan)
        if sha256_file(executable) != authority["native_executable_sha256"]:
            raise ValueError("staged executable does not match the corrected execution authority")
        action = prepare_action_directory(root)
        return run_original_session(
            executable, action, paths["attempt"], paths["prompt"], paths["execution"],
            plan, authority, authority_sha256,
        )


def response_for(client: StdioSession, request_id: int, deadline: float) -> dict[str, Any]:
    while True:
        message = client.receive(deadline)
        if message.get("id") == request_id and ("result" in message or "error" in message):
            return message
        if message.get("method") == "session/update":
            params = message.get("params")
            update = params.get("update", {}) if isinstance(params, dict) else {}
            if isinstance(update, dict) and update.get("sessionUpdate") == "tool_call":
                raise ProtocolFailure("unrequested-tool-call-update")
            continue
        if message.get("method") is not None:
            raise ProtocolFailure("unexpected-callback-before-response")
        raise ProtocolFailure("unexpected-response-id")


TOOL_STATUSES = frozenset({"pending", "in_progress", "completed", "failed"})
TOOL_KINDS = frozenset({
    "read", "edit", "delete", "move", "search", "execute", "think", "fetch", "switch_mode",
})
NON_TOOL_SESSION_UPDATES = frozenset({
    "user_message_chunk", "agent_message_chunk", "agent_thought_chunk", "plan",
    "available_commands_update", "current_mode_update", "config_option_update",
    "session_info_update", "usage_update",
})
MAX_TOOL_CALL_ID_LENGTH = 256
MAX_SESSION_ID_LENGTH = 256
MAX_PROMPT_MESSAGES = 64


def new_permission_exchange() -> dict[str, Any]:
    return {
        "failure": "none",
        "permission_request_count": 0,
        "permission_reply": "none",
        "permission_outcome": "none",
        "permission_action_matches_sentinel": False,
        "permission_action_attributed": False,
        "correlation_valid": False,
        "tool_announced": False,
        "tool_announcement_count": 0,
        "tool_announcement_status": "unknown",
        "tool_status": "unknown",
        "tool_kind": "unknown",
        "tool_call_updates": 0,
        "execution_reported": False,
        "actual_effect_observed": False,
        "prompt_result": "none",
        "session_cancel_sent": False,
    }


def permission_exchange(
    client: StdioSession,
    session_id: str,
    action: Path,
    deadline: float,
    directory_unchanged: Any,
) -> dict[str, Any]:
    """Correlate the announced pending call, permission request and cancellation."""
    evidence = new_permission_exchange()
    call: dict[str, Any] | None = None
    prompt_received = False
    messages_seen = 0
    correlation_failed = False
    cancel_attempted = False

    def invalidate_correlation() -> None:
        nonlocal correlation_failed
        correlation_failed = True
        evidence["correlation_valid"] = False

    def mark_correlated() -> None:
        if not correlation_failed:
            evidence["correlation_valid"] = True

    def fail(category: str) -> None:
        if evidence["failure"] == "none":
            evidence["failure"] = category

    def send_session_cancel() -> None:
        nonlocal cancel_attempted
        if cancel_attempted:
            return
        cancel_attempted = True
        try:
            client.send({
                "jsonrpc": "2.0",
                "method": "session/cancel",
                "params": {"sessionId": session_id},
            })
        except ProtocolFailure as error:
            fail(error.category)
            return
        evidence["session_cancel_sent"] = True

    def observe_effect() -> None:
        if not directory_unchanged():
            evidence["actual_effect_observed"] = True
            fail("unapproved-effect-observed")
            if not prompt_received:
                send_session_cancel()

    while not prompt_received:
        messages_seen += 1
        if messages_seen > MAX_PROMPT_MESSAGES:
            fail("permission-proof-incomplete")
            if not evidence["session_cancel_sent"]:
                send_session_cancel()
            break
        try:
            message = client.receive(deadline)
        except ProtocolFailure as error:
            fail(error.category)
            break

        if message.get("jsonrpc") != "2.0":
            fail("malformed-frame")
            send_session_cancel()
            continue
        observe_effect()
        method = message.get("method")
        if method == "session/update":
            if "id" in message:
                fail("malformed-session-update")
                send_session_cancel()
                continue
            params = message.get("params")
            update = params.get("update") if isinstance(params, dict) else None
            if not isinstance(params, dict) or params.get("sessionId") != session_id:
                invalidate_correlation()
                fail("session-id-mismatch")
                send_session_cancel()
                continue
            if not isinstance(update, dict):
                fail("malformed-session-update")
                send_session_cancel()
                continue
            update_kind = update.get("sessionUpdate")
            if update_kind in NON_TOOL_SESSION_UPDATES:
                continue
            if update_kind == "tool_call":
                evidence["tool_announcement_count"] += 1
                evidence["tool_announced"] = True
                if evidence["tool_announcement_count"] != 1 or call is not None:
                    invalidate_correlation()
                    fail("duplicate-tool-call-announcement")
                    send_session_cancel()
                    continue
                tool_id = update.get("toolCallId")
                if not isinstance(tool_id, str) or not tool_id or len(tool_id) > MAX_TOOL_CALL_ID_LENGTH:
                    invalidate_correlation()
                    fail("tool-call-id-invalid")
                    send_session_cancel()
                    continue
                try:
                    call = merge_tool_call(None, update, initial=True)
                except ValueError as error:
                    invalidate_correlation()
                    fail(str(error))
                    send_session_cancel()
                    continue
                evidence["tool_status"] = call["status"]
                evidence["tool_announcement_status"] = call["status"]
                evidence["tool_kind"] = call.get("kind") or "unknown"
                mark_correlated()
                if call["status"] != "pending":
                    evidence["execution_reported"] = True
                    fail("tool-execution-before-permission")
                    send_session_cancel()
                continue
            if update_kind == "tool_call_update":
                evidence["tool_call_updates"] += 1
                tool_id = update.get("toolCallId")
                if call is None or not isinstance(tool_id, str) or tool_id != call["tool_call_id"]:
                    invalidate_correlation()
                    fail("tool-call-update-uncorrelated")
                    send_session_cancel()
                    continue
                try:
                    call = merge_tool_call(call, update, initial=False)
                except ValueError as error:
                    invalidate_correlation()
                    fail(str(error))
                    send_session_cancel()
                    continue
                mark_correlated()
                evidence["tool_status"] = call["status"]
                evidence["tool_kind"] = call.get("kind") or "unknown"
                if call["status"] != "pending":
                    evidence["execution_reported"] = True
                    if evidence["permission_reply"] == "none":
                        fail("tool-execution-before-permission")
                        send_session_cancel()
                    elif call["status"] in {"in_progress", "completed"}:
                        fail("tool-execution-after-cancel")
                continue
            fail("unexpected-session-update")
            send_session_cancel()
            continue

        if method == "session/request_permission":
            evidence["permission_request_count"] += 1
            params = message.get("params")
            request_id = message.get("id")
            valid_request_id = (
                not isinstance(request_id, bool)
                and isinstance(request_id, (int, str))
            )
            if valid_request_id:
                send_session_cancel()
                try:
                    client.send({
                        "jsonrpc": "2.0",
                        "id": request_id,
                        "result": {"outcome": {"outcome": "cancelled"}},
                    })
                except ProtocolFailure as error:
                    fail(error.category)
                    send_session_cancel()
                    continue
                evidence["permission_reply"] = "cancelled"
                evidence["permission_outcome"] = "cancelled"
            else:
                fail("permission-request-missing-id")

            if evidence["permission_request_count"] > 1:
                fail("duplicate-permission-request")
            session_matches = isinstance(params, dict) and params.get("sessionId") == session_id
            if not session_matches:
                invalidate_correlation()
                fail("session-id-mismatch")
            if not valid_permission_options(params):
                fail("permission-request-malformed")
            tool_call = params.get("toolCall") if isinstance(params, dict) else None
            permission_tool_id = tool_call.get("toolCallId") if isinstance(tool_call, dict) else None
            if (
                call is None
                or not isinstance(permission_tool_id, str)
                or permission_tool_id != call["tool_call_id"]
            ):
                invalidate_correlation()
                fail("permission-tool-call-uncorrelated")
            else:
                if session_matches:
                    mark_correlated()
                try:
                    call = merge_tool_call(call, tool_call, initial=False)
                except ValueError as error:
                    invalidate_correlation()
                    fail(str(error))
                    send_session_cancel()
                    continue
                evidence["tool_status"] = call["status"]
                evidence["tool_kind"] = call.get("kind") or "unknown"
                if call["status"] != "pending":
                    evidence["execution_reported"] = True
                    fail("tool-execution-before-permission")
                matched, attributed = expected_permission_action(call, action)
                evidence["permission_action_matches_sentinel"] = matched
                evidence["permission_action_attributed"] = attributed
                if not matched:
                    fail("permission-action-mismatch" if attributed else "permission-action-unattributable")
            if not valid_request_id and evidence["failure"] == "none":
                fail("permission-request-missing-id")
            send_session_cancel()
            continue

        if method is not None:
            fail("unexpected-callback")
            send_session_cancel()
            continue

        response_id = message.get("id")
        if response_id == 3:
            prompt_received = True
            if "error" in message:
                evidence["prompt_result"] = "other"
                fail("prompt-rpc-error")
            else:
                if "result" not in message:
                    evidence["prompt_result"] = "unknown"
                    fail("prompt-rpc-error")
                    continue
                response = message.get("result")
                stop_reason = response.get("stopReason") if isinstance(response, dict) else None
                evidence["prompt_result"] = (
                    stop_reason
                    if isinstance(stop_reason, str) and stop_reason in {"cancelled", "end_turn"}
                    else "unknown" if stop_reason is None else "other"
                )
                if evidence["permission_request_count"] == 0:
                    fail("permission-request-missing")
                if evidence["session_cancel_sent"] and evidence["prompt_result"] != "cancelled":
                    fail("prompt-not-cancelled")
            continue
        fail("unexpected-response-id")
        send_session_cancel()

    if not prompt_received and not evidence["session_cancel_sent"]:
        send_session_cancel()
    observe_effect()
    return evidence


def valid_permission_options(params: Any) -> bool:
    if not isinstance(params, dict) or not isinstance(params.get("options"), list):
        return False
    option_ids: set[str] = set()
    allowed_kinds = {"allow_once", "allow_always", "reject_once", "reject_always"}
    for option in params["options"]:
        if not isinstance(option, dict):
            return False
        option_id = option.get("optionId")
        if (
            not isinstance(option_id, str)
            or not option_id
            or len(option_id) > MAX_TOOL_CALL_ID_LENGTH
            or option_id in option_ids
            or not isinstance(option.get("name"), str)
            or option.get("kind") not in allowed_kinds
        ):
            return False
        option_ids.add(option_id)
    return True


def merge_tool_call(
    current: dict[str, Any] | None,
    snapshot: dict[str, Any],
    *,
    initial: bool,
) -> dict[str, Any]:
    tool_id = snapshot.get("toolCallId")
    if not isinstance(tool_id, str) or not tool_id or len(tool_id) > MAX_TOOL_CALL_ID_LENGTH:
        raise ValueError("tool-call-id-invalid")
    if current is not None and tool_id != current["tool_call_id"]:
        raise ValueError("tool-call-update-uncorrelated")
    result = dict(current or {})
    result["tool_call_id"] = tool_id

    title = snapshot.get("title")
    if title is not None:
        if not isinstance(title, str):
            raise ValueError("tool-call-snapshot-invalid")
        result["title"] = title
    elif initial:
        raise ValueError("tool-call-snapshot-invalid")

    status = snapshot.get("status")
    if status is not None:
        if not isinstance(status, str) or status not in TOOL_STATUSES:
            raise ValueError("tool-call-status-invalid")
        result["status"] = status
    elif initial:
        result["status"] = "pending"

    kind = snapshot.get("kind")
    if kind is not None:
        if not isinstance(kind, str):
            raise ValueError("tool-call-kind-invalid")
        result["kind"] = kind if kind in TOOL_KINDS else "other"
    elif initial:
        result["kind"] = "other"

    raw_input = snapshot.get("rawInput")
    if raw_input is not None:
        result["raw_input"] = raw_input

    for field in ("content", "locations"):
        if field in snapshot and snapshot[field] is not None:
            if not isinstance(snapshot[field], list):
                raise ValueError("tool-call-snapshot-invalid")
            if field == "content" and any(not valid_tool_content(item) for item in snapshot[field]):
                raise ValueError("tool-call-snapshot-invalid")
            if field == "locations" and any(
                not isinstance(item, dict)
                or not isinstance(item.get("path"), str)
                or "line" in item
                and item["line"] is not None
                and (
                    type(item["line"]) is not int
                    or not 0 <= item["line"] <= 0xFFFFFFFF
                )
                for item in snapshot[field]
            ):
                raise ValueError("tool-call-snapshot-invalid")
            result[field] = snapshot[field]
    return result


def valid_tool_content(item: Any) -> bool:
    if not isinstance(item, dict):
        return False
    item_type = item.get("type")
    if item_type == "content":
        content = item.get("content")
        return isinstance(content, dict) and isinstance(content.get("type"), str)
    if item_type == "diff":
        return (
            isinstance(item.get("path"), str)
            and isinstance(item.get("newText"), str)
            and (item.get("oldText") is None or isinstance(item.get("oldText"), str))
        )
    if item_type == "terminal":
        return isinstance(item.get("terminalId"), str) and bool(item["terminalId"])
    return False


def _matches_sentinel_path(raw_path: Any, action: Path) -> bool:
    if not isinstance(raw_path, str) or not raw_path:
        return False
    sentinel = action.resolve() / "permission-sentinel.txt"
    candidate = Path(raw_path)
    return candidate.is_absolute() and candidate.resolve(strict=False) == sentinel


def expected_permission_action(call: dict[str, Any], action: Path) -> tuple[bool, bool]:
    """Attribute the exact edit from ACP fields; titles and missing fields never count."""
    kind = call.get("kind")
    if kind not in TOOL_KINDS:
        return False, False
    if kind != "edit":
        return False, True

    if call.get("raw_input") is not None:
        return False, False

    content = call.get("content")
    if not isinstance(content, list) or len(content) != 1 or content[0].get("type") != "diff":
        return False, False
    diff = content[0]
    attributed = True
    matched = (
        _matches_sentinel_path(diff.get("path"), action)
        and diff.get("newText") == SENTINEL_AFTER.decode()
        and diff.get("oldText") in (None, SENTINEL_BEFORE.decode())
    )

    locations = call.get("locations")
    if locations:
        if any(not _matches_sentinel_path(item.get("path"), action) for item in locations):
            return False, True

    return matched, attributed


def process_group_exists(pgid: int) -> bool:
    try:
        os.killpg(pgid, 0)
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True


def signal_owned_group(process: subprocess.Popen[bytes], sig: int) -> None:
    try:
        os.killpg(process.pid, sig)
    except ProcessLookupError:
        pass
    except OSError:
        pass


def cleanup_process(
    process: subprocess.Popen[bytes], client: StdioSession, cleanup_deadline: float
) -> dict[str, Any]:
    client.close_input()
    if process.poll() is None:
        first_wait = min(cleanup_deadline, time.monotonic() + 0.15)
        try:
            process.wait(timeout=max(0.0, first_wait - time.monotonic()))
        except subprocess.TimeoutExpired:
            pass
    if process.poll() is None or process_group_exists(process.pid):
        signal_owned_group(process, signal.SIGTERM)
    term_deadline = min(cleanup_deadline, time.monotonic() + 0.15)
    while process.poll() is None and time.monotonic() < term_deadline:
        try:
            process.wait(timeout=min(0.025, max(0.001, term_deadline - time.monotonic())))
        except subprocess.TimeoutExpired:
            pass
    if process.poll() is None or process_group_exists(process.pid):
        signal_owned_group(process, signal.SIGKILL)
    if process.poll() is None:
        try:
            process.wait(timeout=max(0.001, cleanup_deadline - time.monotonic()))
        except subprocess.TimeoutExpired:
            pass
    try:
        client.drain_until(cleanup_deadline)
    except ProtocolFailure:
        pass
    root_joined = process.poll() is not None
    group_empty = not process_group_exists(process.pid)
    readers_joined = client.stdout_eof and client.stderr_eof
    client.close()
    return {
        "root_exit_observed": root_joined,
        "process_group_empty_observed": group_empty,
        "stdout_reader_joined": client.stdout_eof,
        "stderr_reader_joined": client.stderr_eof,
        "streams_joined": readers_joined,
        "descendant_cleanup": "unknown-for-arbitrary-vendor-descendants",
    }


def fresh_fake_scratch(root: Path) -> dict[str, Path]:
    root.mkdir(mode=0o700)
    paths = {
        "root": root,
        "action": root / "action",
        "home": root / "home",
        "copilot_home": root / "copilot-home",
        "xdg_config": root / "xdg-config",
        "xdg_cache": root / "xdg-cache",
        "tmp": root / "tmp",
        "records": root / "records",
        "state": root / "copilot-home" / "state",
        "logs": root / "copilot-home" / "logs",
        "children": root / "children",
    }
    for key in ("action", "copilot_home", "records"):
        paths[key].mkdir(mode=0o700)
    paths["action"].joinpath("permission-sentinel.txt").write_bytes(SENTINEL_BEFORE)
    paths["copilot_home"].joinpath("config.json").write_text(
        json.dumps({"fixture": FAKE_CONFIG}), encoding="utf-8"
    )
    return paths


def fake_environment(paths: dict[str, Path], scenario: str, attempt: Path, prompt: Path) -> dict[str, str]:
    return {
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "HOME": str(paths["home"]),
        "COPILOT_HOME": str(paths["copilot_home"]),
        "XDG_CONFIG_HOME": str(paths["xdg_config"]),
        "XDG_CACHE_HOME": str(paths["xdg_cache"]),
        "TMPDIR": str(paths["tmp"]),
        "TERM": "dumb",
        "SWALLOWTAIL_FAKE_SCENARIO": scenario,
        "SWALLOWTAIL_FAKE_ACTION": str(paths["action"]),
        "SWALLOWTAIL_FAKE_STATE": str(paths["state"]),
        "SWALLOWTAIL_FAKE_LOGS": str(paths["logs"]),
        "SWALLOWTAIL_FAKE_CHILDREN": str(paths["children"]),
        "SWALLOWTAIL_FAKE_ATTEMPT": str(attempt),
        "SWALLOWTAIL_FAKE_PROMPT_SLOT": str(prompt),
    }


def run_fake_scenario(
    root: Path, scenario: str, *, outer_seconds: float = 3.0, cleanup_seconds: float = 0.4
) -> tuple[dict[str, Any], dict[str, Path]]:
    if scenario not in FAKE_SCENARIOS:
        raise ValueError("fake scenario is outside the closed test vocabulary")
    plan = validate_plan()
    paths = fresh_fake_scratch(root)
    attempt_path = paths["records"] / "1.0.93-attempt.json"
    prompt_path = paths["records"] / "1.0.93-prompt-1.json"
    attempt_bytes = write_attempt_ledger(attempt_path, plan, "fake")
    started = time.monotonic()
    outer_budget = min(MAX_OUTER_SECONDS, max(0.05, outer_seconds))
    cleanup_budget = min(MAX_CLEANUP_SECONDS, max(0.02, cleanup_seconds), outer_budget)
    operation_deadline = started + outer_budget - cleanup_budget
    total_deadline = started + outer_budget
    process: subprocess.Popen[bytes] | None = None
    client: StdioSession | None = None
    failure = "none"
    stage = "launch"
    failure_stage = "none"
    initialize_observed = False
    session_observed = False
    exchange = new_permission_exchange()
    prompt_send_attempted = False
    prompt_send_completed = False
    prompt_slot_bytes: bytes | None = None
    cleanup = {
        "root_exit_observed": False,
        "process_group_empty_observed": False,
        "stdout_reader_joined": False,
        "stderr_reader_joined": False,
        "streams_joined": False,
        "descendant_cleanup": "unknown-for-arbitrary-vendor-descendants",
    }
    agent_version_observation = "unknown"
    agent_version_reported: str | None = None
    stderr = BoundedDiagnostics()
    launch_error = "none"
    try:
        process = subprocess.Popen(
            [sys.executable, str(SCRIPT_PATH), "--fake-agent", scenario],
            cwd=paths["action"],
            env=fake_environment(paths, scenario, attempt_path, prompt_path),
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            bufsize=0,
            start_new_session=True,
        )
        client = StdioSession(process)
        stage = "initialize"
        client.send(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {"protocolVersion": 1, "clientInfo": {"name": "swallowtail-fake", "version": "1"}},
            }
        )
        initialize = response_for(client, 1, operation_deadline)
        if "error" in initialize:
            failure = "initialize-rpc-error"
            raise ProtocolFailure(failure)
        result = initialize.get("result")
        if not isinstance(result, dict) or result.get("protocolVersion") != 1:
            failure = "initialize-protocol-mismatch"
            raise ProtocolFailure(failure)
        info = result.get("agentInfo")
        agent_version = info.get("version") if isinstance(info, dict) else None
        agent_version_observation, agent_version_reported = classify_agent_version(agent_version)
        if agent_version_observation != "matched-1.0.93":
            failure = {
                "mismatch": "agent-version-mismatch",
                "invalid": "agent-version-invalid",
                "unknown": "agent-version-unobserved",
            }[agent_version_observation]
            raise ProtocolFailure(failure)
        initialize_observed = True
        stage = "session-new"
        client.send({"jsonrpc": "2.0", "method": "initialized", "params": {}})
        client.send(
            {
                "jsonrpc": "2.0",
                "id": 2,
                "method": "session/new",
                "params": {"cwd": str(paths["action"]), "mcpServers": []},
            }
        )
        session = response_for(client, 2, operation_deadline)
        if "error" in session:
            failure = "session-new-rpc-error"
            raise ProtocolFailure(failure)
        session_result = session.get("result")
        session_id = session_result.get("sessionId") if isinstance(session_result, dict) else None
        if not isinstance(session_id, str) or not session_id or len(session_id) > MAX_SESSION_ID_LENGTH:
            failure = "session-new-protocol-mismatch"
            raise ProtocolFailure(failure)
        session_observed = True
        if paths["action"].joinpath("permission-sentinel.txt").read_bytes() != SENTINEL_BEFORE:
            failure = "sentinel-changed-before-prompt"
            raise ProtocolFailure(failure)
        if paths["action"].joinpath("unapproved-effect.json").exists():
            failure = "unapproved-effect-before-prompt"
            raise ProtocolFailure(failure)
        prompt_slot_bytes = write_prompt_slot(prompt_path, attempt_bytes, plan)
        stage = "session-prompt"
        prompt_send_attempted = True
        client.send(
            {
                "jsonrpc": "2.0",
                "id": 3,
                "method": "session/prompt",
                "params": {
                    "sessionId": session_id,
                    "prompt": [{"type": "text", "text": PROMPT_TEXT}],
                },
            }
        )
        prompt_send_completed = True
        def fake_directory_unchanged() -> bool:
            return (
                sorted(item.name for item in paths["action"].iterdir()) == ["permission-sentinel.txt"]
                and paths["action"].joinpath("permission-sentinel.txt").read_bytes() == SENTINEL_BEFORE
                and not paths["action"].joinpath("unapproved-effect.json").exists()
            )

        exchange = permission_exchange(
            client, session_id, paths["action"], operation_deadline,
            fake_directory_unchanged,
        )
        if exchange["failure"] != "none":
            failure = exchange["failure"]
    except ProtocolFailure as error:
        if failure == "none":
            failure = error.category
        failure_stage = stage
    except subprocess.SubprocessError:
        launch_error = "subprocess-launch-error"
        failure = "launch-failure"
        failure_stage = stage
    except OSError as error:
        launch_error = "pipe-or-launch-error" if process is None else "pipe-error"
        failure = "launch-failure" if process is None else "protocol-write-failure"
        failure_stage = stage
        del error
    finally:
        if failure != "none" and failure_stage == "none":
            failure_stage = stage
        stage = "cleanup"
        if process is not None and client is not None:
            cleanup_deadline = min(total_deadline, time.monotonic() + cleanup_budget)
            cleanup = cleanup_process(process, client, cleanup_deadline)
            stderr = client.stderr_diagnostics
        elif process is not None:
            try:
                process.wait(timeout=max(0.001, min(cleanup_budget, total_deadline - time.monotonic())))
            except subprocess.TimeoutExpired:
                signal_owned_group(process, signal.SIGKILL)
                try:
                    process.wait(timeout=0.1)
                except subprocess.TimeoutExpired:
                    pass

    sentinel_after = paths["action"].joinpath("permission-sentinel.txt").read_bytes()
    effect_absent = not paths["action"].joinpath("unapproved-effect.json").exists()
    passed = bool(
        failure == "none"
        and initialize_observed
        and session_observed
        and exchange["permission_request_count"] == 1
        and exchange["permission_reply"] == "cancelled"
        and exchange["permission_action_matches_sentinel"]
        and exchange["correlation_valid"]
        and exchange["session_cancel_sent"]
        and exchange["prompt_result"] == "cancelled"
        and not exchange["actual_effect_observed"]
        and sentinel_after == SENTINEL_BEFORE
        and effect_absent
        and cleanup["root_exit_observed"]
        and cleanup["streams_joined"]
    )
    if not passed and failure == "none":
        failure = "permission-proof-incomplete"
    if failure != "none" and failure_stage == "none":
        failure_stage = "cleanup"
    record = {
        "schema": "copilot-cli-acp-host-permission-execution.v3",
        "plan_sha256": sha256_file(PLAN_PATH),
        "runner_sha256": sha256_file(SCRIPT_PATH),
        "execution_kind": "fake",
        "original_execution": False,
        "scenario": scenario,
        "status": "passed" if passed else "failed",
        "failure_class": failure,
        "launch_error": launch_error,
        "agent_version_observation": agent_version_observation,
        "agent_version_reported": agent_version_reported,
        "failure_stage": failure_stage,
        "initialize_observed": initialize_observed,
        "session_observed": session_observed,
        "prompt_send_attempted": prompt_send_attempted,
        "prompt_send_completed": prompt_send_completed,
        "elapsed_milliseconds": int((time.monotonic() - started) * 1000),
        "prompt_slot_fsynced_before_send": prompt_slot_bytes is not None,
        **{key: exchange[key] for key in (
            "permission_request_count", "permission_reply", "permission_outcome",
            "permission_action_matches_sentinel", "permission_action_attributed",
            "correlation_valid", "tool_announced", "tool_announcement_count",
            "tool_announcement_status", "tool_status", "tool_kind", "tool_call_updates", "execution_reported",
            "actual_effect_observed", "prompt_result", "session_cancel_sent",
        )},
        "sentinel_unchanged": sentinel_after == SENTINEL_BEFORE,
        "unapproved_effect_absent": effect_absent,
        "exit_code": process.returncode if process is not None else None,
        "cleanup": cleanup,
        "descendant_cleanup": "unknown-for-arbitrary-vendor-descendants",
        "stderr": stderr.summary(),
        "raw_protocol_persisted": False,
        "raw_stderr_persisted": False,
        "completed_at": utc_now(),
    }
    validate_execution_record(record, plan)
    execution_record_path = paths["records"] / "fake-execution.json"
    durable_create_once(execution_record_path, record)
    return record, paths


def query_host_identity(
    *, command: tuple[str, ...] = ("/usr/bin/sw_vers",), timeout: float = 2.0
) -> dict[str, str]:
    """Bounded identity query; never waits for host quiet or exposes command output."""
    if platform.system() != "Darwin":
        raise RuntimeError("host identity is not Darwin")
    values: dict[str, str] = {}
    deadline = time.monotonic() + max(0.0, timeout)
    for flag, name in (("-productVersion", "product_version"), ("-buildVersion", "build")):
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            raise RuntimeError("host identity query timed out")
        try:
            result = subprocess.run(
                [*command, flag],
                cwd="/",
                stdin=subprocess.DEVNULL,
                stdout=subprocess.PIPE,
                stderr=subprocess.DEVNULL,
                timeout=remaining,
                check=False,
                env={"PATH": "/usr/bin:/bin", "HOME": "/"},
            )
        except subprocess.TimeoutExpired:
            raise RuntimeError("host identity query timed out") from None
        text = result.stdout.decode("ascii", errors="ignore").strip()
        if result.returncode != 0 or not text or len(text) > 32:
            raise RuntimeError("host identity query failed")
        values[name] = text
    architecture = platform.machine().lower()
    if architecture not in {"arm64", "aarch64", "x86_64"}:
        raise RuntimeError("host architecture is outside the recorded vocabulary")
    values["architecture"] = "arm64" if architecture == "aarch64" else architecture
    return values


def run_identity_fixture(root: Path, mode: str) -> tuple[bool, str]:
    root.mkdir(mode=0o700, parents=True, exist_ok=True)
    program = root / f"identity-{mode}.py"
    if mode == "success":
        body = "import sys; print('26.6.2' if sys.argv[1] == '-productVersion' else '25G83')\n"
    elif mode == "slow":
        body = "import time; time.sleep(2)\n"
    else:
        body = "raise SystemExit(7)\n"
    program.write_text(body, encoding="utf-8")
    return True, str(program)


def self_test_identity_queries(scratch: Path) -> dict[str, Any]:
    # The system predicate and child command are patched only for deterministic fixtures.
    global platform
    original_system = platform.system
    original_machine = platform.machine
    outcomes: dict[str, str] = {}
    try:
        platform.system = lambda: "Darwin"  # type: ignore[method-assign]
        platform.machine = lambda: "arm64"  # type: ignore[method-assign]
        for mode, timeout in (("success", 1.0), ("slow", 0.05), ("failure", 1.0)):
            _, program = run_identity_fixture(scratch, mode)
            try:
                identity = query_host_identity(command=(sys.executable, program), timeout=timeout)
                outcomes[mode] = "passed" if identity == {
                    "product_version": "26.6.2", "build": "25G83", "architecture": "arm64"
                } else "mismatch"
            except RuntimeError as error:
                outcomes[mode] = "timeout" if "timed out" in str(error) else "failed"
    finally:
        platform.system = original_system  # type: ignore[method-assign]
        platform.machine = original_machine  # type: ignore[method-assign]
    if outcomes != {"success": "passed", "slow": "timeout", "failure": "failed"}:
        raise RuntimeError("bounded host identity fixture outcomes changed")
    return outcomes


def fake_child(name: str, state_root: Path, release_path: Path) -> int:
    state_root.mkdir(mode=0o700, parents=True, exist_ok=True)
    (state_root / f"{name}.started").write_text("started\n", encoding="utf-8")
    deadline = time.monotonic() + 2.0
    while not release_path.exists() and time.monotonic() < deadline:
        time.sleep(0.01)
    (state_root / f"{name}.finished").write_text("finished\n", encoding="utf-8")
    return 0 if release_path.exists() else 9


def fake_agent(scenario: str) -> int:
    if scenario not in FAKE_SCENARIOS:
        return 90
    env = os.environ
    action = Path(env["SWALLOWTAIL_FAKE_ACTION"])
    state = Path(env["SWALLOWTAIL_FAKE_STATE"])
    logs = Path(env["SWALLOWTAIL_FAKE_LOGS"])
    children_dir = Path(env["SWALLOWTAIL_FAKE_CHILDREN"])
    attempt_path = Path(env["SWALLOWTAIL_FAKE_ATTEMPT"])
    prompt_path = Path(env["SWALLOWTAIL_FAKE_PROMPT_SLOT"])
    plan = validate_plan()
    attempt_bytes = attempt_path.read_bytes()
    attempt = load_object(attempt_path)
    try:
        validate_attempt_record(attempt, plan, "fake")
    except ValueError:
        return 91
    for name in ("HOME", "COPILOT_HOME", "XDG_CONFIG_HOME", "XDG_CACHE_HOME", "TMPDIR"):
        Path(env[name]).mkdir(mode=0o700, parents=True, exist_ok=True)
    config = load_object(Path(env["COPILOT_HOME"]) / "config.json")
    if config.get("fixture") != FAKE_CONFIG:
        return 92
    state.mkdir(mode=0o700, parents=True, exist_ok=True)
    logs.mkdir(mode=0o700, parents=True, exist_ok=True)
    (state / "runtime-state.json").write_text('{"started":true}\n', encoding="utf-8")
    (logs / "runtime.log").write_text("fake runtime started\n", encoding="utf-8")
    os.write(2, (FAKE_SECRET + "\n").encode())

    if scenario == "malformed-frame":
        os.write(1, b"{malformed-json\n")
        return 0
    if scenario == "oversized-frame":
        sys.stdout.buffer.write(b"x" * (FRAME_LIMIT + 1) + b"\n")
        sys.stdout.buffer.flush()
        return 0

    fork_pid: int | None = None
    spawned: subprocess.Popen[bytes] | None = None
    escaped: subprocess.Popen[bytes] | None = None
    release = state / "release-children"
    if scenario == "success":
        children_dir.mkdir(mode=0o700, parents=True, exist_ok=True)
        fork_pid = os.fork()
        if fork_pid == 0:
            code = fake_child("fork", children_dir, release)
            os._exit(code)
        script = str(SCRIPT_PATH)
        child_env = dict(env)
        spawned = subprocess.Popen(
            [sys.executable, script, "--fake-child", "spawn"],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            env=child_env,
        )
        escaped = subprocess.Popen(
            [sys.executable, script, "--fake-child", "setsid"],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            env=child_env,
            start_new_session=True,
        )
    if scenario == "early-eof":
        return 93
    initialize = read_fake_request()
    if initialize.get("method") != "initialize":
        return 94
    if scenario == "hang-initialize":
        time.sleep(5)
        return 95
    if scenario == "initialize-error":
        send_fake({"jsonrpc": "2.0", "id": 1, "error": {"code": -32000, "message": "Authentication required"}})
        return 0
    version = {
        "wrong-version": "1.0.92",
        "invalid-version": "/Users/operator/copilot",
        "oversized-version": "1.0.93+" + ("a" * REPORTED_VERSION_MAX_LENGTH),
        "secret-version": "1.0.93+token-secret",
        "arbitrary-version": "Copilot 1.0.93 authentication failed",
        "nonstring-version": 93,
    }.get(scenario, "1.0.93")
    agent_info = {"name": "task-owned-fake"}
    if scenario != "missing-version":
        agent_info["version"] = version
    send_fake(
        {
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "protocolVersion": 1,
                "agentInfo": agent_info,
                "agentCapabilities": {},
            },
        }
    )
    initialized = read_fake_request()
    session_new = read_fake_request()
    if initialized.get("method") != "initialized" or session_new.get("method") != "session/new":
        return 96
    params = session_new.get("params", {})
    if params.get("cwd") != str(action) or params.get("mcpServers") != []:
        return 97
    if scenario == "session-new-error":
        send_fake({"jsonrpc": "2.0", "id": 2, "error": {"code": -32000, "message": "synthetic"}})
        return 0
    send_fake({"jsonrpc": "2.0", "id": 2, "result": {"sessionId": "synthetic-session"}})
    if scenario == "crash-before-prompt":
        return 17
    prompt = read_fake_request()
    if prompt.get("method") != "session/prompt":
        return 98
    if not prompt_path.exists():
        return 99
    prompt_record = load_object(prompt_path)
    try:
        validate_prompt_slot(prompt_record, attempt_bytes, plan)
    except ValueError:
        return 100
    if scenario == "crash-after-prompt":
        (state / "prompt-received").write_text("received\n", encoding="utf-8")
        return 18
    if scenario == "no-permission":
        send_fake({"jsonrpc": "2.0", "id": 3, "result": {"stopReason": "end_turn"}})
        return 0
    tool_id = "fake-tool"
    target = "other.txt" if scenario == "wrong-action" else "permission-sentinel.txt"
    if scenario in {
        "in-progress-before-permission", "completed-before-permission", "failed-before-permission"
    }:
        status = {
            "in-progress-before-permission": "in_progress",
            "completed-before-permission": "completed",
            "failed-before-permission": "failed",
        }[scenario]
        send_fake({
            "jsonrpc": "2.0", "method": "session/update",
            "params": {"sessionId": "synthetic-session", "update": {
                "sessionUpdate": "tool_call", "toolCallId": tool_id,
                "title": "Editing file", "kind": "edit",
            }},
        })
        send_fake({
            "jsonrpc": "2.0", "method": "session/update",
            "params": {"sessionId": "synthetic-session", "update": {
                "sessionUpdate": "tool_call_update", "toolCallId": tool_id,
                "status": status,
            }},
        })
        cancel = read_fake_request()
        if cancel.get("method") != "session/cancel" or "id" in cancel:
            return 101
        send_fake({"jsonrpc": "2.0", "id": 3, "result": {"stopReason": "cancelled"}})
        return 0
    if scenario == "spontaneous-effect":
        (action / "permission-sentinel.txt").write_bytes(SENTINEL_AFTER)
        send_fake({
            "jsonrpc": "2.0", "method": "session/update",
            "params": {"sessionId": "synthetic-session", "update": {
                "sessionUpdate": "tool_call", "toolCallId": tool_id,
                "title": "Editing file", "kind": "edit",
            }},
        })
        cancel = read_fake_request()
        if cancel.get("method") != "session/cancel" or "id" in cancel:
            return 101
        send_fake({"jsonrpc": "2.0", "id": 3, "result": {"stopReason": "cancelled"}})
        return 0
    if scenario == "missing-tool-id":
        send_fake({
            "jsonrpc": "2.0", "method": "session/update",
            "params": {"sessionId": "synthetic-session", "update": {
                "sessionUpdate": "tool_call", "title": "Editing file", "kind": "edit",
            }},
        })
        cancel = read_fake_request()
        if cancel.get("method") != "session/cancel" or "id" in cancel:
            return 101
        send_fake({"jsonrpc": "2.0", "id": 3, "result": {"stopReason": "cancelled"}})
        return 0
    if scenario == "unknown-tool-update":
        send_fake({
            "jsonrpc": "2.0", "method": "session/update",
            "params": {"sessionId": "synthetic-session", "update": {
                "sessionUpdate": "tool_call_update", "toolCallId": "unrelated-tool",
                "status": "pending",
            }},
        })
        cancel = read_fake_request()
        if cancel.get("method") != "session/cancel" or "id" in cancel:
            return 101
        send_fake({"jsonrpc": "2.0", "id": 3, "result": {"stopReason": "cancelled"}})
        return 0
    if scenario == "tool-call-without-permission":
        send_fake({
            "jsonrpc": "2.0", "method": "session/update",
            "params": {"sessionId": "synthetic-session", "update": {
                "sessionUpdate": "tool_call", "toolCallId": tool_id,
                "title": "Editing file", "kind": "edit",
            }},
        })
        send_fake({"jsonrpc": "2.0", "id": 3, "result": {"stopReason": "end_turn"}})
        return 0

    announcement: dict[str, Any] = {
        "sessionUpdate": "tool_call",
        "toolCallId": tool_id,
        "title": "Editing file",
    }
    if scenario != "missing-kind":
        announcement["kind"] = "write" if scenario == "wrong-kind" else "edit"
    if scenario == "diff-action":
        announcement["status"] = "pending"
    if scenario == "raw-input-only":
        announcement["rawInput"] = {
            "file_path": target,
            "new_string": SENTINEL_AFTER.decode(),
        }
    elif scenario != "missing-action":
        announcement["content"] = [{
            "type": "diff",
            "path": str((action / target).resolve()),
            "oldText": SENTINEL_BEFORE.decode(),
            "newText": SENTINEL_AFTER.decode(),
        }]
    send_fake({
        "jsonrpc": "2.0", "method": "session/update",
        "params": {"sessionId": "synthetic-session", "update": announcement},
    })
    sparse_update: dict[str, Any] = {
        "sessionUpdate": "tool_call_update", "toolCallId": tool_id,
        "locations": [{"path": str((action / target).resolve()), "line": 1}],
    }
    send_fake({
        "jsonrpc": "2.0", "method": "session/update",
        "params": {"sessionId": "synthetic-session", "update": sparse_update},
    })

    permission_tool_id = "unrelated-tool" if scenario == "wrong-id" else tool_id
    permission_session_id = "other-session" if scenario == "wrong-session" else "synthetic-session"
    permission_tool: dict[str, Any] = {"toolCallId": permission_tool_id}
    if scenario == "duplicate-permission":
        permission_tool["kind"] = "edit"
    options = [
        {"optionId": "allow_once", "name": "Allow once", "kind": "allow_once"},
        {"optionId": "reject_once", "name": "Reject once", "kind": "reject_once"},
    ]
    if scenario == "malformed-permission-options":
        options = [{"optionId": "reject_once", "name": "Reject once", "kind": "unknown"}]
    send_fake({
        "jsonrpc": "2.0", "id": 90, "method": "session/request_permission",
        "params": {
            "sessionId": permission_session_id,
            "toolCall": permission_tool,
            "options": options,
        },
    })
    cancel = read_fake_request()
    if cancel.get("method") != "session/cancel" or "id" in cancel:
        return 101
    reply = read_fake_request()
    reply_outcome = reply.get("result", {}).get("outcome", {})
    if reply.get("id") != 90 or reply_outcome.get("outcome") != "cancelled":
        return 101
    if scenario == "duplicate-permission":
        send_fake({
            "jsonrpc": "2.0", "id": 91, "method": "session/request_permission",
            "params": {
                "sessionId": "synthetic-session",
                "toolCall": {"toolCallId": tool_id},
                "options": [{"optionId": "reject_once", "name": "Reject", "kind": "reject_once"}],
            },
        })
        duplicate_reply = read_fake_request()
        duplicate_outcome = duplicate_reply.get("result", {}).get("outcome", {})
        if duplicate_reply.get("id") != 91 or duplicate_outcome.get("outcome") != "cancelled":
            return 101
    if scenario == "hang-after-permission":
        time.sleep(5)
        return 102
    if scenario == "success":
        release.write_text("release\n", encoding="utf-8")
        fork_status = os.waitpid(fork_pid, 0)[1] if fork_pid is not None else 255
        spawn_status = spawned.wait(timeout=2.0) if spawned is not None else 255
        setsid_status = escaped.wait(timeout=2.0) if escaped is not None else 255
        (state / "children-joined.json").write_text(
            json.dumps(
                {
                    "fork_reaped": os.WIFEXITED(fork_status) and os.WEXITSTATUS(fork_status) == 0,
                    "spawn_wait_exit": spawn_status,
                    "setsid_wait_exit": setsid_status,
                },
                sort_keys=True,
            ),
            encoding="utf-8",
        )
    if scenario in {"success", "permission-end-turn"}:
        send_fake({
            "jsonrpc": "2.0", "method": "session/update",
            "params": {"sessionId": "synthetic-session", "update": {
                "sessionUpdate": "tool_call_update", "toolCallId": tool_id,
                "status": "failed",
            }},
        })
    if scenario == "post-cancel-completed":
        send_fake({
            "jsonrpc": "2.0", "method": "session/update",
            "params": {"sessionId": "synthetic-session", "update": {
                "sessionUpdate": "tool_call_update", "toolCallId": tool_id,
                "status": "completed",
            }},
        })
    stop_reason = "end_turn" if scenario == "permission-end-turn" else "cancelled"
    send_fake({"jsonrpc": "2.0", "id": 3, "result": {"stopReason": stop_reason}})
    return 0


def read_fake_request() -> dict[str, Any]:
    line = sys.stdin.buffer.readline(FRAME_LIMIT + 2)
    if not line or len(line) > FRAME_LIMIT + 1 or not line.endswith(b"\n"):
        return {}
    try:
        value = json.loads(line)
    except json.JSONDecodeError:
        return {}
    return value if isinstance(value, dict) else {}


def send_fake(value: dict[str, Any]) -> None:
    sys.stdout.buffer.write(json.dumps(value, separators=(",", ":")).encode() + b"\n")
    sys.stdout.buffer.flush()


def fake_agent_main() -> int:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--fake-agent", choices=sorted(FAKE_SCENARIOS))
    parser.add_argument("--fake-child", choices=("spawn", "setsid"))
    args = parser.parse_args(sys.argv[1:])
    if args.fake_child:
        return fake_child(
            args.fake_child,
            Path(os.environ["SWALLOWTAIL_FAKE_CHILDREN"]),
            Path(os.environ["SWALLOWTAIL_FAKE_STATE"]) / "release-children",
        )
    return fake_agent(args.fake_agent or "success")


FAKE_NATIVE_SOURCE = r'''#!/usr/bin/env python3
import json
import os
import sys
import time
from pathlib import Path

if sys.argv[1:] != ["--model", "auto", "--acp", "--stdio"]:
    raise SystemExit(20)
if os.environ.get("SWALLOWTAIL_TEST_INHERITED_VALUE") != "synthetic-inherited-value":
    raise SystemExit(21)
scenario = os.environ.get("SWALLOWTAIL_TEST_ORIGINAL_SCENARIO")
receipt = Path(os.environ["SWALLOWTAIL_TEST_RECEIPT"])
launch_observation = Path(os.environ["SWALLOWTAIL_TEST_LAUNCH_OBSERVATION"])
package_cache = Path(os.environ["COPILOT_PKG_CACHE_HOME"])
launch_values = {
    "auto_update_disabled": os.environ.get("COPILOT_AUTO_UPDATE") == "false",
    "private_package_cache": package_cache.parent == Path(os.environ["SWALLOWTAIL_TEST_CACHE_PARENT"]),
    "package_cache_empty": not any(package_cache.iterdir()),
    "cli_version_override_removed": "COPILOT_CLI_VERSION" not in os.environ,
    "distribution_override_removed": "COPILOT_CLI_DIST_DIR" not in os.environ,
    "existing_home_preserved": os.environ.get("HOME") == os.environ.get("SWALLOWTAIL_TEST_HOST_HOME"),
    "existing_copilot_home_preserved": os.environ.get("COPILOT_HOME") == "/synthetic-existing-copilot-home",
    "existing_login_preserved": os.environ.get("COPILOT_GITHUB_TOKEN") == "synthetic-login-marker" and os.environ.get("GH_TOKEN") == "synthetic-gh-marker",
    "model_default_preserved": os.environ.get("COPILOT_MODEL") == "auto",
}
launch_observation.write_text(json.dumps(launch_values, sort_keys=True), encoding="utf-8")
if not all(launch_values.values()):
    raise SystemExit(30)

def read_message():
    line = sys.stdin.buffer.readline(262145)
    if not line or len(line) > 262144 or not line.endswith(b"\n"):
        raise SystemExit(22)
    return json.loads(line)

def send_message(value):
    sys.stdout.buffer.write(json.dumps(value, separators=(",", ":")).encode() + b"\n")
    sys.stdout.buffer.flush()

initialize = read_message()
if initialize.get("method") != "initialize" or initialize.get("id") != 1:
    raise SystemExit(23)
send_message({"jsonrpc":"2.0","id":1,"result":{"protocolVersion":1,"agentInfo":{"name":"task-owned-fake","version":"1.0.93","model":"fake-model"},"agentCapabilities":{}}})
initialized = read_message()
session_new = read_message()
if initialized.get("method") != "initialized" or session_new.get("method") != "session/new" or session_new.get("id") != 2:
    raise SystemExit(24)
if session_new.get("params", {}).get("mcpServers") != [] or session_new.get("params", {}).get("cwd") != os.getcwd():
    raise SystemExit(25)
send_message({"jsonrpc":"2.0","id":2,"result":{"sessionId":"synthetic-session"}})
if scenario == "crash-before-prompt":
    time.sleep(0.05)
    raise SystemExit(17)
prompt = read_message()
if prompt.get("method") != "session/prompt" or prompt.get("id") != 3:
    raise SystemExit(26)
if prompt.get("params", {}).get("prompt") != [{"type":"text","text":"Overwrite the existing file permission-sentinel.txt in this working directory with exactly: SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1 followed by a newline."}]:
    raise SystemExit(29)
if scenario == "crash-after-prompt":
    receipt.write_text("received\n", encoding="utf-8")
    raise SystemExit(18)
sentinel = Path(os.getcwd()) / "permission-sentinel.txt"
send_message({"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"synthetic-session","update":{"sessionUpdate":"tool_call","toolCallId":"fake-tool","title":"Editing task sentinel","kind":"edit","content":[{"type":"diff","path":str(sentinel),"oldText":"SWALLOWTAIL_PERMISSION_SENTINEL_BEFORE_V1\n","newText":"SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n"}]}}})
send_message({"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"synthetic-session","update":{"sessionUpdate":"tool_call_update","toolCallId":"fake-tool","locations":[{"path":str(sentinel),"line":1}]}}})
send_message({"jsonrpc":"2.0","id":90,"method":"session/request_permission","params":{"sessionId":"synthetic-session","toolCall":{"toolCallId":"fake-tool"},"options":[{"optionId":"reject_once","name":"Reject once","kind":"reject_once"}]}})
cancel = read_message()
if cancel.get("method") != "session/cancel" or "id" in cancel:
    raise SystemExit(31)
reply = read_message()
outcome = reply.get("result", {}).get("outcome", {})
if reply.get("id") != 90 or outcome.get("outcome") != "cancelled":
    raise SystemExit(27)
if scenario == "hang-after-permission":
    time.sleep(5)
    raise SystemExit(28)
send_message({"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"synthetic-session","update":{"sessionUpdate":"tool_call_update","toolCallId":"fake-tool","status":"failed"}}})
send_message({"jsonrpc":"2.0","id":3,"result":{"stopReason":"cancelled"}})
'''


def write_fake_native(path: Path) -> None:
    with path.open("xb") as stream:
        stream.write(FAKE_NATIVE_SOURCE.encode("utf-8"))
        stream.flush()
        os.fsync(stream.fileno())
    path.chmod(0o755)


def run_fake_original(
    root: Path,
    scenario: str,
    plan: dict[str, Any],
    authority: dict[str, Any],
    authority_sha256: str,
    *,
    outer_seconds: float = 2.0,
    cleanup_seconds: float = 0.4,
) -> tuple[dict[str, Any], dict[str, Path]]:
    root.mkdir(mode=0o700)
    action = prepare_action_directory(root)
    records = root / "records"
    records.mkdir(mode=0o700)
    synthetic_home = root / "synthetic-host-home"
    synthetic_home.mkdir(mode=0o700)
    executable = root / "fake-native"
    write_fake_native(executable)
    attempt = records / "attempt.json"
    prompt = records / "prompt.json"
    execution = records / "execution.json"
    receipt = root / "prompt-received"
    launch_observation = root / "launch-observation.json"
    base_environment = {
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "HOME": str(synthetic_home),
        "COPILOT_HOME": "/synthetic-existing-copilot-home",
        "COPILOT_GITHUB_TOKEN": "synthetic-login-marker",
        "GH_TOKEN": "synthetic-gh-marker",
        "COPILOT_AUTO_UPDATE": "true",
        "COPILOT_PKG_CACHE_HOME": "/synthetic-old-package-cache",
        "COPILOT_CLI_VERSION": "1.0.95",
        "COPILOT_CLI_DIST_DIR": "/synthetic-dist-override",
        "COPILOT_MODEL": "auto",
        "SWALLOWTAIL_TEST_INHERITED_VALUE": "synthetic-inherited-value",
        "SWALLOWTAIL_TEST_ORIGINAL_SCENARIO": scenario,
        "SWALLOWTAIL_TEST_RECEIPT": str(receipt),
        "SWALLOWTAIL_TEST_LAUNCH_OBSERVATION": str(launch_observation),
        "SWALLOWTAIL_TEST_CACHE_PARENT": str(root),
        "SWALLOWTAIL_TEST_HOST_HOME": str(synthetic_home),
    }
    record = run_original_session(
        executable, action, attempt, prompt, execution, plan, authority,
        authority_sha256, outer_seconds=outer_seconds,
        cleanup_seconds=cleanup_seconds, fake_test=True,
        base_environment=base_environment,
    )
    return record, {
        "root": root,
        "action": action,
        "records": records,
        "attempt": attempt,
        "prompt": prompt,
        "execution": execution,
        "receipt": receipt,
        "launch_observation": launch_observation,
    }


def self_test() -> dict[str, Any]:
    plan = validate_plan()
    correction_plan = validate_correction_plan()
    final_correction_plan = validate_final_correction_plan()
    task172_proposal = validate_task172_proposal()
    authority, authority_sha256 = validate_execution_authority(plan=plan)
    fake_authority = {"execution_authorized": False}
    fake_authority_sha256 = sha256_bytes(b"disabled-correction-plan-fake-authority")
    historical_record_hashes = {
        "attempt": sha256_file(ROOT / "docs/research/435-copilot-acp-normal-host-permission-attempt.json"),
        "observation": sha256_file(ROOT / "docs/research/435-copilot-acp-normal-host-permission-observation.json"),
    }
    results: dict[str, Any] = {}
    with tempfile.TemporaryDirectory(prefix="copilot-acp-host-permission-proof.") as temp_root:
        scratch = Path(temp_root).resolve()
        results["identity_queries"] = self_test_identity_queries(scratch / "identity")
        results["authority"] = {
            "preparation_plan_sha256": sha256_file(PLAN_PATH),
            "preparation_runner_sha256": PREPARATION_RUNNER_SHA256,
            "final_runner_sha256": sha256_file(SCRIPT_PATH),
            "execution_authority_sha256": authority_sha256,
        }
        results["corrected_proposal"] = {
            "plan_sha256": sha256_file(CORRECTION_PLAN_PATH),
            "execution_authorized": correction_plan["execution_authorized"],
            "pre_probes": len(correction_plan["pre_probes"]),
        }
        results["final_corrected_plan"] = {
            "plan_sha256": sha256_file(FINAL_CORRECTION_PLAN_PATH),
            "task_number": final_correction_plan["task_number"],
            "execution_authorized": final_correction_plan["execution_authorized"],
            "fake_pass_record_path": final_correction_plan["fake_pass_record_path"],
        }
        results["task172_proposal"] = {
            "plan_id": task172_proposal["plan_id"],
            "execution_authorized": task172_proposal["execution_authorized"],
            "shared_prompt_slots_remaining": task172_proposal["source_attempt"]["shared_prompt_slots_remaining"],
            "protocol_schema": task172_proposal["protocol"]["schema_version"],
        }

        bad_authority_path = scratch / "mismatched-authority.json"
        bad_authority = dict(authority)
        bad_authority["native_archive_sha256"] = "0" * 64
        with bad_authority_path.open("xb") as stream:
            stream.write(json.dumps(bad_authority, sort_keys=True).encode() + b"\n")
        mismatch_refused = False
        try:
            execute_original(bad_authority_path)
        except ValueError:
            mismatch_refused = True
        if not mismatch_refused or (scratch / "mismatch-effects").exists():
            raise RuntimeError("authority identity mismatch was not rejected before effects")
        results["identity_mismatch_before_effects"] = mismatch_refused

        stale_authority_refused = False
        try:
            execute_original()
        except RuntimeError as error:
            stale_authority_refused = "earlier runner" in str(error)
        if not stale_authority_refused:
            raise RuntimeError("consumed authority did not refuse launch under the changed runner")
        results["consumed_authority_stale_runner_refused"] = stale_authority_refused

        def run(name: str, scenario: str, **kwargs: Any) -> tuple[dict[str, Any], dict[str, Path]]:
            return run_fake_scenario(scratch / name, scenario, **kwargs)

        success, success_paths = run("success", "success")
        children = load_object(success_paths["state"] / "children-joined.json")
        if (
            success.get("status") != "passed"
            or success.get("permission_reply") != "cancelled"
            or success.get("sentinel_unchanged") is not True
            or success.get("unapproved_effect_absent") is not True
            or success.get("tool_announced") is not True
            or success.get("tool_announcement_status") != "pending"
            or success.get("tool_status") != "failed"
            or success.get("execution_reported") is not True
            or success.get("session_cancel_sent") is not True
            or success.get("prompt_result") != "cancelled"
            or success.get("permission_outcome") != "cancelled"
            or success.get("actual_effect_observed") is not False
            or children.get("fork_reaped") is not True
            or children.get("spawn_wait_exit") != 0
            or children.get("setsid_wait_exit") != 0
        ):
            raise RuntimeError("successful fake ACP permission exchange or child lifecycle failed")
        for path in (
            success_paths["home"],
            success_paths["xdg_config"],
            success_paths["xdg_cache"],
            success_paths["tmp"],
            success_paths["state"] / "runtime-state.json",
            success_paths["logs"] / "runtime.log",
        ):
            if not path.exists():
                raise RuntimeError("normal fake state creation, read/write, or log path was not exercised")
        success_record_bytes = (success_paths["records"] / "fake-execution.json").read_bytes()
        if (
            any(marker.encode() in success_record_bytes for marker in FORBIDDEN_PERSISTED_MARKERS)
            or any(value in success_record_bytes for value in (
                b"fake-tool", b"permission-sentinel.txt", SENTINEL_AFTER,
                b"rawInput", b"sessionId",
            ))
        ):
            raise RuntimeError("sanitized fake result persisted a raw secret/config marker")
        results["success"] = success

        diff_action, _ = run("diff-action", "diff-action")
        if (
            diff_action.get("status") != "passed"
            or diff_action.get("permission_action_attributed") is not True
            or diff_action.get("permission_action_matches_sentinel") is not True
            or diff_action.get("tool_kind") != "edit"
        ):
            raise RuntimeError("ACP edit diff fields did not attribute the sentinel action")
        results["diff_action"] = {
            "status": diff_action["status"],
            "permission_action_attributed": diff_action["permission_action_attributed"],
            "tool_kind": diff_action["tool_kind"],
        }

        end_turn, _ = run("permission-end-turn", "permission-end-turn")
        if (
            end_turn.get("status") != "failed"
            or end_turn.get("failure_class") != "prompt-not-cancelled"
            or end_turn.get("prompt_result") != "end_turn"
        ):
            raise RuntimeError("prompt result after a sent session/cancel was misclassified")
        results["permission_end_turn"] = {
            "status": end_turn["status"],
            "prompt_result": end_turn["prompt_result"],
            "session_cancel_sent": end_turn["session_cancel_sent"],
        }

        fsynced_regular_inodes: set[int] = set()
        fsynced_directory_inodes: set[int] = set()
        real_fsync = os.fsync

        def record_fsync(descriptor: int) -> None:
            metadata = os.fstat(descriptor)
            if stat.S_ISREG(metadata.st_mode):
                fsynced_regular_inodes.add(metadata.st_ino)
            elif stat.S_ISDIR(metadata.st_mode):
                fsynced_directory_inodes.add(metadata.st_ino)
            real_fsync(descriptor)

        os.fsync = record_fsync
        try:
            original_success, original_paths = run_fake_original(
                scratch / "original-success", "success", final_correction_plan,
                fake_authority, fake_authority_sha256,
            )
        finally:
            os.fsync = real_fsync
        original_validation = validate_original_execution_evidence(
            original_paths["execution"], original_paths["attempt"],
            original_paths["prompt"], final_correction_plan, fake_authority_sha256,
            allow_fake_authority=True,
        )
        results["original_shaped_record_validation"] = original_validation
        ledger_parent_inode = original_paths["attempt"].parent.stat().st_ino
        ledger_fsync = {
            "attempt_file": original_paths["attempt"].stat().st_ino in fsynced_regular_inodes,
            "prompt_file": original_paths["prompt"].stat().st_ino in fsynced_regular_inodes,
            "parent_directory": ledger_parent_inode in fsynced_directory_inodes,
        }
        if not all(ledger_fsync.values()):
            raise RuntimeError("original-shaped fake did not fsync both consumed ledgers and their parent")
        results["consumed_ledger_fsync"] = ledger_fsync
        launch_observation = load_object(original_paths["launch_observation"])
        if (
            original_success["status"] != "passed"
            or original_success["agent_version_reported"] != "1.0.93"
            or original_success["permission_reply"] != "cancelled"
            or original_success["permission_action_matches_sentinel"] is not True
            or original_success["sentinel_unchanged"] is not True
            or original_success["action_directory_unchanged"] is not True
            or original_success["model_observation"] != "fake-model"
            or not all(launch_observation.values())
            or list(original_paths["root"].glob("copilot-acp-package-cache.*"))
        ):
            raise RuntimeError(
                "corrected original-shaped fake failed sanitized checks: "
                + json.dumps({
                    "status": original_success["status"],
                    "failure_class": original_success["failure_class"],
                    "session_observed": original_success["session_observed"],
                    "action_directory_unchanged": original_success["action_directory_unchanged"],
                    "action_entry_names": sorted(item.name for item in original_paths["action"].iterdir()),
                    "sentinel_matches": (
                        (original_paths["action"] / "permission-sentinel.txt").is_file()
                        and (original_paths["action"] / "permission-sentinel.txt").read_bytes() == SENTINEL_BEFORE
                    ),
                    "launch": launch_observation,
                    "cache_remains": bool(list(original_paths["root"].glob("copilot-acp-package-cache.*"))),
                }, sort_keys=True)
            )
        results["original_shaped_launch"] = {
            "status": original_success["status"],
            "package_cache_cleanup": original_success["package_cache_cleanup"],
            "argv": "matched --model auto --acp --stdio",
            "environment": launch_observation,
            "private_package_cache_removed_after_cleanup": True,
            "permission_reply": original_success["permission_reply"],
            "prompt_result": original_success["prompt_result"],
            "sentinel_unchanged": original_success["sentinel_unchanged"],
            "action_directory_unchanged": original_success["action_directory_unchanged"],
        }
        original_record_tamper_rejections: dict[str, bool] = {}
        for name, changes in (
            ("authority", {"authority_sha256": "0" * 64}),
            ("permission", {"permission_reply": "approved"}),
            ("raw-field", {"raw_stderr": "synthetic-secret"}),
        ):
            tampered = dict(original_success)
            tampered.update(changes)
            try:
                validate_original_execution_record(
                    tampered, final_correction_plan, fake_authority_sha256,
                    allow_fake_authority=True,
                )
            except ValueError:
                original_record_tamper_rejections[name] = True
            else:
                original_record_tamper_rejections[name] = False
        if original_record_tamper_rejections != {
            "authority": True, "permission": True, "raw-field": True
        }:
            raise RuntimeError("original record validator accepted a mismatched authority or unsafe result")
        version_record_tamper_rejections: dict[str, bool] = {}
        for name, reported in (
            ("path", "/Users/operator/copilot"),
            ("oversized", "1.0.93+" + ("a" * REPORTED_VERSION_MAX_LENGTH)),
            ("secret-like", "1.0.93+token-secret"),
            ("arbitrary-string", "Copilot 1.0.93 authentication failed"),
            ("valid-drift", "1.0.92"),
        ):
            tampered = dict(original_success)
            tampered["agent_version_reported"] = reported
            try:
                validate_original_execution_record(
                    tampered, final_correction_plan, fake_authority_sha256,
                    allow_fake_authority=True,
                )
            except ValueError:
                version_record_tamper_rejections[name] = True
            else:
                version_record_tamper_rejections[name] = False
        if version_record_tamper_rejections != {
            "path": True,
            "oversized": True,
            "secret-like": True,
            "arbitrary-string": True,
            "valid-drift": True,
        }:
            raise RuntimeError("original record validator accepted an unsafe reported version")
        results["reported_version_tamper_rejections"] = version_record_tamper_rejections
        results["original_record_tamper_rejections"] = original_record_tamper_rejections

        original_attempt_bytes = original_paths["attempt"].read_bytes()
        original_attempt_replay_refused = False
        try:
            write_attempt_ledger(
                original_paths["attempt"], final_correction_plan, "original",
                authority_sha256=fake_authority_sha256,
            )
        except FileExistsError:
            original_attempt_replay_refused = True
        if not original_attempt_replay_refused or original_paths["attempt"].read_bytes() != original_attempt_bytes:
            raise RuntimeError("original-shaped fake attempt ledger allowed replay")
        original_prompt_bytes = original_paths["prompt"].read_bytes()
        original_prompt_replay_refused = False
        try:
            write_prompt_slot(original_paths["prompt"], original_attempt_bytes, final_correction_plan)
        except FileExistsError:
            original_prompt_replay_refused = True
        if not original_prompt_replay_refused or original_paths["prompt"].read_bytes() != original_prompt_bytes:
            raise RuntimeError("original-shaped fake prompt slot allowed replay")
        results["original_replay_refused"] = {
            "attempt": original_attempt_replay_refused,
            "prompt": original_prompt_replay_refused,
        }

        for scenario, expected_failure, received in (
            ("crash-before-prompt", {"unknown-eof", "protocol-write-failure"}, False),
            ("crash-after-prompt", {"unknown-eof"}, True),
        ):
            record, paths = run_fake_original(
                scratch / f"original-{scenario}", scenario, final_correction_plan,
                fake_authority, fake_authority_sha256,
            )
            validate_original_execution_evidence(
                paths["execution"], paths["attempt"], paths["prompt"],
                final_correction_plan, fake_authority_sha256,
                allow_fake_authority=True,
            )
            if (
                record["failure_class"] not in expected_failure
                or record["prompt_slot_fsynced_before_send"] is not True
                or paths["receipt"].exists() is not received
                or received and record["prompt_send_completed"] is not True
            ):
                raise RuntimeError(
                    f"original-shaped fake lost {scenario} consumption evidence: "
                    + json.dumps({
                        "failure_class": record["failure_class"],
                        "prompt_slot_fsynced_before_send": record["prompt_slot_fsynced_before_send"],
                        "prompt_send_completed": record["prompt_send_completed"],
                        "fake_prompt_received": paths["receipt"].exists(),
                        "exit_code": record["exit_code"],
                    }, sort_keys=True)
                )
            results[f"original-{scenario}"] = {
                "failure_class": record["failure_class"],
                "prompt_slot_fsynced_before_send": record["prompt_slot_fsynced_before_send"],
                "prompt_send_completed": record["prompt_send_completed"],
                "fake_prompt_received": paths["receipt"].exists(),
            }

        bounded, bounded_paths = run_fake_original(
            scratch / "original-bounded-cleanup", "hang-after-permission", final_correction_plan,
            fake_authority, fake_authority_sha256, outer_seconds=0.8, cleanup_seconds=0.25,
        )
        bounded_validation = validate_original_execution_evidence(
            bounded_paths["execution"], bounded_paths["attempt"],
            bounded_paths["prompt"], final_correction_plan,
            fake_authority_sha256, allow_fake_authority=True,
        )
        if (
            bounded["failure_class"] != "timeout"
            or bounded["elapsed_milliseconds"] > 800
            or bounded["cleanup"]["root_exit_observed"] is not True
            or bounded["cleanup"]["streams_joined"] is not True
        ):
            raise RuntimeError("original-shaped fake exceeded bounded cleanup or left the owned root unobserved")
        results["bounded_cleanup"] = {
            "record_validation": bounded_validation["status"],
            "failure_class": bounded["failure_class"],
            "elapsed_milliseconds": bounded["elapsed_milliseconds"],
            "root_exit_observed": bounded["cleanup"]["root_exit_observed"],
            "streams_joined": bounded["cleanup"]["streams_joined"],
            "descendant_cleanup": bounded["descendant_cleanup"],
            "package_cache_cleanup": bounded["package_cache_cleanup"],
        }

        real_temporary_directory_cleanup = tempfile.TemporaryDirectory.cleanup

        def fail_package_cache_cleanup(directory: tempfile.TemporaryDirectory) -> None:
            if Path(directory.name).name.startswith("copilot-acp-package-cache."):
                raise OSError(5, "synthetic cleanup failure")
            real_temporary_directory_cleanup(directory)

        tempfile.TemporaryDirectory.cleanup = fail_package_cache_cleanup
        try:
            cleanup_failed, cleanup_failed_paths = run_fake_original(
                scratch / "original-package-cache-cleanup-error", "success",
                final_correction_plan, fake_authority, fake_authority_sha256,
            )
        finally:
            tempfile.TemporaryDirectory.cleanup = real_temporary_directory_cleanup
        cleanup_failed_validation = validate_original_execution_evidence(
            cleanup_failed_paths["execution"], cleanup_failed_paths["attempt"],
            cleanup_failed_paths["prompt"], final_correction_plan,
            fake_authority_sha256, allow_fake_authority=True,
        )
        if (
            cleanup_failed.get("status") != "failed"
            or cleanup_failed.get("failure_class") != "package-cache-cleanup-failed"
            or cleanup_failed.get("package_cache_cleanup") != "failed"
            or cleanup_failed.get("package_cache_cleanup_error") != "os-error"
            or cleanup_failed.get("attempt_record_sha256") is None
            or cleanup_failed.get("prompt_slot_record_sha256") is None
            or not cleanup_failed_paths["attempt"].is_file()
            or not cleanup_failed_paths["prompt"].is_file()
            or not cleanup_failed_paths["execution"].is_file()
            or cleanup_failed.get("permission_reply") != "cancelled"
            or cleanup_failed.get("sentinel_unchanged") is not True
        ):
            raise RuntimeError("package-cache cleanup failure was swallowed or lost consumed evidence")
        results["package_cache_cleanup_error"] = {
            "record_validation": cleanup_failed_validation["status"],
            "failure_class": cleanup_failed["failure_class"],
            "cleanup": cleanup_failed["package_cache_cleanup"],
            "error_category": cleanup_failed["package_cache_cleanup_error"],
            "attempt_record_persisted": cleanup_failed_paths["attempt"].is_file(),
            "prompt_slot_persisted": cleanup_failed_paths["prompt"].is_file(),
            "execution_record_persisted": cleanup_failed_paths["execution"].is_file(),
        }

        tamper_rejections: dict[str, bool] = {}
        tampered_action = dict(success)
        tampered_action["permission_action_matches_sentinel"] = False
        tampered_extra = dict(success)
        tampered_extra["unreviewed_diagnostic"] = FAKE_SECRET
        tampered_version = dict(success)
        tampered_version["agent_version_reported"] = "1.0.92"
        for name, tampered in (
            ("action", tampered_action),
            ("field", tampered_extra),
            ("reported-version", tampered_version),
        ):
            try:
                validate_execution_record(tampered, plan)
            except ValueError:
                tamper_rejections[name] = True
            else:
                tamper_rejections[name] = False
        if tamper_rejections != {
            "action": True, "field": True, "reported-version": True
        }:
            raise RuntimeError("execution record validator accepted a false proof or an extra raw field")
        results["tamper_rejections"] = tamper_rejections

        expectations = {
            "spontaneous-effect": "unapproved-effect-observed",
            "no-permission": "permission-request-missing",
            "wrong-action": "permission-action-mismatch",
            "wrong-kind": "permission-action-unattributable",
            "missing-kind": "permission-action-unattributable",
            "missing-action": "permission-action-unattributable",
            "raw-input-only": "permission-action-unattributable",
            "wrong-id": "permission-tool-call-uncorrelated",
            "wrong-session": "session-id-mismatch",
            "missing-tool-id": "tool-call-id-invalid",
            "unknown-tool-update": "tool-call-update-uncorrelated",
            "in-progress-before-permission": "tool-execution-before-permission",
            "completed-before-permission": "tool-execution-before-permission",
            "failed-before-permission": "tool-execution-before-permission",
            "post-cancel-completed": "tool-execution-after-cancel",
            "malformed-permission-options": "permission-request-malformed",
            "duplicate-permission": "duplicate-permission-request",
            "malformed-frame": "malformed-frame",
            "oversized-frame": "inbound-frame-oversized",
            "early-eof": "unknown-eof",
            "initialize-error": "initialize-rpc-error",
            "wrong-version": "agent-version-mismatch",
            "missing-version": "agent-version-unobserved",
            "invalid-version": "agent-version-invalid",
            "oversized-version": "agent-version-invalid",
            "secret-version": "agent-version-invalid",
            "arbitrary-version": "agent-version-invalid",
            "nonstring-version": "agent-version-invalid",
            "session-new-error": "session-new-rpc-error",
            "tool-call-without-permission": "permission-request-missing",
            "hang-initialize": "timeout",
            "hang-after-permission": "timeout",
            "crash-before-prompt": "unknown-eof",
            "crash-after-prompt": "unknown-eof",
        }
        for scenario, expected in expectations.items():
            options = {"outer_seconds": 0.45, "cleanup_seconds": 0.20} if scenario.startswith("hang-") else {}
            record, paths = run(scenario, scenario, **options)
            if record.get("failure_class") != expected:
                raise RuntimeError(f"fake {scenario} classified as {record.get('failure_class')!r}, expected {expected!r}")
            if record.get("permission_reply") not in {"none", "cancelled"}:
                raise RuntimeError(f"fake {scenario} was not fail-closed")
            if scenario == "missing-kind" and (
                record.get("tool_kind") != "other"
                or record.get("permission_action_attributed") is not False
                or record.get("permission_action_matches_sentinel") is not False
            ):
                raise RuntimeError("missing optional kind was not safely defaulted as unattributable")
            if scenario in {"crash-before-prompt", "crash-after-prompt"}:
                prompt_record_exists = (paths["records"] / "1.0.93-prompt-1.json").is_file()
                received_marker = (paths["state"] / "prompt-received").exists()
                if not prompt_record_exists or received_marker != (scenario == "crash-after-prompt"):
                    raise RuntimeError(f"fake {scenario} lost before/after-send consumption evidence")
            if scenario in {
                "wrong-version", "missing-version", "invalid-version", "oversized-version",
                "secret-version", "arbitrary-version", "nonstring-version",
            } and (
                record["initialize_observed"] is not False
                or record["session_observed"] is not False
                or record["prompt_send_attempted"] is not False
            ):
                raise RuntimeError("version-drift fake reached a session or prompt before identity rejection")
            expected_reported_versions = {
                "wrong-version": "1.0.92",
                "missing-version": None,
                "invalid-version": None,
                "oversized-version": None,
                "secret-version": None,
                "arbitrary-version": None,
                "nonstring-version": None,
            }
            if scenario in expected_reported_versions and record.get("agent_version_reported") != expected_reported_versions[scenario]:
                raise RuntimeError(f"fake {scenario} did not preserve only a safe public version")
            saved = (paths["records"] / "fake-execution.json").read_bytes()
            if any(marker.encode() in saved for marker in FORBIDDEN_PERSISTED_MARKERS):
                raise RuntimeError(f"fake {scenario} persisted a forbidden marker")
            if scenario in {
                "invalid-version", "oversized-version", "secret-version", "arbitrary-version"
            }:
                forbidden_versions = (
                    b"/Users/operator/copilot", b"1.0.93+" + b"a" * REPORTED_VERSION_MAX_LENGTH,
                    b"1.0.93+token-secret", b"Copilot 1.0.93 authentication failed",
                )
                if any(value in saved for value in forbidden_versions):
                    raise RuntimeError(f"fake {scenario} retained an unsafe version string")
            results[scenario] = {
                "failure_class": record["failure_class"],
                "agent_version_observation": record["agent_version_observation"],
                "agent_version_reported": record["agent_version_reported"],
                "permission_reply": record["permission_reply"],
                "prompt_slot_fsynced_before_send": record["prompt_slot_fsynced_before_send"],
                "root_exit_observed": record["cleanup"]["root_exit_observed"],
                "streams_joined": record["cleanup"]["streams_joined"],
                "descendant_cleanup": record["descendant_cleanup"],
            }

        replay_record = success_paths["records"] / "1.0.93-attempt.json"
        original_bytes = replay_record.read_bytes()
        refused = False
        try:
            write_attempt_ledger(replay_record, plan, "fake")
        except FileExistsError:
            refused = True
        if not refused or replay_record.read_bytes() != original_bytes:
            raise RuntimeError("exclusive one-shot ledger allowed replay or changed prior bytes")
        prompt_record = success_paths["records"] / "1.0.93-prompt-1.json"
        prompt_bytes = prompt_record.read_bytes()
        if not load_object(prompt_record).get("prompt_consumed_before_send"):
            raise RuntimeError("prompt slot was not consumed before send")
        prompt_refused = False
        try:
            write_prompt_slot(prompt_record, original_bytes, plan)
        except FileExistsError:
            prompt_refused = True
        if not prompt_refused or prompt_record.read_bytes() != prompt_bytes:
            raise RuntimeError("exclusive prompt slot allowed replay or changed prior bytes")
        disabled_path = success_paths["records"] / "disabled-original-attempt.json"
        original_refused = False
        try:
            write_attempt_ledger(disabled_path, correction_plan, "original")
        except RuntimeError:
            original_refused = True
        if not original_refused or disabled_path.exists():
            raise RuntimeError("disabled plan wrote an original attempt record")
        results["prompt_slot_bytes"] = len(prompt_bytes)
        results["replay_refused"] = refused
        results["prompt_replay_refused"] = prompt_refused
        results["original_entrypoint_refused"] = original_refused
        if historical_record_hashes != {
            "attempt": sha256_file(ROOT / "docs/research/435-copilot-acp-normal-host-permission-attempt.json"),
            "observation": sha256_file(ROOT / "docs/research/435-copilot-acp-normal-host-permission-observation.json"),
        }:
            raise RuntimeError("offline fake work changed the consumed Research 435 records")
        results["research_435_records_unchanged"] = True
    results["status"] = "passed"
    results["execution_authorized"] = final_correction_plan["execution_authorized"]
    results["historical_authority_validated_but_stale_for_current_runner"] = (
        authority["final_runner_sha256"] != sha256_file(SCRIPT_PATH)
    )
    results["execution_authority_sha256"] = authority_sha256
    results["runner_sha256"] = sha256_file(SCRIPT_PATH)
    results["preparation_plan_sha256"] = sha256_file(PLAN_PATH)
    results["originals_run"] = False
    results["qualification_changed"] = False
    results["task_number"] = final_correction_plan["task_number"]
    results["task_id"] = final_correction_plan["task_id"]
    results["run_id"] = final_correction_plan["run_id"]
    results["operator_decision"] = final_correction_plan["operator_decision"]
    return results


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    actions = parser.add_mutually_exclusive_group(required=True)
    actions.add_argument("--validate-plan", action="store_true")
    actions.add_argument("--validate-correction-plan", action="store_true")
    actions.add_argument("--validate-final-plan", action="store_true")
    actions.add_argument("--validate-task-172-proposal", action="store_true")
    actions.add_argument("--validate-authority", action="store_true")
    actions.add_argument("--validate-corrected-authority", action="store_true")
    actions.add_argument("--self-test", action="store_true")
    actions.add_argument("--validate-record", metavar="PATH")
    actions.add_argument("--validate-original-record", metavar="PATH")
    actions.add_argument("--validate-corrected-original-record", metavar="PATH")
    actions.add_argument("--execute-original", action="store_true")
    actions.add_argument("--execute-corrected-original", action="store_true")
    parser.add_argument("--fake-pass-record", metavar="PATH")
    args = parser.parse_args()
    try:
        if args.fake_pass_record and not args.self_test:
            raise ValueError("fake-pass evidence can only be written by the fake self-test")
        if args.validate_plan:
            plan = validate_plan()
            print(json.dumps({"status": "valid", "plan_id": plan["plan_id"], "original_execution_enabled": False}))
        elif args.validate_correction_plan:
            plan = validate_correction_plan()
            print(json.dumps({
                "status": "valid",
                "plan_id": plan["plan_id"],
                "original_execution_enabled": plan["original_execution_enabled"],
                "execution_authorized": plan["execution_authorized"],
                "implementation_sha256": plan["runner"]["implementation_sha256"],
            }, sort_keys=True))
        elif args.validate_final_plan:
            plan = validate_final_correction_plan()
            print(json.dumps({
                "status": "valid",
                "plan_id": plan["plan_id"],
                "task_number": plan["task_number"],
                "execution_authorized": plan["execution_authorized"],
                "plan_sha256": sha256_file(FINAL_CORRECTION_PLAN_PATH),
                "runner_sha256": plan["runner"]["implementation_sha256"],
            }, sort_keys=True))
        elif args.validate_task_172_proposal:
            proposal = validate_task172_proposal()
            print(json.dumps({
                "status": "valid",
                "plan_id": proposal["plan_id"],
                "execution_authorized": proposal["execution_authorized"],
                "shared_prompt_slots_remaining": proposal["source_attempt"]["shared_prompt_slots_remaining"],
                "runner_sha256": proposal["runner"]["implementation_sha256"],
            }, sort_keys=True))
        elif args.validate_authority:
            plan = validate_plan()
            authority, authority_sha256 = validate_execution_authority(plan=plan)
            print(json.dumps({
                "status": "valid",
                "execution_authorized": (
                    authority["execution_authorized"]
                    and authority["final_runner_sha256"] == sha256_file(SCRIPT_PATH)
                ),
                "authority_sha256": authority_sha256,
                "final_runner_sha256": authority["final_runner_sha256"],
                "preparation_plan_sha256": authority["preparation_plan_sha256"],
            }, sort_keys=True))
        elif args.validate_corrected_authority:
            plan = validate_final_correction_plan()
            authority, authority_sha256 = validate_corrected_authority(plan=plan)
            print(json.dumps({
                "status": "valid",
                "execution_authorized": authority["execution_authorized"],
                "authority_sha256": authority_sha256,
                "final_plan_sha256": authority["final_plan_sha256"],
                "final_runner_sha256": authority["final_runner_sha256"],
                "fake_pass_record_sha256": authority["fake_pass_record_sha256"],
            }, sort_keys=True))
        elif args.validate_record:
            plan = validate_plan()
            record = load_object(Path(args.validate_record))
            print(json.dumps(validate_execution_record(record, plan)))
        elif args.validate_original_record:
            plan = validate_plan()
            _, authority_sha256 = validate_execution_authority(plan=plan)
            record_path = Path(args.validate_original_record)
            print(json.dumps(validate_original_execution_evidence(
                record_path,
                record_path.with_name(ATTEMPT_EVIDENCE_NAME),
                record_path.with_name(PROMPT_EVIDENCE_NAME),
                plan,
                authority_sha256,
                require_private_records=False,
            ), sort_keys=True))
        elif args.validate_corrected_original_record:
            print(json.dumps(validate_corrected_original_evidence(
                Path(args.validate_corrected_original_record)
            ), sort_keys=True))
        elif args.execute_original:
            record = execute_original()
            print(json.dumps(record, sort_keys=True))
        elif args.execute_corrected_original:
            record = execute_corrected_original()
            print(json.dumps(record, sort_keys=True))
        else:
            if args.fake_pass_record and Path(args.fake_pass_record).resolve() != CORRECTED_FAKE_PASS_PATH.resolve():
                raise ValueError("fake-pass record must use the reviewed Research 437 path")
            result = self_test()
            if args.fake_pass_record:
                create_fake_pass_record(result)
                result["fake_pass_record_sha256"] = sha256_file(CORRECTED_FAKE_PASS_PATH)
            print(json.dumps(result, sort_keys=True))
        return 0
    except (OSError, ValueError, RuntimeError, ProtocolFailure, subprocess.SubprocessError) as error:
        detail = str(error) if isinstance(error, (ValueError, RuntimeError, ProtocolFailure)) else type(error).__name__
        if args.self_test and isinstance(error, OSError) and isinstance(error.filename, str):
            detail += f" near {Path(error.filename).name}"
        print(f"copilot host permission proof failed: {detail}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] in {"--fake-agent", "--fake-child"}:
        raise SystemExit(fake_agent_main())
    raise SystemExit(main())
