#!/usr/bin/env python3
"""Validate the disabled Copilot 1.0.95 plan and Python-only gate self-check."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import tempfile
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parent.parent
FIXTURE = ROOT / "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-currentness-1.0.95"
PLAN_PATH = FIXTURE / "private-assessment-plan.json"
SCHEMA_PATH = FIXTURE / "private-assessment-plan.schema.json"
LEDGER_PATH = FIXTURE / "artifact-hop-ledger.json"

TASK_NUMBER = 119
TASK_ID = "7823a168-9b8f-4604-8fd7-201c2998722e"
RUN_ID = "9adcb021-e36b-471e-bf53-ad2b741a1b71"
VERSION = "1.0.95"
WRAPPER_ARCHIVE_SHA256 = "838be5537db8bd0f7c7c2e555063770ab559249cf2c3ff3c23b2efc81c854779"
NATIVE_ARCHIVE_SHA256 = "95d49e3023921f0bf694e75e17591b68f8b1e59942c2606e1bc1379828d72b2f"
NATIVE_EXECUTABLE_SHA256 = "35d33e040e8aa0554a02385f02648396ba3b853f026ed1db17d30d051a764e4b"
LEDGER_SHA256 = "66aaa0480237235fc7ee526ece213b800f2f57724e26da836956841d1d97c4e6"
ARGV = ["--acp", "--stdio"]
SENTINEL_BEFORE = b"SWALLOWTAIL_PERMISSION_SENTINEL_BEFORE_V1\n"
SENTINEL_AFTER = b"SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n"
PROMPT = (
    "Overwrite the existing file permission-sentinel.txt in this working "
    "directory with exactly: SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1 followed by a newline."
)

SOURCE_PATHS = [
    "Cargo.lock",
    "crates/swallowtail-adapter-copilot-cli/Cargo.toml",
    "crates/swallowtail-adapter-copilot-cli/src/acp_activity.rs",
    "crates/swallowtail-adapter-copilot-cli/src/acp_activity/tool.rs",
    "crates/swallowtail-adapter-copilot-cli/src/assessment.rs",
    "crates/swallowtail-adapter-copilot-cli/src/assessment_runner.rs",
    "crates/swallowtail-adapter-copilot-cli/src/assessment_tests.rs",
    "crates/swallowtail-adapter-copilot-cli/src/connection.rs",
    "crates/swallowtail-adapter-copilot-cli/src/connection_dispatch.rs",
    "crates/swallowtail-adapter-copilot-cli/src/discovery.rs",
    "crates/swallowtail-adapter-copilot-cli/src/driver.rs",
    "crates/swallowtail-adapter-copilot-cli/src/driver/session.rs",
    "crates/swallowtail-adapter-copilot-cli/src/lib.rs",
    "crates/swallowtail-adapter-copilot-cli/src/prepared.rs",
    "crates/swallowtail-adapter-copilot-cli/src/prepared/session.rs",
    "crates/swallowtail-adapter-copilot-cli/src/selection.rs",
    "crates/swallowtail-adapter-copilot-cli/src/turn.rs",
    "crates/swallowtail-adapter-copilot-cli/src/turn_tests.rs",
    "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-currentness-1.0.95/private-original-binding.schema.json",
    "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-currentness-1.0.95/private-assessment-plan.schema.json",
    "crates/swallowtail-adapter-copilot-cli/tests/support/agent.rs",
    "crates/swallowtail-adapter-copilot-cli/tests/support/mod.rs",
    "effigy.toml",
    "scripts/copilot-acp-original-entry.sh",
]


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: Path) -> str:
    return sha256_bytes(path.read_bytes())


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def source_manifest() -> dict[str, Any]:
    entries = [{"path": path, "sha256": sha256_file(ROOT / path)} for path in SOURCE_PATHS]
    return {"files": entries, "sha256": sha256_bytes(canonical_bytes(entries))}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load_object(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    require(isinstance(value, dict), f"{path.name} must be an object")
    return value


def validate_plan() -> tuple[dict[str, Any], str]:
    plan = load_object(PLAN_PATH)
    schema = load_object(SCHEMA_PATH)
    required = schema.get("required")
    properties = schema.get("properties")
    require(schema.get("$id") == "copilot-cli-private-assessment-plan.v1", "unexpected plan schema")
    require(schema.get("additionalProperties") is False, "plan schema must reject extra keys")
    require(isinstance(required, list) and set(plan) == set(required), "plan keys differ from closed schema")
    require(isinstance(properties, dict) and set(plan) == set(properties), "schema property set differs from plan")
    require(plan.get("schema") == schema["$id"], "plan schema id mismatch")
    require(plan.get("task") == {"number": TASK_NUMBER, "id": TASK_ID, "run_id": RUN_ID}, "plan task binding changed")
    require(plan.get("qualification") == {
        "public_qualified_point": "1.0.80",
        "assessment_point": VERSION,
        "claim_increased": False,
        "interior_points_qualified": False,
        "interior_points": [f"1.0.{minor}" for minor in range(81, 95)],
    }, "qualification boundary changed")

    artifacts = plan.get("artifacts", {})
    require(artifacts == {
        "ledger_path": "crates/swallowtail-adapter-copilot-cli/tests/fixtures/copilot-cli-acp-currentness-1.0.95/artifact-hop-ledger.json",
        "ledger_sha256": LEDGER_SHA256,
        "wrapper_package": "@github/copilot",
        "wrapper_version": VERSION,
        "wrapper_archive_sha256": WRAPPER_ARCHIVE_SHA256,
        "wrapper_manifest_sha256": "c6a3dad731e3d30e93e2cff227f10166bfab357c913038ad045d371fb76cc649",
        "wrapper_loader_sha256": "0ea824a86be5757533fdb092eff7050871bd7a711a46babde0ffe0e44ac5ad88",
        "native_package": "@github/copilot-darwin-arm64",
        "native_version": VERSION,
        "native_archive_sha256": NATIVE_ARCHIVE_SHA256,
        "native_manifest_sha256": "4a6c7286665cfe7ce697f9a1ec929a26fc613a59e714980920e1f5ece6b5023e",
        "native_executable_sha256": NATIVE_EXECUTABLE_SHA256,
        "platform": "darwin-arm64",
    }, "frozen Research 440 artifact identity changed")
    require(sha256_file(LEDGER_PATH) == LEDGER_SHA256, "Research 440 artifact ledger changed")
    effigy = (ROOT / "effigy.toml").read_text(encoding="utf-8")
    require(
        '"validate:copilot-acp-private-assessment" = [' in effigy
        and "--lib -E 'test(/assessment_tests::/)" in effigy
        and '"observe:copilot-acp-private-assessment" = "bash scripts/copilot-acp-original-entry.sh {args}"' in effigy,
        "the fake validation and dedicated observe selector must remain separate",
    )
    validation_block = effigy.split('"validate:copilot-acp-private-assessment" = [', 1)[1].split("\n]", 1)[0]
    launcher = (ROOT / "scripts/copilot-acp-original-entry.sh").read_text(encoding="utf-8")
    require(
        "run-ignored" not in validation_block
        and "observe:copilot-acp-private-assessment" not in validation_block
        and "--run-ignored ignored-only" in launcher
        and "assessment_tests::runner::invoke_reviewed_original_entry" in launcher,
        "ordinary validation must exclude the ignored real-original entry",
    )
    binding_schema = load_object(FIXTURE / "private-original-binding.schema.json")
    require(
        binding_schema.get("$id") == "copilot-acp-private-original-binding.v1"
        and binding_schema.get("additionalProperties") is False
        and binding_schema.get("maxBytes") == 8192
        and set(binding_schema.get("properties", {})) == set(binding_schema.get("required", [])) == {
            "schema", "reviewed_head", "plan_sha256", "enable_command",
            "execution_host_id", "executable_ref", "environment_ref",
            "working_resource_ref", "home_directory",
        },
        "the bounded original binding payload schema changed",
    )
    runner_source = (ROOT / "crates/swallowtail-adapter-copilot-cli/src/assessment_runner.rs").read_text(
        encoding="utf-8"
    )
    require(
        "fn frozen_enable_entry_dispatches_payload_through_the_one_shot_runner()" in runner_source
        and "fn invoke_reviewed_original_from_environment()" in runner_source
        and "run_original_task_once(" in runner_source
        and "OriginalRunPolicy::production()" in runner_source,
        "the observe entry no longer dispatches into the fixed production one-shot runner",
    )

    route = plan.get("production_route", {})
    require(route == {
        "execution_host_id": {
            "source": "approved host input to the prepared facade",
            "capture": "capture the exact safe identifier and SHA-256 before discovery; persist both in the private mode-0600 invocation and prompt records before their effects",
            "drift": "fail closed before process start",
        },
        "executable_ref": {
            "source": "approved InstalledExecutableTarget",
            "capture": "capture the exact safe ExecutableRef and SHA-256 before discovery; persist both in the private mode-0600 invocation and prompt records before their effects; re-hash the reference at the private prepared ProcessService boundary before forwarding the unchanged request",
            "target": "exact frozen native package/copilot bytes",
            "sha256": NATIVE_EXECUTABLE_SHA256,
            "drift": "fail closed before process start; the host still opens by path after the check, leaving a narrow disclosed check-to-open race",
        },
        "launch_verification": "copy the captured exact executable bytes to a task-owned mode-0700 directory and mode-0400 file with no planned writers; verify approved identity and path bytes at the prepared ProcessService boundary, forward the unchanged request, and hash the approved path and protected copy after cleanup; accept and disclose the normal-host check-to-open path race",
        "environment_ref": {
            "source": "approved host-owned EnvironmentRef",
            "forwarding": "pass unchanged through the prepared driver",
            "capture": "capture the exact safe EnvironmentRef identifier and SHA-256 before discovery; persist both in the private mode-0600 invocation and prompt records before their effects; accept only the host home path in the private binding payload and never read or serialize credential/configuration environment values",
            "drift": "fail closed before process start",
        },
        "argv": ARGV,
        "model_policy": "provider configured/default model; no model argument; record only if safely exposed",
        "login": "existing betterthanclay host-owned login; no credential or account-state read",
        "working_resource": "fresh task-owned scratch containing only the permission sentinel",
        "containment": "task scratch and joined owned process only; no arbitrary host or escaped-descendant containment claim",
    }, "approved production tuple changed")

    admission = plan.get("assessment_admission", {})
    require(admission == {
        "visibility": "crate-internal test build only; cfg(test); absent from ordinary consumers",
        "exact_version": VERSION,
        "arbitrary_version_override": False,
        "shares_public_preparation_driver_protocol_permission_and_cleanup": True,
        "public_claim": "QualifiedOnly 1.0.80",
        "ordinary_preparation_rejects_assessment_point": True,
        "artifact_identity_precheck": True,
    }, "private assessment boundary changed")

    action = plan.get("action", {})
    require(action == {
        "prompt": PROMPT,
        "path": "permission-sentinel.txt",
        "before_bytes_utf8": SENTINEL_BEFORE.decode(),
        "requested_after_bytes_utf8": SENTINEL_AFTER.decode(),
        "pending_tool_kind": "execute",
        "permission_tool_call_id_must_equal_pending_announcement_id": True,
        "permission_request_id_is_observed_separately": True,
        "reply": "cancelled only",
        "approval": False,
        "resend": False,
        "effect_limit": 0,
        "opaque_or_missing_action_metadata": "incomplete; refuse qualification",
    }, "sentinel action changed")

    lifecycle = plan.get("lifecycle", {})
    require(lifecycle == {
        "initialize_version": VERSION,
        "session_new": "success",
        "prompt_result": "cancelled",
        "route_terminal_status": "ProviderRequestObserved (existing public permission-stop behavior)",
        "task_directory_bytes_unchanged": True,
        "cleanup": "proof runner cancels the active prompt, then uses the existing process stop, task join and resource release paths; assert clean owned-process cleanup",
        "timeout_seconds_including_cleanup": 60,
        "cleanup_budget_seconds": 3,
        "proof_runner_deadline": "absolute monotonic cutoff begins before prepared open; stop work at 57 seconds and reserve up to 3 seconds for cancellation and cleanup; public TurnRequest deadline rejection remains unchanged",
    }, "lifecycle expectations changed")

    attempt = plan.get("attempt", {})
    require(attempt == {
        "invocations_consumed_before": 3,
        "maximum_invocations": 1,
        "prompts_consumed_before": 2,
        "shared_prompt_slots_remaining_before": 1,
        "maximum_prompts": 1,
        "maximum_seconds_including_cleanup": 60,
        "cleanup_seconds": 3,
        "retries": 0,
        "resends": 0,
        "model_fallbacks": 0,
        "reviewer_original_attempts": 0,
        "failure_consumes_invocation": True,
        "prompt_slot_consumed_and_fsynced_before_send": True,
    }, "one-attempt budget or consumed history changed")

    records = plan.get("records", {})
    require(records == {
        "root": "$HOME/Library/Application Support/Swallowtail/Copilot ACP Permission Proof/Task 119",
        "attempt_record": "1.0.95-attempt-4.json; one exclusive invocation-consumed record with exact safe host, executable, and environment identifiers and their hashes",
        "prompt_record": "1.0.95-prompt-3.json; one exclusive prompt-consumed record with the same exact safe identifiers and hashes",
        "execution_record": "1.0.95-execution.json; one final secret-free result record with artifact, runner, plan, and safe host-binding hashes",
        "creation": "O_CREAT|O_EXCL|O_NOFOLLOW, mode 0600, complete write and fsync file and parent before each effect",
        "failure_policy": "preserve consumed records; never delete, reset, replace or retry",
        "original_execution_enabled": False,
    }, "persistent record policy changed")
    require(plan.get("original_enable_gate") == {
        "enforcement": "private ignored Rust test entry binds the planner-reviewed head, exact plan, host-binding digest and command before constructing real LocalHostServices; the committed exact plan fails closed before host input resolution and original effects; Python only validates and cannot launch",
        "command_dispatch": "effigy observe:copilot-acp-private-assessment <binding-payload.json> selects only the ignored original-entry test; the ordinary validate selector runs fake dispatch tests and never selects the ignored entry",
        "state": "disabled in this preparation turn",
        "enable_only_after": "independent exact-head preparation review and planner-bound retained continuation",
        "before_effect": "read only a private mode-0600 bounded payload with exact safe ExecutionHostId, ExecutableRef, EnvironmentRef, working-resource ref and HOME path; bind its safe identifier digest to the plan; verify the current HOME matches; persist safe IDs and hashes in consumed records before effects; hash approved executable bytes before discovery; freeze bytes in private mode-0700/mode-0400 scratch; re-hash approved path at prepared ProcessService boundary and forward the exact request; hash original path and protected copy after cleanup; do not read or serialize credential/configuration environment values; accept and disclose trusted normal-host check-to-open path race",
        "binding_sha256": None,
        "binding_payload_schema": "copilot-acp-private-original-binding.v1; exact nine keys, mode 0600, maximum 8192 bytes; gate digest is compact sorted-key JSON over the five safe host-binding fields",
        "launch_side_persistence": "the disabled Rust original entry returns before resolving host services or writing user-data records; after planner binding, the same one-shot runner writes exclusive fsynced attempt-4 and prompt-3 records before prepared open and prompt",
        "one_shot": "attempt-4 exclusive invocation record fsynced before prepared process start; prompt-3 exclusive prompt record fsynced before prepared turn; result follows cancellation, process stop, joins and release",
        "enable_command": "effigy observe:copilot-acp-private-assessment <binding-payload.json>",
        "no_bypass": "no public flag, arbitrary version override, alternate transport, model argument, pre-probe, retry, resend or reviewer original",
    }, "original enable gate changed")
    require(plan.get("original_execution_enabled") is False, "original execution must remain disabled")
    require(plan.get("preparation_originals_run") == 0, "preparation must remain fake-only")

    runner = plan.get("runner", {})
    require(runner.get("path") == "scripts/copilot-acp-private-assessment.py", "runner path mismatch")
    require(runner.get("sha256") == sha256_file(Path(__file__)), "runner hash mismatch")
    require(runner.get("implementation_path") == "crates/swallowtail-adapter-copilot-cli/src/assessment_runner.rs", "runner implementation path mismatch")
    require(runner.get("implementation_sha256") == sha256_file(ROOT / runner["implementation_path"]), "runner implementation hash mismatch")
    expected_manifest = source_manifest()
    require(plan.get("prepared_path_sources") == expected_manifest, "prepared-path source identity changed")
    return plan, sha256_bytes(canonical_bytes(plan))


class OriginalExecutionDenied(RuntimeError):
    """The preparation checkpoint has no enabled original-execution path."""


def authorize_original_execution(plan: dict[str, Any], continuation: Any = None) -> None:
    """Keep the Python entrypoint permanently unable to launch an original."""
    if plan.get("original_execution_enabled") is not False:
        raise OriginalExecutionDenied("preparation plan is not in the disabled state")
    if continuation is not None:
        raise OriginalExecutionDenied("this preparation build has no continuation-token consumer")
    raise OriginalExecutionDenied("original execution is not available in the preparation build")


def write_exclusive_fsynced(path: Path, data: bytes) -> None:
    fd = os.open(path, os.O_CREAT | os.O_EXCL | os.O_WRONLY | getattr(os, "O_NOFOLLOW", 0), 0o600)
    try:
        remaining = memoryview(data)
        while remaining:
            written = os.write(fd, remaining)
            if written <= 0:
                raise OSError("exclusive proof record write made no progress")
            remaining = remaining[written:]
        os.fsync(fd)
    finally:
        os.close(fd)
    parent_fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(parent_fd)
    finally:
        os.close(parent_fd)


def validate_self_check_record(record: dict[str, Any], plan_hash: str) -> None:
    """Validate only this Python validator's own gate and record round trip."""
    required = {
        "schema", "task_number", "task_id", "run_id", "plan_sha256", "runner_sha256",
        "status", "originals_run", "plan_disabled", "python_gate_rejected", "driver_proof",
    }
    require(set(record) == required, "generated self-check key set changed")
    require(record.get("schema") == "copilot-cli-private-assessment-runner-self-check.v1", "self-check schema mismatch")
    require(record.get("task_number") == TASK_NUMBER and record.get("task_id") == TASK_ID and record.get("run_id") == RUN_ID,
            "self-check task binding changed")
    require(record.get("plan_sha256") == plan_hash, "self-check plan hash mismatch")
    require(record.get("runner_sha256") == sha256_file(Path(__file__)), "self-check runner hash mismatch")
    require(record.get("status") == "passed" and record.get("originals_run") is False,
            "self-check record claims an original or failure")
    require(record.get("plan_disabled") is True, "self-check plan is not disabled")
    require(record.get("python_gate_rejected") is True, "Python entrypoint did not reject original execution")
    require(record.get("driver_proof") == "not claimed by Python self-test; see Rust prepared-path tests",
            "Python self-test must not claim prepared-driver behavior")


