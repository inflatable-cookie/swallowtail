use serde_json::Value;
use std::collections::BTreeSet;

const INVENTORY: &str = include_str!("fixtures/direct-activity-applicability.json");

const ALIBABA_POSITIVE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-alibaba-model-studio/tests/fixtures/model-studio-2026-07-22/success.sse"
);
const ALIBABA_UNAVAILABLE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-alibaba-model-studio/tests/fixtures/model-studio-2026-07-22/reasoning.sse"
);
const ANTHROPIC_POSITIVE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-anthropic/tests/fixtures/anthropic-2023-06-01/success.sse"
);
const ANTHROPIC_TOOL: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-anthropic/tests/fixtures/anthropic-2023-06-01/tool-use.sse"
);
const ANTHROPIC_PROVIDER_TOOL: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-anthropic/tests/fixtures/anthropic-2023-06-01/web-search.sse"
);
const DEEPSEEK_POSITIVE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-deepseek/tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-2-final.sse"
);
const DEEPSEEK_TOOL: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-deepseek/tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-1-tool-response.json"
);
const KIMI_POSITIVE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-protocol-openai-chat/tests/fixtures/kimi-platform-k3-2026-07-21/success.sse"
);
const OPENAI_INITIAL: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-openai/tests/fixtures/openai-responses-2026-07-21/initial-stream.sse"
);
const OPENAI_REATTACHED: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-openai/tests/fixtures/openai-responses-2026-07-21/reattached-stream.sse"
);
const XAI_POSITIVE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-xai/tests/fixtures/xai-responses-websocket-2026-04-23/first-turn-events.ndjson"
);
const OLLAMA_POSITIVE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-ollama/tests/fixtures/ollama-native-v0.14.0-v0.32.1/chat-success.ndjson"
);
const OLLAMA_UNAVAILABLE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-ollama/tests/fixtures/ollama-native-v0.14.0-v0.32.1/chat-unsupported.ndjson"
);
const LLAMA_POSITIVE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-llama-cpp/tests/fixtures/llama-cpp-b9910-openai-chat/success.sse"
);
const LLAMA_UNAVAILABLE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-llama-cpp/tests/fixtures/llama-cpp-b9910-openai-chat/unsupported-semantics.sse"
);
const BEDROCK_TYPED_FIXTURES: &str =
    include_str!("fixtures/provider-evidence/swallowtail-adapter-bedrock/tests/sdk_fixtures.rs");

const ALIBABA_UNKNOWN: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-alibaba-model-studio/tests/fixtures/model-studio-2026-07-22/unknown.sse"
);
const ANTHROPIC_MALFORMED: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-anthropic/tests/fixtures/anthropic-2023-06-01/disconnect.sse"
);
const DEEPSEEK_FAILURE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-deepseek/tests/fixtures/deepseek-openai-chat-2026-07-22/provider-error.sse"
);
const KIMI_UNKNOWN: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-protocol-openai-chat/tests/fixtures/kimi-platform-k3-2026-07-21/unknown.sse"
);
const OPENAI_FAILURE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-openai/tests/fixtures/openai-responses-2026-07-21/failed-stream.sse"
);
const XAI_UNKNOWN: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-xai/tests/fixtures/xai-responses-websocket-2026-04-23/unknown-event.json"
);
const OLLAMA_MALFORMED: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-ollama/tests/fixtures/ollama-native-v0.14.0-v0.32.1/chat-malformed.ndjson"
);
const LLAMA_FAILURE: &str = include_str!(
    "fixtures/provider-evidence/swallowtail-adapter-llama-cpp/tests/fixtures/llama-cpp-b9910-openai-chat/midstream-error.sse"
);

