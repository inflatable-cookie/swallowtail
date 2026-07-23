use super::request::{ToolSpec, encode_after_tool, encode_initial, encode_later_user};
use super::response::{ProviderFailureKind, classify_failure, parse_tool_attempt};
use super::stream::parse_final_stream;
use super::*;
use crate::{DEEPSEEK_ENDPOINT, DEEPSEEK_FACADE_REVISION, DEEPSEEK_MODEL_ID, deepseek_v4_config};
use serde_json::{Value, json};
use swallowtail_runtime::DirectInferenceAttemptId;

const ROOT: &str = "../tests/fixtures/deepseek-openai-chat-2026-07-22";

fn tools() -> Vec<ToolSpec> {
    vec![ToolSpec {
        name: "lookup_weather".to_owned(),
        description: "Return fixture weather".to_owned(),
        parameters: json!({
            "type": "object",
            "properties": {"city": {"type": "string"}},
            "required": ["city"],
            "additionalProperties": false
        }),
    }]
}

fn fixture_json(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("fixture is valid JSON")
}

#[test]
fn manifest_freezes_exact_route_bounds_cache_and_exclusions() {
    let protocol = fixture_json(include_bytes!(
        "../../tests/fixtures/deepseek-openai-chat-2026-07-22/protocol.json"
    ));
    assert_eq!(protocol["facade_revision"], DEEPSEEK_FACADE_REVISION);
    assert_eq!(protocol["endpoint"], DEEPSEEK_ENDPOINT);
    assert_eq!(protocol["path"], "/chat/completions");
    assert_eq!(protocol["model"], DEEPSEEK_MODEL_ID);
    assert_eq!(protocol["attempts"]["maximum"], 3);
    assert_eq!(protocol["attempts"]["first"], "buffered");
    assert_eq!(protocol["attempts"]["continued"], "sse");
    assert_eq!(protocol["provider_cache"]["accepted"], true);
    assert_eq!(protocol["provider_cache"]["read"], false);
    assert_eq!(protocol["provider_cache"]["delete"], false);
    assert!(
        protocol["excluded"]
            .as_array()
            .unwrap()
            .contains(&json!("/v1"))
    );
    assert!(ROOT.ends_with(DEEPSEEK_FACADE_REVISION));
    let models = fixture_json(include_bytes!(
        "../../tests/fixtures/deepseek-openai-chat-2026-07-22/models.json"
    ));
    assert!(
        models["data"]
            .as_array()
            .unwrap()
            .iter()
            .any(|model| model["id"] == DEEPSEEK_MODEL_ID)
    );
}

#[test]
fn exact_initial_tool_attempt_is_buffered_and_omits_tool_choice() {
    let encoded = encode_initial("What is the fixture weather in London?", &tools()).unwrap();
    let expected = fixture_json(include_bytes!(
        "../../tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-1-request.json"
    ));
    let actual = fixture_json(&encoded);
    assert_eq!(actual, expected);
    assert_eq!(actual["stream"], false);
    assert!(actual.get("tool_choice").is_none());
    assert_eq!(actual["thinking"]["type"], "enabled");
    assert_eq!(actual["reasoning_effort"], "high");
}

