use super::super::protocol::PiSdkSidecarProtocolFailureKind;
use super::replay::{PiSdkReplayItem, PiSdkReplayPart};
use super::{
    PiSdkSidecarCommand, PiSdkSidecarDiagnosticLevel, PiSdkSidecarEvent, PiSdkSidecarRecord,
    decode_record, encode_command,
};

fn decode(bytes: &[u8]) -> Result<PiSdkSidecarRecord, PiSdkSidecarProtocolFailureKind> {
    decode_record(bytes).map_err(|failure| failure.kind())
}

fn decode_err(bytes: &[u8]) -> PiSdkSidecarProtocolFailureKind {
    match decode_record(bytes) {
        Err(failure) => failure.kind(),
        Ok(_) => panic!("record decoded unexpectedly"),
    }
}

fn event(bytes: &[u8]) -> Result<PiSdkSidecarEvent, PiSdkSidecarProtocolFailureKind> {
    match decode(bytes)? {
        PiSdkSidecarRecord::Event(event) => Ok(event),
        _ => panic!("expected an event record"),
    }
}

fn event_err(bytes: &[u8]) -> PiSdkSidecarProtocolFailureKind {
    match decode(bytes) {
        Err(kind) => kind,
        Ok(_) => panic!("event decoded unexpectedly"),
    }
}

#[test]
fn responses_require_correlation_and_exactly_one_outcome() {
    let response = match decode(
        br#"{"type":"response","id":"c-1","command":"state","success":true,"data":{"idle":true}}"#,
    ) {
        Ok(PiSdkSidecarRecord::Response(response)) => response,
        _ => panic!("expected a response"),
    };
    assert_eq!(response.id, "c-1");
    assert_eq!(response.command, "state");
    assert!(response.success);
    assert!(response.data.is_some());

    let failure = match decode(
        br#"{"type":"response","id":"c-2","command":"state","success":false,"failure":{"code":"not_bootstrapped","message":"sidecar command failed: not_bootstrapped"}}"#,
    ) {
        Ok(PiSdkSidecarRecord::Response(response)) => response,
        _ => panic!("expected a response"),
    };
    assert!(!failure.success);
    assert_eq!(failure.failure.unwrap().code, "not_bootstrapped");

    for (bytes, kind) in [
        (
            br#"{"type":"response","id":"c-1","command":"state","success":true,"failure":{"code":"x","message":"y"}}"#.as_slice(),
            PiSdkSidecarProtocolFailureKind::InvalidResponse,
        ),
        (
            br#"{"type":"response","id":"c-1","command":"state","success":false,"data":{}}"#.as_slice(),
            PiSdkSidecarProtocolFailureKind::InvalidResponse,
        ),
        (
            br#"{"type":"response","id":"c-1","command":"transmogrify","success":true}"#.as_slice(),
            PiSdkSidecarProtocolFailureKind::InvalidResponse,
        ),
        (
            br#"{"type":"response","command":"state","success":true}"#.as_slice(),
            PiSdkSidecarProtocolFailureKind::InvalidResponse,
        ),
        (
            br#"{"type":"response","id":"c-1","command":"state","success":true,"data":[1]}"#.as_slice(),
            PiSdkSidecarProtocolFailureKind::InvalidResponse,
        ),
    ] {
        assert_eq!(decode_err(bytes), kind);
    }

    let long_id = format!(
        "{{\"type\":\"response\",\"id\":\"{}\",\"command\":\"state\",\"success\":true}}",
        "c".repeat(129)
    );
    assert_eq!(
        decode_err(long_id.as_bytes()),
        PiSdkSidecarProtocolFailureKind::InvalidResponse
    );
}

#[test]
fn message_end_requires_assistant_role_and_well_formed_usage() {
    let PiSdkSidecarEvent::MessageEnded { stop_reason, usage } = event(
        br#"{"type":"event","event":"message_end","role":"assistant","stopReason":"stop","usage":{"input":1200,"output":42,"cacheRead":3,"cacheWrite":0}}"#,
    )
    .unwrap()
    else {
        panic!("expected message end");
    };
    assert_eq!(stop_reason, "stop");
    let usage = usage.unwrap();
    assert_eq!(usage.input_tokens(), Some(1200));
    assert_eq!(usage.output_tokens(), Some(42));

    for bytes in [
        br#"{"type":"event","event":"message_end","role":"user","stopReason":"stop"}"#.as_slice(),
        br#"{"type":"event","event":"message_end","role":"assistant"}"#.as_slice(),
        br#"{"type":"event","event":"message_end","role":"assistant","stopReason":"stop","usage":{"input":-1,"output":1,"cacheRead":0,"cacheWrite":0}}"#
            .as_slice(),
    ] {
        assert_eq!(
            event_err(bytes),
            PiSdkSidecarProtocolFailureKind::InvalidEvent
        );
    }
}

