#!/usr/bin/env python3
"""Run Copilot ACP proof fixtures inside a network-denied macOS sandbox.

The fake proof validates containment and record ordering. Exact artifact runs
are separately gated by a durable, secret-free record created by --prepare.
"""

from __future__ import annotations

import argparse
import hashlib
import ipaddress
import json
import os
import platform
import re
import select
import shutil
import socket
import socketserver
import ssl
import selectors
import signal
import subprocess
import sys
import tempfile
import threading
import time
import uuid
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[1]
FIXTURE_DIR = (
    ROOT
    / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof"
)
INVENTORY_PATH = FIXTURE_DIR / "artifact-inventory.json"
AUTHENTICATED_PLAN_PATH = FIXTURE_DIR / "authenticated-proof-plan.json"
AUTHENTICATED_SCHEMA_PATH = FIXTURE_DIR / "authenticated-proof-plan.schema.json"
PERMISSION_PROOF_PLAN_PATH = FIXTURE_DIR / "permission-proof-plan.json"
PERMISSION_CORRECTION_PLAN_PATH = FIXTURE_DIR / "permission-proof-correction-plan.json"
PERMISSION_CORRECTION_SCHEMA_PATH = (
    FIXTURE_DIR / "permission-proof-correction-plan.schema.json"
)
PERMISSION_PREFLIGHT_SCHEMA_PATH = (
    FIXTURE_DIR / "permission-proof-preflight-record.schema.json"
)
PERMISSION_EXECUTION_V2_SCHEMA_PATH = (
    FIXTURE_DIR / "permission-proof-execution-record-v2.schema.json"
)
PERMISSION_FAKE_SOURCE_PATH = ROOT / "scripts/copilot-acp-permission-fake.c"
COMMITTED_PERMISSION_RECORD_PATH = (
    FIXTURE_DIR / "permission-proof-execution-record.json"
)
COMMITTED_PERMISSION_RECORD_SHA256 = (
    "ed9c866a90c96374422bd5aad3a900a4a4ca11517159aa14f4e29eec2cd2e049"
)
HISTORICAL_PERMISSION_PREFLIGHT_SHA256 = (
    "1e7552f20e34dbf797f56bc1a00cd4cf2dd11d3f41105192acfc0e5f53f59a0f"
)
PRIOR_PERMISSION_PROOF_HARNESS_SHA256 = (
    "fb99eb2f1d2c60260829160fbfa6f103ad279a797417a9281872bb5dcfe20ba3"
)
RENEWAL_DECISION = "411be8ce-77a0-4a50-930f-d6aeacdffce9"
RENEWAL_VERSION = "1.0.93"
RENEWAL_HISTORICAL_HARNESS_SHA256 = (
    "f11f44d43e8baf432ed3627cd75c9630d65f0fd8fb95535cc1437fd1dbc40418"
)
RENEWAL_BINARY_SHA256 = (
    "df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1"
)
RENEWAL_ACCOUNT_REF = "github-account:betterthanclay"
RENEWAL_AUTHORITY_SCHEMA = "copilot-cli-acp-permission-renewal-authority.v1"
RENEWAL_ATTEMPT_SCHEMA = "copilot-cli-acp-permission-renewal-attempt.v1"
RENEWAL_RECORD_SCHEMA = "copilot-cli-acp-permission-renewal-execution.v1"
RENEWAL_AUTHORITY_PATH = FIXTURE_DIR / "permission-proof-renewal-authority.json"
RENEWAL_ATTEMPT_PATH = FIXTURE_DIR / "permission-proof-renewal-attempt.json"
RENEWAL_RECORD_PATH = FIXTURE_DIR / "permission-proof-renewal-execution-record.json"
RENEWAL_BUDGETS = {
    "invocations": 1,
    "max_prompts": 1,
    "shared_prompt_maximum": 3,
    "historical_prompts": 0,
    "max_seconds": 60,
    "harness_retries": 0,
    "harness_resends": 0,
    "harness_model_fallbacks": 0,
    "max_tool_effects": 0,
    "older_version_starts": 0,
}
RENEWAL_AUTHORITY_FIELDS = frozenset(
    {
        "schema",
        "operator_decision",
        "operator_ruling",
        "operation_id",
        "version",
        "binary_sha256",
        "account_access_ref",
        "permission_plan_sha256",
        "correction_plan_sha256",
        "artifact_inventory_sha256",
        "harness_sha256",
        "prior_consumed_record_sha256",
        "start_order",
        "budgets",
        "profile_basis",
        "qualification_changed",
    }
)
VERSIONS = ("1.0.80", "1.0.81", "1.0.93")
MAX_FRAME_BYTES = 256 * 1024
READ_TIMEOUT_SECONDS = 20.0
RUN_TIMEOUT_SECONDS = 18.0
FAKE_TOKEN = "SWALLOWTAIL_OFFLINE_FAKE_TOKEN"
FAKE_DELEGATED_CREDENTIAL = b"test-only-delegated-capability"
FAKE_ACCOUNT_REF = "github-account:betterthanclay"
FAKE_MODEL_ID = "fake-github-hosted-model"
FAKE_AUDIENCE = "copilot-api.fake.test"
MAX_AUTH_FAKE_SECONDS = 5.0
DISCOVERY_REQUEST_METHODS = frozenset({"initialize", "authenticate", "session/new"})
DISCOVERY_METADATA_UPDATES = frozenset(
    {"available_commands_update", "config_option_update", "current_mode_update"}
)
OFFICIAL_COPILOT_HOSTS = frozenset(
    {
        "github.com",
        "github.githubassets.com",
        "avatars.githubusercontent.com",
        "api.github.com",
        "default.exp-tas.com",
        "copilot-proxy.githubusercontent.com",
        "origin-tracker.githubusercontent.com",
    }
)
OFFICIAL_COPILOT_SUFFIXES = ("githubcopilot.com",)
LIVE_PERMISSION_SECONDS = 60.0
LIVE_CLEANUP_SECONDS = 3.0
MAX_STDERR_CAPTURE_BYTES = 4096
MAX_STDERR_COUNTED_BYTES = 65536
STDERR_CATEGORIES = frozenset(
    {
        "empty",
        "sandbox-denial",
        "access-denied",
        "dynamic-loader-failure",
        "launcher-failure",
        "unknown",
    }
)
ORIGINAL_PROCESS_CONTAINMENT_PROVEN = False
ORIGINAL_PROCESS_CONTAINMENT_STOP = (
    "escaping descendants cannot be race-safely enumerated and joined by this harness"
)


def require_original_process_containment() -> None:
    if not ORIGINAL_PROCESS_CONTAINMENT_PROVEN:
        raise RuntimeError(
            "Copilot original-artifact admission is blocked: "
            f"{ORIGINAL_PROCESS_CONTAINMENT_STOP}"
        )


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds").replace(
        "+00:00", "Z"
    )


def json_bytes(value: Any) -> bytes:
    return (json.dumps(value, separators=(",", ":"), ensure_ascii=False) + "\n").encode()


def json_digest(value: Any) -> str:
    canonical = json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    return hashlib.sha256(canonical.encode()).hexdigest()


def write_json_durable(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temp_path = path.with_name(f".{path.name}.{os.getpid()}.tmp")
    with temp_path.open("wb") as output:
        output.write(json.dumps(value, indent=2, sort_keys=True).encode() + b"\n")
        output.flush()
        os.fsync(output.fileno())
    os.replace(temp_path, path)
    directory_fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(directory_fd)
    finally:
        os.close(directory_fd)


class BoundedStderrCollector:
    """Drain stderr concurrently while retaining only a small in-memory prefix."""

    def __init__(self, stream: Any, limit: int = MAX_STDERR_CAPTURE_BYTES) -> None:
        self._stream = stream
        self._limit = limit
        self._captured = bytearray()
        self._total_bytes = 0
        self._total_bytes_capped = False
        self._read_failed = False
        self._thread = threading.Thread(target=self._drain, daemon=True)
        self._thread.start()

    def _drain(self) -> None:
        try:
            while True:
                chunk = os.read(self._stream.fileno(), 16384)
                if not chunk:
                    return
                counted = min(
                    len(chunk), MAX_STDERR_COUNTED_BYTES - self._total_bytes
                )
                self._total_bytes += counted
                if counted < len(chunk):
                    self._total_bytes_capped = True
                available = self._limit - len(self._captured)
                if available > 0:
                    self._captured.extend(chunk[:available])
        except OSError:
            self._read_failed = True

    def join(self, timeout: float) -> bool:
        self._thread.join(max(0.0, timeout))
        return not self._thread.is_alive()

    def summary(self, *, reader_joined: bool) -> dict[str, Any]:
        content = bytes(self._captured).lower()
        if not content:
            category = "empty"
        elif b"operation not permitted" in content or b"sandbox" in content:
            category = "sandbox-denial"
        elif b"permission denied" in content or b"access denied" in content:
            category = "access-denied"
        elif any(
            marker in content
            for marker in (b"dyld", b"library not loaded", b"image not found")
        ):
            category = "dynamic-loader-failure"
        elif any(
            marker in content
            for marker in (b"exec format error", b"no such file or directory")
        ):
            category = "launcher-failure"
        else:
            category = "unknown"
        if category not in STDERR_CATEGORIES:
            category = "unknown"
        return {
            "classification": category,
            "captured_bytes": len(self._captured),
            "total_bytes": self._total_bytes,
            "total_bytes_capped": self._total_bytes_capped,
            "total_byte_count_limit": MAX_STDERR_COUNTED_BYTES,
            "capture_limit_bytes": self._limit,
            "truncated": self._total_bytes_capped
            or self._total_bytes > len(self._captured),
            "reader_joined": reader_joined,
            "reader_error": self._read_failed,
            "raw_persisted": False,
            "raw_displayed": False,
        }


def load_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text())
    if not isinstance(value, dict):
        raise ValueError(f"expected an object in {path.name}")
    return value


def compile_permission_fake(output_path: Path) -> dict[str, str]:
    output_path = output_path.resolve(strict=False)
    temp_root = Path(tempfile.gettempdir()).resolve()
    if not output_path.parent.resolve(strict=True).is_relative_to(temp_root):
        raise RuntimeError("native permission fake must compile inside fresh task temp scratch")
    compiler = shutil.which("cc")
    if compiler is None:
        raise RuntimeError("native permission fake compiler is unavailable")
    environment = os.environ.copy()
    environment["HOME"] = str(output_path.parent)
    environment["TMPDIR"] = str(output_path.parent)
    try:
        result = subprocess.run(
            [
                compiler,
                "-std=c11",
                "-Wall",
                "-Wextra",
                "-Werror",
                "-O2",
                str(PERMISSION_FAKE_SOURCE_PATH),
                "-o",
                str(output_path),
            ],
            cwd=ROOT,
            env=environment,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=30,
            check=False,
        )
    except subprocess.TimeoutExpired:
        raise RuntimeError("native permission fake compilation exceeded 30 seconds") from None
    if result.returncode != 0 or not output_path.is_file():
        compiler_messages = result.stderr.decode("utf-8", errors="replace").splitlines()
        safe_messages = [
            line.split(": error: ", 1)[1][:160]
            for line in compiler_messages
            if ": error: " in line
        ][:4]
        detail = "; ".join(safe_messages) if safe_messages else "compiler rejected the source"
        raise RuntimeError(
            f"native permission fake compilation failed (exit={result.returncode}; {detail})"
        )
    output_path.chmod(0o700)
    return {
        "source_sha256": hashlib.sha256(PERMISSION_FAKE_SOURCE_PATH.read_bytes()).hexdigest(),
        "binary_sha256": hashlib.sha256(output_path.read_bytes()).hexdigest(),
    }


def validate_schema_value(value: Any, schema: dict[str, Any], label: str = "$") -> None:
    """Validate the small JSON Schema subset used by the committed plan."""
    supported = {
        "$schema",
        "$id",
        "title",
        "description",
        "type",
        "const",
        "enum",
        "required",
        "properties",
        "items",
        "additionalProperties",
        "minItems",
        "maxItems",
        "minLength",
        "maxLength",
        "pattern",
    }
    unknown = set(schema) - supported
    if unknown:
        raise ValueError(f"unsupported plan schema keywords at {label}: {sorted(unknown)}")

    expected_type = schema.get("type")
    if expected_type is not None:
        matches = {
            "object": isinstance(value, dict),
            "array": isinstance(value, list),
            "string": isinstance(value, str),
            "integer": isinstance(value, int) and not isinstance(value, bool),
            "boolean": isinstance(value, bool),
            "null": value is None,
        }.get(expected_type)
        if matches is not True:
            raise ValueError(f"{label} must be {expected_type}")
    if "const" in schema and value != schema["const"]:
        raise ValueError(f"{label} does not match its required constant")
    if "enum" in schema and value not in schema["enum"]:
        raise ValueError(f"{label} is outside its allowed values")

    if isinstance(value, dict):
        required = schema.get("required", [])
        missing = set(required) - set(value)
        if missing:
            raise ValueError(f"{label} is missing required fields: {sorted(missing)}")
        properties = schema.get("properties", {})
        extra = set(value) - set(properties)
        if schema.get("additionalProperties") is False and extra:
            raise ValueError(f"{label} has unexpected fields: {sorted(extra)}")
        for key, child_schema in properties.items():
            if key in value:
                validate_schema_value(value[key], child_schema, f"{label}.{key}")
    elif isinstance(value, list):
        if len(value) < schema.get("minItems", 0):
            raise ValueError(f"{label} has fewer items than required")
        if len(value) > schema.get("maxItems", sys.maxsize):
            raise ValueError(f"{label} has more items than allowed")
        if "items" in schema:
            for index, item in enumerate(value):
                validate_schema_value(item, schema["items"], f"{label}[{index}]")
    elif isinstance(value, str):
        if len(value) < schema.get("minLength", 0):
            raise ValueError(f"{label} is shorter than required")
        if len(value) > schema.get("maxLength", sys.maxsize):
            raise ValueError(f"{label} is longer than allowed")
        pattern = schema.get("pattern")
        if pattern is not None and re.search(pattern, value) is None:
            raise ValueError(f"{label} does not match its required pattern")


def validate_authenticated_plan() -> dict[str, Any]:
    schema = load_json(AUTHENTICATED_SCHEMA_PATH)
    plan = load_json(AUTHENTICATED_PLAN_PATH)
    if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        raise ValueError("authenticated plan schema is not Draft 2020-12")
    validate_schema_value(plan, schema)
    serialized = json.dumps(plan, sort_keys=True)
    for forbidden in (
        "/Users/",
        "/home/",
        "github_pat_",
        "ghp_",
        "gho_",
        "ghs_",
        "xoxb-",
        "Bearer ",
    ):
        if forbidden in serialized:
            raise ValueError(f"authenticated plan contains forbidden secret/path marker {forbidden!r}")
    if plan.get("execution_authorized") is not False:
        raise ValueError("unattested plan must remain non-executable")
    return plan


def inventory_packages(inventory: dict[str, Any]) -> dict[tuple[str, str], dict[str, Any]]:
    result: dict[tuple[str, str], dict[str, Any]] = {}
    for package in inventory.get("packages", []):
        name = package["name"]
        for version in package.get("versions", []):
            result[(name, version["version"])] = version
    return result


def verify_inventory(path: Path = INVENTORY_PATH) -> dict[str, Any]:
    inventory = load_json(path)
    if inventory.get("schema") != "copilot-cli-acp-offline-artifact-inventory.v1":
        raise ValueError("unexpected Copilot artifact inventory schema")
    if inventory.get("platform") != "darwin-arm64":
        raise ValueError("inventory is not for the selected darwin-arm64 host")
    packages = inventory_packages(inventory)
    for name in ("@github/copilot", "@github/copilot-darwin-arm64"):
        for version in VERSIONS:
            record = packages.get((name, version))
            if record is None:
                raise ValueError(f"inventory is missing {name}@{version}")
            files = record.get("files")
            if not isinstance(files, list) or len(files) != record.get("dist", {}).get("fileCount"):
                raise ValueError(f"inventory file count mismatch for {name}@{version}")
            paths = [item.get("path") for item in files]
            if paths != sorted(paths) or len(paths) != len(set(paths)):
                raise ValueError(f"inventory paths are not a sorted unique set for {name}@{version}")
            manifest = "".join(
                f"{item['path']}\0{item['kind']}\0{item['size']}\0{item['mode']}\0{item['sha256']}\n"
                for item in files
            ).encode()
            if hashlib.sha256(manifest).hexdigest() != record.get("inventory_sha256"):
                raise ValueError(f"inventory digest mismatch for {name}@{version}")
            if sum(item["size"] for item in files) != record.get("dist", {}).get("unpackedSize"):
                raise ValueError(f"inventory byte count mismatch for {name}@{version}")
            if any(
                not item.get("path", "").startswith("package/")
                or item.get("kind") != "file"
                or len(item.get("sha256", "")) != 64
                for item in files
            ):
                raise ValueError(f"invalid file entry for {name}@{version}")
    tags = inventory.get("stable_channel_cross_check", {})
    if not (
        tags.get("npm_latest")
        == tags.get("native_latest")
        == tags.get("github_latest_release", {}).get("tag", "").removeprefix("v")
        == "1.0.93"
    ):
        raise ValueError("the frozen official stable channels do not agree on 1.0.93")
    if tags.get("npm_prerelease") == "1.0.93":
        raise ValueError("the prerelease dist-tag was confused with stable")
    return inventory


def quote_profile_path(path: Path) -> str:
    return json.dumps(str(path.resolve()))


def quote_literal_policy_path(path: Path) -> str:
    # Keep a declared exception lexical so resolving it cannot follow a real
    # auth-file symlink during fake-only preparation.
    return json.dumps(os.path.normpath(os.path.abspath(str(path))))


def sandbox_profile(
    scratch: Path, host_home: Path, repository_root: Path = ROOT
) -> str:
    scratch = scratch.resolve()
    host_home = host_home.resolve()
    repository_root = repository_root.resolve()
    isolated_home_helper = (ROOT / "scripts/run-with-isolated-home.sh").resolve()
    return " ".join(
        (
            "(version 1)",
            "(deny default)",
            "(allow process*)",
            "(allow sysctl-read)",
            "(allow mach-lookup)",
            "(deny mach-lookup (global-name \"com.apple.securityd\"))",
            "(allow file-read*)",
            f"(deny file-read* (subpath {quote_profile_path(host_home)}))",
            f"(deny file-read* (subpath {quote_profile_path(repository_root)}))",
            f"(allow file-read* (literal {quote_profile_path(isolated_home_helper)}))",
            f"(allow file-write* (subpath {quote_profile_path(scratch)}))",
            "(deny network*)",
        )
    )


def permission_sandbox_profile(
    scratch: Path,
    record_dir: Path,
    host_home: Path,
    repository_root: Path,
    executable: Path,
    proxy_port: int,
    permission_plan: dict[str, Any],
    correction_plan: dict[str, Any],
) -> str:
    if correction_plan.get("source_permission_plan_sha256") != hashlib.sha256(
        PERMISSION_PROOF_PLAN_PATH.read_bytes()
    ).hexdigest():
        raise ValueError("permission sandbox correction is not bound to the approved plan")
    if permission_plan.get("network_policy", {}).get("allowed_hosts") is None:
        raise ValueError("permission sandbox lacks the approved network plan")
    if proxy_port <= 0 or proxy_port > 65535:
        raise ValueError("permission sandbox proxy port is invalid")
    metadata_paths = correction_plan.get("auth_metadata_read_paths", [])
    if [item.get("relative_path") for item in metadata_paths] != [
        ".copilot/config.json"
    ]:
        raise ValueError("permission sandbox auth metadata exceptions differ from the correction plan")
    scratch = scratch.resolve()
    record_dir = record_dir.resolve()
    host_home = host_home.resolve()
    repository_root = repository_root.resolve()
    executable = executable.resolve(strict=True)
    auth_metadata_path = host_home / metadata_paths[0]["relative_path"]
    return " ".join(
        (
            "(version 1)",
            '(import "/System/Library/Sandbox/Profiles/dyld-support.sb")',
            "(deny default)",
            "(allow process-fork)",
            f"(allow process-exec* (literal {quote_profile_path(executable)}))",
            f"(allow file-map-executable (literal {quote_profile_path(executable)}))",
            '(allow file-map-executable (subpath "/System/Library"))',
            '(allow file-map-executable (subpath "/usr/lib"))',
            "(allow sysctl-read)",
            '(allow mach-lookup (global-name "com.apple.securityd"))',
            "(allow file-read*)",
            f"(deny file-read* (require-all (subpath {quote_profile_path(host_home)}) (require-not (literal {quote_literal_policy_path(auth_metadata_path)}))))",
            f"(deny file-read* (subpath {quote_profile_path(repository_root)}))",
            f"(deny file-read* (subpath {quote_profile_path(record_dir)}))",
            f"(deny file-write* (subpath {quote_profile_path(host_home)}))",
            f"(deny file-write* (subpath {quote_profile_path(repository_root)}))",
            f"(allow file-write* (subpath {quote_profile_path(scratch)}))",
            "(deny network-inbound)",
            "(deny network-bind)",
            f'(allow network-outbound (remote ip "localhost:{proxy_port}"))',
        )
    )


def path_is_within(path: Path, root: Path) -> bool:
    try:
        return path.resolve(strict=False).is_relative_to(root.resolve(strict=False))
    except (OSError, RuntimeError):
        return False


def discovery_request_allowed(method: str) -> bool:
    return method in DISCOVERY_REQUEST_METHODS


def discovery_callback_allowed(method: str, update_type: str | None = None) -> bool:
    return method == "session/update" and update_type in DISCOVERY_METADATA_UPDATES


def discovery_endpoint_allowed(origin: str, allowed_origins: set[str]) -> bool:
    return origin in allowed_origins


def discovery_budget_allowed(invocation_number: int, elapsed_seconds: float) -> bool:
    return invocation_number == 1 and 0 <= elapsed_seconds <= 60.0


def discovery_read_allowed(path: Path, host_home: Path, repository_root: Path) -> bool:
    return not path_is_within(path, host_home) and not path_is_within(
        path, repository_root
    )


def discovery_write_allowed(path: Path, scratch: Path) -> bool:
    return path_is_within(path, scratch)


def validate_permission_proof_plan() -> dict[str, Any]:
    plan = load_json(PERMISSION_PROOF_PLAN_PATH)
    if plan.get("schema") != "copilot-cli-acp-permission-proof-plan.v1":
        raise ValueError("unexpected original permission proof plan schema")
    if plan.get("operator_decision") != "8b098b85-fb9e-4543-b5f1-1fbfe8bc5301":
        raise ValueError("original permission proof does not bind the approved operator ruling")
    if plan.get("execution_authorized") is not True:
        raise ValueError("original permission proof lacks the approved execution boundary")
    if plan.get("scope") != {
        "route": "copilot-cli.acp",
        "platform": "darwin-arm64",
        "versions": ["1.0.80", "1.0.81", "1.0.93"],
        "inventory_sha256": "2d122117ccbb52dd547a783117ea3b1699df8e15357bca86a4d27a65416c8b0f",
    }:
        raise ValueError("original permission proof scope differs from the frozen artifact inventory")
    if plan.get("selection_policy", {}).get("requested") != "Auto" or plan.get(
        "selection_policy", {}
    ).get("binding") != "--model auto on the original process":
        raise ValueError("original permission proof does not bind the requested Auto selection")
    if plan.get("credential_boundary", {}).get("item_level_mediation") != (
        "unavailable; no per-item filter claimed"
    ):
        raise ValueError("original permission proof misstates the approved Keychain boundary")
    if plan.get("containment", {}).get("network") != (
        "local CONNECT proxy only; verify official host and matching TLS SNI before external dial"
    ):
        raise ValueError("original permission proof lacks its verified egress gate")
    if plan.get("start_order") != ["1.0.93", "1.0.81", "1.0.80"]:
        raise ValueError("original permission proof must start with 1.0.93")
    network = plan.get("network_policy", {})
    if set(network.get("allowed_hosts", [])) != OFFICIAL_COPILOT_HOSTS:
        raise ValueError("network allowlist differs from the reviewed official host set")
    if tuple(network.get("allowed_subdomains", [])) != OFFICIAL_COPILOT_SUFFIXES:
        raise ValueError("network wildcard differs from the reviewed official host set")
    budgets = plan.get("budgets", {})
    if budgets != {
        "max_invocations_per_version": 1,
        "max_total_invocations": 3,
        "max_prompts_per_invocation": 1,
        "max_seconds_per_invocation": 60,
        "max_harness_retries": 0,
        "max_harness_resends": 0,
        "max_harness_model_fallbacks": 0,
        "vendor_managed_retries": "allowed_within_single_prompt_and_deadline; count_unobserved",
        "max_tool_effects": 0,
        "reviewer_original_attempts": 0,
    }:
        raise ValueError("original permission proof budgets differ from the approved boundary")
    serialized = json.dumps(plan, sort_keys=True)
    for forbidden in (
        "/Users/",
        "/home/",
        "github_pat_",
        "ghp_",
        "gho_",
        "ghs_",
        "xoxb-",
        "Bearer ",
    ):
        if forbidden in serialized:
            raise ValueError(f"original permission plan contains forbidden marker {forbidden!r}")
    return plan


def validate_permission_correction_plan(
    permission_plan: dict[str, Any] | None = None,
) -> dict[str, Any]:
    approved_plan = permission_plan or validate_permission_proof_plan()
    correction = load_json(PERMISSION_CORRECTION_PLAN_PATH)
    schema = load_json(PERMISSION_CORRECTION_SCHEMA_PATH)
    validate_schema_value(correction, schema, "permission_proof_correction")
    if correction.get("source_permission_plan_sha256") != hashlib.sha256(
        PERMISSION_PROOF_PLAN_PATH.read_bytes()
    ).hexdigest():
        raise ValueError("permission correction plan does not bind the approved original plan")
    if correction.get("operator_decision") != approved_plan.get("operator_decision"):
        raise ValueError("permission correction plan changed the approved operator ruling")
    if correction.get("auth_metadata_read_paths") != [
        {
            "relative_path": ".copilot/config.json",
            "source": "https://docs.github.com/en/copilot/reference/copilot-cli-reference/cli-config-dir-reference",
            "purpose": "official CLI configuration directory reference identifies config.json as automatically managed application state including authentication",
            "version_scope": "current official documentation; not frozen-artifact-specific",
        }
    ]:
        raise ValueError("permission correction auth metadata path differs from official-source evidence")
    if correction.get("sandbox_delta", {}).get("fake_runtime_roots") != []:
        raise ValueError("permission correction must not add fake-only runtime roots")
    serialized = json.dumps(correction, sort_keys=True)
    for forbidden in ("/Users/", "/home/", "github_pat_", "gho_", "Bearer "):
        if forbidden in serialized:
            raise ValueError("permission correction plan contains a forbidden path or secret marker")
    return correction


