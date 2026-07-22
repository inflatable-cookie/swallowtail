use serde_json::{Value, json};
use swallowtail_protocol_openai_chat::{
    ChatMessage, ChatRequest, CodecLimits, SseDecoder, encode_request,
};

#[path = "kimi_k3_fixtures/support.rs"]
mod support;
use support::{FixtureError, KimiEvent, SafeErrorCategory, decode_kimi, map_error};

const ROOT: &str = "tests/fixtures/kimi-platform-k3-2026-07-21";
const MANIFEST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/kimi-platform-k3-2026-07-21/protocol.json"
));
const MODELS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/kimi-platform-k3-2026-07-21/models.json"
));
const REQUEST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/kimi-platform-k3-2026-07-21/chat-request.json"
));
const SUCCESS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/kimi-platform-k3-2026-07-21/success.sse"
));
const ERRORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/kimi-platform-k3-2026-07-21/errors.json"
));
const UNKNOWN: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/kimi-platform-k3-2026-07-21/unknown.sse"
));
const MISMATCH: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/kimi-platform-k3-2026-07-21/model-mismatch.sse"
));
const DISCONNECT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/kimi-platform-k3-2026-07-21/disconnect.sse"
));

#[test]
fn manifest_pins_exact_platform_access_route_and_exclusions() {
    let fixture: Value = serde_json::from_str(MANIFEST).expect("manifest parses");
    assert_eq!(fixture["fixture_schema"], 1);
    assert_eq!(fixture["checked_at"], "2026-07-21");
    assert_eq!(
        fixture["configured_instance"]["endpoint_audience"],
        "https://api.moonshot.ai"
    );
    assert_eq!(
        fixture["configured_instance"]["credential_issuer"],
        "platform.kimi.ai"
    );
    assert_eq!(fixture["configured_instance"]["metering"], "pay_as_you_go");
    assert_eq!(fixture["routes"]["catalogue"], json!(["GET", "/v1/models"]));
    assert_eq!(
        fixture["routes"]["inference"],
        json!(["POST", "/v1/chat/completions"])
    );
    assert_eq!(fixture["model_route"]["model"], "kimi-k3");
    assert_eq!(
        fixture["request"]["reasoning_efforts"],
        json!(["low", "high", "max"])
    );
    assert_eq!(fixture["request"]["maximum_attempts"], 1);
    assert_eq!(fixture["request"]["retry_allowed"], false);
    assert_eq!(fixture["default_qa"]["requires_credential"], false);
    assert_eq!(fixture["default_qa"]["requires_network"], false);
    assert!(ROOT.ends_with("kimi-platform-k3-2026-07-21"));
}

#[test]
fn catalogue_is_source_scoped_and_request_omits_fixed_or_excluded_fields() {
    let models: Value = serde_json::from_str(MODELS).expect("catalogue parses");
    assert_eq!(models["object"], "list");
    assert_eq!(models["data"].as_array().expect("models array").len(), 1);
    let model = &models["data"][0];
    assert_eq!(model["id"], "kimi-k3");
    assert_eq!(model["context_length"], 1_048_576);
    assert_eq!(model["supports_reasoning"], true);

    let mut request = ChatRequest::new(
        "kimi-k3",
        vec![ChatMessage::new("user", "Fixture prompt")],
        true,
        true,
    );
    request
        .insert_extension("reasoning_effort", json!("high"))
        .expect("effort field accepted");
    request
        .insert_extension("max_completion_tokens", json!(128))
        .expect("bound field accepted");
    let actual: Value = serde_json::from_slice(
        &encode_request(&request, CodecLimits::default()).expect("request encodes"),
    )
    .expect("request parses");
    let expected: Value = serde_json::from_str(REQUEST).expect("fixture request parses");
    assert_eq!(actual, expected);
    for excluded in [
        "temperature",
        "top_p",
        "n",
        "tools",
        "response_format",
        "partial",
    ] {
        assert!(actual.get(excluded).is_none());
    }
}

#[test]
fn stream_maps_reasoning_output_usage_and_done_without_flattening() {
    let events = decode_kimi(SUCCESS).expect("success stream maps");
    assert_eq!(events[0], KimiEvent::RoleStart);
    assert_eq!(events[1], KimiEvent::Reasoning("Fixture ".to_owned()));
    assert_eq!(events[2], KimiEvent::Reasoning("reasoning".to_owned()));
    assert_eq!(events[3], KimiEvent::Output("Fixture ".to_owned()));
    assert_eq!(events[4], KimiEvent::Output("answer".to_owned()));
    assert_eq!(events[5], KimiEvent::Finished("stop".to_owned()));
    assert_eq!(
        events[6],
        KimiEvent::Usage {
            input: 18,
            output: 9,
            cached: 4
        }
    );
    assert_eq!(events[7], KimiEvent::Done);
}

#[test]
fn errors_unknowns_model_mismatch_and_disconnect_remain_distinct() {
    let errors: Value = serde_json::from_str(ERRORS).expect("errors parse");
    let categories = errors
        .as_array()
        .expect("error array")
        .iter()
        .map(map_error)
        .collect::<Result<Vec<_>, _>>()
        .expect("safe error categories map");
    assert_eq!(
        categories,
        vec![
            SafeErrorCategory::Authentication,
            SafeErrorCategory::Permission,
            SafeErrorCategory::ModelUnavailable,
            SafeErrorCategory::Quota,
            SafeErrorCategory::RateLimited,
            SafeErrorCategory::ServiceUnavailable,
        ]
    );
    assert_eq!(decode_kimi(UNKNOWN), Err(FixtureError::UnknownSemantic));
    assert_eq!(decode_kimi(MISMATCH), Err(FixtureError::ModelMismatch));

    let mut decoder = SseDecoder::default();
    let records = decoder.push(DISCONNECT).expect("complete prefix decodes");
    assert_eq!(records.len(), 1);
    assert!(decoder.finish().is_err());
}
