use serde_json::Value;
use swallowtail_protocol_acp::{
    AcpContentBlock, AcpMessageRole, AcpOptionalUpdate, AcpSessionUpdate,
    AcpSessionUpdateSemantics, AcpToolCallContent, ActivityDecodeErrorKind, ActivityDecodeLimits,
    DEFAULT_MAX_FRAME_BYTES, decode_session_update, decode_session_update_with_limits,
};

const UPDATES: &str = include_str!("../fixtures/acp-v1-activity-schema-v1.20.0/updates.jsonl");

#[test]
fn decoded_records_preserve_delta_replacement_and_metadata_truth() {
    let cases = json_lines(UPDATES);
    let message = decode_fixture(case(&cases, "agent-message")).expect("message decodes");
    assert_eq!(
        message.update.semantics(),
        AcpSessionUpdateSemantics::ContentDelta
    );
    let AcpSessionUpdate::Message(message) = message.update else {
        panic!("message variant retained");
    };
    assert_eq!(message.role, AcpMessageRole::Agent);
    assert!(matches!(message.content, AcpContentBlock::Text(_)));

    let plan = decode_fixture(case(&cases, "plan-replacement")).expect("plan decodes");
    assert_eq!(
        plan.update.semantics(),
        AcpSessionUpdateSemantics::ReplacementSnapshot
    );
    let AcpSessionUpdate::Plan(entries) = plan.update else {
        panic!("plan replacement retained");
    };
    assert_eq!(entries.len(), 2);

    let commands = decode_fixture(case(&cases, "commands-replacement")).expect("commands decode");
    assert_eq!(
        commands.update.semantics(),
        AcpSessionUpdateSemantics::ReplacementSnapshot
    );
    let AcpSessionUpdate::AvailableCommands(commands) = commands.update else {
        panic!("commands replacement retained");
    };
    assert_eq!(commands[0].name.as_str(), "compact");

    let info = decode_fixture(case(&cases, "session-info")).expect("session info decodes");
    let AcpSessionUpdate::SessionInfo { title, updated_at } = info.update else {
        panic!("partial session metadata retained");
    };
    assert!(matches!(title, AcpOptionalUpdate::Set(_)));
    assert!(matches!(updated_at, AcpOptionalUpdate::Set(_)));
}

#[test]
fn typed_content_config_and_additive_fields_decode_without_raw_json() {
    let tool = decode_session_update(&serde_json::json!({
        "sessionId": "session-fixture",
        "additiveEnvelopeField": true,
        "update": {
            "sessionUpdate": "tool_call_update",
            "toolCallId": "tool-fixture",
            "additiveUpdateField": {"private": true},
            "content": [
                {"type": "diff", "path": "/fixture/file", "oldText": "old", "newText": "new"},
                {"type": "terminal", "terminalId": "terminal-fixture"},
                {"type": "content", "content": {
                    "type": "resource_link",
                    "name": "fixture",
                    "uri": "file:///fixture",
                    "description": "display"
                }}
            ],
            "locations": [{"path": "/fixture/file", "line": 7}]
        }
    }))
    .expect("typed tool content decodes");
    let AcpSessionUpdate::ToolCallUpdate(tool) = tool.update else {
        panic!("tool update retained");
    };
    let content = tool
        .content_replacement
        .expect("content collection is a replacement");
    assert!(matches!(content[0], AcpToolCallContent::Diff { .. }));
    assert!(matches!(content[1], AcpToolCallContent::Terminal { .. }));
    assert!(matches!(
        content[2],
        AcpToolCallContent::Content(AcpContentBlock::ResourceLink { .. })
    ));
    assert_eq!(
        tool.locations_replacement
            .expect("locations are a replacement")[0]
            .line,
        Some(7)
    );

    let config = decode_session_update(&serde_json::json!({
        "sessionId": "session-fixture",
        "update": {
            "sessionUpdate": "config_option_update",
            "configOptions": [{
                "id": "model",
                "name": "Model",
                "category": "model",
                "type": "select",
                "currentValue": "model-a",
                "options": [{"value": "model-a", "name": "Model A"}]
            }, {
                "id": "safe",
                "name": "Safe mode",
                "type": "boolean",
                "currentValue": true
            }]
        }
    }))
    .expect("typed config replacement decodes");
    let AcpSessionUpdate::ConfigOptions(options) = config.update else {
        panic!("config replacement retained");
    };
    assert_eq!(options.len(), 2);
}

#[test]
fn default_activity_decode_is_independent_of_the_transport_frame_limit() {
    let file_content = "x".repeat(41 * 1024);
    let params = serde_json::json!({
        "sessionId": "session-fixture",
        "update": {
            "sessionUpdate": "tool_call_update",
            "toolCallId": "tool-fixture",
            "title": "Read",
            "status": "completed",
            "content": [{
                "type": "content",
                "content": {"type": "text", "text": file_content}
            }],
            "rawOutput": file_content
        }
    });
    assert!(serde_json::to_vec(&params).unwrap().len() > DEFAULT_MAX_FRAME_BYTES);

    let decoded = decode_session_update(&params).expect("large delivered update decodes");
    assert!(matches!(
        decoded.update,
        AcpSessionUpdate::ToolCallUpdate(_)
    ));
}

#[test]
fn configured_bounds_and_contradictory_shapes_fail_safely() {
    let params = serde_json::json!({
        "sessionId": "session-fixture",
        "update": {
            "sessionUpdate": "agent_message_chunk",
            "content": {"type": "text", "text": "x".repeat(128)}
        }
    });
    assert_eq!(
        decode_session_update_with_limits(&params, ActivityDecodeLimits::new(100, 8, 64))
            .expect_err("aggregate update bound applies")
            .kind(),
        ActivityDecodeErrorKind::LimitExceeded
    );

    let plan = serde_json::json!({
        "sessionId": "session-fixture",
        "update": {
            "sessionUpdate": "plan",
            "entries": [
                {"content": "one", "priority": "high", "status": "pending"},
                {"content": "two", "priority": "low", "status": "completed"}
            ]
        }
    });
    assert_eq!(
        decode_session_update_with_limits(&plan, ActivityDecodeLimits::new(4096, 1, 64))
            .expect_err("collection bound applies")
            .kind(),
        ActivityDecodeErrorKind::LimitExceeded
    );

    let identifier = serde_json::json!({
        "sessionId": "sid",
        "update": {
            "sessionUpdate": "tool_call_update",
            "toolCallId": "x".repeat(21)
        }
    });
    assert_eq!(
        decode_session_update_with_limits(&identifier, ActivityDecodeLimits::new(4096, 8, 20))
            .expect_err("identifier bound applies")
            .kind(),
        ActivityDecodeErrorKind::IdentifierInvalid
    );

    for usage in [
        serde_json::json!({"used": 2, "size": 1}),
        serde_json::json!({"used": 0, "size": 0}),
    ] {
        let params = serde_json::json!({
            "sessionId": "session-fixture",
            "update": {
                "sessionUpdate": "usage_update",
                "used": usage["used"],
                "size": usage["size"]
            }
        });
        assert_eq!(
            decode_session_update(&params)
                .expect_err("contradictory usage fails")
                .kind(),
            ActivityDecodeErrorKind::UsageInvalid
        );
    }
}

fn decode_fixture(
    fixture: &Value,
) -> Result<
    swallowtail_protocol_acp::DecodedSessionUpdate,
    swallowtail_protocol_acp::ActivityDecodeError,
> {
    decode_session_update(&fixture["message"]["params"])
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