def consumed_permission_versions(
    record_path: Path = COMMITTED_PERMISSION_RECORD_PATH,
    expected_sha256: str = COMMITTED_PERMISSION_RECORD_SHA256,
) -> set[str]:
    if record_path.is_symlink() or not record_path.is_file():
        raise RuntimeError(
            "committed original permission record is missing or unsafe; refusing original starts"
        )
    record_bytes = record_path.read_bytes()
    if hashlib.sha256(record_bytes).hexdigest() != expected_sha256:
        raise RuntimeError(
            "committed original permission record changed; refusing original starts"
        )
    validate_permission_execution_record(record_path)
    if hashlib.sha256(record_path.read_bytes()).hexdigest() != expected_sha256:
        raise RuntimeError(
            "committed original permission record changed during validation; refusing original starts"
        )
    record = json.loads(record_bytes)
    return {
        item["version"]
        for item in record["invocations"]
        if item.get("invocation_consumed") is True
    }


def refuse_consumed_permission_versions(
    plan: dict[str, Any], consumed_versions: set[str]
) -> None:
    for version in plan["start_order"]:
        if version in consumed_versions:
            raise RuntimeError(
                f"refusing original {version}: committed permission record shows "
                "its invocation is consumed; separate authority is required"
            )


def enforce_permission_invocation_budget(plan: dict[str, Any]) -> set[str]:
    consumed_versions = consumed_permission_versions()
    refuse_consumed_permission_versions(plan, consumed_versions)
    return consumed_versions


def host_allowed(host: str, exact_hosts: set[str], allowed_subdomains: tuple[str, ...]) -> bool:
    normalized = host.rstrip(".").lower()
    if normalized in exact_hosts:
        return True
    return any(normalized.endswith(f".{suffix}") for suffix in allowed_subdomains)


def parse_connect_request(request: bytes) -> tuple[str, int] | None:
    try:
        first_line = request.split(b"\r\n", 1)[0].decode("ascii")
        method, authority, protocol = first_line.split(" ")
        if method != "CONNECT" or protocol not in {"HTTP/1.0", "HTTP/1.1"}:
            return None
        host, separator, port_text = authority.rpartition(":")
        if not separator or not host or not port_text.isdecimal():
            return None
        if not host.isascii() or any(character in host for character in "/@[]"):
            return None
        port = int(port_text)
        if port != 443:
            return None
        return host.rstrip(".").lower(), port
    except (UnicodeDecodeError, ValueError):
        return None


def read_http_headers(connection: socket.socket) -> tuple[bytes, bytes]:
    data = bytearray()
    while b"\r\n\r\n" not in data:
        chunk = connection.recv(4096)
        if not chunk:
            raise ConnectionError("proxy client closed before CONNECT headers")
        data.extend(chunk)
        if len(data) > 16 * 1024:
            raise ValueError("proxy CONNECT headers exceeded the proof limit")
    end = data.index(b"\r\n\r\n") + 4
    return bytes(data[:end]), bytes(data[end:])


def read_client_hello(connection: socket.socket, prefix: bytes = b"") -> tuple[bytes, str | None]:
    data = bytearray(prefix)
    while len(data) < 5:
        chunk = connection.recv(4096)
        if not chunk:
            raise ConnectionError("proxy client closed before TLS ClientHello")
        data.extend(chunk)
    if data[0] != 0x16:
        raise ValueError("proxy tunnel did not start with a TLS handshake")
    record_length = int.from_bytes(data[3:5], "big")
    if record_length < 4 or record_length > 65535:
        raise ValueError("proxy TLS record length is outside the proof limit")
    total_length = 5 + record_length
    while len(data) < total_length:
        chunk = connection.recv(min(4096, total_length - len(data)))
        if not chunk:
            raise ConnectionError("proxy client closed during TLS ClientHello")
        data.extend(chunk)
    record = bytes(data[:total_length])
    payload = record[5:]
    if payload[0] != 0x01:
        raise ValueError("proxy TLS handshake did not begin with ClientHello")
    hello_length = int.from_bytes(payload[1:4], "big")
    hello = payload[4 : 4 + hello_length]
    if len(hello) != hello_length or hello_length < 38:
        raise ValueError("proxy TLS ClientHello is incomplete")
    offset = 34
    session_length = hello[offset]
    offset += 1 + session_length
    if offset + 2 > len(hello):
        raise ValueError("proxy TLS ClientHello has no cipher list")
    cipher_length = int.from_bytes(hello[offset : offset + 2], "big")
    offset += 2 + cipher_length
    if offset >= len(hello):
        raise ValueError("proxy TLS ClientHello has no compression list")
    compression_length = hello[offset]
    offset += 1 + compression_length
    if offset + 2 > len(hello):
        return record, None
    extensions_length = int.from_bytes(hello[offset : offset + 2], "big")
    offset += 2
    extension_end = min(len(hello), offset + extensions_length)
    while offset + 4 <= extension_end:
        extension_type = int.from_bytes(hello[offset : offset + 2], "big")
        extension_length = int.from_bytes(hello[offset + 2 : offset + 4], "big")
        offset += 4
        extension = hello[offset : offset + extension_length]
        offset += extension_length
        if extension_type != 0x0000 or len(extension) < 5:
            continue
        names_length = int.from_bytes(extension[:2], "big")
        name_offset = 2
        names_end = min(len(extension), 2 + names_length)
        while name_offset + 3 <= names_end:
            name_type = extension[name_offset]
            name_length = int.from_bytes(
                extension[name_offset + 1 : name_offset + 3], "big"
            )
            name_offset += 3
            name = extension[name_offset : name_offset + name_length]
            name_offset += name_length
            if name_type == 0 and len(name) == name_length:
                try:
                    return record, name.decode("ascii").rstrip(".").lower()
                except UnicodeDecodeError:
                    return record, None
    return record, None


class CopilotEgressProxy(socketserver.ThreadingMixIn, socketserver.TCPServer):
    allow_reuse_address = False
    daemon_threads = True
    block_on_close = False

    def __init__(
        self,
        allowed_hosts: set[str],
        allowed_subdomains: tuple[str, ...],
        deadline: float,
        *,
        fake_only: bool = False,
    ) -> None:
        self.allowed_hosts = allowed_hosts
        self.allowed_subdomains = allowed_subdomains
        self.deadline = deadline
        self.fake_only = fake_only
        self.allowed_destinations: set[str] = set()
        self.unlisted_destination_count = 0
        self.sni_mismatch_count = 0
        self.connection_failure_count = 0
        self._evidence_lock = threading.Lock()
        self._active_sockets: set[socket.socket] = set()
        self._client_threads: set[threading.Thread] = set()
        self._threads_lock = threading.Lock()
        super().__init__(("127.0.0.1", 0), CopilotEgressProxyHandler)
        self._serve_thread = threading.Thread(target=self.serve_forever, daemon=True)

    def start(self) -> int:
        self._serve_thread = threading.Thread(
            target=lambda: self.serve_forever(poll_interval=0.05), daemon=True
        )
        self._serve_thread.start()
        return int(self.server_address[1])

    def attach_socket(self, connection: socket.socket) -> None:
        with self._evidence_lock:
            self._active_sockets.add(connection)

    def detach_socket(self, connection: socket.socket) -> None:
        with self._evidence_lock:
            self._active_sockets.discard(connection)

    def attach_upstream(self, connection: socket.socket) -> None:
        with self._evidence_lock:
            self._active_sockets.add(connection)

    def record_unlisted(self) -> None:
        with self._evidence_lock:
            self.unlisted_destination_count += 1

    def record_sni_mismatch(self) -> None:
        with self._evidence_lock:
            self.sni_mismatch_count += 1

    def record_connection_failure(self) -> None:
        with self._evidence_lock:
            self.connection_failure_count += 1

    def record_allowed(self, host: str) -> None:
        with self._evidence_lock:
            self.allowed_destinations.add(host)

    def snapshot(self) -> dict[str, Any]:
        with self._evidence_lock:
            return {
                "allowed_destination_hosts": sorted(self.allowed_destinations),
                "unlisted_destination_count": self.unlisted_destination_count,
                "sni_mismatch_count": self.sni_mismatch_count,
                "connection_failure_count": self.connection_failure_count,
            }

    def process_request(self, request: socket.socket, client_address: Any) -> None:
        thread = threading.Thread(
            target=self.process_request_thread, args=(request, client_address)
        )
        thread.daemon = True
        with self._threads_lock:
            self._client_threads.add(thread)
        thread.start()

    def process_request_thread(self, request: socket.socket, client_address: Any) -> None:
        self.attach_socket(request)
        try:
            super().process_request_thread(request, client_address)
        finally:
            self.detach_socket(request)
            with self._threads_lock:
                self._client_threads.discard(threading.current_thread())

    def stop_and_join(self, timeout: float) -> bool:
        self.shutdown()
        self.server_close()
        with self._evidence_lock:
            sockets = list(self._active_sockets)
        for connection in sockets:
            try:
                connection.shutdown(socket.SHUT_RDWR)
            except OSError:
                pass
            try:
                connection.close()
            except OSError:
                pass
        deadline = time.monotonic() + timeout
        self._serve_thread.join(max(0.0, deadline - time.monotonic()))
        while time.monotonic() < deadline:
            with self._threads_lock:
                threads = list(self._client_threads)
            if not threads:
                break
            for thread in threads:
                thread.join(min(0.05, max(0.0, deadline - time.monotonic())))
        with self._threads_lock:
            return not self._serve_thread.is_alive() and not self._client_threads


class CopilotEgressProxyHandler(socketserver.BaseRequestHandler):
    def handle(self) -> None:
        server: CopilotEgressProxy = self.server  # type: ignore[assignment]
        connection: socket.socket = self.request
        upstream: socket.socket | None = None
        try:
            connection.settimeout(min(3.0, max(0.1, server.deadline - time.monotonic())))
            request, buffered = read_http_headers(connection)
            target = parse_connect_request(request)
            if target is None:
                server.record_unlisted()
                connection.sendall(b"HTTP/1.1 403 Forbidden\r\nConnection: close\r\n\r\n")
                return
            host, port = target
            if not host_allowed(host, server.allowed_hosts, server.allowed_subdomains):
                server.record_unlisted()
                connection.sendall(b"HTTP/1.1 403 Forbidden\r\nConnection: close\r\n\r\n")
                return
            connection.sendall(
                b"HTTP/1.1 200 Connection Established\r\nProxy-Agent: Swallowtail-proof\r\n\r\n"
            )
            hello, sni = read_client_hello(connection, buffered)
            if sni is None or sni != host:
                server.record_sni_mismatch()
                return
            if not host_allowed(sni, server.allowed_hosts, server.allowed_subdomains):
                server.record_unlisted()
                return
            if server.fake_only:
                server.record_allowed(host)
                return
            upstream = connect_public_host(host, port, server.deadline)
            server.attach_upstream(upstream)
            upstream.sendall(hello)
            server.record_allowed(host)
            relay_until_deadline(connection, upstream, server.deadline)
        except (OSError, ValueError, ConnectionError, TimeoutError):
            server.record_connection_failure()
        finally:
            if upstream is not None:
                server.detach_socket(upstream)
                try:
                    upstream.close()
                except OSError:
                    pass


def connect_public_host(host: str, port: int, deadline: float) -> socket.socket:
    remaining = deadline - time.monotonic()
    if remaining <= 0:
        raise TimeoutError("Copilot egress deadline expired")
    addresses = socket.getaddrinfo(host, port, type=socket.SOCK_STREAM)
    last_error: OSError | None = None
    for family, socktype, proto, _, address in addresses:
        ip = ipaddress.ip_address(address[0].split("%", 1)[0])
        if not ip.is_global:
            continue
        upstream = socket.socket(family, socktype, proto)
        try:
            upstream.settimeout(min(3.0, max(0.1, deadline - time.monotonic())))
            upstream.connect(address)
            return upstream
        except OSError as error:
            last_error = error
            upstream.close()
    raise OSError("no public address was reachable for an approved Copilot host") from last_error


def relay_until_deadline(client: socket.socket, upstream: socket.socket, deadline: float) -> None:
    client.setblocking(False)
    upstream.setblocking(False)
    while time.monotonic() < deadline:
        readable, _, _ = select.select(
            [client, upstream], [], [], min(0.5, deadline - time.monotonic())
        )
        if not readable:
            continue
        for source in readable:
            data = source.recv(64 * 1024)
            if not data:
                return
            target = upstream if source is client else client
            target.setblocking(True)
            target.settimeout(min(1.0, max(0.1, deadline - time.monotonic())))
            target.sendall(data)
            target.setblocking(False)


def clean_child_environment(
    scratch: Path, *, include_fake_token: bool = True
) -> dict[str, str]:
    home = scratch / "home"
    temp = scratch / "tmp"
    config = scratch / "config"
    cache = scratch / "cache"
    for directory in (home, temp, config, cache):
        directory.mkdir(parents=True, exist_ok=True)
    environment = {
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "HOME": str(home),
        "COPILOT_HOME": str(home),
        "GH_COPILOT_HOME": str(home),
        "XDG_CONFIG_HOME": str(config),
        "XDG_CACHE_HOME": str(cache),
        "TMPDIR": str(temp),
        "TERM": "dumb",
    }
    if include_fake_token:
        environment["GITHUB_TOKEN"] = FAKE_TOKEN
        environment["GH_TOKEN"] = FAKE_TOKEN
    return environment


class StdioClient:
    def __init__(self, process: subprocess.Popen[bytes]):
        if process.stdout is None or process.stdin is None:
            raise ValueError("ACP child has no piped stdio")
        self.process = process
        self.stdout = process.stdout
        self.stdin = process.stdin
        self.buffer = bytearray()
        self.selector = selectors.DefaultSelector()
        self.selector.register(self.stdout, selectors.EVENT_READ)

    def send(self, value: dict[str, Any]) -> None:
        self.stdin.write(json_bytes(value))
        self.stdin.flush()

    def receive(self, timeout: float) -> dict[str, Any]:
        deadline = time.monotonic() + timeout
        while True:
            newline = self.buffer.find(b"\n")
            if newline >= 0:
                if newline > MAX_FRAME_BYTES:
                    raise ValueError("ACP frame exceeded the 256 KiB proof limit")
                line = bytes(self.buffer[:newline])
                del self.buffer[: newline + 1]
                value = json.loads(line)
                if not isinstance(value, dict):
                    raise ValueError("ACP frame was not an object")
                return value
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise TimeoutError("ACP response timeout")
            if not self.selector.select(remaining):
                raise TimeoutError("ACP response timeout")
            chunk = os.read(self.stdout.fileno(), 65536)
            if not chunk:
                raise EOFError("ACP child closed stdout")
            self.buffer.extend(chunk)
            if len(self.buffer) > MAX_FRAME_BYTES and b"\n" not in self.buffer:
                raise ValueError("ACP frame exceeded the 256 KiB proof limit")

    def close(self) -> None:
        self.selector.close()


def wait_for_response(
    client: StdioClient,
    request_id: int,
    evidence: dict[str, Any],
    timeout: float,
    *,
    discovery_mode: bool = False,
) -> dict[str, Any] | None:
    deadline = time.monotonic() + timeout
    while True:
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            evidence["timed_out"] = True
            return None
        try:
            message = client.receive(remaining)
        except TimeoutError:
            evidence["timed_out"] = True
            return None
        except (EOFError, ValueError, json.JSONDecodeError):
            evidence["stream_failed"] = True
            return None
        method = message.get("method")
        if method == "session/request_permission":
            if discovery_mode:
                evidence["unexpected_discovery_callback"] = method
                return None
            evidence["permission_request_observed"] = True
            evidence["permission_methods_observed"].append(method)
            params = message.get("params")
            options = params.get("options", []) if isinstance(params, dict) else []
            if not isinstance(options, list):
                options = []
            evidence["permission_option_ids"] = sorted(
                {
                    option["optionId"]
                    for option in options
                    if isinstance(option, dict)
                    and isinstance(option.get("optionId"), str)
                    and option["optionId"]
                    in {"allow_once", "allow_always", "reject_once", "reject_always"}
                }
            )
            permission_id = message.get("id")
            if permission_id is not None:
                pending_wait = PendingPermissionWait()
                pending_wait.abandon()
                evidence["pending_permission_wait_abandoned"] = (
                    pending_wait.state == "abandoned"
                )
                evidence["late_permission_response_accepted"] = pending_wait.resolve(
                    "allow_once"
                )
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "id": permission_id,
                        "result": {"outcome": {"outcome": "cancelled"}},
                    }
                )
                evidence["permission_reply"] = "cancelled"
            continue
        if method == "session/update":
            update = message.get("params", {}).get("update", {})
            update_type = update.get("sessionUpdate") if isinstance(update, dict) else None
            if discovery_mode and not discovery_callback_allowed(method, update_type):
                evidence["unexpected_discovery_callback"] = update_type or method
                return None
            evidence["session_updates"] += 1
            if update.get("sessionUpdate") == "tool_call":
                evidence["tool_call_updates"] += 1
            continue
        if discovery_mode and method is not None:
            evidence["unexpected_discovery_callback"] = method
            return None
        if message.get("id") == request_id:
            return message
        evidence["unexpected_messages"] += 1


class PendingPermissionWait:
    """Model one unresolved host decision that cancellation abandons."""

    def __init__(self) -> None:
        self.state = "pending"

    def abandon(self) -> None:
        if self.state == "pending":
            self.state = "abandoned"

    def resolve(self, option_id: str) -> bool:
        if self.state != "pending" or option_id not in {"allow_once", "reject_once"}:
            return False
        self.state = "resolved"
        return True


def record_rpc_error(
    stage: str,
    response: dict[str, Any],
    evidence: dict[str, Any],
) -> bool:
    error = response.get("error")
    if not isinstance(error, dict):
        return False
    code = error.get("code")
    safe_code = code if isinstance(code, int) and not isinstance(code, bool) else None
    authentication_required = (
        safe_code == -32000 and error.get("message") == "Authentication required"
    )
    evidence[stage] = (
        "authentication-required" if authentication_required else "json-rpc-error"
    )
    evidence[f"{stage}_error"] = {
        "code": safe_code,
        "message": "Authentication required" if authentication_required else "redacted",
        "data_omitted": "data" in error,
    }
    return True


def auth_method_ids(result: dict[str, Any]) -> list[str]:
    methods = result.get("authMethods", [])
    if not isinstance(methods, list):
        return []
    ids = {
        method["id"]
        for method in methods
        if isinstance(method, dict)
        and isinstance(method.get("id"), str)
        and 0 < len(method["id"]) <= 80
        and all(character.isalnum() or character in "_.:-" for character in method["id"])
    }
    return sorted(ids)


def launch_command(profile: str, command: list[str], child_environment: dict[str, str]) -> list[str]:
    helper = ROOT / "scripts/run-with-isolated-home.sh"
    return [
        "/bin/bash",
        str(helper),
        "--home-var",
        "COPILOT_HOME",
        "--home-var",
        "GH_COPILOT_HOME",
        "--",
        "/usr/bin/env",
        "-i",
        *[f"{key}={value}" for key, value in child_environment.items()],
        "/usr/bin/sandbox-exec",
        "-p",
        profile,
        *command,
    ]


FAKE_AGENT_SOURCE = r'''#!/bin/bash
set -eu
scratch=${SWALLOWTAIL_EXPECTED_HOME%/home}
[[ "$HOME" == "$SWALLOWTAIL_EXPECTED_HOME" ]]
[[ "$GITHUB_TOKEN" == "SWALLOWTAIL_OFFLINE_FAKE_TOKEN" ]]

for host in 127.0.0.1 203.0.113.1; do
  denial_file="$scratch/network-denied-${host}.txt"
  if (exec 3<>"/dev/tcp/$host/443") 2>"$denial_file"; then
    exit 31
  fi
  denial=$(<"$denial_file")
  [[ "$denial" == *"Operation not permitted"* || "$denial" == *"Permission denied"* ]]
done

keychain_denial="$scratch/keychain-denied.txt"
if (exec 3<"$SWALLOWTAIL_BLOCKED_HOST_HOME/Library/Keychains/login.keychain-db") 2>"$keychain_denial"; then
  exit 32
fi
denial=$(<"$keychain_denial")
[[ "$denial" == *"Operation not permitted"* || "$denial" == *"Permission denied"* ]]

for blocked_file in \
  "$SWALLOWTAIL_BLOCKED_HOST_HOME/.copilot/config.json" \
  "$SWALLOWTAIL_BLOCKED_HOST_HOME/.copilot/settings.json" \
  "$SWALLOWTAIL_BLOCKED_REPOSITORY/README.md"; do
  denial_file="$scratch/resource-read-denied-$(basename "$blocked_file").txt"
  if (exec 3<"$blocked_file") 2>"$denial_file"; then
    exit 33
  fi
  denial=$(<"$denial_file")
  [[ "$denial" == *"Operation not permitted"* || "$denial" == *"Permission denied"* ]]
done

for blocked_file in \
  "$SWALLOWTAIL_BLOCKED_HOST_HOME/.copilot/config.json" \
  "$SWALLOWTAIL_BLOCKED_HOST_HOME/.copilot/settings.json" \
  "$SWALLOWTAIL_BLOCKED_HOST_HOME/Library/Keychains/copilot-cli.fake" \
  "$SWALLOWTAIL_BLOCKED_REPOSITORY/.swallowtail-write-probe"; do
  denial_file="$scratch/resource-write-denied-$(basename "$blocked_file").txt"
  if (printf 'must-not-write' >"$blocked_file") 2>"$denial_file"; then
    exit 34
  fi
  denial=$(<"$denial_file")
  [[ "$denial" == *"Operation not permitted"* || "$denial" == *"Permission denied"* ]]
done

record_has_pre_execution=false
while IFS= read -r line; do
  if [[ "$line" == *'"artifact_execution_started": false'* ]]; then
    record_has_pre_execution=true
  fi
done < "$SWALLOWTAIL_RECORD_PATH"
[[ "$record_has_pre_execution" == true ]]

IFS= read -r initialize
[[ "$initialize" == *'"method":"initialize"'* ]]
printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":1,"agentCapabilities":{},"agentInfo":{"name":"fake-copilot","version":"fake"}}}'
IFS= read -r session_new
[[ "$session_new" == *'"method":"session/new"'* ]]
printf '%s\n' '{"jsonrpc":"2.0","id":2,"result":{"sessionId":"fake-session"}}'
IFS= read -r prompt
[[ "$prompt" == *'"method":"session/prompt"'* ]]
printf '%s\n' '{"jsonrpc":"2.0","id":99,"method":"session/request_permission","params":{"sessionId":"fake-session","toolCall":{"toolCallId":"fake-tool","status":"pending"},"options":[{"optionId":"allow_always","name":"Allow always","kind":"allow_always"},{"optionId":"allow_once","name":"Allow once","kind":"allow_once"},{"optionId":"reject_once","name":"Reject once","kind":"reject_once"},{"optionId":"reject_always","name":"Reject always","kind":"reject_always"}]}}'
IFS= read -r permission
[[ "$permission" == *'"id":99'* && "$permission" == *'"outcome":"cancelled"'* ]]
[[ "$permission" != *'"optionId":"allow_always"'* ]]
[[ ! -e "$SWALLOWTAIL_EFFECT_MARKER" ]]
printf '%s\n' '{"jsonrpc":"2.0","id":3,"result":{"stopReason":"cancelled"}}'
'''


FAKE_AUTH_AGENT_SOURCE = r'''import hashlib, json, os, select, sys

auth_fd = int(os.environ["SWALLOWTAIL_AUTH_FD"])
credential = os.read(auth_fd, 128)
os.close(auth_fd)
if hashlib.sha256(credential).hexdigest() != "de9e245bec7a5380291b92667b75b0b789c8ce92bd2ce370e84f41f33d12af91":
    raise SystemExit(71)
if any(name in os.environ for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN")):
    raise SystemExit(72)
marker = os.environ["SWALLOWTAIL_EFFECT_MARKER"]

def send(value):
    sys.stdout.write(json.dumps(value, separators=(",", ":")) + "\n")
    sys.stdout.flush()

def read_line(timeout):
    ready, _, _ = select.select([sys.stdin], [], [], timeout)
    if not ready:
        return None
    line = sys.stdin.buffer.readline()
    return json.loads(line) if line else None

def report(**values):
    report_path = os.environ["SWALLOWTAIL_FAKE_AGENT_REPORT"]
    with open(report_path, "a", encoding="utf-8") as output:
        output.write(json.dumps(values, sort_keys=True) + "\n")

initialize = read_line(1.0)
if not initialize or initialize.get("method") != "initialize":
    raise SystemExit(73)
send({"jsonrpc":"2.0","id":1,"result":{"protocolVersion":1,"agentCapabilities":{},"authMethods":[{"id":"copilot-login","name":"Fake delegated auth"}],"agentInfo":{"name":"fake-copilot-auth","version":"fake"}}})
authenticate = read_line(1.0)
if not authenticate or authenticate.get("method") != "authenticate":
    raise SystemExit(74)
if authenticate.get("params") != {"methodId":"copilot-login"}:
    raise SystemExit(75)
send({"jsonrpc":"2.0","id":2,"result":{}})
session_new = read_line(1.0)
if not session_new or session_new.get("method") != "session/new":
    raise SystemExit(76)
send({"jsonrpc":"2.0","id":3,"result":{"sessionId":"fake-auth-session","models":{"currentModelId":"fake-github-hosted-model"}}})
prompt = read_line(1.0)
if not prompt or prompt.get("method") != "session/prompt":
    raise SystemExit(77)
if os.path.exists(marker):
    raise SystemExit(78)
send({"jsonrpc":"2.0","id":99,"method":"session/request_permission","params":{"sessionId":"fake-auth-session","toolCall":{"toolCallId":"fake-tool","status":"pending"},"options":[{"optionId":"allow_once","name":"Allow once","kind":"allow_once"},{"optionId":"reject_once","name":"Reject once","kind":"reject_once"}]}})
permission = read_line(1.0)
if not permission or permission.get("id") != 99:
    raise SystemExit(79)
outcome = permission.get("result", {}).get("outcome", {}).get("outcome")
if outcome != "cancelled":
    selected = permission.get("result", {}).get("outcome", {}).get("optionId")
    if selected in ("allow_once", "allow_always"):
        with open(marker, "w", encoding="utf-8") as output:
            output.write("effect")
    report(permission_outcome="not-cancelled", effect_marker_present=os.path.exists(marker))
    raise SystemExit(80)
report(permission_outcome="cancelled", pending_prompt=True, effect_marker_present=os.path.exists(marker))
cancel = read_line(0.8)
if not cancel or cancel.get("method") != "session/cancel":
    raise SystemExit(81)
send({"jsonrpc":"2.0","id":4,"result":{"stopReason":"cancelled"}})
report(permission_outcome="cancelled", pending_prompt=False, stop_reason="cancelled", effect_marker_present=os.path.exists(marker))
'''


