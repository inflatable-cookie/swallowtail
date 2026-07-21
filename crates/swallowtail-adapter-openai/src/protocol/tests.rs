use super::{
    BackgroundStatus, BackgroundStream, Method, ProviderEvent, ProviderFailureKind, Request,
    SseDecoder, parse_failure, parse_snapshot,
};
use crate::{ENDPOINT_AUDIENCE, INTEGRATION_FAMILY, SUPPORT_AUTHORITY};
use serde_json::Value;
use swallowtail_runtime::OperationContent;

const ROOT: &str = "../../tests/fixtures/openai-responses-2026-07-21";
const INITIAL: &[u8] =
    include_bytes!("../../tests/fixtures/openai-responses-2026-07-21/initial-stream.sse");
const REATTACHED: &[u8] =
    include_bytes!("../../tests/fixtures/openai-responses-2026-07-21/reattached-stream.sse");

#[test]
fn create_and_management_requests_match_the_frozen_public_api_shape() {
    let content = OperationContent::new("Say hello").expect("content is valid");
    let create = Request::create("gpt-5.6", &content, 64).expect("create request is valid");
    let expected: Value = serde_json::from_slice(include_bytes!(
        "../../tests/fixtures/openai-responses-2026-07-21/create-request.json"
    ))
    .expect("fixture is JSON");
    assert_eq!(create.method, Method::Post);
    assert_eq!(create.path, "/v1/responses");
    assert_eq!(
        serde_json::from_slice::<Value>(create.body.as_ref().expect("create has a body"))
            .expect("request body is JSON"),
        expected
    );

    let retrieve = Request::retrieve("resp_fixture_123").expect("id is valid");
    assert_eq!(retrieve.method, Method::Get);
    assert_eq!(retrieve.path, "/v1/responses/resp_fixture_123");
    assert!(retrieve.query.is_empty());

    let reattach = Request::reattach("resp_fixture_123", 3).expect("id is valid");
    assert_eq!(reattach.method, Method::Get);
    assert_eq!(
        reattach.query,
        [
            ("stream".to_owned(), "true".to_owned()),
            ("starting_after".to_owned(), "3".to_owned())
        ]
    );

    let cancel = Request::cancel("resp_fixture_123").expect("id is valid");
    assert_eq!(cancel.method, Method::Post);
    assert_eq!(cancel.path, "/v1/responses/resp_fixture_123/cancel");
    assert!(cancel.body.is_none());
    assert!(Request::cancel("../credential").is_err());
}

#[test]
fn retrieve_and_cancel_corpus_preserves_every_background_status() {
    let fixtures = [
        ("retrieve-queued.json", BackgroundStatus::Queued),
        ("retrieve-in-progress.json", BackgroundStatus::InProgress),
        ("retrieve-completed.json", BackgroundStatus::Completed),
        ("retrieve-incomplete.json", BackgroundStatus::Incomplete),
        ("retrieve-failed.json", BackgroundStatus::Failed),
        ("cancelled.json", BackgroundStatus::Cancelled),
    ];
    for (name, expected) in fixtures {
        let bytes = fixture(name);
        let snapshot = parse_snapshot(bytes).expect("snapshot parses");
        assert_eq!(snapshot.response_id, "resp_fixture_123");
        assert_eq!(snapshot.status, expected);
    }
    let completed = parse_snapshot(fixture("retrieve-completed.json")).expect("completed parses");
    assert_eq!(completed.output_text.as_deref(), Some("Hello"));
    assert_eq!(
        completed.usage.expect("usage exists").output_tokens(),
        Some(2)
    );
}

#[test]
fn one_reattachment_continues_after_the_provider_cursor_without_replay() {
    let mut initial = BackgroundStream::initial();
    let first = decode(INITIAL);
    let mut common_sequence = 40_u64;
    for frame in &first {
        initial.apply(frame).expect("initial event applies");
        common_sequence += 1;
    }
    assert_eq!(initial.response_id(), Some("resp_fixture_123"));
    assert_eq!(initial.last_sequence(), Some(3));

    let mut reattached = BackgroundStream::reattached(
        initial
            .response_id()
            .expect("response id exists")
            .to_owned(),
        initial.last_sequence().expect("cursor exists"),
    );
    let events: Vec<_> = decode(REATTACHED)
        .iter()
        .map(|frame| {
            common_sequence += 1;
            reattached.apply(frame).expect("reattached event applies")
        })
        .collect();
    assert!(matches!(events[0], ProviderEvent::OutputDelta(ref text) if text == "lo"));
    assert!(matches!(events[2], ProviderEvent::Terminal(ref snapshot)
        if snapshot.status == BackgroundStatus::Completed));
    assert_eq!(reattached.last_sequence(), Some(6));
    assert_eq!(common_sequence, 47);
}

