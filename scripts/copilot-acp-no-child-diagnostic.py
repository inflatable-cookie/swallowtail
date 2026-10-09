#!/usr/bin/env python3
"""Prove a credential-free, no-child Copilot initialize boundary with fakes only."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
import platform
import re
import selectors
import shutil
import signal
import subprocess
import sys
import tempfile
import time
import uuid
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
FIXTURE_DIR = ROOT / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-no-child-diagnostic"
PREPARATION_RECORD_PATH = FIXTURE_DIR / "preparation-record.json"
OFFLINE_HARNESS_PATH = ROOT / "scripts/copilot-acp-offline-proof.py"
STAGE_SOURCE_PATH = ROOT / "scripts/copilot-acp-stage-launcher.c"
FAKE_SOURCE_PATH = ROOT / "scripts/copilot-acp-no-child-fake.c"
DYLD_SUPPORT_PROFILE = Path("/System/Library/Sandbox/Profiles/dyld-support.sb")
PREPARATION_OPERATION_ID = "d67e7bb3-8314-48f3-92d1-fd73c73f1b77"
PREPARATION_AUTHORITY_ID = "4d499c96-4046-4189-b14e-39b1a572e035"
COPILOT_VERSION = "1.0.93"
COPILOT_EXECUTABLE_SHA256 = "df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1"
COPILOT_ARCHIVE_SHA256 = "98640ca0de6576807f369c533c839b5742b038f105a970bdd7cb0d7efc8a7a71"
MAX_CAPTURED_STDOUT = 16 * 1024
MAX_DIAGNOSTIC_SECONDS = 60
MAX_CLEANUP_SECONDS = 3


class BoundedOutputTimeout(TimeoutError):
    def __init__(self, output: bytes):
        super().__init__()
        self.output = output


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_offline_harness() -> Any:
    name = "swallowtail_copilot_acp_offline_harness"
    spec = importlib.util.spec_from_file_location(name, OFFLINE_HARNESS_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("the reviewed Copilot stage and diagnostic helpers are unavailable")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def current_host_identity() -> dict[str, str]:
    if platform.system() != "Darwin":
        raise RuntimeError("the no-child diagnostic requires macOS sandbox-exec")
    product_version = platform.mac_ver()[0]
    if not re.fullmatch(r"[0-9]+(?:\.[0-9]+){1,2}", product_version):
        raise RuntimeError("the current macOS product version is unavailable")
    sw_vers = Path("/usr/bin/sw_vers")
    if not sw_vers.is_file():
        raise RuntimeError("the current macOS build identity is unavailable")
    with tempfile.TemporaryDirectory(prefix="copilot-acp-host-identity.") as temporary:
        result = subprocess.run(
            [str(sw_vers), "-buildVersion"],
            cwd=temporary,
            env={"HOME": temporary, "PATH": "/usr/bin:/bin", "TMPDIR": temporary},
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            timeout=2,
            check=False,
        )
    build = result.stdout.decode("ascii", errors="ignore").strip()
    architecture = platform.machine().lower()
    if result.returncode != 0 or not re.fullmatch(r"[A-Za-z0-9.]{1,32}", build):
        raise RuntimeError("the current macOS build identity is unavailable")
    if architecture not in {"arm64", "aarch64", "x86_64"}:
        raise RuntimeError("the current macOS architecture is outside the recorded vocabulary")
    return {
        "product_version": product_version,
        "build": build,
        "architecture": architecture,
    }


def compile_native(source: Path, output: Path, temporary: Path) -> str:
    compiler = shutil.which("cc", path="/usr/bin:/bin:/usr/sbin:/sbin")
    if compiler is None:
        raise RuntimeError("the task-owned native fake compiler is unavailable")
    environment = {
        "HOME": str(temporary),
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "TMPDIR": str(temporary),
    }
    result = subprocess.run(
        [compiler, "-std=c11", "-Wall", "-Wextra", "-Werror", "-O2", str(source), "-o", str(output)],
        cwd=temporary,
        env=environment,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=30,
        check=False,
    )
    if result.returncode != 0 or not output.is_file():
        raise RuntimeError("a task-owned Copilot diagnostic native source did not compile")
    output.chmod(0o700)
    return hashlib.sha256(output.read_bytes()).hexdigest()


def compiler_identity(temporary: Path) -> dict[str, str]:
    compiler = shutil.which("cc", path="/usr/bin:/bin:/usr/sbin:/sbin")
    if compiler is None:
        raise RuntimeError("the task-owned native fake compiler is unavailable")
    result = subprocess.run(
        [compiler, "--version"],
        cwd=temporary,
        env={"HOME": str(temporary), "PATH": "/usr/bin:/bin:/usr/sbin:/sbin", "TMPDIR": str(temporary)},
        stdin=subprocess.DEVNULL,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        timeout=2,
        check=False,
    )
    first_line = result.stdout.decode("ascii", errors="ignore").splitlines()
    match = re.search(r"Apple clang version ([0-9]+(?:\.[0-9]+){1,2})", first_line[0]) if first_line else None
    if result.returncode != 0 or match is None:
        raise RuntimeError("the native compiler identity is outside the reviewed vocabulary")
    return {"family": "Apple clang", "version": match.group(1)}


def quote_path(path: Path) -> str:
    return json.dumps(str(path.resolve()))


def render_profile(
    *,
    artifact_root: Path,
    action_root: Path,
    record_root: Path,
    blocked_home: Path,
    blocked_repository: Path,
    host_home: Path,
    repository_root: Path,
    stage_launcher: Path,
    executable: Path,
) -> str:
    system_read = '(allow file-read* (subpath "/System/Library")) (allow file-read* (subpath "/usr/lib"))'
    system_map = '(allow file-map-executable (subpath "/System/Library")) (allow file-map-executable (subpath "/usr/lib"))'
    return " ".join(
        (
            "(version 1)",
            '(import "/System/Library/Sandbox/Profiles/dyld-support.sb")',
            "(deny default)",
            f"(allow file-read* (subpath {quote_path(artifact_root)}))",
            f"(allow file-read* (subpath {quote_path(action_root)}))",
            f"(allow file-write* (subpath {quote_path(action_root)}))",
            f"(allow file-map-executable (literal {quote_path(stage_launcher)}))",
            f"(allow file-map-executable (literal {quote_path(executable)}))",
            f"(allow process-exec* (literal {quote_path(stage_launcher)}))",
            f"(allow process-exec* (literal {quote_path(executable)}))",
            system_read,
            system_map,
            "(allow sysctl-read)",
            f"(deny file-read* (subpath {quote_path(host_home)}))",
            f"(deny file-write* (subpath {quote_path(host_home)}))",
            f"(deny file-read* (subpath {quote_path(repository_root)}))",
            f"(deny file-write* (subpath {quote_path(repository_root)}))",
            f"(deny file-read* (subpath {quote_path(record_root)}))",
            f"(deny file-write* (subpath {quote_path(record_root)}))",
            f"(deny file-read* (subpath {quote_path(blocked_home)}))",
            f"(deny file-write* (subpath {quote_path(blocked_home)}))",
            f"(deny file-read* (subpath {quote_path(blocked_repository)}))",
            f"(deny file-write* (subpath {quote_path(blocked_repository)}))",
            '(deny mach-lookup (global-name "com.apple.securityd"))',
            '(deny mach-lookup (global-name "com.apple.SecurityServer"))',
            "(deny mach-lookup)",
            "(deny network*)",
            "(deny process-fork)",
        )
    )


def symbolic_profile(profile: str, roles: dict[str, Path], task_root: Path) -> str:
    result = profile
    for role, path in sorted(roles.items(), key=lambda item: len(str(item[1].resolve())), reverse=True):
        result = result.replace(str(path.resolve()), f"${{{role}}}")
    private_paths = [str(path.resolve()) for path in roles.values()]
    private_paths.extend((str(task_root.resolve()), str(ROOT.resolve())))
    if any(path in result for path in private_paths):
        raise RuntimeError("the replayable no-child profile retained a private path")
    return result


def fresh_layout(root: Path) -> dict[str, Path]:
    paths = {
        "task_root": root,
        "artifact_root": root / "artifacts",
        "action_root": root / "actions" / "initialize",
        "record_root": root / "records",
        "blocked_home": root / "blocked-home",
        "blocked_repository": root / "blocked-repository",
    }
    for key in ("artifact_root", "action_root", "record_root", "blocked_home", "blocked_repository"):
        paths[key].mkdir(parents=True, exist_ok=True)
        paths[key].chmod(0o700)
    (paths["blocked_home"] / ".copilot").mkdir()
    (paths["blocked_home"] / "Library/Keychains").mkdir(parents=True)
    (paths["blocked_repository"]).mkdir(exist_ok=True)
    sentinels = (
        (paths["blocked_home"] / ".copilot/config.json", '{"sentinel":"SWALLOWTAIL_SYNTHETIC_AUTH_SENTINEL"}'),
        (paths["blocked_home"] / "Library/Keychains/fake.keychain", "SWALLOWTAIL_SYNTHETIC_KEYCHAIN_SENTINEL"),
        (paths["blocked_repository"] / "README.md", "SWALLOWTAIL_SYNTHETIC_REPOSITORY_SENTINEL"),
    )
    for path, contents in sentinels:
        path.write_text(contents, encoding="utf-8")
        path.chmod(0o600)
    return paths


def replay_layout(paths: dict[str, Path], task_root: Path) -> dict[str, str]:
    roles = {
        "artifact_root": paths["artifact_root"],
        "action_root": paths["action_root"],
        "record_root": paths["record_root"],
        "blocked_home": paths["blocked_home"],
        "blocked_repository": paths["blocked_repository"],
    }
    output: dict[str, str] = {}
    for name, path in roles.items():
        try:
            output[name] = path.resolve().relative_to(task_root.resolve()).as_posix()
        except ValueError as error:
            raise RuntimeError("a synthetic path role escaped the fresh task root") from error
    output.update(
        {
            "synthetic_home": "actions/initialize/home",
            "synthetic_copilot_home": "actions/initialize/copilot-home",
            "synthetic_xdg_config": "actions/initialize/xdg-config",
            "synthetic_xdg_cache": "actions/initialize/xdg-cache",
            "synthetic_tmp": "actions/initialize/tmp",
            "authorization_record": "records/initialize.json",
            "blocked_config_relative_path": ".copilot/config.json",
            "blocked_keychain_relative_path": "Library/Keychains/fake.keychain",
            "blocked_repository_sentinel": "README.md",
            "task_root": ".",
            "fake_stage_launcher": "artifacts/copilot-acp-stage-launcher",
            "fake_executable": "artifacts/copilot-acp-no-child-fake",
            "stage_launcher": "artifacts/copilot-acp-stage-launcher",
            "approved_original_executable": "artifacts/1.0.93/copilot",
            "host_home": "${host_home}; deny-only; absolute value omitted",
            "repository_root": "${repository_root}; deny-only; absolute value omitted",
        }
    )
    return output


def child_environment(paths: dict[str, Path]) -> dict[str, str]:
    action = paths["action_root"]
    home = action / "home"
    copilot_home = action / "copilot-home"
    xdg_config = action / "xdg-config"
    xdg_cache = action / "xdg-cache"
    temporary = action / "tmp"
    for directory in (home, copilot_home, xdg_config, xdg_cache, temporary):
        directory.mkdir(parents=True, exist_ok=True)
    return {
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "HOME": str(home),
        "COPILOT_HOME": str(copilot_home),
        "GH_COPILOT_HOME": str(copilot_home),
        "XDG_CONFIG_HOME": str(xdg_config),
        "XDG_CACHE_HOME": str(xdg_cache),
        "TMPDIR": str(temporary),
        "TERM": "dumb",
        "SWALLOWTAIL_ACTION_DIR": str(action),
        "SWALLOWTAIL_BLOCKED_HOME": str(paths["blocked_home"]),
        "SWALLOWTAIL_BLOCKED_REPOSITORY": str(paths["blocked_repository"]),
    }


def durable_exclusive_record(path: Path, value: dict[str, Any]) -> None:
    encoded = (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()
    flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
    if hasattr(os, "O_NOFOLLOW"):
        flags |= os.O_NOFOLLOW
    descriptor = os.open(path, flags, 0o600)
    try:
        offset = 0
        while offset < len(encoded):
            offset += os.write(descriptor, encoded[offset:])
        os.fsync(descriptor)
    finally:
        os.close(descriptor)
    directory_descriptor = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(directory_descriptor)
    finally:
        os.close(directory_descriptor)


def durable_replace_record(path: Path, value: dict[str, Any]) -> None:
    temporary = path.with_name(path.name + ".complete")
    encoded = (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()
    descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    try:
        offset = 0
        while offset < len(encoded):
            offset += os.write(descriptor, encoded[offset:])
        os.fsync(descriptor)
    finally:
        os.close(descriptor)
    os.replace(temporary, path)
    directory_descriptor = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(directory_descriptor)
    finally:
        os.close(directory_descriptor)


def read_stdout_bounded(process: subprocess.Popen[bytes], timeout: float) -> bytes:
    if process.stdout is None:
        raise RuntimeError("the fake diagnostic root has no stdout pipe")
    selector = selectors.DefaultSelector()
    selector.register(process.stdout, selectors.EVENT_READ)
    chunks = bytearray()
    deadline = time.monotonic() + timeout
    try:
        while True:
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise BoundedOutputTimeout(bytes(chunks))
            ready = selector.select(remaining)
            if not ready:
                raise BoundedOutputTimeout(bytes(chunks))
            chunk = os.read(process.stdout.fileno(), 4096)
            if not chunk:
                break
            chunks.extend(chunk)
            if len(chunks) > MAX_CAPTURED_STDOUT:
                raise RuntimeError("the fake diagnostic exceeded its output cap")
        try:
            process.wait(timeout=max(0.0, deadline - time.monotonic()))
        except subprocess.TimeoutExpired:
            raise BoundedOutputTimeout(bytes(chunks)) from None
    finally:
        selector.close()
    return bytes(chunks)


def read_stdout_until_marker(
    process: subprocess.Popen[bytes], marker: bytes, timeout: float
) -> bytes:
    if process.stdout is None:
        raise RuntimeError("the fake diagnostic root has no stdout pipe")
    selector = selectors.DefaultSelector()
    selector.register(process.stdout, selectors.EVENT_READ)
    chunks = bytearray()
    deadline = time.monotonic() + timeout
    try:
        while marker not in chunks:
            remaining = deadline - time.monotonic()
            if remaining <= 0 or not selector.select(remaining):
                raise BoundedOutputTimeout(bytes(chunks))
            chunk = os.read(process.stdout.fileno(), 4096)
            if not chunk:
                raise RuntimeError("the fake cancellation root exited before its ready marker")
            chunks.extend(chunk)
            if len(chunks) > MAX_CAPTURED_STDOUT:
                raise RuntimeError("the fake cancellation root exceeded its output cap")
    finally:
        selector.close()
    return bytes(chunks)


def stop_owned_root(process: subprocess.Popen[bytes], timeout: float = 0.2) -> bool:
    if process.poll() is None:
        process.terminate()
        try:
            process.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            process.kill()
            try:
                process.wait(timeout=MAX_CLEANUP_SECONDS)
            except subprocess.TimeoutExpired:
                return False
    return process.poll() is not None


def start_root(
    *,
    profile: str,
    stage_launcher: Path,
    executable: Path,
    arguments: list[str],
    paths: dict[str, Path],
    record_path: Path,
    environment: dict[str, str],
    offline: Any,
) -> tuple[subprocess.Popen[bytes], int, Any]:
    attempt = {
        "schema": "copilot-cli-acp-no-child-attempt.v1",
        "operation_id": PREPARATION_OPERATION_ID,
        "authority_id": PREPARATION_AUTHORITY_ID,
        "use": "task-owned fake only",
        "execution_state": "launch-may-have-occurred",
        "record_barrier": "exclusive-create; fsync-file; fsync-directory-before-start",
        "one_shot": True,
    }
    durable_exclusive_record(record_path, attempt)
    read_descriptor, write_descriptor = os.pipe()
    try:
        process = subprocess.Popen(
            offline.staged_fake_command(
                profile,
                stage_launcher,
                executable,
                write_descriptor,
                arguments,
            ),
            cwd=paths["action_root"],
            env=environment,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            bufsize=0,
            start_new_session=True,
            pass_fds=(write_descriptor,),
        )
    except BaseException:
        os.close(read_descriptor)
        os.close(write_descriptor)
        raise
    os.close(write_descriptor)
    stderr_collector = offline.BoundedStderrCollector(process.stderr)
    return process, read_descriptor, stderr_collector


def run_fake(
    *,
    profile: str,
    stage_launcher: Path,
    executable: Path,
    arguments: list[str],
    paths: dict[str, Path],
    record_name: str,
    environment: dict[str, str],
    offline: Any,
    initialize: bool = False,
    timeout: float = 8.0,
    crash_after_record: bool = False,
    cancel_after_start: bool = False,
) -> dict[str, Any]:
    record_path = paths["record_root"] / record_name
    if crash_after_record:
        durable_exclusive_record(
            record_path,
            {
                "schema": "copilot-cli-acp-no-child-attempt.v1",
                "operation_id": PREPARATION_OPERATION_ID,
                "authority_id": PREPARATION_AUTHORITY_ID,
                "use": "task-owned fake crash control",
                "execution_state": "launch-may-have-occurred",
                "record_barrier": "exclusive-create; fsync-file; fsync-directory-before-start",
                "one_shot": True,
            },
        )
        try:
            durable_exclusive_record(record_path, {"replay": "must-fail"})
        except FileExistsError:
            return {"crash_record_replay_refused": True, "root_started": False}
        raise RuntimeError("one-shot fake record unexpectedly allowed a replay")

    process, stage_descriptor, stderr_collector = start_root(
        profile=profile,
        stage_launcher=stage_launcher,
        executable=executable,
        arguments=arguments,
        paths=paths,
        record_path=record_path,
        environment=environment,
        offline=offline,
    )
    started_at = time.monotonic()
    stage = offline.read_stage_channel(stage_descriptor, timeout=min(3.0, timeout))
    stdout = b""
    timed_out = False
    launch_exception = False
    try:
        if initialize:
            request = (
                b'{"jsonrpc":"2.0","id":1,"method":"initialize",'
                b'"params":{"protocolVersion":1,"clientCapabilities":{}}}\n'
            )
            if process.stdin is None:
                raise RuntimeError("the fake diagnostic root has no stdin pipe")
            process.stdin.write(request)
            process.stdin.flush()
            process.stdin.close()
        try:
            remaining = max(0.0, timeout - (time.monotonic() - started_at))
            if cancel_after_start:
                prefix = read_stdout_until_marker(process, b"FAKE_CANCEL_READY\n", remaining)
                process.send_signal(signal.SIGTERM)
                remaining = max(0.0, timeout - (time.monotonic() - started_at))
                stdout = prefix + read_stdout_bounded(process, remaining)
            else:
                stdout = read_stdout_bounded(process, remaining)
        except BoundedOutputTimeout as error:
            timed_out = True
            stdout = error.output
            if not stop_owned_root(process):
                raise RuntimeError("the owned fake root did not stop within its cleanup bound")
            if process.stdout is not None:
                remaining = process.stdout.read(MAX_CAPTURED_STDOUT + 1)
                stdout += remaining[: max(0, MAX_CAPTURED_STDOUT - len(stdout))]
                if len(remaining) + len(error.output) > MAX_CAPTURED_STDOUT:
                    raise RuntimeError("the stopped fake diagnostic exceeded its output cap")
    except OSError:
        launch_exception = True
        stop_owned_root(process)
    finally:
        if process.poll() is None and not stop_owned_root(process):
            raise RuntimeError("the owned fake root did not join")
        stderr_joined = stderr_collector.join(MAX_CLEANUP_SECONDS)
        stderr = stderr_collector.summary(reader_joined=stderr_joined)
        if not stderr_joined or stderr["reader_error"]:
            raise RuntimeError("the bounded fake stderr reader did not join cleanly")

    exit_code = process.returncode
    fake_started = b"FAKE_ROOT_PID:" in stdout or b"FAKE_ROOT_STARTED\n" in stdout
    root_pid_stable = False
    for line in stdout.decode("utf-8", errors="replace").splitlines():
        if line.startswith("FAKE_ROOT_PID:"):
            try:
                root_pid_stable = int(line.partition(":")[2]) == process.pid
            except ValueError:
                root_pid_stable = False
            break
    if arguments == ["--exec-replacement"]:
        replacement_pid_path = paths["action_root"] / "exec-replacement-pid"
        try:
            root_pid_stable = int(replacement_pid_path.read_text()) == process.pid
        except (OSError, ValueError):
            root_pid_stable = False
    summary = offline.stage_summary_for_launch(
        stage,
        process_exit_observed=exit_code is not None,
        fake_initialization_confirmed=initialize and b'"id":1,"result"' in stdout,
        confirmed_stage="acp-initialize",
    )
    sanitized = {
        "stage": summary["stage"],
        "operation": summary["operation"],
        "errno": summary["errno"],
        "exec_boundary_eof": summary["exec_boundary_eof"],
        "fake_start_marker_observed": fake_started,
        "root_pid_stable": root_pid_stable,
        "initialize_response_observed": initialize and b'"id":1,"result"' in stdout,
        "timed_out": timed_out,
        "cancelled": b"FAKE_CANCELLED\n" in stdout,
        "launch_exception": launch_exception,
        "exit_observed": exit_code is not None,
        "exit_success": exit_code == 0,
        "stderr_category": stderr["classification"],
        "stderr_marker_facets": stderr["marker_facets"],
        "stderr_captured_bytes": stderr["captured_bytes"],
        "stderr_total_bytes": stderr["total_bytes"],
        "stderr_total_bytes_capped": stderr["total_bytes_capped"],
        "stderr_raw_persisted": False,
        "stderr_raw_displayed": False,
        "stderr_hash_persisted": False,
        "stderr_paths_persisted": False,
        "stderr_secrets_persisted": False,
        "root_joined": process.poll() is not None,
        "stderr_reader_joined": stderr_joined,
        "stage_channel_joined": stage["channel_joined"],
        "elapsed_milliseconds": min(60000, int((time.monotonic() - started_at) * 1000)),
    }
    durable_replace_record(
        record_path,
        {
            "schema": "copilot-cli-acp-no-child-attempt.v1",
            "operation_id": PREPARATION_OPERATION_ID,
            "authority_id": PREPARATION_AUTHORITY_ID,
            "use": "task-owned fake only",
            "execution_state": "completed",
            "one_shot": True,
            "result": sanitized,
        },
    )
    return {"summary": sanitized, "stdout": stdout}


def parse_fixed_lines(output: bytes) -> set[str]:
    lines = output.decode("utf-8", errors="strict").splitlines()
    if any(len(line) > 1024 for line in lines):
        raise RuntimeError("the fake diagnostic emitted an overlong line")
    return {line for line in lines if line.startswith(("POLICY:", "CHILD_CASE:"))}


def ensure_no_child_effects(action: Path, case_names: list[str]) -> bool:
    return all(not (action / case_name).exists() for case_name in case_names)


def run_self_test() -> dict[str, Any]:
    if not Path("/usr/bin/sandbox-exec").is_file():
        raise RuntimeError("macOS sandbox-exec is unavailable; a stronger isolated environment is required")
    host_identity = current_host_identity()
    offline = load_offline_harness()
    with tempfile.TemporaryDirectory(prefix="copilot-acp-no-child.") as temporary:
        task_root = Path(temporary).resolve()
        paths = fresh_layout(task_root)
        stage_launcher = paths["artifact_root"] / "copilot-acp-stage-launcher"
        fake = paths["artifact_root"] / "copilot-acp-no-child-fake"
        stage_identity = offline.compile_stage_launcher(stage_launcher)
        fake_digest = compile_native(FAKE_SOURCE_PATH, fake, paths["artifact_root"])
        host_home = Path.home().resolve()
        repository_root = ROOT.resolve()
        profile = render_profile(
            artifact_root=paths["artifact_root"],
            action_root=paths["action_root"],
            record_root=paths["record_root"],
            blocked_home=paths["blocked_home"],
            blocked_repository=paths["blocked_repository"],
            host_home=host_home,
            repository_root=repository_root,
            stage_launcher=stage_launcher,
            executable=fake,
        )
        profile_roles = {
            "artifact_root": paths["artifact_root"],
            "action_root": paths["action_root"],
            "record_root": paths["record_root"],
            "blocked_home": paths["blocked_home"],
            "blocked_repository": paths["blocked_repository"],
            "host_home": host_home,
            "repository_root": repository_root,
            "fake_stage_launcher": stage_launcher,
            "fake_executable": fake,
        }
        template = symbolic_profile(profile, profile_roles, task_root)
        if "(deny process-fork)" not in template or "(allow process-fork)" in template:
            raise RuntimeError("the rendered fake profile does not end with a closed no-child rule")
        profile_digest = hashlib.sha256(template.encode()).hexdigest()
        environment = child_environment(paths)
        environment["SWALLOWTAIL_DIAGNOSTIC_SHIM"] = str(stage_launcher)
        attempt = run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=fake,
            arguments=["--acp", "--stdio"],
            paths=paths,
            record_name="initialize.json",
            environment=environment,
            offline=offline,
            initialize=True,
            timeout=MAX_DIAGNOSTIC_SECONDS,
        )
        summary = attempt["summary"]
        fixed_lines = parse_fixed_lines(attempt["stdout"])
        expected_policy = {
            "POLICY:file-denial:passed",
            "POLICY:network-denial:passed",
            "POLICY:auth-service-denial:passed",
            "POLICY:child-creation-denial:passed",
        }
        expected_children = {
            "CHILD_CASE:fork-self-exec:denied",
            "CHILD_CASE:fork-session-escape:denied",
            "CHILD_CASE:fork-system-helper:denied",
            "CHILD_CASE:vfork-self-exec:denied",
            "CHILD_CASE:vfork-system-helper:denied",
            "CHILD_CASE:spawn-self-exec:denied",
            "CHILD_CASE:spawn-system-helper:denied",
            "CHILD_CASE:spawn-stage-launcher:denied",
        }
        marker_names = sorted(
            {
                "fork-self-exec",
                "fork-session-escape",
                "fork-system-helper",
                "vfork-self-exec",
                "vfork-system-helper",
                "spawn-self-exec",
                "spawn-system-helper",
                "atexit-child-effect",
            }
        )
        missing_policy = sorted(expected_policy - fixed_lines)
        missing_children = sorted(expected_children - fixed_lines)
        observed_policy = sorted(
            line for line in fixed_lines if line.startswith("POLICY:")
        )
        observed_children = sorted(
            line
            for line in fixed_lines
            if line.startswith("CHILD_CASE:") and line not in expected_children
        )
        missing_effects = sorted(
            name for name in marker_names if (paths["action_root"] / name).exists()
        )
        failed_startup = [
            gate
            for gate, passed in (
                ("stage-not-initialize", summary["stage"] == "acp-initialize"),
                ("initialize-response-missing", summary["initialize_response_observed"]),
                ("fake-exit-failed", summary["exit_success"]),
                ("owned-root-not-joined", summary["root_joined"]),
                ("stderr-reader-not-joined", summary["stderr_reader_joined"]),
                ("stage-channel-not-joined", summary["stage_channel_joined"]),
                ("root-pid-changed", summary["root_pid_stable"]),
                ("at-exit-spawn-not-denied", (paths["action_root"] / "atexit-result").exists() and (paths["action_root"] / "atexit-result").read_text() == "D"),
                ("system-helper-effect", not (paths["action_root"] / "system-helper-effect").exists()),
            )
            if not passed
        ]
        if missing_policy:
            observed = ",".join(observed_policy) if observed_policy else "none"
            raise RuntimeError(
                "sandbox policy counterexample: missing=" + ",".join(missing_policy)
                + "; observed=" + observed
            )
        if missing_children:
            observed = ",".join(observed_children) if observed_children else "none"
            raise RuntimeError(
                "no-child syscall counterexample: missing=" + ",".join(missing_children)
                + "; observed=" + observed
            )
        if missing_effects:
            raise RuntimeError("child effect occurred before denied boundary: " + ",".join(missing_effects))
        if failed_startup:
            raise RuntimeError(
                "fake initialize gate failed: " + ",".join(failed_startup)
                + f"; stage={summary['stage']}; errno={summary['errno']}"
            )
        replacement = run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=fake,
            arguments=["--exec-replacement"],
            paths=paths,
            record_name="exec-replacement.json",
            environment=environment,
            offline=offline,
        )
        replacement_pid_path = paths["action_root"] / "exec-replacement-pid"
        replacement_pid = int(replacement_pid_path.read_text()) if replacement_pid_path.exists() else -1
        if not (
            replacement["summary"]["exit_success"]
            and replacement["summary"]["root_pid_stable"]
            and replacement_pid > 0
            and b"EXEC_REPLACEMENT:same-root\n" in replacement["stdout"]
        ):
            raise RuntimeError("a self-exec replacement did not remain in one owned root")

        helper_replacement = run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=fake,
            arguments=["--system-helper-replacement"],
            paths=paths,
            record_name="system-helper-replacement.json",
            environment=environment,
            offline=offline,
        )
        if not (
            helper_replacement["summary"]["exit_success"]
            and not (paths["action_root"] / "system-helper-effect").exists()
        ):
            raise RuntimeError("the exact target exec grant allowed an unlisted system helper")

        cancelled = run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=fake,
            arguments=["--cancel-ready"],
            paths=paths,
            record_name="cancel-stop-join.json",
            environment=environment,
            offline=offline,
            timeout=5.0,
            cancel_after_start=True,
        )
        if not (
            cancelled["summary"]["cancelled"]
            and not cancelled["summary"]["timed_out"]
            and cancelled["summary"]["exit_success"]
            and cancelled["summary"]["root_joined"]
            and cancelled["summary"]["stderr_reader_joined"]
            and cancelled["summary"]["stage_channel_joined"]
        ):
            raise RuntimeError("the cancellation, direct-root stop, or join fake did not pass")

        sensitive_diagnostic = run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=fake,
            arguments=["--diagnostic-secret"],
            paths=paths,
            record_name="sanitized-diagnostic.json",
            environment=environment,
            offline=offline,
        )
        sensitive_record = (paths["record_root"] / "sanitized-diagnostic.json").read_text()
        sensitive_sentinel = (
            "SWALLOWTAIL_DIAGNOSTIC_SECRET_SENTINEL",
            "SWALLOWTAIL_RAW_HASH_SENTINEL",
            str(paths["action_root"]),
            str(Path.home()),
        )
        if (
            sensitive_diagnostic["summary"]["stderr_total_bytes"] == 0
            or sensitive_diagnostic["summary"]["stderr_category"] != "unknown"
            or any(item in sensitive_record for item in sensitive_sentinel)
            or sensitive_diagnostic["summary"]["stderr_raw_persisted"]
            or sensitive_diagnostic["summary"]["stderr_hash_persisted"]
            or sensitive_diagnostic["summary"]["stderr_paths_persisted"]
            or sensitive_diagnostic["summary"]["stderr_secrets_persisted"]
        ):
            raise RuntimeError("the fake diagnostic record retained raw text, a hash, path, or secret")

        early_exit = run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=fake,
            arguments=["--fail-before-start"],
            paths=paths,
            record_name="early-exit.json",
            environment=environment,
            offline=offline,
        )
        if early_exit["summary"]["stage"] != "unknown" or early_exit["summary"]["fake_start_marker_observed"]:
            raise RuntimeError("stage-channel EOF without the fake startup marker was not unknown")

        timed = run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=fake,
            arguments=["--hang-after-start"],
            paths=paths,
            record_name="timeout-stop-join.json",
            environment=environment,
            offline=offline,
            timeout=0.2,
        )
        if not (
            timed["summary"]["timed_out"]
            and timed["summary"]["fake_start_marker_observed"]
            and timed["summary"]["root_joined"]
            and timed["summary"]["stderr_reader_joined"]
            and timed["summary"]["stage_channel_joined"]
        ):
            raise RuntimeError("timeout, stop, or owned-root join did not pass its fake control")

        crash = run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=fake,
            arguments=["--acp", "--stdio"],
            paths=paths,
            record_name="crash-before-start.json",
            environment=environment,
            offline=offline,
            crash_after_record=True,
        )
        if crash != {"crash_record_replay_refused": True, "root_started": False}:
            raise RuntimeError("the fsynced one-shot record did not fail closed after a crash")

        closure = offline.dyld_support_profile_closure()
        if not closure["files"]:
            raise RuntimeError("the imported macOS runtime profile closure is empty")
        candidate_profile = template.replace(
            "${fake_executable}", "${approved_original_executable}"
        ).replace("${fake_stage_launcher}", "${stage_launcher}")
        return {
            "status": "passed",
            "operation_id": PREPARATION_OPERATION_ID,
            "authority_id": PREPARATION_AUTHORITY_ID,
            "host_identity": host_identity,
            "runtime_profile_closure": closure,
            "synthetic_inputs": {
                "directory_mode_octal": "0700",
                "sentinel_file_mode_octal": "0600",
                "sentinels": {
                    ".copilot/config.json": '{"sentinel":"SWALLOWTAIL_SYNTHETIC_AUTH_SENTINEL"}',
                    "Library/Keychains/fake.keychain": "SWALLOWTAIL_SYNTHETIC_KEYCHAIN_SENTINEL",
                    "blocked_repository/README.md": "SWALLOWTAIL_SYNTHETIC_REPOSITORY_SENTINEL",
                },
                "allowed_synthetic_state_directories_start_empty": True,
            },
            "profile_template": template,
            "profile_template_sha256": profile_digest,
            "path_roles": replay_layout(paths, task_root),
            "profile_role_bindings": {
                "host_home": "current Path.home(); deny-only; absolute value omitted",
                "repository_root": "invocation checkout root; deny-only; absolute value omitted",
                "record_root": "records",
                "artifact_root": "artifacts",
                "action_root": "actions/initialize",
                "blocked_home": "blocked-home",
                "blocked_repository": "blocked-repository",
                "fake_stage_launcher": "artifacts/copilot-acp-stage-launcher",
                "fake_executable": "artifacts/copilot-acp-no-child-fake",
                "stage_launcher": "artifacts/copilot-acp-stage-launcher",
                "approved_original_executable": "artifacts/1.0.93/copilot",
            },
            "identities": {
                "harness_sha256": sha256(Path(__file__).resolve()),
                "offline_stage_and_diagnostic_helpers_sha256": sha256(OFFLINE_HARNESS_PATH),
                "fake_source_sha256": sha256(FAKE_SOURCE_PATH),
                "fake_binary_sha256": fake_digest,
                "stage_launcher_source_sha256": stage_identity["source_sha256"],
                "stage_launcher_binary_sha256": stage_identity["binary_sha256"],
                "compiler": compiler_identity(paths["artifact_root"]),
                "compiler_flags": ["-std=c11", "-Wall", "-Wextra", "-Werror", "-O2"],
                "frozen_original_target": {
                    "route": "copilot-cli.acp",
                    "version": COPILOT_VERSION,
                    "platform": "darwin-arm64",
                    "archive_sha256": COPILOT_ARCHIVE_SHA256,
                    "executable_sha256": COPILOT_EXECUTABLE_SHA256,
                    "staged_or_executed": False,
                },
            },
            "fake_claims": {
                "single_root_pid_through_stage_exec": True,
                "fake_initialize_observed_once": summary["initialize_response_observed"],
                "session_new_sent": False,
                "session_prompt_sent": False,
                "fork_vfork_posix_spawn_denied_before_child_effect": True,
                "self_exec_and_shim_exec_grants_do_not_allow_spawn": True,
                "unlisted_system_helper_exec_denied": True,
                "session_escape_child_denied": True,
                "child_at_exit_race_denied": True,
                "synthetic_config_keychain_and_repository_reads_writes_denied": True,
                "security_auth_mach_lookups_denied": True,
                "external_and_inbound_network_denied": True,
                "timeout_stop_direct_pid_join_and_reader_join": True,
                "eof_without_fake_startup_remains_unknown": True,
                "one_shot_fsync_and_crash_replay_refusal": True,
                "sanitized_bounded_diagnostics_only": True,
            },
            "diagnostic_request": {
                "argv": ["--acp", "--stdio"],
                "profile_template": candidate_profile,
                "profile_template_sha256": hashlib.sha256(candidate_profile.encode()).hexdigest(),
                "stage_launcher_source": "scripts/copilot-acp-stage-launcher.c",
                "stage_launcher_binary_sha256": stage_identity["binary_sha256"],
                "stage_launcher_transition": "sandboxed root execs stage launcher, then stage launcher execs exact target in the same pid",
                "executable_relative_role": "artifacts/1.0.93/copilot",
                "working_directory": "${action_root}",
                "environment": {
                    "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
                    "HOME": "${synthetic_home}",
                    "COPILOT_HOME": "${synthetic_copilot_home}",
                    "GH_COPILOT_HOME": "${synthetic_copilot_home}",
                    "XDG_CONFIG_HOME": "${synthetic_xdg_config}",
                    "XDG_CACHE_HOME": "${synthetic_xdg_cache}",
                    "TMPDIR": "${synthetic_tmp}",
                    "TERM": "dumb",
                    "GITHUB_TOKEN": "absent",
                    "GH_TOKEN": "absent",
                    "COPILOT_GITHUB_TOKEN": "absent",
                    "COPILOT_MODEL": "absent",
                    "COPILOT_AUTO_UPDATE": "absent",
                    "HTTP_PROXY": "absent",
                    "HTTPS_PROXY": "absent",
                    "ALL_PROXY": "absent",
                    "http_proxy": "absent",
                    "https_proxy": "absent",
                    "all_proxy": "absent",
                },
                "maximum_seconds": MAX_DIAGNOSTIC_SECONDS,
                "cleanup_seconds": MAX_CLEANUP_SECONDS,
                "maximum_initialize_requests": 1,
                "session_new_requests": 0,
                "session_prompt_requests": 0,
                "model_auto": False,
                "synthetic_home_copilot_xdg_cache_and_tmp_only": True,
                "real_home_config_keychain_auth_and_repository_denied": True,
                "external_network_denied": True,
                "execution_authorized": False,
                "original_launch_included": False,
            },
            "fake_controls": {
                "initialize": {
                    "outcome": "observed",
                    "request_count": 1,
                    "session_new_count": 0,
                    "session_prompt_count": 0,
                    "root_pid_stable_through_stage_and_target_exec": summary["root_pid_stable"],
                    "sanitized_diagnostic": summary,
                },
                "child_creation_attempts": [
                    {"case": name.partition(":")[2].partition(":")[0], "denied": name in fixed_lines}
                    for name in sorted(expected_children)
                ],
                "child_effect_markers": {
                    name: (paths["action_root"] / name).exists() for name in marker_names
                },
                "at_exit_spawn": {
                    "result": "denied" if (paths["action_root"] / "atexit-result").read_text() == "D" else "unexpected-child",
                    "effect_marker_present": (paths["action_root"] / "atexit-child-effect").exists(),
                },
                "exec_replacement": {
                    "self_exec_same_root_pid": replacement["summary"]["root_pid_stable"],
                    "system_helper_denied": helper_replacement["summary"]["exit_success"]
                    and not (paths["action_root"] / "system-helper-effect").exists(),
                },
                "cancellation": {
                    "cancel_signal_sent": True,
                    "root_exited_on_cancel": cancelled["summary"]["cancelled"],
                    "root_joined": cancelled["summary"]["root_joined"],
                    "stderr_reader_joined": cancelled["summary"]["stderr_reader_joined"],
                },
                "diagnostics": {
                    "sanitized_secret_control": sensitive_diagnostic["summary"],
                    "raw_text_hash_path_and_secret_absent_from_record": True,
                    "capture_limit_bytes": offline.MAX_STDERR_CAPTURE_BYTES,
                    "count_limit_bytes": offline.MAX_STDERR_COUNTED_BYTES,
                },
                "policy_controls": {
                    "synthetic_home_config_keychain_and_repository_denied": "POLICY:file-denial:passed" in fixed_lines,
                    "fake_reserved_network_destination_denied": "203.0.113.1:443",
                    "network_bind_denied": "POLICY:network-denial:passed" in fixed_lines,
                    "securityd_and_securityserver_lookup_denied": "POLICY:auth-service-denial:passed" in fixed_lines,
                },
                "early_exit": {
                    "stage": early_exit["summary"]["stage"],
                    "fake_start_marker_observed": early_exit["summary"]["fake_start_marker_observed"],
                    "stage_channel_joined": early_exit["summary"]["stage_channel_joined"],
                },
                "timeout_stop_join": timed["summary"],
                "crash_after_fsync_replay": crash,
            },
            "limitation": "Fake success proves only the recorded macOS profile boundary and fake ACP behavior. It proves no host-login, Auto-model, auth compatibility, provider access, or Copilot startup behavior. A later original attempt requires a new exact authority and independent review.",
        }


def validate_record(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict) or value.get("schema") != "copilot-cli-acp-no-child-preparation.v1":
        raise ValueError("the no-child preparation record schema is unknown")
    if (
        value.get("operation_id") != PREPARATION_OPERATION_ID
        or value.get("authority_id") != PREPARATION_AUTHORITY_ID
    ):
        raise ValueError("the preparation record is not bound to this fake-only ruling")
    proof = value.get("fake_proof")
    if not isinstance(proof, dict) or proof.get("status") != "passed":
        raise ValueError("the committed record does not contain a successful fake-only proof")
    required = {
        "fork_vfork_posix_spawn_denied_before_child_effect",
        "self_exec_and_shim_exec_grants_do_not_allow_spawn",
        "session_escape_child_denied",
        "child_at_exit_race_denied",
        "timeout_stop_direct_pid_join_and_reader_join",
        "eof_without_fake_startup_remains_unknown",
        "one_shot_fsync_and_crash_replay_refusal",
        "sanitized_bounded_diagnostics_only",
    }
    claims = proof.get("fake_claims", {})
    if not isinstance(claims, dict) or any(claims.get(claim) is not True for claim in required):
        raise ValueError("the committed fake proof is missing a required no-child gate")
    controls = value.get("fake_controls", {})
    attempts = controls.get("child_creation_attempts", []) if isinstance(controls, dict) else []
    if (
        not isinstance(attempts, list)
        or len(attempts) != 8
        or any(not isinstance(item, dict) or item.get("denied") is not True for item in attempts)
    ):
        raise ValueError("the committed record does not retain every independent child API result")
    markers = controls.get("child_effect_markers", {}) if isinstance(controls, dict) else {}
    if not isinstance(markers, dict) or any(markers.values()):
        raise ValueError("the committed fake run observed a child effect marker")
    cancellation = controls.get("cancellation", {})
    diagnostics = controls.get("diagnostics", {})
    if (
        cancellation.get("cancel_signal_sent") is not True
        or cancellation.get("root_exited_on_cancel") is not True
        or cancellation.get("root_joined") is not True
        or diagnostics.get("raw_text_hash_path_and_secret_absent_from_record") is not True
    ):
        raise ValueError("the committed record lacks cancellation or safe-diagnostic proof")
    if value.get("diagnostic_request", {}).get("execution_authorized") is not False:
        raise ValueError("the preparation record must not authorize an original launch")
    template = value.get("profile_template")
    template_digest = value.get("profile_template_sha256")
    if not isinstance(template, str) or hashlib.sha256(template.encode()).hexdigest() != template_digest:
        raise ValueError("the replayable profile template digest is invalid")
    if (
        "(deny process-fork)" not in template
        or "(allow process-fork)" in template
        or "(allow process*" in template
        or "(deny network*)" not in template
        or "(deny mach-lookup)" not in template
    ):
        raise ValueError("the profile template does not preserve the closed no-child rule")
    if claims.get("single_root_pid_through_stage_exec") is not True:
        raise ValueError("the fake root PID was not bound across its launcher transition")
    candidate_profile = value.get("diagnostic_request", {}).get("profile_template")
    candidate_digest = value.get("diagnostic_request", {}).get("profile_template_sha256")
    if (
        not isinstance(candidate_profile, str)
        or hashlib.sha256(candidate_profile.encode()).hexdigest() != candidate_digest
        or "${approved_original_executable}" not in candidate_profile
        or "${stage_launcher}" not in candidate_profile
        or "(deny process-fork)" not in candidate_profile
    ):
        raise ValueError("the data-only original diagnostic profile binding is invalid")
    host = value.get("host_identity", {})
    if not isinstance(host, dict) or host != current_host_identity():
        raise ValueError("the preparation record belongs to a different macOS build or architecture")
    offline = load_offline_harness()
    if value.get("runtime_profile_closure") != offline.dyld_support_profile_closure():
        raise ValueError("the imported macOS runtime profile closure has changed")
    synthetic = value.get("synthetic_inputs", {})
    if (
        synthetic.get("directory_mode_octal") != "0700"
        or synthetic.get("sentinel_file_mode_octal") != "0600"
        or synthetic.get("allowed_synthetic_state_directories_start_empty") is not True
        or set(synthetic.get("sentinels", {}))
        != {
            ".copilot/config.json",
            "Library/Keychains/fake.keychain",
            "blocked_repository/README.md",
        }
    ):
        raise ValueError("the replayable synthetic filesystem inputs are incomplete")
    raw = json.dumps(value, sort_keys=True)
    if any(
        sentinel in raw
        for sentinel in (
            str(ROOT),
            str(Path.home()),
            "SWALLOWTAIL_FAKE_AUTH",
            "SWALLOWTAIL_DIAGNOSTIC_SECRET_SENTINEL",
            "SWALLOWTAIL_RAW_HASH_SENTINEL",
        )
    ):
        raise ValueError("the preparation record retained a private path or fake credential")
    frozen_target = value.get("identities", {}).get("frozen_original_target", {})
    if (
        frozen_target.get("route") != "copilot-cli.acp"
        or frozen_target.get("executable_sha256") != COPILOT_EXECUTABLE_SHA256
        or frozen_target.get("archive_sha256") != COPILOT_ARCHIVE_SHA256
        or frozen_target.get("platform") != "darwin-arm64"
        or frozen_target.get("version") != COPILOT_VERSION
        or frozen_target.get("staged_or_executed") is not False
    ):
        raise ValueError("the preparation record changed the frozen target or execution boundary")
    required_role_bindings = {
        "host_home",
        "repository_root",
        "record_root",
        "artifact_root",
        "action_root",
        "blocked_home",
        "blocked_repository",
        "fake_stage_launcher",
        "fake_executable",
        "stage_launcher",
        "approved_original_executable",
    }
    role_bindings = value.get("profile_role_bindings", {})
    path_roles = value.get("path_roles", {})
    if not isinstance(role_bindings, dict) or set(role_bindings) != required_role_bindings:
        raise ValueError("the profile role bindings are incomplete")
    if not isinstance(path_roles, dict) or not required_role_bindings.issubset(path_roles):
        raise ValueError("the replayable layout omits a profile path role")
    identities = value.get("identities", {})
    if (
        identities.get("harness_sha256") != sha256(Path(__file__).resolve())
        or identities.get("offline_stage_and_diagnostic_helpers_sha256") != sha256(OFFLINE_HARNESS_PATH)
        or identities.get("fake_source_sha256") != sha256(FAKE_SOURCE_PATH)
        or identities.get("stage_launcher_source_sha256") != sha256(STAGE_SOURCE_PATH)
    ):
        raise ValueError("the preparation record does not bind the current reviewed source identities")
    request = value.get("diagnostic_request", {})
    if (
        request.get("argv") != ["--acp", "--stdio"]
        or request.get("maximum_seconds") != MAX_DIAGNOSTIC_SECONDS
        or request.get("maximum_initialize_requests") != 1
        or request.get("session_new_requests") != 0
        or request.get("session_prompt_requests") != 0
        or request.get("model_auto") is not False
        or request.get("original_launch_included") is not False
        or request.get("working_directory") != "${action_root}"
    ):
        raise ValueError("the data-only request exceeds its initialize-only boundary")
    return {
        "status": "passed",
        "schema": value["schema"],
        "operation_id": value["operation_id"],
        "authority_id": value["authority_id"],
        "fake_only": True,
        "execution_authorized": False,
        "profile_template_sha256": template_digest,
    }


def compile_fake_only() -> dict[str, str]:
    with tempfile.TemporaryDirectory(prefix="copilot-acp-fake-compile.") as temporary:
        root = Path(temporary).resolve()
        fake = root / "copilot-acp-no-child-fake"
        digest = compile_native(FAKE_SOURCE_PATH, fake, root)
    return {"status": "compiled", "fake_binary_sha256": digest}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--prepare-record", action="store_true")
    parser.add_argument("--validate-record", action="store_true")
    parser.add_argument("--compile-fake-only", action="store_true")
    parser.add_argument("--record", type=Path)
    arguments = parser.parse_args()
    try:
        if arguments.compile_fake_only:
            print(json.dumps(compile_fake_only(), sort_keys=True))
        elif arguments.validate_record:
            print(json.dumps(validate_record(arguments.record or PREPARATION_RECORD_PATH), sort_keys=True))
        elif arguments.self_test:
            proof = run_self_test()
            print(json.dumps(proof, indent=2, sort_keys=True))
        elif arguments.prepare_record:
            proof = run_self_test()
            record = {
                "schema": "copilot-cli-acp-no-child-preparation.v1",
                "operation_id": proof["operation_id"],
                "authority_id": proof["authority_id"],
                "prepared_at_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
                "host_identity": proof["host_identity"],
                "runtime_profile_closure": proof["runtime_profile_closure"],
                "synthetic_inputs": proof["synthetic_inputs"],
                "identities": proof["identities"],
                "path_roles": proof["path_roles"],
                "profile_role_bindings": proof["profile_role_bindings"],
                "profile_template": proof["profile_template"],
                "profile_template_sha256": proof["profile_template_sha256"],
                "fake_proof": {
                    "status": proof["status"],
                    "fake_claims": proof["fake_claims"],
                },
                "fake_controls": proof["fake_controls"],
                "diagnostic_request": proof["diagnostic_request"],
                "limitation": proof["limitation"],
            }
            destination = (arguments.record or PREPARATION_RECORD_PATH).resolve(strict=False)
            if not destination.is_relative_to(FIXTURE_DIR.resolve()):
                raise RuntimeError("the preparation record must stay in its committed fixture directory")
            if not destination.parent.is_dir():
                raise RuntimeError("the preparation record destination is unavailable")
            durable_exclusive_record(destination, record)
            print(json.dumps(validate_record(destination), sort_keys=True))
        else:
            parser.error("choose --self-test, --prepare-record, or --validate-record")
    except (OSError, RuntimeError, ValueError, KeyError, TimeoutError, json.JSONDecodeError) as error:
        if isinstance(error, OSError):
            print(f"no-child preparation failed safely (errno={error.errno})", file=sys.stderr)
        elif isinstance(error, RuntimeError):
            print(f"no-child preparation failed safely ({str(error)[:600]})", file=sys.stderr)
        else:
            print(f"no-child preparation failed safely ({type(error).__name__})", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
