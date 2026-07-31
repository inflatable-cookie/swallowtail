use crate::support;

use support::{Direction, methods, parse_json, parse_transcript};
use swallowtail_protocol_acp::ACP_PROTOCOL_VERSION;

const PROTOCOL: &str =
    include_str!("fixtures/acp-v1-cursor-agent-2026.07.01-41b2de7/protocol.json");
const INITIALIZE: &str =
    include_str!("fixtures/acp-v1-cursor-agent-2026.07.01-41b2de7/initialize.ndjson");
const INTERACTIVE_SOURCE: &str =
    include_str!("fixtures/acp-v1-cursor-agent-2026.07.01-41b2de7/interactive-source.json");
const INTERACTIVE: &str =
    include_str!("fixtures/acp-v1-cursor-agent-2026.07.01-41b2de7/interactive-derived.ndjson");
const HEADLESS_SOURCE: &str =
    include_str!("fixtures/acp-v1-cursor-agent-2026.07.01-41b2de7/headless-source.json");
const HEADLESS: &str =
    include_str!("fixtures/acp-v1-cursor-agent-2026.07.01-41b2de7/headless-derived.jsonl");

#[test]
fn exact_cursor_artifacts_remain_separate_and_route_scoped() {
    let fixture = parse_json(PROTOCOL);

    assert_eq!(
        fixture["installed_artifact"]["version"],
        "2026.07.01-41b2de7"
    );
    assert_eq!(fixture["registry_artifact"]["version"], "2026.07.23");
    assert_eq!(fixture["registry_artifact"]["build"], "2026.07.23-e383d2b");
    assert_eq!(fixture["registry_artifact"]["executed"], false);
    assert_eq!(fixture["qualification"]["production_claim_created"], true);
    assert_eq!(
        fixture["qualification"]["production_claim_scope"],
        "installed ACP interactive and headless structured routes"
    );
    assert_eq!(
        fixture["qualification"]["continuous_calendar_range_allowed"],
        false
    );
    assert_eq!(
        fixture["qualification"]["installed_and_registry_artifacts_share_behavior"],
        "unproven"
    );
}

#[test]
fn discovery_records_the_generic_agent_collision_without_filename_rejection() {
    let fixture = parse_json(PROTOCOL);
    let discovery = &fixture["discovery"];

    assert_eq!(discovery["automatic_candidate"], "cursor-agent");
    assert_eq!(discovery["generic_agent_candidate_allowed"], false);
    assert_eq!(discovery["generic_agent_observed_identity"], "grok");
    assert_eq!(
        discovery["host_approved_path_requires_cursor_identity"],
        true
    );
    assert_eq!(discovery["silent_candidate_fallback"], false);
}

#[test]
fn installed_cursor_initialization_is_exact_and_read_only() {
    let frames = parse_transcript(INITIALIZE).expect("Cursor initialize transcript parses");
    assert_eq!(methods(&frames), ["initialize"]);
    assert_eq!(frames.len(), 2);
    assert_eq!(frames[0].direction(), Direction::ClientToAgent);
    assert_eq!(frames[1].direction(), Direction::AgentToClient);
    assert_eq!(frames[0].id(), frames[1].id());
    assert_eq!(
        frames[0].message()["params"]["protocolVersion"],
        ACP_PROTOCOL_VERSION
    );
    assert_eq!(
        frames[0].message()["params"]["clientCapabilities"]["fs"]["readTextFile"],
        true
    );
    assert_eq!(
        frames[0].message()["params"]["clientCapabilities"]["fs"]["writeTextFile"],
        false
    );
    assert_eq!(
        frames[1].message()["result"]["protocolVersion"],
        ACP_PROTOCOL_VERSION
    );
}

#[test]
fn advertised_cursor_capabilities_are_observations_not_route_claims() {
    let frames = parse_transcript(INITIALIZE).expect("Cursor initialize transcript parses");
    let result = &frames[1].message()["result"];
    let capabilities = &result["agentCapabilities"];

    assert_eq!(capabilities["loadSession"], true);
    assert!(capabilities["sessionCapabilities"]["list"].is_object());
    assert_eq!(capabilities["promptCapabilities"]["image"], true);
    assert_eq!(capabilities["promptCapabilities"]["audio"], false);
    assert_eq!(capabilities["promptCapabilities"]["embeddedContext"], false);
    assert_eq!(capabilities["mcpCapabilities"]["http"], true);
    assert_eq!(capabilities["mcpCapabilities"]["sse"], true);
    assert_eq!(result["authMethods"][0]["id"], "cursor_login");

    let fixture = parse_json(PROTOCOL);
    assert_eq!(
        fixture["capabilities"]["claimed_before_route_corpus"],
        serde_json::json!([])
    );
    assert_eq!(fixture["protocol"]["session_created"], false);
    assert_eq!(fixture["protocol"]["prompt_sent"], false);
}