FAKE_DISCOVERY_AGENT_SOURCE = r'''import hashlib, json, os, select, sys

auth_fd = int(os.environ["SWALLOWTAIL_AUTH_FD"])
credential = os.read(auth_fd, 128)
os.close(auth_fd)
if hashlib.sha256(credential).hexdigest() != "de9e245bec7a5380291b92667b75b0b789c8ce92bd2ce370e84f41f33d12af91":
    raise SystemExit(91)
if any(name in os.environ for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN")):
    raise SystemExit(92)

def send(value):
    sys.stdout.write(json.dumps(value, separators=(",", ":")) + "\n")
    sys.stdout.flush()

def read_line(timeout):
    ready, _, _ = select.select([sys.stdin], [], [], timeout)
    if not ready:
        return None
    line = sys.stdin.buffer.readline()
    return json.loads(line) if line else None

initialize = read_line(1.0)
if not initialize or initialize.get("method") != "initialize":
    raise SystemExit(93)
send({"jsonrpc":"2.0","id":1,"result":{"protocolVersion":1,"agentCapabilities":{},"authMethods":[{"id":"copilot-login","name":"Fake delegated auth"}],"agentInfo":{"name":"fake-copilot-discovery","version":"fake"}}})
authenticate = read_line(1.0)
if not authenticate or authenticate.get("method") != "authenticate":
    raise SystemExit(94)
if authenticate.get("params") != {"methodId":"copilot-login"}:
    raise SystemExit(95)
send({"jsonrpc":"2.0","id":2,"result":{}})
session_new = read_line(1.0)
if not session_new or session_new.get("method") != "session/new":
    raise SystemExit(96)
send({"jsonrpc":"2.0","id":3,"result":{"sessionId":"fake-discovery-session","models":{"currentModelId":"fake-auto","availableModels":[{"modelId":"fake-auto"},{"modelId":"fake-gpt-5.4"}]},"configOptions":[{"id":"model","currentValue":"fake-auto","options":[{"value":"fake-auto"},{"value":"fake-gpt-5.4"}]}]}})
if read_line(1.0) is not None:
    raise SystemExit(97)
'''



def fake_delegated_credential(context: dict[str, Any]) -> bytes | None:
    """Return a non-secret fake capability only for an exact synthetic grant."""
    if context != {
        "account_ref": FAKE_ACCOUNT_REF,
        "entitlement": "confirmed-in-fake",
        "model_id": FAKE_MODEL_ID,
        "model_policy": "pinned-no-fallback",
        "credential_request": "one-use-copilot-session",
        "audience": FAKE_AUDIENCE,
        "provider_traffic": [FAKE_AUDIENCE],
    }:
        return None
    return FAKE_DELEGATED_CREDENTIAL


def discovery_keychain_service_allowed(
    service: str, approved_service: str
) -> bool:
    return bool(service) and service == approved_service


def discovery_fake_preflight(
    scratch: Path, host_home: Path, repository_root: Path
) -> dict[str, Any]:
    valid = {
        "account_ref": FAKE_ACCOUNT_REF,
        "entitlement": "confirmed-in-fake",
        "model_id": FAKE_MODEL_ID,
        "model_policy": "pinned-no-fallback",
        "credential_request": "one-use-copilot-session",
        "audience": FAKE_AUDIENCE,
        "provider_traffic": [FAKE_AUDIENCE],
    }
    allowed_origins = {"https://github.fake.test", "https://copilot.fake.test"}
    controls = {
        "wrong_account": fake_delegated_credential(
            {**valid, "account_ref": "github-account:other"}
        ) is None,
        "unapproved_endpoint": not discovery_endpoint_allowed(
            "https://unapproved.fake.test", allowed_origins
        ),
        "prompt_request": not discovery_request_allowed("session/prompt"),
        "config_write_rpc": not discovery_request_allowed(
            "session/set_config_option"
        ),
        "permission_callback": not discovery_callback_allowed(
            "session/request_permission"
        ),
        "tool_callback": not discovery_callback_allowed(
            "session/update", "tool_call"
        ),
        "model_output_callback": not discovery_callback_allowed(
            "session/update", "agent_message_chunk"
        ),
        "excess_invocation": not discovery_budget_allowed(2, 1.0),
        "excess_time": not discovery_budget_allowed(1, 60.001),
        "wrong_keychain_service": not discovery_keychain_service_allowed(
            "other-service", "copilot-cli"
        ),
        "auth_config_write": not discovery_write_allowed(
            host_home / ".copilot" / "config.json", scratch
        )
        and not discovery_write_allowed(
            host_home / ".copilot" / "settings.json", scratch
        ),
        "keychain_write": not discovery_write_allowed(
            host_home / "Library" / "Keychains" / "copilot-cli.fake", scratch
        ),
        "host_home_read_escape": not discovery_read_allowed(
            host_home / ".copilot" / "config.json", host_home, repository_root
        ),
        "repository_read_escape": not discovery_read_allowed(
            repository_root / "README.md", host_home, repository_root
        ),
    }
    failed = [name for name, rejected in controls.items() if not rejected]
    if failed:
        raise RuntimeError(f"discovery fake admitted negative controls: {failed}")
    if not all(
        discovery_request_allowed(method)
        for method in ("initialize", "authenticate", "session/new")
    ):
        raise RuntimeError("discovery fake rejected an allowed ACP operation")
    if not discovery_endpoint_allowed(
        "https://copilot.fake.test", allowed_origins
    ):
        raise RuntimeError("discovery fake rejected its exact synthetic endpoint")
    if not discovery_budget_allowed(1, 60.0):
        raise RuntimeError("discovery fake rejected the exact one-invocation budget")
    if not discovery_write_allowed(scratch / "record.json", scratch):
        raise RuntimeError("discovery fake rejected task-scratch writes")
    if not discovery_keychain_service_allowed("copilot-cli", "copilot-cli"):
        raise RuntimeError("discovery fake rejected its exact synthetic keychain service")
    return {
        "status": "passed",
        "allowed_acp_requests": sorted(DISCOVERY_REQUEST_METHODS),
        "allowed_synthetic_origins": sorted(allowed_origins),
        "negative_controls": {
            name: "rejected-before-original-start" for name in controls
        },
    }


def authenticated_fake_preflight(
    scratch: Path, host_home: Path, repository_root: Path
) -> dict[str, Any]:
    valid = {
        "account_ref": FAKE_ACCOUNT_REF,
        "entitlement": "confirmed-in-fake",
        "model_id": FAKE_MODEL_ID,
        "model_policy": "pinned-no-fallback",
        "credential_request": "one-use-copilot-session",
        "audience": FAKE_AUDIENCE,
        "provider_traffic": [FAKE_AUDIENCE],
    }
    if fake_delegated_credential(valid) != FAKE_DELEGATED_CREDENTIAL:
        raise RuntimeError("valid fake host delegation was not admitted")

    negative_cases: dict[str, Any] = {
        "wrong_account": {**valid, "account_ref": "github-account:other"},
        "wrong_audience": {**valid, "audience": "unapproved.fake.test"},
        "wrong_model": {**valid, "model_id": "fake-local-gemma-12b"},
        "missing_entitlement": {**valid, "entitlement": "missing"},
        "unexpected_credential_request": {**valid, "credential_request": "read-host-keychain"},
        "unexpected_provider_traffic": {
            **valid,
            "provider_traffic": [FAKE_AUDIENCE, "unapproved.fake.test"],
        },
    }
    rejected: dict[str, dict[str, Any]] = {}
    for name, context in negative_cases.items():
        if fake_delegated_credential(context) is not None:
            raise RuntimeError(f"fake preflight admitted negative case {name}")
        rejected[name] = {
            "status": "rejected-before-process-start",
            "prompt_sent": False,
            "effect_marker_present": False,
        }
    return {
        "status": "passed",
        "accepted_fake_grant": "exact-account-entitlement-model-policy-and-audience",
        "credential_transport": "one-use-inherited-pipe",
        "credential_environment_variables": "absent",
        "negative_controls": rejected,
        "discovery_controls": discovery_fake_preflight(
            scratch, host_home, repository_root
        ),
    }


def discovery_session_observation(result: dict[str, Any]) -> dict[str, Any]:
    def safe_model_id(value: Any) -> str | None:
        if (
            isinstance(value, str)
            and 0 < len(value) <= 160
            and re.fullmatch(r"[A-Za-z0-9._:/+-]+", value)
        ):
            return value
        return None

    models = result.get("models")
    if not isinstance(models, dict):
        models = {}
    current_model_id = safe_model_id(models.get("currentModelId"))
    available = models.get("availableModels", [])
    model_ids = {
        model_id
        for model in available
        if isinstance(model, dict)
        for model_id in (safe_model_id(model.get("modelId")),)
        if model_id is not None
    } if isinstance(available, list) else set()
    if current_model_id is not None:
        model_ids.add(current_model_id)

    model_config: dict[str, Any] | None = None
    options = result.get("configOptions", [])
    if isinstance(options, list):
        for option in options:
            if not isinstance(option, dict) or option.get("id") != "model":
                continue
            values = option.get("options", [])
            value_ids = {
                model_id
                for item in values
                if isinstance(item, dict)
                for model_id in (safe_model_id(item.get("value")),)
                if model_id is not None
            } if isinstance(values, list) else set()
            model_config = {
                "option_id": "model",
                "current_value": safe_model_id(option.get("currentValue")),
                "available_values": sorted(value_ids),
            }
            model_ids.update(value_ids)
            break
    return {
        "current_model_id": current_model_id,
        "available_model_ids": sorted(model_ids),
        "model_config_option": model_config,
    }


def launch_acp(
    executable: Path,
    command: list[str],
    scratch: Path,
    host_home: Path,
    record_path: Path,
    exact_version: str | None,
    *,
    delegated_credential: bytes | None = None,
    authenticate_method_id: str | None = None,
    run_timeout_seconds: float = RUN_TIMEOUT_SECONDS,
    mode: str = "permission-proof",
    repository_root: Path = ROOT,
) -> dict[str, Any]:
    if not command or Path(command[0]) != executable:
        raise ValueError("the launched executable must match the selected artifact")
    if mode not in {"permission-proof", "pre-prompt-discovery"}:
        raise ValueError("unknown Copilot ACP harness mode")
    scratch.mkdir(parents=True, exist_ok=True)
    marker = scratch / "permission-effect-marker"
    if marker.exists():
        marker.unlink()
    auth_read_fd: int | None = None
    pass_fds: tuple[int, ...] = ()
    env = clean_child_environment(scratch, include_fake_token=delegated_credential is None)
    if delegated_credential is not None:
        auth_read_fd, auth_write_fd = os.pipe()
        try:
            written = os.write(auth_write_fd, delegated_credential)
            if written != len(delegated_credential):
                raise OSError("short write while delegating fake capability")
        except OSError:
            os.close(auth_read_fd)
            auth_read_fd = None
            raise
        finally:
            os.close(auth_write_fd)
        pass_fds = (auth_read_fd,)
        env["SWALLOWTAIL_AUTH_FD"] = str(auth_read_fd)
        env["SWALLOWTAIL_AUTH_TRANSPORT"] = "one-use-inherited-pipe"
        env["SWALLOWTAIL_FAKE_AGENT_REPORT"] = str(
            (scratch / "fake-agent-report.jsonl").resolve()
        )
    env.update(
        {
            "SWALLOWTAIL_BLOCKED_HOST_HOME": str(host_home.resolve()),
            "SWALLOWTAIL_EXPECTED_HOME": env["HOME"],
            "SWALLOWTAIL_RECORD_PATH": str(record_path.resolve()),
            "SWALLOWTAIL_EFFECT_MARKER": str(marker),
            "SWALLOWTAIL_BLOCKED_REPOSITORY": str(repository_root.resolve()),
        }
    )
    profile = sandbox_profile(scratch, host_home, repository_root)
    argv = launch_command(profile, command, env)
    started = utc_now()
    try:
        process = subprocess.Popen(
            argv,
            cwd=scratch,
            env={
                "HOME": str(host_home.resolve()),
                "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
                "TMPDIR": str((scratch / "tmp").resolve()),
            },
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            bufsize=0,
            start_new_session=True,
            pass_fds=pass_fds,
        )
    finally:
        if auth_read_fd is not None:
            os.close(auth_read_fd)
    stderr_collector = BoundedStderrCollector(process.stderr)
    client = StdioClient(process)
    evidence: dict[str, Any] = {
        "started_at_utc": started,
        "selected_argv": (
            [Path(command[0]).name, *command[-2:]]
            if exact_version
            else ["python3", "fake ACP agent"]
        ),
        "initialize": "not-reached",
        "auth_method_ids": [],
        "authenticate": "not-requested",
        "authenticate_method_selected": None,
        "session_new": "not-reached",
        "session_prompt": "not-reached",
        "permission_request_observed": False,
        "permission_methods_observed": [],
        "permission_reply": "none",
        "session_cancel_sent": False,
        "session_updates": 0,
        "tool_call_updates": 0,
        "unexpected_messages": 0,
        "timed_out": False,
        "stream_failed": False,
    }
    if delegated_credential is not None:
        evidence["credential_transport"] = "one-use-inherited-pipe"
        evidence["credential_environment_variables_absent"] = not any(
            name in env for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN")
        )
        evidence["effect_marker_absent_before_prompt"] = not marker.exists()

    def send_request(method: str, request: dict[str, Any]) -> None:
        if mode == "pre-prompt-discovery" and not discovery_request_allowed(method):
            raise RuntimeError(f"discovery operation is not permitted: {method}")
        client.send(request)

    try:
        send_request(
            "initialize",
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {"protocolVersion": 1, "clientCapabilities": {}},
            }
        )
        response = wait_for_response(
            client,
            1,
            evidence,
            READ_TIMEOUT_SECONDS,
            discovery_mode=mode == "pre-prompt-discovery",
        )
        if response is None:
            evidence["initialize"] = "no-result"
            return evidence
        if record_rpc_error("initialize", response, evidence):
            return evidence
        if "result" not in response or not isinstance(response["result"], dict):
            evidence["initialize"] = "invalid-response"
            return evidence
        evidence["initialize"] = "success"
        result = response["result"]
        evidence["auth_method_ids"] = auth_method_ids(result)
        reported = result.get("agentInfo", {}).get("version")
        evidence["reported_version_matches"] = (
            reported == exact_version if exact_version else reported == "fake"
        )

        next_request_id = 2
        if authenticate_method_id is not None:
            if authenticate_method_id not in evidence["auth_method_ids"]:
                evidence["authenticate"] = "advertised-method-missing"
                return evidence
            evidence["authenticate_method_selected"] = authenticate_method_id
            send_request(
                "authenticate",
                {
                    "jsonrpc": "2.0",
                    "id": next_request_id,
                    "method": "authenticate",
                    "params": {"methodId": authenticate_method_id},
                }
            )
            response = wait_for_response(
                client,
                next_request_id,
                evidence,
                READ_TIMEOUT_SECONDS,
                discovery_mode=mode == "pre-prompt-discovery",
            )
            if response is None:
                evidence["authenticate"] = "no-result"
                return evidence
            if record_rpc_error("authenticate", response, evidence):
                evidence["authenticate"] = "rpc-error"
                return evidence
            if "result" not in response or not isinstance(response["result"], dict):
                evidence["authenticate"] = "invalid-response"
                return evidence
            evidence["authenticate"] = "success"
            next_request_id += 1

        send_request(
            "session/new",
            {
                "jsonrpc": "2.0",
                "id": next_request_id,
                "method": "session/new",
                "params": {"cwd": str(scratch), "mcpServers": []},
            }
        )
        response = wait_for_response(
            client,
            next_request_id,
            evidence,
            READ_TIMEOUT_SECONDS,
            discovery_mode=mode == "pre-prompt-discovery",
        )
        if response is None:
            evidence["session_new"] = "no-result"
            return evidence
        if record_rpc_error("session_new", response, evidence):
            return evidence
        if "result" not in response or not isinstance(response["result"], dict):
            evidence["session_new"] = "invalid-response"
            return evidence
        evidence["session_new"] = "success"
        session_id = response["result"].get("sessionId")
        models = response["result"].get("models", {})
        if isinstance(models, dict) and isinstance(models.get("currentModelId"), str):
            evidence["observed_model_id"] = models["currentModelId"]
        if not isinstance(session_id, str):
            evidence["session_new"] = "result-without-session-id"
            return evidence
        if mode == "pre-prompt-discovery":
            evidence["session_prompt"] = "not-authorized-not-sent"
            evidence["discovery_observation"] = discovery_session_observation(
                response["result"]
            )
            return evidence
        prompt_request_id = next_request_id + 1

        marker_prompt = f"Run the tool needed to create this empty marker file, then stop: {marker}"
        send_request(
            "session/prompt",
            {
                "jsonrpc": "2.0",
                "id": prompt_request_id,
                "method": "session/prompt",
                "params": {
                    "sessionId": session_id,
                    "prompt": [{"type": "text", "text": marker_prompt}],
                },
            }
        )
        evidence["session_prompt"] = "sent"
        response = wait_for_response(
            client, prompt_request_id, evidence, run_timeout_seconds
        )
        if response is None:
            try:
                send_request(
                    "session/cancel",
                    {
                        "jsonrpc": "2.0",
                        "method": "session/cancel",
                        "params": {"sessionId": session_id},
                    }
                )
                evidence["session_cancel_sent"] = True
                response = wait_for_response(client, prompt_request_id, evidence, 2.0)
            except (BrokenPipeError, OSError):
                evidence["stream_failed"] = True
        if response is not None:
            if "result" in response:
                evidence["session_prompt"] = str(
                    response["result"].get("stopReason", "result")
                )[:40]
            elif record_rpc_error("session_prompt", response, evidence):
                pass
            else:
                evidence["session_prompt"] = "error-response"
        else:
            evidence["session_prompt"] = "no-result"
        if evidence["permission_request_observed"]:
            evidence["permission_policy_outcome"] = "cancelled"
        elif evidence["session_prompt"] == "no-result":
            evidence["permission_policy_outcome"] = "not-reached"
        else:
            evidence["permission_policy_outcome"] = "no-request-observed"
        return evidence
    finally:
        client.close()
        code, forced, process_group_joined = stop_owned_process_group(process)
        stderr_reader_joined = stderr_collector.join(LIVE_CLEANUP_SECONDS)
        stderr_summary = stderr_collector.summary(reader_joined=stderr_reader_joined)
        if process.stderr is not None:
            process.stderr.close()
        evidence["exit_code"] = code
        evidence["launcher_exit_status"] = code
        evidence["vendor_startup_status"] = (
            "acp-initialize-response-observed"
            if exact_version is not None
            and evidence.get("initialize") not in {"not-reached", "no-result"}
            else "unknown"
            if exact_version is not None
            else "fake-acp-initialize-response-observed"
            if evidence.get("initialize") == "success"
            else "unknown"
        )
        evidence["forced_process_group_kill"] = forced
        evidence["process_joined"] = process.returncode is not None
        evidence["process_group_joined"] = process_group_joined
        evidence["stderr_drain_joined"] = stderr_reader_joined
        evidence["stderr"] = stderr_summary
        evidence["effect_marker_present"] = marker.exists()


def safe_model_id(value: Any) -> str | None:
    if not isinstance(value, str) or not re.fullmatch(r"[A-Za-z0-9._:+/-]{1,100}", value):
        return None
    if value.startswith(("gho_", "ghp_", "ghs_", "github_pat_", "Bearer")):
        return None
    return value


def capture_model_observation(result: dict[str, Any]) -> dict[str, Any]:
    models = result.get("models")
    current = None
    if isinstance(models, dict):
        current = safe_model_id(models.get("currentModelId"))
    underlying = current if current and current.lower() != "auto" else None
    return {
        "session_new_current_model_id": current,
        "underlying_model_identity": underlying,
        "underlying_model_identity_status": "observed" if underlying else "unobserved",
        "observation_source": "session/new result.models.currentModelId"
        if current
        else "not-exposed-by-session/new",
    }


def capture_model_update(update: dict[str, Any], observations: list[dict[str, Any]]) -> None:
    if update.get("sessionUpdate") != "config_option_update":
        return
    option_id = update.get("configOptionId", update.get("configId"))
    value = update.get("value", update.get("currentValue"))
    config_option = update.get("configOption")
    if isinstance(config_option, dict):
        option_id = config_option.get("id", option_id)
        value = config_option.get("currentValue", value)
    if option_id != "model":
        return
    model_id = safe_model_id(value)
    if model_id is not None:
        observations.append(
            {
                "model_id": model_id,
                "source": "session/update config_option_update for model",
            }
        )


def wait_for_original_response(
    client: StdioClient,
    request_id: int,
    evidence: dict[str, Any],
    deadline: float,
) -> dict[str, Any] | None:
    while time.monotonic() < deadline:
        try:
            message = client.receive(
                min(READ_TIMEOUT_SECONDS, deadline - time.monotonic())
            )
        except TimeoutError:
            evidence["timed_out"] = True
            return None
        except (EOFError, ValueError, json.JSONDecodeError):
            evidence["stream_failed"] = True
            return None
        method = message.get("method")
        if method == "session/update":
            params = message.get("params")
            update = params.get("update") if isinstance(params, dict) else None
            if isinstance(update, dict):
                evidence["session_updates"] += 1
                if update.get("sessionUpdate") == "tool_call":
                    evidence["tool_call_updates"] += 1
                capture_model_update(update, evidence["model_observations"])
            continue
        if method == "session/request_permission":
            evidence["pre_prompt_permission_request"] = True
            permission_id = message.get("id")
            if isinstance(permission_id, int):
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "id": permission_id,
                        "result": {"outcome": {"outcome": "cancelled"}},
                    }
                )
            continue
        if message.get("id") == request_id:
            return message
        evidence["unexpected_messages"] += 1
    evidence["timed_out"] = True
    return None


def safe_rpc_error(response: dict[str, Any]) -> dict[str, Any] | None:
    error = response.get("error")
    if not isinstance(error, dict):
        return None
    code = error.get("code")
    safe_code = code if isinstance(code, int) and not isinstance(code, bool) else None
    message = error.get("message")
    normalized = message.strip().lower() if isinstance(message, str) else ""
    if normalized == "authentication required":
        kind = "authentication-required"
    elif any(word in normalized for word in ("quota", "entitlement", "rate limit", "billing")):
        kind = "quota-or-entitlement-stop"
    else:
        kind = "redacted-provider-error"
    return {
        "kind": kind,
        "code": safe_code,
        "message": "Authentication required" if kind == "authentication-required" else "redacted",
        "data_omitted": "data" in error,
    }


def permission_evidence_is_complete(
    evidence: dict[str, Any],
    expected_probe_proxy: dict[str, int] | None = None,
) -> bool:
    proxy = evidence.get("proxy", {})
    stderr = evidence.get("stderr", {})
    expected_proxy = expected_probe_proxy or {
        "unlisted_destination_count": 0,
        "sni_mismatch_count": 0,
        "connection_failure_count": 0,
    }
    return bool(
        evidence.get("initialize") == "success"
        and evidence.get("reported_version_matches") is True
        and evidence.get("session_new") == "success"
        and evidence.get("auto_session_created") is True
        and not evidence.get("pre_prompt_permission_request")
        and evidence.get("prompt_requests_sent") == 1
        and evidence.get("credential_environment_variables_absent") is True
        and evidence.get("permission_request_count") == 1
        and evidence.get("permission_reply") == "cancelled"
        and not evidence.get("permission_callback_ambiguous")
        and not evidence.get("late_permission_response_accepted")
        and evidence.get("session_cancel_sent") is True
        and evidence.get("session_prompt") == "cancelled"
        and not evidence.get("effect_marker_present")
        and evidence.get("process_joined") is True
        and evidence.get("process_group_joined") is True
        and evidence.get("stderr_drain_joined") is True
        and stderr.get("reader_error") is False
        and stderr.get("raw_persisted") is False
        and stderr.get("raw_displayed") is False
        and evidence.get("proxy_threads_joined") is True
        and evidence.get("elapsed_seconds", LIVE_PERMISSION_SECONDS + 1)
        <= LIVE_PERMISSION_SECONDS
        and all(proxy.get(name) == count for name, count in expected_proxy.items())
    )


