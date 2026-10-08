#!/usr/bin/env python3
"""Run the approved, offline Kiro ACP owner-state proof in a task container."""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import pathlib
import platform
import re
import selectors
import shutil
import signal
import socket
import stat
import subprocess
import sys
import tempfile
import tarfile
import threading
import time
import urllib.request
import uuid
from typing import Any


REPO_ROOT = pathlib.Path(__file__).resolve().parents[1]
RESEARCH = REPO_ROOT / "docs/research/394-kiro-acp-2-28-0-owner-only-state-evidence-stop.md"
DOCKERFILE = REPO_ROOT / "scripts/kiro-acp-owner-state-offline.Dockerfile"
CONTAINER = "swallowtail-kiro-owner-proof-trace-exec"
IMAGE = "swallowtail-kiro-owner-proof:task-150"
CONTEXT = "colima-swallowtail-kiro-owner-proof"
ARTIFACT_NAME = "kirocli-aarch64-linux.tar.xz"
ARTIFACT_BASE = "https://prod.download.cli.kiro.dev/stable"
VERSIONS = ("2.26.1", "2.27.0", "2.27.1", "2.28.0")
OWNER_UID = 21001
OTHER_UID = 21002
SCRIPT_IN_CONTAINER = "/scratch/kiro-owner-proof.py"
MAX_RPC_WAIT_SECONDS = 5.0
MAX_PROCESS_SECONDS = 18.0


def utc_now() -> str:
    return dt.datetime.now(dt.timezone.utc).isoformat(timespec="milliseconds")


def sha256_file(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_json_fsync(path: pathlib.Path, value: Any) -> str:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".partial")
    payload = (json.dumps(value, indent=2, sort_keys=True) + "\n").encode()
    with temporary.open("wb") as stream:
        stream.write(payload)
        stream.flush()
        os.fsync(stream.fileno())
    os.replace(temporary, path)
    directory_fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(directory_fd)
    finally:
        os.close(directory_fd)
    return hashlib.sha256(payload).hexdigest()


def load_inventory() -> dict[str, Any]:
    text = RESEARCH.read_text(encoding="utf-8")
    for match in re.finditer(r"```json\s*(.*?)\s*```", text, re.DOTALL):
        inventory = json.loads(match.group(1))
        if isinstance(inventory, dict) and "versions" in inventory and "records" in inventory:
            missing = set(VERSIONS) - set(inventory["records"])
            if missing:
                raise RuntimeError(f"frozen Research 394 inventory is missing {sorted(missing)}")
            return inventory
    raise RuntimeError("frozen Research 394 tree inventory was not found")


def docker_prefix(context: str) -> list[str]:
    return ["docker", "--context", context]