#[test]
fn tool_response_and_correlated_result_preserve_private_reasoning_only_inside_adapter() {
    let config = deepseek_v4_config();
    let attempt = parse_tool_attempt(
        include_bytes!(
            "../../tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-1-tool-response.json"
        ),
        DirectInferenceAttemptId::new("attempt-1").unwrap(),
        &config,
    )
    .unwrap();
    assert_eq!(attempt.call.call_id().as_str(), "call_weather_1");
    assert_eq!(attempt.call.tool_name(), "lookup_weather");
    assert_eq!(attempt.call.arguments().as_bytes(), br#"{"city":"London"}"#);
    assert_eq!(attempt.usage.cache_hit_tokens, 96);
    assert_eq!(attempt.usage.cache_miss_tokens, 24);
    let rendered = format!("{:?}", attempt.reasoning);
    assert!(rendered.contains("<private:"));
    assert!(!rendered.contains("fixture-private-reasoning-1"));

    let encoded = encode_after_tool(
        "What is the fixture weather in London?",
        &attempt.reasoning,
        attempt.call.call_id().as_str(),
        attempt.call.tool_name(),
        std::str::from_utf8(attempt.call.arguments().as_bytes()).unwrap(),
        r#"{"temperature_c":18,"condition":"clear"}"#,
        &tools(),
    )
    .unwrap();
    assert_eq!(
        fixture_json(&encoded),
        fixture_json(include_bytes!(
            "../../tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-2-request.json"
        ))
    );
}

#[test]
fn streaming_final_and_later_turn_keep_continuation_and_usage_exact() {
    let config = deepseek_v4_config();
    let tool_attempt = parse_tool_attempt(
        include_bytes!(
            "../../tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-1-tool-response.json"
        ),
        DirectInferenceAttemptId::new("attempt-1").unwrap(),
        &config,
    )
    .unwrap();
    let final_attempt = parse_final_stream(
        include_bytes!("../../tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-2-final.sse"),
        &config,
    )
    .unwrap();
    assert_eq!(final_attempt.output, "London is 18 C and clear.");
    assert_eq!(final_attempt.finish_reason, "stop");
    assert_eq!(final_attempt.usage.prompt_tokens, 180);
    assert_eq!(final_attempt.usage.completion_tokens, 22);
    assert_eq!(final_attempt.usage.total_tokens, 202);
    assert_eq!(final_attempt.usage.cache_hit_tokens, 128);
    assert_eq!(final_attempt.usage.cache_miss_tokens, 52);
    assert!(!format!("{:?}", final_attempt.reasoning).contains("fixture-private-reasoning-2"));

    let encoded = encode_later_user(
        "What is the fixture weather in London?",
        &tool_attempt.reasoning,
        tool_attempt.call.call_id().as_str(),
        tool_attempt.call.tool_name(),
        std::str::from_utf8(tool_attempt.call.arguments().as_bytes()).unwrap(),
        r#"{"temperature_c":18,"condition":"clear"}"#,
        &final_attempt.reasoning,
        &final_attempt.output,
        "Summarise that in three words.",
        &tools(),
    )
    .unwrap();
    assert_eq!(
        fixture_json(&encoded),
        fixture_json(include_bytes!(
            "../../tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-3-request.json"
        ))
    );
    let last = parse_final_stream(
        include_bytes!("../../tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-3-final.sse"),
        &config,
    )
    .unwrap();
    assert_eq!(last.output, "Clear, mild, London.");
}

#[test]
fn failures_rate_cancellation_and_drift_remain_distinct_offline() {
    let errors = fixture_json(include_bytes!(
        "../../tests/fixtures/deepseek-openai-chat-2026-07-22/errors.json"
    ));
    let expected = [
        (400, ProviderFailureKind::InvalidRequest),
        (401, ProviderFailureKind::Authentication),
        (402, ProviderFailureKind::InsufficientBalance),
        (429, ProviderFailureKind::AccountConcurrency),
        (500, ProviderFailureKind::Provider),
        (503, ProviderFailureKind::Overloaded),
    ];
    for (entry, (status, kind)) in errors.as_array().unwrap().iter().zip(expected) {
        assert_eq!(entry["status"], status);
        assert_eq!(classify_failure(status), Some(kind));
    }
    let lifecycle = fixture_json(include_bytes!(
        "../../tests/fixtures/deepseek-openai-chat-2026-07-22/lifecycle.json"
    ));
    assert_eq!(lifecycle["retry_count"], 0);
    assert_eq!(lifecycle["cancellation"]["remote_stop_confirmed"], false);
    assert_eq!(lifecycle["deadline"]["uses_host_monotonic_time"], true);

    for (fixture, kind) in [
        (
            include_bytes!(
                "../../tests/fixtures/deepseek-openai-chat-2026-07-22/model-mismatch.sse"
            )
            .as_slice(),
            ProtocolFailureKind::ModelMismatch,
        ),
        (
            include_bytes!("../../tests/fixtures/deepseek-openai-chat-2026-07-22/unknown.sse")
                .as_slice(),
            ProtocolFailureKind::UnknownSemanticField,
        ),
        (
            include_bytes!("../../tests/fixtures/deepseek-openai-chat-2026-07-22/disconnect.sse")
                .as_slice(),
            ProtocolFailureKind::IncompleteStream,
        ),
        (
            include_bytes!(
                "../../tests/fixtures/deepseek-openai-chat-2026-07-22/provider-error.sse"
            )
            .as_slice(),
            ProtocolFailureKind::ProviderFailure,
        ),
    ] {
        assert_eq!(
            parse_final_stream(fixture, &deepseek_v4_config())
                .unwrap_err()
                .kind(),
            kind
        );
    }
}

#[test]
fn private_continuation_and_history_bounds_fail_closed() {
    let config = deepseek_v4_config();
    let oversized = vec![b'x'; config.maximum_private_continuation_bytes().get() as usize + 1];
    assert_eq!(
        private::PrivateContinuation::new(oversized, config.maximum_private_continuation_bytes())
            .unwrap_err()
            .kind(),
        ProtocolFailureKind::BoundExceeded
    );
    let drift = include_bytes!(
        "../../tests/fixtures/deepseek-openai-chat-2026-07-22/tool-response-drift.json"
    );
    assert_eq!(
        parse_tool_attempt(
            drift,
            DirectInferenceAttemptId::new("attempt-drift").unwrap(),
            &config,
        )
        .unwrap_err()
        .kind(),
        ProtocolFailureKind::InvalidStructure
    );
}