def wait_for_original_prompt(
    client: StdioClient,
    request_id: int,
    session_id: str,
    evidence: dict[str, Any],
    deadline: float,
) -> dict[str, Any] | None:
    while time.monotonic() < deadline:
        try:
            message = client.receive(
                min(READ_TIMEOUT_SECONDS, deadline - time.monotonic())
            )
        except TimeoutError:
            evidence["timed_out"] = True
            return None
        except (EOFError, ValueError, json.JSONDecodeError):
            evidence["stream_failed"] = True
            return None
        method = message.get("method")
        if method == "session/request_permission":
            evidence["permission_request_count"] += 1
            params = message.get("params")
            options = params.get("options", []) if isinstance(params, dict) else []
            if isinstance(options, list):
                evidence["permission_option_ids"].append(
                    sorted(
                        {
                            option["optionId"]
                            for option in options
                            if isinstance(option, dict)
                            and isinstance(option.get("optionId"), str)
                            and option["optionId"] in {
                                "allow_once",
                                "allow_always",
                                "reject_once",
                                "reject_always",
                            }
                        }
                    )
                )
            permission_id = message.get("id")
            if not isinstance(permission_id, int):
                evidence["permission_callback_ambiguous"] = True
                if not evidence["session_cancel_sent"]:
                    client.send(
                        {
                            "jsonrpc": "2.0",
                            "method": "session/cancel",
                            "params": {"sessionId": session_id},
                        }
                    )
                    evidence["session_cancel_sent"] = True
                continue
            pending_wait = PendingPermissionWait()
            pending_wait.abandon()
            evidence["pending_permission_wait_abandoned"] = (
                pending_wait.state == "abandoned"
            )
            evidence["late_permission_response_accepted"] = pending_wait.resolve(
                "allow_once"
            )
            client.send(
                {
                    "jsonrpc": "2.0",
                    "id": permission_id,
                    "result": {"outcome": {"outcome": "cancelled"}},
                }
            )
            evidence["permission_reply"] = "cancelled"
            if not evidence["session_cancel_sent"]:
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "method": "session/cancel",
                        "params": {"sessionId": session_id},
                    }
                )
                evidence["session_cancel_sent"] = True
            continue
        if method == "session/update":
            params = message.get("params")
            update = params.get("update") if isinstance(params, dict) else None
            if isinstance(update, dict):
                evidence["session_updates"] += 1
                if update.get("sessionUpdate") == "tool_call":
                    evidence["tool_call_updates"] += 1
                capture_model_update(update, evidence["model_observations"])
            continue
        if message.get("id") == request_id:
            return message
        evidence["unexpected_messages"] += 1
    evidence["timed_out"] = True
    return None


def run_permission_attempt(
    binary: Path,
    version: str,
    scratch: Path,
    record_dir: Path,
    record_path: Path,
    record: dict[str, Any],
    repository_root: Path,
    plan: dict[str, Any],
    correction_plan: dict[str, Any],
    *,
    binary_sha256: str,
    host_home: Path | None = None,
    extra_environment: Callable[[int], dict[str, str]] | None = None,
    proxy_fake_only: bool = False,
    expected_probe_proxy: dict[str, int] | None = None,
) -> dict[str, Any]:
    if not proxy_fake_only:
        require_original_process_containment()
    host_home = (host_home or Path(os.environ["HOME"])).resolve(strict=True)
    scratch.mkdir(parents=True, exist_ok=False)
    (scratch / "tmp").mkdir()
    marker = scratch / "permission-effect-marker"
    proxy = CopilotEgressProxy(
        set(plan["network_policy"]["allowed_hosts"]),
        tuple(plan["network_policy"]["allowed_subdomains"]),
        time.monotonic() + LIVE_PERMISSION_SECONDS,
        fake_only=proxy_fake_only,
    )
    proxy_port = proxy.start()
    executable = binary.resolve(strict=True)
    command = [str(executable), "--model", "auto", "--acp", "--stdio"]
    profile = permission_sandbox_profile(
        scratch,
        record_dir,
        host_home,
        repository_root,
        executable,
        proxy_port,
        plan,
        correction_plan,
    )
    environment = original_child_environment(host_home, scratch, proxy_port)
    if extra_environment is not None:
        environment.update(extra_environment(proxy_port))
    profile_digest = hashlib.sha256(profile.encode()).hexdigest()
    invocation: dict[str, Any] = {
        "version": version,
        "binary_sha256": binary_sha256,
        "invocation_consumed": True,
        "pre_execution_fsynced": False,
        "original_start_issued": False,
        "pre_execution_record": {
            "requires_fsync_before_original_start": True,
            "permission_plan_sha256": hashlib.sha256(
                PERMISSION_PROOF_PLAN_PATH.read_bytes()
            ).hexdigest(),
            "correction_plan_sha256": hashlib.sha256(
                PERMISSION_CORRECTION_PLAN_PATH.read_bytes()
            ).hexdigest(),
            "identity": {
                "package": "@github/copilot-darwin-arm64",
                "version": version,
                "platform": "darwin-arm64",
                "binary_sha256": binary_sha256,
            },
            "account_access_ref": "github-account:betterthanclay",
            "account_identity_observation": "operator-reported-reference; not independently queried",
            "credential_boundary": {
                "source": "existing original-CLI host-owned login",
                "normal_keychain_access": "permitted through com.apple.securityd",
                "keychain_item_level_mediation": "unavailable; no per-item filter claimed",
                "token_handling_by_harness": "never extracted, copied, logged, or persisted",
                "token_environment_variables_absent": True,
                "copilot_home_relocated": False,
                "acp_authenticate_request_sent": False,
            },
            "selection_policy": {
                "requested": "Auto",
                "binding": "original CLI --model auto process argument",
                "underlying_model_identity": "record only if structured ACP data exposes it; otherwise unobserved",
                "vendor_managed_retries": "allowed within this prompt and the process/network ceiling; internal count unobserved",
                "harness_model_fallbacks": 0,
            },
            "network_policy": {
                "authority": "official Copilot authentication and provider destinations",
                "source": plan["network_policy"]["source"],
                "allowed_exact_hosts": plan["network_policy"]["allowed_hosts"],
                "allowed_subdomains": plan["network_policy"]["allowed_subdomains"],
                "default": "deny",
                "enforcement": "sandbox permits only local CONNECT proxy; proxy verifies official CONNECT host and matching TLS SNI before external dial",
                "unexpected_destinations": "reject and fail attempt without expanding policy",
                "proxy_endpoint": {"host": "127.0.0.1", "port": proxy_port},
            },
            "containment": {
                "profile_sha256": profile_digest,
                "host_home_reads": {
                    "default": "denied",
                    "read_only_exception": ".copilot/config.json",
                    "exception_basis": "current official CLI configuration directory docs; not frozen-artifact-specific",
                    "original_read_observed": False,
                },
                "repository_reads": "denied",
                "writes": "task scratch only",
        "subprocess_escape": "forks inherit this sandbox; process exec limited to this exact artifact; shell exec denied",
                "keychain_service_lookup": "com.apple.securityd only; item-level reads trusted to hash-verified vendor CLI",
            },
            "action": {
                "kind": "create-one-empty-task-scratch-marker",
                "path": "task-scratch/permission-effect-marker",
                "permission_required": True,
                "approval": "never",
                "max_effects": 0,
            },
            "budgets": plan["budgets"],
            "selected_argv": ["copilot", "--model", "auto", "--acp", "--stdio"],
        },
        "pre_execution_record_sha256": None,
        "started_at_utc": utc_now(),
        "permission_request_count": 0,
        "permission_reply": "none",
        "session_cancel_sent": False,
        "effect_marker_present": False,
        "model_observations": [],
    }
    invocation["pre_execution_record_sha256"] = json_digest(
        invocation["pre_execution_record"]
    )
    invocation["pre_execution_fsynced"] = True
    record["invocations"].append(invocation)
    write_json_durable(record_path, record)
    saved = load_json(record_path)
    if saved.get("invocations", [])[-1] != invocation:
        proxy_joined = proxy.stop_and_join(LIVE_CLEANUP_SECONDS)
        if not proxy_joined:
            raise RuntimeError("proxy threads survived failed pre-execution record read-back")
        raise RuntimeError(f"pre-execution record did not persist before {version} start")

    started = time.monotonic()
    deadline = started + LIVE_PERMISSION_SECONDS - (2 * LIVE_CLEANUP_SECONDS)
    proxy.deadline = started + LIVE_PERMISSION_SECONDS - LIVE_CLEANUP_SECONDS
    evidence: dict[str, Any] = {
        "initialize": "not-reached",
        "reported_version_matches": False,
        "auth_method_ids": [],
        "authenticate_request_sent": False,
        "session_new": "not-reached",
        "auto_policy_requested": True,
        "auto_session_created": False,
        "model_observations": [],
        "model_identity_observation_source": None,
        "session_prompt": "not-reached",
        "prompt_requests_sent": 0,
        "permission_request_count": 0,
        "permission_option_ids": [],
        "permission_reply": "none",
        "pending_permission_wait_abandoned": False,
        "late_permission_response_accepted": False,
        "permission_callback_ambiguous": False,
        "session_cancel_sent": False,
        "session_updates": 0,
        "tool_call_updates": 0,
        "unexpected_messages": 0,
        "pre_prompt_permission_request": False,
        "timed_out": False,
        "stream_failed": False,
        "effect_marker_absent_before_prompt": not marker.exists(),
        "credential_environment_variables_absent": not any(
            name in environment
            for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN")
        ),
        "proxy_threads_joined": False,
        "process_group_joined": False,
        "stderr_drain_joined": False,
        "stderr": None,
        "launcher_exit_status": None,
        "vendor_startup_status": "unknown",
    }
    process: subprocess.Popen[bytes] | None = None
    client: StdioClient | None = None
    stderr_collector: BoundedStderrCollector | None = None
    try:
        process = subprocess.Popen(
            ["/usr/bin/sandbox-exec", "-p", profile, *command],
            cwd=scratch,
            env=environment,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            bufsize=0,
            start_new_session=True,
        )
        stderr_collector = BoundedStderrCollector(process.stderr)
        invocation["original_start_issued"] = True
        record["artifact_execution_started"] = True
        write_json_durable(record_path, record)
        client = StdioClient(process)
        client.send(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {"protocolVersion": 1, "clientCapabilities": {}},
            }
        )
        response = wait_for_original_response(client, 1, evidence, deadline)
        if response is not None:
            error = safe_rpc_error(response)
            if error:
                evidence["initialize_error"] = error
                evidence["initialize"] = error["kind"]
            elif isinstance(response.get("result"), dict):
                evidence["initialize"] = "success"
                evidence["auth_method_ids"] = auth_method_ids(response["result"])
                agent_info = response["result"].get("agentInfo")
                reported = agent_info.get("version") if isinstance(agent_info, dict) else None
                evidence["reported_version_matches"] = reported == version
                evidence["reported_version"] = (
                    reported if reported == version else "mismatch-or-unobserved"
                )
        if evidence["initialize"] == "success" and evidence["reported_version_matches"]:
            client.send(
                {
                    "jsonrpc": "2.0",
                    "id": 2,
                    "method": "session/new",
                    "params": {"cwd": str(scratch), "mcpServers": []},
                }
            )
            response = wait_for_original_response(client, 2, evidence, deadline)
            if response is not None:
                error = safe_rpc_error(response)
                if error:
                    evidence["session_new_error"] = error
                    evidence["session_new"] = error["kind"]
                elif isinstance(response.get("result"), dict):
                    result = response["result"]
                    session_id = result.get("sessionId")
                    evidence["session_new"] = "success" if isinstance(session_id, str) else "result-without-session-id"
                    evidence["auto_session_created"] = isinstance(session_id, str)
                    model_observation = capture_model_observation(result)
                    evidence["model_identity_observation_source"] = model_observation["observation_source"]
                    evidence["underlying_model_identity"] = model_observation["underlying_model_identity"]
                    if model_observation["session_new_current_model_id"]:
                        evidence["model_observations"].append(
                            {
                                "model_id": model_observation["session_new_current_model_id"],
                                "source": model_observation["observation_source"],
                            }
                        )
                    if isinstance(session_id, str):
                        marker_prompt = (
                            "Use the shell tool once to create an empty file at this exact task-scratch path: "
                            f"{marker}. Do not perform any other action. The host will cancel the permission request."
                        )
                        client.send(
                            {
                                "jsonrpc": "2.0",
                                "id": 3,
                                "method": "session/prompt",
                                "params": {
                                    "sessionId": session_id,
                                    "prompt": [{"type": "text", "text": marker_prompt}],
                                },
                            }
                        )
                        evidence["prompt_requests_sent"] = 1
                        evidence["session_prompt"] = "sent"
                        response = wait_for_original_prompt(
                            client, 3, session_id, evidence, deadline
                        )
                        if response is None:
                            evidence["session_prompt"] = "no-result"
                            if not evidence["session_cancel_sent"]:
                                try:
                                    client.send(
                                        {
                                            "jsonrpc": "2.0",
                                            "method": "session/cancel",
                                            "params": {"sessionId": session_id},
                                        }
                                    )
                                    evidence["session_cancel_sent"] = True
                                except (BrokenPipeError, OSError):
                                    evidence["stream_failed"] = True
                        elif safe_rpc_error(response):
                            evidence["prompt_error"] = safe_rpc_error(response)
                            evidence["session_prompt"] = evidence["prompt_error"]["kind"]
                        elif isinstance(response.get("result"), dict):
                            reason = response["result"].get("stopReason")
                            evidence["session_prompt"] = (
                                reason if isinstance(reason, str) and len(reason) <= 40 else "result"
                            )
        if evidence["session_new"] in {"not-reached", "authentication-required", "quota-or-entitlement-stop"}:
            evidence["stop_before_prompt_reason"] = evidence["session_new"]
    except (OSError, RuntimeError, ValueError, TimeoutError, EOFError) as error:
        evidence["launch_error_class"] = type(error).__name__
    finally:
        if client is not None:
            client.close()
        process_exit_code: int | None = None
        forced_stop = False
        process_group_joined = True
        if process is not None:
            process_exit_code, forced_stop, process_group_joined = stop_owned_process_group(
                process, LIVE_CLEANUP_SECONDS
            )
        stderr_reader_joined = (
            stderr_collector.join(LIVE_CLEANUP_SECONDS)
            if stderr_collector is not None
            else True
        )
        evidence["stderr_drain_joined"] = stderr_reader_joined
        evidence["stderr"] = (
            stderr_collector.summary(reader_joined=stderr_reader_joined)
            if stderr_collector is not None
            else {
                "classification": "empty",
                "captured_bytes": 0,
                "total_bytes": 0,
                "total_bytes_capped": False,
                "total_byte_count_limit": MAX_STDERR_COUNTED_BYTES,
                "capture_limit_bytes": MAX_STDERR_CAPTURE_BYTES,
                "truncated": False,
                "reader_joined": True,
                "reader_error": False,
                "raw_persisted": False,
                "raw_displayed": False,
            }
        )
        if process is not None and process.stderr is not None:
            process.stderr.close()
        evidence["process_exit_code"] = process_exit_code
        evidence["launcher_exit_status"] = process_exit_code
        evidence["vendor_startup_status"] = (
            "acp-initialize-response-observed"
            if evidence["initialize"] != "not-reached"
            else "unknown"
        )
        evidence["forced_process_group_stop"] = forced_stop
        evidence["process_joined"] = process is not None and process.returncode is not None
        evidence["process_group_joined"] = process_group_joined
        evidence["effect_marker_present"] = marker.exists()
        elapsed_before_proxy_close = time.monotonic() - started
        remaining_cleanup = max(
            0.0, LIVE_PERMISSION_SECONDS - elapsed_before_proxy_close
        )
        evidence["proxy_threads_joined"] = proxy.stop_and_join(
            min(LIVE_CLEANUP_SECONDS, remaining_cleanup)
        )
        evidence["proxy"] = proxy.snapshot()
        evidence["elapsed_seconds"] = round(time.monotonic() - started, 3)

    evidence["observed_network_audiences_only"] = evidence["proxy"][
        "allowed_destination_hosts"
    ]
    for observation in evidence["model_observations"]:
        model_id = observation.get("model_id")
        if isinstance(model_id, str) and model_id.lower() != "auto":
            evidence["underlying_model_identity"] = model_id
            evidence["model_identity_observation_source"] = observation["source"]
            break
    evidence["underlying_model_identity"] = evidence.get("underlying_model_identity") or "unobserved"
    evidence["vendor_internal_request_count"] = "unobserved"
    evidence["vendor_managed_retry_policy"] = "allowed within one ACP prompt and enforced deadline"
    evidence["harness_prompt_retry_count"] = 0
    evidence["harness_resend_count"] = 0
    evidence["harness_model_fallback_count"] = 0
    evidence["permission_evidence_complete"] = permission_evidence_is_complete(
        evidence, expected_probe_proxy
    )
    evidence["safe_to_continue_older_versions"] = evidence[
        "permission_evidence_complete"
    ]
    invocation["result"] = evidence
    invocation["result_recorded_at_utc"] = utc_now()
    record["permission_boundary_proven_for_all_targets"] = all(
        item.get("result", {}).get("permission_evidence_complete") is True
        for item in record["invocations"]
    )
    write_json_durable(record_path, record)
    return evidence


def require_macos_sandbox() -> str:
    if sys.platform != "darwin":
        raise RuntimeError("the offline containment proof requires macOS sandbox-exec")
    sandbox = "/usr/bin/sandbox-exec"
    if not os.path.isfile(sandbox) or not os.access(sandbox, os.X_OK):
        raise RuntimeError("/usr/bin/sandbox-exec is unavailable")
    return "/bin/bash"


def validate_fake_invocation_record(record: dict[str, Any]) -> None:
    if record.get("schema") != "copilot-cli-acp-authenticated-invocation.v1":
        raise ValueError("unexpected authenticated invocation record schema")
    if record.get("execution_kind") != "fake_control" or record.get("original_artifact_started"):
        raise ValueError("fake invocation record confuses a control with original execution")
    if record.get("pre_execution_fsynced") is not True:
        raise ValueError("fake invocation is missing its durable pre-execution record")
    pre_execution = record.get("pre_execution_record")
    if not isinstance(pre_execution, dict):
        raise ValueError("fake invocation record has no pre-execution object")
    expected_digest = json_digest(pre_execution)
    if record.get("pre_execution_sha256") != expected_digest:
        raise ValueError("fake invocation result does not bind the persisted pre-execution record")
    result = record.get("result")
    if not isinstance(result, dict):
        raise ValueError("fake invocation record has no result")
    if result.get("pre_execution_sha256") != expected_digest:
        raise ValueError("result record does not correlate to the pre-execution record")
    if result.get("pre_execution_record_sha256") != expected_digest:
        raise ValueError("result record is missing its pre-execution record digest")
    required_result_fields = {
        "recorded_provider_selection",
        "underlying_model_identity_and_observation_source",
        "pre_execution_record_sha256",
        "auth_method_observed",
        "permission_request_count_and_safe_shape",
        "permission_response_cancelled_or_rejected",
        "pending_permission_wait_abandoned",
        "late_permission_response_accepted",
        "session_cancel_sent",
        "prompt_terminal_outcome",
        "effect_marker_absent_before_prompt",
        "effect_marker_absent_after_stop",
        "observed_network_audiences_only",
        "retry_resend_fallback_and_tool_attempt_counts",
        "process_exit_status_and_joined_state",
        "forced_process_group_stop",
        "elapsed_seconds",
    }
    if not required_result_fields <= set(result):
        raise ValueError("fake result record is missing required permission or cleanup evidence")
    serialized = json.dumps(record, sort_keys=True)
    for forbidden in (
        "/Users/",
        "/home/",
        "test-only-delegated-capability",
        "github_pat_",
        "ghp_",
        "gho_",
        "ghs_",
        "xoxb-",
        "Bearer ",
    ):
        if forbidden in serialized:
            raise ValueError(f"invocation record contains forbidden secret/path marker {forbidden!r}")


def authenticated_fake_run(
    scratch: Path, host_home: Path, repository_root: Path
) -> tuple[dict[str, Any], dict[str, Any], dict[str, Any]]:
    preflight = authenticated_fake_preflight(scratch, host_home, repository_root)
    credential = fake_delegated_credential(
        {
            "account_ref": FAKE_ACCOUNT_REF,
            "entitlement": "confirmed-in-fake",
            "model_id": FAKE_MODEL_ID,
            "model_policy": "pinned-no-fallback",
            "credential_request": "one-use-copilot-session",
            "audience": FAKE_AUDIENCE,
            "provider_traffic": [FAKE_AUDIENCE],
        }
    )
    if credential is None:
        raise RuntimeError("fake host broker withheld the valid test capability")

    auth_scratch = scratch / "authenticated-fake"
    auth_scratch.mkdir()
    record_path = auth_scratch / "authenticated-invocation.json"
    marker = auth_scratch / "permission-effect-marker"
    pre_execution = {
        "target_identity": {
            "route": "copilot-cli.acp",
            "version": "1.0.80",
            "binary_sha256": "fe779da7dd2342c1d23f0744873fa27d0251eaaee4dc6637fa53093639c0f3c9",
        },
        "executed_identity": {
            "kind": "fake-acp-auth-agent",
            "source_sha256": hashlib.sha256(FAKE_AUTH_AGENT_SOURCE.encode()).hexdigest(),
        },
        "account_access_ref": FAKE_ACCOUNT_REF,
        "entitlement": "synthetic-fake-approval-only",
        "selected_model": FAKE_MODEL_ID,
        "model_policy": "pinned-no-fallback",
        "credential_mechanism": "fake-host-broker-one-use-inherited-pipe",
        "network_audiences": [FAKE_AUDIENCE],
        "containment": "deny-all-network-and-host-home; task-scratch-writes-only",
        "intended_action": "create-one-empty-task-scratch-marker-if-approved",
        "approval_policy": "cancel-permission; never-approve",
        "budgets": {
            "prompts": 1,
            "seconds": MAX_AUTH_FAKE_SECONDS,
            "retries": 0,
            "resends": 0,
            "auto_fallbacks": 0,
            "permission_requests": 1,
            "tool_attempts": 1,
            "effects": 0,
        },
    }
    pre_execution_sha256 = json_digest(pre_execution)
    record = {
        "schema": "copilot-cli-acp-authenticated-invocation.v1",
        "execution_kind": "fake_control",
        "original_artifact_started": False,
        "pre_execution_fsynced": True,
        "pre_execution_sha256": pre_execution_sha256,
        "pre_execution_record": pre_execution,
        "result": None,
    }
    write_json_durable(record_path, record)
    persisted = load_json(record_path)
    if persisted != record or persisted.get("result") is not None:
        raise RuntimeError("fake pre-execution record did not persist before process start")

    started = time.monotonic()
    fake_agent = Path(sys.executable)
    outcome = launch_acp(
        fake_agent,
        [sys.executable, "-c", FAKE_AUTH_AGENT_SOURCE],
        auth_scratch,
        host_home,
        record_path,
        None,
        delegated_credential=credential,
        authenticate_method_id="copilot-login",
        run_timeout_seconds=0.25,
        repository_root=repository_root,
    )
    elapsed = time.monotonic() - started
    report_path = auth_scratch / "fake-agent-report.jsonl"
    if not report_path.is_file() or report_path.stat().st_size > 16384:
        raise RuntimeError("authenticated fake did not write its bounded structured report")
    try:
        diagnostics = [
            json.loads(line)
            for line in report_path.read_text().splitlines()
            if line.strip()
        ]
    except (OSError, json.JSONDecodeError):
        raise RuntimeError("authenticated fake structured report is malformed") from None
    final_agent_report = diagnostics[-1] if diagnostics else {}
    if marker.exists():
        raise RuntimeError("authenticated fake left an action marker after cancellation")
    if outcome.get("initialize") != "success" or outcome.get("session_new") != "success":
        raise RuntimeError("authenticated fake did not accept the delegated in-memory capability")
    if outcome.get("auth_method_ids") != ["copilot-login"]:
        raise RuntimeError("authenticated fake did not report the expected auth method")
    if (
        outcome.get("authenticate") != "success"
        or outcome.get("authenticate_method_selected") != "copilot-login"
    ):
        raise RuntimeError("authenticated fake ACP authenticate exchange did not complete")
    if outcome.get("observed_model_id") != FAKE_MODEL_ID:
        raise RuntimeError("authenticated fake did not report its selected model")
    if not outcome.get("permission_request_observed") or outcome.get("permission_reply") != "cancelled":
        raise RuntimeError("authenticated fake permission was not observed and cancelled")
    if len(outcome.get("permission_methods_observed", [])) != 1:
        raise RuntimeError("authenticated fake exceeded its one-permission-request budget")
    if outcome.get("permission_option_ids") != ["allow_once", "reject_once"]:
        raise RuntimeError("authenticated fake permission options were not safely recorded")
    if not outcome.get("pending_permission_wait_abandoned") or outcome.get("late_permission_response_accepted"):
        raise RuntimeError("authenticated fake kept a pending permission wait or accepted a late reply")
    if outcome.get("session_cancel_sent") is not True or outcome.get("session_prompt") != "cancelled":
        raise RuntimeError("authenticated fake prompt did not stop through ACP cancellation")
    if outcome.get("effect_marker_absent_before_prompt") is not True or outcome.get("effect_marker_present"):
        raise RuntimeError("authenticated fake marker was present before prompt or after stop")
    if outcome.get("credential_environment_variables_absent") is not True:
        raise RuntimeError("authenticated fake delegation leaked into token environment variables")
    if outcome.get("process_joined") is not True or outcome.get("exit_code") != 0:
        raise RuntimeError("authenticated fake child was not cleanly joined")
    if outcome.get("forced_process_group_kill") or elapsed > MAX_AUTH_FAKE_SECONDS:
        raise RuntimeError("authenticated fake exceeded its bounded cleanup window")
    if final_agent_report.get("permission_outcome") != "cancelled" or final_agent_report.get("effect_marker_present"):
        raise RuntimeError("authenticated fake child did not observe cancellation without an effect")

    record["result"] = {
        "pre_execution_sha256": pre_execution_sha256,
        "pre_execution_record_sha256": pre_execution_sha256,
        "recorded_provider_selection": outcome["observed_model_id"],
        "underlying_model_identity_and_observation_source": (
            "fake currentModelId from session/new"
        ),
        "auth_method_observed": {
            "advertised_ids": outcome["auth_method_ids"],
            "selected_method_id": outcome["authenticate_method_selected"],
            "result": outcome["authenticate"],
        },
        "permission_request_count_and_safe_shape": {
            "count": len(outcome["permission_methods_observed"]),
            "method": outcome["permission_methods_observed"][0],
            "option_ids": outcome["permission_option_ids"],
        },
        "permission_response_cancelled_or_rejected": outcome["permission_reply"],
        "pending_permission_wait_abandoned": outcome["pending_permission_wait_abandoned"],
        "late_permission_response_accepted": outcome["late_permission_response_accepted"],
        "session_cancel_sent": outcome["session_cancel_sent"],
        "prompt_terminal_outcome": outcome["session_prompt"],
        "effect_marker_absent_before_prompt": outcome["effect_marker_absent_before_prompt"],
        "effect_marker_absent_after_stop": not outcome["effect_marker_present"],
        "observed_network_audiences_only": [],
        "retry_resend_fallback_and_tool_attempt_counts": {
            "prompts": 1,
            "retries": 0,
            "resends": 0,
            "auto_fallbacks": 0,
            "permission_requests": len(outcome["permission_methods_observed"]),
            "tool_attempts": 1,
            "effects": 0,
        },
        "process_exit_status_and_joined_state": {
            "exit_code": outcome["exit_code"],
            "joined": outcome["process_joined"],
        },
        "forced_process_group_stop": outcome["forced_process_group_kill"],
        "elapsed_seconds": round(elapsed, 3),
    }
    write_json_durable(record_path, record)
    persisted = load_json(record_path)
    validate_fake_invocation_record(persisted)
    discovery_result = discovery_fake_run(
        scratch, host_home, repository_root, credential
    )
    return preflight, persisted["result"], discovery_result


