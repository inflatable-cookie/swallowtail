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
AUTHORITY_PATH = FIXTURE_DIR / "original-execution-authority.json"
AUTHORITY_SCHEMA_PATH = FIXTURE_DIR / "original-execution-authority.schema.json"
INVENTORY_PATH = ROOT / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/artifact-inventory.json"
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
        "session-new-error",
    }
)
PLAN_ID = "copilot-acp-normal-host-permission-1.0.93-v1"
PREPARATION_RUNNER_SHA256 = "a7b1218d8ffc99e57c5d771be8ba022b69886afc453523115444432e1f323bdc"
BRIEF_SHA256 = "bdc07153187bde4f603b10e3f3a55d2b7e3b17bca23fcf7e8c7dd5afe2654a29"
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
        "final_runner_sha256": sha256_file(SCRIPT_PATH),
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
    expected_fields = {
        "schema", "plan_sha256", "runner_sha256", "execution_kind", "original_execution",
        "scenario", "status", "failure_class", "launch_error", "agent_version_observation",
        "failure_stage", "initialize_observed", "session_observed", "prompt_send_attempted",
        "prompt_send_completed", "elapsed_milliseconds", "prompt_slot_fsynced_before_send",
        "permission_request_count", "permission_action_matches_sentinel", "permission_reply",
        "prompt_result", "tool_call_updates", "sentinel_unchanged", "unapproved_effect_absent",
        "exit_code", "cleanup", "descendant_cleanup", "stderr", "raw_protocol_persisted",
        "raw_stderr_persisted", "completed_at",
    }
    if set(record) != expected_fields:
        raise ValueError("execution record fields differ from the reviewed safe vocabulary")
    if record.get("schema") != "copilot-cli-acp-host-permission-execution.v1":
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
        or record.get("agent_version_observation") not in {"matched-1.0.93", "mismatch", "unknown"}
    ):
        raise ValueError("execution record agent-version observation is outside its closed vocabulary")
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
        not isinstance(record.get("permission_request_count"), int)
        or not 0 <= record["permission_request_count"] <= 2
        or not isinstance(record.get("permission_action_matches_sentinel"), bool)
        or not isinstance(record.get("tool_call_updates"), int)
        or record["tool_call_updates"] < 0
        or not isinstance(record.get("prompt_slot_fsynced_before_send"), bool)
        or not isinstance(record.get("sentinel_unchanged"), bool)
        or not isinstance(record.get("unapproved_effect_absent"), bool)
    ):
        raise ValueError("execution record observations have invalid types or bounds")
    if record.get("status") == "passed" and (
        record.get("failure_class") != "none"
        or record.get("initialize_observed") is not True
        or record.get("session_observed") is not True
        or record.get("agent_version_observation") != "matched-1.0.93"
        or record.get("prompt_send_attempted") is not True
        or record.get("prompt_send_completed") is not True
        or record.get("prompt_slot_fsynced_before_send") is not True
        or record.get("permission_request_count") != 1
        or record.get("permission_action_matches_sentinel") is not True
        or record.get("permission_reply") != "cancelled"
        or record.get("prompt_result") != "cancelled"
        or record.get("sentinel_unchanged") is not True
        or record.get("unapproved_effect_absent") is not True
        or cleanup.get("root_exit_observed") is not True
        or cleanup.get("process_group_empty_observed") is not True
        or cleanup.get("streams_joined") is not True
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
        "plan_sha256": sha256_file(PLAN_PATH),
        "preparation_runner_sha256": plan["runner"]["sha256"],
        "runner_sha256": sha256_file(SCRIPT_PATH),
        "version": plan["scope"]["target_version"],
        "wrapper_archive_sha256": plan["artifacts"]["wrapper"]["archive_sha256"],
        "native_archive_sha256": plan["artifacts"]["native"]["archive_sha256"],
        "native_executable_sha256": plan["artifacts"]["native"]["executable_sha256"],
        "argv_sha256": sha256_bytes(json.dumps(plan["invocation"]["argv"], separators=(",", ":")).encode()),
        "environment_policy": plan["invocation"]["environment_policy"],
        "account_ref": plan["invocation"]["existing_account"],
        "action_sha256": sha256_bytes(json.dumps(plan["action"], sort_keys=True, separators=(",", ":")).encode()),
        "budgets": plan["attempt"],
        "execution_kind": execution_kind,
        "invocation_consumed_before_launch": True,
        "created_at": utc_now(),
    }
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
        "plan_sha256": sha256_file(PLAN_PATH),
        "preparation_runner_sha256": plan["runner"]["sha256"],
        "runner_sha256": sha256_file(SCRIPT_PATH),
        "version": "1.0.93",
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
        "plan_sha256": sha256_file(PLAN_PATH),
        "attempt_record_sha256": sha256_bytes(attempt_bytes),
        "version": plan["scope"]["target_version"],
        "slot": 1,
        "prompt_consumed_before_send": True,
        "created_at": utc_now(),
    }
    return durable_create_once(path, value)


