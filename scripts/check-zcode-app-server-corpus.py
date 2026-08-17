#!/usr/bin/env python3
"""Validate the redacted ZCode app-server fixture corpus.

This validator intentionally has no package or network dependency. It freezes
the low-level evidence needed by the Rust driver before that package exists.
It validates line-delimited JSON framing, the create/preferences handshake,
the admitted lifecycle, correlation, redaction, and stream bounds. It does
not replay a provider conversation.
"""

from __future__ import annotations

import copy
import json
import sys
import unittest
from pathlib import Path
from typing import Any, Iterable


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
FIXTURE_ROOT = (
    REPOSITORY_ROOT
    / "crates"
    / "swallowtail-adapter-zcode"
    / "tests"
    / "fixtures"
    / "zcode-runtime-0.16.3"
)

EXPECTED_PAYLOAD_SHA256 = (
    "3e3433d90fa502e5d02498dfde6c2090df898331359bcfe5f3dbc9a1d00b685f"
)
EXPECTED_LAUNCHER_SHA256 = (
    "36b9cb48bb79eab0c568909fb9830750f68f701a5aab16cb181c735909555362"
)

RUN_FIXTURES = {
    "text-success.jsonl": ("completed", False),
    "tool-success.jsonl": ("completed", False),
    "tool-error.jsonl": ("completed", False),
    "missing-key.jsonl": ("error", False),
    "unknown-event.jsonl": ("completed", True),
}

CREATE_RESULT_KEYS = {
    "messages",
    "projection",
    "protocol",
    "runtime",
    "session",
    "settings",
    "slashCommands",
    "todoGroups",
    "todos",
}
PREFS_KEYS = {
    "nativeSearchEnhancementsEnabled",
    "memoryEnabled",
    "askUserQuestionAutoResolutionEnabled",
}


class CorpusError(ValueError):
    """A fixture violates a frozen stream or redaction rule."""