def discovery_fake_run(
    scratch: Path,
    host_home: Path,
    repository_root: Path,
    credential: bytes,
) -> dict[str, Any]:
    discovery_scratch = scratch / "authenticated-discovery-fake"
    discovery_scratch.mkdir()
    record_path = discovery_scratch / "discovery-invocation.json"
    pre_execution = {
        "target_identity": {
            "kind": "fake-acp-discovery-agent",
            "source_sha256": hashlib.sha256(
                FAKE_DISCOVERY_AGENT_SOURCE.encode()
            ).hexdigest(),
        },
        "account_access_ref": FAKE_ACCOUNT_REF,
        "credential_mechanism": "fake-host-broker-one-use-inherited-pipe",
        "allowed_acp_requests": sorted(DISCOVERY_REQUEST_METHODS),
        "endpoint_allowlist": [
            "https://github.fake.test",
            "https://copilot.fake.test",
        ],
        "containment": "deny-network-host-home-repository-and-keychain; scratch-writes-only",
        "budgets": {"invocations": 1, "seconds": 60, "prompts": 0},
    }
    digest = json_digest(pre_execution)
    record = {
        "schema": "copilot-cli-acp-authenticated-invocation.v1",
        "execution_kind": "fake_control",
        "original_artifact_started": False,
        "pre_execution_fsynced": True,
        "pre_execution_sha256": digest,
        "pre_execution_record": pre_execution,
        "result": None,
    }
    write_json_durable(record_path, record)
    if load_json(record_path) != record:
        raise RuntimeError("discovery fake plan was not durable before child start")

    started = time.monotonic()
    outcome = launch_acp(
        Path(sys.executable),
        [sys.executable, "-c", FAKE_DISCOVERY_AGENT_SOURCE],
        discovery_scratch,
        host_home,
        record_path,
        None,
        delegated_credential=credential,
        authenticate_method_id="copilot-login",
        mode="pre-prompt-discovery",
        repository_root=repository_root,
    )
    elapsed = time.monotonic() - started
    if (
        outcome.get("initialize") != "success"
        or outcome.get("authenticate") != "success"
        or outcome.get("session_new") != "success"
    ):
        raise RuntimeError("discovery fake did not complete initialize/authenticate/session-new")
    if outcome.get("session_prompt") != "not-authorized-not-sent":
        raise RuntimeError("discovery fake did not stop before session/prompt")
    observation = outcome.get("discovery_observation")
    if not isinstance(observation, dict) or observation.get("available_model_ids") != [
        "fake-auto",
        "fake-gpt-5.4",
    ]:
        raise RuntimeError("discovery fake did not retain its synthetic model identifiers")
    if observation.get("current_model_id") != "fake-auto":
        raise RuntimeError("discovery fake did not retain its synthetic current model")
    if observation.get("model_config_option") != {
        "option_id": "model",
        "current_value": "fake-auto",
        "available_values": ["fake-auto", "fake-gpt-5.4"],
    }:
        raise RuntimeError("discovery fake did not retain its synthetic model config surface")
    if outcome.get("permission_request_observed") or outcome.get("tool_call_updates"):
        raise RuntimeError("discovery fake observed a permission or tool effect")
    if outcome.get("effect_marker_present") or not outcome.get("process_joined"):
        raise RuntimeError("discovery fake did not close cleanly without an effect")
    if outcome.get("forced_process_group_kill") or elapsed > MAX_AUTH_FAKE_SECONDS:
        raise RuntimeError("discovery fake exceeded its joined cleanup budget")

    record["result"] = {
        "pre_execution_record_sha256": digest,
        "outbound_methods": ["initialize", "authenticate", "session/new"],
        "prompt_requests": 0,
        "permission_callbacks": 0,
        "tool_callbacks": 0,
        "model_observation": observation,
        "effect_marker_absent_after_close": not outcome["effect_marker_present"],
        "process_joined": outcome["process_joined"],
        "elapsed_seconds": round(elapsed, 3),
    }
    write_json_durable(record_path, record)
    validate_fake_discovery_record(load_json(record_path))
    return record["result"]


def validate_fake_discovery_record(record: dict[str, Any]) -> None:
    if (
        record.get("schema") != "copilot-cli-acp-authenticated-invocation.v1"
        or record.get("execution_kind") != "fake_control"
        or record.get("original_artifact_started") is not False
        or record.get("pre_execution_fsynced") is not True
    ):
        raise ValueError("discovery fake record is not a durable fake-only control")
    pre_execution = record.get("pre_execution_record")
    result = record.get("result")
    if not isinstance(pre_execution, dict) or not isinstance(result, dict):
        raise ValueError("discovery fake record is missing its plan or result")
    expected = json_digest(pre_execution)
    if record.get("pre_execution_sha256") != expected:
        raise ValueError("discovery fake result is not bound to its pre-execution plan")
    if result.get("pre_execution_record_sha256") != expected:
        raise ValueError("discovery fake result is not correlated to its plan")
    if result.get("outbound_methods") != ["initialize", "authenticate", "session/new"]:
        raise ValueError("discovery fake used an unapproved ACP operation")
    if result.get("prompt_requests") != 0:
        raise ValueError("discovery fake sent a prompt")
    if result.get("permission_callbacks") != 0 or result.get("tool_callbacks") != 0:
        raise ValueError("discovery fake observed a permission or tool callback")
    if result.get("effect_marker_absent_after_close") is not True:
        raise ValueError("discovery fake produced an action effect")
    serialized = json.dumps(record, sort_keys=True)
    for forbidden in (
        "/Users/",
        "/home/",
        "test-only-delegated-capability",
        "github_pat_",
        "ghp_",
        "gho_",
        "ghs_",
        "xoxb-",
        "Bearer ",
    ):
        if forbidden in serialized:
            raise ValueError(f"discovery fake record contains forbidden marker {forbidden!r}")


def original_child_environment(
    host_home: Path, scratch: Path, proxy_port: int
) -> dict[str, str]:
    proxy_url = f"http://127.0.0.1:{proxy_port}"
    environment = {
        "HOME": str(host_home.resolve()),
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "TMPDIR": str((scratch / "tmp").resolve()),
        "TERM": "dumb",
        "COPILOT_AUTO_UPDATE": "false",
        "HTTP_PROXY": proxy_url,
        "HTTPS_PROXY": proxy_url,
        "ALL_PROXY": proxy_url,
        "http_proxy": proxy_url,
        "https_proxy": proxy_url,
        "all_proxy": proxy_url,
    }
    for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN", "COPILOT_HOME"):
        if name in environment:
            raise RuntimeError(f"original environment unexpectedly contains {name}")
    return environment


def process_group_exists(process_group_id: int) -> bool:
    try:
        os.killpg(process_group_id, 0)
        return True
    except ProcessLookupError:
        return False
    except PermissionError:
        return True


def stop_owned_process_group(
    process: subprocess.Popen[bytes], timeout: float = LIVE_CLEANUP_SECONDS
) -> tuple[int | None, bool, bool]:
    if process.stdin is not None:
        try:
            process.stdin.close()
        except OSError:
            pass
    process_group_id = process.pid
    deadline = time.monotonic() + timeout
    forced = False
    try:
        process.wait(timeout=min(0.1, max(0.0, deadline - time.monotonic())))
    except subprocess.TimeoutExpired:
        pass
    if process_group_exists(process_group_id):
        try:
            os.killpg(process_group_id, signal.SIGTERM)
        except ProcessLookupError:
            pass
        except PermissionError:
            # The exact Popen child is ours even when the host refuses group
            # signalling. Its sandbox denies process-fork, so signal that PID
            # directly and still require the process group to be empty below.
            try:
                process.send_signal(signal.SIGTERM)
            except ProcessLookupError:
                pass
    try:
        process.wait(timeout=min(0.8, max(0.0, deadline - time.monotonic())))
    except subprocess.TimeoutExpired:
        forced = True
    if process_group_exists(process_group_id):
        forced = True
        try:
            os.killpg(process_group_id, signal.SIGKILL)
        except ProcessLookupError:
            pass
        except PermissionError:
            try:
                process.kill()
            except ProcessLookupError:
                pass
    try:
        process.wait(timeout=max(0.0, deadline - time.monotonic()))
    except subprocess.TimeoutExpired:
        pass
    while process_group_exists(process_group_id) and time.monotonic() < deadline:
        time.sleep(min(0.05, deadline - time.monotonic()))
    return process.poll(), forced, not process_group_exists(process_group_id)


def run_native_stderr_control(
    executable: Path,
    mode: str,
    profile: str,
    environment: dict[str, str],
    cwd: Path,
) -> dict[str, Any]:
    process = subprocess.Popen(
        ["/usr/bin/sandbox-exec", "-p", profile, str(executable), mode],
        cwd=cwd,
        env=environment,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        bufsize=0,
        start_new_session=True,
    )
    collector = BoundedStderrCollector(process.stderr)
    forced = False
    process_group_joined = True
    try:
        if mode == "--hang":
            time.sleep(0.1)
            exit_code, forced, process_group_joined = stop_owned_process_group(
                process, LIVE_CLEANUP_SECONDS
            )
        else:
            try:
                exit_code = process.wait(timeout=3.0)
            except subprocess.TimeoutExpired:
                exit_code, forced, process_group_joined = stop_owned_process_group(
                    process, 1.0
                )
    finally:
        if process.poll() is None:
            _, forced_stop, joined = stop_owned_process_group(process, 1.0)
            forced = forced or forced_stop
            process_group_joined = process_group_joined and joined
    stdout_marker = process.stdout.read(128) if process.stdout is not None else b""
    if process.stdout is not None:
        process.stdout.close()
    reader_joined = collector.join(LIVE_CLEANUP_SECONDS)
    stderr_summary = collector.summary(reader_joined=reader_joined)
    if process.stderr is not None:
        process.stderr.close()
    return {
        "mode": mode,
        "fake_process_startup": (
            "observed" if stdout_marker == f"FAKE_NATIVE_STARTED:{mode}\n".encode() else "unknown"
        ),
        "launcher_exit_status": exit_code,
        "hard_deadline_forced_stop": forced,
        "process_group_joined": process_group_joined,
        "stderr": stderr_summary,
    }


def run_permission_boundary_fake(
    scratch: Path,
    host_home: Path,
    repository_root: Path,
    permission_plan: dict[str, Any],
    correction_plan: dict[str, Any],
) -> dict[str, Any]:
    proof_scratch = scratch / "host-login-permission-fake"
    record_dir = proof_scratch / "records"
    action_scratch = proof_scratch / "action"
    action_scratch.mkdir(parents=True)
    (action_scratch / "tmp").mkdir()
    record_dir.mkdir(parents=True)
    marker = action_scratch / "permission-effect-marker"
    diagnostic_path = action_scratch / "fake-child-diagnostic.json"
    executable = action_scratch / "copilot-permission-fake"
    executable_identity = compile_permission_fake(executable)
    proxy = CopilotEgressProxy(
        set(permission_plan["network_policy"]["allowed_hosts"]),
        tuple(permission_plan["network_policy"]["allowed_subdomains"]),
        time.monotonic() + 10.0,
        fake_only=True,
    )
    try:
        proxy_port = int(proxy.server_address[1])
    except OSError as error:
        raise RuntimeError(
            f"host-login fake could not bind its local proxy (errno={error.errno})"
        ) from None

    profile = permission_sandbox_profile(
        action_scratch,
        record_dir,
        host_home,
        repository_root,
        executable,
        proxy_port,
        permission_plan,
        correction_plan,
    )
    if '(allow mach-lookup (global-name "com.apple.securityd"))' not in profile:
        raise RuntimeError("host-login sandbox omitted the approved securityd lookup")
    if "(allow process*)" in profile or "(allow network*)" in profile:
        raise RuntimeError("host-login sandbox grants broad process or network access")
    if '(allow file-map-executable (subpath "/System/Library"))' not in profile:
        raise RuntimeError("host-login sandbox omitted the system runtime root")
    if '(allow file-map-executable (subpath "/usr/lib"))' not in profile:
        raise RuntimeError("host-login sandbox omitted the system runtime root")
    if correction_plan["sandbox_delta"]["fake_runtime_roots"] != []:
        raise RuntimeError("host-login sandbox correction contains fake-only runtime roots")
    if "(deny network-inbound)" not in profile or "(deny network-bind)" not in profile:
        raise RuntimeError("host-login sandbox does not deny inbound or bound sockets")
    if f'(allow network-outbound (remote ip "localhost:{proxy_port}"))' not in profile:
        raise RuntimeError("host-login sandbox does not bind outbound traffic to the tested proxy")
    approved_path = host_home / correction_plan["auth_metadata_read_paths"][0]["relative_path"]
    expected_exception = (
        f"(deny file-read* (require-all (subpath {quote_profile_path(host_home)}) "
        f"(require-not (literal {quote_literal_policy_path(approved_path)}))))"
    )
    if expected_exception not in profile:
        raise RuntimeError("host-login sandbox metadata exception differs from the correction plan")
    if "(deny file-write* (subpath " + quote_profile_path(host_home) + "))" not in profile:
        raise RuntimeError("host-login sandbox does not deny host-home writes")

    environment = original_child_environment(host_home, action_scratch, proxy_port)
    environment.update(
        {
            "SWALLOWTAIL_BLOCKED_REPOSITORY": str(repository_root.resolve()),
            "SWALLOWTAIL_EFFECT_MARKER": str(marker),
            "SWALLOWTAIL_PROXY_PORT": str(proxy_port),
            "SWALLOWTAIL_FAKE_DIAGNOSTIC": str(diagnostic_path),
        }
    )
    early_exit_control = run_native_stderr_control(
        executable, "--early-exit", profile, environment, action_scratch
    )
    if (
        early_exit_control["fake_process_startup"] != "observed"
        or early_exit_control["launcher_exit_status"] != 41
    ):
        summary = early_exit_control["stderr"]
        raise RuntimeError(
            "native fake could not start under the shared profile "
            f"(exit={early_exit_control['launcher_exit_status']}, "
            f"stderr={summary['classification']}, bytes={summary['total_bytes']})"
        )

    plan_digest = hashlib.sha256(PERMISSION_PROOF_PLAN_PATH.read_bytes()).hexdigest()
    correction_digest = hashlib.sha256(PERMISSION_CORRECTION_PLAN_PATH.read_bytes()).hexdigest()
    profile_digest = hashlib.sha256(profile.encode()).hexdigest()
    pre_execution = {
        "target": "task-owned-native-fake; no original artifact",
        "fake_source": "scripts/copilot-acp-permission-fake.c",
        "fake_source_sha256": executable_identity["source_sha256"],
        "fake_binary_sha256": executable_identity["binary_sha256"],
        "permission_plan_sha256": plan_digest,
        "correction_plan_sha256": correction_digest,
        "model_policy": "--model auto fake argument only",
        "credential_boundary": "synthetic metadata sentinel only; no token environment or real auth-store read",
        "auth_metadata_read_exception": ".copilot/config.json; read-only; current official docs, not frozen-artifact-specific",
        "network_allowlist": sorted(permission_plan["network_policy"]["allowed_hosts"]),
        "network_subdomains": list(permission_plan["network_policy"]["allowed_subdomains"]),
        "network_default": "deny except local tested CONNECT proxy",
        "profile_sha256": profile_digest,
        "fake_runtime_roots": correction_plan["sandbox_delta"]["fake_runtime_roots"],
        "action": "cancel one permission request before marker creation",
        "budgets": {"prompts": 1, "seconds": 3, "effects": 0},
    }
    fake_record_path = record_dir / "invocation.json"
    fake_record = {
        "schema": "copilot-cli-acp-permission-proof-preflight-invocation.v2",
        "execution_kind": "fake_control",
        "original_artifact_started": False,
        "pre_execution_fsynced": True,
        "pre_execution_sha256": json_digest(pre_execution),
        "pre_execution_record": pre_execution,
        "result": None,
    }
    secret_free_serialization(fake_record)
    write_json_durable(fake_record_path, fake_record)
    if load_json(fake_record_path) != fake_record:
        proxy.stop_and_join(LIVE_CLEANUP_SECONDS)
        raise RuntimeError("host-login fake pre-execution record did not persist before child start")

    started = time.monotonic()
    try:
        proxy.start()
    except OSError as error:
        proxy.server_close()
        raise RuntimeError(
            f"host-login fake could not start its local proxy (errno={error.errno})"
        ) from None
    command = [
        "/usr/bin/sandbox-exec",
        "-p",
        profile,
        str(executable),
        "--model",
        "auto",
        "--acp",
        "--stdio",
    ]
    try:
        process = subprocess.Popen(
            command,
            cwd=action_scratch,
            env=environment,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            bufsize=0,
            start_new_session=True,
        )
    except OSError as error:
        proxy.stop_and_join(LIVE_CLEANUP_SECONDS)
        raise RuntimeError(
            f"host-login native fake could not start in its sandbox (errno={error.errno})"
        ) from None
    collector = BoundedStderrCollector(process.stderr)
    client = StdioClient(process)
    permission_count = 0
    pending_wait_abandoned = False
    late_permission_response_accepted = False
    session_cancel_sent = False
    prompt_result: dict[str, Any] | None = None
    fake_startup_observed = False
    try:
        try:
            startup_marker = client.receive(3.0)
            if startup_marker.get("method") != "swallowtail/native-fake-started":
                raise RuntimeError("native fake startup marker did not match its test protocol")
            fake_startup_observed = True
            client.send(
                {
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "initialize",
                    "params": {"protocolVersion": 1, "clientCapabilities": {}},
                }
            )
            initialize = client.receive(3.0)
        except EOFError:
            try:
                diagnostic = load_json(diagnostic_path)
            except (OSError, ValueError, json.JSONDecodeError):
                diagnostic = {}
            failed_controls = [
                name
                for name in (
                    "auth_metadata_read",
                    "auth_metadata_write_denied",
                    "adjacent_home_reads_denied",
                    "keychain_file_read_denied",
                    "repository_read_denied",
                    "repository_write_denied",
                    "direct_network_denied",
                    "inbound_bind_denied",
                    "shell_exec_denied",
                    "proxy_allowlist_exercised",
                    "proxy_sni_mismatch_exercised",
                    "proxy_unlisted_denied",
                    "effect_marker_absent",
                )
                if diagnostic.get(name) is not True
            ]
            stderr_reader_joined = collector.join(0.1)
            stderr_summary = collector.summary(reader_joined=stderr_reader_joined)
            raise RuntimeError(
                "native fake exited before ACP initialize "
                f"(exit={process.poll()}, stage={diagnostic.get('stage', 'unknown')}, "
                f"failed_controls={','.join(failed_controls) or 'unclassified'}, "
                f"stderr={stderr_summary['classification']}, bytes={stderr_summary['total_bytes']})"
            ) from None
        if initialize.get("id") != 1 or initialize.get("result", {}).get("agentInfo", {}).get("version") != "1.0.93":
            raise RuntimeError("host-login native fake did not initialize as its pinned control")
        client.send(
            {
                "jsonrpc": "2.0",
                "id": 2,
                "method": "session/new",
                "params": {"cwd": str(action_scratch), "mcpServers": []},
            }
        )
        session = client.receive(3.0)
        if session.get("id") != 2 or not isinstance(session.get("result", {}).get("sessionId"), str):
            raise RuntimeError("host-login native fake did not create its synthetic session")
        client.send(
            {
                "jsonrpc": "2.0",
                "id": 3,
                "method": "session/prompt",
                "params": {
                    "sessionId": session["result"]["sessionId"],
                    "prompt": [{"type": "text", "text": "Create the empty marker in task scratch"}],
                },
            }
        )
        deadline = time.monotonic() + 3.0
        while time.monotonic() < deadline:
            message = client.receive(deadline - time.monotonic())
            if message.get("method") == "session/request_permission":
                permission_count += 1
                permission_id = message.get("id")
                if permission_count != 1 or not isinstance(permission_id, int):
                    raise RuntimeError("host-login native fake produced an ambiguous permission callback")
                wait = PendingPermissionWait()
                wait.abandon()
                pending_wait_abandoned = wait.state == "abandoned"
                late_permission_response_accepted = wait.resolve("allow_once")
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "id": permission_id,
                        "result": {"outcome": {"outcome": "cancelled"}},
                    }
                )
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "method": "session/cancel",
                        "params": {"sessionId": session["result"]["sessionId"]},
                    }
                )
                session_cancel_sent = True
                continue
            if message.get("id") == 3:
                prompt_result = message
                break
        if prompt_result is None:
            raise RuntimeError("host-login native fake did not finish its cancelled prompt")
        if prompt_result.get("result", {}).get("stopReason") != "cancelled":
            raise RuntimeError("host-login native fake prompt did not report cancellation")
    finally:
        client.close()
        exit_code, forced, process_group_joined = stop_owned_process_group(process)
        stderr_reader_joined = collector.join(LIVE_CLEANUP_SECONDS)
        stderr_summary = collector.summary(reader_joined=stderr_reader_joined)
        if process.stderr is not None:
            process.stderr.close()
        proxy_joined = proxy.stop_and_join(LIVE_CLEANUP_SECONDS)

    child_diagnostic = load_json(diagnostic_path) if diagnostic_path.is_file() else {}
    if child_diagnostic.get("stage") != "policy-probes-passed":
        raise RuntimeError(
            "host-login native fake policy probes failed "
            f"(startup={fake_startup_observed}, stderr={stderr_summary['classification']}, "
            f"bytes={stderr_summary['total_bytes']})"
        )
    snapshot = proxy.snapshot()
    if permission_count != 1 or not pending_wait_abandoned or late_permission_response_accepted:
        raise RuntimeError("host-login native fake did not enforce one cancelled permission request")
    if not session_cancel_sent or marker.exists():
        raise RuntimeError("host-login native fake cancellation caused an effect or missed session/cancel")
    if exit_code != -signal.SIGKILL or not forced or not process_group_joined or not proxy_joined:
        raise RuntimeError("host-login native fake did not exercise bounded forced cleanup and join")
    if not stderr_reader_joined or stderr_summary["reader_error"]:
        raise RuntimeError("host-login native fake stderr reader did not join cleanly")
    if snapshot != {
        "allowed_destination_hosts": ["api.github.com"],
        "unlisted_destination_count": 1,
        "sni_mismatch_count": 1,
        "connection_failure_count": 0,
    }:
        raise RuntimeError("host-login native fake egress controls differ")

    stderr_controls = {
        mode: run_native_stderr_control(executable, mode, profile, environment, action_scratch)
        for mode in ("--stderr-fullpipe", "--stderr-secret", "--hang")
    }
    stderr_controls["--early-exit"] = early_exit_control
    fullpipe = stderr_controls["--stderr-fullpipe"]
    secret = stderr_controls["--stderr-secret"]
    early_exit = stderr_controls["--early-exit"]
    hang = stderr_controls["--hang"]
    if (
        not fullpipe["stderr"]["total_bytes_capped"]
        or fullpipe["stderr"]["total_bytes"] != MAX_STDERR_COUNTED_BYTES
        or not fullpipe["stderr"]["truncated"]
    ):
        raise RuntimeError("native fake stderr full-pipe control did not exceed its bounded capture")
    if secret["stderr"]["classification"] != "unknown":
        raise RuntimeError("native fake secret-bearing stderr did not fail closed to unknown")
    if early_exit["fake_process_startup"] != "observed" or early_exit["launcher_exit_status"] != 41:
        raise RuntimeError("native fake early-exit control lost its fake startup evidence")
    if hang["fake_process_startup"] != "observed" or not hang["hard_deadline_forced_stop"]:
        raise RuntimeError("native fake timeout control did not force a bounded stop")
    if not all(
        control["process_group_joined"] and control["stderr"]["reader_joined"]
        for control in stderr_controls.values()
    ):
        modes = [
            mode
            for mode, control in stderr_controls.items()
            if not control["process_group_joined"] or not control["stderr"]["reader_joined"]
        ]
        raise RuntimeError(
            "native fake stderr controls left a process or reader unjoined "
            f"(modes={','.join(modes)})"
        )
    if any(control["stderr"]["reader_error"] for control in stderr_controls.values()):
        raise RuntimeError("native fake stderr control reader reported a failure")

    sandbox_policy = {
        "auth_metadata_read": child_diagnostic.get("auth_metadata_read") is True,
        "auth_metadata_write_denied": child_diagnostic.get("auth_metadata_write_denied") is True,
        "adjacent_home_reads_denied": child_diagnostic.get("adjacent_home_reads_denied") is True,
        "keychain_file_read_denied": child_diagnostic.get("keychain_file_read_denied") is True,
        "repository_reads_denied": child_diagnostic.get("repository_read_denied") is True,
        "repository_writes_denied": child_diagnostic.get("repository_write_denied") is True,
        "direct_egress_denied": child_diagnostic.get("direct_network_denied") is True,
        "inbound_bind_denied": child_diagnostic.get("inbound_bind_denied") is True,
        "shell_exec_denied": child_diagnostic.get("shell_exec_denied") is True,
        "fake_runtime_roots": correction_plan["sandbox_delta"]["fake_runtime_roots"],
        "securityd_trust_boundary": "permitted service lookup; no item-level mediation or read performed",
    }
    valid_evidence = {
        "initialize": "success",
        "reported_version_matches": True,
        "session_new": "success",
        "auto_session_created": True,
        "pre_prompt_permission_request": False,
        "prompt_requests_sent": 1,
        "credential_environment_variables_absent": not any(
            name in environment
            for name in ("COPILOT_GITHUB_TOKEN", "GH_TOKEN", "GITHUB_TOKEN")
        ),
        "permission_request_count": permission_count,
        "permission_reply": "cancelled",
        "permission_callback_ambiguous": False,
        "late_permission_response_accepted": late_permission_response_accepted,
        "session_cancel_sent": session_cancel_sent,
        "session_prompt": "cancelled",
        "effect_marker_present": marker.exists(),
        "process_joined": process.returncode is not None,
        "process_group_joined": process_group_joined,
        "stderr_drain_joined": stderr_reader_joined,
        "stderr": stderr_summary,
        "proxy_threads_joined": proxy_joined,
        "elapsed_seconds": 1.0,
        "proxy": {
            "allowed_destination_hosts": ["api.github.com"],
            "unlisted_destination_count": 0,
            "sni_mismatch_count": 0,
            "connection_failure_count": 0,
        },
    }
    negative_controls = {
        "missing_permission_rejected": not permission_evidence_is_complete(
            {**valid_evidence, "permission_request_count": 0}
        ),
        "late_approval_rejected": not permission_evidence_is_complete(
            {**valid_evidence, "late_permission_response_accepted": True}
        ),
        "effect_marker_rejected": not permission_evidence_is_complete(
            {**valid_evidence, "effect_marker_present": True}
        ),
        "unlisted_egress_rejected": not permission_evidence_is_complete(
            {
                **valid_evidence,
                "proxy": {
                    **valid_evidence["proxy"],
                    "unlisted_destination_count": 1,
                },
            }
        ),
        "sni_mismatch_rejected": not permission_evidence_is_complete(
            {
                **valid_evidence,
                "proxy": {
                    **valid_evidence["proxy"],
                    "sni_mismatch_count": 1,
                },
            }
        ),
        "unjoined_stderr_rejected": not permission_evidence_is_complete(
            {**valid_evidence, "stderr_drain_joined": False}
        ),
    }
    if not permission_evidence_is_complete(valid_evidence) or not all(negative_controls.values()):
        raise RuntimeError("host-login native fake permission acceptance controls failed")

    fake_record["result"] = {
        "fake_process_startup": "observed" if fake_startup_observed else "unknown",
        "sandbox_profile_sha256": profile_digest,
        "sandbox_policy": sandbox_policy,
        "stderr": stderr_summary,
        "stderr_controls": stderr_controls,
        "prompt_requests": 1,
        "permission_requests": permission_count,
        "permission_reply": "cancelled",
        "pending_wait_abandoned": pending_wait_abandoned,
        "late_approval_accepted": late_permission_response_accepted,
        "session_cancel_sent": session_cancel_sent,
        "effect_marker_absent": not marker.exists(),
        "egress": snapshot,
        "process_group_joined": process_group_joined,
        "forced_cleanup_verified": forced,
        "stderr_reader_joined": stderr_reader_joined,
        "proxy_threads_joined": proxy_joined,
        "negative_controls": negative_controls,
    }
    write_json_durable(fake_record_path, fake_record)
    secret_free_serialization(load_json(fake_record_path))
    result = {
        "pre_execution_record_fsynced_before_child": True,
        "pre_execution_record_sha256": fake_record["pre_execution_sha256"],
        "permission_plan_sha256": plan_digest,
        "correction_plan_sha256": correction_digest,
        "fake_source_sha256": executable_identity["source_sha256"],
        "fake_binary_sha256": executable_identity["binary_sha256"],
        "fake_process_startup": fake_record["result"]["fake_process_startup"],
        "sandbox_profile_sha256": profile_digest,
        "sandbox_profile": "exact fake executable; system runtime roots only; one read-only auth metadata literal; scratch-only writes; local reviewed proxy; securityd lookup trust boundary",
        "sandbox_policy": sandbox_policy,
        "network": snapshot,
        "permission_requests": permission_count,
        "permission_reply": "cancelled",
        "pending_wait_abandoned": pending_wait_abandoned,
        "late_approval_accepted": late_permission_response_accepted,
        "session_cancel_sent": session_cancel_sent,
        "effect_marker_absent": not marker.exists(),
        "child_exit_code": exit_code,
        "forced_cleanup_verified": forced,
        "process_group_joined": process_group_joined,
        "stderr": stderr_summary,
        "stderr_controls": stderr_controls,
        "stderr_reader_joined": stderr_reader_joined,
        "proxy_threads_joined": proxy_joined,
        "negative_controls": negative_controls,
        "elapsed_seconds": round(time.monotonic() - started, 3) if "started" in locals() else 1.0,
    }
    return result


