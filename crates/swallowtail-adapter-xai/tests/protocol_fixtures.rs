mod support;

use serde_json::{Value, json};
use support::{
    Conversation, Event, FixtureError, FixtureServer, MAX_FRAME_BYTES, ProviderFailure,
    ServerScenario, TurnEvidence, authenticated_connect, connect_request, parse_event, read_turn,
};
use swallowtail_adapter_xai::{RESPONSES_WEBSOCKET_PATH, USD_TICKS_PER_USD};
use tungstenite::Message;

const PROTOCOL: &str = include_str!("fixtures/xai-responses-websocket-2026-04-23/protocol.json");
const FIRST_REQUEST: &str =
    include_str!("fixtures/xai-responses-websocket-2026-04-23/first-turn.json");
const CHAINED_REQUEST: &str =
    include_str!("fixtures/xai-responses-websocket-2026-04-23/chained-turn.json");
const FIRST_EVENTS: &str =
    include_str!("fixtures/xai-responses-websocket-2026-04-23/first-turn-events.ndjson");
const SECOND_EVENTS: &str =
    include_str!("fixtures/xai-responses-websocket-2026-04-23/second-turn-events.ndjson");
const PREVIOUS_NOT_FOUND: &str =
    include_str!("fixtures/xai-responses-websocket-2026-04-23/previous-response-not-found.json");
const CONNECTION_LIMIT: &str =
    include_str!("fixtures/xai-responses-websocket-2026-04-23/connection-limit.json");
const UNKNOWN: &str =
    include_str!("fixtures/xai-responses-websocket-2026-04-23/unknown-event.json");

fn fixture_json(input: &str) -> Value {
    serde_json::from_str(input).expect("fixture JSON parses")
}

#[test]
fn manifest_is_an_evidence_snapshot_with_exact_route_auth_and_exclusions() {
    let manifest = fixture_json(PROTOCOL);
    assert_eq!(manifest["fixture_schema"], 1);
    assert_eq!(manifest["evidence_snapshot"], "2026-07-20");
    assert_eq!(manifest["source_guide_updated"], "2026-04-23");
    assert_eq!(manifest["provider_api_version_claimed"], false);
    assert_eq!(manifest["endpoint_audience"], "api.x.ai");
    assert_eq!(manifest["transport"]["path"], RESPONSES_WEBSOCKET_PATH);
    assert_eq!(manifest["authentication"]["mechanism"], "api_key_bearer");
    assert_eq!(manifest["request"]["store"], false);
    assert_eq!(
        manifest["request"]["continuation"],
        "adapter_private_latest_successful_response"
    );
    assert_eq!(manifest["chain"]["caller_can_supply"], false);
    assert_eq!(manifest["chain"]["reconnect"], false);
    assert_eq!(manifest["cancellation"]["text_cancel_frame"], Value::Null);
    assert!(
        manifest["excluded"]
            .as_array()
            .expect("exclusions exist")
            .contains(&json!("live_authentication"))
    );
}

#[test]
fn first_and_chained_requests_are_store_disabled_and_continuation_is_internal() {
    let mut conversation = Conversation::new("grok-fixture-exact");
    let first = conversation
        .begin_turn("First fixture turn.")
        .expect("first turn begins");
    assert_eq!(fixture_json(&first), fixture_json(FIRST_REQUEST));
    apply_lines(&mut conversation, FIRST_EVENTS);

    let second = conversation
        .begin_turn("Second fixture turn.")
        .expect("second turn begins");
    assert_eq!(fixture_json(&second), fixture_json(CHAINED_REQUEST));
    assert_eq!(
        fixture_json(&second)["previous_response_id"],
        "resp_fixture_first"
    );
    assert!(SECOND_EVENTS.contains("resp_fixture_second"));
}

#[test]
fn ordered_events_preserve_output_and_replace_cumulative_evidence() {
    let mut conversation = Conversation::new("grok-fixture-exact");
    conversation.begin_turn("First fixture turn.").unwrap();
    let frames: Vec<_> = FIRST_EVENTS.lines().collect();
    conversation.apply(parse_event(frames[0]).unwrap()).unwrap();
    conversation
        .apply(Event::InProgress {
            response_id: "resp_fixture_first".to_owned(),
            usage: Some(TurnEvidence {
                input_tokens: 4,
                output_tokens: 1,
                total_tokens: 5,
                cost_in_usd_ticks: 100_000,
            }),
        })
        .unwrap();
    assert_eq!(
        conversation.latest_evidence().unwrap().cost_in_usd_ticks,
        100_000
    );
    for frame in &frames[2..] {
        conversation.apply(parse_event(frame).unwrap()).unwrap();
    }
    let final_evidence = conversation.latest_evidence().unwrap();
    assert_eq!(final_evidence.input_tokens, 5);
    assert_eq!(final_evidence.output_tokens, 3);
    assert_eq!(final_evidence.total_tokens, 8);
    assert_eq!(final_evidence.cost_in_usd_ticks, 125_000);
    assert_eq!(USD_TICKS_PER_USD, 10_000_000_000);
}

