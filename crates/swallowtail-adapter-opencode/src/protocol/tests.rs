#[cfg(test)]
mod tests {
    use super::{
        Event, Response, SseDecoder, abort, parse_catalog, parse_event, parse_health, prompt,
        session_create,
    };

    const ROOT: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/opencode-1.14.48"
    );

    #[test]
    fn catalogue_keeps_provider_and_model_identity_separate() {
        let fixture: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(format!("{ROOT}/http-success.json")).expect("fixture reads"),
        )
        .expect("fixture parses");
        let responses = fixture.as_array().expect("fixture is an array");
        let health = serde_json::to_vec(&responses[0]["response"]["body"]).expect("serializes");
        parse_health(&Response {
            status: 200,
            body: health,
        })
        .expect("health parses");
        let body = serde_json::to_vec(&responses[1]["response"]["body"]).expect("serializes");
        let models = parse_catalog(&Response { status: 200, body }).expect("catalogue parses");
        assert_eq!(
            models[0].provider_id().expect("provider").as_str(),
            "anthropic"
        );
        assert_eq!(models[0].id().as_str(), "claude-sonnet");
        assert!(models[0].metadata().is_default());
        assert_eq!(
            models[0]
                .metadata()
                .token_limits()
                .expect("limits")
                .maximum_input_tokens(),
            Some(190_000)
        );
    }

    #[test]
    fn sse_is_incremental_and_duplicate_idle_is_visible() {
        let bytes = std::fs::read(format!("{ROOT}/success.sse")).expect("fixture reads");
        let mut decoder = SseDecoder::default();
        let mut frames = Vec::new();
        for chunk in bytes.chunks(11) {
            frames.extend(decoder.push(chunk).expect("chunk parses"));
        }
        decoder.finish().expect("stream finishes");
        let events: Vec<_> = frames
            .iter()
            .map(|frame| parse_event(frame, "ses_fixture").expect("event parses"))
            .collect();
        assert_eq!(
            events.iter().filter(|event| **event == Event::Idle).count(),
            2
        );
        assert!(events.contains(&Event::OutputDelta("hello".to_owned())));
        assert!(events.contains(&Event::OutputSnapshot("hello world".to_owned())));
    }

    #[test]
    fn partial_sse_frame_is_a_disconnect_not_provider_failure() {
        let bytes = std::fs::read(format!("{ROOT}/disconnect.sse")).expect("fixture reads");
        let mut decoder = SseDecoder::default();
        let _ = decoder.push(&bytes).expect("complete prefix parses");
        let failure = decoder.finish().expect_err("partial frame fails");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.opencode.sse_disconnected"
        );
    }

    #[test]
    fn provider_payload_is_not_copied_into_failure() {
        let bytes = std::fs::read(format!("{ROOT}/provider-error.sse")).expect("fixture reads");
        let mut decoder = SseDecoder::default();
        let frames = decoder.push(&bytes).expect("fixture parses");
        let event = parse_event(&frames[1], "ses_fixture").expect("event parses");
        assert_eq!(event, Event::ProviderFailed);
        assert!(!format!("{event:?}").contains("raw-secret-error-sentinel"));
    }

    #[test]
    fn production_requests_match_the_frozen_route_and_permission_subset() {
        let create = session_create("anthropic", "claude-sonnet", "/workspace/fixture");
        assert_eq!(create.path, "/session");
        assert_eq!(
            create.query,
            vec![("directory".to_owned(), "/workspace/fixture".to_owned())]
        );
        let body: serde_json::Value =
            serde_json::from_slice(create.body.as_ref().expect("create has body"))
                .expect("body parses");
        assert_eq!(body["model"]["providerID"], "anthropic");
        assert_eq!(body["model"]["id"], "claude-sonnet");
        assert_eq!(body["permission"][0]["permission"], "*");
        assert_eq!(body["permission"][0]["action"], "deny");
        assert_eq!(body["permission"][1]["permission"], "read");
        assert_eq!(body["permission"][2]["permission"], "glob");
        assert_eq!(body["permission"][3]["permission"], "grep");

        let prompt = prompt(
            "ses_fixture",
            "anthropic",
            "claude-sonnet",
            "/workspace/fixture",
            "private prompt",
        );
        assert_eq!(prompt.path, "/session/ses_fixture/prompt_async");
        let body: serde_json::Value =
            serde_json::from_slice(prompt.body.as_ref().expect("prompt has body"))
                .expect("body parses");
        assert_eq!(body["model"]["modelID"], "claude-sonnet");
        assert_eq!(body["parts"][0]["text"], "private prompt");

        let abort = abort("ses_fixture", "/workspace/fixture");
        assert_eq!(abort.path, "/session/ses_fixture/abort");
        assert!(abort.body.is_none());
    }

    #[test]
    fn foreign_events_are_quarantined_and_provider_requests_fail_closed() {
        let foreign = br#"{"id":"evt","type":"session.idle","properties":{"sessionID":"other"}}"#;
        assert_eq!(
            parse_event(foreign, "ses_fixture").expect("foreign event parses"),
            Event::Foreign
        );
        let permission =
            br#"{"id":"evt","type":"permission.asked","properties":{"sessionID":"ses_fixture"}}"#;
        assert_eq!(
            parse_event(permission, "ses_fixture").expect("permission parses"),
            Event::StopAndAbort
        );
        let unknown =
            br#"{"id":"evt","type":"provider.future","properties":{"sessionID":"ses_fixture"}}"#;
        let error = parse_event(unknown, "ses_fixture").expect_err("unknown event fails");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.event_unknown"
        );
    }
}

