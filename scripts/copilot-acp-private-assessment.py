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
    "crates/swallowtail-adapter-copilot-cli/Cargo.toml",
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
            "capture": "capture exact safe identifier and SHA-256 in private prepared binding before discovery; fake-only records bind its digest",
            "drift": "fail closed before process start",
        },
        "executable_ref": {
            "source": "approved InstalledExecutableTarget",
            "capture": "capture exact safe ExecutableRef and SHA-256 in private prepared binding before discovery; re-hash that reference at the private prepared ProcessService boundary before forwarding the unchanged request",
            "target": "exact frozen native package/copilot bytes",
            "sha256": NATIVE_EXECUTABLE_SHA256,
            "drift": "fail closed before process start; the host still opens by path after the check, leaving a narrow disclosed check-to-open race",
        },
        "environment_ref": {
            "source": "approved host-owned EnvironmentRef",
            "forwarding": "pass unchanged through the prepared driver",
            "capture": "capture exact safe EnvironmentRef identifier and SHA-256 in private prepared binding before discovery; fake-only records bind its digest; never read or serialize environment values",
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
        "attempt_record": "1.0.95-attempt-4.json; one exclusive invocation-consumed record",
        "prompt_record": "1.0.95-prompt-3.json; one exclusive prompt-consumed record",
        "execution_record": "1.0.95-execution.json; one final secret-free result record with artifact, runner, plan, and safe host-binding hashes",
        "creation": "O_CREAT|O_EXCL|O_NOFOLLOW, mode 0600, complete write and fsync file and parent before each effect",
        "failure_policy": "preserve consumed records; never delete, reset, replace or retry",
        "original_execution_enabled": False,
    }, "persistent record policy changed")
    require(plan.get("original_enable_gate") == {
        "enforcement": "test-only prepared-path runner and one-shot ledger are implemented; this preparation plan hard-disables original effects and authorize_original_execution rejects every request",
        "state": "disabled in this preparation turn",
        "enable_only_after": "independent exact-head preparation review and planner-bound retained continuation",
        "before_effect": "capture exact approved ExecutionHostId, ExecutableRef and EnvironmentRef safe identities before discovery; hash approved executable bytes before discovery and re-hash that reference at the private prepared ProcessService boundary before forwarding the exact request; never read or serialize environment values; disclose the remaining check-to-open race",
        "launch_side_persistence": "the same exclusive fsynced writer is exercised for attempt-4 and prompt-3 fake records in fresh scratch before prepared open and prompt; the disabled original branch writes no user-data record and starts no original",
        "one_shot": "attempt-4 exclusive invocation record fsynced before prepared process start; prompt-3 exclusive prompt record fsynced before prepared turn; result follows cancellation, process stop, joins and release",
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


class OriginalExecutionDenied(RuntimeError):
    """The preparation checkpoint has no enabled original-execution path."""


def authorize_original_execution(plan: dict[str, Any], continuation: Any = None) -> None:
    """Fail closed; only a later planner-bound continuation can add a launcher."""
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


def fake_ledger_scenarios(plan: dict[str, Any]) -> dict[str, bool]:
    results: dict[str, bool] = {}
    results["preparation_original_gate_rejected"] = fake_gate_scenarios(plan)
    with tempfile.TemporaryDirectory(prefix="copilot-acp-private-assessment-") as root_text:
        root = Path(root_text)
        scratch = root / "task-scratch"
        scratch.mkdir(mode=0o700)
        sentinel = scratch / "permission-sentinel.txt"
        with sentinel.open("xb") as stream:
            stream.write(SENTINEL_BEFORE)
            stream.flush()
            os.fsync(stream.fileno())
        invocation = root / "1.0.95-attempt-4.json"
        prompt = root / "1.0.95-prompt-3.json"
        before_sha256 = sha256_file(sentinel)
        binding = {
            "execution_host_id_sha256": sha256_bytes(b"fixture.execution-host"),
            "executable_ref_sha256": sha256_bytes(b"fixture.executable-ref"),
            "environment_ref_sha256": sha256_bytes(b"fixture.environment-ref"),
            "executable_bytes_sha256": sha256_bytes(b"fixture executable bytes"),
        }
        invocation_record = canonical_bytes({
            "schema": "copilot-cli-private-assessment-consumed.v1",
            "phase": "invocation-consumed-before-prepared-open",
            "task_id": TASK_ID,
            "version": VERSION,
            "invocation_number": 4,
            "plan_sha256": sha256_bytes(canonical_bytes(plan)),
            "runner_sha256": sha256_file(Path(__file__)),
            "binding": binding,
            "artifact_sha256": NATIVE_EXECUTABLE_SHA256,
            "argv": ARGV,
            "mode": "fake-prepared-ledger",
        })
        write_exclusive_fsynced(invocation, invocation_record)
        order: list[str] = []
        require(invocation.read_bytes() == invocation_record, "invocation record readback differs")
        order.append("invocation-fsynced")

        def fake_prepared_process_start() -> None:
            require(invocation.read_bytes() == invocation_record, "process start lacks durable attempt record")
            order.append("prepared-process-start")

        fake_prepared_process_start()
        prompt_record = canonical_bytes({
            "schema": "copilot-cli-private-assessment-consumed.v1",
            "phase": "prompt-consumed-before-prepared-turn",
            "task_id": TASK_ID,
            "version": VERSION,
            "prompt_number": 3,
            "plan_sha256": sha256_bytes(canonical_bytes(plan)),
            "runner_sha256": sha256_file(Path(__file__)),
            "invocation_record_sha256": sha256_bytes(invocation_record),
            "binding": binding,
            "argv": ARGV,
            "mode": "fake-prepared-ledger",
        })
        write_exclusive_fsynced(prompt, prompt_record)
        require(prompt.read_bytes() == prompt_record, "prompt record readback differs")
        order.append("prompt-fsynced")

        def fake_prepared_prompt_send() -> None:
            require(prompt.read_bytes() == prompt_record, "prompt send lacks durable prompt record")
            order.append("prepared-prompt-send")

        fake_prepared_prompt_send()
        # The Rust prepared-facade test exercises cancellation and measures this
        # same task-owned file. This script only proves record ordering/durability.
        after_sha256 = sha256_file(sentinel)
        require(before_sha256 == after_sha256, "ledger fake changed task bytes")
        duplicate_refused = False
        try:
            write_exclusive_fsynced(invocation, b"replacement")
        except FileExistsError:
            duplicate_refused = True
        retained_after_failure = invocation.read_bytes() == invocation_record and prompt.read_bytes() == prompt_record
        require(
            order == [
                "invocation-fsynced",
                "prepared-process-start",
                "prompt-fsynced",
                "prepared-prompt-send",
            ]
            and duplicate_refused
            and retained_after_failure,
            "one-shot fake ledger did not fail closed and preserve consumption",
        )
        results["exclusive_attempt_and_prompt_records_precede_fake_prepared_effects"] = True
        results["consumed_records_reject_replacement_and_remain_after_failure"] = True
        results["ledger_self_test_measures_real_scratch_bytes"] = True
    return results


def validate_fake_pass_record(record: dict[str, Any], plan_hash: str, results: dict[str, bool]) -> None:
    """Validate the pass record just produced by this self-test run."""
    required = {
        "schema", "task_number", "task_id", "run_id", "plan_sha256", "runner_sha256",
        "status", "originals_run", "original_gate_rejected", "fake_results",
        "fake_results_sha256", "launch_side_capture",
    }
    require(set(record) == required, "generated fake pass key set changed")
    require(record.get("schema") == "copilot-cli-private-assessment-fake-pass.v3", "fake pass schema mismatch")
    require(record.get("task_number") == TASK_NUMBER and record.get("task_id") == TASK_ID and record.get("run_id") == RUN_ID,
            "fake pass task binding changed")
    require(record.get("plan_sha256") == plan_hash, "fake pass plan hash mismatch")
    require(record.get("runner_sha256") == sha256_file(Path(__file__)), "fake pass runner hash mismatch")
    require(record.get("status") == "passed" and record.get("originals_run") is False,
            "fake pass record claims an original or failure")
    require(record.get("original_gate_rejected") is True, "original execution gate was not exercised")
    require(record.get("fake_results") == results, "generated fake result set changed")
    require(record.get("fake_results_sha256") == sha256_bytes(canonical_bytes(results)),
            "generated fake result digest changed")
    require(record.get("launch_side_capture") ==
            "prepared launch rechecks executable bytes before forwarding the unchanged host request; one-shot records are fake-tested in scratch; original stays disabled and the check-to-open race is disclosed",
            "launch-side capture disclosure changed")


def self_test() -> None:
    plan, plan_hash = validate_plan()
    results = fake_ledger_scenarios(plan)
    record = {
        "schema": "copilot-cli-private-assessment-fake-pass.v3",
        "task_number": TASK_NUMBER,
        "task_id": TASK_ID,
        "run_id": RUN_ID,
        "plan_sha256": plan_hash,
        "runner_sha256": sha256_file(Path(__file__)),
        "status": "passed",
        "originals_run": False,
        "original_gate_rejected": results["preparation_original_gate_rejected"],
        "fake_results": results,
        "fake_results_sha256": sha256_bytes(canonical_bytes(results)),
        "launch_side_capture": (
            "prepared launch rechecks executable bytes before forwarding the unchanged host request; "
            "one-shot records are fake-tested in scratch; original stays disabled and the check-to-open race is disclosed"
        ),
    }
    validate_fake_pass_record(record, plan_hash, results)
    # Exercise actual record creation and readback in a fresh scratch directory;
    # the checked-in evidence is generated by this run, never a static attestation.
    with tempfile.TemporaryDirectory(prefix="copilot-acp-assessment-result-") as root_text:
        result_path = Path(root_text) / "fake-pass.json"
        write_exclusive_fsynced(result_path, canonical_bytes(record))
        generated = load_object(result_path)
        validate_fake_pass_record(generated, plan_hash, results)
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