def run_host(command: list[str], *, timeout: float = 30.0, input_text: str | None = None) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(
        command,
        input=input_text,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=timeout,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(
            f"command failed ({result.returncode}): {command!r}\n"
            f"stdout: {result.stdout[-4000:]}\nstderr: {result.stderr[-4000:]}"
        )
    return result


def docker(context: str, *args: str, timeout: float = 30.0, input_text: str | None = None) -> subprocess.CompletedProcess[str]:
    return run_host(docker_prefix(context) + list(args), timeout=timeout, input_text=input_text)


def docker_exec(
    context: str,
    *args: str,
    user: int | None = None,
    timeout: float = 30.0,
    input_text: str | None = None,
) -> subprocess.CompletedProcess[str]:
    prefix = ["exec"]
    if input_text is not None:
        prefix.append("-i")
    if user is not None:
        prefix.extend(("--user", f"{user}:{user}"))
    return docker(context, *prefix, CONTAINER, *args, timeout=timeout, input_text=input_text)


def docker_copy_file(
    context: str, source: pathlib.Path, destination: str, *, timeout: float = 120.0
) -> None:
    if not destination.startswith("/scratch/") or ".." in pathlib.PurePosixPath(destination).parts:
        raise RuntimeError(f"refusing to copy outside the task scratch tmpfs: {destination!r}")
    docker_exec(
        context,
        "sh",
        "-c",
        'if [ -e "$1" ]; then chmod u+w "$1"; fi',
        "sh",
        destination,
        user=0,
        timeout=10.0,
    )
    command = docker_prefix(context) + [
        "exec",
        "-i",
        "--user",
        "0:0",
        CONTAINER,
        "sh",
        "-c",
        'cat > "$1"',
        "sh",
        destination,
    ]
    try:
        with source.open("rb") as stream:
            result = subprocess.run(
                command,
                stdin=stream,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                timeout=timeout,
                check=False,
            )
    except subprocess.TimeoutExpired as error:
        raise RuntimeError(f"bounded file staging timed out for {destination}") from error
    if result.returncode != 0:
        stderr = result.stderr.decode("utf-8", errors="replace")[-2000:]
        raise RuntimeError(f"staging to task scratch failed ({result.returncode}): {stderr}")


def export_container_evidence(context: str, destination: pathlib.Path) -> None:
    archive = destination.parent / "container-evidence.tar"
    command = docker_prefix(context) + [
        "exec",
        CONTAINER,
        "tar",
        "-C",
        "/scratch/evidence",
        "-cf",
        "-",
        "traces",
        "results",
    ]
    with archive.open("wb") as output:
        process = subprocess.Popen(command, stdout=output, stderr=subprocess.PIPE)
        try:
            _, stderr = process.communicate(timeout=120.0)
        except subprocess.TimeoutExpired as error:
            process.kill()
            _, _ = process.communicate()
            raise RuntimeError("bounded task evidence export timed out") from error
        if process.returncode != 0:
            raise RuntimeError(
                "task evidence export failed: "
                + stderr.decode("utf-8", errors="replace")[-2000:]
            )
        output.flush()
        os.fsync(output.fileno())
    destination.mkdir(mode=0o700, parents=True, exist_ok=True)
    with tarfile.open(archive, mode="r:") as bundle:
        for member in bundle.getmembers():
            relative = pathlib.PurePosixPath(member.name)
            if (
                relative.is_absolute()
                or ".." in relative.parts
                or not relative.parts
                or relative.parts[0] not in {"traces", "results"}
                or not (member.isdir() or member.isfile())
            ):
                raise RuntimeError(f"task evidence archive contains an unexpected entry: {member.name!r}")
            target = destination.joinpath(*relative.parts)
            if member.isdir():
                target.mkdir(mode=0o700, parents=True, exist_ok=True)
                continue
            target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            stream = bundle.extractfile(member)
            if stream is None:
                raise RuntimeError(f"task evidence archive entry is unreadable: {member.name!r}")
            with stream, target.open("wb") as output:
                shutil.copyfileobj(stream, output)
                output.flush()
                os.fsync(output.fileno())


def inspect_container(context: str) -> tuple[dict[str, Any], dict[str, Any]]:
    raw = docker(context, "inspect", CONTAINER).stdout
    value = json.loads(raw)[0]
    host = value["HostConfig"]
    tmpfs = host.get("Tmpfs") or {}
    mounts = value.get("Mounts") or []
    binds = host.get("Binds") or []
    devices = host.get("Devices") or []
    device_requests = host.get("DeviceRequests") or []
    cap_add = host.get("CapAdd") or []
    security_options = host.get("SecurityOpt") or []
    if value["State"]["Running"] is not True:
        raise RuntimeError("task-owned proof container is not running")
    if host.get("NetworkMode") != "none":
        raise RuntimeError(f"container network is not disabled: {host.get('NetworkMode')!r}")
    if host.get("ReadonlyRootfs") is not True:
        raise RuntimeError("container root filesystem is writable")
    if "ALL" not in (host.get("CapDrop") or []):
        raise RuntimeError("container capabilities are not all dropped")
    if set(cap_add) != {"CAP_SYS_PTRACE", "CAP_SETUID", "CAP_SETGID"}:
        raise RuntimeError(f"container capabilities do not match bounded tracing and UID switching: {cap_add!r}")
    if set(security_options) != {"no-new-privileges:true"}:
        raise RuntimeError(f"container security options do not match the bounded trace profile: {security_options!r}")
    if host.get("PidMode"):
        raise RuntimeError(f"container shares a host or external PID namespace: {host.get('PidMode')!r}")
    if binds or devices or device_requests:
        raise RuntimeError("container has a bind, host device, or device request")
    if mounts:
        raise RuntimeError(f"container has unexpected runtime mounts: {mounts!r}")
    if set(tmpfs) != {"/scratch"}:
        raise RuntimeError(f"container writable tmpfs is not limited to /scratch: {tmpfs!r}")
    scratch_options = set(tmpfs["/scratch"].split(","))
    if "exec" not in scratch_options or "noexec" in scratch_options:
        raise RuntimeError(f"task scratch tmpfs must permit staged executable artifacts: {tmpfs['/scratch']!r}")
    if int(host.get("PidsLimit") or 0) <= 0 or int(host.get("Memory") or 0) <= 0:
        raise RuntimeError("container process or memory bound is missing")
    env_names = sorted(item.split("=", 1)[0] for item in value["Config"].get("Env", []))
    sensitive_names = [
        name
        for name in env_names
        if re.search(r"AWS|KIRO|TOKEN|SECRET|CREDENTIAL|API.?KEY", name, re.IGNORECASE)
    ]
    if sensitive_names:
        raise RuntimeError(f"container image has unexpected sensitive environment names: {sensitive_names}")
    summary = {
        "container_id": value["Id"],
        "image_id": value["Image"],
        "network_mode": host["NetworkMode"],
        "root_read_only": host["ReadonlyRootfs"],
        "cap_drop": host.get("CapDrop") or [],
        "cap_add": cap_add,
        "security_options": security_options,
        "pid_mode": host.get("PidMode") or "private",
        "binds": binds,
        "mounts": mounts,
        "tmpfs": tmpfs,
        "devices": devices,
        "device_requests": device_requests,
        "pids_limit": host["PidsLimit"],
        "memory_bytes": host["Memory"],
        "cpu_nano": host.get("NanoCpus"),
        "container_environment_names": env_names,
    }
    return value, summary


def verify_colima_profile(context: str) -> dict[str, Any]:
    profile_name = "swallowtail-kiro-owner-proof"
    profile_rows = run_host(["colima", "list", "--json"]).stdout.splitlines()
    profiles = [json.loads(line) for line in profile_rows if line.strip()]
    profile = next((item for item in profiles if item.get("name") == profile_name), None)
    if not profile or profile.get("status") != "Running" or profile.get("arch") != "aarch64" or profile.get("runtime") != "docker":
        raise RuntimeError(f"task Colima profile is not running as native aarch64 Docker: {profile!r}")
    config_path = pathlib.Path.home() / ".colima" / profile_name / "colima.yaml"
    config_text = config_path.read_text(encoding="utf-8")
    required = {
        "arch": r"(?m)^arch:\s+aarch64\s*$",
        "runtime": r"(?m)^runtime:\s+docker\s*$",
        "autoActivate": r"(?m)^autoActivate:\s+false\s*$",
        "vmType": r"(?m)^vmType:\s+vz\s*$",
        "forwardAgent": r"(?m)^forwardAgent:\s+false\s*$",
        "portForwarder": r"(?m)^portForwarder:\s+none\s*$",
        "rosetta": r"(?m)^rosetta:\s+false\s*$",
        "binfmt": r"(?m)^binfmt:\s+false\s*$",
        "sshConfig": r"(?m)^sshConfig:\s+false\s*$",
        "mounts": r"(?m)^mounts:\s+null\s*$",
    }
    missing = [name for name, pattern in required.items() if not re.search(pattern, config_text)]
    if missing:
        raise RuntimeError(f"task Colima isolation configuration changed: {missing}")
    guest_machine = run_host(["colima", "--profile", profile_name, "ssh", "--", "uname", "-m"]).stdout.strip()
    guest_glibc = run_host(["colima", "--profile", profile_name, "ssh", "--", "getconf", "GNU_LIBC_VERSION"]).stdout.strip()
    mounts = run_host(["colima", "--profile", profile_name, "ssh", "--", "findmnt", "-rn", "-o", "FSTYPE,TARGET"]).stdout.splitlines()
    binfmt = run_host(["colima", "--profile", profile_name, "ssh", "--", "ls", "/proc/sys/fs/binfmt_misc"]).stdout.splitlines()
    if guest_machine != "aarch64" or not guest_glibc.startswith("glibc "):
        raise RuntimeError(f"task Linux guest is not native AArch64 GNU: {guest_machine}/{guest_glibc}")
    if any("virtiofs" in item or "9p" in item or "sshfs" in item for item in mounts):
        raise RuntimeError(f"task Linux guest has an unexpected host filesystem mount: {mounts!r}")
    handler_names = [item for item in binfmt if item not in {"register", "status"}]
    benign_handlers = []
    for handler in handler_names:
        if handler != "python3.12":
            raise RuntimeError(f"task Linux guest has an unreviewed binfmt handler: {handler!r}")
        descriptor = run_host(
            ["colima", "--profile", profile_name, "ssh", "--", "cat", f"/proc/sys/fs/binfmt_misc/{handler}"]
        ).stdout.splitlines()
        interpreter = run_host(
            [
                "colima",
                "--profile",
                profile_name,
                "ssh",
                "--",
                "python3.12",
                "-c",
                "import importlib.util,json,platform,sys; print(json.dumps({'machine':platform.machine(),'version':list(sys.version_info[:2]),'magic':importlib.util.MAGIC_NUMBER.hex()}))",
            ]
        ).stdout.strip()
        python_identity = json.loads(interpreter)
        descriptor_magic = next((line.split(maxsplit=1)[1] for line in descriptor if line.startswith("magic ")), None)
        if (
            "enabled" not in descriptor
            or "interpreter /usr/bin/python3.12" not in descriptor
            or descriptor_magic != python_identity["magic"]
            or python_identity["machine"] != "aarch64"
            or python_identity["version"] != [3, 12]
        ):
            raise RuntimeError(f"task Linux guest Python binfmt handler did not match its native interpreter")
        benign_handlers.append(
            {"name": handler, "kind": "native-python-bytecode", "magic": descriptor_magic, "machine": python_identity["machine"]}
        )
    docker_info = docker(context, "info", "--format", "{{.OSType}}/{{.Architecture}}").stdout.strip()
    if docker_info != "linux/aarch64":
        raise RuntimeError(f"task Docker backend is not linux/aarch64: {docker_info}")
    return {
        "profile": profile_name,
        "profile_status": profile["status"],
        "profile_arch": profile["arch"],
        "profile_runtime": profile["runtime"],
        "vm_type": "vz",
        "binfmt_configuration_enabled": False,
        "rosetta_enabled": False,
        "guest_machine": guest_machine,
        "guest_glibc": guest_glibc,
        "guest_host_filesystem_mounts": [],
        "foreign_arch_binfmt_handlers": [],
        "benign_binfmt_handlers": benign_handlers,
        "docker_backend": docker_info,
    }


def prepare_container(context: str, evidence: pathlib.Path) -> None:
    build_context = evidence / "image-build-context"
    build_context.mkdir(mode=0o700)
    shutil.copyfile(DOCKERFILE, build_context / "Dockerfile")
    run_host(
        docker_prefix(context)
        + ["build", "--platform", "linux/arm64", "-t", IMAGE, str(build_context)],
        timeout=600.0,
    )
    docker(
        context,
        "run",
        "--detach",
        "--name",
        CONTAINER,
        "--network",
        "none",
        "--read-only",
        "--cap-drop",
        "ALL",
        "--cap-add",
        "SYS_PTRACE",
        "--cap-add",
        "SETUID",
        "--cap-add",
        "SETGID",
        "--security-opt",
        "no-new-privileges:true",
        "--pids-limit",
        "128",
        "--memory",
        "5g",
        "--cpus",
        "4",
        "--tmpfs",
        "/scratch:rw,exec,nosuid,nodev,size=8g,mode=1777",
        IMAGE,
        timeout=120.0,
    )


def download_artifacts(inventory: dict[str, Any], evidence: pathlib.Path) -> dict[str, dict[str, Any]]:
    artifact_dir = evidence / "artifacts"
    artifact_dir.mkdir(mode=0o700, exist_ok=True)
    result: dict[str, dict[str, Any]] = {}
    for version in VERSIONS:
        record = inventory["records"][version]
        target = artifact_dir / f"{version}-{ARTIFACT_NAME}"
        expected_sha = record["archive_sha256"]
        expected_size = int(record["archive_size"])
        if not target.is_file() or sha256_file(target) != expected_sha:
            temporary = target.with_suffix(target.suffix + ".partial")
            url = f"{ARTIFACT_BASE}/{version}/{ARTIFACT_NAME}"
            request = urllib.request.Request(
                url,
                headers={"User-Agent": "swallowtail-kiro-acp-owner-proof/150"},
            )
            opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
            with opener.open(request, timeout=120) as response:
                with temporary.open("wb") as output:
                    shutil.copyfileobj(response, output, length=1024 * 1024)
                    output.flush()
                    os.fsync(output.fileno())
            os.replace(temporary, target)
            directory_fd = os.open(artifact_dir, os.O_RDONLY)
            try:
                os.fsync(directory_fd)
            finally:
                os.close(directory_fd)
        actual_sha = sha256_file(target)
        actual_size = target.stat().st_size
        if actual_sha != expected_sha or actual_size != expected_size:
            raise RuntimeError(
                f"{version} artifact identity mismatch: sha256={actual_sha}, size={actual_size}"
            )
        result[version] = {
            "path": str(target),
            "url": f"{ARTIFACT_BASE}/{version}/{ARTIFACT_NAME}",
            "archive_sha256": actual_sha,
            "archive_size": actual_size,
        }
    return result


def persist_plan_in_container(context: str, path: str, digest: str) -> None:
    docker_exec(
        context,
        "python3",
        SCRIPT_IN_CONTAINER,
        "--inside",
        "persist-plan",
        "--path",
        path,
        "--sha256",
        digest,
        user=0,
        timeout=20.0,
    )


def seed_in_container(context: str, plan: dict[str, Any], uid: int) -> dict[str, Any]:
    result = docker_exec(
        context,
        "python3",
        SCRIPT_IN_CONTAINER,
        "--inside",
        "seed",
        user=uid,
        timeout=20.0,
        input_text=json.dumps(plan),
    )
    return json.loads(result.stdout)


def run_attempt(
    context: str,
    evidence: pathlib.Path,
    env_summary: dict[str, Any],
    artifact: dict[str, Any],
    inventory_record: dict[str, Any],
    case_id: str,
    case_root: str,
    scenario: str,
    uid: int,
    home: str,
    cwd: str,
    tmpdir: str,
    kiro_home: str,
    method: str,
    session_id: str | None = None,
    state_before: dict[str, Any] | None = None,
    snapshot_uid: int | None = None,
) -> dict[str, Any]:
    attempt_id = f"{case_id}-{method.replace('/', '-')}-{uid}"
    binary = f"/scratch/artifacts/{artifact['version']}/kirocli/bin/kiro-cli"
    snapshot_uid = snapshot_uid or uid
    state_precondition = json.loads(
        docker_exec(
            context,
            "python3",
            SCRIPT_IN_CONTAINER,
            "--inside",
            "snapshot",
            "--scenario",
            scenario,
            "--kiro-home",
            kiro_home,
            "--case-root",
            case_root,
            user=snapshot_uid,
            timeout=20.0,
        ).stdout
    )
    rpc_requests: list[dict[str, Any]] = [
        {
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {
                "protocolVersion": 1,
                "clientCapabilities": {
                    "fs": {"readTextFile": False, "writeTextFile": False},
                    "terminal": False,
                },
                "clientInfo": {"name": "swallowtail", "version": "0.5.1"},
            },
        }
    ]
    if method == "session/new":
        rpc_requests.append(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": method,
                "params": {"cwd": cwd, "mcpServers": []},
            }
        )
    else:
        rpc_requests.append(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "session/load",
                "params": {"sessionId": session_id, "cwd": cwd, "mcpServers": []},
            }
        )
    plan = {
        "plan_version": 1,
        "attempt_id": attempt_id,
        "created_at_utc": utc_now(),
        "task": {
            "task_id": "21376339-8146-4426-ab46-69d920b8442d",
            "run_id": "f698dedf-08ae-472f-89cf-5a1f4b321580",
            "brief_sha256": "63481658eb0c78949ac10a1955c4768be36f9a860380551c1a527e36678d5ef4",
            "authority_decisions": [
                "73ff6b53-f8be-4d7f-9031-c8b0a81e3ff1",
                "8079018e-2ec0-45fd-aaf3-0351c1e3ff1f",
                "Tom approved provider-free offline evidence, not qualification",
                "Tom designated a fresh native Linux ARM64 GNU environment",
            ],
        },
        "containment": env_summary,
        "identity": {
            "version": artifact["version"],
            "axis": "kiro-cli.release",
            "route": "kiro.acp",
            "build": inventory_record["build"],
            "archive_sha256": artifact["archive_sha256"],
            "archive_size": artifact["archive_size"],
            "selected_files": {
                name: inventory_record["files"][name]
                for name in ("kirocli/BUILD-INFO", "kirocli/bin/kiro-cli", "kirocli/bin/kiro-cli-chat")
            },
        },
        "invocation": {
            "argv": [binary, "acp"],
            "uid": uid,
            "cwd": cwd,
            "environment": {
                "HOME": home,
                "KIRO_HOME": kiro_home,
                "TMPDIR": tmpdir,
                "XDG_CACHE_HOME": f"{case_root}/cache-{uid}",
                "XDG_CONFIG_HOME": f"{case_root}/config-{uid}",
                "XDG_STATE_HOME": f"{case_root}/state-{uid}",
                "XDG_RUNTIME_DIR": tmpdir,
                "PATH": "/usr/bin:/bin",
            },
            "rpc_requests": rpc_requests,
            "session_access_is_observational": method == "session/load",
            "session_prompt_sent": False,
            "tool_calls_started": False,
            "permission_approvals_sent": False,
            "timeout_seconds": MAX_PROCESS_SECONDS,
        },
        "synthetic_state": {
            "case_root": case_root,
            "scenario": scenario,
            "state_precondition": state_precondition,
            "contents_are_synthetic": True,
            "file_contents_recorded": False,
        },
    }
    host_plan = evidence / "plans" / f"{attempt_id}.json"
    plan_sha = write_json_fsync(host_plan, plan)
    container_plan = f"/scratch/evidence/plans/{attempt_id}.json"
    docker_copy_file(context, host_plan, container_plan, timeout=30.0)
    persist_plan_in_container(context, container_plan, plan_sha)
    start_record = {
        "attempt_id": attempt_id,
        "plan_sha256": plan_sha,
        "persisted_at_utc": utc_now(),
        "container_id": env_summary["container_id"],
        "artifact_sha256": artifact["archive_sha256"],
        "uid": uid,
    }
    start_path = evidence / "plans" / f"{attempt_id}.start.json"
    start_sha = write_json_fsync(start_path, start_record)
    container_start = f"/scratch/evidence/plans/{attempt_id}.start.json"
    docker_copy_file(context, start_path, container_start, timeout=30.0)
    persist_plan_in_container(context, container_start, start_sha)
    result = docker_exec(
        context,
        "python3",
        SCRIPT_IN_CONTAINER,
        "--inside",
        "run-acp",
        "--plan",
        container_plan,
        "--sha256",
        plan_sha,
        "--start-sha256",
        start_sha,
        user=0,
        timeout=MAX_PROCESS_SECONDS + 15,
    )
    outcome = json.loads(result.stdout)
    state_after = json.loads(
        docker_exec(
            context,
            "python3",
            SCRIPT_IN_CONTAINER,
            "--inside",
            "snapshot",
            "--scenario",
            scenario,
            "--kiro-home",
            kiro_home,
            "--case-root",
            case_root,
            user=snapshot_uid,
            timeout=20.0,
        ).stdout
    )
    outcome["attempt_plan_sha256"] = plan_sha
    outcome["start_record_sha256"] = start_sha
    outcome["state_before_seed"] = state_before
    outcome["state_before"] = state_precondition
    outcome["state_after"] = state_after
    outcome["state_delta"] = state_delta(state_precondition, state_after)
    syscall_counts = outcome["state_syscall_trace"]["syscall_counts_under_synthetic_state"]
    traced_modes = [
        item
        for item in outcome["state_syscall_trace"]["operations"]
        if item["syscall"] in {"chmod", "fchmodat", "fchmodat2"}
    ]
    outcome["selected_state_path_evidence"] = {
        "state_path_access_observed": bool(outcome["state_syscall_trace"]["operation_count"]),
        "directory_enumeration_calls": syscall_counts.get("getdents64", 0),
        "chmod_attempts": len(traced_modes),
        "successful_chmod_attempts": sum(item["result"] == "0" for item in traced_modes),
        "metadata_mode_changes": sum(
            "mode" in item["fields"] for item in outcome["state_delta"]["metadata_changes"]
        ),
        "interpretation": "syscall and before/after metadata evidence; stripped function symbols are not directly traced",
    }
    outcome_path = evidence / "results" / f"{attempt_id}.json"
    write_json_fsync(outcome_path, outcome)
    return outcome


