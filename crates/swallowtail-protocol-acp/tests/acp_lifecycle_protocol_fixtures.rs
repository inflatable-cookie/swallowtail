use crate::support;

use serde_json::{Value, json};
use support::{Direction, methods, parse_json, parse_transcript};
use swallowtail_protocol_acp::{
    ACP_PROTOCOL_VERSION, NdjsonDecoder, decode_message, encode_message,
};

const ROOT: &str = "fixtures/acp-v1-lifecycle-schema-v1.20.0";
const PROTOCOL: &str = include_str!("fixtures/acp-v1-lifecycle-schema-v1.20.0/protocol.json");
const CLOSE_ONLY: &str =
    include_str!("fixtures/acp-v1-lifecycle-schema-v1.20.0/initialize-close-only.ndjson");
const DELETE_ONLY: &str =
    include_str!("fixtures/acp-v1-lifecycle-schema-v1.20.0/initialize-delete-only.ndjson");
const NO_LIFECYCLE: &str =
    include_str!("fixtures/acp-v1-lifecycle-schema-v1.20.0/initialize-no-lifecycle.ndjson");
const CLOSE_SUCCESS: &str =
    include_str!("fixtures/acp-v1-lifecycle-schema-v1.20.0/close-success.ndjson");
const DELETE_SUCCESS: &str =
    include_str!("fixtures/acp-v1-lifecycle-schema-v1.20.0/delete-success.ndjson");
const ERRORS: &str =
    include_str!("fixtures/acp-v1-lifecycle-schema-v1.20.0/lifecycle-errors.ndjson");

#[test]
fn exact_stable_schema_and_wire_axes_are_pinned_separately() {
    let boundary = parse_json(PROTOCOL);
    assert_eq!(boundary["protocol"]["wire_version"], ACP_PROTOCOL_VERSION);
    assert_eq!(boundary["protocol"]["schema_artifact"], "schema-v1.20.0");
    assert_eq!(
        boundary["protocol"]["schema_source_commit"],
        "5e89c71497fe07dd4ae633c181a17224f4a8956d"
    );
    assert_eq!(
        boundary["protocol"]["schema_sha256"],
        "92c1dfcda10dd47e99127500a3763da2b471f9ac61e12b9bf0430c32cf953796"
    );
    assert_eq!(
        boundary["protocol"]["schema_1_18_through_1_20_lifecycle_shape_unchanged"],
        true
    );
}

#[test]
fn close_and_delete_capability_gates_are_independent() {
    let close = parse_transcript(CLOSE_ONLY).expect("close capability parses");
    let delete = parse_transcript(DELETE_ONLY).expect("delete capability parses");
    let absent = parse_transcript(NO_LIFECYCLE).expect("empty capabilities parse");

    let close_caps = &close[1].message()["result"]["agentCapabilities"]["sessionCapabilities"];
    assert_eq!(close_caps["close"], json!({}));
    assert!(close_caps.get("delete").is_none());

    let delete_caps = &delete[1].message()["result"]["agentCapabilities"]["sessionCapabilities"];
    assert_eq!(delete_caps["close"], Value::Null);
    assert_eq!(delete_caps["delete"], json!({}));

    assert_eq!(methods(&absent), ["initialize"]);
    assert_eq!(
        absent[1].message()["result"]["agentCapabilities"]["sessionCapabilities"],
        json!({})
    );
}

#[test]
fn lifecycle_requests_responses_and_errors_are_bounded_and_correlated() {
    for (transcript, method) in [
        (CLOSE_SUCCESS, "session/close"),
        (DELETE_SUCCESS, "session/delete"),
    ] {
        let frames = parse_transcript(transcript).expect("success transcript parses");
        assert_eq!(methods(&frames), [method]);
        assert_eq!(frames[0].direction(), Direction::ClientToAgent);
        assert_eq!(frames[1].direction(), Direction::AgentToClient);
        assert_eq!(frames[0].id(), frames[1].id());
        assert_eq!(frames[1].message()["result"], json!({}));
        assert_eq!(
            frames[0].message()["params"]["sessionId"],
            "fixture-session"
        );
    }

    let errors = parse_transcript(ERRORS).expect("error transcript parses");
    assert_eq!(methods(&errors), ["session/close", "session/delete"]);
    for pair in errors.chunks_exact(2) {
        assert_eq!(pair[0].id(), pair[1].id());
        assert_eq!(pair[1].message()["error"]["code"], -32603);
        assert_eq!(pair[1].message()["error"]["message"], "Internal error");
        assert!(pair[1].message()["error"]["data"].is_null());
    }
}

#[test]
fn stdio_and_remote_transports_share_the_same_lifecycle_messages() {
    for transcript in [CLOSE_SUCCESS, DELETE_SUCCESS, ERRORS] {
        for frame in parse_transcript(transcript).expect("fixture parses") {
            let bytes = serde_json::to_vec(frame.message()).expect("fixture message serializes");
            let remote = decode_message(&bytes).expect("remote record decodes");

            let mut stdio_bytes = bytes;
            stdio_bytes.push(b'\n');
            let mut decoder = NdjsonDecoder::default();
            let stdio = decoder.push(&stdio_bytes).expect("stdio record decodes");
            decoder.finish().expect("stdio record is complete");

            assert_eq!(stdio, std::slice::from_ref(&remote));
            let encoded = encode_message(&remote).expect("record re-encodes");
            assert_eq!(
                decode_message(&encoded[..encoded.len() - 1]).expect("record round trips"),
                remote
            );
        }
    }

    assert_eq!(
        parse_json(PROTOCOL)["transport"]["shared_record"],
        "swallowtail_protocol_acp.Message"
    );
    assert_eq!(
        parse_json(PROTOCOL)["transport"]["implicit_fallback"],
        false
    );
    assert!(ROOT.contains("schema-v1.20.0"));
}

#[test]
fn portable_delete_truth_stays_history_only() {
    let boundary = parse_json(PROTOCOL);
    assert_eq!(
        boundary["delete"]["portable_deletion_strength"],
        "history_removed"
    );
    assert_eq!(boundary["delete"]["soft_or_hard"], "provider_specific");
    assert_eq!(
        boundary["delete"]["load_after_delete"],
        "implementation_defined"
    );
    assert_eq!(
        boundary["delete"]["active_target"],
        "implementation_defined"
    );
    assert_eq!(boundary["close"]["durable_history"], "not_deleted");
}
