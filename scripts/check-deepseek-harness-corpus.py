#!/usr/bin/env python3
"""Validate the redacted DeepSeek Harness JSON-RPC fixture corpus.

This validator intentionally has no package or network dependency. It freezes
the low-level evidence needed by the Rust driver before that package exists.
It validates JSON-RPC framing, the admitted lifecycle, correlation, redaction,
and stream bounds; it does not attempt to replay a provider conversation.
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
    / "swallowtail-adapter-deepseek-harness"
    / "tests"
    / "fixtures"
    / "deepseek-harness-runtime-bin-0.1.0rc6"
)

EXPECTED_FIXTURES = {
    "text-success.jsonl": ("completed", False),
    "tool-success.jsonl": ("completed", False),
    "tool-error.jsonl": ("completed", False),
    "missing-key.jsonl": ("error", False),
    "unknown-event.jsonl": ("completed", True),
}

EXPECTED_ARTIFACT_SHA256 = (
    "ac1c91462518427467bd0a0ca3bf1049df62be0dbe8b0ee8014c6761cb8f80bf"
)
EXPECTED_SPAWN_HELPER_SHA256 = (
    "21c589109bca43e287df884f3c34ab888033a83927ea7d273949ac5030583f26"
)


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


def decode_frames(data: bytes, bounds: dict[str, int]) -> list[dict[str, Any]]:
    """Decode bounded NDJSON frames without tolerating blank or partial lines."""

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
        require(
            not raw_line.endswith(b"\r"),
            f"CRLF frame at line {line_number}",
        )
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
        require(
            frame.get("jsonrpc") == "2.0",
            f"frame {line_number} is not JSON-RPC 2.0",
        )
        has_method = "method" in frame
        has_result = "result" in frame
        has_error = "error" in frame
        require(
            has_method ^ (has_result or has_error),
            f"frame {line_number} has invalid JSON-RPC shape",
        )
        if has_method:
            require(
                isinstance(frame["method"], str) and frame["method"],
                f"frame {line_number} has invalid method",
            )
        else:
            require(
                "id" in frame and frame["id"] is not None,
                f"frame {line_number} response has no id",
            )
            require(
                not isinstance(frame["id"], (dict, list, bool)),
                f"frame {line_number} has invalid id",
            )
        frames.append(frame)
    require(frames, "stream contains no frames")
    return frames


def frame_bytes(frames: Iterable[dict[str, Any]]) -> bytes:
    return b"".join(
        json.dumps(frame, sort_keys=True, separators=(",", ":")).encode("utf-8")
        + b"\n"
        for frame in frames
    )


def id_key(value: Any) -> tuple[type[Any], Any]:
    require(
        isinstance(value, (str, int, float)) and not isinstance(value, bool),
        "JSON-RPC id must be a scalar string or number",
    )
    return (type(value), value)


def require_string(value: Any, label: str) -> str:
    require(isinstance(value, str) and value, f"{label} must be a non-empty string")
    return value


def require_safe_error(error: Any, label: str) -> None:
    require(isinstance(error, dict), f"{label} must be an object")
    require(
        set(error).issubset({"code", "name"}),
        f"{label} contains an unbounded error field",
    )
    require("code" in error or "name" in error, f"{label} has no safe code")
    for key, value in error.items():
        require_string(value, f"{label}.{key}")


def validate_text_content(content: Any, label: str, *, prompt: bool = False) -> str:
    require(isinstance(content, list), f"{label} content must be a list")
    output: list[str] = []
    for index, block in enumerate(content):
        require(isinstance(block, dict), f"{label}[{index}] must be an object")
        require(block.get("type") == "text", f"{label}[{index}] is not text")
        text = block.get("text")
        require(isinstance(text, str), f"{label}[{index}] text is not a string")
        if prompt:
            require(
                text == "<redacted-prompt>",
                f"{label}[{index}] contains a prompt body",
            )
        output.append(text)
    return "".join(output)


def validate_stream(
    frames: list[dict[str, Any]],
    protocol: dict[str, Any],
    *,
    expected_terminal: str,
    expect_unknown: bool = False,
) -> dict[str, Any]:
    """Validate one JSON-RPC run and return only safe projection counters."""

    bounds = protocol["bounds"]
    known_events = set(protocol["known_event_types"])
    namespace = protocol["rules"]["unknown_namespace"]
    artifact = load_json(FIXTURE_ROOT / "artifact.json")

    pending: dict[tuple[type[Any], Any], str] = {}
    initialized = False
    prompt_requested = False
    prompt_receipt = False
    shutdown_requested = False
    shutdown_responded = False
    provider: str | None = None
    model: str | None = None
    session_id: str | None = None
    status_running = False
    status_idle = False
    terminal: str | None = None
    turn_id: str | None = None
    current_step: str | None = None
    step_ended = True
    sequence = 0
    unknown_observations = 0
    tool_calls: set[str] = set()
    tool_results: set[str] = set()
    tool_record_count = 0
    text_deltas: list[str] = []
    assistant_messages: list[str] = []
    usage_count = 0
    usage_by_step: set[str] = set()
    finish_by_step: set[str] = set()
    active_step_usage = False

    for frame_number, frame in enumerate(frames, start=1):
        if "method" not in frame:
            response_id = id_key(frame["id"])
            operation = pending.pop(response_id, None)
            require(operation is not None, f"frame {frame_number} has unmatched response")
            if "error" in frame:
                require_safe_error(frame["error"], f"frame {frame_number}.error")
                fail(f"frame {frame_number} returned an unapproved JSON-RPC error")
            result = frame.get("result")
            if operation == "initialize":
                require(not initialized, "duplicate initialize response")
                require(isinstance(result, dict), "initialize result is not an object")
                server_info = result.get("serverInfo")
                require(
                    isinstance(server_info, dict)
                    and server_info.get("name") == artifact["server_info"]["name"],
                    "initialize server name does not match the expected runtime",
                )
                require(
                    isinstance(server_info.get("version"), str)
                    and bool(server_info["version"]),
                    "initialize server version is malformed",
                )
                initialized = True
            elif operation == "prompt":
                require(isinstance(result, dict), "prompt result is not an object")
                require(
                    require_string(result.get("messageId"), "prompt messageId")
                    .startswith("fixture-message-"),
                    "prompt result is not an enqueue receipt",
                )
                prompt_receipt = True
            elif operation == "shutdown":
                require(shutdown_requested, "unexpected shutdown response")
                require(result == {}, "shutdown result must be empty")
                shutdown_responded = True
            else:
                fail(f"unknown pending operation: {operation}")
            continue

        method = frame["method"]
        params = frame.get("params")
        if method == "initialize":
            require(not initialized and not pending, "initialize is not first")
            require(isinstance(params, dict), "initialize params are not an object")
            require(params.get("cwd") == "<fixture-cwd>", "cwd is not redacted")
            provider = require_string(params.get("provider"), "provider")
            model = require_string(params.get("model"), "model")
            request_id = id_key(frame.get("id"))
            require(request_id not in pending, "duplicate initialize id")
            pending[request_id] = "initialize"
            continue

        require(initialized, f"{method} arrived before initialize")
        if method == "session/prompt":
            require(not prompt_requested, "duplicate session/prompt")
            require(isinstance(params, dict), "session/prompt params are not an object")
            session_id = require_string(params.get("sessionId"), "sessionId")
            blocks = params.get("contentBlocks")
            validate_text_content(blocks, "session/prompt.contentBlocks", prompt=True)
            request_id = id_key(frame.get("id"))
            pending[request_id] = "prompt"
            prompt_requested = True
            continue

        if method == "shutdown":
            require(prompt_receipt and status_idle and terminal is not None, "shutdown before terminal idle")
            require(not shutdown_requested, "duplicate shutdown")
            require(isinstance(params, dict) and not params, "shutdown params must be empty")
            request_id = id_key(frame.get("id"))
            pending[request_id] = "shutdown"
            shutdown_requested = True
            continue

        require(method in {"session.status", "session.event"}, f"unsupported notification: {method}")
        require("id" not in frame, f"notification {method} has an id")
        require(isinstance(params, dict), f"{method} params are not an object")
        require(
            prompt_receipt,
            f"{method} arrived before the prompt enqueue receipt",
        )
        require(
            params.get("sessionId") == session_id,
            f"{method} session id does not match the admitted operation",
        )

        if method == "session.status":
            value = params.get("status")
            require(value in {"running", "idle"}, "invalid session status")
            if value == "running":
                require(not status_running and not status_idle, "duplicate or late running status")
                status_running = True
            else:
                require(status_running and not status_idle, "idle arrived before running or twice")
                require(terminal is not None, "idle arrived before terminal event")
                status_idle = True
            continue

        event = params.get("event")
        require(isinstance(event, dict), "session.event payload is not an object")
        event_type = require_string(event.get("type"), "event type")
        event_sequence = event.get("seq")
        require(isinstance(event_sequence, int) and not isinstance(event_sequence, bool), "event seq is invalid")
        require(event_sequence == sequence + 1, "event sequence is not contiguous")
        sequence = event_sequence
        require(isinstance(event.get("time"), (int, float)), "event time is invalid")
        require(terminal is None, "event arrived after terminal turn/end")

        if event_type not in known_events:
            require(event_type.startswith(namespace), "unknown event is not namespaced")
            require(event.get("ignorable") is True, "unknown event is not explicitly ignorable")
            unknown_observations += 1
            continue

        data = event.get("data")
        require(isinstance(data, dict), f"{event_type} data is not an object")
        if event_type == "turn/start":
            require(turn_id is None, "duplicate turn/start")
            turn_id = require_string(data.get("turn"), "turn id")
            continue

        require(turn_id is not None, f"{event_type} arrived before turn/start")
        if event_type != "turn/start":
            require(data.get("turn") == turn_id, f"{event_type} turn correlation failed")

        if event_type == "step/start":
            require(current_step is None and step_ended, "step/start overlaps an active step")
            current_step = require_string(data.get("step"), "step id")
            step_ended = False
            active_step_usage = False
            continue

        if event_type in {"user/message", "request/header", "request/context", "assistant/chunk", "assistant/message", "tool/call", "tool/result", "step/end"}:
            require(current_step is not None, f"{event_type} arrived outside a step")
            require(data.get("step") == current_step, f"{event_type} step correlation failed")

        if event_type == "user/message":
            message = data.get("message")
            require(isinstance(message, dict), "user message is not an object")
            require(message.get("role") == "user", "user message role is invalid")
            validate_text_content(message.get("content"), "user.message", prompt=True)
        elif event_type in {"request/header", "request/context"}:
            if "provider" in data:
                require(data["provider"] == provider, f"{event_type} provider mismatch")
            if "model" in data:
                require(data["model"] == model, f"{event_type} model mismatch")
            if event_type == "request/context":
                require(data.get("system") == "<redacted-system-prompt>", "system context is not redacted")
                require(isinstance(data.get("tools"), list), "tool composition is not explicit")
        elif event_type == "assistant/chunk":
            chunk = data.get("chunk")
            require(isinstance(chunk, dict), "assistant chunk is not an object")
            chunk_type = require_string(chunk.get("type"), "assistant chunk type")
            if chunk_type == "reasoning-delta":
                require(chunk.get("delta") == "", "reasoning body was retained")
            elif chunk_type == "text-delta":
                require(not chunk.get("delta") is None, "text delta is missing")
                require(isinstance(chunk.get("delta"), str), "text delta is not a string")
                text_deltas.append(chunk["delta"])
            elif chunk_type == "usage":
                require(not finish_by_step.__contains__(current_step), "usage arrived after finish")
                usage = chunk.get("usage")
                require(isinstance(usage, dict), "usage chunk is not an object")
                for token_key in ("inputTokens", "outputTokens"):
                    require(isinstance(usage.get(token_key), int), f"usage.{token_key} is invalid")
                    require(usage[token_key] >= 0, f"usage.{token_key} is negative")
                usage_count += 1
                usage_by_step.add(current_step)
                active_step_usage = True
            elif chunk_type == "finish":
                require(chunk.get("finishReason") in {"stop", "tool-calls"}, "unsupported finish reason")
                finish_by_step.add(current_step)
            elif chunk_type == "block-start":
                block = chunk.get("contentBlock")
                require(isinstance(block, dict), "block-start content block is missing")
                require(block.get("type") in {"reasoning", "text"}, "unsupported content block")
            elif chunk_type == "block-end":
                require(isinstance(chunk.get("contentIndex"), int), "block-end content index is invalid")
            else:
                fail(f"unsupported assistant chunk type: {chunk_type}")
        elif event_type == "assistant/message":
            message = data.get("message")
            require(isinstance(message, dict), "assistant message is not an object")
            require(message.get("role") == "assistant", "assistant message role is invalid")
            assistant_messages.append(validate_text_content(message.get("content"), "assistant.message"))
            if "usage" in data:
                require(isinstance(data["usage"], dict), "assistant message usage is invalid")
        elif event_type == "tool/call":
            call_id = require_string(data.get("callId"), "tool call id")
            require(call_id not in tool_calls, "duplicate tool call id")
            require_string(data.get("name"), "tool name")
            require(data.get("arguments") == "", "tool arguments were retained")
            tool_calls.add(call_id)
            tool_record_count += 1
        elif event_type == "tool/result":
            call_id = require_string(data.get("callId"), "tool result call id")
            require(call_id in tool_calls, "tool result has no admitted call")
            require(call_id not in tool_results, "duplicate tool result")
            message = data.get("message")
            require(isinstance(message, dict), "tool result message is not an object")
            require(message.get("role") == "tool", "tool result role is invalid")
            require(message.get("content") == [], "tool result body was retained")
            if "error" in data:
                require_safe_error(data["error"], "tool result error")
            tool_results.add(call_id)
            tool_record_count += 1
        elif event_type == "step/end":
            require(data.get("status") in {"stop", "tool-calls", "error"}, "invalid step status")
            require(not step_ended, "duplicate step/end")
            step_ended = True
            current_step = None
        elif event_type == "turn/end":
            require(current_step is None and step_ended, "turn/end arrived inside a step")
            require(data.get("status") in {"completed", "error"}, "invalid terminal status")
            terminal = data["status"]
            if terminal == "error":
                require_safe_error(data.get("error"), "turn error")
        else:
            fail(f"known event not handled: {event_type}")

    require(initialized, "initialize did not complete")
    require(prompt_requested and prompt_receipt, "prompt enqueue receipt is missing")
    require(status_running and status_idle, "running/idle status pair is incomplete")
    require(terminal == expected_terminal, f"terminal status was {terminal!r}, expected {expected_terminal!r}")
    require(shutdown_requested and shutdown_responded, "shutdown handshake is incomplete")
    require(not pending, "JSON-RPC requests remain unresolved")
    require(tool_results.issubset(tool_calls), "tool result correlation is incomplete")
    if terminal == "completed":
        require(usage_count > 0, "completed run has no usage")
    if text_deltas and assistant_messages:
        require(
            "".join(text_deltas) == assistant_messages[-1],
            "assistant text does not match streamed deltas",
        )
    require(tool_record_count <= bounds["maximum_tool_records"], "tool records exceed bound")
    require(unknown_observations > 0 if expect_unknown else unknown_observations == 0, "unexpected unknown-event observation count")
    return {
        "terminal": terminal,
        "unknown_observations": unknown_observations,
        "usage_chunks": usage_count,
        "tool_calls": len(tool_calls),
        "reasoning_progress": sum(
            1
            for frame in frames
            if frame.get("method") == "session.event"
            and frame.get("params", {}).get("event", {}).get("data", {}).get("chunk", {}).get("type")
            == "reasoning-delta"
        ),
    }


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
            "fixture contains credential or private-path material",
        )


def load_frames(name: str, protocol: dict[str, Any]) -> list[dict[str, Any]]:
    path = FIXTURE_ROOT / name
    return decode_frames(path.read_bytes(), protocol["bounds"])


class CorpusTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.protocol = load_protocol()
        cls.artifact = load_json(FIXTURE_ROOT / "artifact.json")

    def test_artifact_identity_is_exact_and_qualified_only(self) -> None:
        self.assertEqual(self.artifact["published_identity"], "0.1.0rc6")
        self.assertEqual(self.artifact["version_axis"], "deepseek-harness.runtime-bin")
        self.assertEqual(self.artifact["qualification"], "qualified-only")
        self.assertFalse(self.artifact["unverified_newer"])
        self.assertEqual(self.artifact["executable"]["sha256"], EXPECTED_ARTIFACT_SHA256)
        self.assertEqual(self.artifact["spawn_helper"]["sha256"], EXPECTED_SPAWN_HELPER_SHA256)
        self.assertFalse(self.artifact["server_info"]["version_is_compatibility_axis"])

    def test_protocol_records_bounds_and_cardinality_split(self) -> None:
        self.assertEqual(self.protocol["route_id"], "deepseek-harness.jsonrpc")
        self.assertEqual(self.protocol["compatibility_revision"], "deepseek-harness.jsonrpc-rc6-1")
        self.assertEqual(self.protocol["protocol_facade_revision"], "deepseek-harness.sdk-jsonrpc-v1")
        cardinality = self.protocol["stream_cardinality"]
        self.assertEqual(cardinality["live_probe_events"], 4626)
        self.assertEqual(cardinality["durable_jsonl_records"], 668)
        self.assertGreater(cardinality["live_probe_events"], cardinality["durable_jsonl_records"])
        self.assertGreaterEqual(
            self.protocol["bounds"]["maximum_live_notifications"],
            cardinality["live_probe_events"],
        )

    def test_all_redacted_lifecycle_fixtures(self) -> None:
        for name, (terminal, expect_unknown) in EXPECTED_FIXTURES.items():
            with self.subTest(fixture=name):
                frames = load_frames(name, self.protocol)
                validate_redaction(frames)
                result = validate_stream(
                    frames,
                    self.protocol,
                    expected_terminal=terminal,
                    expect_unknown=expect_unknown,
                )
                self.assertEqual(result["terminal"], terminal)

    def test_server_info_version_is_wire_metadata(self) -> None:
        frames = load_frames("text-success.jsonl", self.protocol)
        initialize = next(
            frame
            for frame in frames
            if frame.get("id") == 1 and "result" in frame
        )
        initialize["result"]["serverInfo"]["version"] = "9.9.9"
        result = validate_stream(
            frames,
            self.protocol,
            expected_terminal="completed",
        )
        self.assertEqual(result["terminal"], "completed")

    def test_negative_cases_are_declared(self) -> None:
        negative = load_json(FIXTURE_ROOT / "negative-cases.json")
        names = {case["name"] for case in negative["cases"]}
        self.assertTrue(
            {
                "malformed_json",
                "oversized_frame",
                "post_terminal_event",
                "mismatched_model",
                "unknown_unscoped_event",
            }.issubset(names)
        )

    def assert_rejected(self, frames: list[dict[str, Any]]) -> None:
        with self.assertRaises(CorpusError):
            validate_stream(
                frames,
                self.protocol,
                expected_terminal="completed",
            )

    def test_malformed_frame_is_rejected(self) -> None:
        valid = (FIXTURE_ROOT / "text-success.jsonl").read_bytes()
        with self.assertRaises(CorpusError):
            decode_frames(valid + b"not-json\n", self.protocol["bounds"])

    def test_oversized_and_overbound_streams_are_rejected(self) -> None:
        bounds = self.protocol["bounds"]
        oversized = b"{" + b"x" * bounds["maximum_frame_bytes"] + b"}\n"
        with self.assertRaises(CorpusError):
            decode_frames(oversized, bounds)
        valid_frame = {
            "jsonrpc": "2.0",
            "method": "session.status",
            "params": {"sessionId": "fixture-session", "status": "running"},
        }
        overbound = frame_bytes([valid_frame] * (bounds["maximum_live_notifications"] + 1))
        with self.assertRaises(CorpusError):
            decode_frames(overbound, bounds)

    def test_post_terminal_event_is_rejected(self) -> None:
        frames = load_frames("text-success.jsonl", self.protocol)
        terminal_index = next(
            index
            for index, frame in enumerate(frames)
            if frame.get("method") == "session.event"
            and frame["params"]["event"]["type"] == "turn/end"
        )
        inserted = {
            "jsonrpc": "2.0",
            "method": "session.event",
            "params": {
                "sessionId": "fixture-session-1",
                "event": {
                    "type": "deepseek-harness/future.after-terminal",
                    "seq": 17,
                    "time": 1700000017,
                    "data": {},
                    "ignorable": True,
                },
            },
        }
        frames.insert(terminal_index + 1, inserted)
        self.assert_rejected(frames)

    def test_mismatched_model_and_session_are_rejected(self) -> None:
        model_frames = load_frames("text-success.jsonl", self.protocol)
        model_frame = next(
            frame
            for frame in model_frames
            if frame.get("method") == "session.event"
            and frame["params"]["event"]["type"] == "request/header"
        )
        model_frame["params"]["event"]["data"]["model"] = "other-model"
        self.assert_rejected(model_frames)

        session_frames = load_frames("text-success.jsonl", self.protocol)
        status_frame = next(
            frame
            for frame in session_frames
            if frame.get("method") == "session.status"
        )
        status_frame["params"]["sessionId"] = "other-session"
        self.assert_rejected(session_frames)

    def test_unscoped_unknown_event_is_rejected(self) -> None:
        frames = load_frames("unknown-event.jsonl", self.protocol)
        unknown_frame = next(
            frame
            for frame in frames
            if frame.get("method") == "session.event"
            and frame["params"]["event"]["type"] == "deepseek-harness/future.notice"
        )
        unknown_frame["params"]["event"]["type"] = "future.notice"
        self.assert_rejected(frames)

    def test_idle_before_terminal_is_rejected(self) -> None:
        frames = load_frames("text-success.jsonl", self.protocol)
        status_index = next(
            index
            for index, frame in enumerate(frames)
            if frame.get("method") == "session.status"
            and frame["params"]["status"] == "running"
        )
        idle = copy.deepcopy(frames[status_index])
        idle["params"]["status"] = "idle"
        frames.insert(status_index + 1, idle)
        self.assert_rejected(frames)

    def test_usage_after_finish_is_rejected(self) -> None:
        frames = load_frames("text-success.jsonl", self.protocol)
        finish_index = next(
            index
            for index, frame in enumerate(frames)
            if frame.get("method") == "session.event"
            and frame["params"]["event"]["data"].get("chunk", {}).get("type") == "finish"
        )
        usage = copy.deepcopy(frames[finish_index - 1])
        usage["params"]["event"]["seq"] = 14
        usage["params"]["event"]["data"]["chunk"] = {
            "type": "usage",
            "usage": {"inputTokens": 1, "outputTokens": 1, "reasoningTokens": 0},
        }
        for index in range(finish_index + 1, len(frames)):
            event = frames[index].get("params", {}).get("event")
            if event is not None and isinstance(event.get("seq"), int):
                event["seq"] += 1
        frames.insert(finish_index + 1, usage)
        self.assert_rejected(frames)

    def test_tool_result_without_call_is_rejected(self) -> None:
        frames = load_frames("tool-success.jsonl", self.protocol)
        result_frame = next(
            frame
            for frame in frames
            if frame.get("method") == "session.event"
            and frame["params"]["event"]["type"] == "tool/result"
        )
        result_frame["params"]["event"]["data"]["callId"] = "not-admitted"
        self.assert_rejected(frames)


if __name__ == "__main__":
    unittest.main(argv=[sys.argv[0], *sys.argv[1:]])