def prep_case(
    context: str,
    version: str,
    label: str,
    scenario: str,
    state_uid: int,
    inventory_record: dict[str, Any],
) -> tuple[str, str, str, str, str, dict[str, Any]]:
    version_root = f"/scratch/cases/{version}"
    docker_exec(context, "mkdir", "-p", version_root, user=0)
    docker_exec(context, "chmod", "1777", version_root, user=0)
    case_root = f"/scratch/cases/{version}/{label}"
    home = f"{case_root}/home-{OWNER_UID}"
    cwd = f"{case_root}/cwd"
    tmpdir = f"{case_root}/tmp-{OWNER_UID}"
    if scenario == "canonical-alias":
        kiro_home = f"{case_root}/kiro-alias"
    elif scenario == "non-directory-home":
        kiro_home = f"{case_root}/kiro-file"
    elif scenario == "missing-home":
        kiro_home = f"{case_root}/missing-kiro"
    else:
        kiro_home = f"{case_root}/kiro-home"
    synthetic_id = str(uuid.uuid5(uuid.NAMESPACE_DNS, f"swallowtail-kiro-owner:{version}:{label}"))
    seed = {
        "case_root": case_root,
        "scenario": scenario,
        "uid": state_uid,
        "process_uid": OWNER_UID,
        "state_uid": state_uid,
        "home": home,
        "cwd": cwd,
        "tmpdir": tmpdir,
        "kiro_home": kiro_home,
        "session_id": synthetic_id,
        "create_misowned_entry": False,
        "home_only": False,
    }
    seed_result = seed_in_container(context, seed, state_uid)
    if state_uid != OWNER_UID:
        seed["uid"] = OWNER_UID
        seed["home_only"] = True
        seed_result = seed_in_container(context, seed, OWNER_UID)
    if scenario == "misowned-entry":
        seed["uid"] = OTHER_UID
        seed["create_misowned_entry"] = True
        seed_result = seed_in_container(context, seed, OTHER_UID)
    return case_root, home, cwd, tmpdir, kiro_home, seed_result


