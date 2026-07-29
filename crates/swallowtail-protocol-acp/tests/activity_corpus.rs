use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_protocol_acp::{
    ACP_PROTOCOL_VERSION, AcpSessionUpdate, AcpToolCallStatus, ActivityDecodeErrorKind,
    NdjsonDecoder, decode_message, decode_session_update,
};

#[path = "activity_corpus/content.rs"]
mod activity_content_cases;
#[path = "activity_corpus/records.rs"]
mod activity_corpus_cases;

const ROOT: &str = "fixtures/acp-v1-activity-schema-v1.20.0";
const MANIFEST: &str = include_str!("fixtures/acp-v1-activity-schema-v1.20.0/manifest.json");
const UPDATES: &str = include_str!("fixtures/acp-v1-activity-schema-v1.20.0/updates.jsonl");
const MALFORMED: &str = include_str!("fixtures/acp-v1-activity-schema-v1.20.0/malformed.jsonl");
const STDIO: &str = include_str!("fixtures/acp-v1-activity-schema-v1.20.0/stdio.ndjson");
const REMOTE: &str = include_str!("fixtures/acp-v1-activity-schema-v1.20.0/remote.jsonl");

#[test]
fn current_stable_schema_freezes_every_selected_update_semantic() {
    let manifest: Value = serde_json::from_str(MANIFEST).expect("manifest JSON is valid");
    assert_eq!(manifest["protocol"]["wire_version"], ACP_PROTOCOL_VERSION);
    assert_eq!(manifest["protocol"]["schema_artifact"], "schema-v1.20.0");
    assert_eq!(manifest["sdk"]["schema_package_version"], "1.6.0");
    assert_eq!(manifest["sdk"]["core_package_version"], "2.0.0");
    assert_eq!(manifest["sdk"]["remote_transport_package_version"], "2.0.0");
    assert_eq!(manifest["sdk"]["draft_v2_enabled"], false);

    let cases = json_lines(UPDATES);
    let kinds: BTreeSet<_> = cases
        .iter()
        .filter_map(|case| update(case).get("sessionUpdate").and_then(Value::as_str))
        .collect();
    assert_eq!(
        kinds,
        BTreeSet::from([
            "agent_message_chunk",
            "agent_thought_chunk",
            "available_commands_update",
            "config_option_update",
            "current_mode_update",
            "future_activity",
            "plan",
            "session_info_update",
            "tool_call",
            "tool_call_update",
            "usage_update",
            "user_message_chunk",
        ])
    );
    assert_eq!(
        manifest["session_updates"]["agent_thought_chunk"]["semantics"],
        "client_display_content_delta"
    );
    assert_eq!(
        manifest["session_updates"]["plan"]["semantics"],
        "authoritative_full_replacement"
    );
    assert_eq!(
        manifest["session_updates"]["tool_call_update"]["semantics"],
        "partial_field_update_with_collection_replacement"
    );
    for fixture in cases
        .iter()
        .filter(|fixture| fixture["message"]["method"] == "session/update")
    {
        decode_fixture(fixture).expect("qualified session update decodes");
    }
}

#[test]
fn typed_display_fields_are_kept_separate_from_raw_and_terminal_truth() {
    let cases = json_lines(UPDATES);
    let tool = case(&cases, "tool-create");
    assert_eq!(tool["expected"]["rawInput"], "excluded");
    assert_eq!(
        case(&cases, "tool-completed")["expected"]["rawOutput"],
        "excluded"
    );
    assert_eq!(
        case(&cases, "permission")["expected"]["semantics"],
        "separate_callback_exchange"
    );
    assert_eq!(
        case(&cases, "completion")["expected"]["activity_completion"],
        "not_implied"
    );
    assert_eq!(
        case(&cases, "unknown-additive")["expected"]["semantics"],
        "bounded_namespaced_unknown"
    );

    let tool = decode_fixture(case(&cases, "tool-create")).expect("tool create decodes");
    let AcpSessionUpdate::ToolCall(tool) = tool.update else {
        panic!("tool fixture preserves create identity");
    };
    assert_eq!(tool.tool_call_id.as_str(), "tool-fixture");
    assert_eq!(tool.status, AcpToolCallStatus::Pending);
    let rendered = format!("{tool:?}");
    assert!(!rendered.contains("private"));
    assert!(!rendered.contains("rawInput"));

    let completed =
        decode_fixture(case(&cases, "tool-completed")).expect("tool completion decodes");
    let AcpSessionUpdate::ToolCallUpdate(completed) = completed.update else {
        panic!("tool fixture preserves update identity");
    };
    assert_eq!(completed.status, Some(AcpToolCallStatus::Completed));
    assert!(
        completed
            .status
            .expect("fixture supplies status")
            .is_terminal()
    );
    assert!(completed.content_replacement.is_some());

    let unknown = decode_fixture(case(&cases, "unknown-additive")).expect("safe unknown decodes");
    let AcpSessionUpdate::Unknown { ref namespace } = unknown.update else {
        panic!("unknown fixture stays namespaced");
    };
    assert_eq!(namespace.as_str(), "future_activity");
    assert!(!format!("{unknown:?}").contains("futureField"));
}

