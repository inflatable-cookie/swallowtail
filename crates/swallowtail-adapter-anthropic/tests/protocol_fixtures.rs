mod support;

use serde_json::{Value, json};
use support::{
    FixtureEventKind, FixtureParseError, FixtureServer, MAX_FIXTURE_HTTP_BYTES,
    MAX_FIXTURE_STREAM_BYTES, exchange, parse_http_json, parse_sse,
};

const PROTOCOL: &str = include_str!("fixtures/anthropic-2023-06-01/protocol.json");
const PAGE_1: &str = include_str!("fixtures/anthropic-2023-06-01/models-page-1.json");
const PAGE_2: &str = include_str!("fixtures/anthropic-2023-06-01/models-page-2.json");
const REQUEST: &str = include_str!("fixtures/anthropic-2023-06-01/message-request.json");
const HEADERS: &str = include_str!("fixtures/anthropic-2023-06-01/response-headers.json");
const SUCCESS: &str = include_str!("fixtures/anthropic-2023-06-01/success.sse");
const MIDSTREAM_ERROR: &str = include_str!("fixtures/anthropic-2023-06-01/midstream-error.sse");
const UNKNOWN: &str = include_str!("fixtures/anthropic-2023-06-01/unknown-event.sse");
const DISCONNECT: &str = include_str!("fixtures/anthropic-2023-06-01/disconnect.sse");
const HTTP_ERRORS: &str = include_str!("fixtures/anthropic-2023-06-01/http-errors.json");

fn json_fixture(input: &str) -> Value {
    parse_http_json(input).expect("fixture JSON is valid and bounded")
}

#[test]
fn manifest_binds_public_api_key_audience_version_and_minimal_routes() {
    let fixture = json_fixture(PROTOCOL);
    assert_eq!(fixture["fixture_schema"], 1);
    assert_eq!(fixture["api_version"], "2023-06-01");
    assert_eq!(fixture["endpoint_audience"], "api.anthropic.com");
    assert_eq!(fixture["endpoint_source"], "host_approved_grant");
    assert_eq!(fixture["authentication"]["mechanism"], "api_key");
    assert_eq!(fixture["authentication"]["header"], "x-api-key");
    assert_eq!(
        fixture["authentication"]["subscription_oauth_in_subset"],
        false
    );
    assert_eq!(fixture["headers"]["beta_allowed"], false);
    assert_eq!(
        fixture["routes"],
        json!([["GET", "/v1/models"], ["POST", "/v1/messages"]])
    );
}

#[test]
fn catalogue_pages_preserve_cursor_identity_limits_and_unknown_limits() {
    let first = json_fixture(PAGE_1);
    let second = json_fixture(PAGE_2);
    assert_eq!(first["data"][0]["id"], "claude-fixture-primary");
    assert_eq!(first["data"][0]["max_input_tokens"], 123_456);
    assert_eq!(first["data"][0]["max_tokens"], 4_096);
    assert_eq!(first["has_more"], true);
    assert_eq!(first["last_id"], "claude-fixture-secondary");
    assert_eq!(second["has_more"], false);
    assert!(second["data"][0].get("max_input_tokens").is_none());
    assert!(second["data"][0].get("max_tokens").is_none());
    assert_eq!(
        json_fixture(PROTOCOL)["catalogue"]["unknown_limit_means"],
        "unknown"
    );
}

#[test]
fn message_request_requires_consumer_owned_output_bound_and_one_attempt() {
    let request = json_fixture(REQUEST);
    let protocol = json_fixture(PROTOCOL);
    assert_eq!(request["model"], "claude-fixture-primary");
    assert_eq!(request["max_tokens"], 64);
    assert_eq!(request["stream"], true);
    assert_eq!(protocol["message"]["maximum_inference_attempts"], 1);
    assert_eq!(
        protocol["message"]["max_tokens_source"],
        "consumer_owned_run_input"
    );
    assert_eq!(protocol["message"]["catalogue_default_allowed"], false);
    assert_eq!(protocol["message"]["retry_or_recovery_allowed"], false);
}

#[test]
fn success_stream_preserves_order_output_and_cumulative_usage() {
    let events = parse_sse(SUCCESS).expect("success stream parses");
    let names: Vec<_> = events.iter().map(|event| event.name()).collect();
    assert_eq!(
        names,
        [
            "message_start",
            "content_block_start",
            "ping",
            "content_block_delta",
            "content_block_delta",
            "content_block_stop",
            "message_delta",
            "message_stop"
        ]
    );
    let output = events
        .iter()
        .filter_map(|event| match event.kind() {
            FixtureEventKind::TextDelta(text) => Some(text.as_str()),
            _ => None,
        })
        .collect::<String>();
    assert_eq!(output, "Hello world");
    assert_eq!(events[0].data()["message"]["usage"]["output_tokens"], 1);
    assert_eq!(events[6].data()["usage"]["output_tokens"], 3);
}