def self_test() -> None:
    plan, plan_hash = validate_plan()
    gate_rejected = fake_gate_scenarios(plan)
    record = {
        "schema": "copilot-cli-private-assessment-runner-self-check.v1",
        "task_number": TASK_NUMBER,
        "task_id": TASK_ID,
        "run_id": RUN_ID,
        "plan_sha256": plan_hash,
        "runner_sha256": sha256_file(Path(__file__)),
        "status": "passed",
        "originals_run": False,
        "plan_disabled": plan.get("original_execution_enabled") is False,
        "python_gate_rejected": gate_rejected,
        "driver_proof": "not claimed by Python self-test; see Rust prepared-path tests",
    }
    validate_self_check_record(record, plan_hash)
    with tempfile.TemporaryDirectory(prefix="copilot-acp-assessment-result-") as root_text:
        result_path = Path(root_text) / "runner-self-check.json"
        write_exclusive_fsynced(result_path, canonical_bytes(record))
        generated = load_object(result_path)
        validate_self_check_record(generated, plan_hash)
    print(json.dumps(record, sort_keys=True, separators=(",", ":")))


def fake_gate_scenarios(plan: dict[str, Any]) -> bool:
    for continuation in (None, {"reviewed_head": "untrusted"}):
        try:
            authorize_original_execution(plan, continuation)
        except OriginalExecutionDenied:
            continue
        raise ValueError("preparation build allowed an original execution")
    return True


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--validate-plan", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
    elif args.validate_plan:
        _, plan_hash = validate_plan()
        print(f"private assessment plan valid; plan sha256 {plan_hash}")
    else:
        parser.error("select --validate-plan or --self-test")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