#[test]
fn every_non_harness_production_route_and_operation_is_classified() {
    let inventory = json(INVENTORY);
    assert_eq!(
        inventory["contract"],
        "044-observable-agent-activity-and-disclosure"
    );
    let routes = inventory["routes"].as_array().expect("routes are an array");
    assert_eq!(routes.len(), 13);
    assert_eq!(inventory["route_count"], 13);

    let ids: BTreeSet<_> = routes
        .iter()
        .map(|route| route["id"].as_str().expect("route id is text"))
        .collect();
    assert_eq!(
        ids,
        BTreeSet::from([
            "alibaba.conversations",
            "anthropic.messages",
            "bedrock.catalogue",
            "bedrock.runtime",
            "deepseek.continuation",
            "gemini.live",
            "kimi-platform.chat",
            "llama-cpp.attached",
            "llama-cpp.owned",
            "ollama.attached",
            "openai.background",
            "openai.realtime",
            "xai.responses-websocket",
        ])
    );

    let ordinary_count: usize = routes
        .iter()
        .map(|route| {
            route["ordinary_profiles"]
                .as_array()
                .expect("ordinary profiles are an array")
                .len()
        })
        .sum();
    assert_eq!(ordinary_count, 14);
    assert_eq!(inventory["ordinary_text_profile_count"], 14);

    let not_applicable_count: usize = routes
        .iter()
        .map(|route| {
            route["non_activity_operations"]
                .as_array()
                .expect("non-activity operations are an array")
                .len()
        })
        .sum::<usize>()
        + inventory["auxiliary_catalogues"]
            .as_array()
            .expect("auxiliary catalogues are an array")
            .len();
    assert_eq!(not_applicable_count, 13);
    assert_eq!(inventory["not_applicable_operation_count"], 13);
}

#[test]
fn text_profiles_keep_available_unavailable_and_separate_evidence_exact() {
    let inventory = json(INVENTORY);
    for route in inventory["routes"].as_array().expect("route array") {
        for profile in route["ordinary_profiles"]
            .as_array()
            .expect("ordinary profiles")
        {
            assert_eq!(profile["applicability"], "available");
            assert_eq!(profile["assistant_message"], "available");
            for field in ["lifecycle", "correlation", "ownership", "disclosure"] {
                assert!(
                    profile[field]
                        .as_str()
                        .is_some_and(|value| !value.is_empty()),
                    "{} {} is missing {field}",
                    route["id"],
                    profile["operation"]
                );
            }
        }
        assert!(
            route["reasoning_boundary"]
                .as_str()
                .is_some_and(|value| !value.is_empty()),
            "{} is missing a reasoning boundary",
            route["id"]
        );
    }

    assert!(ALIBABA_POSITIVE.contains("\"type\":\"response.output_item.added\""));
    assert!(ALIBABA_POSITIVE.contains("\"item_id\":\"msg_output_01\""));
    assert!(ALIBABA_UNAVAILABLE.contains("reasoning_summary_text.delta"));

    assert!(ANTHROPIC_POSITIVE.contains("\"type\":\"text_delta\""));
    assert!(ANTHROPIC_TOOL.contains("\"type\":\"tool_use\""));
    assert!(ANTHROPIC_TOOL.contains("\"id\":\"toolu_fixture_1\""));
    assert!(ANTHROPIC_PROVIDER_TOOL.contains("\"type\":\"server_tool_use\""));
    assert!(ANTHROPIC_PROVIDER_TOOL.contains("\"type\":\"web_search_tool_result\""));

    assert!(DEEPSEEK_POSITIVE.contains("\"delta\":{\"content\""));
    assert!(DEEPSEEK_TOOL.contains("\"tool_calls\""));
    assert!(DEEPSEEK_TOOL.contains("\"reasoning_content\""));

    assert!(KIMI_POSITIVE.contains("\"reasoning_content\""));
    assert!(KIMI_POSITIVE.contains("\"delta\":{\"content\""));

    assert!(OPENAI_INITIAL.contains("response.output_text.delta"));
    assert!(OPENAI_REATTACHED.contains("response.output_text.done"));
    assert!(OPENAI_REATTACHED.contains("response.completed"));

    assert!(XAI_POSITIVE.contains("\"item_id\":\"msg_fixture_first\""));
    assert!(XAI_POSITIVE.contains("\"type\":\"response.completed\""));

    assert!(OLLAMA_POSITIVE.contains("\"role\":\"assistant\""));
    assert!(!OLLAMA_POSITIVE.contains("\"thinking\""));
    assert!(OLLAMA_UNAVAILABLE.contains("\"thinking\":\"private reasoning\""));

    assert!(LLAMA_POSITIVE.contains("\"role\":\"assistant\""));
    assert!(LLAMA_UNAVAILABLE.contains("reasoning_content"));

    for required in [
        "typed_sdk_success_stream_preserves_text_stop_and_usage_order",
        "UnsupportedSemanticEvent",
        "UnknownSdkVariant",
        "EventOutOfOrder",
    ] {
        assert!(
            BEDROCK_TYPED_FIXTURES.contains(required),
            "missing Bedrock typed fixture {required}"
        );
    }
}