def inaccessible_boundary(outcome: dict[str, Any]) -> str | None:
    calls = outcome.get("rpc", [])
    initialize = next((item for item in calls if item.get("method") == "initialize"), None)
    response = initialize.get("response") if initialize else None
    if not isinstance(response, dict) or "result" not in response:
        error = response.get("error") if isinstance(response, dict) else None
        return f"initialize did not succeed; response={error!r}"
    classifications = outcome.get("stderr_classification", {})
    messages = [
        call.get("response", {}).get("error", {}).get("message", "")
        for call in calls
        if isinstance(call.get("response"), dict)
    ]
    if classifications.get("authentication_related") or re.search(
        r"(?i)auth|credential|login|sign[ -]?in|unauthenticated", " ".join(messages)
    ):
        return "runtime reported an authentication or credential boundary"
    if outcome.get("process_timed_out"):
        return "ACP process reached its bounded runtime deadline"
    if outcome.get("process_exit_code") not in (0, None):
        return f"ACP process exited with code {outcome.get('process_exit_code')}"
    return None


def run_validation(context: str, evidence_arg: str | None) -> pathlib.Path:
    evidence = pathlib.Path(evidence_arg) if evidence_arg else pathlib.Path(
        tempfile.mkdtemp(prefix="swallowtail-kiro-owner-proof-")
    )
    evidence.mkdir(mode=0o700, parents=True, exist_ok=True)
    for name in ("artifacts", "plans", "results", "traces", "container-evidence"):
        (evidence / name).mkdir(mode=0o700, exist_ok=True)
    inventory = load_inventory()
    docker_copy_file(context, pathlib.Path(__file__).resolve(), SCRIPT_IN_CONTAINER, timeout=30.0)
    docker_exec(context, "chmod", "0444", SCRIPT_IN_CONTAINER, user=0)
    _, env_summary = inspect_container(context)
    env_summary["colima"] = verify_colima_profile(context)
    environment_check = docker_exec(
        context,
        "python3",
        SCRIPT_IN_CONTAINER,
        "--inside",
        "environment-check",
        timeout=20.0,
    )
    guest = json.loads(environment_check.stdout)
    if guest["machine"] != "aarch64" or not guest["glibc"].startswith("glibc "):
        raise RuntimeError(f"container runtime is not native AArch64 GNU: {guest!r}")
    if "noexec" in guest["scratch_mount"].split()[3].split(","):
        raise RuntimeError(f"task scratch tmpfs does not allow exact vendor executable staging: {guest['scratch_mount']!r}")
    if guest["interfaces"] != ["lo"] or guest["default_routes"]:
        raise RuntimeError(f"container has a non-loopback network path: {guest!r}")
    env_summary["guest"] = guest
    write_json_fsync(evidence / "containment.json", env_summary)

    docker_exec(
        context,
        "mkdir",
        "-p",
        "/scratch/evidence/plans",
        "/scratch/evidence/results",
        "/scratch/evidence/traces",
        "/scratch/cases",
        "/scratch/artifacts",
        "/scratch/stage",
        user=0,
    )
    for path in ("/scratch/evidence", "/scratch/evidence/plans", "/scratch/evidence/results", "/scratch/evidence/traces", "/scratch/cases", "/scratch/artifacts", "/scratch/stage"):
        docker_exec(context, "chmod", "1777", path, user=0)

    fake_result = docker_exec(
        context,
        "python3",
        SCRIPT_IN_CONTAINER,
        "--inside",
        "fake-self-test",
        user=0,
        timeout=15.0,
    )
    fake = json.loads(fake_result.stdout)
    write_json_fsync(evidence / "fake-self-test.json", fake)
    if fake.get("passed") is not True:
        raise RuntimeError(f"fake-first isolation checks did not pass: {fake!r}")

    artifacts = download_artifacts(inventory, evidence)
    verified_artifacts: dict[str, dict[str, Any]] = {}
    for version in VERSIONS:
        staged = artifacts[version]
        archive_path = f"/scratch/stage/{version}.tar.xz"
        docker_copy_file(context, pathlib.Path(staged["path"]), archive_path, timeout=240.0)
        copied_archive_sha = docker_exec(context, "sha256sum", archive_path, user=0).stdout.split()[0]
        if copied_archive_sha != staged["archive_sha256"]:
            raise RuntimeError(f"staged {version} archive digest changed across the container boundary")
        extract_dir = f"/scratch/artifacts/{version}"
        docker_exec(context, "mkdir", "-p", extract_dir, user=0)
        docker_exec(
            context,
            "tar",
            "--no-same-owner",
            "--no-same-permissions",
            "-xJf",
            f"/scratch/stage/{version}.tar.xz",
            "-C",
            extract_dir,
            user=0,
            timeout=120.0,
        )
        artifact_expectation = {
            "version": version,
            "files": {
                name: inventory["records"][version]["files"][name]
                for name in ("kirocli/BUILD-INFO", "kirocli/bin/kiro-cli", "kirocli/bin/kiro-cli-chat")
            },
            "build": inventory["records"][version]["build"],
            "archive_sha256": staged["archive_sha256"],
            "archive_size": staged["archive_size"],
        }
        verified = docker_exec(
            context,
            "python3",
            SCRIPT_IN_CONTAINER,
            "--inside",
            "verify-artifact",
            user=0,
            timeout=120.0,
            input_text=json.dumps(artifact_expectation),
        )
        artifact_result = json.loads(verified.stdout)
        write_json_fsync(evidence / "results" / f"artifact-{version}.json", artifact_result)
        if artifact_result.get("verified") is not True:
            raise RuntimeError(f"exact {version} archive contents did not match Research 394")
        verified_artifacts[version] = {**staged, "version": version, "verification": artifact_result}

    attempts: list[dict[str, Any]] = []
    stop_reason: str | None = None
    for version in VERSIONS:
        record = inventory["records"][version]
        artifact = verified_artifacts[version]
        owner_case = f"{version}-owned-session"
        case_root, home, cwd, tmpdir, kiro_home, seed_before = prep_case(
            context, version, owner_case, "owned", OWNER_UID, record
        )
        new_result = run_attempt(
            context,
            evidence,
            env_summary,
            artifact,
            record,
            owner_case,
            case_root,
            "owned",
            OWNER_UID,
            home,
            cwd,
            tmpdir,
            kiro_home,
            "session/new",
            state_before=seed_before,
        )
        attempts.append(new_result)
        stop_reason = inaccessible_boundary(new_result)
        if stop_reason:
            stop_reason = f"{version} owned session/new boundary: {stop_reason}"
            break
        created_id = extract_session_id(new_result) or seed_before["session_id"]
        # Create a private HOME/TMPDIR for the second principal while preserving the first user's KIRO_HOME.
        cross_root = case_root
        other_env = {
            "case_root": cross_root,
            "scenario": "cross-owner-session",
            "uid": OTHER_UID,
            "process_uid": OTHER_UID,
            "state_uid": OWNER_UID,
            "home": f"{cross_root}/home-{OTHER_UID}",
            "cwd": cwd,
            "tmpdir": f"{cross_root}/tmp-{OTHER_UID}",
            "kiro_home": kiro_home,
            "session_id": created_id,
            "home_only": True,
            "create_misowned_entry": False,
        }
        other_seed = seed_in_container(context, other_env, OTHER_UID)
        cross_result = run_attempt(
            context,
            evidence,
            env_summary,
            artifact,
            record,
            f"{version}-foreign-session-access",
            cross_root,
            "foreign-owner-session-load",
            OTHER_UID,
            other_env["home"],
            other_env["cwd"],
            other_env["tmpdir"],
            kiro_home,
            "session/load",
            session_id=created_id,
            state_before=other_seed,
            snapshot_uid=OWNER_UID,
        )
        attempts.append(cross_result)
        stop_reason = inaccessible_boundary(cross_result)
        if stop_reason:
            stop_reason = f"{version} foreign session/load boundary: {stop_reason}"
            break
        owner_load = run_attempt(
            context,
            evidence,
            env_summary,
            artifact,
            record,
            f"{version}-owner-session-access",
            case_root,
            "owner-session-load",
            OWNER_UID,
            home,
            cwd,
            tmpdir,
            kiro_home,
            "session/load",
            session_id=created_id,
        )
        attempts.append(owner_load)
        stop_reason = inaccessible_boundary(owner_load)
        if stop_reason:
            stop_reason = f"{version} owner session/load boundary: {stop_reason}"
            break

        if stop_reason:
            break
        for scenario, label, state_uid in (
            ("misowned-home", "misowned-home", OTHER_UID),
            ("misowned-entry", "misowned-entry", OWNER_UID),
            ("non-directory-home", "non-directory-home", OWNER_UID),
            ("missing-home", "missing-home", OWNER_UID),
            ("canonical-alias", "canonical-alias", OWNER_UID),
            ("internal-symlink", "internal-symlink", OWNER_UID),
            ("external-sentinel", "external-sentinel", OWNER_UID),
        ):
            label = f"{version}-{label}"
            case_root, home, cwd, tmpdir, kiro_home, seed_before = prep_case(
                context, version, label, scenario, state_uid, record
            )
            attempt = run_attempt(
                context,
                evidence,
                env_summary,
                artifact,
                record,
                label,
                case_root,
                scenario,
                OWNER_UID,
                home,
                cwd,
                tmpdir,
                kiro_home,
                "session/new",
                state_before=seed_before,
            )
            attempts.append(attempt)
            stop_reason = inaccessible_boundary(attempt)
            if stop_reason:
                stop_reason = f"{version} {scenario} boundary: {stop_reason}"
                break
        if stop_reason:
            break

    liveness = json.loads(
        docker_exec(
            context,
            "python3",
            SCRIPT_IN_CONTAINER,
            "--inside",
            "process-liveness",
            user=0,
            timeout=20.0,
        ).stdout
    )
    write_json_fsync(evidence / "process-liveness.json", liveness)
    if liveness.get("clear") is not True:
        raise RuntimeError(f"disposable task UIDs still own live processes: {liveness!r}")

    export = evidence / "container-evidence"
    export_container_evidence(context, export)
    summary = {
        "task_id": "21376339-8146-4426-ab46-69d920b8442d",
        "run_id": "f698dedf-08ae-472f-89cf-5a1f4b321580",
        "evidence_directory": str(evidence.resolve()),
        "container_id": env_summary["container_id"],
        "image_id": env_summary["image_id"],
        "versions": VERSIONS,
        "artifact_identity": {version: verified_artifacts[version]["verification"] for version in VERSIONS},
        "fake_first": fake,
        "attempt_count": len(attempts),
        "attempt_coverage_complete": len(attempts) == len(VERSIONS) * 10,
        "finite_stop_reason": stop_reason,
        "attempts": attempts,
        "task_process_liveness": liveness,
        "container_retained_running": True,
        "provider_prompt_sent": False,
        "credentials_supplied": False,
        "external_container_egress": False,
        "file_contents_captured": False,
    }
    write_json_fsync(evidence / "summary.json", summary)
    print(json.dumps({"evidence_directory": str(evidence.resolve()), "attempt_count": len(attempts), "summary": str((evidence / 'summary.json').resolve())}, indent=2))
    return evidence


