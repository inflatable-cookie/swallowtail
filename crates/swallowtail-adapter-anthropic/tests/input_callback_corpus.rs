use serde_json::{Value, json};

const CORPUS: &str = include_str!("fixtures/anthropic-2023-06-01/input-callback-corpus.json");
const IMAGE: &str = include_str!("fixtures/anthropic-2023-06-01/image-request.json");
const TOOL: &str = include_str!("fixtures/anthropic-2023-06-01/client-tool-request.json");
const TOOL_RESULT: &str =
    include_str!("fixtures/anthropic-2023-06-01/client-tool-result-request.json");
const SEARCH: &str = include_str!("fixtures/anthropic-2023-06-01/web-search-request.json");

fn fixture(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture parses")
}

#[test]
fn image_input_is_model_gated_bounded_base64_without_path_authority() {
    let corpus = fixture(CORPUS);
    let request = fixture(IMAGE);
    let source = &request["messages"][0]["content"][0]["source"];

    assert_eq!(corpus["attachment"]["maximum_count"], 1);
    assert_eq!(corpus["attachment"]["maximum_bytes"], 1_048_576);
    assert_eq!(corpus["attachment"]["model_capability_required"], true);
    assert_eq!(source["type"], "base64");
    assert_eq!(source["media_type"], "image/png");
    assert!(source["data"].as_str().is_some());
    assert!(source.get("path").is_none());
    assert!(source.get("url").is_none());
}

#[test]
fn client_tool_result_authorizes_a_separate_correlated_attempt() {
    let corpus = fixture(CORPUS);
    let first = fixture(TOOL);
    let continuation = fixture(TOOL_RESULT);

    assert_eq!(
        corpus["client_tools"]["operation_shape"],
        "consumer_owned_direct_continuation"
    );
    assert_eq!(corpus["client_tools"]["maximum_attempts"], 3);
    assert_eq!(corpus["client_tools"]["consumer_executes"], true);
    assert_eq!(corpus["client_tools"]["implicit_next_attempt"], false);
    assert_eq!(first["tools"][0]["name"], "lookup_customer");
    assert_eq!(
        continuation["messages"][1]["content"][0]["id"],
        "toolu_fixture_1"
    );
    assert_eq!(
        continuation["messages"][2]["content"][0]["tool_use_id"],
        "toolu_fixture_1"
    );
}

#[test]
fn provider_search_is_not_a_consumer_tool_callback() {
    let corpus = fixture(CORPUS);
    let request = fixture(SEARCH);

    assert_eq!(
        request["tools"][0]["type"],
        corpus["external_search"]["tool_type"]
    );
    assert_eq!(request["tools"][0]["max_uses"], 2);
    assert_eq!(corpus["external_search"]["execution_source"], "provider");
    assert_eq!(corpus["external_search"]["explicit_policy_required"], true);
    assert_eq!(
        corpus["mixed_tool_rule"]["client_tool_result_port_accepts_provider_search_result"],
        false
    );
    assert!(request["tools"][0].get("input_schema").is_none());
}

#[test]
fn rejection_cancellation_and_credential_last_cleanup_are_frozen() {
    let corpus = fixture(CORPUS);

    assert!(
        corpus["rejections"]
            .as_array()
            .expect("rejections are an array")
            .contains(&json!("search_not_enabled_for_organization"))
    );
    assert_eq!(corpus["cancellation"]["pending_tool_results"], "abandoned");
    assert_eq!(
        corpus["cleanup"],
        json!([
            "abandon_pending_tool_results",
            "close_network_work",
            "zeroize_private_continuation",
            "join_tasks_and_timers",
            "release_attachment",
            "release_credential"
        ])
    );
}
