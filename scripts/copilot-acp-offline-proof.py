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
VERSIONS = ("1.0.80", "1.0.81", "1.0.93")
MAX_FRAME_BYTES = 256 * 1024
READ_TIMEOUT_SECONDS = 5.0
RUN_TIMEOUT_SECONDS = 18.0
FAKE_TOKEN = "SWALLOWTAIL_OFFLINE_FAKE_TOKEN"


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds").replace(
        "+00:00", "Z"
    )


def json_bytes(value: Any) -> bytes:
    return (json.dumps(value, separators=(",", ":"), ensure_ascii=False) + "\n").encode()


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


def sandbox_profile(scratch: Path, host_home: Path) -> str:
    scratch = scratch.resolve()
    host_home = host_home.resolve()
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
            f"(allow file-write* (subpath {quote_profile_path(scratch)}))",
            "(deny network*)",
        )
    )


def clean_child_environment(scratch: Path) -> dict[str, str]:
    home = scratch / "home"
    temp = scratch / "tmp"
    config = scratch / "config"
    cache = scratch / "cache"
    for directory in (home, temp, config, cache):
        directory.mkdir(parents=True, exist_ok=True)
    return {
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "HOME": str(home),
        "COPILOT_HOME": str(home),
        "GH_COPILOT_HOME": str(home),
        "XDG_CONFIG_HOME": str(config),
        "XDG_CACHE_HOME": str(cache),
        "TMPDIR": str(temp),
        "GITHUB_TOKEN": FAKE_TOKEN,
        "GH_TOKEN": FAKE_TOKEN,
        "CI": "1",
        "TERM": "dumb",
    }


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
            evidence["permission_request_observed"] = True
            evidence["permission_methods_observed"].append(method)
            permission_id = message.get("id")
            if permission_id is not None:
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
            evidence["session_updates"] += 1
            update = message.get("params", {}).get("update", {})
            if update.get("sessionUpdate") == "tool_call":
                evidence["tool_call_updates"] += 1
            continue
        if message.get("id") == request_id:
            return message
        evidence["unexpected_messages"] += 1


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


def launch_acp(
    executable: Path,
    command: list[str],
    scratch: Path,
    host_home: Path,
    record_path: Path,
    exact_version: str | None,
) -> dict[str, Any]:
    if not command or Path(command[0]) != executable:
        raise ValueError("the launched executable must match the selected artifact")
    scratch.mkdir(parents=True, exist_ok=True)
    marker = scratch / "permission-effect-marker"
    if marker.exists():
        marker.unlink()
    env = clean_child_environment(scratch)
    env.update(
        {
            "SWALLOWTAIL_BLOCKED_HOST_HOME": str(host_home.resolve()),
            "SWALLOWTAIL_EXPECTED_HOME": env["HOME"],
            "SWALLOWTAIL_RECORD_PATH": str(record_path.resolve()),
            "SWALLOWTAIL_EFFECT_MARKER": str(marker),
        }
    )
    profile = sandbox_profile(scratch, host_home)
    argv = launch_command(profile, command, env)
    started = utc_now()
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
    )
    client = StdioClient(process)
    evidence: dict[str, Any] = {
        "started_at_utc": started,
        "selected_argv": (
            [Path(command[0]).name, *command[-2:]]
            if exact_version
            else ["python3", "fake ACP agent"]
        ),
        "initialize": "not-reached",
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
    try:
        client.send(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {"protocolVersion": 1, "clientCapabilities": {}},
            }
        )
        response = wait_for_response(client, 1, evidence, READ_TIMEOUT_SECONDS)
        if response is None or "result" not in response:
            evidence["initialize"] = "no-result"
            return evidence
        evidence["initialize"] = "success"
        result = response["result"]
        reported = result.get("agentInfo", {}).get("version")
        evidence["reported_version_matches"] = (
            reported == exact_version if exact_version else reported == "fake"
        )

        client.send(
            {
                "jsonrpc": "2.0",
                "id": 2,
                "method": "session/new",
                "params": {"cwd": str(scratch), "mcpServers": []},
            }
        )
        response = wait_for_response(client, 2, evidence, READ_TIMEOUT_SECONDS)
        if response is None or "result" not in response:
            evidence["session_new"] = "no-result"
            return evidence
        evidence["session_new"] = "success"
        session_id = response["result"].get("sessionId")
        if not isinstance(session_id, str):
            evidence["session_new"] = "result-without-session-id"
            return evidence

        marker_prompt = f"Run the tool needed to create this empty marker file, then stop: {marker}"
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
        evidence["session_prompt"] = "sent"
        response = wait_for_response(client, 3, evidence, RUN_TIMEOUT_SECONDS)
        if response is None:
            try:
                client.send(
                    {
                        "jsonrpc": "2.0",
                        "method": "session/cancel",
                        "params": {"sessionId": session_id},
                    }
                )
                evidence["session_cancel_sent"] = True
                response = wait_for_response(client, 3, evidence, 2.0)
            except (BrokenPipeError, OSError):
                evidence["stream_failed"] = True
        if response is not None:
            if "result" in response:
                evidence["session_prompt"] = str(
                    response["result"].get("stopReason", "result")
                )[:40]
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
        evidence["effect_marker_present"] = marker.exists()
        if exact_version is None and process.stderr is not None:
            diagnostic = process.stderr.read(4096).decode("utf-8", errors="replace")
            evidence["fake_diagnostic"] = (
                diagnostic.replace(str(host_home), "<host-home>")
                .replace(str(scratch), "<task-scratch>")
                .replace(FAKE_TOKEN, "<fake-token>")[:500]
            )


