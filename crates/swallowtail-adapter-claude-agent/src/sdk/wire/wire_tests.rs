use super::{
    ClaudeAgentSdkCommand, ClaudeAgentSdkDiagnosticLevel, ClaudeAgentSdkEvent,
    ClaudeAgentSdkRecord, ClaudeAgentSdkToolDecision, decode_record, encode_callback_response,
    encode_command,
};
use crate::sdk::protocol::ClaudeAgentSdkProtocolFailureKind;
use serde_json::json;

#[test]
fn commands_and_callback_responses_encode_as_lf_terminated_records() {
    let bytes = encode_command("open-1", ClaudeAgentSdkCommand::Open, json!({"cwd": "/w"}))
        .expect("command encodes");
    assert!(bytes.ends_with(b"\n"));
    let value: serde_json::Value =
        serde_json::from_slice(&bytes[..bytes.len() - 1]).expect("command is JSON");
    assert_eq!(value["type"], "command");
    assert_eq!(value["command"], "open");
    assert_eq!(value["params"]["cwd"], "/w");

    let bytes = encode_callback_response("cb-1", ClaudeAgentSdkToolDecision::Deny)
        .expect("callback response encodes");
    let value: serde_json::Value =
        serde_json::from_slice(&bytes[..bytes.len() - 1]).expect("response is JSON");
    assert_eq!(value["type"], "callback_response");
    assert_eq!(value["decision"], "deny");
}

#[test]
fn responses_must_correlate_a_qualified_command_and_one_outcome() {
    for invalid in [
        json!({"type": "response", "id": "a", "command": "explode", "success": true}),
        json!({"type": "response", "id": "", "command": "open", "success": true}),
        json!({"type": "response", "id": "a", "command": "open"}),
        json!({"type": "response", "id": "a", "command": "open", "success": true, "failure": {"code": "x", "message": "y"}}),
        json!({"type": "response", "id": "a", "command": "open", "success": false}),
        json!({"type": "response", "id": "a", "command": "open", "success": true, "data": 7}),
    ] {
        let bytes = serde_json::to_vec(&invalid).expect("fixture serializes");
        assert_eq!(
            decode_record(&bytes).err().map(|error| error.kind()),
            Some(ClaudeAgentSdkProtocolFailureKind::InvalidResponse),
            "record {invalid} must fail closed"
        );
    }
}

#[test]
fn callbacks_admit_only_the_qualified_tool_admission_shape() {
    let bytes = serde_json::to_vec(
        &json!({"type": "callback", "id": "cb-1", "callback": "can_use_tool", "toolName": "Read"}),
    )
    .expect("fixture serializes");
    let record = decode_record(&bytes).expect("qualified callback decodes");
    let ClaudeAgentSdkRecord::Callback(callback) = record else {
        panic!("callback expected");
    };
    assert_eq!(callback.id, "cb-1");
    assert_eq!(callback.tool_name, "Read");

    for invalid in [
        json!({"type": "callback", "id": "cb-1", "callback": "request_user_dialog", "toolName": "Read"}),
        json!({"type": "callback", "id": "cb-1", "callback": "can_use_tool"}),
        json!({"type": "callback", "callback": "can_use_tool", "toolName": "Read"}),
    ] {
        let bytes = serde_json::to_vec(&invalid).expect("fixture serializes");
        assert_eq!(
            decode_record(&bytes).err().map(|error| error.kind()),
            Some(ClaudeAgentSdkProtocolFailureKind::InvalidCallback)
        );
    }
}

#[test]
fn events_decode_their_qualified_payloads_and_reject_the_rest() {
    let bytes = serde_json::to_vec(
        &json!({"type": "event", "event": "tool_ended", "toolCallId": "t-1", "isError": true}),
    )
    .expect("fixture serializes");
    let ClaudeAgentSdkRecord::Event(ClaudeAgentSdkEvent::ToolEnded { call_id, failed }) =
        decode_record(&bytes).expect("qualified event decodes")
    else {
        panic!("tool_ended expected");
    };
    assert_eq!(call_id, "t-1");
    assert!(failed);

    for (invalid, expected) in [
        (
            json!({"type": "event", "event": "tool_ended", "toolCallId": "t-1"}),
            ClaudeAgentSdkProtocolFailureKind::InvalidEvent,
        ),
        (
            json!({"type": "event", "event": "usage_report"}),
            ClaudeAgentSdkProtocolFailureKind::UnknownRecord,
        ),
        (
            json!({"type": "event"}),
            ClaudeAgentSdkProtocolFailureKind::MissingType,
        ),
        (
            json!({"type": "shell", "event": "turn_started"}),
            ClaudeAgentSdkProtocolFailureKind::UnknownRecord,
        ),
        (
            json!({"event": "turn_started"}),
            ClaudeAgentSdkProtocolFailureKind::MissingType,
        ),
    ] {
        let bytes = serde_json::to_vec(&invalid).expect("fixture serializes");
        assert_eq!(
            decode_record(&bytes).err().map(|error| error.kind()),
            Some(expected),
            "record {invalid} must fail closed"
        );
    }
}

#[test]
fn terminal_and_diagnostic_payloads_stay_bounded() {
    let bytes = serde_json::to_vec(&json!({
        "type": "diagnostic",
        "level": "error",
        "code": "sdk_close_failed",
        "message": "sidecar diagnostic: sdk_close_failed"
    }))
    .expect("fixture serializes");
    let ClaudeAgentSdkRecord::Diagnostic(diagnostic) =
        decode_record(&bytes).expect("qualified diagnostic decodes")
    else {
        panic!("diagnostic expected");
    };
    assert_eq!(diagnostic.level, ClaudeAgentSdkDiagnosticLevel::Error);

    let oversized = "x".repeat(600);
    for (invalid, expected) in [
        (
            json!({"type": "diagnostic", "level": "trace", "code": "a", "message": "b"}),
            ClaudeAgentSdkProtocolFailureKind::InvalidDiagnostic,
        ),
        (
            json!({"type": "diagnostic", "level": "info", "code": "a", "message": oversized}),
            ClaudeAgentSdkProtocolFailureKind::InvalidDiagnostic,
        ),
        (
            json!({"type": "terminal"}),
            ClaudeAgentSdkProtocolFailureKind::InvalidTerminal,
        ),
        (
            json!({"type": "terminal", "failure": {"code": "a"}}),
            ClaudeAgentSdkProtocolFailureKind::InvalidTerminal,
        ),
    ] {
        let bytes = serde_json::to_vec(&invalid).expect("fixture serializes");
        assert_eq!(
            decode_record(&bytes).err().map(|error| error.kind()),
            Some(expected)
        );
    }
}
