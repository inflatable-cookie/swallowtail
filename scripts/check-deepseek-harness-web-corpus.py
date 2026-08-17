#!/usr/bin/env python3
"""Validate the redacted DeepSeek Harness Web `/api` fixture corpus.

This validator has no package, network, or credential dependency. It freezes
the Web carrier boundary before the Rust driver exists: exact npm/CLI
identity, loopback trust, allowlisted unary methods, server-only event
downlinks, control-free history pages, bounds, correlation, and redaction.
"""

from __future__ import annotations

import copy
import json
import re
import unittest
from pathlib import Path
from typing import Any


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
FIXTURE_ROOT = (
    REPOSITORY_ROOT
    / "crates"
    / "swallowtail-adapter-deepseek-harness"
    / "tests"
    / "fixtures"
    / "deepseek-harness-web-0.1.0rc6"
)

EXPECTED_METHODS = {
    "session.list",
    "session.search",
    "session.create",
    "session.history",
    "session.models",
    "session.prompt",
    "session.cancel",
    "session.fork",
    "workspace.list",
    "workspace.archiveSession",
    "host.describe",
}
EXPECTED_MUX_FRAMES = {"session/subscribed", "session/event", "stream/error"}
EXPECTED_HOST_FRAMES = {
    "host/session-added",
    "host/session-removed",
    "host/session-status",
    "host/agent-error",
    "host/workspace-changed",
    "host/workspace-removed",
    "host/workspace-order-changed",
    "host/archived-sessions-changed",
    "host/remote-event",
    "stream/error",
}
SAFE_EVENT_TYPES = {
    "turn/start",
    "turn/end",
    "step/start",
    "step/end",
    "user/message",
    "assistant/chunk",
}
SENSITIVE_KEYS = {
    "access_token",
    "api_key",
    "authorization",
    "cookie",
    "credential",
    "export_bytes",
    "private_key",
    "password",
    "secret",
    "set_cookie",
    "tool_arguments",
    "tool_result",
}
PRIVATE_TEXT = re.compile(
    r"(?:/Users/|/home/|/private/|[A-Za-z]:\\Users\\|file://|Bearer\s+|"
    r"[A-Fa-f0-9]{8}-[A-Fa-f0-9]{4}-[1-8][A-Fa-f0-9]{3}-[89ABab][A-Fa-f0-9]{3}-[A-Fa-f0-9]{12}|"
    r"[\w.+-]+@[\w.-]+\.[A-Za-z]{2,})"
)


class CorpusError(ValueError):
    """A fixture violates a frozen Web boundary or redaction rule."""


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


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    try:
        data = path.read_bytes()
    except OSError as exc:
        fail(f"cannot read {path.name}: {exc}")
    require(data.endswith(b"\n"), f"{path.name} must end with a newline")
    frames: list[dict[str, Any]] = []
    for line_number, raw_line in enumerate(data.split(b"\n")[:-1], start=1):
        require(raw_line, f"blank frame at {path.name}:{line_number}")
        require(not raw_line.endswith(b"\r"), f"CRLF frame at {path.name}:{line_number}")
        try:
            frame = json.loads(
                raw_line.decode("utf-8"),
                parse_constant=reject_json_constant,
            )
        except (UnicodeDecodeError, json.JSONDecodeError, CorpusError) as exc:
            fail(f"malformed frame at {path.name}:{line_number}: {exc}")
        require(isinstance(frame, dict), f"frame at {path.name}:{line_number} is not an object")
        frames.append(frame)
    require(frames, f"{path.name} contains no frames")
    return frames


def require_string(value: Any, label: str) -> str:
    require(isinstance(value, str) and value, f"{label} must be a non-empty string")
    return value


def require_rpc_id(value: Any, label: str, maximum_bytes: int) -> str:
    rpc_id = require_string(value, label)
    require(len(rpc_id.encode("utf-8")) <= maximum_bytes, f"{label} exceeds the rpc id bound")
    require(rpc_id.startswith("fixture-"), f"{label} is not a sanitized fixture identity")
    return rpc_id


