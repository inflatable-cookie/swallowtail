use super::support::{Direction, parse_json, parse_transcript};
use serde_json::{Value, json};
use swallowtail_protocol_acp::{
    AcpSessionListDecodeErrorKind, AcpSessionListLimits, AcpSessionListRequest, Message,
    decode_message, decode_session_list_capabilities,
};

const ROOT: &str = "fixtures/acp-v1-session-list-stable-2026-03-09";

#[test]
fn stable_capability_and_list_shapes_decode_with_exact_correlation() {
    let initialize = parse_transcript(include_str!(
        "fixtures/acp-v1-session-list-stable-2026-03-09/initialize.ndjson"
    ))
    .expect("initialize fixture parses");
    let capabilities = decode_session_list_capabilities(&initialize[1].message()["result"])
        .expect("stable capabilities decode");
    assert!(capabilities.list());
    assert!(capabilities.additional_directories());

    let frames = parse_transcript(include_str!(
        "fixtures/acp-v1-session-list-stable-2026-03-09/list-success.ndjson"
    ))
    .expect("list fixture parses");
    assert_eq!(frames[0].direction(), Direction::ClientToAgent);
    assert_eq!(frames[1].direction(), Direction::AgentToClient);
    let request = AcpSessionListRequest::new(
        json!(7),
        capabilities,
        Some("/fixture/project".to_owned()),
        Some("opaque-page-1".to_owned()),
        AcpSessionListLimits::default(),
    )
    .expect("request is supported and bounded");
    let encoded = request.encode().expect("request encodes");
    let encoded: Value =
        serde_json::from_slice(&encoded[..encoded.len() - 1]).expect("request JSON decodes");
    assert_eq!(encoded, *frames[0].message());

    let response_bytes = serde_json::to_vec(frames[1].message()).expect("response serializes");
    let response = decode_message(&response_bytes).expect("response frame decodes");
    let page = request
        .decode_response(&response)
        .expect("correlated response projects");
    let sessions: Vec<_> = page.sessions().collect();
    assert_eq!(sessions.len(), 2);
    assert_eq!(sessions[0].session_id(), "session-private-a");
    assert_eq!(sessions[0].cwd(), "/fixture/project");
    assert_eq!(sessions[0].title(), Some("First imported session"));
    assert_eq!(sessions[0].updated_at(), Some("2026-08-01T12:34:56.789Z"));
    assert_eq!(
        sessions[0].updated_at_unix_milliseconds(),
        Some(1_785_587_696_789)
    );
    assert_eq!(
        sessions[0].additional_directories().collect::<Vec<_>>(),
        ["/fixture/shared"]
    );
    assert_eq!(page.next_cursor(), Some("opaque-page-2"));
    assert!(!sessions[0].extensions().is_empty());
    assert!(!page.extensions().is_empty());

    let debug = format!("{page:?}");
    assert!(!debug.contains("private-meta-value"));
    assert!(!debug.contains("future-private-value"));
    assert!(debug.contains("field_count"));
}

#[test]
fn list_support_is_independent_and_required_before_dispatch() {
    let absent = json!({
        "protocolVersion": 1,
        "agentCapabilities": {
            "loadSession": true,
            "sessionCapabilities": {"resume": {}, "delete": {}}
        }
    });
    let capabilities = decode_session_list_capabilities(&absent).expect("absence is valid");
    assert!(!capabilities.list());
    let error = AcpSessionListRequest::new(
        json!(1),
        capabilities,
        Some("/fixture/project".to_owned()),
        None,
        AcpSessionListLimits::default(),
    )
    .expect_err("load, resume, and delete do not grant list dispatch");
    assert_eq!(error.kind(), AcpSessionListDecodeErrorKind::Unsupported);

    for malformed in [
        json!({"protocolVersion": 2, "agentCapabilities": {}}),
        json!({"protocolVersion": 1, "agentCapabilities": {"sessionCapabilities": true}}),
        json!({"protocolVersion": 1, "agentCapabilities": {"sessionCapabilities": {"list": true}}}),
    ] {
        assert_eq!(
            decode_session_list_capabilities(&malformed)
                .expect_err("malformed capability fails closed")
                .kind(),
            AcpSessionListDecodeErrorKind::CapabilityInvalid
        );
    }
}

#[test]
fn malformed_cross_request_and_oversized_results_fail_closed() {
    let capabilities = decode_session_list_capabilities(&json!({
        "protocolVersion": 1,
        "agentCapabilities": {"sessionCapabilities": {"list": {}}}
    }))
    .expect("capability decodes");
    let request = AcpSessionListRequest::new(
        json!(7),
        capabilities,
        Some("/fixture/project".to_owned()),
        None,
        AcpSessionListLimits::new(512, 2, 32, 64, 64, 32, 64, 2),
    )
    .expect("request is valid");

    let cross_request = Message::Response {
        id: json!(8),
        result: Ok(json!({"sessions": []})),
    };
    assert_eq!(
        request
            .decode_response(&cross_request)
            .expect_err("wrong response id fails")
            .kind(),
        AcpSessionListDecodeErrorKind::CorrelationMismatch
    );

    let malformed = parse_json(include_str!(
        "fixtures/acp-v1-session-list-stable-2026-03-09/malformed-results.json"
    ));
    for case in malformed.as_array().expect("fixture cases are an array") {
        let expected = match case["expected"].as_str().expect("expected kind") {
            "response_invalid" => AcpSessionListDecodeErrorKind::ResponseInvalid,
            "resource_mismatch" => AcpSessionListDecodeErrorKind::ResourceMismatch,
            "timestamp_invalid" => AcpSessionListDecodeErrorKind::TimestampInvalid,
            "capability_invalid" => AcpSessionListDecodeErrorKind::CapabilityInvalid,
            "extension_invalid" => AcpSessionListDecodeErrorKind::ExtensionInvalid,
            other => panic!("unknown fixture error kind {other}"),
        };
        assert_eq!(
            request
                .decode_result(&case["result"])
                .expect_err("malformed result fails")
                .kind(),
            expected,
            "case {}",
            case["case"]
        );
    }

    let oversized = json!({
        "sessions": [{
            "sessionId": "session-a",
            "cwd": "/fixture/project",
            "title": "x".repeat(600)
        }]
    });
    assert_eq!(
        request
            .decode_result(&oversized)
            .expect_err("oversized result fails")
            .kind(),
        AcpSessionListDecodeErrorKind::LimitExceeded
    );
}

#[test]
fn frozen_schema_identity_and_security_posture_are_explicit() {
    let protocol = parse_json(include_str!(
        "fixtures/acp-v1-session-list-stable-2026-03-09/protocol.json"
    ));
    assert_eq!(protocol["protocol"]["wire_version"], 1);
    assert_eq!(protocol["list"]["method"], "session/list");
    assert_eq!(protocol["list"]["capability"], "sessionCapabilities.list");
    assert_eq!(protocol["authority"]["list_grants_load_or_resume"], false);
    assert_eq!(protocol["authority"]["raw_meta_public"], false);
    assert_eq!(protocol["pagination"]["cursor_persistence"], "forbidden");
    assert!(std::path::Path::new(ROOT).ends_with("acp-v1-session-list-stable-2026-03-09"));
}