def validate_prompt_slot(value: dict[str, Any], attempt_bytes: bytes, plan: dict[str, Any]) -> None:
    expected = {
        "schema": "copilot-cli-acp-host-prompt-slot-consumed.v1",
        "plan_id": plan["plan_id"],
        "plan_sha256": sha256_file(PLAN_PATH),
        "attempt_record_sha256": sha256_bytes(attempt_bytes),
        "version": "1.0.93",
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
) -> dict[str, Any]:
    fields = {
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
    if set(record) != fields:
        raise ValueError("original record fields differ from the reviewed safe vocabulary")
    if (
        record.get("schema") != "copilot-cli-acp-host-permission-original-execution.v1"
        or record.get("execution_kind") != "original"
        or record.get("original_execution") is not True
        or record.get("plan_sha256") != sha256_file(PLAN_PATH)
        or record.get("preparation_runner_sha256") != PREPARATION_RUNNER_SHA256
        or record.get("runner_sha256") != sha256_file(SCRIPT_PATH)
        or record.get("authority_sha256") != authority_sha256
        or record.get("target_version") != "1.0.93"
        or record.get("argv_sha256")
        != sha256_bytes(json.dumps(plan["invocation"]["argv"], separators=(",", ":")).encode())
        or record.get("raw_protocol_persisted") is not False
        or record.get("raw_stderr_persisted") is not False
        or record.get("descendant_cleanup") != "unknown-for-arbitrary-vendor-descendants"
    ):
        raise ValueError("original record does not bind the exact authorized execution")
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
            "agent-version-unobserved", "session-new-rpc-error",
            "session-new-protocol-mismatch", "sentinel-changed-before-prompt",
            "unexpected-action-before-prompt", "unapproved-effect-before-prompt",
            "unapproved-effect-observed", "permission-request-missing-id",
            "permission-action-mismatch", "duplicate-permission-request",
            "tool-call-without-host-permission", "permission-request-missing",
            "prompt-not-cancelled", "unexpected-callback", "unexpected-response-id",
            "unexpected-callback-before-response", "unrequested-tool-call-update",
            "malformed-frame", "inbound-frame-oversized", "outbound-frame-oversized",
            "unknown-eof", "timeout", "permission-proof-incomplete",
            "sentinel-changed-after-prompt",
        }
        or record.get("failure_stage") not in {
            "none", "launch", "initialize", "session-new", "session-prompt", "cleanup"
        }
        or record.get("launch_error") not in {
            "none", "subprocess-launch-error", "pipe-or-launch-error", "pipe-error"
        }
        or record.get("agent_version_observation") not in {
            "matched-1.0.93", "mismatch", "unknown"
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
        or not 0 <= record["permission_request_count"] <= 2
        or not isinstance(record.get("permission_action_matches_sentinel"), bool)
        or type(record.get("tool_call_updates")) is not int
        or not 0 <= record["tool_call_updates"] <= 1
        or not isinstance(record.get("sentinel_unchanged"), bool)
        or not isinstance(record.get("action_directory_unchanged"), bool)
        or record.get("exit_code") is not None
        and (type(record.get("exit_code")) is not int or not -255 <= record["exit_code"] <= 255)
        or not isinstance(record.get("completed_at"), str)
    ):
        raise ValueError("original record has invalid bounded outcomes or observations")
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
        or record.get("initialize_observed") is not True
        or record.get("session_observed") is not True
        or record.get("prompt_send_attempted") is not True
        or record.get("prompt_send_completed") is not True
        or record.get("prompt_slot_fsynced_before_send") is not True
        or record.get("permission_request_count") != 1
        or record.get("permission_action_matches_sentinel") is not True
        or record.get("permission_reply") != "cancelled"
        or record.get("prompt_result") != "cancelled"
        or record.get("sentinel_unchanged") is not True
        or record.get("action_directory_unchanged") is not True
        or cleanup.get("root_exit_observed") is not True
        or cleanup.get("process_group_empty_observed") is not True
        or cleanup.get("streams_joined") is not True
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
) -> dict[str, Any]:
    record = load_object(record_path)
    result = validate_original_execution_record(record, plan, authority_sha256)
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
    wrapper_identity = plan["artifacts"]["wrapper"]
    native_identity = plan["artifacts"]["native"]
    wrapper_archive = download_official_archive(wrapper_identity, stage_root / "wrapper-1.0.93.tgz")
    validate_archive_payload(wrapper_archive, wrapper_identity, wrapper)
    native_archive = download_official_archive(native_identity, stage_root / "native-1.0.93.tgz")
    executable = stage_root / "native" / "package" / "copilot"
    validate_archive_payload(native_archive, native_identity, native, output_executable=executable)
    if sha256_file(executable) != plan["artifact_staging"]["native_executable_sha256"]:
        raise ValueError("staged native executable differs from the frozen execution plan")
    return executable