def self_test() -> dict[str, Any]:
    fake_shell = require_macos_sandbox()
    plan = validate_authenticated_plan()
    permission_plan = validate_permission_proof_plan()
    correction_plan = validate_permission_correction_plan(permission_plan)
    with tempfile.TemporaryDirectory(prefix="copilot-acp-offline-proof-") as temp_root:
        task_root = Path(temp_root).resolve()
        scratch = task_root / "task-scratch"
        scratch.mkdir()
        host_home = task_root / "fake-host-home"
        (host_home / ".copilot").mkdir(parents=True)
        (host_home / ".ssh").mkdir()
        (host_home / "Library" / "Keychains").mkdir(parents=True)
        repository_root = task_root / "fake-repository"
        repository_root.mkdir()
        for path, contents in (
            (
                host_home / ".copilot" / "config.json",
                b"synthetic-auth-metadata="
                + b"SWALLOWTAIL_FAKE_AUTH_METADATA_SENTINEL\n",
            ),
            (host_home / ".copilot" / "settings.json", b"fake settings\n"),
            (host_home / ".ssh" / "id_ed25519", b"synthetic unrelated home file\n"),
            (host_home / "Library" / "Keychains" / "login.keychain-db", b"fake keychain\n"),
            (host_home / "Library" / "Keychains" / "copilot-cli.fake", b"fake item\n"),
            (repository_root / "README.md", b"fake repository\n"),
        ):
            path.write_bytes(contents)
        record_path = scratch / "pre-execution-record.json"
        record = {
            "schema": "copilot-cli-acp-offline-execution-record.v1",
            "artifact_execution_started": False,
            "pre_execution_record_persisted": True,
        }
        write_json_durable(record_path, record)
        if load_json(record_path) != record:
            raise RuntimeError("pre-execution record did not persist before fake launch")
        fake_path = scratch / "fake-copilot-acp.py"
        fake_path.write_text(FAKE_AGENT_SOURCE)
        fake_path.chmod(0o700)
        profile = sandbox_profile(scratch, host_home, repository_root)
        if "(deny network*)" not in profile or "(allow network" in profile:
            raise RuntimeError("sandbox profile does not deny every network operation")
        if f"(allow file-write* (subpath {quote_profile_path(scratch)}))" not in profile:
            raise RuntimeError("sandbox profile does not bind writes to task scratch")
        if f"(deny file-read* (subpath {quote_profile_path(host_home)}))" not in profile:
            raise RuntimeError("sandbox profile does not deny the host home")
        if f"(deny file-read* (subpath {quote_profile_path(repository_root)}))" not in profile:
            raise RuntimeError("sandbox profile does not deny the task repository")
        outcome = launch_acp(
            Path(fake_shell),
            [fake_shell, str(fake_path)],
            scratch,
            host_home,
            record_path,
            None,
            repository_root=repository_root,
        )
        if outcome.get("initialize") != "success":
            raise RuntimeError("fake ACP initialize did not complete")
        if outcome.get("session_new") != "success":
            raise RuntimeError(f"fake ACP session/new did not complete: {json.dumps(outcome, sort_keys=True)}")
        if outcome.get("session_prompt") != "cancelled":
            raise RuntimeError("fake ACP permission denial did not cancel the prompt")
        if not outcome.get("permission_request_observed"):
            raise RuntimeError("fake ACP permission request was not observed")
        if outcome.get("permission_reply") != "cancelled":
            raise RuntimeError("fake ACP permission request was not cancelled")
        if not outcome.get("pending_permission_wait_abandoned"):
            raise RuntimeError("fake ACP pending permission wait was not abandoned")
        if outcome.get("late_permission_response_accepted"):
            raise RuntimeError("fake ACP accepted a late permission response")
        if outcome.get("effect_marker_present"):
            raise RuntimeError("fake ACP rejection allowed the tool effect")
        if (
            outcome.get("exit_code") != 0
            or outcome.get("forced_process_group_kill")
            or not outcome.get("process_joined")
        ):
            raise RuntimeError(
                "fake ACP agent did not exit cleanly "
                f"(exit={outcome.get('exit_code')}, "
                f"forced={outcome.get('forced_process_group_kill')}, "
                f"joined={outcome.get('process_joined')}, "
                f"initialize={outcome.get('initialize')}, "
                f"session_new={outcome.get('session_new')}, "
                f"prompt={outcome.get('session_prompt')}, "
                f"permission={outcome.get('permission_reply')}, "
                f"cancel_sent={outcome.get('session_cancel_sent')})"
            )
        auth_preflight, auth_result, discovery_result = authenticated_fake_run(
            scratch, host_home, repository_root
        )
        permission_boundary_result = run_permission_boundary_fake(
            scratch,
            host_home,
            repository_root,
            permission_plan,
            correction_plan,
        )
        consumed_versions = consumed_permission_versions()
        if "1.0.93" not in consumed_versions:
            raise RuntimeError(
                "cross-run budget fake did not load the consumed 1.0.93 invocation"
            )
        try:
            enforce_permission_invocation_budget(permission_plan)
        except RuntimeError as error:
            if (
                "1.0.93" not in str(error)
                or "separate authority is required" not in str(error)
            ):
                raise
        else:
            raise RuntimeError(
                "cross-run budget fake did not refuse the consumed 1.0.93 invocation"
            )
        legacy_record_path = task_root / "legacy-execution-record.json"
        legacy_artifact_root = task_root / "legacy-artifacts"
        try:
            execute_artifacts(legacy_record_path, legacy_artifact_root)
        except RuntimeError as error:
            if (
                "1.0.93" not in str(error)
                or "separate authority is required" not in str(error)
            ):
                raise
        else:
            raise RuntimeError(
                "cross-run budget fake did not block the legacy execute path"
            )
        if legacy_record_path.exists() or legacy_artifact_root.exists():
            raise RuntimeError(
                "legacy execute path changed record or artifact state before refusal"
            )
        if hashlib.sha256(COMMITTED_PERMISSION_RECORD_PATH.read_bytes()).hexdigest() != COMMITTED_PERMISSION_RECORD_SHA256:
            raise RuntimeError("historical original invocation record identity changed")
        if hashlib.sha256(
            (FIXTURE_DIR / "permission-proof-preflight-record.json").read_bytes()
        ).hexdigest() != HISTORICAL_PERMISSION_PREFLIGHT_SHA256:
            raise RuntimeError("historical permission preflight record identity changed")
        for option in ("--execute", "--permission-proof"):
            entrypoint_record = task_root / f"entrypoint-{option[2:]}-record.json"
            entrypoint_artifacts = task_root / f"entrypoint-{option[2:]}-artifacts"
            arguments = [
                sys.executable,
                str(Path(__file__).resolve()),
                option,
                "--record",
                str(entrypoint_record),
                "--artifact-root",
                str(entrypoint_artifacts),
            ]
            if option == "--permission-proof":
                arguments.extend(["--preflight-record", str(task_root / "unused-preflight.json")])
            try:
                refused = subprocess.run(
                    arguments,
                    cwd=ROOT,
                    env={
                        "HOME": str(host_home),
                        "PATH": os.environ.get("PATH", "/usr/bin:/bin"),
                        "TMPDIR": str(scratch),
                        "TERM": "dumb",
                    },
                    stdin=subprocess.DEVNULL,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    timeout=10,
                    check=False,
                )
            except subprocess.TimeoutExpired:
                raise RuntimeError(f"{option} consumed-record guard exceeded 10 seconds") from None
            refused_output = (refused.stdout + refused.stderr).decode("utf-8", errors="replace")
            if (
                refused.returncode == 0
                or "1.0.93" not in refused_output
                or "separate authority is required" not in refused_output
                or entrypoint_record.exists()
                or entrypoint_artifacts.exists()
            ):
                raise RuntimeError(f"{option} did not refuse before staging or original start")
            if any(
                marker in refused_output
                for marker in (
                    "SWALLOWTAIL_SENTINEL_TOKEN",
                    "SWALLOWTAIL_SENTINEL_COOKIE",
                    "SWALLOWTAIL_SENTINEL_URL",
                    str(host_home),
                )
            ):
                raise RuntimeError(f"{option} exposed a private diagnostic marker")
        altered_record_path = task_root / "altered-committed-permission-record.json"
        altered_record_path.write_bytes(
            COMMITTED_PERMISSION_RECORD_PATH.read_bytes() + b"\n"
        )
        try:
            consumed_permission_versions(altered_record_path)
        except RuntimeError as error:
            if "changed" not in str(error):
                raise
        else:
            raise RuntimeError("cross-run budget fake accepted an altered record")
        missing_record_path = task_root / "missing-committed-permission-record.json"
        try:
            consumed_permission_versions(missing_record_path)
        except RuntimeError as error:
            if "missing or unsafe" not in str(error):
                raise
        else:
            raise RuntimeError("cross-run budget fake accepted a missing record")
        proof = {
            "status": "passed",
            "network_denial": "loopback and reserved external connect both returned EPERM/EACCES",
            "filesystem_boundary": "native permission fake read only the declared synthetic auth metadata file; adjacent home/repository reads and home/repository writes were denied",
            "keychain_boundary": "securityd service lookup remains the trusted vendor boundary; the fake performed no keychain item read",
            "record_before_execution": True,
            "authenticated_plan": {
                "schema": plan["schema"],
                "execution_authorized": plan["execution_authorized"],
                "invocations": len(plan["invocations"]),
                "missing_owner_attestations": len(plan["missing_owner_attestations"]),
                "record_schema_validated": True,
            },
            "fake_acp": {
                "initialize": outcome["initialize"],
                "session_new": outcome["session_new"],
                "permission_request": "observed",
                "permission_reply": "cancelled",
                "pending_permission_wait_abandoned": outcome["pending_permission_wait_abandoned"],
                "prompt_stop_reason": outcome["session_prompt"],
                "tool_effect": "absent",
                "exit_code": outcome["exit_code"],
                "process_joined": outcome["process_joined"],
            },
            "authenticated_fake_preflight": auth_preflight,
            "authenticated_fake_result": auth_result,
            "authenticated_fake_discovery_result": discovery_result,
            "permission_boundary_plan": {
                "execution_authorized": permission_plan["execution_authorized"],
                "decision_id": permission_plan["operator_decision"],
                "allowed_hosts": len(permission_plan["network_policy"]["allowed_hosts"]),
                "attempts": permission_plan["budgets"]["max_total_invocations"],
            },
            "host_login_permission_fake": permission_boundary_result,
            "correction_plan": {
                "schema": correction_plan["schema"],
                "source_permission_plan_sha256": correction_plan[
                    "source_permission_plan_sha256"
                ],
                "auth_metadata_read_paths": [
                    item["relative_path"]
                    for item in correction_plan["auth_metadata_read_paths"]
                ],
                "version_scope": "official documentation path; not frozen-artifact-specific",
            },
            "cross_run_invocation_budget": {
                "committed_record_sha256": COMMITTED_PERMISSION_RECORD_SHA256,
                "consumed_versions": sorted(consumed_versions),
                "consumed_1_0_93_refused": True,
                "legacy_execute_refused_before_staging": True,
                "permission_proof_refused_before_staging": True,
                "missing_or_changed_record_fails_closed": True,
            },
        }
        preflight_path = scratch / "permission-proof-preflight-v2.json"
        preflight = make_permission_preflight_record(proof)
        write_json_durable(preflight_path, preflight)
        validate_preflight_record(preflight_path, permission_plan)
        if str(host_home) in json.dumps(preflight, sort_keys=True):
            raise RuntimeError("permission preflight record exposed a synthetic home path")
        proof["renewal_fake_controls"] = renewal_fake_controls(proof)
        return proof


FAKE_PROBE_PROXY = {
    "unlisted_destination_count": 1,
    "sni_mismatch_count": 1,
    "connection_failure_count": 0,
}


def renewal_authority_document(
    binary_sha256: str, operation_id: str, harness_digest: str
) -> dict[str, Any]:
    return {
        "schema": RENEWAL_AUTHORITY_SCHEMA,
        "operator_decision": RENEWAL_DECISION,
        "operator_ruling": "Approve one corrected 1.0.93 attempt",
        "operation_id": operation_id,
        "version": RENEWAL_VERSION,
        "binary_sha256": binary_sha256,
        "account_access_ref": RENEWAL_ACCOUNT_REF,
        "permission_plan_sha256": file_sha256(PERMISSION_PROOF_PLAN_PATH),
        "correction_plan_sha256": file_sha256(PERMISSION_CORRECTION_PLAN_PATH),
        "artifact_inventory_sha256": file_sha256(INVENTORY_PATH),
        "harness_sha256": harness_digest,
        "prior_consumed_record_sha256": COMMITTED_PERMISSION_RECORD_SHA256,
        "start_order": [RENEWAL_VERSION],
        "budgets": RENEWAL_BUDGETS,
        "profile_basis": (
            "permission_sandbox_profile in the bound harness; the per-run profile "
            "digest is recorded in the renewal execution record"
        ),
        "qualification_changed": False,
    }


def renewal_fake_controls(proof: dict[str, Any]) -> dict[str, Any]:
    """Drive the renewal admission and launch sequence with the native fake only."""
    plan = validate_permission_proof_plan()
    correction_plan = validate_permission_correction_plan(plan)
    current_harness = harness_sha256()
    with tempfile.TemporaryDirectory(prefix="copilot-acp-permission-proof.") as temp_root:
        task_root = Path(temp_root).resolve()
        records = task_root / "records"
        records.mkdir()
        artifact_root = task_root / "artifacts"
        artifact_root.mkdir()
        host_home = task_root / "fake-host-home"
        (host_home / ".copilot").mkdir(parents=True)
        (host_home / ".ssh").mkdir()
        (host_home / "Library" / "Keychains").mkdir(parents=True)
        repository_root = task_root / "fake-repository"
        repository_root.mkdir()
        for path, contents in (
            (
                host_home / ".copilot" / "config.json",
                b"synthetic-auth-metadata="
                + b"SWALLOWTAIL_FAKE_AUTH_METADATA_SENTINEL\n",
            ),
            (host_home / ".copilot" / "settings.json", b"fake settings\n"),
            (host_home / ".ssh" / "id_ed25519", b"synthetic unrelated home file\n"),
            (host_home / "Library" / "Keychains" / "login.keychain-db", b"fake keychain\n"),
            (repository_root / "README.md", b"fake repository\n"),
        ):
            path.write_bytes(contents)
        executable = task_root / "copilot-permission-fake"
        fake_sha256 = compile_permission_fake(executable)["binary_sha256"]
        paths = RenewalPaths(
            authority=task_root / "renewal-authority.json",
            attempt=task_root / "renewal-attempt.json",
            result=task_root / "renewal-result.json",
        )
        authority = renewal_authority_document(
            fake_sha256, str(uuid.uuid4()), current_harness
        )
        write_json_durable(paths.authority, authority)
        refused: list[str] = []

        try:
            require_original_process_containment()
        except RuntimeError as error:
            if ORIGINAL_PROCESS_CONTAINMENT_STOP not in str(error):
                raise
            refused.append("original-containment-not-proven")
        else:
            raise RuntimeError("original admission opened without descendant containment proof")

        def expect_refusal(label: str, action: Callable[[], Any], needle: str) -> None:
            try:
                action()
            except (RuntimeError, ValueError) as error:
                if needle not in str(error):
                    raise RuntimeError(f"renewal control {label} refused for the wrong reason") from error
                refused.append(label)
                return
            raise RuntimeError(f"renewal control {label} was accepted")

        def admit(
            authority_path: Path = paths.authority,
            *,
            binary_sha: str = fake_sha256,
        ) -> dict[str, Any]:
            return admit_renewal_invocation(
                RenewalPaths(authority=authority_path, attempt=paths.attempt, result=paths.result),
                expected_binary_sha256=binary_sha,
            )

        expect_refusal(
            "missing-authority",
            lambda: admit(task_root / "missing-authority.json"),
            "missing or unsafe",
        )
        authority_link = task_root / "authority-link.json"
        authority_link.symlink_to(paths.authority)
        expect_refusal("symlink-authority", lambda: admit(authority_link), "missing or unsafe")
        tampered_overrides = {
            "tampered-decision": {"operator_decision": "00000000-0000-4000-8000-000000000000"},
            "wrong-account": {"account_access_ref": "github-account:other-operator"},
            "widened-prompt-budget": {"budgets": {**RENEWAL_BUDGETS, "max_prompts": 2}},
            "changed-harness": {"harness_sha256": "0" * 64},
            "older-version-start": {"start_order": ["1.0.81"]},
            "qualification-change": {"qualification_changed": True},
            "extra-authority-field": {"network_override": True},
        }
        for label, override in tampered_overrides.items():
            tampered_path = task_root / f"{label}.json"
            write_json_durable(tampered_path, {**authority, **override})
            expect_refusal(label, lambda p=tampered_path: admit(p), "renewal authority")
        expect_refusal(
            "identity-mismatch-binary",
            lambda: admit(binary_sha="0" * 64),
            "renewal authority",
        )
        staged_wrong = artifact_root / RENEWAL_VERSION / "copilot"
        staged_wrong.parent.mkdir(parents=True)
        staged_wrong.write_bytes(b"not the granted executable\n")
        expect_refusal(
            "staged-binary-digest",
            lambda: staged_renewal_binary(artifact_root, fake_sha256),
            "differs from the granted identity",
        )
        cli_record = records / "cli-refusal-record.json"
        try:
            cli_refusal = subprocess.run(
                [
                    sys.executable,
                    str(Path(__file__).resolve()),
                    "--renewed-permission-proof",
                    "--record",
                    str(cli_record),
                    "--artifact-root",
                    str(artifact_root),
                    "--preflight-record",
                    str(task_root / "cli-refusal-preflight.json"),
                ],
                cwd=ROOT,
                env={
                    "HOME": str(host_home),
                    "PATH": os.environ.get("PATH", "/usr/bin:/bin"),
                    "TERM": "dumb",
                },
                stdin=subprocess.DEVNULL,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                timeout=30,
                check=False,
            )
        except subprocess.TimeoutExpired:
            raise RuntimeError("renewal CLI refusal control exceeded 30 seconds") from None
        if (
            cli_refusal.returncode == 0
            or cli_record.exists()
            or (task_root / "action").exists()
        ):
            raise RuntimeError("renewal CLI did not refuse before staging or original start")
        refused.append("cli-refused-before-staging")
        if (task_root / "action").exists():
            raise RuntimeError("a refused renewal grant staged or started an original path")
        admission = admit()
        if paths.attempt.exists() or paths.result.exists():
            raise RuntimeError("read-only admission consumed or wrote a renewal record")
        preflight_path = task_root / "renewal-preflight.json"
        write_json_durable(preflight_path, make_permission_preflight_record(proof))
        record_path, preflight_path, _, resolved_task = validate_original_task_paths(
            records / "renewal-scratch.json", artifact_root, preflight_path
        )
        validate_preflight_record(preflight_path, plan)
        attempt_sha256 = consume_renewal_attempt(paths.attempt, admission)
        expect_refusal("duplicate-after-consumption", lambda: admit(), "already consumed")
        action_scratch = resolved_task / "action" / RENEWAL_VERSION
        record = execute_renewal_invocation(
            admission,
            attempt_sha256=attempt_sha256,
            record_path=record_path,
            task_root=resolved_task,
            binary=executable,
            binary_sha256=fake_sha256,
            preflight_sha256=file_sha256(preflight_path),
            result_path=paths.result,
            repository_root=repository_root,
            host_home=host_home,
            extra_environment=lambda port: {
                "SWALLOWTAIL_BLOCKED_REPOSITORY": str(repository_root.resolve()),
                "SWALLOWTAIL_EFFECT_MARKER": str(action_scratch / "permission-effect-marker"),
                "SWALLOWTAIL_PROXY_PORT": str(port),
                "SWALLOWTAIL_FAKE_DIAGNOSTIC": str(action_scratch / "fake-child-diagnostic.json"),
            },
            proxy_fake_only=True,
            expected_probe_proxy=FAKE_PROBE_PROXY,
        )
        result = record["invocations"][0]["result"]
        launch_checks = {
            "initialize": result.get("initialize") == "success",
            "reported_version_matches": result.get("reported_version_matches") is True,
            "session_new": result.get("session_new") == "success",
            "one_prompt": result.get("prompt_requests_sent") == 1,
            "one_permission_request": result.get("permission_request_count") == 1,
            "permission_cancelled": result.get("permission_reply") == "cancelled",
            "prompt_cancelled": result.get("session_prompt") == "cancelled",
            "no_effect": result.get("effect_marker_present") is False,
            "cleanup_joined": all(
                result.get(name) is True
                for name in (
                    "process_joined",
                    "process_group_joined",
                    "stderr_drain_joined",
                    "proxy_threads_joined",
                )
            ),
            "permission_evidence_complete": result.get("permission_evidence_complete") is True,
            "prompt_accounting": record.get("prompts_used") == 1
            and record.get("remaining_prompt_allowance") == 2,
            "boundary_proven": record.get("permission_boundary_proven") is True,
        }
        failed_checks = sorted(name for name, passed in launch_checks.items() if not passed)
        if failed_checks:
            raise RuntimeError(f"renewal fake launch failed checks: {', '.join(failed_checks)}")
        if permission_evidence_is_complete({**result, "prompt_requests_sent": 0}, FAKE_PROBE_PROXY):
            raise RuntimeError("zero-prompt renewal evidence was accepted as a permission proof")
        if permission_evidence_is_complete({**result, "effect_marker_present": True}, FAKE_PROBE_PROXY):
            raise RuntimeError("renewal evidence with an effect was accepted as a permission proof")
        expect_refusal("result-exists-after-run", lambda: admit(), "already consumed")
        secret_free_serialization(record)
        return {
            "status": "passed",
            "fake_only": True,
            "refused_controls": refused,
            "launch_checks": launch_checks,
            "invocations": len(record["invocations"]),
            "prompts_used": record["prompts_used"],
            "remaining_prompt_allowance": record["remaining_prompt_allowance"],
            "attempt_bound_to_record": record["attempt_sha256"] == attempt_sha256,
            "qualification_changed": False,
        }