def inspect_redaction(value: Any, path: str = "root") -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            normalized = re.sub(r"[^a-z0-9]", "_", str(key).lower()).strip("_")
            require(
                path.endswith(".redaction") or normalized not in SENSITIVE_KEYS,
                f"sensitive field at {path}.{key}",
            )
            inspect_redaction(child, f"{path}.{key}")
        return
    if isinstance(value, list):
        for index, child in enumerate(value):
            inspect_redaction(child, f"{path}[{index}]")
        return
    if isinstance(value, str):
        require(not PRIVATE_TEXT.search(value), f"private or random identity text at {path}")
        if value.startswith("<"):
            require(value.startswith("<redacted-") or value.startswith("<fixture-"), f"unknown marker at {path}")


def validate_artifact(artifact: dict[str, Any]) -> None:
    require(artifact.get("fixture_schema") == 1, "artifact fixture schema drifted")
    require(artifact.get("route_id") == "deepseek-harness.local-server", "artifact route drifted")
    require(artifact.get("driver_id") == "swallowtail.deepseek-harness.local-server", "artifact driver drifted")
    require(artifact.get("artifact_revision") == "0.1.0-rc.6", "artifact version drifted")
    require(artifact.get("version_axis") == "deepseek-harness.web", "artifact axis drifted")
    require(artifact.get("qualification") == "qualified-only", "artifact qualification drifted")
    npm = artifact.get("npm")
    require(isinstance(npm, dict), "artifact npm identity is missing")
    require(npm == {"package": "@deepseek-ai/dsh", "version": "0.1.0-rc.6"}, "artifact npm identity drifted")
    cli = artifact.get("cli")
    require(isinstance(cli, dict), "artifact CLI identity is missing")
    require(cli.get("executable") == "dsh", "artifact executable drifted")
    require(cli.get("arguments") == ["web"], "artifact command drifted")
    require(cli.get("default_bind") == "127.0.0.1", "artifact bind drifted")
    require(cli.get("default_port") == 3080, "artifact port drifted")
    source = artifact.get("source_evidence")
    require(isinstance(source, dict), "artifact source evidence is missing")
    require(source.get("revision") == "47f943859bef60e4160492346772ded9b24f765a", "source evidence revision drifted")
    rules = artifact.get("compatibility_axis_rules")
    require(isinstance(rules, dict), "artifact axis rules are missing")
    require(all(value == "not-an-axis" for value in rules.values()), "artifact axis rule admits an unstable identity")
    startup = artifact.get("startup")
    require(startup == {"process_owned": True, "browser": False, "jsonrpc_binary": False, "network_bind": "loopback-only"}, "startup boundary drifted")


def validate_protocol(protocol: dict[str, Any]) -> None:
    require(protocol.get("fixture_schema") == 1, "protocol fixture schema drifted")
    require(protocol.get("route_id") == "deepseek-harness.local-server", "protocol route drifted")
    require(protocol.get("version_axis") == "deepseek-harness.web", "protocol axis drifted")
    require(protocol.get("qualification") == "qualified-only", "protocol qualification drifted")
    require(set(protocol.get("client_methods", [])) == EXPECTED_METHODS, "client method allowlist drifted")
    denied = set(protocol.get("denied_exact", []))
    require(not EXPECTED_METHODS & denied, "denied methods overlap the allowlist")
    require("session.export" in denied and "session.restore" in denied and "session.delete" in denied, "destructive methods are not denied")
    require(set(protocol.get("server_mux_frames", [])) == EXPECTED_MUX_FRAMES, "mux frame allowlist drifted")
    require(set(protocol.get("server_host_frames", [])) == EXPECTED_HOST_FRAMES, "host frame allowlist drifted")
    transport = protocol.get("transport")
    require(isinstance(transport, dict), "transport rules are missing")
    require(transport.get("command") == ["dsh", "web"], "transport command drifted")
    require(transport.get("http_method") == "POST", "HTTP method drifted")
    require(transport.get("websocket_paths") == ["/api/events.mux", "/api/events.host"], "WebSocket paths drifted")
    envelopes = protocol.get("envelopes")
    require(envelopes == ["client-request", "server-response", "server-request", "client-response"], "envelope set drifted")
    fence = protocol.get("trust_fence")
    require(isinstance(fence, dict), "trust fence is missing")
    require(fence.get("loopback_only") is True and fence.get("same_origin_if_present") is True, "trust fence weakened")
    require(fence.get("reject_fetch_site_cross_site") is True and fence.get("bearer_auth") is False, "browser trust fence weakened")
    rules = protocol.get("rules")
    require(isinstance(rules, dict), "protocol rules are missing")
    require(rules.get("path_method_must_agree") is True, "path/method agreement was removed")
    require(rules.get("business_errors_use_http_200") is True, "business/carrier status split was removed")
    require(rules.get("history_is_control_free") is True, "history control-free rule was removed")
    require(rules.get("native_cancel") is True and rules.get("native_fork") is True and rules.get("native_archive") is True, "native lifecycle rule drifted")
    require(rules.get("restore") is False and rules.get("hard_delete") is False, "unsupported lifecycle was admitted")
    bounds = protocol.get("bounds")
    require(isinstance(bounds, dict), "protocol bounds are missing")
    for key in ("maximum_http_body_bytes", "maximum_websocket_frame_bytes", "maximum_rpc_id_bytes", "maximum_history_entries", "maximum_live_events"):
        require(isinstance(bounds.get(key), int) and bounds[key] > 0, f"invalid bound {key}")
    require(bounds["maximum_history_entries"] <= 64, "history bound is too large")
    inspect_redaction(protocol)