def extract_session_id(outcome: dict[str, Any]) -> str | None:
    for call in outcome.get("rpc", []):
        response = call.get("response") or {}
        value = (response.get("result") or {}).get("sessionId")
        if isinstance(value, str) and value:
            return value
    return None


def mode_string(mode: int) -> str:
    return format(stat.S_IMODE(mode), "04o")


def snapshot_tree(root: pathlib.Path) -> list[dict[str, Any]]:
    output: list[dict[str, Any]] = []
    if not root.exists() and not root.is_symlink():
        return output
    candidates = [root]
    if root.is_symlink():
        target = root.resolve(strict=False)
        if target != root:
            candidates.append(target)
    for candidate in candidates:
        if not candidate.exists() and not candidate.is_symlink():
            continue
        info = candidate.lstat()
        kind = "symlink" if stat.S_ISLNK(info.st_mode) else "dir" if stat.S_ISDIR(info.st_mode) else "file"
        entry: dict[str, Any] = {
            "path": str(candidate),
            "kind": kind,
            "uid": info.st_uid,
            "gid": info.st_gid,
            "mode": mode_string(info.st_mode),
            "size": info.st_size,
            "mtime_ns": info.st_mtime_ns,
            "ctime_ns": info.st_ctime_ns,
        }
        if kind == "symlink":
            entry["target"] = os.readlink(candidate)
        output.append(entry)
        if kind != "dir":
            continue
        for current, dirs, files in os.walk(candidate, topdown=True, followlinks=False):
            for name in sorted(dirs + files):
                path = pathlib.Path(current) / name
                info = path.lstat()
                kind = "symlink" if stat.S_ISLNK(info.st_mode) else "dir" if stat.S_ISDIR(info.st_mode) else "file"
                entry = {
                    "path": str(path),
                    "kind": kind,
                    "uid": info.st_uid,
                    "gid": info.st_gid,
                    "mode": mode_string(info.st_mode),
                    "size": info.st_size,
                    "mtime_ns": info.st_mtime_ns,
                    "ctime_ns": info.st_ctime_ns,
                }
                if kind == "symlink":
                    entry["target"] = os.readlink(path)
                output.append(entry)
                if kind == "symlink" and name in dirs:
                    dirs.remove(name)
                if len(output) > 1000:
                    raise RuntimeError("synthetic state tree exceeded the path bound")
    return sorted(output, key=lambda item: item["path"])


def snapshot_state(kiro_home: str, case_root: str, scenario: str) -> dict[str, Any]:
    roots = [pathlib.Path(kiro_home)]
    if scenario == "external-sentinel":
        roots.append(pathlib.Path(case_root) / "outside-scratch-sentinel")
    entries: list[dict[str, Any]] = []
    seen: set[str] = set()
    for root in roots:
        for entry in snapshot_tree(root):
            if entry["path"] not in seen:
                entries.append(entry)
                seen.add(entry["path"])
    return {
        "kiro_home": kiro_home,
        "scenario": scenario,
        "entries": sorted(entries, key=lambda item: item["path"]),
        "file_contents_recorded": False,
    }


def state_delta(before: dict[str, Any], after: dict[str, Any]) -> dict[str, Any]:
    old = {item["path"]: item for item in before.get("entries", [])}
    new = {item["path"]: item for item in after.get("entries", [])}
    added = sorted(new.keys() - old.keys())
    removed = sorted(old.keys() - new.keys())
    changed = []
    for path in sorted(old.keys() & new.keys()):
        fields = ("kind", "uid", "gid", "mode", "size", "target")
        differences = {
            field: {"before": old[path].get(field), "after": new[path].get(field)}
            for field in fields
            if old[path].get(field) != new[path].get(field)
        }
        if differences:
            changed.append({"path": path, "fields": differences})
    return {
        "created_paths": added,
        "removed_paths": removed,
        "metadata_changes": changed,
        "file_contents_recorded": False,
    }


def summarize_state_trace(
    trace_paths: list[pathlib.Path], case_root: str, kiro_home: str, scenario: str
) -> dict[str, Any]:
    counts: dict[str, int] = {}
    operations: list[dict[str, Any]] = []
    total = 0
    relevant_roots = [kiro_home]
    if scenario == "canonical-alias":
        relevant_roots.append(str(pathlib.Path(case_root) / "kiro-real"))
    if scenario == "external-sentinel":
        relevant_roots.append(str(pathlib.Path(case_root) / "outside-scratch-sentinel"))
    for path in trace_paths:
        try:
            lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError:
            continue
        for line in lines:
            if not any(root in line for root in relevant_roots):
                continue
            match = re.search(r"\b(openat|newfstatat|readlinkat|getdents64|chmod|fchmodat|fchmodat2)\(", line)
            if not match:
                continue
            syscall = match.group(1)
            counts[syscall] = counts.get(syscall, 0) + 1
            total += 1
            if len(operations) >= 1000:
                continue
            quoted = re.search(r'"([^"\\]*(?:\\.[^"\\]*)*)"', line)
            dirfd_path = re.search(r"<(/scratch/[^>]+)>", line)
            trace_path = quoted.group(1) if quoted else (dirfd_path.group(1) if dirfd_path else None)
            if syscall == "chmod":
                mode_match = re.search(r'"[^"\\]*(?:\\.[^"\\]*)*",\s*(0[0-7]{3,4})', line)
            else:
                mode_match = re.search(r',\s*"[^"\\]*(?:\\.[^"\\]*)*",\s*(0[0-7]{3,4})', line)
            result_match = re.search(r"\)\s+=\s+([^\s]+)", line)
            operations.append(
                {
                    "syscall": syscall,
                    "path_argument": trace_path,
                    "mode_argument": mode_match.group(1) if mode_match else None,
                    "result": result_match.group(1) if result_match else None,
                }
            )
    return {
        "syscall_counts_under_synthetic_state": counts,
        "operations": operations,
        "operation_count": total,
        "operations_truncated": total > len(operations),
        "file_contents_recorded": False,
    }