def require_macos_sandbox() -> str:
    if sys.platform != "darwin":
        raise RuntimeError("the offline containment proof requires macOS sandbox-exec")
    sandbox = "/usr/bin/sandbox-exec"
    if not os.path.isfile(sandbox) or not os.access(sandbox, os.X_OK):
        raise RuntimeError("/usr/bin/sandbox-exec is unavailable")
    return "/bin/bash"


def self_test() -> dict[str, Any]:
    fake_shell = require_macos_sandbox()
    host_home = Path(os.environ["HOME"]).resolve()
    with tempfile.TemporaryDirectory(prefix="copilot-acp-offline-proof-") as temp_root:
        scratch = Path(temp_root).resolve()
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
        profile = sandbox_profile(scratch, host_home)
        if "(deny network*)" not in profile or "(allow network" in profile:
            raise RuntimeError("sandbox profile does not deny every network operation")
        if f"(allow file-write* (subpath {quote_profile_path(scratch)}))" not in profile:
            raise RuntimeError("sandbox profile does not bind writes to task scratch")
        if f"(deny file-read* (subpath {quote_profile_path(host_home)}))" not in profile:
            raise RuntimeError("sandbox profile does not deny the host home")
        outcome = launch_acp(
            Path(fake_shell),
            [fake_shell, str(fake_path)],
            scratch,
            host_home,
            record_path,
            None,
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
        if outcome.get("effect_marker_present"):
            raise RuntimeError("fake ACP rejection allowed the tool effect")
        if outcome.get("exit_code") != 0 or outcome.get("forced_process_group_kill"):
            raise RuntimeError("fake ACP agent did not exit cleanly")
        return {
            "status": "passed",
            "network_denial": "loopback and reserved external connect both returned EPERM/EACCES",
            "filesystem_boundary": "writes allowed only inside task scratch; host home read denied",
            "keychain_boundary": "securityd Mach lookup denied by profile",
            "record_before_execution": True,
            "fake_acp": {
                "initialize": outcome["initialize"],
                "session_new": outcome["session_new"],
                "permission_request": "observed",
                "permission_reply": "cancelled",
                "prompt_stop_reason": outcome["session_prompt"],
                "tool_effect": "absent",
                "exit_code": outcome["exit_code"],
            },
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
        "pre_execution_record_persisted": True,
        "artifact_execution_started": False,
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
    record["completion_recorded_at_utc"] = utc_now()
    write_json_durable(record_path, record)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--prepare", action="store_true")
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--record", type=Path)
    parser.add_argument("--artifact-root", type=Path)
    args = parser.parse_args()
    try:
        if args.self_test:
            verify_inventory()
            print(json.dumps(self_test(), indent=2))
        elif args.prepare and args.record:
            prepare_record(args.record)
        elif args.execute and args.record and args.artifact_root:
            execute_artifacts(args.record, args.artifact_root)
        else:
            parser.error("choose --self-test, --prepare --record, or --execute --record --artifact-root")
    except (OSError, RuntimeError, ValueError, KeyError, json.JSONDecodeError) as error:
        print(f"offline proof failed closed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