def validate_request_envelope(envelope: dict[str, Any], protocol: dict[str, Any]) -> str:
    bounds = protocol["bounds"]
    require(envelope.get("type") == "client-request", "unary request is not a client-request")
    method = require_string(envelope.get("method"), "request method")
    require(method in EXPECTED_METHODS, f"request method is not allowlisted: {method}")
    require_rpc_id(envelope.get("rpcId"), "request rpcId", bounds["maximum_rpc_id_bytes"])
    require(isinstance(envelope.get("payload"), dict), "request payload is not an object")
    return method


def validate_response_envelope(envelope: dict[str, Any], protocol: dict[str, Any]) -> dict[str, Any]:
    require(envelope.get("type") == "server-response", "unary response is not a server-response")
    require_rpc_id(envelope.get("rpcId"), "response rpcId", protocol["bounds"]["maximum_rpc_id_bytes"])
    result = envelope.get("result")
    require(isinstance(result, dict), "response result is not an object")
    require(set(result) == {"ok", "value"} or set(result) == {"ok", "error"}, "response result envelope drifted")
    require(isinstance(result.get("ok"), bool), "response result ok is not boolean")
    if result["ok"]:
        require(isinstance(result.get("value"), dict), "successful response value is not an object")
    else:
        error = result.get("error")
        require(isinstance(error, dict), "business error is not an object")
        require(set(error) <= {"code", "message", "details"}, "business error exposes unbounded fields")
        require_string(error.get("code"), "business error code")
        require_string(error.get("message"), "business error message")
    return result


def validate_unary(protocol: dict[str, Any], *, frames: list[dict[str, Any]] | None = None) -> None:
    frames = load_jsonl(FIXTURE_ROOT / "unary.jsonl") if frames is None else frames
    require(len(frames) % 2 == 0, "unary corpus does not contain request/response pairs")
    seen: set[str] = set()
    for index in range(0, len(frames), 2):
        request = frames[index]
        response = frames[index + 1]
        require(request.get("kind") == "request", f"unary frame {index} is not a request record")
        require(request.get("method") == "POST", f"unary frame {index} is not POST")
        path = require_string(request.get("path"), f"unary frame {index} path")
        headers = request.get("headers")
        require(isinstance(headers, dict) and headers.get("content-type") == "application/json", f"unary frame {index} has no JSON content type")
        method = validate_request_envelope(request.get("body", {}), protocol)
        require(path == f"/api/{method}", f"unary frame {index} path does not match method")
        require(response.get("kind") == "response", f"unary frame {index + 1} is not a response record")
        require(response.get("path") == path and response.get("status") == 200, f"unary response {index + 1} carrier drifted")
        result = validate_response_envelope(response.get("body", {}), protocol)
        require(response["body"]["rpcId"] == request["body"]["rpcId"], f"unary frame {index} lost rpc correlation")
        require(method not in seen, f"duplicate unary method {method}")
        seen.add(method)
        payload = request["body"]["payload"]
        if method == "session.search":
            require(payload.get("query") == "<redacted-search-query>", "search query was not redacted")
        elif method == "session.prompt":
            require(payload.get("mode") in {"queue", "steer"}, "prompt mode drifted")
            content = payload.get("content")
            require(content == [{"type": "text", "text": "<redacted-prompt>"}], "prompt body was not redacted")
        elif method == "session.history":
            require(payload.get("maxMessages", 0) <= protocol["bounds"]["maximum_history_entries"], "history request exceeds bound")
            if result["ok"]:
                value = result["value"]
                require(len(value.get("events", [])) <= protocol["bounds"]["maximum_history_entries"], "history response exceeds bound")
        if method in {"session.prompt", "session.cancel"} and result["ok"]:
            require(result["value"].get("accepted") is True, f"{method} did not return native acceptance")
    require(seen == EXPECTED_METHODS, "unary corpus does not cover every allowlisted method")


