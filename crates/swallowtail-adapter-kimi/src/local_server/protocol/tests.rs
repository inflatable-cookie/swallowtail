use super::rest::{
    MAX_HTTP_BODY_BYTES, RestFailureKind, RestReply, ServerMetadata, decode_archive, decode_health,
    decode_metadata, decode_rest, decode_session, inspect_asyncapi, inspect_openapi,
};
use super::ws::{
    ResyncReason, TurnEndReason, WsCloseKind, WsCursor, WsEvent, WsEventEnvelope, WsFrame,
    classify_ws_close, decode_ws_frame, encode_pong,
};
use serde_json::{Value, json};

const FIXTURE_ROOT: &str = "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0";

#[test]
fn both_qualified_releases_pass_the_selected_rest_corpus() {
    decode_health(include_bytes!(concat!(
        "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
        "health.json"
    )))
    .expect("health fixture decodes");
    inspect_openapi(include_bytes!(concat!(
        "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
        "openapi-selected.json"
    )))
    .expect("OpenAPI fixture decodes");
    inspect_asyncapi(include_bytes!(concat!(
        "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
        "asyncapi-selected.json"
    )))
    .expect("AsyncAPI fixture decodes");

    for (expected, fixture) in [
        (
            "0.28.1",
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "meta-0.28.1.json"
            ))
            .as_slice(),
        ),
        (
            "0.29.0",
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "meta-0.29.0.json"
            ))
            .as_slice(),
        ),
    ] {
        let metadata = decode_metadata(fixture).expect("metadata fixture decodes");
        assert_eq!(
            metadata,
            ServerMetadata {
                version: expected.to_owned(),
                backend: "v2".to_owned(),
                websocket: true,
            }
        );
        let executable =
            crate::kimi_code_binding(expected).expect("qualified executable version binds");
        crate::local_server::selection::corroborate_versions(&executable, &metadata.version)
            .expect("server metadata corroborates the executable");
    }

    for fixture in [
        include_bytes!(concat!(
            "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
            "session-create.json"
        ))
        .as_slice(),
        include_bytes!(concat!(
            "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
            "session-get.json"
        ))
        .as_slice(),
        include_bytes!(concat!(
            "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
            "session-restore.json"
        ))
        .as_slice(),
    ] {
        let session = decode_session(fixture).expect("session fixture decodes");
        assert_eq!(session.id, "fixture-session-private");
        assert!(!session.archived);
    }
    decode_archive(include_bytes!(concat!(
        "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
        "session-archive.json"
    )))
    .expect("archive fixture decodes");
}

#[test]
fn later_release_metadata_and_global_event_delta_are_frozen() {
    for (expected, fixture) in [
        (
            "0.29.1",
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-code-0.29.1-0.29.2/",
                "meta-0.29.1.json"
            ))
            .as_slice(),
        ),
        (
            "0.29.2",
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-code-0.29.1-0.29.2/",
                "meta-0.29.2.json"
            ))
            .as_slice(),
        ),
    ] {
        let metadata = decode_metadata(fixture).expect("later metadata decodes");
        assert_eq!(metadata.version, expected);
        let executable =
            crate::kimi_code_binding(expected).expect("qualified executable version binds");
        crate::local_server::selection::corroborate_versions(&executable, &metadata.version)
            .expect("later server metadata corroborates the executable");
    }

    let WsFrame::Event(event) = decode_ws_frame(include_bytes!(concat!(
        "../../../tests/fixtures/kimi-code-0.29.1-0.29.2/",
        "ws-global-session-created.json"
    )))
    .expect("global event decodes") else {
        panic!("global fixture must be an event");
    };
    assert_eq!(event.session_id, "foreign-session");
    assert_eq!(event.event, WsEvent::Progress);
}

#[test]
fn selected_rest_failures_are_classified_without_wire_detail() {
    for (status, fixture, expected) in [
        (
            400,
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "error-validation.json"
            ))
            .as_slice(),
            RestFailureKind::Validation,
        ),
        (
            401,
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "error-unauthorized.json"
            ))
            .as_slice(),
            RestFailureKind::Unauthorized,
        ),
        (
            404,
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "error-missing.json"
            ))
            .as_slice(),
            RestFailureKind::Missing,
        ),
        (
            409,
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "error-busy.json"
            ))
            .as_slice(),
            RestFailureKind::Busy,
        ),
        (
            500,
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "error-server.json"
            ))
            .as_slice(),
            RestFailureKind::Server,
        ),
    ] {
        assert_eq!(
            decode_rest(status, fixture).expect("error envelope decodes"),
            RestReply::Failure(expected)
        );
    }
}