#[test]
fn installed_source_derives_only_the_bounded_interactive_corpus() {
    let source = parse_json(INTERACTIVE_SOURCE);
    assert_eq!(
        source["capture_kind"],
        "installed-source-derived-normalized-corpus"
    );
    assert_eq!(source["live_provider_session_created"], false);
    assert_eq!(source["live_prompt_sent"], false);
    assert_eq!(
        source["artifacts"][0]["sha256"],
        "0332efbd33814b900e00b52753eb2b9d4ab0fa022dc264c162d2b4f535bda48f"
    );
    assert!(
        source["not_claimed"]
            .as_array()
            .expect("not-claimed list")
            .iter()
            .any(|value| value == "model_selection")
    );

    let frames = parse_transcript(INTERACTIVE).expect("Cursor interactive corpus parses");
    assert_eq!(
        methods(&frames),
        [
            "session/new",
            "session/prompt",
            "session/update",
            "session/update",
            "session/update",
            "session/update",
            "session/update",
            "session/cancel",
        ]
    );
    let updates = frames
        .iter()
        .filter(|frame| frame.method() == Some("session/update"))
        .map(|frame| {
            frame.message()["params"]["update"]["sessionUpdate"]
                .as_str()
                .expect("update kind")
        })
        .collect::<Vec<_>>();
    assert_eq!(
        updates,
        [
            "agent_thought_chunk",
            "plan",
            "tool_call",
            "tool_call_update",
            "agent_message_chunk",
        ]
    );
}

#[test]
fn catalogue_and_headless_source_records_preserve_boundaries() {
    let fixture = parse_json(PROTOCOL);
    let source = parse_json(HEADLESS_SOURCE);

    assert_eq!(fixture["catalogue"]["normalized_entry_count"], 193);
    assert_eq!(fixture["catalogue"]["dynamic"], true);
    assert_eq!(
        fixture["catalogue"]["entry_grammar"],
        "<model-id> - <display-name>"
    );
    assert_eq!(
        fixture["catalogue"]["model_invocation_availability_claimed"],
        false
    );
    assert_eq!(
        fixture["headless"]["output_formats"],
        serde_json::json!(["text", "json", "stream-json"])
    );
    assert_eq!(
        fixture["headless"]["documented_event_types"],
        serde_json::json!([
            "system",
            "user",
            "assistant",
            "thinking",
            "tool_call",
            "result"
        ])
    );
    assert_eq!(fixture["headless"]["thinking_events_suppressed"], false);
    assert_eq!(
        fixture["headless"]["terminal_fields"],
        serde_json::json!([
            "duration_ms",
            "duration_api_ms",
            "is_error",
            "result",
            "session_id",
            "request_id"
        ])
    );
    assert_eq!(
        fixture["headless"]["token_usage_fields"],
        serde_json::json!([
            "inputTokens",
            "outputTokens",
            "cacheReadTokens",
            "cacheWriteTokens"
        ])
    );
    assert!(fixture["headless"]["cancellation_stream_event"].is_null());
    assert!(fixture["headless"]["json_schema_output_flag"].is_null());
    assert_eq!(
        fixture["headless"]["provider_tool_arguments_and_results_private"],
        true
    );
    assert_eq!(fixture["headless"]["dangerous_force_flags_selected"], false);
    assert_eq!(source["live_provider_run_created"], false);
    assert_eq!(source["live_prompt_sent"], false);
    assert_eq!(
        source["artifacts"][0]["sha256"],
        "ac4050a1cd5c798979f890d21c4abc2faf074f6ac3586036090ad87f36191811"
    );
    let lines = HEADLESS.lines().map(parse_json).collect::<Vec<_>>();
    assert_eq!(lines.first().expect("system event")["type"], "system");
    assert_eq!(lines.last().expect("result event")["type"], "result");
    assert!(lines.iter().any(|event| event["type"] == "thinking"));
    assert!(
        lines
            .iter()
            .any(|event| { event["type"] == "tool_call" && event["subtype"] == "completed" })
    );
}