def validate_event_frame(frame: dict[str, Any], protocol: dict[str, Any], allowed: set[str]) -> tuple[str, int | None]:
    bounds = protocol["bounds"]
    require(frame.get("type") == "server-request", "downlink sent a non-server request")
    method = require_string(frame.get("method"), "downlink method")
    require(method in allowed, f"downlink frame is not allowlisted: {method}")
    require_rpc_id(frame.get("rpcId"), "downlink rpcId", bounds["maximum_rpc_id_bytes"])
    payload = frame.get("payload")
    require(isinstance(payload, dict) and payload.get("type") == method, "downlink method/payload type mismatch")
    if method == "session/event":
        session_id = require_string(payload.get("sessionId"), "session event sessionId")
        event = payload.get("event")
        require(isinstance(event, dict), "session event is not an object")
        require(event.get("type") in SAFE_EVENT_TYPES, "session event type is not allowlisted")
        require(isinstance(event.get("seq"), int) and event["seq"] >= 0, "session event sequence is invalid")
        require(isinstance(event.get("time"), (int, float)), "session event time is invalid")
        require("data" in event, "session event data is missing")
        return session_id, event["seq"]
    if method == "session/subscribed":
        return require_string(payload.get("sessionId"), "subscription sessionId"), payload.get("lastSeq")
    if method == "stream/error":
        error = payload.get("error")
        require(isinstance(error, dict) and require_string(error.get("code"), "stream error code"), "stream error is not safe")
    return "", None


def validate_downlink(name: str, protocol: dict[str, Any]) -> None:
    frames = load_jsonl(FIXTURE_ROOT / name)
    allowed = EXPECTED_MUX_FRAMES if name == "mux.jsonl" else EXPECTED_HOST_FRAMES
    previous: dict[str, int] = {}
    require(len(frames) <= protocol["bounds"]["maximum_live_events"], f"{name} exceeds live event bound")
    for frame in frames:
        session_id, sequence = validate_event_frame(frame, protocol, allowed)
        if name == "mux.jsonl" and frame["method"] == "session/subscribed":
            require(frame["payload"].get("lastSeq") == -1, "subscription baseline is not control-free")
        if sequence is not None and session_id:
            require(session_id not in previous or sequence == previous[session_id] + 1, f"{name} sequence is not contiguous")
            previous[session_id] = sequence
    require(frames, f"{name} is empty")