#[test]
fn websocket_v2_frames_keep_lifecycle_and_sync_meanings_distinct() {
    assert_eq!(
        decode_ws_frame(include_bytes!(concat!(
            "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
            "ws-server-hello.json"
        )))
        .expect("hello decodes"),
        WsFrame::ServerHello {
            protocol_version: 2
        }
    );
    assert_eq!(
        decode_ws_frame(include_bytes!(concat!(
            "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
            "ws-subscribe.json"
        )))
        .expect("subscribe decodes"),
        WsFrame::Subscribe {
            session_count: 1,
            cursor_count: 1
        }
    );
    assert_eq!(
        decode_ws_frame(include_bytes!(concat!(
            "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
            "ws-subscribe-ack.json"
        )))
        .expect("ack decodes"),
        WsFrame::Ack {
            code: 0,
            accepted_count: 1,
            resync_count: 0,
            cursors: vec![WsCursor {
                session_id: "fixture-session-private".to_owned(),
                seq: 9,
                epoch: Some("fixture-epoch-private".to_owned()),
            }]
        }
    );
    assert_eq!(
        decode_ws_frame(include_bytes!(concat!(
            "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
            "ws-event.json"
        )))
        .expect("durable event decodes"),
        WsFrame::Event(WsEventEnvelope {
            durable_seq: 10,
            epoch: Some("fixture-epoch-private".to_owned()),
            volatile: false,
            offset: None,
            session_id: "fixture-session-private".to_owned(),
            event: WsEvent::Progress,
        })
    );
    assert_eq!(
        decode_ws_frame(include_bytes!(concat!(
            "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
            "ws-volatile-event.json"
        )))
        .expect("volatile event decodes"),
        WsFrame::Event(WsEventEnvelope {
            durable_seq: 10,
            epoch: None,
            volatile: true,
            offset: Some(0),
            session_id: "fixture-session-private".to_owned(),
            event: WsEvent::AssistantDelta {
                turn_id: 7,
                delta: "fixture-private-prompt-fragment".to_owned(),
            },
        })
    );
    assert_eq!(
        decode_ws_frame(include_bytes!(concat!(
            "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
            "ws-resync.json"
        )))
        .expect("resync decodes"),
        WsFrame::ResyncRequired {
            reason: ResyncReason::EpochChanged,
            current_seq: 12
        }
    );
    assert_eq!(
        decode_ws_frame(include_bytes!(concat!(
            "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
            "ws-abort.json"
        )))
        .expect("abort decodes"),
        WsFrame::Abort
    );
    assert_eq!(
        decode_ws_frame(include_bytes!(concat!(
            "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
            "ws-error.json"
        )))
        .expect("error decodes"),
        WsFrame::Error { fatal: true }
    );
    assert_eq!(classify_ws_close(1000), WsCloseKind::Normal);
    assert_eq!(classify_ws_close(1001), WsCloseKind::GoingAway);
    assert_eq!(classify_ws_close(1011), WsCloseKind::Unexpected);
}

#[test]
fn application_ping_decodes_without_session_seq_and_encodes_matching_pong() {
    assert_eq!(
        decode_ws_frame(
            br#"{"type":"ping","timestamp":"2026-08-14T00:00:00.000Z","payload":{"nonce":"n1"}}"#
        )
        .expect("ping decodes"),
        WsFrame::Ping {
            nonce: "n1".to_owned()
        }
    );
    let pong: Value = serde_json::from_str(&encode_pong("n1")).expect("pong encodes JSON");
    assert_eq!(pong, json!({"type":"pong","payload":{"nonce":"n1"}}));
    assert!(
        decode_ws_frame(br#"{"type":"ping","timestamp":"2026-08-14T00:00:00.000Z","payload":{}}"#)
            .is_err()
    );
}

#[test]
fn selected_interactive_events_preserve_turn_and_terminal_meaning() {
    let root = "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/";
    let cases = [
        (
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "ws-turn-started.json"
            ))
            .as_slice(),
            WsEvent::TurnStarted { turn_id: 7 },
        ),
        (
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "ws-thinking-delta.json"
            ))
            .as_slice(),
            WsEvent::ThinkingDelta {
                turn_id: 7,
                delta: "fixture-private-reasoning".to_owned(),
            },
        ),
        (
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "ws-turn-ended-completed.json"
            ))
            .as_slice(),
            WsEvent::TurnEnded {
                turn_id: 7,
                reason: TurnEndReason::Completed,
            },
        ),
        (
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "ws-awaiting-approval.json"
            ))
            .as_slice(),
            WsEvent::AwaitingApproval,
        ),
        (
            include_bytes!(concat!(
                "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
                "ws-awaiting-question.json"
            ))
            .as_slice(),
            WsEvent::AwaitingQuestion,
        ),
    ];
    for (fixture, expected) in cases {
        let WsFrame::Event(envelope) = decode_ws_frame(fixture).expect("event decodes") else {
            panic!("fixture must be an event");
        };
        assert_eq!(envelope.event, expected);
    }
    assert!(root.ends_with("kimi-local-server-0.28.1-0.29.0/"));
}

