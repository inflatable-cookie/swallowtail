use super::*;
use std::collections::BTreeMap;

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../swallowtail-protocol-openai-chat/tests/fixtures/kimi-platform-k3-2026-07-21"
);

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(format!("{FIXTURE}/{name}")).expect("frozen fixture exists")
}

fn response(status: u32, body: Vec<u8>) -> Response {
    Response {
        status,
        _headers: BTreeMap::new(),
        body,
    }
}

#[test]
fn request_matches_the_frozen_k3_shape() {
    let content = OperationContent::new("Fixture prompt").expect("content is valid");
    let request = Request::chat(MODEL_ID, &content, &mode("high"), 128).expect("request is valid");
    let actual: serde_json::Value =
        serde_json::from_slice(request.body.as_ref().expect("body exists")).expect("JSON");
    let expected: serde_json::Value =
        serde_json::from_slice(&fixture("chat-request.json")).expect("fixture JSON");
    assert_eq!(actual, expected);
}

#[test]
fn catalogue_maps_source_scoped_metadata_without_creating_routes() {
    let models = parse_models(&response(200, fixture("models.json"))).expect("catalogue parses");
    assert_eq!(models.len(), 1);
    let model = &models[0];
    assert_eq!(model.id().as_str(), MODEL_ID);
    assert_eq!(model.provider_id().expect("provider").as_str(), PROVIDER_ID);
    let metadata = model.metadata();
    assert_eq!(
        metadata
            .token_limits()
            .expect("limits")
            .maximum_input_tokens(),
        Some(MAXIMUM_OUTPUT_TOKENS)
    );
    let reasoning = metadata.reasoning().expect("reasoning evidence");
    assert!(reasoning.supports(&mode("low")));
    assert!(reasoning.supports(&mode("high")));
    assert_eq!(reasoning.default_mode().expect("default").as_str(), "max");
    let observations = metadata
        .catalog_observations()
        .expect("catalogue observations");
    assert_eq!(observations.source().as_str(), "kimi-platform");
    assert_eq!(observations.response_streaming_supported(), Some(true));
}

#[test]
fn success_stream_preserves_reasoning_output_usage_and_done_order() {
    let mut decoder = SseDecoder::default();
    let frames = decoder.push(&fixture("success.sse")).expect("SSE decodes");
    decoder.finish().expect("stream is complete");
    let events: Vec<_> = frames
        .iter()
        .flat_map(|frame| parse_events(frame, MODEL_ID).expect("event parses"))
        .collect();
    assert_eq!(events[0], Event::RoleStart);
    assert_eq!(events[1], Event::ReasoningDelta("Fixture ".to_owned()));
    assert_eq!(events[2], Event::ReasoningDelta("reasoning".to_owned()));
    assert_eq!(events[3], Event::OutputDelta("Fixture ".to_owned()));
    assert_eq!(events[4], Event::OutputDelta("answer".to_owned()));
    assert!(matches!(events[5], Event::Finished(ref reason) if reason == "stop"));
    assert!(matches!(events[6], Event::Usage(_)));
    assert_eq!(events[7], Event::Done);
}

#[test]
fn unknown_semantics_and_returned_model_mismatch_are_distinct() {
    let unknown = decode_first("unknown.sse", MODEL_ID).expect_err("unknown field fails");
    assert_eq!(
        unknown.diagnostic().code(),
        "swallowtail.kimi_platform.content_semantics_unknown"
    );
    let mismatch = decode_first("model-mismatch.sse", MODEL_ID).expect_err("model mismatch fails");
    assert_eq!(
        mismatch.diagnostic().code(),
        "swallowtail.kimi_platform.returned_model_mismatch"
    );
}

#[test]
fn incomplete_sse_is_a_distinct_disconnect() {
    let mut decoder = SseDecoder::default();
    let _ = decoder
        .push(&fixture("disconnect.sse"))
        .expect("complete prefix decodes");
    let error = decoder.finish().expect_err("partial record fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.kimi_platform.sse_disconnected"
    );
}

#[test]
fn frozen_provider_errors_map_without_exposing_provider_messages() {
    let values: serde_json::Value = serde_json::from_slice(&fixture("errors.json")).expect("JSON");
    let expected = [
        "authentication_rejected",
        "permission_denied",
        "model_unavailable",
        "quota_unavailable",
        "rate_limited",
        "provider_unavailable",
    ];
    for (item, suffix) in values.as_array().expect("array").iter().zip(expected) {
        let status = item["status"].as_u64().expect("status") as u32;
        let body = serde_json::to_vec(&item["body"]).expect("body");
        let error = require_success(&response(status, body), "fixture request")
            .expect_err("provider error fails");
        assert!(error.diagnostic().code().ends_with(suffix));
        assert!(!error.to_string().contains("synthetic"));
    }
}

fn decode_first(name: &str, model: &str) -> Result<Vec<Event>, RuntimeFailure> {
    let mut decoder = SseDecoder::default();
    let frames = decoder.push(&fixture(name))?;
    parse_events(frames.first().expect("fixture frame"), model)
}