#[test]
fn direct_tool_ownership_and_private_reasoning_are_not_flattened() {
    let inventory = json(INVENTORY);
    let routes = inventory["routes"].as_array().expect("route array");

    let anthropic = route(routes, "anthropic.messages");
    let structured = profile(anthropic, "structured-run");
    assert_eq!(
        structured["provider_tool"],
        "available-with-explicit-web-search"
    );
    assert_eq!(structured["consumer_tool"], "not-applicable");
    let interactive = profile(anthropic, "interactive-session");
    assert_eq!(interactive["consumer_tool"], "available");
    assert!(
        interactive["ownership"]
            .as_str()
            .expect("ownership is text")
            .contains("consumer-executed")
    );

    let deepseek = route(routes, "deepseek.continuation");
    assert_eq!(
        profile(deepseek, "interactive-session")["consumer_tool"],
        "available"
    );
    assert!(
        deepseek["reasoning_boundary"]
            .as_str()
            .expect("reasoning boundary")
            .contains("adapter-private continuation")
    );

    let kimi = route(routes, "kimi-platform.chat");
    assert_eq!(
        profile(kimi, "structured-run")["reasoning_summary"],
        "available"
    );
    assert!(
        profile(kimi, "structured-run")["disclosure"]
            .as_str()
            .expect("disclosure")
            .contains("never complete reasoning")
    );

    let xai = route(routes, "xai.responses-websocket");
    assert!(
        xai["reasoning_boundary"]
            .as_str()
            .expect("reasoning boundary")
            .contains("private continuation")
    );
}

#[test]
fn realtime_catalogue_inventory_and_serving_stay_not_applicable() {
    let inventory = json(INVENTORY);
    let routes = inventory["routes"].as_array().expect("route array");
    for id in ["openai.realtime", "gemini.live"] {
        let route = route(routes, id);
        assert!(
            route["ordinary_profiles"]
                .as_array()
                .expect("ordinary profiles")
                .is_empty()
        );
        assert_eq!(
            route["non_activity_operations"][0]["operation"],
            "realtime-media-session"
        );
        assert_eq!(
            route["non_activity_operations"][0]["applicability"],
            "not-applicable"
        );
    }

    let serving = route(routes, "llama-cpp.owned");
    assert_eq!(
        serving["non_activity_operations"][0]["operation"],
        "serving-lifecycle"
    );
    assert_eq!(
        serving["non_activity_operations"][0]["applicability"],
        "not-applicable"
    );

    for operation in routes
        .iter()
        .flat_map(|route| {
            route["non_activity_operations"]
                .as_array()
                .expect("non-activity operations")
        })
        .chain(
            inventory["auxiliary_catalogues"]
                .as_array()
                .expect("auxiliary catalogues"),
        )
    {
        assert_eq!(operation["applicability"], "not-applicable");
    }
}

#[test]
fn positive_unknown_malformed_and_failure_corpora_remain_bounded() {
    assert!(ALIBABA_UNKNOWN.contains("response.provider_metadata"));
    assert!(KIMI_UNKNOWN.contains("future_semantic"));
    assert!(XAI_UNKNOWN.contains("response.future_semantic_content"));

    assert!(!ANTHROPIC_MALFORMED.trim_end().ends_with("\n\n"));
    assert!(OLLAMA_MALFORMED.contains("\"message\":"));
    assert!(!OLLAMA_MALFORMED.trim_end().ends_with('}'));

    assert!(DEEPSEEK_FAILURE.contains("\"error\""));
    assert!(OPENAI_FAILURE.contains("response.failed"));
    assert!(LLAMA_FAILURE.contains("\"error\""));

    for forbidden in ["authorization", "api_key", "credential_value"] {
        assert!(
            !INVENTORY.contains(forbidden),
            "inventory must not carry secret-bearing field {forbidden}"
        );
    }
}

fn json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture is valid JSON")
}

fn route<'a>(routes: &'a [Value], id: &str) -> &'a Value {
    routes
        .iter()
        .find(|route| route["id"] == id)
        .unwrap_or_else(|| panic!("missing route {id}"))
}

fn profile<'a>(route: &'a Value, operation: &str) -> &'a Value {
    route["ordinary_profiles"]
        .as_array()
        .expect("ordinary profiles")
        .iter()
        .find(|profile| profile["operation"] == operation)
        .unwrap_or_else(|| panic!("missing operation {operation}"))
}