#[test]
fn malformed_update_shapes_have_frozen_fail_closed_outcomes() {
    let cases = json_lines(MALFORMED);
    let outcomes: BTreeSet<_> = cases
        .iter()
        .map(|case| {
            case["expected_error"]
                .as_str()
                .expect("expected error is text")
        })
        .collect();
    assert_eq!(
        outcomes,
        BTreeSet::from([
            "content_invalid",
            "plan_entries_invalid",
            "session_id_missing",
            "tool_identity_missing",
            "tool_status_invalid",
            "update_kind_invalid",
            "update_kind_missing",
            "usage_invalid",
        ])
    );
    for fixture in cases {
        assert_eq!(fixture["message"]["method"], "session/update");
        assert!(fixture["case"].is_string());
        let actual = decode_fixture(&fixture)
            .expect_err("malformed update fails closed")
            .kind();
        assert_eq!(actual, frozen_error(&fixture));
    }
}

#[test]
fn stdio_and_remote_framing_preserve_payload_but_not_transport_identity() {
    let mut decoder = NdjsonDecoder::default();
    let stdio_messages = decoder
        .push(STDIO.as_bytes())
        .expect("bounded stdio frames decode");
    decoder
        .finish()
        .expect("stdio corpus ends on a frame boundary");

    let remote = json_lines(REMOTE);
    assert_eq!(stdio_messages.len(), remote.len());
    for (index, record) in remote.iter().enumerate() {
        let body = serde_json::to_vec(&record["body"]).expect("remote body serializes");
        let decoded = decode_message(&body).expect("remote ACP body decodes");
        assert_eq!(decoded, stdio_messages[index]);
    }
    assert_eq!(remote[0]["transport"], "streamable_http_sse");
    assert_eq!(remote[1]["transport"], "websocket");
    assert!(ROOT.contains("schema-v1.20.0"));
}

fn update(case: &Value) -> &Value {
    &case["message"]["params"]["update"]
}

fn decode_fixture(
    fixture: &Value,
) -> Result<
    swallowtail_protocol_acp::DecodedSessionUpdate,
    swallowtail_protocol_acp::ActivityDecodeError,
> {
    decode_session_update(&fixture["message"]["params"])
}

fn frozen_error(fixture: &Value) -> ActivityDecodeErrorKind {
    match fixture["expected_error"]
        .as_str()
        .expect("expected error is text")
    {
        "content_invalid" => ActivityDecodeErrorKind::ContentInvalid,
        "plan_entries_invalid" => ActivityDecodeErrorKind::PlanEntriesInvalid,
        "session_id_missing" => ActivityDecodeErrorKind::SessionIdMissing,
        "tool_identity_missing" => ActivityDecodeErrorKind::ToolIdentityMissing,
        "tool_status_invalid" => ActivityDecodeErrorKind::ToolStatusInvalid,
        "update_kind_invalid" => ActivityDecodeErrorKind::UpdateKindInvalid,
        "update_kind_missing" => ActivityDecodeErrorKind::UpdateKindMissing,
        "usage_invalid" => ActivityDecodeErrorKind::UsageInvalid,
        unexpected => panic!("unexpected frozen error {unexpected}"),
    }
}

fn case<'a>(cases: &'a [Value], name: &str) -> &'a Value {
    cases
        .iter()
        .find(|case| case["case"] == name)
        .expect("fixture case exists")
}

fn json_lines(value: &str) -> Vec<Value> {
    value
        .lines()
        .map(|line| serde_json::from_str(line).expect("fixture line is valid JSON"))
        .collect()
}