#[test]
fn cursor_duplicate_gap_response_mismatch_and_unknown_semantics_fail_closed() {
    for (name, code) in [
        (
            "duplicate-sequence.sse",
            "swallowtail.openai.event_order_invalid",
        ),
        (
            "missing-sequence.sse",
            "swallowtail.openai.event_order_invalid",
        ),
        (
            "mismatched-response.sse",
            "swallowtail.openai.response_correlation_failed",
        ),
        ("unknown-event.sse", "swallowtail.openai.event_unknown"),
    ] {
        let mut stream = BackgroundStream::reattached("resp_fixture_123".to_owned(), 3);
        let frame = decode(fixture(name))
            .into_iter()
            .next()
            .expect("frame exists");
        let error = stream.apply(&frame).expect_err("fixture must fail");
        assert_eq!(error.diagnostic().code(), code);
    }

    let mut decoder = SseDecoder::default();
    decoder
        .push(fixture("malformed-disconnect.sse"))
        .expect("complete prefix is accepted");
    let error = decoder.finish().expect_err("partial frame fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.sse_disconnected"
    );
}

#[test]
fn semantic_status_regression_fails_closed() {
    let frames = decode(INITIAL);
    let mut stream = BackgroundStream::initial();
    for frame in &frames[..3] {
        stream.apply(frame).expect("ordered prefix applies");
    }
    let regression = super::SseFrame {
        name: "response.queued".to_owned(),
        data: br#"{"type":"response.queued","sequence_number":3,"response":{"id":"resp_fixture_123","status":"queued"}}"#.to_vec(),
    };
    let error = stream
        .apply(&regression)
        .expect_err("status regression must fail");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.event_order_invalid"
    );
}

#[test]
fn failures_and_access_pins_are_safe_and_exact() {
    assert_eq!(
        parse_failure(fixture("rate-limit-error.json")).expect("error parses"),
        ProviderFailureKind::RateLimited
    );
    let malformed = parse_snapshot(br#"{"id":"resp_secret","status":"mystery"}"#)
        .expect_err("unknown status fails");
    assert_eq!(
        malformed.diagnostic().code(),
        "swallowtail.openai.protocol_malformed"
    );
    assert!(!format!("{malformed:?}").contains("resp_secret"));

    let protocol: Value = serde_json::from_slice(fixture("protocol.json")).expect("protocol JSON");
    assert_eq!(protocol["endpoint_audience"], ENDPOINT_AUDIENCE);
    assert_eq!(protocol["integration_family"], INTEGRATION_FAMILY);
    assert_eq!(protocol["support_authority"], SUPPORT_AUTHORITY);
    assert_eq!(protocol["credential_mechanism"], "public_api_key");
    assert_eq!(protocol["metering"], "openai_api_billing");
    assert_eq!(protocol["maximum_reattachments"], 1);

    let headers: Value =
        serde_json::from_slice(fixture("response-headers.json")).expect("header fixture JSON");
    assert_eq!(headers["x-request-id"], "req_fixture_success");
    assert_eq!(headers["x-ratelimit-limit-requests"], "100");
    assert_eq!(headers["x-ratelimit-remaining-requests"], "99");
    assert_eq!(headers["x-ratelimit-limit-tokens"], "100000");
    assert_eq!(headers["x-ratelimit-remaining-tokens"], "99000");
}

fn decode(bytes: &[u8]) -> Vec<super::SseFrame> {
    let mut decoder = SseDecoder::default();
    let frames = decoder.push(bytes).expect("SSE decodes");
    decoder.finish().expect("SSE finishes cleanly");
    frames
}

fn fixture(name: &str) -> &'static [u8] {
    match name {
        "retrieve-queued.json" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "retrieve-queued.json"
        )),
        "retrieve-in-progress.json" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "retrieve-in-progress.json"
        )),
        "retrieve-completed.json" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "retrieve-completed.json"
        )),
        "retrieve-incomplete.json" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "retrieve-incomplete.json"
        )),
        "retrieve-failed.json" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "retrieve-failed.json"
        )),
        "cancelled.json" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "cancelled.json"
        )),
        "duplicate-sequence.sse" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "duplicate-sequence.sse"
        )),
        "missing-sequence.sse" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "missing-sequence.sse"
        )),
        "mismatched-response.sse" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "mismatched-response.sse"
        )),
        "unknown-event.sse" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "unknown-event.sse"
        )),
        "malformed-disconnect.sse" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "malformed-disconnect.sse"
        )),
        "rate-limit-error.json" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "rate-limit-error.json"
        )),
        "protocol.json" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "protocol.json"
        )),
        "response-headers.json" => include_bytes!(concat!(
            "../../tests/fixtures/openai-responses-2026-07-21/",
            "response-headers.json"
        )),
        _ => panic!("unknown fixture {ROOT}/{name}"),
    }
}
