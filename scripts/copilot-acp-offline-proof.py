#!/usr/bin/env python3
"""Run Copilot ACP proof fixtures inside a network-denied macOS sandbox.

The fake proof validates containment and record ordering. Exact artifact runs
are separately gated by a durable, secret-free record created by --prepare.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import re
import selectors
import signal
import subprocess
import sys
import tempfile
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
FIXTURE_DIR = (
    ROOT
    / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof"
)
INVENTORY_PATH = FIXTURE_DIR / "artifact-inventory.json"
AUTHENTICATED_PLAN_PATH = FIXTURE_DIR / "authenticated-proof-plan.json"
AUTHENTICATED_SCHEMA_PATH = FIXTURE_DIR / "authenticated-proof-plan.schema.json"
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


def load_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text())
    if not isinstance(value, dict):
        raise ValueError(f"expected an object in {path.name}")
    return value


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
        "CI": "1",
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


def stop_process(process: subprocess.Popen[bytes], grace_seconds: float = 1.0) -> tuple[int | None, bool]:
    if process.stdin is not None:
        try:
            process.stdin.close()
        except OSError:
            pass
    forced = False
    try:
        code = process.wait(timeout=grace_seconds)
    except subprocess.TimeoutExpired:
        forced = True
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        code = process.wait(timeout=2.0)
    return code, forced


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
    sys.stderr.write(json.dumps(values, sort_keys=True) + "\n")
    sys.stderr.flush()

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
            stderr=subprocess.PIPE if exact_version is None else subprocess.DEVNULL,
            bufsize=0,
            start_new_session=True,
            pass_fds=pass_fds,
        )
    finally:
        if auth_read_fd is not None:
            os.close(auth_read_fd)
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
        code, forced = stop_process(process)
        evidence["exit_code"] = code
        evidence["forced_process_group_kill"] = forced
        evidence["process_joined"] = process.returncode is not None
        evidence["effect_marker_present"] = marker.exists()
        if exact_version is None and process.stderr is not None:
            diagnostic = process.stderr.read(4096).decode("utf-8", errors="replace")
            evidence["fake_diagnostic"] = (
                diagnostic.replace(str(host_home), "<host-home>")
                .replace(str(scratch), "<task-scratch>")
                .replace(FAKE_TOKEN, "<fake-token>")
                .replace(
                    FAKE_DELEGATED_CREDENTIAL.decode(),
                    "<fake-delegated-credential>",
                )[:500]
            )


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
    diagnostics = [
        json.loads(line)
        for line in outcome.get("fake_diagnostic", "").splitlines()
        if line.strip()
    ]
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


def self_test() -> dict[str, Any]:
    fake_shell = require_macos_sandbox()
    plan = validate_authenticated_plan()
    with tempfile.TemporaryDirectory(prefix="copilot-acp-offline-proof-") as temp_root:
        task_root = Path(temp_root).resolve()
        scratch = task_root / "task-scratch"
        scratch.mkdir()
        host_home = task_root / "fake-host-home"
        (host_home / ".copilot").mkdir(parents=True)
        (host_home / "Library" / "Keychains").mkdir(parents=True)
        repository_root = task_root / "fake-repository"
        repository_root.mkdir()
        for path, contents in (
            (host_home / ".copilot" / "config.json", b"fake config\n"),
            (host_home / ".copilot" / "settings.json", b"fake settings\n"),
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
            raise RuntimeError("fake ACP agent did not exit cleanly")
        auth_preflight, auth_result, discovery_result = authenticated_fake_run(
            scratch, host_home, repository_root
        )
        return {
            "status": "passed",
            "network_denial": "loopback and reserved external connect both returned EPERM/EACCES",
            "filesystem_boundary": "writes allowed only inside task scratch; fake host home and repository reads denied",
            "keychain_boundary": "securityd Mach lookup denied by profile",
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


def execute_artifacts(record_path: Path, artifact_root: Path) -> None:
    require_macos_sandbox()
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
    parser.add_argument("--validate-plan", action="store_true")
    parser.add_argument("--prepare", action="store_true")
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--record", type=Path)
    parser.add_argument("--artifact-root", type=Path)
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
        elif args.self_test:
            verify_inventory()
            print(json.dumps(self_test(), indent=2))
        elif args.prepare and args.record:
            prepare_record(args.record)
        elif args.execute and args.record and args.artifact_root:
            execute_artifacts(args.record, args.artifact_root)
        else:
            parser.error("choose --validate-plan, --self-test, --prepare --record, or --execute --record --artifact-root")
    except (OSError, RuntimeError, ValueError, KeyError, json.JSONDecodeError) as error:
        print(f"offline proof failed closed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