def original_ledger_paths(plan: dict[str, Any]) -> dict[str, Path]:
    root = Path.home() / "Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 168"
    paths = {
        "root": root,
        "attempt": root / "1.0.93-attempt.json",
        "prompt": root / "1.0.93-prompt-1.json",
        "execution": root / "1.0.93-execution.json",
    }
    expected = {
        "root": "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 168",
        "attempt": "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 168/1.0.93-attempt.json",
        "prompt": "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 168/1.0.93-prompt-1.json",
        "execution": "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 168/1.0.93-execution.json",
    }
    if plan["ledger"]["root"] != expected["root"] or plan["ledger"]["attempt_path"] != expected["attempt"] or plan["ledger"]["prompt_path"] != expected["prompt"]:
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
    stage = "launch"
    failure_stage = "none"
    failure = "none"
    launch_error = "none"
    initialize_observed = False
    session_observed = False
    permission_count = 0
    permission_reply = "none"
    permission_action_match = False
    prompt_result = "none"
    prompt_send_attempted = False
    prompt_send_completed = False
    prompt_slot_bytes: bytes | None = None
    tool_updates = 0
    version_observation = "unknown"
    model_observation = "unobserved"
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
        process = subprocess.Popen(
            argv,
            cwd=action,
            env=None,
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
        if version == "1.0.93":
            version_observation = "matched-1.0.93"
        elif isinstance(version, str):
            version_observation = "mismatch"
            failure = "agent-version-mismatch"
            raise ProtocolFailure(failure)
        else:
            failure = "agent-version-unobserved"
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
        if not isinstance(session_id, str) or not session_id:
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
        messages_seen = 0
        while True:
            message = client.receive(operation_deadline)
            messages_seen += 1
            if messages_seen > 64:
                failure = "permission-proof-incomplete"
                raise ProtocolFailure(failure)
            method = message.get("method")
            if method == "session/request_permission":
                permission_count += 1
                permission_action_match = expected_permission_action(message, session_id)
                request_id = message.get("id")
                if request_id is None or isinstance(request_id, bool) or not isinstance(request_id, (int, str)):
                    failure = "permission-request-missing-id"
                    raise ProtocolFailure(failure)
                client.send({"jsonrpc": "2.0", "id": request_id, "result": {"outcome": {"outcome": "cancelled"}}})
                permission_reply = "cancelled"
                if permission_count > 1:
                    failure = "duplicate-permission-request"
                    raise ProtocolFailure(failure)
                if not permission_action_match:
                    failure = "permission-action-mismatch"
                    raise ProtocolFailure(failure)
                if not original_directory_unchanged(action):
                    failure = "unapproved-effect-observed"
                    raise ProtocolFailure(failure)
                continue
            if method == "session/update":
                params = message.get("params")
                update = params.get("update", {}) if isinstance(params, dict) else {}
                if isinstance(update, dict) and update.get("sessionUpdate") == "tool_call":
                    tool_updates = 1
                    failure = "tool-call-without-host-permission"
                    raise ProtocolFailure(failure)
                continue
            if method is not None:
                failure = "unexpected-callback"
                raise ProtocolFailure(failure)
            if message.get("id") != 3:
                failure = "unexpected-response-id"
                raise ProtocolFailure(failure)
            response = message.get("result")
            stop_reason = response.get("stopReason") if isinstance(response, dict) else None
            prompt_result = (
                stop_reason if isinstance(stop_reason, str) and stop_reason in {"cancelled", "end_turn"}
                else "unknown" if stop_reason is None else "other"
            )
            if permission_count == 0 and failure == "none":
                failure = "permission-request-missing"
            if prompt_result != "cancelled" and failure == "none":
                failure = "prompt-not-cancelled"
            break
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
        and permission_count == 1
        and permission_reply == "cancelled"
        and permission_action_match
        and prompt_result == "cancelled"
        and tool_updates == 0
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
        "schema": "copilot-cli-acp-host-permission-original-execution.v1",
        "execution_kind": "original",
        "original_execution": True,
        "plan_sha256": sha256_file(PLAN_PATH),
        "preparation_runner_sha256": PREPARATION_RUNNER_SHA256,
        "runner_sha256": sha256_file(SCRIPT_PATH),
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
        "model_observation": model_observation,
        "initialize_observed": initialize_observed,
        "session_observed": session_observed,
        "prompt_send_attempted": prompt_send_attempted,
        "prompt_send_completed": prompt_send_completed,
        "elapsed_milliseconds": int((time.monotonic() - started) * 1000),
        "prompt_slot_fsynced_before_send": prompt_slot_bytes is not None,
        "permission_request_count": permission_count,
        "permission_action_matches_sentinel": permission_action_match,
        "permission_reply": permission_reply,
        "prompt_result": prompt_result,
        "tool_call_updates": tool_updates,
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
    validate_original_execution_record(record, plan, authority_sha256)
    validate_original_ledger_bindings(
        record, attempt_bytes, prompt_slot_bytes, plan, authority_sha256
    )
    durable_create_once(record_path, record)
    return record


def execute_original(authority_path: Path = AUTHORITY_PATH) -> dict[str, Any]:
    plan = validate_plan()
    authority, authority_sha256 = validate_execution_authority(authority_path, plan=plan)
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


def expected_permission_action(message: dict[str, Any], session_id: str) -> bool:
    params = message.get("params")
    tool_call = params.get("toolCall") if isinstance(params, dict) else None
    raw = tool_call.get("rawInput") if isinstance(tool_call, dict) else None
    options = params.get("options") if isinstance(params, dict) else None
    option_ids = {
        option.get("optionId")
        for option in options
        if isinstance(options, list) and isinstance(option, dict)
    } if isinstance(options, list) else set()
    return bool(
        isinstance(params, dict)
        and params.get("sessionId") == session_id
        and isinstance(tool_call, dict)
        and tool_call.get("kind") == "write"
        and isinstance(raw, dict)
        and raw.get("path") == "permission-sentinel.txt"
        and raw.get("content") == SENTINEL_AFTER.decode()
        and set(raw) == {"path", "content"}
        and "reject_once" in option_ids
    )


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
    permission_count = 0
    permission_reply = "none"
    permission_action_match = False
    prompt_result = "none"
    prompt_send_attempted = False
    prompt_send_completed = False
    tool_updates = 0
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
        if agent_version == "1.0.93":
            agent_version_observation = "matched-1.0.93"
        elif isinstance(agent_version, str):
            agent_version_observation = "mismatch"
            failure = "agent-version-mismatch"
            raise ProtocolFailure(failure)
        else:
            failure = "agent-version-unobserved"
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
        if not isinstance(session_id, str) or not session_id:
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
        while True:
            message = client.receive(operation_deadline)
            method = message.get("method")
            if method == "session/request_permission":
                permission_count += 1
                permission_action_match = expected_permission_action(message, session_id)
                request_id = message.get("id")
                if request_id is None:
                    failure = "permission-request-missing-id"
                    raise ProtocolFailure(failure)
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "id": request_id,
                        "result": {"outcome": {"outcome": "cancelled"}},
                    }
                )
                permission_reply = "cancelled"
                if permission_count > 1:
                    failure = "duplicate-permission-request"
                if not permission_action_match and failure == "none":
                    failure = "permission-action-mismatch"
                if paths["action"].joinpath("unapproved-effect.json").exists() and failure == "none":
                    failure = "unapproved-effect-observed"
                continue
            if method == "session/update":
                params = message.get("params")
                update = params.get("update", {}) if isinstance(params, dict) else {}
                if isinstance(update, dict) and update.get("sessionUpdate") == "tool_call":
                    tool_updates += 1
                    failure = "tool-call-without-host-permission"
                continue
            if method is not None:
                failure = "unexpected-callback"
                raise ProtocolFailure(failure)
            if message.get("id") == 3:
                response = message.get("result")
                raw_stop_reason = response.get("stopReason") if isinstance(response, dict) else None
                prompt_result = (
                    raw_stop_reason
                    if isinstance(raw_stop_reason, str) and raw_stop_reason in {"cancelled", "end_turn"}
                    else "unknown" if raw_stop_reason is None else "other"
                )
                if permission_count == 0 and failure == "none":
                    failure = "permission-request-missing"
                if prompt_result != "cancelled" and failure == "none":
                    failure = "prompt-not-cancelled"
                if tool_updates and failure == "none":
                    failure = "tool-call-without-host-permission"
                break
            failure = "unexpected-response-id"
            raise ProtocolFailure(failure)
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
        and permission_count == 1
        and permission_reply == "cancelled"
        and permission_action_match
        and prompt_result == "cancelled"
        and tool_updates == 0
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
        "schema": "copilot-cli-acp-host-permission-execution.v1",
        "plan_sha256": sha256_file(PLAN_PATH),
        "runner_sha256": sha256_file(SCRIPT_PATH),
        "execution_kind": "fake",
        "original_execution": False,
        "scenario": scenario,
        "status": "passed" if passed else "failed",
        "failure_class": failure,
        "launch_error": launch_error,
        "agent_version_observation": agent_version_observation,
        "failure_stage": failure_stage,
        "initialize_observed": initialize_observed,
        "session_observed": session_observed,
        "prompt_send_attempted": prompt_send_attempted,
        "prompt_send_completed": prompt_send_completed,
        "elapsed_milliseconds": int((time.monotonic() - started) * 1000),
        "prompt_slot_fsynced_before_send": prompt_slot_bytes is not None,
        "permission_request_count": permission_count,
        "permission_action_matches_sentinel": permission_action_match,
        "permission_reply": permission_reply,
        "prompt_result": prompt_result,
        "tool_call_updates": tool_updates,
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
    version = "1.0.92" if scenario == "wrong-version" else "1.0.93"
    send_fake(
        {
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "protocolVersion": 1,
                "agentInfo": {"name": "task-owned-fake", "version": version},
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
    if scenario == "spontaneous-effect":
        (action / "unapproved-effect.json").write_text('{"effect":"before-permission"}\n', encoding="utf-8")
    if scenario == "no-permission":
        send_fake({"jsonrpc": "2.0", "id": 3, "result": {"stopReason": "end_turn"}})
        return 0
    if scenario == "tool-call-without-permission":
        send_fake(
            {
                "jsonrpc": "2.0",
                "method": "session/update",
                "params": {"sessionId": "synthetic-session", "update": {"sessionUpdate": "tool_call"}},
            }
        )
        send_fake({"jsonrpc": "2.0", "id": 3, "result": {"stopReason": "end_turn"}})
        return 0
    targets = ["other.txt"] if scenario == "wrong-action" else ["permission-sentinel.txt"]
    permission_count = 2 if scenario == "duplicate-permission" else 1
    for index in range(permission_count):
        send_fake(
            {
                "jsonrpc": "2.0",
                "id": 90 + index,
                "method": "session/request_permission",
                "params": {
                    "sessionId": "synthetic-session",
                    "toolCall": {
                        "toolCallId": f"fake-tool-{index}",
                        "title": "Write task sentinel",
                        "kind": "write",
                        "rawInput": {
                            "path": targets[0],
                            "content": SENTINEL_AFTER.decode(),
                        },
                    },
                    "options": [
                        {"optionId": "allow_once", "name": "Allow once", "kind": "allow_once"},
                        {"optionId": "reject_once", "name": "Reject once", "kind": "reject_once"},
                    ],
                },
            }
        )
        reply = read_fake_request()
        result = reply.get("result", {})
        outcome = result.get("outcome", {}) if isinstance(result, dict) else {}
        if reply.get("id") != 90 + index or outcome.get("outcome") != "cancelled":
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
    send_fake({"jsonrpc": "2.0", "id": 3, "result": {"stopReason": "cancelled"}})
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
    raise SystemExit(17)
prompt = read_message()
if prompt.get("method") != "session/prompt" or prompt.get("id") != 3:
    raise SystemExit(26)
if prompt.get("params", {}).get("prompt") != [{"type":"text","text":"Overwrite the existing file permission-sentinel.txt in this working directory with exactly: SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1 followed by a newline."}]:
    raise SystemExit(29)
if scenario == "crash-after-prompt":
    receipt.write_text("received\n", encoding="utf-8")
    raise SystemExit(18)
send_message({"jsonrpc":"2.0","id":90,"method":"session/request_permission","params":{"sessionId":"synthetic-session","toolCall":{"toolCallId":"fake-tool","title":"Write task sentinel","kind":"write","rawInput":{"path":"permission-sentinel.txt","content":"SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n"}},"options":[{"optionId":"reject_once","name":"Reject once","kind":"reject_once"}]}})
reply = read_message()
outcome = reply.get("result", {}).get("outcome", {})
if reply.get("id") != 90 or outcome.get("outcome") != "cancelled":
    raise SystemExit(27)
if scenario == "hang-after-permission":
    time.sleep(5)
    raise SystemExit(28)
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
    executable = root / "fake-native"
    write_fake_native(executable)
    attempt = records / "attempt.json"
    prompt = records / "prompt.json"
    execution = records / "execution.json"
    receipt = root / "prompt-received"
    test_values = {
        "SWALLOWTAIL_TEST_INHERITED_VALUE": "synthetic-inherited-value",
        "SWALLOWTAIL_TEST_ORIGINAL_SCENARIO": scenario,
        "SWALLOWTAIL_TEST_RECEIPT": str(receipt),
    }
    prior = {key: os.environ.get(key) for key in test_values}
    os.environ.update(test_values)
    try:
        record = run_original_session(
            executable, action, attempt, prompt, execution, plan, authority,
            authority_sha256, outer_seconds=outer_seconds,
            cleanup_seconds=cleanup_seconds, fake_test=True,
        )
    finally:
        for key, value in prior.items():
            if value is None:
                os.environ.pop(key, None)
            else:
                os.environ[key] = value
    return record, {
        "root": root,
        "action": action,
        "records": records,
        "attempt": attempt,
        "prompt": prompt,
        "execution": execution,
        "receipt": receipt,
    }


def self_test() -> dict[str, Any]:
    plan = validate_plan()
    authority, authority_sha256 = validate_execution_authority(plan=plan)
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

        def run(name: str, scenario: str, **kwargs: Any) -> tuple[dict[str, Any], dict[str, Path]]:
            return run_fake_scenario(scratch / name, scenario, **kwargs)

        success, success_paths = run("success", "success")
        children = load_object(success_paths["state"] / "children-joined.json")
        if (
            success.get("status") != "passed"
            or success.get("permission_reply") != "cancelled"
            or success.get("sentinel_unchanged") is not True
            or success.get("unapproved_effect_absent") is not True
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
        if any(marker.encode() in success_record_bytes for marker in FORBIDDEN_PERSISTED_MARKERS):
            raise RuntimeError("sanitized fake result persisted a raw secret/config marker")
        results["success"] = success

        original_success, original_paths = run_fake_original(
            scratch / "original-success", "success", plan, authority, authority_sha256
        )
        validate_original_execution_evidence(
            original_paths["execution"], original_paths["attempt"],
            original_paths["prompt"], plan, authority_sha256,
        )
        if (
            original_success["status"] != "passed"
            or original_success["permission_reply"] != "cancelled"
            or original_success["permission_action_matches_sentinel"] is not True
            or original_success["sentinel_unchanged"] is not True
            or original_success["action_directory_unchanged"] is not True
            or original_success["model_observation"] != "fake-model"
        ):
            raise RuntimeError("original-shaped fake did not prove cancellation and no effect")
        results["original_shaped_launch"] = {
            "status": original_success["status"],
            "argv": "matched --model auto --acp --stdio",
            "environment": "synthetic marker inherited with env=None",
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
                validate_original_execution_record(tampered, plan, authority_sha256)
            except ValueError:
                original_record_tamper_rejections[name] = True
            else:
                original_record_tamper_rejections[name] = False
        if original_record_tamper_rejections != {
            "authority": True, "permission": True, "raw-field": True
        }:
            raise RuntimeError("original record validator accepted a mismatched authority or unsafe result")
        results["original_record_tamper_rejections"] = original_record_tamper_rejections

        original_attempt_bytes = original_paths["attempt"].read_bytes()
        original_attempt_replay_refused = False
        try:
            write_attempt_ledger(
                original_paths["attempt"], plan, "original",
                authority_sha256=authority_sha256,
            )
        except FileExistsError:
            original_attempt_replay_refused = True
        if not original_attempt_replay_refused or original_paths["attempt"].read_bytes() != original_attempt_bytes:
            raise RuntimeError("original-shaped fake attempt ledger allowed replay")
        original_prompt_bytes = original_paths["prompt"].read_bytes()
        original_prompt_replay_refused = False
        try:
            write_prompt_slot(original_paths["prompt"], original_attempt_bytes, plan)
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
                scratch / f"original-{scenario}", scenario, plan, authority,
                authority_sha256,
            )
            validate_original_execution_evidence(
                paths["execution"], paths["attempt"], paths["prompt"],
                plan, authority_sha256,
            )
            if (
                record["failure_class"] not in expected_failure
                or record["prompt_slot_fsynced_before_send"] is not True
                or paths["receipt"].exists() is not received
                or received and record["prompt_send_completed"] is not True
            ):
                raise RuntimeError(f"original-shaped fake lost {scenario} consumption evidence")
            results[f"original-{scenario}"] = {
                "failure_class": record["failure_class"],
                "prompt_slot_fsynced_before_send": record["prompt_slot_fsynced_before_send"],
                "prompt_send_completed": record["prompt_send_completed"],
                "fake_prompt_received": paths["receipt"].exists(),
            }

        bounded, bounded_paths = run_fake_original(
            scratch / "original-bounded-cleanup", "hang-after-permission", plan,
            authority, authority_sha256, outer_seconds=0.8, cleanup_seconds=0.25,
        )
        if (
            bounded["failure_class"] != "timeout"
            or bounded["elapsed_milliseconds"] > 800
            or bounded["cleanup"]["root_exit_observed"] is not True
            or bounded["cleanup"]["streams_joined"] is not True
        ):
            raise RuntimeError("original-shaped fake exceeded bounded cleanup or left the owned root unobserved")
        results["bounded_cleanup"] = {
            "failure_class": bounded["failure_class"],
            "elapsed_milliseconds": bounded["elapsed_milliseconds"],
            "root_exit_observed": bounded["cleanup"]["root_exit_observed"],
            "streams_joined": bounded["cleanup"]["streams_joined"],
            "descendant_cleanup": bounded["descendant_cleanup"],
        }

        tamper_rejections: dict[str, bool] = {}
        tampered_action = dict(success)
        tampered_action["permission_action_matches_sentinel"] = False
        tampered_extra = dict(success)
        tampered_extra["unreviewed_diagnostic"] = FAKE_SECRET
        for name, tampered in (("action", tampered_action), ("field", tampered_extra)):
            try:
                validate_execution_record(tampered, plan)
            except ValueError:
                tamper_rejections[name] = True
            else:
                tamper_rejections[name] = False
        if tamper_rejections != {"action": True, "field": True}:
            raise RuntimeError("execution record validator accepted a false proof or an extra raw field")
        results["tamper_rejections"] = tamper_rejections

        expectations = {
            "spontaneous-effect": "unapproved-effect-observed",
            "no-permission": "permission-request-missing",
            "wrong-action": "permission-action-mismatch",
            "duplicate-permission": "duplicate-permission-request",
            "malformed-frame": "malformed-frame",
            "oversized-frame": "inbound-frame-oversized",
            "early-eof": "unknown-eof",
            "initialize-error": "initialize-rpc-error",
            "wrong-version": "agent-version-mismatch",
            "session-new-error": "session-new-rpc-error",
            "tool-call-without-permission": "tool-call-without-host-permission",
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
            if scenario in {"crash-before-prompt", "crash-after-prompt"}:
                prompt_record_exists = (paths["records"] / "1.0.93-prompt-1.json").is_file()
                received_marker = (paths["state"] / "prompt-received").exists()
                if not prompt_record_exists or received_marker != (scenario == "crash-after-prompt"):
                    raise RuntimeError(f"fake {scenario} lost before/after-send consumption evidence")
            if scenario == "wrong-version" and (
                record["initialize_observed"] is not False
                or record["session_observed"] is not False
                or record["prompt_send_attempted"] is not False
            ):
                raise RuntimeError("wrong-version fake reached a session or prompt before identity rejection")
            saved = (paths["records"] / "fake-execution.json").read_bytes()
            if any(marker.encode() in saved for marker in FORBIDDEN_PERSISTED_MARKERS):
                raise RuntimeError(f"fake {scenario} persisted a forbidden marker")
            results[scenario] = {
                "failure_class": record["failure_class"],
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
            write_attempt_ledger(disabled_path, plan, "original")
        except RuntimeError:
            original_refused = True
        if not original_refused or disabled_path.exists():
            raise RuntimeError("disabled plan wrote an original attempt record")
        results["prompt_slot_bytes"] = len(prompt_bytes)
        results["replay_refused"] = refused
        results["prompt_replay_refused"] = prompt_refused
        results["original_entrypoint_refused"] = original_refused
    results["status"] = "passed"
    results["execution_authorized"] = plan["original_execution_enabled"]
    results["separate_execution_authority_validated"] = True
    results["execution_authority_sha256"] = authority_sha256
    results["runner_sha256"] = sha256_file(SCRIPT_PATH)
    results["preparation_plan_sha256"] = sha256_file(PLAN_PATH)
    results["originals_run"] = False
    results["qualification_changed"] = False
    return results


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    actions = parser.add_mutually_exclusive_group(required=True)
    actions.add_argument("--validate-plan", action="store_true")
    actions.add_argument("--validate-authority", action="store_true")
    actions.add_argument("--self-test", action="store_true")
    actions.add_argument("--validate-record", metavar="PATH")
    actions.add_argument("--validate-original-record", metavar="PATH")
    actions.add_argument("--execute-original", action="store_true")
    args = parser.parse_args()
    try:
        if args.validate_plan:
            plan = validate_plan()
            print(json.dumps({"status": "valid", "plan_id": plan["plan_id"], "original_execution_enabled": False}))
        elif args.validate_authority:
            plan = validate_plan()
            authority, authority_sha256 = validate_execution_authority(plan=plan)
            print(json.dumps({
                "status": "valid",
                "execution_authorized": authority["execution_authorized"],
                "authority_sha256": authority_sha256,
                "final_runner_sha256": authority["final_runner_sha256"],
                "preparation_plan_sha256": authority["preparation_plan_sha256"],
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
            ), sort_keys=True))
        elif args.execute_original:
            record = execute_original()
            print(json.dumps(record, sort_keys=True))
        else:
            print(json.dumps(self_test(), sort_keys=True))
        return 0
    except (OSError, ValueError, RuntimeError, ProtocolFailure, subprocess.SubprocessError) as error:
        detail = str(error) if isinstance(error, (ValueError, RuntimeError, ProtocolFailure)) else type(error).__name__
        print(f"copilot host permission proof failed: {detail}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] in {"--fake-agent", "--fake-child"}:
        raise SystemExit(fake_agent_main())
    raise SystemExit(main())