#[test]
fn errors_unknown_events_bounds_correlation_and_output_mismatch_fail_closed() {
    assert_eq!(
        parse_event(PREVIOUS_NOT_FOUND),
        Ok(Event::ProviderFailed(
            ProviderFailure::PreviousResponseNotFound
        ))
    );
    assert_eq!(
        parse_event(CONNECTION_LIMIT),
        Ok(Event::ProviderFailed(
            ProviderFailure::ConnectionLimitReached
        ))
    );
    assert_eq!(parse_event(UNKNOWN), Err(FixtureError::UnknownEvent));
    assert_eq!(
        parse_event(&"x".repeat(MAX_FRAME_BYTES + 1)),
        Err(FixtureError::FrameTooLarge)
    );

    let mut conversation = Conversation::new("grok-fixture-exact");
    conversation.begin_turn("fixture").unwrap();
    conversation
        .apply(Event::Created {
            response_id: "expected".to_owned(),
        })
        .unwrap();
    assert_eq!(
        conversation.apply(Event::TextDelta {
            response_id: "other".to_owned(),
            delta: "private".to_owned(),
        }),
        Err(FixtureError::CorrelationFailed)
    );

    let debug = format!("{:?}", parse_event(PREVIOUS_NOT_FOUND).unwrap());
    assert!(!debug.contains("raw response id"));
    assert!(!debug.contains("provider message"));
}

#[test]
fn loopback_endpoint_enforces_upgrade_route_bearer_shape_and_serial_turns() {
    let server = FixtureServer::start(ServerScenario::Success);
    let mut socket = authenticated_connect(&server.endpoint());
    let mut conversation = Conversation::new("grok-fixture-exact");

    let first = conversation.begin_turn("First fixture turn.").unwrap();
    socket.send(Message::Text(first.into())).unwrap();
    read_turn(&mut socket, &mut conversation);
    let second = conversation.begin_turn("Second fixture turn.").unwrap();
    socket.send(Message::Text(second.into())).unwrap();
    read_turn(&mut socket, &mut conversation);

    assert_eq!(
        server.handshake(),
        Some((
            RESPONSES_WEBSOCKET_PATH.to_owned(),
            Some("Bearer fixture-secret".to_owned())
        ))
    );
    assert_eq!(server.frames().len(), 2);
    assert_eq!(
        conversation.latest_evidence().unwrap().cost_in_usd_ticks,
        175_000
    );
}

#[test]
fn bad_route_or_bearer_shape_is_rejected_during_upgrade() {
    let bad_auth = FixtureServer::start(ServerScenario::Success);
    assert!(connect_request(&bad_auth.endpoint(), "Basic fixture-secret").is_err());
    assert_eq!(
        bad_auth.handshake().unwrap().1.as_deref(),
        Some("Basic fixture-secret")
    );

    let bad_route = FixtureServer::start(ServerScenario::Success);
    let endpoint = bad_route
        .endpoint()
        .replace(RESPONSES_WEBSOCKET_PATH, "/v1/chat/completions");
    assert!(connect_request(&endpoint, "Bearer fixture-secret").is_err());
    assert_eq!(bad_route.handshake().unwrap().0, "/v1/chat/completions");
}

#[test]
fn concurrent_turn_is_rejected_before_a_second_provider_frame() {
    let server = FixtureServer::start(ServerScenario::WaitForClientClose);
    let mut socket = authenticated_connect(&server.endpoint());
    let mut conversation = Conversation::new("grok-fixture-exact");
    let first = conversation.begin_turn("First fixture turn.").unwrap();
    socket.send(Message::Text(first.into())).unwrap();
    server.wait_for_frames(1);
    assert_eq!(
        conversation.begin_turn("must not be sent"),
        Err(FixtureError::TurnActive)
    );
    socket.close(None).unwrap();
    assert_eq!(server.frames().len(), 1);
}

#[test]
fn cancellation_is_websocket_close_and_disconnect_invalidates_the_chain() {
    let cancel_server = FixtureServer::start(ServerScenario::WaitForClientClose);
    let mut cancel_socket = authenticated_connect(&cancel_server.endpoint());
    let mut cancelled = Conversation::new("grok-fixture-exact");
    let request = cancelled.begin_turn("cancel fixture").unwrap();
    cancel_socket.send(Message::Text(request.into())).unwrap();
    cancel_server.wait_for_frames(1);
    cancel_socket.close(None).unwrap();
    assert_eq!(cancelled.disconnect(), Err(FixtureError::IncompleteTurn));
    assert_eq!(cancel_server.frames().len(), 1);
    assert_eq!(
        fixture_json(&cancel_server.frames()[0])["type"],
        "response.create"
    );

    let disconnect_server = FixtureServer::start(ServerScenario::Disconnect);
    let mut disconnect_socket = authenticated_connect(&disconnect_server.endpoint());
    let mut disconnected = Conversation::new("grok-fixture-exact");
    let request = disconnected.begin_turn("First fixture turn.").unwrap();
    disconnect_socket
        .send(Message::Text(request.into()))
        .unwrap();
    for _ in 0..2 {
        let Message::Text(frame) = disconnect_socket.read().unwrap() else {
            panic!("fixture event is text");
        };
        disconnected.apply(parse_event(&frame).unwrap()).unwrap();
    }
    assert!(disconnect_socket.read().is_err());
    assert_eq!(disconnected.disconnect(), Err(FixtureError::IncompleteTurn));
    assert_eq!(
        disconnected.begin_turn("cannot resume"),
        Err(FixtureError::ChainInvalid)
    );
}

fn apply_lines(conversation: &mut Conversation, input: &str) {
    for frame in input.lines() {
        conversation.apply(parse_event(frame).unwrap()).unwrap();
    }
}
