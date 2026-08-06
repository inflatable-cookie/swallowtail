#!/usr/bin/env python3
"""Validate the fixture-first Muse Code event boundary without a Rust package."""

from __future__ import annotations

import copy
import json
import pathlib
import unittest


ROOT = pathlib.Path(__file__).resolve().parents[1]
FIXTURES = (
    ROOT
    / "crates"
    / "swallowtail-adapter-muse"
    / "tests"
    / "fixtures"
    / "muse-code-0.1.0-R708.1"
)
PROTOCOL = json.loads((FIXTURES / "protocol.json").read_text())
BOUNDS = PROTOCOL["bounds"]
KNOWN = frozenset(PROTOCOL["known_payload_types"])


class CorpusError(ValueError):
    def __init__(self, code: str):
        super().__init__(code)
        self.code = code


def encoded(record: object) -> bytes:
    return json.dumps(record, separators=(",", ":")).encode()


def decode_lines(lines: list[bytes]) -> list[dict]:
    if len(lines) > BOUNDS["maximum_records"]:
        raise CorpusError("record_count_exceeded")
    if sum(len(line) + 1 for line in lines) > BOUNDS["maximum_stream_bytes"]:
        raise CorpusError("stream_limit_exceeded")

    records = []
    for line in lines:
        if len(line) > BOUNDS["maximum_record_bytes"]:
            raise CorpusError("record_limit_exceeded")
        try:
            record = json.loads(line)
        except (UnicodeDecodeError, json.JSONDecodeError) as error:
            raise CorpusError("malformed_record") from error
        if not isinstance(record, dict):
            raise CorpusError("malformed_record")
        records.append(record)
    return records


def load_jsonl(name: str) -> list[dict]:
    data = (FIXTURES / name).read_bytes()
    if not data.endswith(b"\n"):
        raise CorpusError("unterminated_record")
    return decode_lines(data.splitlines())


def require_run_stream(payload: dict, command_id: str) -> None:
    stream = payload.get("run_stream")
    if stream != {"kind": "run", "id": command_id}:
        raise CorpusError("run_mismatch")


def validate_records(
    records: list[dict],
    *,
    expected_provider: str | None = None,
    expected_model: str | None = None,
) -> list[str]:
    if not records:
        raise CorpusError("empty_stream")

    first = records[0]
    if first.get("payload_type") != "runtime.command.accepted":
        raise CorpusError("missing_command_acceptance")
    command_id = first.get("payload", {}).get("command_id")
    if not isinstance(command_id, str) or not command_id.startswith("fixture-command-"):
        raise CorpusError("command_mismatch")

    session_id = first.get("stream", {}).get("id")
    if not isinstance(session_id, str) or not session_id.startswith("fixture-session-"):
        raise CorpusError("session_mismatch")

    terminal_seen = False
    run_linked = False
    run_started = False
    model_seen = False
    output_bytes = 0
    tasks: set[str] = set()
    unknown_observations = []

    for expected_sequence, record in enumerate(records, start=1):
        if record.get("schema_version") != 1 or record.get("payload_schema_version") != 1:
            raise CorpusError("schema_mismatch")
        if record.get("sequence") != expected_sequence:
            raise CorpusError("sequence_mismatch")
        if record.get("stream") != {"kind": "session", "id": session_id}:
            raise CorpusError("session_mismatch")
        if record.get("causation_id") != command_id:
            raise CorpusError("causation_mismatch")

        payload_type = record.get("payload_type")
        payload = record.get("payload")
        if not isinstance(payload_type, str) or not isinstance(payload, dict):
            raise CorpusError("malformed_record")
        if terminal_seen and payload_type in KNOWN:
            raise CorpusError("post_terminal_record")
        payload_command = payload.get("command_id")
        if payload_command is not None and payload_command != command_id:
            raise CorpusError("command_mismatch")

        if payload_type not in KNOWN:
            if len(encoded(payload)) > BOUNDS["maximum_unknown_payload_bytes"]:
                raise CorpusError("unknown_payload_limit_exceeded")
            unknown_observations.append(
                PROTOCOL["unknown_payload"]["observation_namespace_prefix"] + payload_type
            )
            continue

        if "run_stream" in payload:
            require_run_stream(payload, command_id)

        if payload_type == "session.run.linked":
            if run_linked:
                raise CorpusError("duplicate_run_link")
            run_linked = True
        elif payload_type == "run.model.configured":
            if not run_linked:
                raise CorpusError("model_before_run_link")
            if expected_provider is not None and payload.get("provider_id") != expected_provider:
                raise CorpusError("provider_mismatch")
            if expected_model is not None and payload.get("model_id") != expected_model:
                raise CorpusError("model_mismatch")
            model_seen = True
        elif payload_type == "run.lifecycle.started":
            if not run_linked:
                raise CorpusError("start_before_run_link")
            run_started = True
        elif payload_type == "run.output.delta":
            if not run_started:
                raise CorpusError("output_before_start")
            text = payload.get("text")
            if not isinstance(text, str):
                raise CorpusError("malformed_output")
            output_bytes += len(text.encode())
            if output_bytes > BOUNDS["maximum_output_bytes"]:
                raise CorpusError("output_limit_exceeded")
        elif payload_type == "task.stream.linked":
            task_id = payload.get("task_id")
            task_stream = payload.get("task_stream")
            if not isinstance(task_id, str) or task_stream != {"kind": "task", "id": task_id}:
                raise CorpusError("task_mismatch")
            tasks.add(task_id)
        elif payload_type.startswith("task.lifecycle."):
            task_id = payload.get("task_id")
            if task_id not in tasks:
                raise CorpusError("unknown_task")
            if payload.get("task_stream") != {"kind": "task", "id": task_id}:
                raise CorpusError("task_mismatch")
            if payload.get("event", {}).get("task_id") != task_id:
                raise CorpusError("task_mismatch")
        elif payload_type.startswith("run.terminal."):
            if not run_started:
                raise CorpusError("terminal_before_start")
            terminal_seen = True

    if not run_linked or not run_started or not terminal_seen:
        raise CorpusError("incomplete_run")
    if expected_model is not None and not model_seen:
        raise CorpusError("model_missing")
    return unknown_observations