def make_permission_preflight_record(result: dict[str, Any]) -> dict[str, Any]:
    fake = result.get("host_login_permission_fake", {})
    return {
        "schema": "copilot-cli-acp-permission-proof-preflight.v2",
        "recorded_at_utc": utc_now(),
        "plan_sha256": hashlib.sha256(PERMISSION_PROOF_PLAN_PATH.read_bytes()).hexdigest(),
        "correction_plan_sha256": hashlib.sha256(
            PERMISSION_CORRECTION_PLAN_PATH.read_bytes()
        ).hexdigest(),
        "harness_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "native_fake_source_sha256": fake.get("fake_source_sha256"),
        "native_fake_binary_sha256": fake.get("fake_binary_sha256"),
        "result": result,
    }


def prepare_record(record_path: Path) -> None:
    previous_preflights: list[dict[str, Any]] = []
    if record_path.exists():
        previous = load_json(record_path)
        if previous.get("artifact_execution_started") or previous.get("executions"):
            raise RuntimeError("artifact execution already started; refusing to rewrite the record")
        previous_preflights.extend(previous.get("preflight_attempts", []))
        if previous.get("preflight"):
            previous_preflights.append(previous["preflight"])
    inventory = verify_inventory()
    fake_proof = self_test()
    inventory_digest = hashlib.sha256(INVENTORY_PATH.read_bytes()).hexdigest()
    harness_digest = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    record = {
        "schema": "copilot-cli-acp-offline-execution-record.v1",
        "recorded_at_utc": utc_now(),
        "scope": {
            "route": "copilot-cli.acp",
            "selected_argv": ["copilot", "--acp", "--stdio"],
            "versions": list(VERSIONS),
            "network": "denied, including loopback",
            "authentication": "synthetic placeholder only",
        },
        "artifact_inventory_observed_at_utc": inventory["observed_at_utc"],
        "artifact_inventory_sha256_at_execution": inventory_digest,
        "artifact_inventory_sha256_committed": inventory_digest,
        "harness_source": "scripts/copilot-acp-offline-proof.py",
        "harness_sha256": harness_digest,
        "harness_sha256_recorded_at_utc": utc_now(),
        "execution_host": {
            "architecture": platform.machine().lower(),
            "host_name_recorded": False,
            "os": f"{platform.system()} {platform.release()}",
        },
        "pre_execution_record_persisted": True,
        "artifact_execution_started": False,
        "permission_boundary_proven_for_all_targets": False,
        "exact_permission_path_reached": False,
        "preflight": fake_proof,
        "preflight_attempts": [*previous_preflights, fake_proof],
        "executions": [],
    }
    write_json_durable(record_path, record)
    saved = load_json(record_path)
    if not saved.get("pre_execution_record_persisted") or saved.get("artifact_execution_started"):
        raise RuntimeError("pre-execution record failed its read-back gate")
    print(json.dumps({"record": str(record_path), "preflight": fake_proof}, indent=2))


def validate_preflight_record(path: Path, plan: dict[str, Any]) -> dict[str, Any]:
    preflight = load_json(path)
    validate_schema_value(
        preflight,
        load_json(PERMISSION_PREFLIGHT_SCHEMA_PATH),
        "permission_proof_preflight",
    )
    if preflight.get("harness_sha256") != hashlib.sha256(Path(__file__).read_bytes()).hexdigest():
        raise ValueError("fake preflight was produced by a different harness revision")
    if preflight.get("plan_sha256") != hashlib.sha256(
        PERMISSION_PROOF_PLAN_PATH.read_bytes()
    ).hexdigest():
        raise ValueError("fake preflight was produced for a different permission plan")
    if preflight.get("correction_plan_sha256") != hashlib.sha256(
        PERMISSION_CORRECTION_PLAN_PATH.read_bytes()
    ).hexdigest():
        raise ValueError("fake preflight was produced for a different correction plan")
    if preflight.get("native_fake_source_sha256") != hashlib.sha256(
        PERMISSION_FAKE_SOURCE_PATH.read_bytes()
    ).hexdigest():
        raise ValueError("fake preflight was produced by a different native fake source")
    result = preflight.get("result", {})
    fake = result.get("host_login_permission_fake", {})
    if preflight.get("native_fake_binary_sha256") != fake.get("fake_binary_sha256"):
        raise ValueError("fake preflight is not bound to its compiled native fake")
    if result.get("status") != "passed" or fake.get("permission_reply") != "cancelled":
        raise ValueError("permission proof requires a passing host-login cancellation fake")
    if fake.get("fake_process_startup") != "observed":
        raise ValueError("permission proof fake did not establish its native process startup")
    if fake.get("effect_marker_absent") is not True:
        raise ValueError("host-login cancellation fake did not prove absence of the effect")
    if fake.get("process_group_joined") is not True or fake.get("proxy_threads_joined") is not True:
        raise ValueError("host-login cancellation fake did not prove bounded joined cleanup")
    if fake.get("stderr_reader_joined") is not True:
        raise ValueError("host-login cancellation fake did not join its stderr reader")
    policy = fake.get("sandbox_policy", {})
    if any(
        policy.get(field) is not True
        for field in (
            "auth_metadata_read",
            "auth_metadata_write_denied",
            "adjacent_home_reads_denied",
            "keychain_file_read_denied",
            "repository_reads_denied",
            "repository_writes_denied",
            "direct_egress_denied",
            "inbound_bind_denied",
            "shell_exec_denied",
        )
    ) or policy.get("fake_runtime_roots") != []:
        raise ValueError("host-login fake did not prove the corrected original profile boundary")
    if fake.get("network", {}).get("allowed_destination_hosts") != ["api.github.com"]:
        raise ValueError("host-login fake did not exercise the selected proxy path")
    if fake.get("network") != {
        "allowed_destination_hosts": ["api.github.com"],
        "unlisted_destination_count": 1,
        "sni_mismatch_count": 1,
        "connection_failure_count": 0,
    } or not plan["network_policy"]["allowed_hosts"]:
        raise ValueError("host-login fake did not prove default-deny destinations")
    controls = fake.get("stderr_controls", {})
    if set(controls) != {"--stderr-fullpipe", "--stderr-secret", "--early-exit", "--hang"}:
        raise ValueError("host-login fake omitted a required stderr control")
    if not controls["--stderr-fullpipe"].get("stderr", {}).get("truncated"):
        raise ValueError("host-login fake did not bound full-pipe stderr")
    if controls["--stderr-secret"].get("stderr", {}).get("classification") != "unknown":
        raise ValueError("host-login fake did not fail closed on secret-bearing stderr")
    summaries = [fake.get("stderr", {}), *(control.get("stderr", {}) for control in controls.values())]
    for summary in summaries:
        if (
            summary.get("classification") not in STDERR_CATEGORIES
            or summary.get("captured_bytes", MAX_STDERR_CAPTURE_BYTES + 1)
            > MAX_STDERR_CAPTURE_BYTES
            or summary.get("total_bytes", MAX_STDERR_COUNTED_BYTES + 1)
            > MAX_STDERR_COUNTED_BYTES
            or summary.get("total_byte_count_limit") != MAX_STDERR_COUNTED_BYTES
            or not isinstance(summary.get("total_bytes_capped"), bool)
            or summary.get("total_bytes", -1) < summary.get("captured_bytes", 0)
            or summary.get("truncated")
            != (
                summary.get("total_bytes_capped")
                or summary.get("total_bytes") > summary.get("captured_bytes")
            )
            or (
                summary.get("total_bytes_capped")
                and summary.get("total_bytes") != MAX_STDERR_COUNTED_BYTES
            )
            or summary.get("raw_persisted") is not False
            or summary.get("raw_displayed") is not False
            or not summary.get("reader_joined")
            or summary.get("reader_error")
        ):
            raise ValueError("host-login fake stderr record is not bounded and redacted")
    for control in controls.values():
        if not control.get("stderr", {}).get("reader_joined") or not control.get("process_group_joined"):
            modes = [
                mode
                for mode, item in controls.items()
                if not item.get("stderr", {}).get("reader_joined")
                or not item.get("process_group_joined")
            ]
            raise ValueError(
                "host-login fake stderr control left its reader or process unjoined "
                f"(modes={','.join(modes)})"
            )
    secret_free_serialization(preflight)
    return preflight


def secret_free_serialization(value: Any) -> None:
    serialized = json.dumps(value, sort_keys=True)
    for forbidden in (
        "/Users/",
        "/home/",
        "github_pat_",
        "ghp_",
        "gho_",
        "ghs_",
        "xoxb-",
        "Bearer ",
        "test-only-delegated-capability",
        "SWALLOWTAIL_FAKE_AUTH_METADATA_SENTINEL",
        "SWALLOWTAIL_SENTINEL_TOKEN",
        "SWALLOWTAIL_SENTINEL_COOKIE",
        "SWALLOWTAIL_SENTINEL_URL",
        "session-private",
        "arbitrary vendor error text",
    ):
        if forbidden in serialized:
            raise ValueError(f"permission proof record contains forbidden marker {forbidden!r}")


def validate_original_task_paths(
    record_path: Path, artifact_root: Path, preflight_path: Path
) -> tuple[Path, Path, Path, Path]:
    requested_record_path = record_path
    requested_preflight_path = preflight_path
    requested_artifact_root = artifact_root
    if requested_record_path.is_symlink() or requested_preflight_path.is_symlink() or requested_artifact_root.is_symlink():
        raise RuntimeError("original proof paths must not be symlinks")
    record_path = record_path.resolve(strict=False)
    preflight_path = preflight_path.resolve(strict=True)
    artifact_root = artifact_root.resolve(strict=True)
    if record_path.exists() or record_path.is_symlink():
        raise RuntimeError("refusing to repeat or overwrite an existing original proof record")
    if record_path.parent.name != "records":
        raise RuntimeError("proof records must use the records directory in fresh task scratch")
    task_root = record_path.parent.parent.resolve(strict=True)
    system_temp = Path(tempfile.gettempdir()).resolve()
    if (
        not task_root.is_relative_to(system_temp)
        or not task_root.name.startswith("copilot-acp-permission-proof.")
    ):
        raise RuntimeError("original proof requires a fresh mktemp task directory")
    if not artifact_root.is_relative_to(task_root) or artifact_root.is_symlink():
        raise RuntimeError("exact originals must be staged inside the fresh task directory")
    if not preflight_path.is_relative_to(task_root) or preflight_path == record_path:
        raise RuntimeError("fake preflight record must be in the same task scratch")
    return record_path, preflight_path, artifact_root, task_root


def run_permission_proof(
    record_path: Path,
    artifact_root: Path,
    preflight_path: Path,
) -> None:
    require_macos_sandbox()
    plan = validate_permission_proof_plan()
    enforce_permission_invocation_budget(plan)
    require_original_process_containment()
    correction_plan = validate_permission_correction_plan(plan)
    inventory = verify_inventory()
    record_path, preflight_path, artifact_root, task_root = validate_original_task_paths(
        record_path, artifact_root, preflight_path
    )
    preflight = validate_preflight_record(preflight_path, plan)
    packages = inventory_packages(inventory)
    staged: dict[str, tuple[Path, str]] = {}
    for version in plan["start_order"]:
        binary_path = artifact_root / version / "copilot"
        if binary_path.is_symlink() or not binary_path.is_file():
            raise RuntimeError(f"frozen original binary is absent for {version}")
        binary = binary_path.resolve(strict=True)
        if not binary.is_relative_to(artifact_root):
            raise RuntimeError(f"frozen original binary escapes its staged root for {version}")
        package = packages[("@github/copilot-darwin-arm64", version)]
        selected = next(
            item for item in package["files"] if item["path"] == "package/copilot"
        )
        digest = hashlib.sha256(binary.read_bytes()).hexdigest()
        if digest != selected["sha256"]:
            raise RuntimeError(f"frozen original binary digest mismatch for {version}")
        staged[version] = (binary, digest)

    record: dict[str, Any] = {
        "schema": "copilot-cli-acp-permission-proof-execution.v2",
        "plan_sha256": hashlib.sha256(PERMISSION_PROOF_PLAN_PATH.read_bytes()).hexdigest(),
        "correction_plan_sha256": hashlib.sha256(
            PERMISSION_CORRECTION_PLAN_PATH.read_bytes()
        ).hexdigest(),
        "artifact_inventory_sha256": hashlib.sha256(INVENTORY_PATH.read_bytes()).hexdigest(),
        "harness_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "account_access_ref": "github-account:betterthanclay",
        "account_identity_observation": "operator-reported reference; not independently queried",
        "keychain_boundary": {
            "normal_host_login_access": "permitted through original vendor CLI and com.apple.securityd",
            "queried_item_or_service": "unobserved; no item-level mediation claimed",
            "token_handling_by_harness": "none",
        },
        "network_authority_policy": {
            "source": plan["network_policy"]["source"],
            "allowed_exact_hosts": sorted(plan["network_policy"]["allowed_hosts"]),
            "allowed_subdomains": plan["network_policy"]["allowed_subdomains"],
            "default": "deny",
            "unexpected_destinations": "rejected; policy never expanded",
        },
        "budgets": plan["budgets"],
        "preflight_sha256": hashlib.sha256(preflight_path.read_bytes()).hexdigest(),
        "preflight_status": "passed",
        "artifact_execution_started": False,
        "permission_boundary_proven_for_all_targets": False,
        "exact_permission_path_reached": False,
        "invocations": [],
    }
    secret_free_serialization(record)
    write_json_durable(record_path, record)
    if load_json(record_path) != record:
        raise RuntimeError("top-level execution record did not persist before original starts")

    for index, version in enumerate(plan["start_order"]):
        binary, digest = staged[version]
        if digest != next(
            item["sha256"]
            for item in packages[("@github/copilot-darwin-arm64", version)]["files"]
            if item["path"] == "package/copilot"
        ):
            raise RuntimeError(f"frozen original identity changed before {version} start")
        result = run_permission_attempt(
            binary,
            version,
            task_root / "action" / version,
            record_path.parent,
            record_path,
            record,
            ROOT,
            plan,
            correction_plan,
            binary_sha256=digest,
        )
        secret_free_serialization(record)
        print(json.dumps({"version": version, "result": result}, sort_keys=True))
        if not result["safe_to_continue_older_versions"]:
            record["stopped_before_older_versions"] = version
            record["versions_not_started"] = plan["start_order"][index + 1 :]
            break
        if (
            result.get("prompt_error", {}).get("kind") == "quota-or-entitlement-stop"
            or result.get("session_new_error", {}).get("kind") == "quota-or-entitlement-stop"
            or result.get("initialize_error", {}).get("kind") == "quota-or-entitlement-stop"
        ):
            record["stopped_before_older_versions"] = version
            record["versions_not_started"] = plan["start_order"][index + 1 :]
            break
        if result["effect_marker_present"] or not result["process_joined"] or not result["process_group_joined"] or not result["proxy_threads_joined"]:
            record["stopped_before_older_versions"] = version
            record["versions_not_started"] = plan["start_order"][index + 1 :]
            break
        if result["proxy"]["unlisted_destination_count"] or result["proxy"]["sni_mismatch_count"] or result["proxy"]["connection_failure_count"]:
            record["stopped_before_older_versions"] = version
            record["versions_not_started"] = plan["start_order"][index + 1 :]
            break

    record["permission_boundary_proven_for_all_targets"] = bool(
        [item for item in record["invocations"] if item.get("result")]
        and len(record["invocations"]) == len(plan["start_order"])
        and all(
            item["result"].get("permission_evidence_complete") is True
            for item in record["invocations"]
        )
    )
    record["exact_permission_path_reached"] = any(
        item.get("result", {}).get("permission_request_count", 0) > 0
        for item in record["invocations"]
    )
    record["prompts_used"] = sum(
        item.get("result", {}).get("prompt_requests_sent", 0)
        for item in record["invocations"]
    )
    record["remaining_prompt_allowance"] = max(0, 3 - record["prompts_used"])
    record["qualification_changed"] = False
    record["completed_at_utc"] = utc_now()
    secret_free_serialization(record)
    write_json_durable(record_path, record)
    print(
        json.dumps(
            {
                "permission_boundary_proven_for_all_targets": record[
                    "permission_boundary_proven_for_all_targets"
                ],
                "exact_permission_path_reached": record["exact_permission_path_reached"],
                "invocations": len(record["invocations"]),
                "prompts_used": record["prompts_used"],
                "remaining_prompt_allowance": record["remaining_prompt_allowance"],
                "versions_not_started": record.get("versions_not_started", []),
                "record_schema": record["schema"],
            },
            sort_keys=True,
        )
    )


@dataclass(frozen=True)
class RenewalPaths:
    authority: Path = RENEWAL_AUTHORITY_PATH
    attempt: Path = RENEWAL_ATTEMPT_PATH
    result: Path = RENEWAL_RECORD_PATH


def file_sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def harness_sha256() -> str:
    return file_sha256(Path(__file__))


def read_regular_file(path: Path, refusal: str) -> bytes:
    if path.is_symlink() or not path.is_file():
        raise RuntimeError(refusal)
    return path.read_bytes()


def create_once_durable(path: Path, value: Any) -> None:
    """Create a record exactly once; an existing path, even a partial one, stays consumed."""
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists() or path.is_symlink():
        raise RuntimeError(f"{path.name} already exists; refusing to overwrite a consumed record")
    try:
        descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    except FileExistsError:
        raise RuntimeError(f"{path.name} already exists; refusing to overwrite a consumed record") from None
    with os.fdopen(descriptor, "wb") as output:
        output.write(json.dumps(value, indent=2, sort_keys=True).encode() + b"\n")
        output.flush()
        os.fsync(output.fileno())
    directory_fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(directory_fd)
    finally:
        os.close(directory_fd)
    if load_json(path) != value:
        raise RuntimeError(f"{path.name} did not persist exactly as written")


def validate_renewal_authority(
    authority: dict[str, Any],
    *,
    expected_harness_sha256: str,
    expected_binary_sha256: str,
) -> None:
    if set(authority) != RENEWAL_AUTHORITY_FIELDS:
        raise ValueError("renewal authority fields differ from the granted record shape")
    operation_id = authority.get("operation_id")
    if not isinstance(operation_id, str) or not re.fullmatch(
        r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}", operation_id
    ):
        raise ValueError("renewal authority operation ID is not a canonical UUID")
    expected = {
        "schema": RENEWAL_AUTHORITY_SCHEMA,
        "operator_decision": RENEWAL_DECISION,
        "operator_ruling": "Approve one corrected 1.0.93 attempt",
        "version": RENEWAL_VERSION,
        "binary_sha256": expected_binary_sha256,
        "account_access_ref": RENEWAL_ACCOUNT_REF,
        "permission_plan_sha256": file_sha256(PERMISSION_PROOF_PLAN_PATH),
        "correction_plan_sha256": file_sha256(PERMISSION_CORRECTION_PLAN_PATH),
        "artifact_inventory_sha256": file_sha256(INVENTORY_PATH),
        "harness_sha256": expected_harness_sha256,
        "prior_consumed_record_sha256": COMMITTED_PERMISSION_RECORD_SHA256,
        "start_order": [RENEWAL_VERSION],
        "budgets": RENEWAL_BUDGETS,
        "profile_basis": (
            "permission_sandbox_profile in the bound harness; the per-run profile "
            "digest is recorded in the renewal execution record"
        ),
        "qualification_changed": False,
    }
    for name, value in expected.items():
        if authority.get(name) != value:
            raise ValueError(f"renewal authority field {name} does not match the granted identity")
    secret_free_serialization(authority)


def validate_committed_renewal_authority() -> dict[str, Any]:
    authority = load_json(RENEWAL_AUTHORITY_PATH)
    validate_renewal_authority(
        authority,
        expected_harness_sha256=RENEWAL_HISTORICAL_HARNESS_SHA256,
        expected_binary_sha256=RENEWAL_BINARY_SHA256,
    )
    return authority


def admit_renewal_invocation(
    paths: RenewalPaths = RenewalPaths(),
    *,
    expected_binary_sha256: str = RENEWAL_BINARY_SHA256,
) -> dict[str, Any]:
    """Read-only admission: nothing is staged, consumed or started by this function."""
    plan = validate_permission_proof_plan()
    correction_plan = validate_permission_correction_plan(plan)
    verify_inventory()
    authority_bytes = read_regular_file(
        paths.authority,
        "renewal authority is missing or unsafe; refusing original start",
    )
    authority = json.loads(authority_bytes)
    current_harness = harness_sha256()
    validate_renewal_authority(
        authority,
        expected_harness_sha256=current_harness,
        expected_binary_sha256=expected_binary_sha256,
    )
    if paths.attempt.exists() or paths.attempt.is_symlink():
        raise RuntimeError(
            "renewal invocation is already consumed or uncertain; refusing original start"
        )
    if paths.result.exists() or paths.result.is_symlink():
        raise RuntimeError("renewal result already exists; refusing to repeat the original invocation")
    if RENEWAL_VERSION not in consumed_permission_versions():
        raise RuntimeError(
            "renewal requires the committed prior 1.0.93 consumption record; refusing original start"
        )
    return {
        "plan": plan,
        "correction_plan": correction_plan,
        "authority": authority,
        "authority_sha256": hashlib.sha256(authority_bytes).hexdigest(),
        "harness_sha256": current_harness,
    }


def consume_renewal_attempt(attempt_path: Path, admission: dict[str, Any]) -> str:
    attempt = {
        "schema": RENEWAL_ATTEMPT_SCHEMA,
        "operator_decision": RENEWAL_DECISION,
        "operation_id": admission["authority"]["operation_id"],
        "authority_sha256": admission["authority_sha256"],
        "harness_sha256": admission["harness_sha256"],
        "consumed_before_launcher_start": True,
        "launcher_start_issued": False,
        "consumed_at_utc": utc_now(),
    }
    create_once_durable(attempt_path, attempt)
    return file_sha256(attempt_path)


def staged_renewal_binary(artifact_root: Path, expected_sha256: str) -> tuple[Path, str]:
    binary_path = artifact_root / RENEWAL_VERSION / "copilot"
    if binary_path.is_symlink() or not binary_path.is_file():
        raise RuntimeError(f"frozen original binary is absent for {RENEWAL_VERSION}")
    binary = binary_path.resolve(strict=True)
    if not binary.is_relative_to(artifact_root):
        raise RuntimeError(f"frozen original binary escapes its staged root for {RENEWAL_VERSION}")
    packages = inventory_packages(verify_inventory())
    selected = next(
        item
        for item in packages[("@github/copilot-darwin-arm64", RENEWAL_VERSION)]["files"]
        if item["path"] == "package/copilot"
    )
    digest = file_sha256(binary)
    if digest != selected["sha256"] or digest != expected_sha256:
        raise RuntimeError("staged 1.0.93 original binary differs from the granted identity")
    return binary, digest


def finalize_renewal_record(record: dict[str, Any], renewal_error: str | None) -> None:
    invocations = record["invocations"]
    results = [item.get("result") or {} for item in invocations]
    prompts_used = sum(result.get("prompt_requests_sent", 0) for result in results)
    record["permission_boundary_proven"] = bool(
        len(results) == 1 and results[0].get("permission_evidence_complete") is True
    )
    record["exact_permission_path_reached"] = any(
        result.get("permission_request_count", 0) > 0 for result in results
    )
    record["prompts_used"] = prompts_used
    record["remaining_prompt_allowance"] = max(
        0,
        RENEWAL_BUDGETS["shared_prompt_maximum"]
        - RENEWAL_BUDGETS["historical_prompts"]
        - prompts_used,
    )
    if renewal_error is not None:
        record["renewal_error_class"] = renewal_error
    record["completed_at_utc"] = utc_now()


def execute_renewal_invocation(
    admission: dict[str, Any],
    *,
    attempt_sha256: str,
    record_path: Path,
    task_root: Path,
    binary: Path,
    binary_sha256: str,
    preflight_sha256: str,
    result_path: Path,
    repository_root: Path = ROOT,
    host_home: Path | None = None,
    extra_environment: Callable[[int], dict[str, str]] | None = None,
    proxy_fake_only: bool = False,
    expected_probe_proxy: dict[str, int] | None = None,
) -> dict[str, Any]:
    authority = admission["authority"]
    record: dict[str, Any] = {
        "schema": RENEWAL_RECORD_SCHEMA,
        "operator_decision": RENEWAL_DECISION,
        "operation_id": authority["operation_id"],
        "authority_sha256": admission["authority_sha256"],
        "attempt_sha256": attempt_sha256,
        "harness_sha256": admission["harness_sha256"],
        "plan_sha256": file_sha256(PERMISSION_PROOF_PLAN_PATH),
        "correction_plan_sha256": file_sha256(PERMISSION_CORRECTION_PLAN_PATH),
        "artifact_inventory_sha256": file_sha256(INVENTORY_PATH),
        "preflight_sha256": preflight_sha256,
        "account_access_ref": RENEWAL_ACCOUNT_REF,
        "account_identity_observation": "operator-reported reference; not independently queried",
        "start_order": [RENEWAL_VERSION],
        "versions_not_started": [],
        "budgets": RENEWAL_BUDGETS,
        "artifact_execution_started": False,
        "invocations": [],
        "started_at_utc": utc_now(),
        "qualification_changed": False,
    }
    secret_free_serialization(record)
    write_json_durable(record_path, record)
    if load_json(record_path) != record:
        raise RuntimeError("renewal record did not persist before original start")
    renewal_error: str | None = None
    try:
        run_permission_attempt(
            binary,
            RENEWAL_VERSION,
            task_root / "action" / RENEWAL_VERSION,
            record_path.parent,
            record_path,
            record,
            repository_root,
            admission["plan"],
            admission["correction_plan"],
            binary_sha256=binary_sha256,
            host_home=host_home,
            extra_environment=extra_environment,
            proxy_fake_only=proxy_fake_only,
            expected_probe_proxy=expected_probe_proxy,
        )
    except (OSError, RuntimeError, ValueError, TimeoutError, EOFError, KeyError) as error:
        renewal_error = type(error).__name__
    finalize_renewal_record(record, renewal_error)
    secret_free_serialization(record)
    write_json_durable(record_path, record)
    create_once_durable(result_path, record)
    return record


