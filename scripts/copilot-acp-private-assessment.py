#!/usr/bin/env python3
"""Validate and fake-prove the disabled Copilot 1.0.95 assessment plan."""

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
FAKE_PASS_PATH = FIXTURE / "private-assessment-fake-pass.json"
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
    "crates/swallowtail-adapter-copilot-cli/src/assessment.rs",
    "crates/swallowtail-adapter-copilot-cli/src/assessment_tests.rs",
    "crates/swallowtail-adapter-copilot-cli/src/discovery.rs",
    "crates/swallowtail-adapter-copilot-cli/src/driver.rs",
    "crates/swallowtail-adapter-copilot-cli/src/lib.rs",
    "crates/swallowtail-adapter-copilot-cli/src/prepared.rs",
    "crates/swallowtail-adapter-copilot-cli/src/prepared/session.rs",
    "crates/swallowtail-adapter-copilot-cli/src/selection.rs",
    "crates/swallowtail-adapter-copilot-cli/tests/support/agent.rs",
    "crates/swallowtail-adapter-copilot-cli/tests/support/mod.rs",
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

    route = plan.get("production_route", {})
    require(route == {
        "execution_host_id": {
            "source": "approved host input to the prepared facade",
            "capture": "persist exact safe identifier and SHA-256 before the first process or prompt",
            "drift": "fail closed before process start",
        },
        "executable_ref": {
            "source": "approved InstalledExecutableTarget",
            "capture": "persist exact safe ExecutableRef and SHA-256 before the first process or prompt",
            "target": "exact frozen native package/copilot bytes",
            "sha256": NATIVE_EXECUTABLE_SHA256,
            "drift": "fail closed before process start",
        },
        "environment_ref": {
            "source": "approved host-owned EnvironmentRef",
            "forwarding": "pass unchanged through the prepared driver",
            "capture": "persist exact safe EnvironmentRef identifier and SHA-256 before the first process or prompt; never read or serialize environment values",
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
        "cleanup": "process, streams, task and resource join/release; assert clean owned-process cleanup",
        "timeout_seconds_including_cleanup": 60,
        "cleanup_budget_seconds": 3,
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
        "attempt_record": "1.0.95-attempt.json; one exclusive invocation-consumed record",
        "prompt_record": "1.0.95-prompt-1.json; one exclusive prompt-consumed record",
        "execution_record": "1.0.95-execution.json; one final secret-free result record with artifact, runner, plan, and safe host-binding hashes",
        "creation": "O_CREAT|O_EXCL|O_NOFOLLOW, mode 0600, fsync file and parent before each effect",
        "failure_policy": "preserve consumed records; never delete, reset, replace or retry",
        "original_execution_enabled": False,
    }, "persistent record policy changed")
    require(plan.get("original_enable_gate") == {
        "state": "disabled in this preparation turn",
        "enable_only_after": "independent exact-head preparation review and planner-bound retained continuation",
        "before_effect": "capture and fsync exact approved ExecutionHostId, ExecutableRef, EnvironmentRef safe identities and SHA-256 values; verify the host/executable/environment tuple has not drifted",
        "one_shot": "exclusive invocation record fsynced before process start; exclusive prompt record fsynced before prompt send",
        "no_bypass": "no public flag, arbitrary version override, alternate transport, model argument, pre-probe, retry, resend or reviewer original",
    }, "original enable gate changed")
    require(plan.get("original_execution_enabled") is False, "original execution must remain disabled")
    require(plan.get("preparation_originals_run") == 0, "preparation must remain fake-only")

    runner = plan.get("runner", {})
    require(runner.get("path") == "scripts/copilot-acp-private-assessment.py", "runner path mismatch")
    require(runner.get("sha256") == sha256_file(Path(__file__)), "runner hash mismatch")
    expected_manifest = source_manifest()
    require(plan.get("prepared_path_sources") == expected_manifest, "prepared-path source identity changed")
    return plan, sha256_bytes(canonical_bytes(plan))


def action_gate(trace: dict[str, Any]) -> bool:
    return (
        trace.get("initialize_version") == VERSION
        and trace.get("session_new") is True
        and trace.get("tool_kind") == "execute"
        and trace.get("announced_tool_call_id") == trace.get("permission_tool_call_id")
        and trace.get("announced_tool_call_id") == "sentinel-edit"
        and trace.get("permission_request_id") == 900
        and trace.get("action_path") == "permission-sentinel.txt"
        and trace.get("announced_old_text") == SENTINEL_BEFORE.decode()
        and trace.get("announced_new_text") == SENTINEL_AFTER.decode()
        and trace.get("permission_reply") == "cancelled"
        and trace.get("prompt_result") == "cancelled"
        and trace.get("session_cancel_sent") is True
        and trace.get("task_directory_before") == [{
            "path": "permission-sentinel.txt",
            "sha256": sha256_bytes(SENTINEL_BEFORE),
        }]
        and trace.get("task_directory_after") == trace.get("task_directory_before")
        and trace.get("sentinel_before") == SENTINEL_BEFORE.decode()
        and trace.get("sentinel_after") == SENTINEL_BEFORE.decode()
        and trace.get("effect_count") == 0
        and trace.get("cleanup_joined") is True
        and trace.get("elapsed_seconds", 61) <= 60
    )


def write_exclusive_fsynced(path: Path, data: bytes) -> None:
    fd = os.open(path, os.O_CREAT | os.O_EXCL | os.O_WRONLY | getattr(os, "O_NOFOLLOW", 0), 0o600)
    try:
        os.write(fd, data)
        os.fsync(fd)
    finally:
        os.close(fd)
    parent_fd = os.open(path.parent, os.O_RDONLY)
    try:
        os.fsync(parent_fd)
    finally:
        os.close(parent_fd)


def fake_ledger_scenarios() -> dict[str, bool]:
    results: dict[str, bool] = {}
    successful = {
        "initialize_version": VERSION,
        "session_new": True,
        "tool_kind": "execute",
        "announced_tool_call_id": "sentinel-edit",
        "permission_tool_call_id": "sentinel-edit",
        "permission_request_id": 900,
        "action_path": "permission-sentinel.txt",
        "announced_old_text": SENTINEL_BEFORE.decode(),
        "announced_new_text": SENTINEL_AFTER.decode(),
        "permission_reply": "cancelled",
        "prompt_result": "cancelled",
        "session_cancel_sent": True,
        "task_directory_before": [{
            "path": "permission-sentinel.txt",
            "sha256": sha256_bytes(SENTINEL_BEFORE),
        }],
        "task_directory_after": [{
            "path": "permission-sentinel.txt",
            "sha256": sha256_bytes(SENTINEL_BEFORE),
        }],
        "sentinel_before": SENTINEL_BEFORE.decode(),
        "sentinel_after": SENTINEL_BEFORE.decode(),
        "effect_count": 0,
        "cleanup_joined": True,
        "elapsed_seconds": 42,
    }
    require(action_gate(successful), "matching cancellation fake was rejected")
    results["correlated_execute_permission_cancel_no_effect_cleanup"] = True

    for name, changes in {
        "missing_action_metadata": {"announced_tool_call_id": None, "action_path": None},
        "mismatched_action_id": {"permission_tool_call_id": "different-tool"},
        "wrong_action_path": {"action_path": "other.txt"},
        "provider_effect": {"sentinel_after": SENTINEL_AFTER.decode(), "effect_count": 1},
        "execution_failure": {"session_new": False},
        "deadline_or_cleanup_failure": {"elapsed_seconds": 61, "cleanup_joined": False},
        "identity_drift": {"initialize_version": "1.0.94"},
    }.items():
        case = dict(successful)
        case.update(changes)
        require(not action_gate(case), f"fake gate accepted {name}")
        results[name + "_fails_closed"] = True

    with tempfile.TemporaryDirectory(prefix="copilot-acp-private-assessment-") as root_text:
        root = Path(root_text)
        invocation = root / "invocation-consumed.json"
        prompt = root / "prompt-consumed.json"
        invocation_record = canonical_bytes({"consumed": True, "phase": "before_fake_process"})
        prompt_record = canonical_bytes({"consumed": True, "phase": "before_fake_prompt"})
        write_exclusive_fsynced(invocation, invocation_record)
        process_started_after_record = invocation.read_bytes() == invocation_record
        write_exclusive_fsynced(prompt, prompt_record)
        prompt_sent_after_record = prompt.read_bytes() == prompt_record
        duplicate_refused = False
        try:
            write_exclusive_fsynced(invocation, b"replacement")
        except FileExistsError:
            duplicate_refused = True
        # A failed fake retains both consumed records; no reset or retry is possible.
        retained_after_failure = invocation.read_bytes() == invocation_record and prompt.read_bytes() == prompt_record
        require(process_started_after_record and prompt_sent_after_record and duplicate_refused and retained_after_failure,
                "consumed fake ledger did not fail closed and persist")
        results["exclusive_fsynced_consumption_and_failure_retention"] = True

    return results


def validate_fake_pass(plan_hash: str) -> None:
    record = load_object(FAKE_PASS_PATH)
    required = {
        "schema", "task_number", "task_id", "run_id", "plan_sha256", "runner_sha256",
        "status", "originals_run", "prepared_route_tests", "fake_results",
    }
    require(set(record) == required, "fake pass record key set changed")
    require(record.get("schema") == "copilot-cli-private-assessment-fake-pass.v1", "fake pass schema mismatch")
    require(record.get("task_number") == TASK_NUMBER and record.get("task_id") == TASK_ID and record.get("run_id") == RUN_ID,
            "fake pass task binding changed")
    require(record.get("plan_sha256") == plan_hash, "fake pass plan hash mismatch")
    require(record.get("runner_sha256") == sha256_file(Path(__file__)), "fake pass runner hash mismatch")
    require(record.get("status") == "passed" and record.get("originals_run") is False,
            "fake pass record claims an original or failure")
    require(record.get("prepared_route_tests") == [
        "assessment_tests::exact_095_assessment_is_private_and_ordinary_preparation_stays_080",
        "assessment_tests::prepared_095_assessment_correlates_execute_permission_cancels_and_joins",
        "assessment_tests::assessment_fake_gate_rejects_missing_or_mismatched_execute_metadata",
    ], "prepared-route test set changed")
    expected_results = {
        "decoded_pending_execute_activity_and_cancelled_activity": True,
        "correlated_execute_permission_cancel_no_effect_cleanup": True,
        "missing_action_metadata_fails_closed": True,
        "mismatched_action_id_fails_closed": True,
        "wrong_action_path_fails_closed": True,
        "provider_effect_fails_closed": True,
        "execution_failure_fails_closed": True,
        "deadline_or_cleanup_failure_fails_closed": True,
        "identity_drift_fails_closed": True,
        "exclusive_fsynced_consumption_and_failure_retention": True,
    }
    require(record.get("fake_results") == expected_results, "fake result set changed")


def self_test() -> None:
    plan, plan_hash = validate_plan()
    require(plan.get("original_execution_enabled") is False, "disabled original gate was enabled")
    require(fake_ledger_scenarios() == {
        "correlated_execute_permission_cancel_no_effect_cleanup": True,
        "missing_action_metadata_fails_closed": True,
        "mismatched_action_id_fails_closed": True,
        "wrong_action_path_fails_closed": True,
        "provider_effect_fails_closed": True,
        "execution_failure_fails_closed": True,
        "deadline_or_cleanup_failure_fails_closed": True,
        "identity_drift_fails_closed": True,
        "exclusive_fsynced_consumption_and_failure_retention": True,
    }, "fake result set differs")
    validate_fake_pass(plan_hash)
    print(f"private assessment plan and fake pass validated; plan sha256 {plan_hash}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--validate-plan", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
    elif args.validate_plan:
        _, plan_hash = validate_plan()
        validate_fake_pass(plan_hash)
        print(f"private assessment plan valid; plan sha256 {plan_hash}")
    else:
        parser.error("select --validate-plan or --self-test")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