#[test]
fn replay_items_decode_typed_kinds_in_sequence() {
    let PiSdkSidecarEvent::ReplayItem { sequence, item } = event(
        br#"{"type":"event","event":"replay_item","sequence":0,"item":{"kind":"user","text":"hi","images":1}}"#,
    )
    .unwrap()
    else {
        panic!("expected replay item");
    };
    assert_eq!(sequence, 0);
    let PiSdkReplayItem::User { text, images } = item else {
        panic!("expected user item");
    };
    assert_eq!(text, "hi");
    assert_eq!(images, 1);

    let PiSdkSidecarEvent::ReplayItem { item, .. } = event(
        br#"{"type":"event","event":"replay_item","sequence":1,"item":{"kind":"assistant","parts":[{"type":"thinking","thinking":"t"},{"type":"text","text":"x"},{"type":"tool_call","name":"read","arguments":{"path":"a.rs"}}],"stopReason":"toolUse","usage":{"input":1,"output":2,"cacheRead":0,"cacheWrite":0}}}"#,
    )
    .unwrap()
    else {
        panic!("expected replay item");
    };
    let PiSdkReplayItem::Assistant {
        parts,
        stop_reason,
        usage,
    } = item
    else {
        panic!("expected assistant item");
    };
    assert_eq!(stop_reason, "toolUse");
    assert!(usage.is_some());
    assert_eq!(parts.len(), 3);
    assert!(matches!(&parts[0], PiSdkReplayPart::Reasoning(value) if value == "t"));
    assert!(matches!(&parts[1], PiSdkReplayPart::Text(value) if value == "x"));
    assert!(
        matches!(&parts[2], PiSdkReplayPart::ToolCall { name, .. } if name == "read"),
        "tool call part keeps its name"
    );

    let PiSdkSidecarEvent::ReplayItem { item, .. } = event(
        br#"{"type":"event","event":"replay_item","sequence":2,"item":{"kind":"tool_result","toolName":"read","isError":false,"text":"contents"}}"#,
    )
    .unwrap()
    else {
        panic!("expected replay item");
    };
    assert!(matches!(
        item,
        PiSdkReplayItem::ToolResult { failed: false, .. }
    ));

    assert_eq!(
        event_err(
            br#"{"type":"event","event":"replay_item","sequence":3,"item":{"kind":"bash_execution"}}"#
        ),
        PiSdkSidecarProtocolFailureKind::UnknownRecord
    );
    assert_eq!(
        event_err(
            br#"{"type":"event","event":"replay_item","sequence":3,"item":{"kind":"assistant","parts":[{"type":"image"}],"stopReason":"stop"}}"#
        ),
        PiSdkSidecarProtocolFailureKind::InvalidEvent
    );
}

#[test]
fn terminal_and_diagnostic_records_are_bounded() {
    match decode(
        br#"{"type":"terminal","failure":{"code":"unknown_event","message":"sidecar terminated: unknown_event"}}"#,
    ) {
        Ok(PiSdkSidecarRecord::Terminal(failure)) => {
            assert_eq!(failure.code, "unknown_event");
        }
        _ => panic!("expected a terminal record"),
    }
    match decode(
        br#"{"type":"diagnostic","level":"warning","code":"model_fallback","message":"sidecar diagnostic: model_fallback"}"#,
    ) {
        Ok(PiSdkSidecarRecord::Diagnostic(diagnostic)) => {
            assert_eq!(diagnostic.level, PiSdkSidecarDiagnosticLevel::Warning);
        }
        _ => panic!("expected a diagnostic record"),
    }

    let long_message = format!(
        "{{\"type\":\"terminal\",\"failure\":{{\"code\":\"x\",\"message\":\"{}\"}}}}",
        "m".repeat(513)
    );
    assert_eq!(
        decode_err(long_message.as_bytes()),
        PiSdkSidecarProtocolFailureKind::InvalidTerminal
    );
    assert_eq!(
        decode_err(br#"{"type":"diagnostic","level":"chatty","code":"x","message":"y"}"#),
        PiSdkSidecarProtocolFailureKind::InvalidDiagnostic
    );
}

#[test]
fn encoder_produces_qualified_lf_terminated_commands() {
    let bytes = encode_command(
        "c-7",
        PiSdkSidecarCommand::Prompt,
        serde_json::json!({"text": "hello"}),
    )
    .unwrap();
    assert_eq!(bytes.last(), Some(&b'\n'));
    let value: serde_json::Value = serde_json::from_slice(&bytes[..bytes.len() - 1]).unwrap();
    assert_eq!(value["type"], "command");
    assert_eq!(value["id"], "c-7");
    assert_eq!(value["command"], "prompt");
    assert_eq!(value["params"]["text"], "hello");

    let mut ids = std::collections::BTreeSet::new();
    for command in [
        PiSdkSidecarCommand::Bootstrap,
        PiSdkSidecarCommand::SessionNew,
        PiSdkSidecarCommand::SessionSwitch,
        PiSdkSidecarCommand::SessionReplay,
        PiSdkSidecarCommand::Prompt,
        PiSdkSidecarCommand::Steer,
        PiSdkSidecarCommand::FollowUp,
        PiSdkSidecarCommand::Abort,
        PiSdkSidecarCommand::State,
        PiSdkSidecarCommand::Close,
    ] {
        assert!(ids.insert(command.as_str()));
    }
    assert_eq!(ids.len(), 10);
}