def validate_history(protocol: dict[str, Any], *, history: dict[str, Any] | None = None) -> None:
    history = load_json(FIXTURE_ROOT / "history.json") if history is None else history
    bound = history.get("page_bound")
    require(isinstance(bound, int) and 0 < bound <= protocol["bounds"]["maximum_history_entries"], "history page bound drifted")
    require(history.get("session_id") == "fixture-session-1", "history session identity drifted")
    pages = history.get("pages")
    require(isinstance(pages, list) and pages, "history has no pages")
    previous_before: int | None = None
    for page_number, page in enumerate(pages, start=1):
        require(isinstance(page, dict), f"history page {page_number} is not an object")
        request = page.get("request")
        response = page.get("response")
        require(isinstance(request, dict) and isinstance(response, dict), f"history page {page_number} is incomplete")
        require(validate_request_envelope(request, protocol) == "session.history", f"history page {page_number} is not session.history")
        payload = request["payload"]
        require(payload.get("sessionId") == history["session_id"], f"history page {page_number} session drifted")
        max_messages = payload.get("maxMessages")
        require(isinstance(max_messages, int) and 0 < max_messages <= bound, f"history page {page_number} exceeds page bound")
        before_seq = payload.get("beforeSeq")
        if before_seq is not None:
            require(isinstance(before_seq, int) and before_seq >= 0, f"history page {page_number} beforeSeq is invalid")
            if previous_before is not None:
                require(before_seq < previous_before, f"history page {page_number} does not move toward older events")
            previous_before = before_seq
        result = validate_response_envelope(response, protocol)
        require(response["rpcId"] == request["rpcId"], f"history page {page_number} lost correlation")
        require(result["ok"] is True, f"history page {page_number} is not a successful inspection")
        value = result["value"]
        events = value.get("events")
        require(isinstance(events, list) and len(events) <= bound, f"history page {page_number} exceeds result bound")
        sequences: list[int] = []
        for entry in events:
            require(isinstance(entry, dict) and isinstance(entry.get("event"), dict), f"history page {page_number} has invalid entry")
            event = entry["event"]
            require(event.get("type") in SAFE_EVENT_TYPES, f"history page {page_number} has an unprojected event")
            require(isinstance(event.get("seq"), int) and event["seq"] >= 0, f"history page {page_number} sequence is invalid")
            sequences.append(event["seq"])
        require(sequences == sorted(sequences), f"history page {page_number} is not ascending")
        effects = page.get("effects")
        require(isinstance(effects, dict), f"history page {page_number} has no side-effect proof")
        require(effects.get("agent_started") is False, f"history page {page_number} resumed an agent")
        require(effects.get("prompt_dispatched") is False, f"history page {page_number} dispatched a prompt")
        require(effects.get("provider_work_invocations") == 0, f"history page {page_number} called provider work")
        require(effects.get("interactive_handle_created") is False, f"history page {page_number} created a control handle")
    proof = history.get("proof")
    require(proof == {"reads_persisted_log": True, "resumes_agent": False, "publishes_prompt": False, "provider_work_invocations": 0, "interactive_handle_created": False}, "history proof drifted")


def validate_negative_cases(protocol: dict[str, Any], *, cases: dict[str, Any] | None = None) -> None:
    cases = load_json(FIXTURE_ROOT / "negative-cases.json") if cases is None else cases
    for case in cases.get("denied_methods", []):
        method = require_string(case.get("method"), "denied method")
        require(method not in EXPECTED_METHODS, f"denied method entered allowlist: {method}")
        require(case.get("reason") == "not-in-allowlist", f"denied method {method} does not fail closed")
        require(case.get("provider_work_invocations") == 0, f"denied method {method} reached provider work")
    trust = cases.get("trust_fence", [])
    require(trust, "trust fence cases are missing")
    for case in trust:
        require(case.get("accepted") is False, f"trust case {case.get('name')} was accepted")
        require(case.get("provider_work_invocations") == 0, f"trust case {case.get('name')} reached provider work")
    identities = cases.get("identity_cases", [])
    require(identities, "identity cases are missing")
    for case in identities:
        require(case.get("accepted") is True, f"identity case {case.get('name')} was not correlated")
        if "response_correlation_id" in case:
            require(case["correlation_id"] == case["response_correlation_id"], f"identity case {case.get('name')} weakened correlation")
        if "list_session_id" in case:
            require(case["session_id"] == case["list_session_id"] == case["history_session_id"], f"identity case {case.get('name')} weakened session correlation")
    for case in cases.get("protocol_cases", []):
        require(case.get("accepted") is False, f"protocol case {case.get('name')} was accepted")
        require(case.get("provider_work_invocations") == 0, f"protocol case {case.get('name')} reached provider work")


def validate_malformed(protocol: dict[str, Any]) -> None:
    malformed = load_json(FIXTURE_ROOT / "malformed.json")
    expected = protocol["carrier_status"]
    for case in malformed.get("carrier_cases", []):
        name = require_string(case.get("name"), "carrier case name")
        status = case.get("expected_status")
        require(status == expected.get(name.replace("-", "_")) or (name == "method-path-mismatch" and status == expected["business_error"]), f"carrier case {name} status drifted")
        require(case.get("provider_work_invocations") == 0, f"carrier case {name} reached provider work")
    business = malformed.get("business_error_case")
    require(isinstance(business, dict) and business.get("http_status") == expected["business_error"], "business-error carrier split drifted")
    response = business.get("response")
    result = validate_response_envelope(response, protocol)
    require(result["ok"] is False, "business-error fixture became a transport failure")
    require(business.get("provider_work_invocations") == 0, "business-error fixture reached provider work")