def fail(message: str) -> None:
    raise CorpusError(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def reject_json_constant(value: str) -> None:
    fail(f"non-standard JSON constant: {value}")


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(
            path.read_text(encoding="utf-8"),
            parse_constant=reject_json_constant,
        )
    except (OSError, json.JSONDecodeError, CorpusError) as exc:
        fail(f"cannot decode {path.name}: {exc}")
    require(isinstance(value, dict), f"{path.name} must contain an object")
    return value


def load_protocol() -> dict[str, Any]:
    return load_json(FIXTURE_ROOT / "protocol.json")


def id_key(value: Any) -> tuple[type[Any], Any]:
    require(
        isinstance(value, (str, int)) and not isinstance(value, bool),
        "frame id must be a string or number",
    )
    return (type(value), value)


def require_string(value: Any, label: str) -> str:
    require(isinstance(value, str) and value, f"{label} must be a non-empty string")
    return value


def require_safe_error(error: Any, label: str) -> None:
    require(isinstance(error, dict), f"{label} must be an object")
    require(set(error).issubset({"code", "name"}), f"{label} contains an unbounded error field")
    require("code" in error or "name" in error, f"{label} has no safe code")
    for key, value in error.items():
        require_string(value, f"{label}.{key}")


def decode_frames(data: bytes, bounds: dict[str, int]) -> list[dict[str, Any]]:
    require(
        len(data) <= bounds["maximum_stream_bytes"],
        "stream exceeds maximum_stream_bytes",
    )
    require(data.endswith(b"\n"), "stream must end with one newline")

    raw_lines = data.split(b"\n")[:-1]
    require(
        len(raw_lines) <= bounds["maximum_live_notifications"],
        "stream exceeds maximum_live_notifications",
    )
    frames: list[dict[str, Any]] = []
    for line_number, raw_line in enumerate(raw_lines, start=1):
        require(raw_line, f"blank frame at line {line_number}")
        require(not raw_line.endswith(b"\r"), f"CRLF frame at line {line_number}")
        require(
            len(raw_line) <= bounds["maximum_frame_bytes"],
            f"frame {line_number} exceeds maximum_frame_bytes",
        )
        try:
            frame = json.loads(
                raw_line.decode("utf-8"),
                parse_constant=reject_json_constant,
            )
        except (UnicodeDecodeError, json.JSONDecodeError, CorpusError) as exc:
            fail(f"malformed frame {line_number}: {exc}")
        require(isinstance(frame, dict), f"frame {line_number} is not an object")
        require("jsonrpc" not in frame, f"frame {line_number} is JSON-RPC, not ZCode protocol")
        has_method = "method" in frame
        has_result = "result" in frame
        has_error = "error" in frame
        has_id = "id" in frame
        if has_method:
            require(
                isinstance(frame["method"], str) and frame["method"],
                f"frame {line_number} has invalid method",
            )
            require(not has_result and not has_error, f"frame {line_number} mixes method with result")
        else:
            require(has_id, f"frame {line_number} response has no id")
            require(has_result ^ has_error, f"frame {line_number} has invalid response shape")
        if has_id:
            id_key(frame["id"])
        frames.append(frame)
    require(frames, "stream contains no frames")
    return frames


def frame_bytes(frames: Iterable[dict[str, Any]]) -> bytes:
    return b"".join(
        json.dumps(frame, sort_keys=True, separators=(",", ":")).encode("utf-8") + b"\n"
        for frame in frames
    )


def validate_redaction(frames: list[dict[str, Any]]) -> None:
    forbidden_fragments = (
        "/Users/",
        "/home/",
        "/private/",
        "sk-",
        "Bearer ",
        "api_key",
        "access_token",
        "refresh_token",
        "Authorization",
        "sess_",
    )

    def walk(value: Any) -> Iterable[str]:
        if isinstance(value, str):
            yield value
        elif isinstance(value, dict):
            for key, child in value.items():
                yield from walk(key)
                yield from walk(child)
        elif isinstance(value, list):
            for child in value:
                yield from walk(child)

    for value in walk(frames):
        require(
            not any(fragment in value for fragment in forbidden_fragments),
            "fixture contains credential, session, or private-path material",
        )


def validate_create_result(result: Any, artifact: dict[str, Any]) -> str:
    require(isinstance(result, dict), "create result is not an object")
    require(set(result) == CREATE_RESULT_KEYS, "create result keys drifted")
    runtime = result.get("runtime")
    require(isinstance(runtime, dict), "create runtime is not an object")
    require(
        runtime.get("cliVersion") == artifact["published_identity"],
        "create runtime does not match the pinned artifact",
    )
    session = result.get("session")
    require(isinstance(session, dict), "create session is not an object")
    session_id = require_string(session.get("sessionId"), "sessionId")
    require(session_id.startswith("fixture-session-"), "session id is not a fixture marker")
    workspace = session.get("workspace")
    require(isinstance(workspace, dict), "workspace is not an object")
    require(workspace.get("workspacePath") == "<fixture-cwd>", "cwd is not redacted")
    require(workspace.get("workspaceKey") == "<fixture-cwd>", "workspace key is not redacted")
    require(session.get("mode") == "plan", "create mode is not the host-supplied plan mode")
    require(session.get("model") == "fixture-model", "create model is not the fixture model")
    return session_id


def validate_handshake(
    frames: list[dict[str, Any]],
    protocol: dict[str, Any],
    artifact: dict[str, Any],
) -> str:
    require(len(frames) >= 4, "handshake is truncated")
    create = frames[0]
    require(create.get("method") == "session/create", "handshake does not start with session/create")
    require(create.get("id") == 1, "create request id drifted")
    params = create.get("params")
    require(isinstance(params, dict), "create params are not an object")
    require(params.get("mode") == "plan", "create mode is not plan")
    workspace = params.get("workspace")
    require(isinstance(workspace, dict), "create workspace is not an object")
    require(workspace.get("workspacePath") == "<fixture-cwd>", "create cwd is not redacted")

    prefs_req = frames[1]
    require(
        prefs_req.get("method") == "session/requestRuntimePreferences",
        "create did not block on runtime preferences",
    )
    require("id" in prefs_req, "runtime preferences request has no id")
    prefs_params = prefs_req.get("params")
    require(isinstance(prefs_params, dict), "runtime preferences params are not an object")
    require(prefs_params.get("scope") == "runtime-materialization", "runtime preferences scope drifted")
    session_id = require_string(prefs_params.get("sessionId"), "preferences sessionId")

    prefs_reply = frames[2]
    require(prefs_reply.get("id") == prefs_req.get("id"), "preferences reply id mismatch")
    require("method" not in prefs_reply, "preferences reply is not a result")
    result = prefs_reply.get("result")
    require(isinstance(result, dict), "preferences result is not an object")
    require(set(result) == PREFS_KEYS, "preferences result keys drifted")
    expected = protocol["runtime_preferences"]
    require(result == expected, "preferences result is not the fail-closed default")

    create_result = frames[3]
    require(create_result.get("id") == 1, "create result id mismatch")
    require("method" not in create_result, "create result is not a response")
    admitted = validate_create_result(create_result.get("result"), artifact)
    require(admitted == session_id, "create session id does not match preferences")
    return session_id


def validate_stream(
    frames: list[dict[str, Any]],
    protocol: dict[str, Any],
    artifact: dict[str, Any],
    *,
    expected_terminal: str,
    expect_unknown: bool = False,
    handshake_only: bool = False,
) -> dict[str, Any]:
    known_events = set(protocol["known_event_types"])
    namespace = protocol["rules"]["unknown_namespace"]
    bounds = protocol["bounds"]
    session_id = validate_handshake(frames, protocol, artifact)
    if handshake_only:
        require(len(frames) == 4, "handshake fixture has extra frames")
        return {"terminal": None, "unknown_observations": 0, "tool_calls": 0}

    pending: dict[tuple[type[Any], Any], str] = {}
    subscribed = False
    send_requested = False
    send_accepted = False
    sequence = 0
    terminal: str | None = None
    unknown_observations = 0
    tool_calls: set[str] = set()
    tool_finished: set[str] = set()
    text_deltas: list[str] = []
    usage_count = 0
    tool_record_count = 0

    for frame_number, frame in enumerate(frames[4:], start=5):
        if "method" not in frame:
            response_id = id_key(frame["id"])
            operation = pending.pop(response_id, None)
            require(operation is not None, f"frame {frame_number} has unmatched response")
            if "error" in frame:
                require_safe_error(frame["error"], f"frame {frame_number}.error")
                fail(f"frame {frame_number} returned an unapproved protocol error")
            result = frame.get("result")
            if operation == "subscribe":
                require(isinstance(result, dict), "subscribe result is not an object")
                require(result.get("eventSeq") == 0, "subscribe eventSeq drifted")
                subscribed = True
            elif operation == "send":
                require(isinstance(result, dict), "send result is not an object")
                require(result.get("accepted") is True, "send result is not an enqueue receipt")
                send_accepted = True
            else:
                fail(f"unknown pending operation: {operation}")
            continue

        method = frame["method"]
        params = frame.get("params")
        if method == "session/subscribe":
            require(not subscribed and not send_requested, "subscribe is out of order")
            require(isinstance(params, dict), "subscribe params are not an object")
            require(params.get("sessionId") == session_id, "subscribe session mismatch")
            require(params.get("deliveryKind") == "desktop-continuous", "subscribe delivery drifted")
            pending[id_key(frame.get("id"))] = "subscribe"
            continue

        if method == "session/send":
            require(subscribed and not send_requested, "send arrived before subscribe or twice")
            require(isinstance(params, dict), "send params are not an object")
            require(params.get("sessionId") == session_id, "send session mismatch")
            require(params.get("content") == "<redacted-prompt>", "send prompt is not redacted")
            pending[id_key(frame.get("id"))] = "send"
            send_requested = True
            continue

        if method != "session/event":
            require("id" not in frame, f"unsupported server request: {method}")
            continue

        require("id" not in frame, "session/event has an id")
        require(send_accepted, "session/event arrived before send acceptance")
        require(isinstance(params, dict), "session/event params are not an object")
        require(params.get("sessionId") == session_id, "event session mismatch")
        event_type = require_string(params.get("type"), "event type")
        event_sequence = params.get("seq")
        require(isinstance(event_sequence, int) and not isinstance(event_sequence, bool), "event seq is invalid")
        require(event_sequence == sequence + 1, "event sequence is not contiguous")
        sequence = event_sequence
        require(terminal is None, "event arrived after terminal")
        payload = params.get("payload")
        require(isinstance(payload, dict), f"{event_type} payload is not an object")

        if event_type not in known_events:
            if event_type.startswith(namespace):
                unknown_observations += 1
            continue

        if event_type == "turn.started":
            continue
        if event_type == "model.streaming":
            kind = require_string(payload.get("kind"), "streaming kind")
            if kind == "reasoning_delta":
                require(payload.get("delta") == "", "reasoning body was retained")
            elif kind == "text_delta":
                require(isinstance(payload.get("delta"), str), "text delta is not a string")
                text_deltas.append(payload["delta"])
            elif kind == "tool_call":
                require_string(payload.get("toolName") or payload.get("name") or "tool", "tool_call name")
            else:
                fail(f"unsupported streaming kind: {kind}")
        elif event_type == "tool.updated":
            kind = require_string(payload.get("kind"), "tool kind")
            call_id = require_string(payload.get("toolCallId"), "toolCallId")
            require_string(payload.get("toolName"), "toolName")
            if kind == "scheduled":
                require(call_id not in tool_calls, "duplicate tool call id")
                require(payload.get("input") == {}, "tool input was retained")
                tool_calls.add(call_id)
                tool_record_count += 1
            else:
                require(call_id in tool_calls, "tool update has no admitted call")
                if kind in {"result", "error"}:
                    require(call_id not in tool_finished, "duplicate tool terminal")
                    if kind == "error":
                        require_safe_error(payload.get("error"), "tool error")
                    tool_finished.add(call_id)
                    tool_record_count += 1
                elif kind not in {"started", "progress", "batch"}:
                    fail(f"unsupported tool kind: {kind}")
        elif event_type == "turn.completed":
            require(payload.get("resultType") == "success", "completed resultType drifted")
            usage = payload.get("usage")
            require(isinstance(usage, dict), "completed usage is not an object")
            for token_key in ("inputTokens", "outputTokens", "totalTokens"):
                require(isinstance(usage.get(token_key), int), f"usage.{token_key} is invalid")
                require(usage[token_key] >= 0, f"usage.{token_key} is negative")
            usage_count += 1
            terminal = "completed"
        elif event_type == "turn.failed":
            require_safe_error(payload.get("error"), "turn error")
            terminal = "error"
        elif event_type in {"session.updated", "turn.terminal"}:
            continue
        else:
            fail(f"known event not handled: {event_type}")

    require(subscribed and send_requested and send_accepted, "subscribe/send handshake is incomplete")
    require(not pending, "protocol requests remain unresolved")
    require(terminal == expected_terminal, f"terminal status was {terminal!r}, expected {expected_terminal!r}")
    require(tool_finished.issubset(tool_calls), "tool result correlation is incomplete")
    if terminal == "completed":
        require(usage_count > 0, "completed run has no usage")
        require(text_deltas, "completed run has no assistant text")
    require(tool_record_count <= bounds["maximum_tool_records"], "tool records exceed bound")
    require(
        unknown_observations > 0 if expect_unknown else unknown_observations == 0,
        "unexpected unknown-event observation count",
    )
    return {
        "terminal": terminal,
        "unknown_observations": unknown_observations,
        "usage_chunks": usage_count,
        "tool_calls": len(tool_calls),
        "text": "".join(text_deltas),
    }


def load_frames(name: str, protocol: dict[str, Any]) -> list[dict[str, Any]]:
    return decode_frames((FIXTURE_ROOT / name).read_bytes(), protocol["bounds"])


class CorpusTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.protocol = load_protocol()
        cls.artifact = load_json(FIXTURE_ROOT / "artifact.json")

    def test_artifact_identity_is_exact_and_qualified_only(self) -> None:
        self.assertEqual(self.artifact["published_identity"], "0.16.3")
        self.assertEqual(self.artifact["version_axis"], "zcode.runtime")
        self.assertEqual(self.artifact["qualification"], "qualified-only")
        self.assertFalse(self.artifact["unverified_newer"])
        self.assertEqual(self.artifact["executable"]["sha256"], EXPECTED_PAYLOAD_SHA256)
        self.assertEqual(self.artifact["launcher"]["sha256"], EXPECTED_LAUNCHER_SHA256)
        self.assertFalse(self.artifact["launcher"]["is_compatibility_axis"])
        self.assertFalse(self.artifact["desktop"]["is_compatibility_axis"])
        self.assertFalse(self.artifact["probe_package"]["is_compatibility_axis"])
        self.assertEqual(self.artifact["spawn"]["kind"], "interpreted-script")
        self.assertEqual(self.artifact["spawn"]["args"], ["app-server"])
        self.assertEqual((FIXTURE_ROOT / "version.txt").read_text(encoding="utf-8").strip(), "0.16.3")

    def test_protocol_records_handshake_and_snapshot_split(self) -> None:
        self.assertEqual(self.protocol["route_id"], "zcode.app-server")
        self.assertEqual(self.protocol["compatibility_revision"], "zcode.app-server-0.16.3-1")
        self.assertEqual(self.protocol["protocol_facade_revision"], "zcode.protocol-stdio-v1")
        self.assertFalse(self.protocol["transport"]["jsonrpc_field"])
        self.assertTrue(self.protocol["rules"]["create_blocks_on_runtime_preferences"])
        self.assertEqual(self.protocol["rules"]["cancel"], "force-stop-owned-process")
        cardinality = self.protocol["stream_cardinality"]
        self.assertEqual(cardinality["handshake_live_messages"], 4)
        self.assertEqual(cardinality["prompt_lifecycle"], "reconstructed-from-protocol")

    def test_handshake_fixture_is_create_complete(self) -> None:
        frames = load_frames("handshake.jsonl", self.protocol)
        validate_redaction(frames)
        result = validate_stream(
            frames,
            self.protocol,
            self.artifact,
            expected_terminal="completed",
            handshake_only=True,
        )
        self.assertIsNone(result["terminal"])

    def test_all_redacted_lifecycle_fixtures(self) -> None:
        for name, (terminal, expect_unknown) in RUN_FIXTURES.items():
            with self.subTest(fixture=name):
                frames = load_frames(name, self.protocol)
                validate_redaction(frames)
                result = validate_stream(
                    frames,
                    self.protocol,
                    self.artifact,
                    expected_terminal=terminal,
                    expect_unknown=expect_unknown,
                )
                self.assertEqual(result["terminal"], terminal)

    def test_create_without_preferences_reply_is_rejected(self) -> None:
        frames = load_frames("create-without-preferences.jsonl", self.protocol)
        validate_redaction(frames)
        with self.assertRaises(CorpusError):
            validate_stream(
                frames,
                self.protocol,
                self.artifact,
                expected_terminal="completed",
                handshake_only=True,
            )

    def test_negative_cases_are_declared(self) -> None:
        negative = load_json(FIXTURE_ROOT / "negative-cases.json")
        names = {case["name"] for case in negative["cases"]}
        self.assertTrue(
            {
                "malformed_json",
                "jsonrpc_field_present",
                "oversized_frame",
                "post_terminal_event",
                "mismatched_runtime",
                "unknown_unscoped_event",
                "create_without_preferences_reply",
            }.issubset(names)
        )

    def assert_rejected(self, frames: list[dict[str, Any]]) -> None:
        with self.assertRaises(CorpusError):
            validate_stream(
                frames,
                self.protocol,
                self.artifact,
                expected_terminal="completed",
            )

    def test_malformed_frame_is_rejected(self) -> None:
        valid = (FIXTURE_ROOT / "text-success.jsonl").read_bytes()
        with self.assertRaises(CorpusError):
            decode_frames(valid + b"not-json\n", self.protocol["bounds"])

    def test_jsonrpc_field_is_rejected(self) -> None:
        frames = load_frames("text-success.jsonl", self.protocol)
        frames[0]["jsonrpc"] = "2.0"
        with self.assertRaises(CorpusError):
            decode_frames(frame_bytes(frames), self.protocol["bounds"])

    def test_oversized_and_overbound_streams_are_rejected(self) -> None:
        bounds = self.protocol["bounds"]
        oversized = b"{" + b"x" * bounds["maximum_frame_bytes"] + b"}\n"
        with self.assertRaises(CorpusError):
            decode_frames(oversized, bounds)
        valid_frame = {
            "method": "session/event",
            "params": {
                "sessionId": "fixture-session-1",
                "seq": 1,
                "type": "turn.started",
                "payload": {},
            },
        }
        overbound = frame_bytes([valid_frame] * (bounds["maximum_live_notifications"] + 1))
        with self.assertRaises(CorpusError):
            decode_frames(overbound, bounds)

    def test_post_terminal_event_is_rejected(self) -> None:
        frames = load_frames("text-success.jsonl", self.protocol)
        terminal_index = next(
            index
            for index, frame in enumerate(frames)
            if frame.get("method") == "session/event"
            and frame["params"]["type"] == "turn.completed"
        )
        frames.insert(
            terminal_index + 1,
            {
                "method": "session/event",
                "params": {
                    "sessionId": "fixture-session-1",
                    "seq": 5,
                    "type": "zcode/future.after-terminal",
                    "payload": {},
                },
            },
        )
        self.assert_rejected(frames)

    def test_mismatched_runtime_and_session_are_rejected(self) -> None:
        runtime_frames = load_frames("text-success.jsonl", self.protocol)
        runtime_frames[3]["result"]["runtime"]["cliVersion"] = "9.9.9"
        self.assert_rejected(runtime_frames)

        session_frames = load_frames("text-success.jsonl", self.protocol)
        event = next(
            frame
            for frame in session_frames
            if frame.get("method") == "session/event"
        )
        event["params"]["sessionId"] = "other-session"
        self.assert_rejected(session_frames)

    def test_unscoped_unknown_event_is_content_free_progress(self) -> None:
        frames = load_frames("unknown-event.jsonl", self.protocol)
        unknown_frame = next(
            frame
            for frame in frames
            if frame.get("method") == "session/event"
            and frame["params"]["type"] == "zcode/future.notice"
        )
        unknown_frame["params"]["type"] = "session.titleUpdated"
        result = validate_stream(
            frames,
            self.protocol,
            self.artifact,
            expected_terminal="completed",
        )
        self.assertEqual(result["terminal"], "completed")
        self.assertEqual(result["unknown_observations"], 0)

    def test_tool_result_without_call_is_rejected(self) -> None:
        frames = load_frames("tool-success.jsonl", self.protocol)
        result_frame = next(
            frame
            for frame in frames
            if frame.get("method") == "session/event"
            and frame["params"]["type"] == "tool.updated"
            and frame["params"]["payload"].get("kind") == "result"
        )
        result_frame["params"]["payload"]["toolCallId"] = "not-admitted"
        self.assert_rejected(frames)


if __name__ == "__main__":
    unittest.main(argv=[sys.argv[0], *sys.argv[1:]])