#[test]
fn top_level_unknown_is_ignored_but_unknown_content_semantics_fail_closed() {
    let events = parse_sse(UNKNOWN).expect("forward-compatible stream parses");
    assert!(matches!(
        events[1].kind(),
        FixtureEventKind::UnknownTopLevel(name) if name == "future_transport_notice"
    ));
    let unknown_delta = concat!(
        "event: content_block_delta\n",
        "data: {\"type\":\"content_block_delta\",\"index\":0,",
        "\"delta\":{\"type\":\"future_delta\",\"value\":1}}\n\n"
    );
    assert_eq!(
        parse_sse(unknown_delta),
        Err(FixtureParseError::UnknownContentSemantics)
    );
    assert_eq!(
        json_fixture(PROTOCOL)["stream"]["unknown_top_level_event"],
        "ignore"
    );
}

#[test]
fn midstream_error_is_terminal_provider_failure_and_disconnect_is_protocol_failure() {
    let events = parse_sse(MIDSTREAM_ERROR).expect("error stream parses");
    assert!(matches!(
        events.last().expect("error exists").kind(),
        FixtureEventKind::ProviderError(kind) if kind == "overloaded_error"
    ));
    assert_eq!(
        parse_sse(DISCONNECT),
        Err(FixtureParseError::IncompleteFrame)
    );
    assert!(MIDSTREAM_ERROR.contains("raw-provider-message-must-not-enter-diagnostics"));
}

#[test]
fn request_id_rate_headers_and_http_error_types_remain_distinct_evidence() {
    let headers = json_fixture(HEADERS);
    assert_eq!(headers["request-id"], "req_fixture_success");
    for dimension in ["requests", "input-tokens", "output-tokens"] {
        for field in ["limit", "remaining", "reset"] {
            let name = format!("anthropic-ratelimit-{dimension}-{field}");
            assert!(headers.get(name).is_some(), "missing {dimension} {field}");
        }
    }
    let errors = json_fixture(HTTP_ERRORS);
    assert_eq!(errors.as_array().expect("errors are an array").len(), 8);
    assert!(
        errors
            .as_array()
            .expect("errors are an array")
            .iter()
            .any(|error| error == &json!([401, "authentication_error", "authentication_rejected"]))
    );
    assert!(
        errors
            .as_array()
            .expect("errors are an array")
            .iter()
            .any(|error| error == &json!([429, "rate_limit_error", "rate_limited"]))
    );
    assert!(
        errors
            .as_array()
            .expect("errors are an array")
            .iter()
            .any(|error| error == &json!([529, "overloaded_error", "provider_overloaded"]))
    );
}

#[test]
fn deterministic_endpoint_enforces_headers_pagination_and_single_message_attempt() {
    let server = FixtureServer::start();
    let first = exchange(server.endpoint(), &get_request("/v1/models?limit=2"));
    assert_eq!(first.status, 200);
    assert_eq!(first.headers["request-id"], "req_fixture_success");
    let first_body: Value = serde_json::from_slice(&first.body).expect("page parses");
    assert_eq!(first_body["has_more"], true);

    let second = exchange(
        server.endpoint(),
        &get_request("/v1/models?limit=2&after_id=claude-fixture-secondary"),
    );
    assert_eq!(second.status, 200);

    let message = exchange(server.endpoint(), &post_request(REQUEST.trim()));
    assert_eq!(message.status, 200);
    assert_eq!(message.headers["content-type"], "text/event-stream");
    assert!(parse_sse(std::str::from_utf8(&message.body).expect("SSE is utf-8")).is_ok());
    assert_eq!(server.inference_attempts(), 1);

    let requests = server.requests();
    assert_eq!(requests.len(), 3);
    assert!(requests.iter().all(|request| {
        request.headers.get("x-api-key").map(String::as_str) == Some("fixture-secret")
            && request.headers.get("anthropic-version").map(String::as_str) == Some("2023-06-01")
            && !request.headers.contains_key("anthropic-beta")
    }));
    assert_eq!(requests[2].body, REQUEST.trim().as_bytes());
}

#[test]
fn fixture_bounds_and_second_attempt_rejection_are_testable() {
    assert_eq!(
        parse_http_json(&"x".repeat(MAX_FIXTURE_HTTP_BYTES + 1)),
        Err(FixtureParseError::HttpTooLarge)
    );
    assert_eq!(
        parse_sse(&"x".repeat(MAX_FIXTURE_STREAM_BYTES + 1)),
        Err(FixtureParseError::StreamTooLarge)
    );

    let server = FixtureServer::start();
    assert_eq!(
        exchange(server.endpoint(), &post_request(REQUEST.trim())).status,
        200
    );
    assert_eq!(
        exchange(server.endpoint(), &post_request(REQUEST.trim())).status,
        409
    );
    assert_eq!(server.inference_attempts(), 2);
}

fn get_request(target: &str) -> Vec<u8> {
    format!(
        "GET {target} HTTP/1.1\r\nHost: fixture\r\nx-api-key: fixture-secret\r\nanthropic-version: 2023-06-01\r\nConnection: close\r\n\r\n"
    )
    .into_bytes()
}

fn post_request(body: &str) -> Vec<u8> {
    format!(
        "POST /v1/messages HTTP/1.1\r\nHost: fixture\r\nx-api-key: fixture-secret\r\nanthropic-version: 2023-06-01\r\ncontent-type: application/json\r\ncontent-length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .into_bytes()
}