def run_renewed_permission_proof(
    record_path: Path,
    artifact_root: Path,
    preflight_path: Path,
) -> None:
    require_macos_sandbox()
    admission = admit_renewal_invocation()
    require_original_process_containment()
    record_path, preflight_path, artifact_root, task_root = validate_original_task_paths(
        record_path, artifact_root, preflight_path
    )
    validate_preflight_record(preflight_path, admission["plan"])
    binary, binary_sha256 = staged_renewal_binary(artifact_root, RENEWAL_BINARY_SHA256)
    attempt_sha256 = consume_renewal_attempt(RENEWAL_ATTEMPT_PATH, admission)
    record = execute_renewal_invocation(
        admission,
        attempt_sha256=attempt_sha256,
        record_path=record_path,
        task_root=task_root,
        binary=binary,
        binary_sha256=binary_sha256,
        preflight_sha256=file_sha256(preflight_path),
        result_path=RENEWAL_RECORD_PATH,
    )
    summary = validate_renewal_execution_record(RENEWAL_RECORD_PATH)
    print(
        json.dumps(
            {
                "record_schema": record["schema"],
                "invocations": len(record["invocations"]),
                "prompts_used": record["prompts_used"],
                "remaining_prompt_allowance": record["remaining_prompt_allowance"],
                "permission_boundary_proven": record["permission_boundary_proven"],
                "exact_permission_path_reached": record["exact_permission_path_reached"],
                "renewal_error_class": record.get("renewal_error_class"),
                "validation": summary["status"],
            },
            sort_keys=True,
        )
    )


def validate_renewal_execution_record(record_path: Path) -> dict[str, Any]:
    record = load_json(record_path)
    if record.get("schema") != RENEWAL_RECORD_SCHEMA:
        raise ValueError("unexpected renewal execution record schema")
    authority_bytes = read_regular_file(
        RENEWAL_AUTHORITY_PATH, "committed renewal authority is missing or unsafe"
    )
    authority = json.loads(authority_bytes)
    validate_renewal_authority(
        authority,
        expected_harness_sha256=RENEWAL_HISTORICAL_HARNESS_SHA256,
        expected_binary_sha256=RENEWAL_BINARY_SHA256,
    )
    if record.get("authority_sha256") != hashlib.sha256(authority_bytes).hexdigest():
        raise ValueError("renewal record does not bind the committed authority")
    if record.get("operation_id") != authority["operation_id"]:
        raise ValueError("renewal record operation ID differs from the granted operation")
    if record.get("harness_sha256") != RENEWAL_HISTORICAL_HARNESS_SHA256:
        raise ValueError("renewal record was produced by a different historical harness revision")
    if record.get("plan_sha256") != authority["permission_plan_sha256"]:
        raise ValueError("renewal record does not match the approved permission plan")
    if record.get("correction_plan_sha256") != authority["correction_plan_sha256"]:
        raise ValueError("renewal record does not match the correction plan")
    if record.get("artifact_inventory_sha256") != authority["artifact_inventory_sha256"]:
        raise ValueError("renewal record does not match the frozen inventory")
    attempt = load_json(RENEWAL_ATTEMPT_PATH)
    if (
        hashlib.sha256(RENEWAL_ATTEMPT_PATH.read_bytes()).hexdigest() != record.get("attempt_sha256")
        or attempt.get("consumed_before_launcher_start") is not True
        or attempt.get("operation_id") != authority["operation_id"]
    ):
        raise ValueError("renewal record does not bind its consumed attempt")
    if not re.fullmatch(r"[0-9a-f]{64}", str(record.get("preflight_sha256", ""))):
        raise ValueError("renewal record lacks a preflight digest")
    if record.get("account_access_ref") != RENEWAL_ACCOUNT_REF:
        raise ValueError("renewal record names a different account")
    if record.get("budgets") != RENEWAL_BUDGETS or record.get("start_order") != [RENEWAL_VERSION]:
        raise ValueError("renewal record budget or start order differs from the grant")
    if record.get("versions_not_started") != []:
        raise ValueError("renewal record misstates unstarted versions")
    invocations = record.get("invocations")
    if not isinstance(invocations, list) or len(invocations) > 1:
        raise ValueError("renewal record must hold at most one invocation")
    prompts_used = 0
    proven = False
    for item in invocations:
        pre_execution = item.get("pre_execution_record")
        if (
            item.get("version") != RENEWAL_VERSION
            or item.get("invocation_consumed") is not True
            or item.get("pre_execution_fsynced") is not True
            or not isinstance(pre_execution, dict)
            or item.get("pre_execution_record_sha256") != json_digest(pre_execution)
            or pre_execution.get("identity", {}).get("version") != RENEWAL_VERSION
            or pre_execution.get("identity", {}).get("binary_sha256") != RENEWAL_BINARY_SHA256
            or pre_execution.get("permission_plan_sha256") != record["plan_sha256"]
            or pre_execution.get("correction_plan_sha256") != record["correction_plan_sha256"]
            or pre_execution.get("account_access_ref") != RENEWAL_ACCOUNT_REF
            or pre_execution.get("selection_policy", {}).get("requested") != "Auto"
            or pre_execution.get("credential_boundary", {}).get("token_handling_by_harness")
            != "never extracted, copied, logged, or persisted"
            or not re.fullmatch(
                r"[0-9a-f]{64}",
                str(pre_execution.get("containment", {}).get("profile_sha256", "")),
            )
        ):
            raise ValueError("renewal invocation is not bound to its pre-execution record")
        result = item.get("result")
        if not isinstance(result, dict):
            raise ValueError("renewal invocation has no result")
        stderr = result.get("stderr", {})
        if (
            stderr.get("classification") not in STDERR_CATEGORIES
            or not isinstance(stderr.get("captured_bytes"), int)
            or not 0 <= stderr["captured_bytes"] <= MAX_STDERR_CAPTURE_BYTES
            or not isinstance(stderr.get("total_bytes"), int)
            or not stderr["captured_bytes"] <= stderr["total_bytes"] <= MAX_STDERR_COUNTED_BYTES
            or stderr.get("reader_joined") is not True
            or stderr.get("reader_error") is not False
            or stderr.get("raw_persisted") is not False
            or stderr.get("raw_displayed") is not False
        ):
            raise ValueError("renewal invocation has unsafe stderr diagnostics")
        if result.get("launcher_exit_status") != result.get("process_exit_code"):
            raise ValueError("renewal launcher status does not match the process exit")
        if result.get("initialize") == "not-reached" and result.get("vendor_startup_status") != "unknown":
            raise ValueError("renewal vendor startup is overstated")
        if result.get("prompt_requests_sent", 0) not in (0, 1):
            raise ValueError("renewal invocation exceeded its prompt budget")
        if result.get("permission_reply") not in {"none", "cancelled"}:
            raise ValueError("renewal invocation was not safely cancelled")
        plan = validate_permission_proof_plan()
        for host in result.get("proxy", {}).get("allowed_destination_hosts", []):
            if not host_allowed(
                host,
                set(plan["network_policy"]["allowed_hosts"]),
                tuple(plan["network_policy"]["allowed_subdomains"]),
            ):
                raise ValueError("renewal record includes a non-allowlisted host")
        prompts_used += result.get("prompt_requests_sent", 0)
        complete = result.get("permission_evidence_complete") is True
        if complete != permission_evidence_is_complete(result):
            raise ValueError("renewal permission evidence flag does not match its observations")
        if complete and (
            result.get("prompt_requests_sent") != 1
            or result.get("permission_request_count") != 1
            or result.get("effect_marker_present") is not False
        ):
            raise ValueError("renewal permission proof lacks its prompt, request or no-effect proof")
        proven = complete
    if record.get("prompts_used") != prompts_used or prompts_used > RENEWAL_BUDGETS["max_prompts"]:
        raise ValueError("renewal record does not reconcile prompt accounting")
    if record.get("remaining_prompt_allowance") != (
        RENEWAL_BUDGETS["shared_prompt_maximum"] - RENEWAL_BUDGETS["historical_prompts"] - prompts_used
    ):
        raise ValueError("renewal record misstates the remaining shared prompt allowance")
    if record.get("permission_boundary_proven") is not (proven and len(invocations) == 1):
        raise ValueError("renewal record misstates its permission proof result")
    if record.get("qualification_changed") is not False:
        raise ValueError("renewal evidence must not change route qualification")
    secret_free_serialization(record)
    return {
        "status": "valid",
        "invocations": len(invocations),
        "prompts_used": prompts_used,
        "permission_boundary_proven": record["permission_boundary_proven"],
        "qualification_changed": False,
    }


def validate_permission_execution_record(record_path: Path) -> dict[str, Any]:
    record = load_json(record_path)
    if record.get("schema") == RENEWAL_RECORD_SCHEMA:
        return validate_renewal_execution_record(record_path)
    plan = validate_permission_proof_plan()
    record_schema = record.get("schema")
    if record_schema not in {
        "copilot-cli-acp-permission-proof-execution.v1",
        "copilot-cli-acp-permission-proof-execution.v2",
    }:
        raise ValueError("unexpected original permission execution record schema")
    if record_schema == "copilot-cli-acp-permission-proof-execution.v2":
        validate_schema_value(
            record,
            load_json(PERMISSION_EXECUTION_V2_SCHEMA_PATH),
            "permission_proof_execution_v2",
        )
        if record.get("correction_plan_sha256") != hashlib.sha256(
            PERMISSION_CORRECTION_PLAN_PATH.read_bytes()
        ).hexdigest():
            raise ValueError("original execution record does not match the correction plan")
    if record.get("plan_sha256") != hashlib.sha256(
        PERMISSION_PROOF_PLAN_PATH.read_bytes()
    ).hexdigest():
        raise ValueError("original execution record does not match the approved plan")
    if record.get("artifact_inventory_sha256") != hashlib.sha256(
        INVENTORY_PATH.read_bytes()
    ).hexdigest():
        raise ValueError("original execution record does not match the frozen inventory")
    accepted_harness_digests = {
        PRIOR_PERMISSION_PROOF_HARNESS_SHA256,
        hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    }
    if record.get("harness_sha256") not in accepted_harness_digests:
        raise ValueError("original execution record does not match this harness revision")
    invocations = record.get("invocations")
    if not isinstance(invocations, list) or not 0 < len(invocations) <= 3:
        raise ValueError("original execution record has an invalid invocation count")
    order = plan["start_order"]
    if [item.get("version") for item in invocations] != order[: len(invocations)]:
        raise ValueError("original execution record is out of approved start order")
    for index, item in enumerate(invocations):
        safe_to_continue = item.get("result", {}).get(
            "safe_to_continue_older_versions"
        )
        if index < len(invocations) - 1 and safe_to_continue is not True:
            raise ValueError("an older version started after an incomplete permission proof")
        if index == len(invocations) - 1 and len(invocations) < len(order) and safe_to_continue is not False:
            raise ValueError("an incomplete version sequence lacks its required stop")
    if record.get("versions_not_started", []) != order[len(invocations) :]:
        raise ValueError("original execution record misstates the unstarted versions")
    packages = inventory_packages(verify_inventory())
    for item in invocations:
        version = item.get("version")
        expected_hash = next(
            file["sha256"]
            for file in packages[("@github/copilot-darwin-arm64", version)]["files"]
            if file["path"] == "package/copilot"
        )
        pre_execution = item.get("pre_execution_record")
        if (
            item.get("invocation_consumed") is not True
            or item.get("pre_execution_fsynced") is not True
            or not isinstance(pre_execution, dict)
            or item.get("pre_execution_record_sha256") != json_digest(pre_execution)
            or pre_execution.get("identity", {}).get("version") != version
            or pre_execution.get("identity", {}).get("binary_sha256") != expected_hash
            or pre_execution.get("account_access_ref") != "github-account:betterthanclay"
            or pre_execution.get("selection_policy", {}).get("requested") != "Auto"
            or pre_execution.get("credential_boundary", {}).get("token_handling_by_harness")
            != "never extracted, copied, logged, or persisted"
        ):
            raise ValueError(f"invalid or unbound pre-execution record for {version}")
        if record_schema == "copilot-cli-acp-permission-proof-execution.v2" and (
            pre_execution.get("permission_plan_sha256") != record.get("plan_sha256")
            or pre_execution.get("correction_plan_sha256")
            != record.get("correction_plan_sha256")
        ):
            raise ValueError(f"corrected profile for {version} is not bound to both plans")
        result = item.get("result")
        if not isinstance(result, dict):
            raise ValueError(f"original invocation {version} has no result")
        if record_schema == "copilot-cli-acp-permission-proof-execution.v2":
            stderr = result.get("stderr", {})
            captured = stderr.get("captured_bytes")
            total = stderr.get("total_bytes")
            if (
                result.get("vendor_startup_status")
                not in {"unknown", "acp-initialize-response-observed"}
                or result.get("launcher_exit_status") != result.get("process_exit_code")
                or stderr.get("classification") not in STDERR_CATEGORIES
                or not isinstance(captured, int)
                or not isinstance(total, int)
                or captured < 0
                or captured > MAX_STDERR_CAPTURE_BYTES
                or total > MAX_STDERR_COUNTED_BYTES
                or stderr.get("total_byte_count_limit") != MAX_STDERR_COUNTED_BYTES
                or not isinstance(stderr.get("total_bytes_capped"), bool)
                or total < captured
                or stderr.get("truncated")
                != (stderr.get("total_bytes_capped") or total > captured)
                or (
                    stderr.get("total_bytes_capped")
                    and total != MAX_STDERR_COUNTED_BYTES
                )
                or stderr.get("capture_limit_bytes") != MAX_STDERR_CAPTURE_BYTES
                or stderr.get("reader_joined") is not True
                or stderr.get("reader_error") is not False
                or stderr.get("raw_persisted") is not False
                or stderr.get("raw_displayed") is not False
            ):
                raise ValueError(f"original invocation {version} has unsafe stderr diagnostics")
            if (
                result.get("initialize") == "not-reached"
                and result.get("vendor_startup_status") != "unknown"
            ):
                raise ValueError(f"original startup for {version} is overstated")
        if result.get("prompt_requests_sent", 0) not in (0, 1):
            raise ValueError(f"original invocation {version} exceeded its prompt budget")
        if result.get("permission_reply") not in {"none", "cancelled"}:
            raise ValueError(f"original invocation {version} was not safely cancelled")
        network = result.get("proxy", {})
        for host in network.get("allowed_destination_hosts", []):
            if not host_allowed(
                host,
                set(plan["network_policy"]["allowed_hosts"]),
                tuple(plan["network_policy"]["allowed_subdomains"]),
            ):
                raise ValueError(f"execution record includes a non-allowlisted host for {version}")
    calculated_prompts = sum(
        item.get("result", {}).get("prompt_requests_sent", 0) for item in invocations
    )
    if record.get("prompts_used") != calculated_prompts or calculated_prompts > 3:
        raise ValueError("original execution record does not reconcile prompt accounting")
    if record.get("qualification_changed") is not False:
        raise ValueError("permission evidence must not silently change route qualification")
    if record.get("permission_boundary_proven_for_all_targets") is not all(
        len(invocations) == 3
        and item.get("result", {}).get("permission_evidence_complete") is True
        for item in invocations
    ):
        raise ValueError("original execution record misstates its permission proof result")
    secret_free_serialization(record)
    return {
        "status": "valid",
        "invocations": len(invocations),
        "prompts_used": calculated_prompts,
        "permission_boundary_proven_for_all_targets": record[
            "permission_boundary_proven_for_all_targets"
        ],
        "exact_permission_path_reached": record.get("exact_permission_path_reached") is True,
        "qualification_changed": False,
    }


def execute_artifacts(record_path: Path, artifact_root: Path) -> None:
    require_macos_sandbox()
    plan = validate_permission_proof_plan()
    enforce_permission_invocation_budget(plan)
    require_original_process_containment()
    inventory = verify_inventory()
    record = load_json(record_path)
    if record.get("pre_execution_record_persisted") is not True:
        raise RuntimeError("exact artifacts require a persisted pre-execution record")
    if record.get("preflight", {}).get("status") != "passed":
        raise RuntimeError("exact artifacts require a passing fake containment proof")
    if record.get("artifact_execution_started") or record.get("executions"):
        raise RuntimeError("exact artifact executions are already recorded; refusing a repeat")
    packages = inventory_packages(inventory)
    host_home = Path(os.environ["HOME"]).resolve()
    root = artifact_root.resolve(strict=True)
    temp_root = Path(tempfile.gettempdir()).resolve()
    if artifact_root.is_symlink() or not root.is_relative_to(temp_root):
        raise RuntimeError("exact artifacts must be staged inside a fresh task-owned temp directory")
    staged: list[tuple[str, Path, str, Path]] = []

    for version in VERSIONS:
        binary_path = root / version / "copilot"
        if binary_path.is_symlink() or not binary_path.is_file():
            raise RuntimeError(f"selected native binary is absent for {version}")
        binary = binary_path.resolve(strict=True)
        if not binary.is_relative_to(root):
            raise RuntimeError(f"selected native binary escapes the artifact root for {version}")
        package = packages[("@github/copilot-darwin-arm64", version)]
        binary_record = next((item for item in package["files"] if item["path"] == "package/copilot"), None)
        if binary_record is None:
            raise RuntimeError(f"inventory has no selected binary for {version}")
        digest_builder = hashlib.sha256()
        with binary.open("rb") as input_file:
            for block in iter(lambda: input_file.read(1024 * 1024), b""):
                digest_builder.update(block)
        digest = digest_builder.hexdigest()
        if digest != binary_record["sha256"]:
            raise RuntimeError(f"native binary digest mismatch for {version}")
        run_scratch = root.parent / f"execution-{version}"
        if run_scratch.exists():
            raise RuntimeError(f"task scratch already has an execution directory for {version}")
        staged.append((version, binary, digest, run_scratch))

    record["executions"] = []
    for version, binary, digest, run_scratch in staged:
        run_scratch.mkdir(parents=True, exist_ok=False)
        attempt: dict[str, Any] = {
            "version": version,
            "binary_sha256": digest,
            "selected_argv": ["copilot", "--acp", "--stdio"],
            "recorded_before_process_start": True,
            "network": "sandbox-denied",
            "credentials": "synthetic-placeholder-only",
            "started_at_utc": utc_now(),
            "permission_request_observed": False,
            "permission_reply": "none",
            "effect_marker_present": False,
        }
        record["executions"].append(attempt)
        record["artifact_execution_started"] = True
        record.setdefault("execution_started_at_utc", utc_now())
        write_json_durable(record_path, record)
        readback = load_json(record_path)
        if readback["executions"][-1] != attempt:
            raise RuntimeError(f"record-before-execution gate failed for {version}")
        try:
            outcome = launch_acp(
                binary,
                [str(binary), "--acp", "--stdio"],
                run_scratch,
                host_home,
                record_path,
                version,
            )
        except Exception as error:
            outcome = {
                "launch_error_class": type(error).__name__,
                "permission_request_observed": False,
                "permission_reply": "none",
                "effect_marker_present": False,
            }
        attempt.update(outcome)
        attempt["artifact_execution_result_recorded_at_utc"] = utc_now()
        attempt["permission_evidence_complete"] = bool(
            outcome.get("permission_request_observed")
            and outcome.get("permission_reply") == "cancelled"
            and outcome.get("session_prompt") == "cancelled"
            and not outcome.get("effect_marker_present")
        )
        write_json_durable(record_path, record)
        print(json.dumps({"version": version, "result": attempt}, sort_keys=True))
    record["permission_boundary_proven_for_all_targets"] = all(
        attempt.get("permission_evidence_complete") is True for attempt in record["executions"]
    )
    record["exact_permission_path_reached"] = any(
        attempt.get("permission_request_observed")
        or attempt.get("session_prompt") not in ("not-reached", None)
        for attempt in record["executions"]
    )
    if not record["permission_boundary_proven_for_all_targets"]:
        record["limitation"] = (
            "All exact stable artifacts advertised copilot-login and rejected session/new "
            "with Authentication required under the synthetic placeholder. No real "
            "authentication or network access was used, so no permission request, "
            "cancellation, or tool-effect boundary is claimed."
        )
    record["completion_recorded_at_utc"] = utc_now()
    write_json_durable(record_path, record)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--compile-permission-fake", action="store_true")
    parser.add_argument("--validate-plan", action="store_true")
    parser.add_argument("--validate-permission-plan", action="store_true")
    parser.add_argument("--validate-permission-record", action="store_true")
    parser.add_argument("--prepare", action="store_true")
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--record", type=Path)
    parser.add_argument("--artifact-root", type=Path)
    parser.add_argument("--permission-proof", action="store_true")
    parser.add_argument("--renewed-permission-proof", action="store_true")
    parser.add_argument("--validate-renewal-authority", action="store_true")
    parser.add_argument("--preflight-record", type=Path)
    args = parser.parse_args()
    try:
        if args.validate_plan:
            plan = validate_authenticated_plan()
            print(
                json.dumps(
                    {
                        "status": "passed",
                        "schema": plan["schema"],
                        "execution_authorized": plan["execution_authorized"],
                        "invocations": len(plan["invocations"]),
                        "missing_owner_attestations": len(plan["missing_owner_attestations"]),
                    },
                    sort_keys=True,
                )
            )
        elif args.validate_permission_plan:
            plan = validate_permission_proof_plan()
            correction = validate_permission_correction_plan(plan)
            print(
                json.dumps(
                    {
                        "status": "passed",
                        "operator_decision": plan["operator_decision"],
                        "execution_authorized": plan["execution_authorized"],
                        "start_order": plan["start_order"],
                        "allowed_hosts": len(plan["network_policy"]["allowed_hosts"]),
                        "maximum_prompts": plan["budgets"]["max_total_invocations"],
                        "correction_plan_sha256": hashlib.sha256(
                            PERMISSION_CORRECTION_PLAN_PATH.read_bytes()
                        ).hexdigest(),
                        "auth_metadata_read_paths": [
                            item["relative_path"]
                            for item in correction["auth_metadata_read_paths"]
                        ],
                    },
                    sort_keys=True,
                )
            )
        elif args.validate_permission_record and args.record:
            print(json.dumps(validate_permission_execution_record(args.record), sort_keys=True))
        elif args.compile_permission_fake:
            with tempfile.TemporaryDirectory(prefix="copilot-acp-native-fake-compile.") as temp_root:
                temp_path = Path(temp_root)
                executable = temp_path / "copilot-permission-fake"
                identity = compile_permission_fake(executable)
                environment = {
                    "HOME": str(temp_path),
                    "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
                    "TMPDIR": str(temp_path),
                    "SWALLOWTAIL_FAKE_DIAGNOSTIC": str(temp_path / "diagnostic.json"),
                }
                try:
                    smoke = subprocess.run(
                        [str(executable), "--early-exit"],
                        cwd=temp_path,
                        env=environment,
                        stdin=subprocess.DEVNULL,
                        stdout=subprocess.PIPE,
                        stderr=subprocess.PIPE,
                        timeout=5,
                        check=False,
                    )
                except subprocess.TimeoutExpired:
                    raise RuntimeError("native permission fake early-exit smoke exceeded 5 seconds") from None
                if (
                    smoke.returncode != 41
                    or smoke.stdout != b"FAKE_NATIVE_STARTED:--early-exit\n"
                    or smoke.stderr
                ):
                    raise RuntimeError(
                        "native permission fake early-exit smoke did not match its safe control"
                    )
            print(json.dumps({"status": "compiled-and-smoke-passed", **identity}, sort_keys=True))
        elif args.self_test:
            verify_inventory()
            proof = self_test()
            preflight_path = os.environ.get(
                "SWALLOWTAIL_COPILOT_PROOF_PREFLIGHT_RECORD"
            )
            if preflight_path:
                target = Path(preflight_path).resolve(strict=False)
                temp_root = Path(tempfile.gettempdir()).resolve()
                if not target.parent.resolve(strict=True).is_relative_to(temp_root):
                    raise RuntimeError("fake preflight records must stay in task temp scratch")
                preflight = make_permission_preflight_record(proof)
                write_json_durable(target, preflight)
                if load_json(target) != preflight:
                    raise RuntimeError("fake preflight record did not persist")
                validate_preflight_record(target, validate_permission_proof_plan())
            print(json.dumps(proof, indent=2))
        elif args.prepare and args.record:
            prepare_record(args.record)
        elif args.execute and args.record and args.artifact_root:
            execute_artifacts(args.record, args.artifact_root)
        elif (
            args.permission_proof
            and args.record
            and args.artifact_root
            and args.preflight_record
        ):
            run_permission_proof(args.record, args.artifact_root, args.preflight_record)
        elif (
            args.renewed_permission_proof
            and args.record
            and args.artifact_root
            and args.preflight_record
        ):
            run_renewed_permission_proof(args.record, args.artifact_root, args.preflight_record)
        elif args.validate_renewal_authority:
            authority = validate_committed_renewal_authority()
            print(
                json.dumps(
                    {
                        "status": "passed",
                        "operator_decision": authority["operator_decision"],
                        "operation_id": authority["operation_id"],
                        "version": authority["version"],
                        "start_order": authority["start_order"],
                        "budgets": authority["budgets"],
                    },
                    sort_keys=True,
                )
            )
        else:
            parser.error(
                "choose --validate-plan, --self-test, --prepare --record, "
                "--execute --record --artifact-root, or --permission-proof "
                "--record --artifact-root --preflight-record"
            )
    except (
        OSError,
        RuntimeError,
        ValueError,
        KeyError,
        EOFError,
        json.JSONDecodeError,
    ) as error:
        if isinstance(error, OSError):
            frames: list[str] = []
            trace = error.__traceback__
            while trace is not None:
                frames.append(
                    f"{trace.tb_frame.f_code.co_name}:{trace.tb_lineno}"
                )
                trace = trace.tb_next
            detail = f"{type(error).__name__}(errno={error.errno})"
            if frames:
                detail += " at " + " -> ".join(frames[-6:])
        else:
            detail = str(error)
        print(f"offline proof failed closed: {detail}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