def assert_rejected(test: unittest.TestCase, callback: Any, message: str) -> None:
    with test.assertRaises(CorpusError, msg=message):
        callback()


class DeepSeekHarnessWebCorpusTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.artifact = load_json(FIXTURE_ROOT / "artifact.json")
        cls.protocol = load_json(FIXTURE_ROOT / "protocol.json")

    def test_artifact_identity_is_pinned(self) -> None:
        validate_artifact(self.artifact)

    def test_protocol_allowlist_and_fences_are_frozen(self) -> None:
        validate_protocol(self.protocol)

    def test_unary_corpus_covers_every_allowlisted_method(self) -> None:
        validate_unary(self.protocol)

    def test_mux_downlink_is_bounded_and_server_only(self) -> None:
        validate_downlink("mux.jsonl", self.protocol)

    def test_host_downlink_is_bounded_and_server_only(self) -> None:
        validate_downlink("host.jsonl", self.protocol)

    def test_history_is_control_free_and_paged(self) -> None:
        validate_history(self.protocol)

    def test_malformed_carriers_keep_business_errors_at_http_200(self) -> None:
        validate_malformed(self.protocol)

    def test_denied_methods_and_trust_fences_fail_closed(self) -> None:
        validate_negative_cases(self.protocol)

    def test_corpus_is_redacted(self) -> None:
        for path in FIXTURE_ROOT.iterdir():
            if path.suffix == ".json":
                inspect_redaction(load_json(path), path.name)
            elif path.suffix == ".jsonl":
                for frame in load_jsonl(path):
                    inspect_redaction(frame, path.name)

    def test_mismatched_unary_correlation_is_rejected(self) -> None:
        frames = load_jsonl(FIXTURE_ROOT / "unary.jsonl")
        mutated = copy.deepcopy(frames)
        mutated[1]["body"]["rpcId"] = "fixture-rpc-wrong"
        assert_rejected(self, lambda: validate_unary(self.protocol, frames=mutated), "mismatched unary ids must fail closed")

    def test_unknown_unary_method_is_rejected(self) -> None:
        frames = load_jsonl(FIXTURE_ROOT / "unary.jsonl")
        mutated = copy.deepcopy(frames)
        mutated[0]["path"] = "/api/session.unknown"
        mutated[0]["body"]["method"] = "session.unknown"
        assert_rejected(self, lambda: validate_unary(self.protocol, frames=mutated), "unknown unary methods must fail closed")

    def test_history_side_effect_is_rejected(self) -> None:
        history = load_json(FIXTURE_ROOT / "history.json")
        mutated = copy.deepcopy(history)
        mutated["pages"][0]["effects"]["agent_started"] = True
        assert_rejected(self, lambda: validate_history(self.protocol, history=mutated), "history resume must fail closed")

    def test_non_loopback_trust_acceptance_is_rejected(self) -> None:
        cases = load_json(FIXTURE_ROOT / "negative-cases.json")
        mutated = copy.deepcopy(cases)
        mutated["trust_fence"][0]["accepted"] = True
        assert_rejected(self, lambda: validate_negative_cases(self.protocol, cases=mutated), "non-loopback acceptance must fail closed")

    def test_random_uuid_identity_is_rejected(self) -> None:
        cases = load_json(FIXTURE_ROOT / "negative-cases.json")
        mutated = copy.deepcopy(cases)
        mutated["identity_cases"][0]["upstream_identity"] = "01234567-89ab-4cde-8fab-0123456789ab"
        assert_rejected(self, lambda: inspect_redaction(mutated), "random upstream identities must be sanitized")

    def test_history_bound_mutation_is_rejected(self) -> None:
        history = load_json(FIXTURE_ROOT / "history.json")
        mutated = copy.deepcopy(history)
        mutated["pages"][0]["request"]["payload"]["maxMessages"] = self.protocol["bounds"]["maximum_history_entries"] + 1
        assert_rejected(self, lambda: validate_history(self.protocol, history=mutated), "history page bound must fail closed")


if __name__ == "__main__":
    unittest.main(verbosity=2)
