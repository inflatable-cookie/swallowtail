mod support;

use serde_json::{Value, json};
use support::{
    EventDecision, FixtureError, MAX_FIXTURE_HTTP_BYTES, MAX_FIXTURE_STREAM_BYTES, decide,
    event_type, parse_http_json, parse_sse, session_id,
};

const ROOT: &str = "fixtures/opencode-1.14.48";
const PROTOCOL: &str = include_str!("fixtures/opencode-1.14.48/protocol.json");
const HTTP_SUCCESS: &str = include_str!("fixtures/opencode-1.14.48/http-success.json");
const HTTP_ERROR: &str = include_str!("fixtures/opencode-1.14.48/http-error.json");
const SUCCESS: &str = include_str!("fixtures/opencode-1.14.48/success.sse");
const PROVIDER_ERROR: &str = include_str!("fixtures/opencode-1.14.48/provider-error.sse");
const ABORTED: &str = include_str!("fixtures/opencode-1.14.48/aborted.sse");
const PROVIDER_REQUESTS: &str = include_str!("fixtures/opencode-1.14.48/provider-requests.sse");
const UNKNOWN_EVENT: &str = include_str!("fixtures/opencode-1.14.48/unknown-event.sse");
const DISCONNECT: &str = include_str!("fixtures/opencode-1.14.48/disconnect.sse");

fn json_fixture(input: &str) -> Value {
    parse_http_json(input).expect("fixture JSON is valid and bounded")
}

#[test]
fn manifest_freezes_only_the_versioned_attached_subset() {
    let fixture = json_fixture(PROTOCOL);
    assert_eq!(fixture["fixture_schema"], 1);
    assert_eq!(fixture["observed_server_version"], "1.14.48");
    assert_eq!(fixture["openapi_version"], "3.1.0");
    assert_eq!(fixture["endpoint_source"], "host_approved_grant");
    assert_eq!(fixture["default_port_assumption"], false);
    assert_eq!(fixture["provider_authentication"], "delegated_to_opencode");
    assert_eq!(fixture["provider_default_is_evidence_only"], true);

    let routes = fixture["routes"].as_array().expect("routes are an array");
    assert_eq!(routes.len(), 6);
    for forbidden in ["/auth", "/config", "/instance/dispose", "/share"] {
        assert!(
            !routes.iter().any(|route| route[2]
                .as_str()
                .expect("fixture route has a path")
                .contains(forbidden)),
            "{ROOT} unexpectedly contains {forbidden}"
        );
    }
    assert_eq!(fixture["attached_close"]["owns_server"], false);
    assert_eq!(fixture["attached_close"]["requests"], json!([]));
}

#[test]
fn http_transcript_keeps_provider_model_policy_and_route_exact() {
    let fixture = json_fixture(HTTP_SUCCESS);
    let exchanges = fixture.as_array().expect("HTTP fixture is an array");
    assert_eq!(exchanges.len(), 5);
    assert_eq!(exchanges[0]["response"]["body"]["version"], "1.14.48");

    let provider = &exchanges[1]["response"]["body"]["all"][0];
    let model = &provider["models"]["claude-sonnet"];
    assert_eq!(provider["id"], "anthropic");
    assert_eq!(model["id"], "claude-sonnet");
    assert_eq!(model["providerID"], "anthropic");
    assert_eq!(model["limit"]["input"], 190_000);
    assert_eq!(model["limit"]["output"], 10_000);

    let protocol = json_fixture(PROTOCOL);
    let create = &exchanges[2]["request"]["body"];
    assert_eq!(create["permission"], protocol["session_permission"]);
    assert_eq!(create["model"]["providerID"], "anthropic");
    assert_eq!(create["model"]["id"], "claude-sonnet");
    let prompt = &exchanges[3]["request"]["body"];
    assert_eq!(prompt["model"]["providerID"], "anthropic");
    assert_eq!(prompt["model"]["modelID"], "claude-sonnet");
    assert_eq!(exchanges[3]["response"]["status"], 204);
    assert_eq!(exchanges[4]["request"]["method"], "POST");
    assert_eq!(exchanges[4]["response"]["body"], true);
}

#[test]
fn ordered_success_stream_is_correlated_and_terminal() {
    let events = parse_sse(SUCCESS).expect("success stream parses");
    let types: Vec<_> = events.iter().map(event_type).collect();
    assert_eq!(
        types,
        [
            "server.connected",
            "session.status",
            "message.part.delta",
            "message.part.updated",
            "session.status",
            "session.idle"
        ]
    );
    assert!(
        events[1..]
            .iter()
            .all(|event| session_id(event) == Some("ses_fixture"))
    );
    assert_eq!(decide(&events[4]), EventDecision::Completed);
    assert_eq!(decide(&events[5]), EventDecision::Completed);
}

#[test]
fn abort_provider_failure_and_http_failure_remain_distinct() {
    let aborted = parse_sse(ABORTED).expect("aborted stream parses");
    let failed = parse_sse(PROVIDER_ERROR).expect("provider error stream parses");
    assert_eq!(decide(&aborted[1]), EventDecision::Aborted);
    assert_eq!(decide(&failed[1]), EventDecision::ProviderFailed);

    let http = json_fixture(HTTP_ERROR);
    assert_eq!(http["response"]["status"], 404);
    assert!(HTTP_ERROR.contains("raw-provider-payload-must-not-enter-diagnostics"));
    assert!(PROVIDER_ERROR.contains("raw-secret-error-sentinel"));
}

#[test]
fn provider_requests_stop_and_unknown_events_fail_closed() {
    let requests = parse_sse(PROVIDER_REQUESTS).expect("provider requests parse");
    assert_eq!(requests.len(), 2);
    assert!(
        requests
            .iter()
            .all(|event| decide(event) == EventDecision::StopAndAbort)
    );

    let unknown = parse_sse(UNKNOWN_EVENT).expect("unknown event parses");
    assert_eq!(decide(&unknown[0]), EventDecision::ProtocolFailed);
    let protocol = json_fixture(PROTOCOL);
    assert_eq!(protocol["event_policy"]["unknown"], "protocol_failure");
}

#[test]
fn disconnect_and_bounds_fail_before_partial_payload_use() {
    assert_eq!(parse_sse(DISCONNECT), Err(FixtureError::IncompleteFrame));
    let oversized = "x".repeat(MAX_FIXTURE_STREAM_BYTES + 1);
    assert_eq!(parse_sse(&oversized), Err(FixtureError::StreamTooLarge));
    let oversized_http = "x".repeat(MAX_FIXTURE_HTTP_BYTES + 1);
    assert_eq!(
        parse_http_json(&oversized_http),
        Err(FixtureError::HttpTranscriptTooLarge)
    );
}
