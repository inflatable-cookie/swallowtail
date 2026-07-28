use serde_json::Value;

#[test]
fn background_fixture_keeps_generation_controls_distinct() {
    let request: Value = serde_json::from_str(include_str!(
        "fixtures/openai-responses-2026-07-21/generation-controls-request.json"
    ))
    .expect("fixture parses");

    assert_eq!(request["max_output_tokens"], 64);
    assert_eq!(request["reasoning"]["effort"], "high");
    assert_eq!(request["text"]["format"]["type"], "json_schema");
    assert_eq!(request["text"]["format"]["strict"], true);
}

#[test]
fn realtime_fixture_places_output_limit_on_session_configuration() {
    let update: Value = serde_json::from_str(include_str!(
        "fixtures/openai-realtime-2026-07-22/generation-controls-session-update.json"
    ))
    .expect("fixture parses");

    assert_eq!(update["type"], "session.update");
    assert_eq!(update["session"]["max_output_tokens"], 512);
    assert!(update["session"].get("reasoning").is_none());
    assert!(update["session"].get("text").is_none());
}