class MuseCodeCorpusTests(unittest.TestCase):
    def test_artifact_and_command_surface_are_exact(self) -> None:
        artifact = json.loads((FIXTURES / "artifact.json").read_text())
        self.assertEqual(artifact["release"], "0.1.0-R708.1")
        self.assertNotEqual(artifact["launcher"]["sha256"], artifact["payload"]["sha256"])
        self.assertTrue(artifact["launcher"]["may_update_before_delegation"])
        self.assertFalse(artifact["launcher"]["selected_as_runtime_artifact"])
        self.assertTrue(artifact["payload"]["selected_as_runtime_artifact"])
        self.assertTrue(artifact["direct_payload_probe"]["echo_jsonl_succeeded"])
        for entry in (artifact["launcher"], artifact["payload"]):
            digest = entry["sha256"]
            self.assertEqual(len(digest), 64)
            int(digest, 16)

        self.assertEqual(
            (FIXTURES / "version.txt").read_text().strip(),
            "Muse Code 0.1.0 (0.1.0-R708.1)",
        )
        root_help = (FIXTURES / "help.txt").read_text()
        exec_help = (FIXTURES / "exec-help.txt").read_text()
        self.assertIn("exec             Run one prompt non-interactively", root_help)
        for option in (
            "--json",
            "--prompt-file",
            "--provider",
            "--model",
            "--reasoning-effort",
            "--max-model-steps",
            "--max-tool-output-bytes",
            "--disable-write",
            "--disable-shell",
            "--no-session-log",
        ):
            self.assertIn(option, exec_help)

    def test_echo_and_meta_success_preserve_exact_correlation(self) -> None:
        echo = load_jsonl("echo-success.jsonl")
        self.assertEqual(validate_records(echo), [])
        self.assertEqual(len(echo), 23)
        self.assertEqual(echo[-1]["payload"]["text"], "echo: fixture prompt")

        meta = load_jsonl("meta-success.jsonl")
        self.assertEqual(
            validate_records(
                meta,
                expected_provider="meta",
                expected_model="muse-spark-1.2",
            ),
            ["muse-code.headless.event.session.workspace_branch.observed"],
        )
        terminal = next(
            record
            for record in meta
            if record["payload_type"] == "run.terminal.completed"
        )
        self.assertEqual(terminal["payload"]["text"], "MUSE_FIXTURE_OK")
        self.assertEqual(
            meta[-1]["payload_type"],
            "session.workspace_branch.observed",
        )
        self.assertEqual(PROTOCOL["meta_success"]["source_record_count"], 26)
        self.assertEqual(PROTOCOL["meta_success"]["reasoning_effort"], "low")
        self.assertFalse(PROTOCOL["meta_success"]["usage_record_observed"])

    def test_unknown_event_is_bounded_namespaced_and_non_authoritative(self) -> None:
        observations = validate_records(load_jsonl("unknown-event.jsonl"))
        self.assertEqual(
            observations,
            ["muse-code.headless.event.provider.future.notice"],
        )
        self.assertFalse(PROTOCOL["unknown_payload"]["semantic_authority"])
        self.assertFalse(PROTOCOL["unknown_payload"]["terminal_authority"])
        self.assertFalse(PROTOCOL["unknown_payload"]["callback_authority"])

    def test_negative_manifest_matches_fail_closed_behavior(self) -> None:
        cases = json.loads((FIXTURES / "negative-cases.json").read_text())["cases"]
        observed = {
            "invalid-json": self.invalid_json,
            "record-over-maximum": self.oversized_record,
            "stream-over-maximum": self.oversized_stream,
            "record-count-over-maximum": self.too_many_records,
            "swap-adjacent-records": self.reordered_sequence,
            "replace-one-session-id": self.cross_session,
            "replace-one-causation-id": self.cross_command,
            "append-record-after-terminal": self.post_terminal,
            "replace-model-id": self.mismatched_model,
            "unknown-payload-over-maximum": self.unknown_oversized,
        }
        self.assertEqual(set(observed), {case["mutation"] for case in cases})
        for case in cases:
            with self.subTest(case=case["name"]):
                with self.assertRaises(CorpusError) as raised:
                    observed[case["mutation"]]()
                self.assertEqual(raised.exception.code, case["expected"])

    def test_corpus_contains_no_private_or_provider_request_material(self) -> None:
        corpus = b"\n".join(path.read_bytes() for path in sorted(FIXTURES.iterdir()))
        for forbidden in (
            b"/Users/",
            b"/private/",
            b"provider_request_id",
            b"provider_response_id",
            b"access_token",
            b"refresh_token",
            b"user_email",
        ):
            self.assertNotIn(forbidden, corpus)

    def invalid_json(self) -> None:
        decode_lines([b'{"schema_version":'])

    def oversized_record(self) -> None:
        decode_lines([b"x" * (BOUNDS["maximum_record_bytes"] + 1)])

    def oversized_stream(self) -> None:
        line = b'"' + b"x" * 900_000 + b'"'
        decode_lines([line] * 10)

    def too_many_records(self) -> None:
        decode_lines([b"{}"] * (BOUNDS["maximum_records"] + 1))

    def reordered_sequence(self) -> None:
        records = load_jsonl("echo-success.jsonl")
        records[4], records[5] = records[5], records[4]
        validate_records(records)

    def cross_session(self) -> None:
        records = load_jsonl("echo-success.jsonl")
        records[4]["stream"]["id"] = "fixture-session-foreign"
        validate_records(records)

    def cross_command(self) -> None:
        records = load_jsonl("echo-success.jsonl")
        records[4]["causation_id"] = "fixture-command-foreign"
        validate_records(records)

    def post_terminal(self) -> None:
        records = load_jsonl("echo-success.jsonl")
        extra = copy.deepcopy(records[-1])
        extra["sequence"] += 1
        records.append(extra)
        validate_records(records)

    def mismatched_model(self) -> None:
        records = load_jsonl("meta-success.jsonl")
        records[2]["payload"]["model_id"] = "muse-spark-foreign"
        validate_records(records, expected_provider="meta", expected_model="muse-spark-1.2")

    def unknown_oversized(self) -> None:
        records = load_jsonl("unknown-event.jsonl")
        records[4]["payload"]["message"] = "x" * BOUNDS["maximum_unknown_payload_bytes"]
        validate_records(records)


if __name__ == "__main__":
    unittest.main(verbosity=2)