#[test]
fn optional_0_31_1_interrupt_reason_does_not_replace_terminal_reason() {
    let fixture = include_bytes!(concat!(
        "../../../tests/fixtures/kimi-code-0.31.1/",
        "turn-ended-interrupted.jsonl"
    ));
    let WsFrame::Event(envelope) = decode_ws_frame(fixture).expect("0.31.1 event decodes") else {
        panic!("fixture must be an event");
    };
    assert_eq!(
        envelope.event,
        WsEvent::TurnEnded {
            turn_id: 8,
            reason: TurnEndReason::Cancelled,
        }
    );
}

#[test]
fn bounded_unknown_semantic_event_preserves_only_its_namespace() {
    let event = decode_ws_frame(
        br#"{"type":"future.private","seq":11,"epoch":"fixture-epoch","session_id":"fixture-session","timestamp":"2026-07-27T00:00:02.000Z","payload":{"secret":"private"}}"#,
    )
    .expect("bounded unknown event decodes");
    let WsFrame::Event(event) = event else {
        panic!("unknown semantic record must remain an event");
    };
    assert_eq!(event.event, WsEvent::Unknown("future.private".to_owned()));
    let rendered = format!("{event:?}");
    assert!(!rendered.contains("\"secret\""));
}

#[test]
fn selected_session_surface_has_no_delete_effect() {
    let fixture = include_bytes!(concat!(
        "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
        "openapi-selected.json"
    ));
    inspect_openapi(fixture).expect("selected surface has no delete route");

    let mut document: Value = serde_json::from_slice(fixture).expect("fixture is JSON");
    document["paths"]["/api/v1/sessions/{session_id}"]["delete"] = json!({});
    let changed = serde_json::to_vec(&document).expect("modified document serializes");
    assert!(inspect_openapi(&changed).is_err());

    let mut document: Value = serde_json::from_slice(fixture).expect("fixture is JSON");
    document["paths"]["/api/v1/sessions/{session_id}:delete"] = json!({"post": {}});
    let changed = serde_json::to_vec(&document).expect("modified document serializes");
    assert!(inspect_openapi(&changed).is_err());
}

#[test]
fn malformed_and_oversized_payloads_produce_bounded_safe_diagnostics() {
    let private = br#"{"private_token":"secret-token","private_path":"/private/workspace"}"#;
    let malformed = decode_rest(200, private).expect_err("malformed response fails");
    let rendered = format!("{malformed:?}");
    assert_eq!(
        malformed.diagnostic().code(),
        "swallowtail.kimi.local_server.malformed_response"
    );
    for forbidden in ["secret-token", "/private/workspace", "private_token"] {
        assert!(!rendered.contains(forbidden));
    }

    let oversized = vec![b'x'; MAX_HTTP_BODY_BYTES + 1];
    assert!(decode_rest(200, &oversized).is_err());
}

#[test]
fn fixture_root_documents_exact_provenance() {
    let provenance = include_str!(concat!(
        "../../../tests/fixtures/kimi-local-server-0.28.1-0.29.0/",
        "README.md"
    ));
    assert!(provenance.contains("efacf0452d46f5dbd67499eabc053869495d5213"));
    assert!(provenance.contains("8bf5bacba9e524c38fb808c0122070037ead25a8"));
    assert!(provenance.contains("no session"));
    assert!(FIXTURE_ROOT.ends_with("kimi-local-server-0.28.1-0.29.0"));
}