def write_inner_json(path: pathlib.Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()
    with path.open("wb") as stream:
        stream.write(payload)
        stream.flush()
        os.fsync(stream.fileno())
    directory_fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(directory_fd)
    finally:
        os.close(directory_fd)


def make_state_tree(root: pathlib.Path, uid: int, session_id: str, scenario: str) -> None:
    sessions = root / "sessions" / "cli"
    session_dir = sessions / session_id
    for directory in (root, root / "sessions", sessions, session_dir):
        directory.mkdir(parents=True, exist_ok=True)
        os.chmod(directory, 0o777)
    write_inner_json(
        session_dir / "metadata.json",
        {"sessionId": session_id, "cwd": str(root.parent / "cwd"), "origin": "synthetic-owner-proof"},
    )
    os.chmod(session_dir / "metadata.json", 0o666)
    with (session_dir / "events.jsonl").open("w", encoding="utf-8") as stream:
        stream.write(json.dumps({"sessionId": session_id, "event": "synthetic"}) + "\n")
        stream.flush()
        os.fsync(stream.fileno())
    os.chmod(session_dir / "events.jsonl", 0o666)
    if scenario == "internal-symlink":
        target = session_dir / "internal-target"
        target.mkdir(mode=0o777)
        with (target / "state.json").open("w", encoding="utf-8") as stream:
            stream.write('{"synthetic":true}\n')
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(target / "state.json", 0o666)
        (session_dir / "internal-link").symlink_to(target)
    if scenario == "external-sentinel":
        outside = root.parent / "outside-scratch-sentinel"
        outside.mkdir(mode=0o777, exist_ok=True)
        sentinel = outside / "sentinel.json"
        with sentinel.open("w", encoding="utf-8") as stream:
            stream.write('{"synthetic_sentinel":true}\n')
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(sentinel, 0o666)
        (session_dir / "external-sentinel").symlink_to(sentinel)


def inside_seed() -> dict[str, Any]:
    spec = json.loads(sys.stdin.read())
    current_uid = os.getuid()
    if current_uid != int(spec["uid"]):
        raise RuntimeError("seed UID did not match the requested disposable UID")
    case_root = pathlib.Path(spec["case_root"])
    if not case_root.exists():
        case_root.mkdir(parents=True)
        os.chmod(case_root, 0o1777)
    else:
        case_info = case_root.stat()
        if case_info.st_uid == current_uid:
            os.chmod(case_root, 0o1777)
        elif stat.S_IMODE(case_info.st_mode) != 0o1777:
            raise RuntimeError("shared case root is not a disposable sticky directory with mode 1777")
    process_uid = int(spec["process_uid"])
    home = pathlib.Path(spec["home"])
    cwd = pathlib.Path(spec["cwd"])
    tmpdir = pathlib.Path(spec["tmpdir"])
    if current_uid == process_uid or spec.get("home_only"):
        for directory in (home, tmpdir, pathlib.Path(str(case_root / f"cache-{process_uid}")), pathlib.Path(str(case_root / f"config-{process_uid}")), pathlib.Path(str(case_root / f"state-{process_uid}"))):
            directory.mkdir(parents=True, exist_ok=True)
            os.chmod(directory, 0o700)
        cwd.mkdir(parents=True, exist_ok=True)
        os.chmod(cwd, 0o755)
    scenario = spec["scenario"]
    state_uid = int(spec["state_uid"])
    if not spec.get("home_only") and current_uid == state_uid:
        session_id = spec["session_id"]
        kiro_home = pathlib.Path(spec["kiro_home"])
        if scenario == "non-directory-home":
            kiro_home.parent.mkdir(parents=True, exist_ok=True)
            kiro_home.write_text("synthetic non-directory state\n", encoding="utf-8")
            os.chmod(kiro_home, 0o666)
        elif scenario != "missing-home":
            if scenario == "canonical-alias":
                actual = kiro_home.parent / "kiro-real"
                make_state_tree(actual, state_uid, session_id, scenario)
                if not kiro_home.exists() and not kiro_home.is_symlink():
                    kiro_home.symlink_to(actual)
            else:
                make_state_tree(kiro_home, state_uid, session_id, scenario)
    if spec.get("create_misowned_entry") and scenario == "misowned-entry":
        entry = pathlib.Path(spec["kiro_home"]) / "sessions" / "cli" / spec["session_id"] / "unowned-entry.json"
        entry.write_text('{"synthetic_owner":"other"}\n', encoding="utf-8")
        os.chmod(entry, 0o666)
    return {
        "seed_uid": current_uid,
        "scenario": scenario,
        "session_id": spec["session_id"],
        "kiro_home": spec["kiro_home"],
        "file_contents_recorded": False,
    }


def read_proc_state(pid: int) -> str | None:
    try:
        with open(f"/proc/{pid}/stat", encoding="utf-8") as stream:
            raw = stream.read()
        return raw.rsplit(")", 1)[1].split()[0]
    except (FileNotFoundError, ProcessLookupError):
        return None


def task_uid_processes() -> list[dict[str, Any]]:
    output: list[dict[str, Any]] = []
    for proc in pathlib.Path("/proc").iterdir():
        if not proc.name.isdigit():
            continue
        try:
            status = (proc / "status").read_text(encoding="utf-8", errors="replace")
            uid_match = re.search(r"(?m)^Uid:\s+(\d+)", status)
            uid = int(uid_match.group(1)) if uid_match else None
            if uid not in (OWNER_UID, OTHER_UID):
                continue
            state = read_proc_state(int(proc.name))
            if state in (None, "Z", "X"):
                continue
            comm = (proc / "comm").read_text(encoding="utf-8", errors="replace").strip()
            try:
                executable = os.readlink(proc / "exe")
            except OSError:
                executable = None
            output.append(
                {
                    "pid": int(proc.name),
                    "uid": uid,
                    "comm": comm,
                    "state": state,
                    "executable": executable,
                }
            )
        except (FileNotFoundError, ProcessLookupError, PermissionError):
            continue
    return sorted(output, key=lambda item: item["pid"])


def inside_process_liveness() -> dict[str, Any]:
    processes = task_uid_processes()
    cleanup: list[dict[str, Any]] = []
    for item in processes:
        pid = item["pid"]
        try:
            status = pathlib.Path(f"/proc/{pid}/status").read_text(encoding="utf-8", errors="replace")
            uid_match = re.search(r"(?m)^Uid:\s+(\d+)", status)
            executable = os.readlink(f"/proc/{pid}/exe")
            comm = pathlib.Path(f"/proc/{pid}/comm").read_text(encoding="utf-8", errors="replace").strip()
        except (FileNotFoundError, ProcessLookupError, PermissionError):
            cleanup.append({"pid": pid, "result": "exited-before-cleanup"})
            continue
        if not uid_match or int(uid_match.group(1)) != item["uid"] or executable != item["executable"] or comm != item["comm"]:
            cleanup.append({"pid": pid, "result": "identity-changed-before-cleanup"})
            continue
        try:
            os.kill(pid, signal.SIGTERM)
            cleanup.append({"pid": pid, "signal": "SIGTERM"})
        except ProcessLookupError:
            cleanup.append({"pid": pid, "result": "exited-before-signal"})
    if processes:
        time.sleep(0.5)
    remaining = task_uid_processes()
    for item in remaining:
        try:
            current_executable = os.readlink(f"/proc/{item['pid']}/exe")
            current_comm = pathlib.Path(f"/proc/{item['pid']}/comm").read_text(encoding="utf-8", errors="replace").strip()
            status = pathlib.Path(f"/proc/{item['pid']}/status").read_text(encoding="utf-8", errors="replace")
            uid_match = re.search(r"(?m)^Uid:\s+(\d+)", status)
            if (
                uid_match
                and int(uid_match.group(1)) == item["uid"]
                and current_executable == item["executable"]
                and current_comm == item["comm"]
            ):
                os.kill(item["pid"], signal.SIGKILL)
                cleanup.append({"pid": item["pid"], "signal": "SIGKILL"})
        except (FileNotFoundError, ProcessLookupError, PermissionError):
            pass
    if remaining:
        time.sleep(0.2)
    final = task_uid_processes()
    return {
        "clear": not final,
        "initial_task_uid_processes": processes,
        "cleanup_actions": cleanup,
        "remaining_task_uid_processes": final,
        "container_retained_running": True,
    }


def inside_fake_self_test() -> dict[str, Any]:
    evidence = pathlib.Path("/scratch/evidence")
    evidence.mkdir(mode=0o1777, exist_ok=True)
    root_mounts = pathlib.Path("/proc/mounts").read_text(encoding="utf-8")
    scratch_mount = next((line for line in root_mounts.splitlines() if " /scratch " in line), "")
    scratch_options = scratch_mount.split()[3].split(",") if len(scratch_mount.split()) >= 4 else []
    root_mount = next((line for line in root_mounts.splitlines() if " / " in line), "")
    status_text = pathlib.Path("/proc/self/status").read_text(encoding="utf-8")
    cap_eff_match = re.search(r"(?m)^CapEff:\s+([0-9a-fA-F]+)", status_text)
    cap_effective = int(cap_eff_match.group(1), 16) if cap_eff_match else None
    sentinel = pathlib.Path("/opt/proof/image-sentinel")
    sentinel_before = sha256_file(sentinel)
    try:
        with sentinel.open("ab") as stream:
            stream.write(b"probe")
        root_write_errno = 0
    except OSError as error:
        root_write_errno = error.errno
    sentinel_after = sha256_file(sentinel)

    fake_trace_prefix = pathlib.Path("/scratch/evidence/traces/fake-strace")
    fake_trace = subprocess.run(
        [
            "strace",
            "-ff",
            "-qq",
            "-e",
            "trace=openat",
            "-u",
            "proof-owner",
            "-o",
            str(fake_trace_prefix),
            "python3",
            "-c",
            "handle=open('/opt/proof/image-sentinel','rb'); handle.close()",
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        check=False,
        timeout=10.0,
    )
    fake_trace_files = sorted(fake_trace_prefix.parent.glob(fake_trace_prefix.name + ".*"))
    fake_trace_contents = [path.read_text(encoding="utf-8", errors="replace") for path in fake_trace_files]
    fake_trace_observed = any(
        "openat(" in content and "/opt/proof/image-sentinel" in content
        for content in fake_trace_contents
    )
    fake_trace_digest = hashlib.sha256()
    for trace_file in fake_trace_files:
        fake_trace_digest.update(trace_file.read_bytes())

    try:
        with socket.create_connection(("1.1.1.1", 443), timeout=2.0):
            egress_error = "connected"
    except OSError as error:
        egress_error = f"{error.__class__.__name__}:{error.errno}"

    pid_file = evidence / "fake-child.pid"
    child_code = (
        "import os,signal,subprocess,time; "
        "child=subprocess.Popen(['sleep','60']); "
        f"open({str(pid_file)!r},'w').write(str(child.pid)); "
        "signal.signal(signal.SIGTERM,lambda signum,frame:(child.wait(),os._exit(0))); "
        "exec('while True: time.sleep(1)')"
    )
    timer = subprocess.Popen(
        ["python3", "-c", child_code],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        start_new_session=True,
    )
    timeout_signal = "none"
    try:
        timer.wait(timeout=0.5)
    except subprocess.TimeoutExpired:
        timeout_signal = "SIGTERM-process-group"
        os.killpg(timer.pid, signal.SIGTERM)
        try:
            timer.wait(timeout=2.0)
        except subprocess.TimeoutExpired:
            timeout_signal = "SIGKILL-process-group"
            os.killpg(timer.pid, signal.SIGKILL)
            timer.wait(timeout=2.0)
    child_pid = int(pid_file.read_text(encoding="utf-8")) if pid_file.exists() else None
    child_state = read_proc_state(child_pid) if child_pid else None
    if child_pid and child_state not in (None, "Z", "X"):
        try:
            os.kill(child_pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        for _ in range(20):
            child_state = read_proc_state(child_pid)
            if child_state in (None, "Z", "X"):
                break
            time.sleep(0.05)
    interfaces = sorted(path.name for path in pathlib.Path("/sys/class/net").iterdir())
    result = {
        "passed": (
            platform.machine() == "aarch64"
            and os.confstr("CS_GNU_LIBC_VERSION").startswith("glibc ")
            and "tmpfs /scratch " in scratch_mount
            and "noexec" not in scratch_options
            and "ro," in root_mount
            and root_write_errno in (13, 30)
            and cap_effective == ((1 << 19) | (1 << 7) | (1 << 6))
            and sentinel_before == sentinel_after
            and egress_error != "connected"
            and interfaces == ["lo"]
            and timer.returncode == 0
            and timeout_signal == "SIGTERM-process-group"
            and fake_trace.returncode == 0
            and fake_trace_observed
            and child_pid is not None
            and child_state is None
            and not pathlib.Path("/Users/tom").exists()
        ),
        "machine": platform.machine(),
        "glibc": os.confstr("CS_GNU_LIBC_VERSION"),
        "scratch_filesystem": scratch_mount.split()[2] if scratch_mount else None,
        "scratch_mount_options": scratch_options,
        "scratch_executable": "noexec" not in scratch_options,
        "root_mount_read_only": "ro," in root_mount,
        "rootfs_write_errno": root_write_errno,
        "effective_capabilities": f"0x{cap_effective:x}" if cap_effective is not None else None,
        "image_sentinel_unchanged": sentinel_before == sentinel_after,
        "image_sentinel_sha256": sentinel_before,
        "fake_strace_exit": fake_trace.returncode,
        "fake_strace_stderr_bytes": len(fake_trace.stderr),
        "fake_strace_stderr_sha256": hashlib.sha256(fake_trace.stderr).hexdigest(),
        "fake_strace_openat_observed": fake_trace_observed,
        "fake_strace_sha256": fake_trace_digest.hexdigest(),
        "external_egress_probe": egress_error,
        "interfaces": interfaces,
        "timeout_exit": timer.returncode,
        "timeout_signal": timeout_signal,
        "fake_child_pid": child_pid,
        "fake_child_state_after_timeout": child_state,
        "host_home_mount_visible": pathlib.Path("/Users/tom").exists(),
        "file_contents_recorded": False,
    }
    write_inner_json(evidence / "fake-self-test.json", result)
    return result


def inside_environment_check() -> dict[str, Any]:
    interfaces = sorted(path.name for path in pathlib.Path("/sys/class/net").iterdir())
    routes = subprocess.run(["ip", "route"], check=False, stdout=subprocess.PIPE, text=True).stdout.strip()
    scratch_mount = next(
        (line for line in pathlib.Path("/proc/mounts").read_text(encoding="utf-8").splitlines() if " /scratch " in line),
        "",
    )
    route_entries = [line for line in routes.splitlines() if line]
    return {
        "machine": platform.machine(),
        "glibc": os.confstr("CS_GNU_LIBC_VERSION"),
        "interfaces": interfaces,
        "default_routes": route_entries,
        "route_table": routes,
        "scratch_mount": scratch_mount,
    }


def inside_snapshot(kiro_home: str, case_root: str, scenario: str) -> dict[str, Any]:
    return snapshot_state(kiro_home, case_root, scenario)


def inside_persist_plan(path_text: str, expected_sha: str) -> dict[str, Any]:
    path = pathlib.Path(path_text)
    with path.open("r+b") as stream:
        data = stream.read()
        actual = hashlib.sha256(data).hexdigest()
        if actual != expected_sha:
            raise RuntimeError(f"plan digest mismatch: {actual}")
        json.loads(data)
        stream.flush()
        os.fsync(stream.fileno())
    directory_fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(directory_fd)
    finally:
        os.close(directory_fd)
    return {"path": path_text, "sha256": actual, "fsynced": True}


def inside_verify_artifact() -> dict[str, Any]:
    expected = json.loads(sys.stdin.read())
    root = pathlib.Path("/scratch/artifacts") / expected["version"]
    checked: dict[str, Any] = {}
    for relative, identity in expected["files"].items():
        path = root / relative
        if not path.is_file():
            raise RuntimeError(f"artifact file is missing: {relative}")
        actual_size = path.stat().st_size
        actual_sha = sha256_file(path)
        if actual_size != identity["size"] or actual_sha != identity["sha256"]:
            raise RuntimeError(f"artifact file identity mismatch: {relative}")
        checked[relative] = {"sha256": actual_sha, "size": actual_size}
    build_text = (root / "kirocli/BUILD-INFO").read_text(encoding="utf-8")
    build = dict(
        line.split("=", 1)
        for line in build_text.splitlines()
        if "=" in line
    )
    if any(build.get(key) != value for key, value in expected["build"].items()):
        raise RuntimeError("BUILD-INFO does not match the frozen Research 394 identity")
    executable = root / "kirocli/bin/kiro-cli"
    if not executable.stat().st_mode & stat.S_IXUSR:
        raise RuntimeError("selected kiro-cli ACP entrypoint is not executable")
    return {
        "version": expected["version"],
        "verified": True,
        "archive_sha256": expected["archive_sha256"],
        "archive_size": expected["archive_size"],
        "file_count": len(checked),
        "build": {key: build.get(key) for key in expected["build"]},
        "selected_binary_executable": True,
        "file_contents_recorded": False,
    }


def sanitize_error(error: dict[str, Any]) -> dict[str, Any]:
    output: dict[str, Any] = {}
    if "code" in error:
        output["code"] = error["code"]
    if isinstance(error.get("message"), str):
        message = error["message"]
        for variable in ("HOME", "KIRO_HOME", "TMPDIR"):
            value = os.environ.get(variable)
            if value:
                message = message.replace(value, f"<{variable}>")
        output["message"] = message
    if isinstance(error.get("data"), dict):
        output["data_keys"] = sorted(error["data"])
    return output


def sanitize_response(method: str, message: dict[str, Any]) -> dict[str, Any]:
    output: dict[str, Any] = {"id": message.get("id")}
    if "error" in message:
        output["error"] = sanitize_error(message["error"])
    elif method == "session/load":
        result = message.get("result")
        output["result"] = {
            "keys": sorted(result) if isinstance(result, dict) else [],
            "session_id": result.get("sessionId") if isinstance(result, dict) else None,
            "config_option_count": len(result.get("configOptions", [])) if isinstance(result, dict) and isinstance(result.get("configOptions"), list) else None,
            "content_omitted": True,
        }
    else:
        output["result"] = message.get("result")
    return output


def inner_rpc(plan: dict[str, Any]) -> dict[str, Any]:
    invocation = plan["invocation"]
    env = invocation["environment"]
    trace_dir = pathlib.Path("/scratch/evidence/traces")
    trace_dir.mkdir(parents=True, exist_ok=True)
    prefix = trace_dir / plan["attempt_id"]
    binary = invocation["argv"][0]
    command = [
        "strace",
        "-ff",
        "-qq",
        "-ttt",
        "-yy",
        "-s",
        "256",
        "-e",
        "trace=openat,newfstatat,readlinkat,getdents64,chmod,fchmodat,fchmodat2",
        "-u",
        "proof-owner" if int(invocation["uid"]) == OWNER_UID else "proof-other",
        "-o",
        str(prefix),
        binary,
        "acp",
    ]
    child_env = {key: str(value) for key, value in env.items()}
    process = subprocess.Popen(
        command,
        cwd=invocation["cwd"],
        env=child_env,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        start_new_session=True,
    )
    stderr_digest = hashlib.sha256()
    stderr_size = 0
    stderr_tail = b""
    stderr_classification = {
        "authentication_related": False,
        "network_related": False,
        "permission_related": False,
        "owner_state_related": False,
        "generic_startup_error": False,
    }

    def drain_stderr() -> None:
        nonlocal stderr_size, stderr_tail
        assert process.stderr is not None
        while True:
            chunk = process.stderr.read(8192)
            if not chunk:
                return
            stderr_digest.update(chunk)
            stderr_size += len(chunk)
            sample = (stderr_tail + chunk).lower()
            stderr_classification["authentication_related"] |= any(
                marker in sample
                for marker in (b"auth", b"credential", b"login", b"sign in", b"sign-in")
            )
            stderr_classification["network_related"] |= any(
                marker in sample for marker in (b"network", b"connect", b"dns", b"econn")
            )
            stderr_classification["permission_related"] |= any(
                marker in sample for marker in (b"permission", b"denied", b"eacces")
            )
            stderr_classification["owner_state_related"] |= any(
                marker in sample for marker in (b"owner", b"sweep", b"kiro_home")
            )
            stderr_classification["generic_startup_error"] |= any(
                marker in sample for marker in (b"error", b"failed", b"panic")
            )
            stderr_tail = sample[-256:]

    drain = threading.Thread(target=drain_stderr, daemon=True)
    drain.start()
    selector = selectors.DefaultSelector()
    assert process.stdout is not None and process.stdin is not None
    selector.register(process.stdout, selectors.EVENT_READ)
    partial = b""
    calls: list[dict[str, Any]] = []
    notifications: list[dict[str, Any]] = []
    requests_to_client: list[str] = []
    responses: dict[Any, dict[str, Any]] = {}
    methods_by_id = {request["id"]: request["method"] for request in invocation["rpc_requests"]}
    malformed_lines = 0
    deadline = time.monotonic() + MAX_PROCESS_SECONDS

    def consume_complete_lines() -> None:
        nonlocal partial, malformed_lines
        while b"\n" in partial:
            raw_line, partial = partial.split(b"\n", 1)
            if not raw_line:
                continue
            try:
                message = json.loads(raw_line)
            except json.JSONDecodeError:
                malformed_lines += 1
                continue
            if "id" in message and ("result" in message or "error" in message):
                response_method = methods_by_id.get(message["id"], "unknown")
                responses[message["id"]] = sanitize_response(response_method, message)
            elif isinstance(message.get("method"), str):
                if "id" in message:
                    requests_to_client.append(message["method"])
                else:
                    params = message.get("params") or {}
                    update = params.get("update") or {}
                    notifications.append(
                        {
                            "method": message["method"],
                            "session_update": update.get("sessionUpdate") if isinstance(update, dict) else None,
                            "content_type": (update.get("content") or {}).get("type") if isinstance(update, dict) and isinstance(update.get("content"), dict) else None,
                            "content_omitted": True,
                        }
                    )

    for request in invocation["rpc_requests"]:
        request_record: dict[str, Any] = {"method": request["method"], "id": request["id"]}
        if request["method"] != "initialize" and (
            not calls
            or not calls[0].get("response")
            or "result" not in calls[0]["response"]
        ):
            request_record["response"] = None
            request_record["transport"] = "not-sent-initialize-did-not-succeed"
            calls.append(request_record)
            continue
        try:
            process.stdin.write((json.dumps(request, separators=(",", ":")) + "\n").encode())
            process.stdin.flush()
        except (BrokenPipeError, OSError):
            request_record["response"] = None
            request_record["transport"] = "process-closed-before-write"
            calls.append(request_record)
            continue
        request_deadline = min(deadline, time.monotonic() + MAX_RPC_WAIT_SECONDS)
        found = False
        while time.monotonic() < request_deadline and time.monotonic() < deadline:
            consume_complete_lines()
            if request["id"] in responses:
                request_record["response"] = responses.pop(request["id"])
                found = True
                break
            events = selector.select(max(0.0, min(0.25, request_deadline - time.monotonic())))
            if not events:
                if process.poll() is not None:
                    break
                continue
            chunk = os.read(process.stdout.fileno(), 65536)
            if not chunk:
                break
            partial += chunk
            if len(partial) > 1024 * 1024:
                raise RuntimeError("ACP output buffer exceeded the one-megabyte bound")
            consume_complete_lines()
            if request["id"] in responses:
                request_record["response"] = responses.pop(request["id"])
                found = True
                break
        if not found:
            request_record["response"] = None
            request_record["transport"] = "response-timeout-or-process-exit"
        calls.append(request_record)

    try:
        process.stdin.close()
    except OSError:
        pass
    cleanup_action = "stdin-eof"
    try:
        process.wait(timeout=2.0)
    except subprocess.TimeoutExpired:
        cleanup_action = "SIGTERM-own-process-group"
        try:
            os.killpg(process.pid, signal.SIGTERM)
        except ProcessLookupError:
            pass
        try:
            process.wait(timeout=2.0)
        except subprocess.TimeoutExpired:
            cleanup_action = "SIGKILL-own-process-group"
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            process.wait(timeout=2.0)
    drain.join(timeout=2.0)
    selector.close()
    traces = sorted(trace_dir.glob(plan["attempt_id"] + ".*"))
    state_trace = summarize_state_trace(
        traces,
        plan["synthetic_state"]["case_root"],
        env["KIRO_HOME"],
        plan["synthetic_state"]["scenario"],
    )
    pids = []
    for path in traces:
        suffix = path.name.rsplit(".", 1)[-1]
        if suffix.isdigit():
            pids.append(int(suffix))
    survivors = []
    for pid in sorted(set(pids)):
        state = read_proc_state(pid)
        if state not in (None, "Z", "X"):
            survivors.append({"pid": pid, "state": state})
            try:
                os.kill(pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
    outcome = {
        "attempt_id": plan["attempt_id"],
        "version": plan["identity"]["version"],
        "scenario": plan["synthetic_state"]["scenario"],
        "uid": invocation["uid"],
        "rpc": calls,
        "notifications": notifications,
        "server_requests_not_answered": requests_to_client,
        "malformed_stdout_line_count": malformed_lines,
        "process_exit_code": process.returncode,
        "process_timed_out": time.monotonic() >= deadline,
        "cleanup_action": cleanup_action,
        "traced_pids": sorted(set(pids)),
        "surviving_traced_processes": survivors,
        "state_syscall_trace": state_trace,
        "stderr_bytes": stderr_size,
        "stderr_sha256": stderr_digest.hexdigest(),
        "stderr_classification": stderr_classification,
        "trace_prefix": str(prefix),
        "file_contents_recorded": False,
        "content_notifications_recorded": False,
    }
    write_inner_json(pathlib.Path("/scratch/evidence/results") / f"{plan['attempt_id']}.json", outcome)
    if survivors:
        raise RuntimeError(f"task-owned Kiro processes survived cleanup: {survivors!r}")
    return outcome


def inside_dispatch(args: argparse.Namespace) -> int:
    if args.inside == "environment-check":
        print(json.dumps(inside_environment_check()))
    elif args.inside == "fake-self-test":
        print(json.dumps(inside_fake_self_test()))
    elif args.inside == "process-liveness":
        print(json.dumps(inside_process_liveness()))
    elif args.inside == "persist-plan":
        print(json.dumps(inside_persist_plan(args.path, args.sha256)))
    elif args.inside == "seed":
        print(json.dumps(inside_seed()))
    elif args.inside == "verify-artifact":
        print(json.dumps(inside_verify_artifact()))
    elif args.inside == "snapshot":
        print(json.dumps(inside_snapshot(args.kiro_home, args.case_root, args.scenario)))
    elif args.inside == "run-acp":
        with pathlib.Path(args.plan).open("rb") as stream:
            payload = stream.read()
        actual = hashlib.sha256(payload).hexdigest()
        if actual != args.sha256:
            raise RuntimeError(f"vendor plan digest mismatch: {actual}")
        plan = json.loads(payload)
        start = pathlib.Path("/scratch/evidence/plans") / f"{plan['attempt_id']}.start.json"
        if not start.is_file():
            raise RuntimeError("fsynced attempt start record is missing")
        if sha256_file(start) != args.start_sha256:
            raise RuntimeError("fsynced attempt start record digest does not match")
        start_payload = json.loads(start.read_text(encoding="utf-8"))
        if start_payload.get("plan_sha256") != actual:
            raise RuntimeError("attempt start record is not bound to this execution plan")
        outcome = inner_rpc(plan)
        print(json.dumps(outcome))
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--context", default=CONTEXT)
    parser.add_argument("--evidence-dir")
    parser.add_argument("--prepare-container", action="store_true")
    parser.add_argument("--run", action="store_true")
    parser.add_argument("--inside", choices=("environment-check", "fake-self-test", "process-liveness", "persist-plan", "seed", "snapshot", "verify-artifact", "run-acp"))
    parser.add_argument("--path", default="")
    parser.add_argument("--scenario", default="")
    parser.add_argument("--kiro-home", default="")
    parser.add_argument("--case-root", default="")
    parser.add_argument("--plan", default="")
    parser.add_argument("--sha256", default="")
    parser.add_argument("--start-sha256", default="")
    args = parser.parse_args()
    if args.inside:
        return inside_dispatch(args)
    if args.prepare_container:
        evidence = pathlib.Path(args.evidence_dir) if args.evidence_dir else pathlib.Path(
            tempfile.mkdtemp(prefix="swallowtail-kiro-owner-proof-build-")
        )
        evidence.mkdir(mode=0o700, parents=True, exist_ok=True)
        prepare_container(args.context, evidence)
        print(json.dumps({"container": CONTAINER, "context": args.context, "evidence_directory": str(evidence.resolve())}))
        return 0
    if args.run:
        run_validation(args.context, args.evidence_dir)
        return 0
    parser.error("choose --prepare-container, --run, or --inside")
    return 2


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as error:  # Report bounded diagnostics; never include captured file data.
        print(f"kiro ACP owner-state proof failed: {error}", file=sys.stderr)
        raise SystemExit(1)
