#!/usr/bin/env python3
"""Prove the original-profile no-child path with fakes, then run one authorized initialize."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
import selectors
import sys
import tempfile
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
PREPARATION_HARNESS_PATH = ROOT / "scripts/copilot-acp-no-child-diagnostic.py"
FIXTURE_DIR = ROOT / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-no-child-diagnostic"
AUTHORITY_RECORD_PATH = FIXTURE_DIR / "authority-record.json"
ORIGINAL_PROFILE_PROOF_PATH = FIXTURE_DIR / "original-profile-proof-record.json"
ATTEMPT_RECORD_PATH = FIXTURE_DIR / "attempt-record.json"
EXECUTION_RECORD_PATH = FIXTURE_DIR / "execution-record.json"
INVENTORY_PATH = (
    ROOT
    / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-offline-proof/artifact-inventory.json"
)
ORIGINAL_OPERATION_ID = "8cc634d0-762a-448e-abc5-f43b06a100e0"
ORIGINAL_AUTHORITY_ID = "e05b517f-e4fc-46c4-9d52-435b2422b5cf"
COPILOT_VERSION = "1.0.93"
COPILOT_EXECUTABLE_SHA256 = "df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1"
INVENTORY_1_0_93_ARCHIVE_SHA256 = "f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb"
ORIGINAL_PROFILE_TEMPLATE_SHA256 = "4ec0e6643d8ff8e87e1186b4fb6daf3e1968a820203e32f28e0c2d2e38705023"
DYLD_SUPPORT_SHA256 = "06215a5d32689aefe395c29710e182eb54ba22162f50df8b4842290f8a19bf1c"
REQUIRED_HOST = {
    "product_version": "26.6.2",
    "build": "25G83",
    "architecture": "arm64",
}
ABSENT_ENVIRONMENT_NAMES = (
    "GITHUB_TOKEN",
    "GH_TOKEN",
    "COPILOT_GITHUB_TOKEN",
    "COPILOT_MODEL",
    "COPILOT_AUTO_UPDATE",
    "HTTP_PROXY",
    "HTTPS_PROXY",
    "ALL_PROXY",
    "http_proxy",
    "https_proxy",
    "all_proxy",
)
ORIGINAL_ENVIRONMENT_KEYS = (
    "PATH",
    "HOME",
    "COPILOT_HOME",
    "GH_COPILOT_HOME",
    "XDG_CONFIG_HOME",
    "XDG_CACHE_HOME",
    "TMPDIR",
    "TERM",
)
KNOWN_AUTH_METHODS = frozenset({"copilot-login"})
INITIALIZE_OUTCOMES = frozenset(
    {"observed", "rpc-error", "invalid-response", "not-reached", "unknown"}
)
VENDOR_STARTUP_STATES = frozenset({"acp-initialize-response-observed", "unknown"})
AGENT_VERSIONS = frozenset({"1.0.93", "mismatch", "absent", "unknown"})
PROTOCOL_VERSIONS = frozenset({1, "mismatch", "absent", "unknown"})
FAILURE_CLASSES = frozenset(
    {
        "none",
        "timeout",
        "exec-denied",
        "sandbox-denial",
        "malformed-or-oversized",
        "unsupported-protocol",
        "unknown",
    }
)
MAX_PROTOCOL_BYTES = 64 * 1024
FORBIDDEN_RECORD_MARKERS = (
    "SWALLOWTAIL_DIAGNOSTIC_SECRET_SENTINEL",
    "SWALLOWTAIL_RAW_HASH_SENTINEL",
    "SWALLOWTAIL_FAKE_AUTH",
    "github-account:",
    "-----BEGIN",
)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError(f"{path.name} is not a JSON object")
    return value


def load_preparation() -> Any:
    name = "swallowtail_copilot_acp_no_child_preparation"
    spec = importlib.util.spec_from_file_location(name, PREPARATION_HARNESS_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("the reviewed no-child preparation harness is unavailable")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def secret_free(value: dict[str, Any], extra: tuple[str, ...] = ()) -> None:
    encoded = json.dumps(value, sort_keys=True)
    markers = (
        str(ROOT),
        str(Path.home()),
        *FORBIDDEN_RECORD_MARKERS,
        *extra,
    )
    if any(marker and marker in encoded for marker in markers):
        raise ValueError("an original diagnostic record retained a private path, secret, or source path")


def copy_regular_file(source: Path, destination: Path) -> str:
    if source.is_symlink() or destination.exists() and destination.is_symlink():
        raise RuntimeError("original bytes must not be copied through a symlink")
    if not source.is_file():
        raise RuntimeError("original bytes are missing")
    resolved = source.resolve(strict=True)
    if resolved.is_relative_to(ROOT.resolve()):
        raise RuntimeError("original bytes must remain outside the repository tree")
    destination.parent.mkdir(parents=True, exist_ok=True)
    digest = hashlib.sha256()
    flags = os.O_WRONLY | os.O_CREAT | os.O_EXCL
    if hasattr(os, "O_NOFOLLOW"):
        flags |= os.O_NOFOLLOW
    descriptor = os.open(destination, flags, 0o600)
    try:
        with source.open("rb") as src:
            while True:
                chunk = src.read(1024 * 1024)
                if not chunk:
                    break
                digest.update(chunk)
                offset = 0
                while offset < len(chunk):
                    offset += os.write(descriptor, chunk[offset:])
        os.fsync(descriptor)
    finally:
        os.close(descriptor)
    os.chmod(destination, 0o700)
    return digest.hexdigest()


def original_environment(paths: dict[str, Path]) -> dict[str, str]:
    action = paths["action_root"]
    home = action / "home"
    copilot_home = action / "copilot-home"
    xdg_config = action / "xdg-config"
    xdg_cache = action / "xdg-cache"
    temporary = action / "tmp"
    for directory in (home, copilot_home, xdg_config, xdg_cache, temporary):
        directory.mkdir(parents=True, exist_ok=True)
        directory.chmod(0o700)
    environment = {
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "HOME": str(home),
        "COPILOT_HOME": str(copilot_home),
        "GH_COPILOT_HOME": str(copilot_home),
        "XDG_CONFIG_HOME": str(xdg_config),
        "XDG_CACHE_HOME": str(xdg_cache),
        "TMPDIR": str(temporary),
        "TERM": "dumb",
    }
    if tuple(environment) != ORIGINAL_ENVIRONMENT_KEYS:
        raise RuntimeError("the original environment key set drifted from Research 432")
    if any(name in environment for name in ABSENT_ENVIRONMENT_NAMES):
        raise RuntimeError("the original environment included a forbidden token, model, or proxy variable")
    return environment


def symbolic_environment() -> dict[str, str]:
    value = {
        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
        "HOME": "${synthetic_home}",
        "COPILOT_HOME": "${synthetic_copilot_home}",
        "GH_COPILOT_HOME": "${synthetic_copilot_home}",
        "XDG_CONFIG_HOME": "${synthetic_xdg_config}",
        "XDG_CACHE_HOME": "${synthetic_xdg_cache}",
        "TMPDIR": "${synthetic_tmp}",
        "TERM": "dumb",
    }
    for name in ABSENT_ENVIRONMENT_NAMES:
        value[name] = "absent"
    return value


def validate_authority(path: Path = AUTHORITY_RECORD_PATH) -> dict[str, Any]:
    value = load_json(path)
    expected = {
        "schema": "copilot-cli-acp-no-child-original-authority.v1",
        "operator_decision": ORIGINAL_AUTHORITY_ID,
        "operator_ruling": "Approve one isolated initialize attempt",
        "operation_id": ORIGINAL_OPERATION_ID,
        "preparation_operation_id": "d67e7bb3-8314-48f3-92d1-fd73c73f1b77",
        "preparation_authority_id": "4d499c96-4046-4189-b14e-39b1a572e035",
        "version": COPILOT_VERSION,
        "platform": "darwin-arm64",
        "executable_sha256": COPILOT_EXECUTABLE_SHA256,
        "inventory_1_0_93_archive_sha256": INVENTORY_1_0_93_ARCHIVE_SHA256,
        "original_profile_template_sha256": ORIGINAL_PROFILE_TEMPLATE_SHA256,
        "required_host": REQUIRED_HOST,
        "execution_authorized": True,
        "original_invocations_authorized": 1,
        "retries": 0,
        "older_artifact_starts": 0,
        "reviewer_original_execution": False,
        "maximum_initialize_requests": 1,
        "session_new_requests": 0,
        "session_prompt_requests": 0,
        "maximum_seconds": 60,
        "cleanup_seconds": 3,
        "permission_prompt_allowance_untouched": True,
        "legacy_invocation_and_renewal_guards_remain_consumed": True,
        "qualification_changed": False,
    }
    for key, expected_value in expected.items():
        if value.get(key) != expected_value:
            raise ValueError(f"the original authority record drifted at {key}")
    secret_free(value)
    return value


def inventory_1_0_93_identities() -> dict[str, str]:
    inventory = load_json(INVENTORY_PATH)
    native = next(
        package
        for package in inventory["packages"]
        if package["name"] == "@github/copilot-darwin-arm64"
    )
    version = next(item for item in native["versions"] if item["version"] == COPILOT_VERSION)
    executable = next(item for item in version["files"] if item["path"] == "package/copilot")
    if (
        version["archive_sha256"] != INVENTORY_1_0_93_ARCHIVE_SHA256
        or executable["sha256"] != COPILOT_EXECUTABLE_SHA256
    ):
        raise ValueError("the frozen inventory no longer binds the authorized 1.0.93 identities")
    return {
        "archive_sha256": version["archive_sha256"],
        "executable_sha256": executable["sha256"],
    }


def classify_auth_methods(result: dict[str, Any]) -> list[str]:
    methods = result.get("authMethods", [])
    if not isinstance(methods, list):
        return ["unknown"]
    ids = []
    for method in methods:
        if not isinstance(method, dict) or not isinstance(method.get("id"), str):
            return ["unknown"]
        ids.append(method["id"])
    unique = set(ids)
    if not unique:
        return []
    if unique <= KNOWN_AUTH_METHODS:
        return sorted(unique)
    return ["unknown"]


def classify_agent_version(result: dict[str, Any]) -> str:
    info = result.get("agentInfo")
    if not isinstance(info, dict) or "version" not in info:
        return "absent"
    version = info.get("version")
    if version == COPILOT_VERSION:
        return COPILOT_VERSION
    return "mismatch"


def classify_protocol_version(result: dict[str, Any]) -> int | str:
    version = result.get("protocolVersion")
    if version == 1:
        return 1
    if version is None:
        return "absent"
    return "mismatch"


def parse_initialize_frames(stdout: bytes) -> dict[str, Any]:
    if len(stdout) > MAX_PROTOCOL_BYTES:
        raise RuntimeError("the original diagnostic exceeded its protocol cap")
    initialize_outcome = "not-reached"
    result: dict[str, Any] | None = None
    unexpected_inbound = 0
    permission_request_observed = False
    session_update_observed = False
    malformed = False
    for raw_line in stdout.split(b"\n"):
        if not raw_line:
            continue
        if len(raw_line) > MAX_PROTOCOL_BYTES:
            raise RuntimeError("the original diagnostic emitted an overlong protocol line")
        try:
            message = json.loads(raw_line)
        except json.JSONDecodeError:
            malformed = True
            break
        if not isinstance(message, dict):
            malformed = True
            break
        method = message.get("method")
        if method == "session/request_permission":
            permission_request_observed = True
            unexpected_inbound += 1
            continue
        if method == "session/update":
            session_update_observed = True
            unexpected_inbound += 1
            continue
        if method is not None:
            unexpected_inbound += 1
            continue
        if message.get("id") != 1:
            unexpected_inbound += 1
            continue
        if isinstance(message.get("error"), dict):
            initialize_outcome = "rpc-error"
            continue
        payload = message.get("result")
        if not isinstance(payload, dict):
            initialize_outcome = "invalid-response"
            continue
        initialize_outcome = "observed"
        result = payload
    if malformed:
        initialize_outcome = "unknown"
    return {
        "initialize_outcome": initialize_outcome,
        "result": result,
        "unexpected_inbound": unexpected_inbound,
        "permission_request_observed": permission_request_observed,
        "session_update_observed": session_update_observed,
        "malformed": malformed,
        "protocol_bytes": len(stdout),
    }


def sanitized_initialize(parsed: dict[str, Any]) -> dict[str, Any]:
    result = parsed["result"]
    if parsed["initialize_outcome"] != "observed" or not isinstance(result, dict):
        return {
            "initialize_outcome": parsed["initialize_outcome"],
            "vendor_startup_status": "unknown",
            "protocol_version": "unknown",
            "agent_version": "unknown",
            "auth_methods": [],
            "session_new_sent": False,
            "session_prompt_sent": False,
            "permission_request_observed": parsed["permission_request_observed"],
            "session_update_observed": parsed["session_update_observed"],
            "unexpected_inbound": parsed["unexpected_inbound"],
            "protocol_bytes": parsed["protocol_bytes"],
        }
    return {
        "initialize_outcome": "observed",
        "vendor_startup_status": "acp-initialize-response-observed",
        "protocol_version": classify_protocol_version(result),
        "agent_version": classify_agent_version(result),
        "auth_methods": classify_auth_methods(result),
        "session_new_sent": False,
        "session_prompt_sent": False,
        "permission_request_observed": parsed["permission_request_observed"],
        "session_update_observed": parsed["session_update_observed"],
        "unexpected_inbound": parsed["unexpected_inbound"],
        "protocol_bytes": parsed["protocol_bytes"],
    }


def failure_class(summary: dict[str, Any], parsed: dict[str, Any]) -> str:
    if parsed["malformed"] or parsed["protocol_bytes"] > MAX_PROTOCOL_BYTES:
        return "malformed-or-oversized"
    if parsed["permission_request_observed"] or parsed["session_update_observed"]:
        return "unsupported-protocol"
    if summary.get("timed_out"):
        return "timeout"
    if summary.get("errno") not in {None, "none", "unknown"} and not parsed["result"]:
        return "exec-denied"
    if summary.get("stderr_category") == "sandbox-denial":
        return "sandbox-denial"
    if parsed["initialize_outcome"] == "observed":
        return "none"
    if parsed["initialize_outcome"] in {"not-reached", "unknown"}:
        return "unknown"
    return "unknown"


def prove_original_profile(prep: Any) -> dict[str, Any]:
    validate_authority()
    preparation = prep.validate_record(prep.PREPARATION_RECORD_PATH)
    if preparation["execution_authorized"] is not False:
        raise RuntimeError("the reviewed preparation record reopened original launch")
    inventory = inventory_1_0_93_identities()
    host_identity = prep.current_host_identity()
    if host_identity != REQUIRED_HOST:
        raise RuntimeError("the host identity drifted from the reviewed no-child closure")
    offline = prep.load_offline_harness()
    closure = offline.dyld_support_profile_closure()
    if closure["files"][0]["sha256"] != DYLD_SUPPORT_SHA256:
        raise RuntimeError("the imported dyld-support.sb closure drifted")
    with tempfile.TemporaryDirectory(prefix="copilot-acp-no-child-original-profile.") as temporary:
        task_root = Path(temporary).resolve()
        paths = prep.fresh_layout(task_root)
        original_role = paths["artifact_root"] / COPILOT_VERSION / "copilot"
        original_role.parent.mkdir(parents=True, exist_ok=True)
        stage_launcher = paths["artifact_root"] / "copilot-acp-stage-launcher"
        stage_identity = offline.compile_stage_launcher(stage_launcher)
        fake_digest = prep.compile_native(prep.FAKE_SOURCE_PATH, original_role, paths["artifact_root"])
        host_home = Path.home().resolve()
        repository_root = ROOT.resolve()
        profile = prep.render_profile(
            artifact_root=paths["artifact_root"],
            action_root=paths["action_root"],
            record_root=paths["record_root"],
            blocked_home=paths["blocked_home"],
            blocked_repository=paths["blocked_repository"],
            host_home=host_home,
            repository_root=repository_root,
            stage_launcher=stage_launcher,
            executable=original_role,
        )
        profile_roles = {
            "artifact_root": paths["artifact_root"],
            "action_root": paths["action_root"],
            "record_root": paths["record_root"],
            "blocked_home": paths["blocked_home"],
            "blocked_repository": paths["blocked_repository"],
            "host_home": host_home,
            "repository_root": repository_root,
            "stage_launcher": stage_launcher,
            "approved_original_executable": original_role,
        }
        template = prep.symbolic_profile(profile, profile_roles, task_root)
        template_digest = hashlib.sha256(template.encode()).hexdigest()
        if template_digest != ORIGINAL_PROFILE_TEMPLATE_SHA256:
            raise RuntimeError("the original-profile template drifted from the reviewed Research 432 request")
        if "${approved_original_executable}" not in template or "${fake_executable}" in template:
            raise RuntimeError("the original-profile path branch still named the fake executable role")
        environment = prep.child_environment(paths)
        environment["SWALLOWTAIL_DIAGNOSTIC_SHIM"] = str(stage_launcher)
        attempt = prep.run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=original_role,
            arguments=["--acp", "--stdio"],
            paths=paths,
            record_name="original-profile-initialize.json",
            environment=environment,
            offline=offline,
            initialize=True,
            timeout=prep.MAX_DIAGNOSTIC_SECONDS,
        )
        summary = attempt["summary"]
        fixed_lines = prep.parse_fixed_lines(attempt["stdout"])
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
        marker_names = [
            "fork-self-exec",
            "fork-session-escape",
            "fork-system-helper",
            "vfork-self-exec",
            "vfork-system-helper",
            "spawn-self-exec",
            "spawn-system-helper",
            "atexit-child-effect",
        ]
        if expected_policy - fixed_lines:
            raise RuntimeError("original-profile policy counterexample: " + ",".join(sorted(expected_policy - fixed_lines)))
        if expected_children - fixed_lines:
            raise RuntimeError("original-profile child counterexample: " + ",".join(sorted(expected_children - fixed_lines)))
        if any((paths["action_root"] / name).exists() for name in marker_names):
            raise RuntimeError("original-profile child effect occurred before the denied boundary")
        if not (
            summary["stage"] == "acp-initialize"
            and summary["initialize_response_observed"]
            and summary["exit_success"]
            and summary["root_joined"]
            and summary["stderr_reader_joined"]
            and summary["stage_channel_joined"]
            and summary["root_pid_stable"]
        ):
            raise RuntimeError("original-profile fake initialize gate failed")
        if b'"method":"session/new"' in attempt["stdout"] or b'"method":"session/prompt"' in attempt["stdout"]:
            raise RuntimeError("original-profile fake sent a session request")

        replacement = prep.run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=original_role,
            arguments=["--exec-replacement"],
            paths=paths,
            record_name="original-profile-exec-replacement.json",
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
            raise RuntimeError("original-profile self-exec replacement left the owned root")

        helper_replacement = prep.run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=original_role,
            arguments=["--system-helper-replacement"],
            paths=paths,
            record_name="original-profile-system-helper.json",
            environment=environment,
            offline=offline,
        )
        if not (
            helper_replacement["summary"]["exit_success"]
            and not (paths["action_root"] / "system-helper-effect").exists()
        ):
            raise RuntimeError("original-profile exec grant allowed an unlisted system helper")

        cancelled = prep.run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=original_role,
            arguments=["--cancel-ready"],
            paths=paths,
            record_name="original-profile-cancel.json",
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
            raise RuntimeError("original-profile cancel/join fake failed")

        sensitive = prep.run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=original_role,
            arguments=["--diagnostic-secret"],
            paths=paths,
            record_name="original-profile-sanitized.json",
            environment=environment,
            offline=offline,
        )
        sensitive_record = (paths["record_root"] / "original-profile-sanitized.json").read_text()
        if (
            sensitive["summary"]["stderr_total_bytes"] == 0
            or any(
                item in sensitive_record
                for item in (
                    "SWALLOWTAIL_DIAGNOSTIC_SECRET_SENTINEL",
                    "SWALLOWTAIL_RAW_HASH_SENTINEL",
                    str(paths["action_root"]),
                    str(Path.home()),
                )
            )
            or sensitive["summary"]["stderr_raw_persisted"]
            or sensitive["summary"]["stderr_hash_persisted"]
        ):
            raise RuntimeError("original-profile diagnostics retained raw text, a hash, path, or secret")

        early_exit = prep.run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=original_role,
            arguments=["--fail-before-start"],
            paths=paths,
            record_name="original-profile-early-exit.json",
            environment=environment,
            offline=offline,
        )
        if early_exit["summary"]["stage"] != "unknown" or early_exit["summary"]["fake_start_marker_observed"]:
            raise RuntimeError("original-profile EOF without startup was not unknown")

        timed = prep.run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=original_role,
            arguments=["--hang-after-start"],
            paths=paths,
            record_name="original-profile-timeout.json",
            environment=environment,
            offline=offline,
            timeout=0.2,
        )
        if not (
            timed["summary"]["timed_out"]
            and timed["summary"]["root_joined"]
            and timed["summary"]["stderr_reader_joined"]
            and timed["summary"]["stage_channel_joined"]
        ):
            raise RuntimeError("original-profile timeout/stop/join fake failed")

        crash = prep.run_fake(
            profile=profile,
            stage_launcher=stage_launcher,
            executable=original_role,
            arguments=["--acp", "--stdio"],
            paths=paths,
            record_name="original-profile-crash.json",
            environment=environment,
            offline=offline,
            crash_after_record=True,
        )
        if crash != {"crash_record_replay_refused": True, "root_started": False}:
            raise RuntimeError("original-profile one-shot record allowed replay after crash")

        replay_root = paths["record_root"] / "committed-attempt-replay.json"
        prep.durable_exclusive_record(
            replay_root,
            {
                "schema": "copilot-cli-acp-no-child-original-attempt.v1",
                "operation_id": ORIGINAL_OPERATION_ID,
                "authority_id": ORIGINAL_AUTHORITY_ID,
                "one_shot": True,
                "execution_state": "launch-may-have-occurred",
            },
        )
        try:
            prep.durable_exclusive_record(replay_root, {"replay": "must-fail"})
        except FileExistsError:
            committed_replay_refused = True
        else:
            raise RuntimeError("a consumed original attempt record allowed replay")

        proof = {
            "schema": "copilot-cli-acp-no-child-original-profile-proof.v1",
            "status": "passed",
            "operation_id": ORIGINAL_OPERATION_ID,
            "authority_id": ORIGINAL_AUTHORITY_ID,
            "preparation_operation_id": preparation["operation_id"],
            "host_identity": host_identity,
            "runtime_profile_closure": closure,
            "original_profile_template": template,
            "original_profile_template_sha256": template_digest,
            "inventory_identities": inventory,
            "identities": {
                "original_diagnostic_sha256": sha256(Path(__file__).resolve()),
                "preparation_harness_sha256": sha256(PREPARATION_HARNESS_PATH),
                "offline_stage_and_diagnostic_helpers_sha256": sha256(prep.OFFLINE_HARNESS_PATH),
                "fake_source_sha256": sha256(prep.FAKE_SOURCE_PATH),
                "fake_binary_sha256": fake_digest,
                "stage_launcher_source_sha256": stage_identity["source_sha256"],
                "stage_launcher_binary_sha256": stage_identity["binary_sha256"],
            },
            "path_roles": {
                "approved_original_executable": "artifacts/1.0.93/copilot",
                "stage_launcher": "artifacts/copilot-acp-stage-launcher",
                "artifact_root": "artifacts",
                "action_root": "actions/initialize",
                "record_root": "records",
            },
            "fake_claims": {
                "original_profile_path_branch": True,
                "eight_child_attempts_denied": True,
                "real_home_service_and_network_denied": True,
                "same_root_exec_transition": True,
                "eof_without_startup_unknown": True,
                "initialize_observed_once": True,
                "session_new_sent": False,
                "session_prompt_sent": False,
                "timeout_cancel_root_and_readers_joined": True,
                "secret_safe_diagnostics": True,
                "exclusive_record_and_crash_replay_refused": True,
                "committed_attempt_replay_refused": committed_replay_refused,
            },
            "authority_sha256": sha256(AUTHORITY_RECORD_PATH),
            "limitation": (
                "Fake success on the original-profile path proves the reviewed no-child grants "
                "and lifecycle controls. It does not prove Copilot startup, auth, Auto, "
                "permissions, cancel/no-effect, or supported versions."
            ),
        }
        secret_free(proof)
        if proof["authority_sha256"] != sha256(AUTHORITY_RECORD_PATH):
            raise RuntimeError("the original authority digest is unbound")
        return proof


def validate_original_profile_proof(path: Path = ORIGINAL_PROFILE_PROOF_PATH) -> dict[str, Any]:
    value = load_json(path)
    if value.get("schema") != "copilot-cli-acp-no-child-original-profile-proof.v1":
        raise ValueError("the original-profile proof schema is unknown")
    if value.get("status") != "passed":
        raise ValueError("the original-profile proof did not pass")
    if (
        value.get("operation_id") != ORIGINAL_OPERATION_ID
        or value.get("authority_id") != ORIGINAL_AUTHORITY_ID
        or value.get("original_profile_template_sha256") != ORIGINAL_PROFILE_TEMPLATE_SHA256
        or hashlib.sha256(str(value.get("original_profile_template", "")).encode()).hexdigest()
        != ORIGINAL_PROFILE_TEMPLATE_SHA256
        or value.get("host_identity") != REQUIRED_HOST
    ):
        raise ValueError("the original-profile proof is not bound to the reviewed identities")
    claims = value.get("fake_claims", {})
    required = {
        "original_profile_path_branch",
        "eight_child_attempts_denied",
        "real_home_service_and_network_denied",
        "same_root_exec_transition",
        "eof_without_startup_unknown",
        "initialize_observed_once",
        "timeout_cancel_root_and_readers_joined",
        "secret_safe_diagnostics",
        "exclusive_record_and_crash_replay_refused",
        "committed_attempt_replay_refused",
    }
    if any(claims.get(name) is not True for name in required):
        raise ValueError("the original-profile proof is missing a required gate")
    if claims.get("session_new_sent") is not False or claims.get("session_prompt_sent") is not False:
        raise ValueError("the original-profile proof sent a session request")
    identities = value.get("identities", {})
    if identities.get("original_diagnostic_sha256") != sha256(Path(__file__).resolve()):
        raise ValueError("the original-profile proof does not bind this diagnostic source")
    if identities.get("preparation_harness_sha256") != sha256(PREPARATION_HARNESS_PATH):
        raise ValueError("the original-profile proof does not bind the reviewed 432 harness")
    secret_free(value)
    return {"status": "passed", "schema": value["schema"]}


def validate_attempt(path: Path = ATTEMPT_RECORD_PATH) -> dict[str, Any]:
    value = load_json(path)
    if value.get("schema") != "copilot-cli-acp-no-child-original-attempt.v1":
        raise ValueError("the original attempt schema is unknown")
    if (
        value.get("operation_id") != ORIGINAL_OPERATION_ID
        or value.get("authority_id") != ORIGINAL_AUTHORITY_ID
        or value.get("one_shot") is not True
        or value.get("execution_state") not in {"launch-may-have-occurred", "completed"}
        or value.get("executable_sha256") != COPILOT_EXECUTABLE_SHA256
        or value.get("original_profile_template_sha256") != ORIGINAL_PROFILE_TEMPLATE_SHA256
        or value.get("argv") != ["--acp", "--stdio"]
        or value.get("maximum_initialize_requests") != 1
        or value.get("session_new_requests") != 0
        or value.get("session_prompt_requests") != 0
        or value.get("host_identity") != REQUIRED_HOST
    ):
        raise ValueError("the original attempt record is not bound to the reviewed tuple")
    if not isinstance(value.get("rendered_profile_sha256"), str) or len(value["rendered_profile_sha256"]) != 64:
        raise ValueError("the original attempt record omitted its rendered profile digest")
    if "profile" in value or "rendered_profile" in value:
        raise ValueError("the original attempt record retained profile text")
    secret_free(value)
    return value


def validate_execution(path: Path = EXECUTION_RECORD_PATH) -> dict[str, Any]:
    value = load_json(path)
    if value.get("schema") != "copilot-cli-acp-no-child-original-execution.v1":
        raise ValueError("the original execution schema is unknown")
    result = value.get("result", {})
    if (
        value.get("operation_id") != ORIGINAL_OPERATION_ID
        or value.get("authority_id") != ORIGINAL_AUTHORITY_ID
        or value.get("attempt_sha256") != sha256(ATTEMPT_RECORD_PATH)
        or value.get("executable_sha256") != COPILOT_EXECUTABLE_SHA256
        or value.get("qualification_changed") is not False
        or value.get("exact_1_0_80_claim_unchanged") is not True
        or result.get("initialize_outcome") not in INITIALIZE_OUTCOMES
        or result.get("vendor_startup_status") not in VENDOR_STARTUP_STATES
        or result.get("agent_version") not in AGENT_VERSIONS
        or result.get("protocol_version") not in PROTOCOL_VERSIONS
        or result.get("failure_class") not in FAILURE_CLASSES
        or result.get("session_new_sent") is not False
        or result.get("session_prompt_sent") is not False
        or result.get("initialize_requests_sent") != 1
        or result.get("stderr_raw_persisted") is not False
        or result.get("stderr_hash_persisted") is not False
        or result.get("stdout_raw_persisted") is not False
    ):
        raise ValueError("the original execution record is unsafe or unbound")
    methods = result.get("auth_methods", [])
    if not isinstance(methods, list) or any(item not in {"copilot-login", "unknown"} for item in methods):
        raise ValueError("the original execution record used an open auth-method vocabulary")
    if result.get("initialize_outcome") != "observed" and result.get("vendor_startup_status") != "unknown":
        raise ValueError("vendor startup is overstated")
    secret_free(value)
    return {"status": "passed", "initialize_outcome": result["initialize_outcome"]}


def validate_original_records() -> dict[str, Any]:
    authority = validate_authority()
    prep = load_preparation()
    preparation = prep.validate_record(prep.PREPARATION_RECORD_PATH)
    proof_status = None
    if ORIGINAL_PROFILE_PROOF_PATH.exists():
        proof_status = validate_original_profile_proof()["status"]
    attempt_exists = ATTEMPT_RECORD_PATH.exists()
    execution_exists = EXECUTION_RECORD_PATH.exists()
    if attempt_exists ^ execution_exists:
        raise ValueError("the original attempt and execution records must exist together")
    execution_status = None
    if attempt_exists:
        validate_attempt()
        execution_status = validate_execution()
    return {
        "status": "passed",
        "execution_authorized": authority["execution_authorized"],
        "preparation_execution_authorized": preparation["execution_authorized"],
        "original_profile_proof": proof_status,
        "original_execution": execution_status,
    }


def write_proof_record(proof: dict[str, Any], destination: Path) -> None:
    prep = load_preparation()
    if not destination.resolve().is_relative_to(FIXTURE_DIR.resolve()):
        raise RuntimeError("original-profile proof must stay in its fixture directory")
    record = {
        **proof,
        "prepared_at_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
    }
    secret_free(record)
    prep.durable_exclusive_record(destination, record)


def consume_attempt(
    prep: Any,
    *,
    rendered_profile_sha256: str,
    executable_sha256: str,
    identities: dict[str, str],
    host_identity: dict[str, str],
    closure: dict[str, Any],
) -> str:
    if ATTEMPT_RECORD_PATH.exists() or ATTEMPT_RECORD_PATH.is_symlink():
        raise RuntimeError("the original initialize attempt is already consumed; refusing original start")
    if EXECUTION_RECORD_PATH.exists() or EXECUTION_RECORD_PATH.is_symlink():
        raise RuntimeError("an original execution record already exists; refusing original start")
    attempt = {
        "schema": "copilot-cli-acp-no-child-original-attempt.v1",
        "operation_id": ORIGINAL_OPERATION_ID,
        "authority_id": ORIGINAL_AUTHORITY_ID,
        "authority_sha256": sha256(AUTHORITY_RECORD_PATH),
        "one_shot": True,
        "execution_state": "launch-may-have-occurred",
        "record_barrier": "exclusive-create; fsync-file; fsync-directory-before-start",
        "consumed_before_original_start": True,
        "host_identity": host_identity,
        "runtime_profile_closure": closure,
        "original_profile_template_sha256": ORIGINAL_PROFILE_TEMPLATE_SHA256,
        "rendered_profile_sha256": rendered_profile_sha256,
        "executable_sha256": executable_sha256,
        "inventory_1_0_93_archive_sha256": INVENTORY_1_0_93_ARCHIVE_SHA256,
        "argv": ["--acp", "--stdio"],
        "working_directory": "${action_root}",
        "environment": symbolic_environment(),
        "maximum_initialize_requests": 1,
        "session_new_requests": 0,
        "session_prompt_requests": 0,
        "maximum_seconds": 60,
        "cleanup_seconds": 3,
        "identities": identities,
        "consumed_at_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
    }
    secret_free(attempt)
    prep.durable_exclusive_record(ATTEMPT_RECORD_PATH, attempt)
    return sha256(ATTEMPT_RECORD_PATH)


def stdout_contains_initialize_response(payload: bytes) -> bool:
    for raw_line in payload.split(b"\n"):
        if not raw_line:
            continue
        try:
            message = json.loads(raw_line)
        except json.JSONDecodeError:
            continue
        if isinstance(message, dict) and message.get("id") == 1 and "method" not in message:
            return True
    return False


def read_stdout_protocol(process: Any, timeout: float, bounded_timeout: type[Exception]) -> bytes:
    if process.stdout is None:
        raise RuntimeError("the original diagnostic root has no stdout pipe")
    selector = selectors.DefaultSelector()
    selector.register(process.stdout, selectors.EVENT_READ)
    chunks = bytearray()
    deadline = time.monotonic() + timeout
    try:
        while not stdout_contains_initialize_response(chunks):
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise bounded_timeout(bytes(chunks))
            ready = selector.select(remaining)
            if not ready:
                raise bounded_timeout(bytes(chunks))
            chunk = os.read(process.stdout.fileno(), 4096)
            if not chunk:
                break
            chunks.extend(chunk)
            if len(chunks) > MAX_PROTOCOL_BYTES:
                raise RuntimeError("the original diagnostic exceeded its protocol cap")
    finally:
        selector.close()
    return bytes(chunks)


def run_original_initialize(
    *,
    prep: Any,
    offline: Any,
    profile: str,
    stage_launcher: Path,
    executable: Path,
    paths: dict[str, Path],
    environment: dict[str, str],
) -> dict[str, Any]:
    record_path = paths["record_root"] / "original-initialize.json"
    process, stage_descriptor, stderr_collector = prep.start_root(
        profile=profile,
        stage_launcher=stage_launcher,
        executable=executable,
        arguments=["--acp", "--stdio"],
        paths=paths,
        record_path=record_path,
        environment=environment,
        offline=offline,
    )
    started_at = time.monotonic()
    stage = offline.read_stage_channel(stage_descriptor, timeout=min(3.0, prep.MAX_DIAGNOSTIC_SECONDS))
    stdout = b""
    timed_out = False
    launch_exception = False
    parsed = {
        "initialize_outcome": "not-reached",
        "result": None,
        "unexpected_inbound": 0,
        "permission_request_observed": False,
        "session_update_observed": False,
        "malformed": False,
        "protocol_bytes": 0,
    }
    try:
        if process.stdin is None:
            raise RuntimeError("the original diagnostic root has no stdin pipe")
        process.stdin.write(
            b'{"jsonrpc":"2.0","id":1,"method":"initialize",'
            b'"params":{"protocolVersion":1,"clientCapabilities":{}}}\n'
        )
        process.stdin.flush()
        process.stdin.close()
        try:
            remaining = max(0.0, prep.MAX_DIAGNOSTIC_SECONDS - (time.monotonic() - started_at))
            stdout = read_stdout_protocol(process, remaining, prep.BoundedOutputTimeout)
        except prep.BoundedOutputTimeout as error:
            timed_out = True
            stdout = error.output
            if not prep.stop_owned_root(process, timeout=prep.MAX_CLEANUP_SECONDS):
                raise RuntimeError("the owned original root did not stop within its cleanup bound")
    except OSError:
        launch_exception = True
        prep.stop_owned_root(process, timeout=prep.MAX_CLEANUP_SECONDS)
    finally:
        if process.poll() is None and not prep.stop_owned_root(process, timeout=prep.MAX_CLEANUP_SECONDS):
            raise RuntimeError("the owned original root did not join")
        stderr_joined = stderr_collector.join(prep.MAX_CLEANUP_SECONDS)
        stderr = stderr_collector.summary(reader_joined=stderr_joined)
        if not stderr_joined or stderr["reader_error"]:
            raise RuntimeError("the bounded original stderr reader did not join cleanly")
    try:
        parsed = parse_initialize_frames(stdout)
    except RuntimeError:
        parsed["malformed"] = True
        parsed["initialize_outcome"] = "unknown"
        parsed["protocol_bytes"] = len(stdout)
        stdout = b""
    stdout = b""
    initialize = sanitized_initialize(parsed)
    stage_summary = offline.stage_summary_for_launch(
        stage,
        process_exit_observed=process.returncode is not None,
        fake_initialization_confirmed=initialize["initialize_outcome"] == "observed",
        confirmed_stage="acp-initialize",
    )
    summary = {
        "stage": stage_summary["stage"],
        "operation": stage_summary["operation"],
        "errno": stage_summary["errno"],
        "exec_boundary_eof": stage_summary["exec_boundary_eof"],
        "timed_out": timed_out,
        "launch_exception": launch_exception,
        "exit_observed": process.returncode is not None,
        "exit_success": process.returncode == 0,
        "stderr_category": stderr["classification"],
        "stderr_marker_facets": stderr["marker_facets"],
        "stderr_captured_bytes": stderr["captured_bytes"],
        "stderr_total_bytes": stderr["total_bytes"],
        "stderr_total_bytes_capped": stderr["total_bytes_capped"],
        "stderr_raw_persisted": False,
        "stderr_hash_persisted": False,
        "stderr_paths_persisted": False,
        "stderr_secrets_persisted": False,
        "stdout_raw_persisted": False,
        "root_joined": process.poll() is not None,
        "stderr_reader_joined": stderr_joined,
        "stage_channel_joined": stage["channel_joined"],
        "elapsed_milliseconds": min(60000, int((time.monotonic() - started_at) * 1000)),
        **initialize,
        "initialize_requests_sent": 1,
        "failure_class": failure_class(
            {
                "timed_out": timed_out,
                "errno": stage_summary["errno"],
                "stderr_category": stderr["classification"],
            },
            parsed,
        ),
    }
    return summary


def execute_original(retained_original: Path) -> dict[str, Any]:
    prep = load_preparation()
    validate_authority()
    prep.validate_record(prep.PREPARATION_RECORD_PATH)
    inventory = inventory_1_0_93_identities()
    proof = prove_original_profile(prep)
    if proof["original_profile_template_sha256"] != ORIGINAL_PROFILE_TEMPLATE_SHA256:
        raise RuntimeError("original-profile fake proof drifted before original start")
    host_identity = prep.current_host_identity()
    if host_identity != REQUIRED_HOST:
        raise RuntimeError("the host identity drifted before original start")
    offline = prep.load_offline_harness()
    closure = offline.dyld_support_profile_closure()
    if closure["files"][0]["sha256"] != DYLD_SUPPORT_SHA256:
        raise RuntimeError("the imported profile closure drifted before original start")
    temp_root = Path(tempfile.gettempdir()).resolve()
    source = retained_original.resolve(strict=True)
    if source.is_symlink() or not source.is_file() or not source.is_relative_to(temp_root):
        raise RuntimeError("retained original bytes must be a regular file in task temp scratch")
    with tempfile.TemporaryDirectory(prefix="copilot-acp-no-child-original.") as temporary:
        task_root = Path(temporary).resolve()
        if not task_root.is_relative_to(temp_root):
            raise RuntimeError("original scratch escaped the system temp directory")
        paths = prep.fresh_layout(task_root)
        executable = paths["artifact_root"] / COPILOT_VERSION / "copilot"
        digest = copy_regular_file(source, executable)
        if digest != COPILOT_EXECUTABLE_SHA256 or digest != inventory["executable_sha256"]:
            raise RuntimeError("retained original bytes drifted from the authorized 1.0.93 identity")
        stage_launcher = paths["artifact_root"] / "copilot-acp-stage-launcher"
        stage_identity = offline.compile_stage_launcher(stage_launcher)
        host_home = Path.home().resolve()
        repository_root = ROOT.resolve()
        profile = prep.render_profile(
            artifact_root=paths["artifact_root"],
            action_root=paths["action_root"],
            record_root=paths["record_root"],
            blocked_home=paths["blocked_home"],
            blocked_repository=paths["blocked_repository"],
            host_home=host_home,
            repository_root=repository_root,
            stage_launcher=stage_launcher,
            executable=executable,
        )
        profile_roles = {
            "artifact_root": paths["artifact_root"],
            "action_root": paths["action_root"],
            "record_root": paths["record_root"],
            "blocked_home": paths["blocked_home"],
            "blocked_repository": paths["blocked_repository"],
            "host_home": host_home,
            "repository_root": repository_root,
            "stage_launcher": stage_launcher,
            "approved_original_executable": executable,
        }
        template = prep.symbolic_profile(profile, profile_roles, task_root)
        if hashlib.sha256(template.encode()).hexdigest() != ORIGINAL_PROFILE_TEMPLATE_SHA256:
            raise RuntimeError("the rendered original profile drifted from the reviewed template")
        environment = original_environment(paths)
        identities = {
            "original_diagnostic_sha256": sha256(Path(__file__).resolve()),
            "preparation_harness_sha256": sha256(PREPARATION_HARNESS_PATH),
            "offline_stage_and_diagnostic_helpers_sha256": sha256(prep.OFFLINE_HARNESS_PATH),
            "stage_launcher_source_sha256": stage_identity["source_sha256"],
            "stage_launcher_binary_sha256": stage_identity["binary_sha256"],
            "fake_source_sha256": sha256(prep.FAKE_SOURCE_PATH),
        }
        attempt_sha256 = consume_attempt(
            prep,
            rendered_profile_sha256=hashlib.sha256(profile.encode()).hexdigest(),
            executable_sha256=digest,
            identities=identities,
            host_identity=host_identity,
            closure=closure,
        )
        try:
            summary = run_original_initialize(
                prep=prep,
                offline=offline,
                profile=profile,
                stage_launcher=stage_launcher,
                executable=executable,
                paths=paths,
                environment=environment,
            )
        except Exception as error:
            summary = {
                "stage": "unknown",
                "operation": "unknown",
                "errno": "unknown",
                "exec_boundary_eof": False,
                "timed_out": isinstance(error, TimeoutError),
                "launch_exception": True,
                "exit_observed": False,
                "exit_success": False,
                "stderr_category": "unknown",
                "stderr_marker_facets": ["unknown"],
                "stderr_captured_bytes": 0,
                "stderr_total_bytes": 0,
                "stderr_total_bytes_capped": False,
                "stderr_raw_persisted": False,
                "stderr_hash_persisted": False,
                "stderr_paths_persisted": False,
                "stderr_secrets_persisted": False,
                "stdout_raw_persisted": False,
                "root_joined": True,
                "stderr_reader_joined": True,
                "stage_channel_joined": True,
                "elapsed_milliseconds": 0,
                "initialize_outcome": "unknown",
                "vendor_startup_status": "unknown",
                "protocol_version": "unknown",
                "agent_version": "unknown",
                "auth_methods": [],
                "session_new_sent": False,
                "session_prompt_sent": False,
                "permission_request_observed": False,
                "session_update_observed": False,
                "unexpected_inbound": 0,
                "protocol_bytes": 0,
                "initialize_requests_sent": 1,
                "failure_class": "unknown",
            }
        execution = {
            "schema": "copilot-cli-acp-no-child-original-execution.v1",
            "operation_id": ORIGINAL_OPERATION_ID,
            "authority_id": ORIGINAL_AUTHORITY_ID,
            "attempt_sha256": attempt_sha256,
            "authority_sha256": sha256(AUTHORITY_RECORD_PATH),
            "original_profile_proof_sha256": (
                sha256(ORIGINAL_PROFILE_PROOF_PATH) if ORIGINAL_PROFILE_PROOF_PATH.exists() else None
            ),
            "host_identity": host_identity,
            "runtime_profile_closure": closure,
            "executable_sha256": digest,
            "inventory_1_0_93_archive_sha256": INVENTORY_1_0_93_ARCHIVE_SHA256,
            "original_profile_template_sha256": ORIGINAL_PROFILE_TEMPLATE_SHA256,
            "identities": identities,
            "result": summary,
            "qualification_changed": False,
            "exact_1_0_80_claim_unchanged": True,
            "permission_prompt_allowance_untouched": True,
            "legacy_invocation_and_renewal_guards_remain_consumed": True,
            "limitation": (
                "This isolated initialize result is startup evidence for the reviewed no-child "
                "tuple only. It does not prove auth, Auto, permissions, cancel/no-effect, or "
                "supported versions. Fail-closed no-child policy may prevent vendor startup "
                "and is not a route narrowing. The exact 1.0.80 claim is unchanged."
            ),
            "completed_at_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        }
        secret_free(execution, extra=(str(source), str(executable)))
        prep.durable_exclusive_record(EXECUTION_RECORD_PATH, execution)
        return {
            "status": "recorded",
            "initialize_outcome": summary["initialize_outcome"],
            "vendor_startup_status": summary["vendor_startup_status"],
            "failure_class": summary["failure_class"],
            "attempt_sha256": attempt_sha256,
            "execution_sha256": sha256(EXECUTION_RECORD_PATH),
        }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test-original-profile", action="store_true")
    parser.add_argument("--prepare-original-profile-record", action="store_true")
    parser.add_argument("--validate-records", action="store_true")
    parser.add_argument("--execute-original", action="store_true")
    parser.add_argument("--retained-original", type=Path)
    arguments = parser.parse_args()
    try:
        if arguments.validate_records:
            print(json.dumps(validate_original_records(), sort_keys=True))
        elif arguments.self_test_original_profile or arguments.prepare_original_profile_record:
            prep = load_preparation()
            proof = prove_original_profile(prep)
            if arguments.prepare_original_profile_record:
                write_proof_record(proof, ORIGINAL_PROFILE_PROOF_PATH)
                print(json.dumps(validate_original_profile_proof(), sort_keys=True))
            else:
                print(json.dumps({"status": proof["status"], "schema": proof["schema"]}, sort_keys=True))
        elif arguments.execute_original:
            if arguments.retained_original is None:
                parser.error("--execute-original requires --retained-original")
            print(json.dumps(execute_original(arguments.retained_original), sort_keys=True))
        else:
            parser.error(
                "choose --self-test-original-profile, --prepare-original-profile-record, "
                "--validate-records, or --execute-original"
            )
    except (OSError, RuntimeError, ValueError, KeyError, TimeoutError, json.JSONDecodeError) as error:
        if isinstance(error, OSError):
            print(f"original no-child diagnostic failed safely (errno={error.errno})", file=sys.stderr)
        elif isinstance(error, RuntimeError):
            print(f"original no-child diagnostic failed safely ({str(error)[:600]})", file=sys.stderr)
        else:
            print(f"original no-child diagnostic failed safely ({type(error).__name__})", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
