use super::{ClientEvent, RealtimeServerEvent, SessionReasoningAck, parse_server_event};
use serde_json::Value;
use std::num::NonZeroU64;

#[test]
fn session_scoped_reasoning_effort_encodes_and_parses_exactly() {
    let expected_update: Value = serde_json::from_slice(include_bytes!(concat!(
        "../../tests/fixtures/openai-realtime-reasoning-effort-2026-08-27/",
        "reasoning-effort-session-update.json"
    )))
    .expect("reasoning update fixture is JSON");
    let expected_updated: Value = serde_json::from_slice(include_bytes!(concat!(
        "../../tests/fixtures/openai-realtime-reasoning-effort-2026-08-27/",
        "reasoning-effort-session-updated.json"
    )))
    .expect("reasoning updated fixture is JSON");
    let encoded = ClientEvent::SessionUpdate {
        maximum_output_tokens: None,
        reasoning_effort: Some("low"),
    }
    .to_json();
    assert_eq!(
        encoded["session"]["reasoning"],
        expected_update["session"]["reasoning"]
    );
    assert_eq!(encoded["type"], "session.update");
    assert!(encoded["session"].get("max_output_tokens").is_none());
    let parsed = parse_server_event(
        serde_json::to_vec(&expected_updated)
            .expect("fixture serializes")
            .as_slice(),
    )
    .expect("reasoning acknowledgement parses");
    let RealtimeServerEvent::SessionUpdated {
        reasoning: SessionReasoningAck::Effort(effort),
    } = parsed
    else {
        panic!("acknowledgement must be session.updated with effort");
    };
    assert_eq!(effort, "low");
    for effort in ["minimal", "low", "medium", "high", "xhigh"] {
        let event = ClientEvent::SessionUpdate {
            maximum_output_tokens: NonZeroU64::new(512),
            reasoning_effort: Some(effort),
        }
        .to_json();
        assert_eq!(event["session"]["reasoning"]["effort"], effort);
        assert_eq!(event["session"]["max_output_tokens"], 512);
    }
}

#[test]
fn session_updated_preserves_absent_exact_and_invalid_reasoning_shapes() {
    let absent = parse_server_event(
        br#"{"type":"session.updated","session":{"model":"gpt-realtime-2.1","audio":{"input":{"format":{"type":"audio/pcm","rate":24000}},"output":{"format":{"type":"audio/pcm","rate":24000}}}}}"#,
    )
    .expect("absent reasoning parses");
    assert!(matches!(
        absent,
        RealtimeServerEvent::SessionUpdated {
            reasoning: SessionReasoningAck::Absent
        }
    ));
    let invalid = parse_server_event(
        br#"{"type":"session.updated","session":{"model":"gpt-realtime-2.1","reasoning":{"effort":1},"audio":{"input":{"format":{"type":"audio/pcm","rate":24000}},"output":{"format":{"type":"audio/pcm","rate":24000}}}}}"#,
    )
    .expect("invalid reasoning parses without failing closed at decode");
    assert!(matches!(
        invalid,
        RealtimeServerEvent::SessionUpdated {
            reasoning: SessionReasoningAck::Invalid
        }
    ));
    let created = parse_server_event(
        br#"{"type":"session.created","session":{"model":"gpt-realtime-2.1","reasoning":{"effort":"low"},"audio":{"input":{"format":{"type":"audio/pcm","rate":24000}},"output":{"format":{"type":"audio/pcm","rate":24000}}}}}"#,
    )
    .expect("session.created parses");
    assert!(matches!(created, RealtimeServerEvent::SessionCreated));
}
