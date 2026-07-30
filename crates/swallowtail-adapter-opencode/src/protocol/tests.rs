#[cfg(test)]
mod tests {
    use super::{
        Event, Response, SessionDeleteResponse, SseDecoder, abort, classify_session_delete,
        callback_response, observe_health, parse_catalog, parse_event, parse_session_for_version,
        prompt, session_create, session_delete, PromptPayload, ProviderRequestKind,
    };
    use crate::selection::opencode_server_binding;
    use swallowtail_core::{InterfaceCompatibilityAssessment, ReasoningMode};
    use swallowtail_runtime::{
        CallbackPayload, CallbackResult, SchemaDocument, StructuredOutputDescriptor,
    };

    const ROOT: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/opencode-1.14.48"
    );
    const RANGE_ROOT: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/opencode-v1.14.48-v1.18.4"
    );

    #[test]
    fn catalogue_keeps_provider_and_model_identity_separate() {
        let fixture: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(format!("{ROOT}/http-success.json")).expect("fixture reads"),
        )
        .expect("fixture parses");
        let responses = fixture.as_array().expect("fixture is an array");
        let health = serde_json::to_vec(&responses[0]["response"]["body"]).expect("serializes");
        observe_health(&Response {
            status: 200,
            body: health,
            next_cursor: None,
        })
        .expect("health parses");
        let body = serde_json::to_vec(&responses[1]["response"]["body"]).expect("serializes");
        let models = parse_catalog(&Response {
            status: 200,
            body,
            next_cursor: None,
        })
        .expect("catalogue parses");
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
        assert!(models[0]
            .metadata()
            .reasoning()
            .expect("reasoning evidence exists")
            .supports(&ReasoningMode::new("high").expect("mode is valid")));
        assert_eq!(
            models[0]
                .metadata()
                .catalog_observations()
                .expect("catalogue observations exist")
                .tool_calling_supported(),
            Some(true)
        );
    }

    #[test]
    fn health_observation_is_exact_classified_and_safe() {
        let response = fixture_response("health-supported.json");
        let observation = observe_health(&response).expect("candidate health is supported");
        assert_eq!(observation.binding().axis().as_str(), "opencode.server");
        assert_eq!(observation.binding().version().as_str(), "1.18.4");
        assert_eq!(
            observation
                .assessment()
                .behavior_revision()
                .expect("qualified observation has behavior")
                .as_str(),
            "opencode.http-sse.surface-18"
        );
        assert!(matches!(
            observation.assessment(),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
        let formatted = format!("{observation:?}");
        assert!(!formatted.contains("endpoint"));
        assert!(!formatted.contains("credential"));

    }

    #[test]
    fn newer_stable_health_is_permitted_but_unverified() {
        let observation =
            observe_health(&fixture_response("health-above.json")).expect("newer version permits");
        let InterfaceCompatibilityAssessment::UnverifiedNewer(unverified) =
            observation.assessment()
        else {
            panic!("newer stable version must remain unverified");
        };
        assert_eq!(unverified.version().as_str(), "1.18.5");
        assert_eq!(unverified.latest_qualified().as_str(), "1.18.4");
        assert_eq!(
            unverified.behavior_revision().as_str(),
            "opencode.http-sse.surface-18"
        );
    }

    #[test]
    fn incompatible_and_invalid_health_envelopes_fail_closed() {
        for fixture in ["health-below.json", "health-prerelease.json"] {
            let error = observe_health(&fixture_response(fixture)).expect_err("version rejects");
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.opencode.version_unsupported"
            );
        }
        let malformed =
            observe_health(&fixture_response("health-malformed.json")).expect_err("malformed");
        assert_eq!(
            malformed.diagnostic().code(),
            "swallowtail.opencode.version_invalid"
        );
        let missing =
            observe_health(&fixture_response("health-missing.json")).expect_err("missing");
        assert_eq!(
            missing.diagnostic().code(),
            "swallowtail.opencode.protocol_invalid"
        );
        let unhealthy =
            observe_health(&fixture_response("health-unhealthy.json")).expect_err("unhealthy");
        assert_eq!(
            unhealthy.diagnostic().code(),
            "swallowtail.opencode.unhealthy"
        );
        let unpublished = observe_health(&Response {
            status: 200,
            body: br#"{"healthy":true,"version":"1.15.8"}"#.to_vec(),
            next_cursor: None,
        })
        .expect_err("unpublished semantic gap rejects");
        assert_eq!(
            unpublished.diagnostic().code(),
            "swallowtail.opencode.version_unsupported"
        );
    }

    #[test]
    fn session_version_must_match_the_exact_health_binding() {
        let expected = opencode_server_binding("1.18.4").expect("expected version is safe");
        let error = parse_session_for_version(
            &fixture_response("session-version-mismatch.json"),
            &expected,
        )
        .expect_err("session drift rejects");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.session_invalid"
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
        assert!(events.contains(&Event::OutputDelta {
            message_id: "msg_fixture".to_owned(),
            part_id: "prt_fixture".to_owned(),
            text: "hello".to_owned(),
        }));
        assert!(events.contains(&Event::OutputSnapshot {
            message_id: "msg_fixture".to_owned(),
            part_id: "prt_fixture".to_owned(),
            text: "hello world".to_owned(),
        }));
        let usage = events
            .iter()
            .filter_map(|event| match event {
                Event::Usage(part_id, usage) => Some((part_id.as_str(), usage)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(usage.len(), 2);
        assert_eq!(usage[0].0, "prt_usage");
        assert_eq!(usage[0].1.input_tokens(), Some(12));
        assert_eq!(usage[0].1.output_tokens(), Some(4));
        assert_eq!(usage[0].1.reasoning_tokens(), Some(2));
        assert_eq!(usage[0].1.cache_read_input_tokens(), Some(3));
        assert_eq!(usage[0].1.cache_write_input_tokens(), Some(1));
    }

    #[test]
    fn malformed_usage_components_and_identity_fail_closed() {
        for frame in [
            br#"{"type":"message.part.updated","properties":{"sessionID":"ses_fixture","part":{"id":"","type":"step-finish","tokens":{"input":1,"output":1,"reasoning":1,"cache":{"read":0,"write":0}}}}}"#.as_slice(),
            br#"{"type":"message.part.updated","properties":{"sessionID":"ses_fixture","part":{"id":"part","type":"step-finish","tokens":{"input":-1,"output":1,"reasoning":1,"cache":{"read":0,"write":0}}}}}"#.as_slice(),
            br#"{"type":"message.part.updated","properties":{"sessionID":"ses_fixture","part":{"id":"part","type":"step-finish","tokens":{"input":1.5,"output":1,"reasoning":1,"cache":{"read":0,"write":0}}}}}"#.as_slice(),
            br#"{"type":"message.part.updated","properties":{"sessionID":"ses_fixture","part":{"id":"part","type":"step-finish","tokens":{"input":1,"output":1,"cache":{"read":0,"write":0}}}}}"#.as_slice(),
        ] {
            let error = parse_event(frame, "ses_fixture").expect_err("usage rejects");
            assert_eq!(error.diagnostic().code(), "swallowtail.opencode.event_invalid");
        }
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
        let create = session_create("anthropic", "claude-sonnet", "/workspace/fixture", false);
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
            PromptPayload {
                content: "private prompt",
                reasoning: None,
                structured_output: None,
                file: None,
            },
        )
        .expect("prompt encodes");
        assert_eq!(prompt.path, "/session/ses_fixture/prompt_async");
        let body: serde_json::Value =
            serde_json::from_slice(prompt.body.as_ref().expect("prompt has body"))
                .expect("body parses");
        assert_eq!(body["model"]["modelID"], "claude-sonnet");
        assert_eq!(body["parts"][0]["text"], "private prompt");

        let abort = abort("ses_fixture", "/workspace/fixture");
        assert_eq!(abort.path, "/session/ses_fixture/abort");
        assert!(abort.body.is_none());

        let delete = session_delete("ses_fixture", "/workspace/fixture").expect("id is safe");
        assert_eq!(delete.path, "/session/ses_fixture");
        assert_eq!(
            delete.query,
            vec![("directory".to_owned(), "/workspace/fixture".to_owned())]
        );
        assert!(delete.body.is_none());
        assert!(
            session_delete("ses/unsafe", "/workspace/fixture").is_err(),
            "provider identity cannot escape its path segment"
        );
    }

    #[test]
    fn generation_controls_match_the_frozen_prompt_shape() {
        let schema = StructuredOutputDescriptor::new(
            SchemaDocument::inline(
                br#"{"type":"object","properties":{"result":{"type":"string"}},"required":["result"],"additionalProperties":false}"#,
                4096,
            )
            .expect("schema is bounded"),
            "application/schema+json",
            "json-schema-2020-12",
        )
        .expect("schema descriptor is valid");
        let reasoning = ReasoningMode::new("high").expect("reasoning is valid");
        let request = prompt(
            "ses_fixture",
            "fixture-provider",
            "fixture-model",
            "/workspace/fixture",
            PromptPayload {
                content: "Return one fixture result",
                reasoning: Some(&reasoning),
                structured_output: Some(&schema),
                file: None,
            },
        )
        .expect("generation controls encode");
        let actual: serde_json::Value =
            serde_json::from_slice(request.body.as_ref().expect("prompt has body"))
                .expect("prompt body is JSON");
        let expected: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(format!(
            "{RANGE_ROOT}/generation-controls-prompt-request.json"
        ))
        .expect("fixture reads"))
        .expect("fixture parses");
        assert_eq!(actual, expected);
    }

    #[test]
    fn provider_callback_responses_preserve_one_shot_and_ordered_question_bounds() {
        let once = callback_response(
            "per_fixture",
            ProviderRequestKind::Permission,
            &CallbackResult::Success(
                CallbackPayload::new(br#"{"reply":"once"}"#, 256).expect("payload is bounded"),
            ),
        )
        .expect("one-shot response encodes");
        assert_eq!(once.path, "/permission/per_fixture/reply");
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(once.body.as_ref().unwrap()).unwrap(),
            serde_json::json!({"reply":"once"})
        );
        let persistent = callback_response(
            "per_fixture",
            ProviderRequestKind::Permission,
            &CallbackResult::Success(
                CallbackPayload::new(br#"{"reply":"always"}"#, 256).expect("payload is bounded"),
            ),
        )
        .expect_err("persistent response rejects");
        assert_eq!(
            persistent.diagnostic().code(),
            "swallowtail.opencode.callback_malformed"
        );

        let answers = callback_response(
            "que_fixture",
            ProviderRequestKind::Question { count: 1 },
            &CallbackResult::Success(
                CallbackPayload::new(br#"{"answers":[["Safe"]]}"#, 256)
                    .expect("payload is bounded"),
            ),
        )
        .expect("ordered answers encode");
        assert_eq!(answers.path, "/question/que_fixture/reply");
        let wrong_count = callback_response(
            "que_fixture",
            ProviderRequestKind::Question { count: 2 },
            &CallbackResult::Success(
                CallbackPayload::new(br#"{"answers":[["Safe"]]}"#, 256)
                    .expect("payload is bounded"),
            ),
        )
        .expect_err("answer count must match question count");
        assert_eq!(
            wrong_count.diagnostic().code(),
            "swallowtail.opencode.callback_malformed"
        );
    }

    #[test]
    fn delete_response_classification_never_trusts_provider_payloads() {
        assert_eq!(
            classify_session_delete(&Response {
                status: 200,
                body: b"true".to_vec(),
                next_cursor: None,
            }),
            SessionDeleteResponse::Applied
        );
        for status in [400, 401, 404] {
            assert_eq!(
                classify_session_delete(&Response {
                    status,
                    body: br#"{"private":"provider detail"}"#.to_vec(),
                    next_cursor: None,
                }),
                SessionDeleteResponse::Rejected
            );
        }
        for response in [
            Response {
                status: 200,
                body: b"false".to_vec(),
                next_cursor: None,
            },
            Response {
                status: 500,
                body: br#"{"private":"provider detail"}"#.to_vec(),
                next_cursor: None,
            },
        ] {
            assert_eq!(
                classify_session_delete(&response),
                SessionDeleteResponse::Unconfirmed
            );
        }
    }

    #[test]
    fn foreign_events_are_quarantined_and_provider_requests_are_typed() {
        let foreign = br#"{"id":"evt","type":"session.idle","properties":{"sessionID":"other"}}"#;
        assert_eq!(
            parse_event(foreign, "ses_fixture").expect("foreign event parses"),
            Event::Foreign
        );
        let permission = br#"{"id":"evt","type":"permission.asked","properties":{"id":"per_fixture","sessionID":"ses_fixture","permission":"edit","patterns":["src/**"],"metadata":{},"always":["*"]}}"#;
        assert!(matches!(
            parse_event(permission, "ses_fixture").expect("permission parses"),
            Event::ProviderRequest(_)
        ));
        let unknown =
            br#"{"id":"evt","type":"provider.future","properties":{"sessionID":"ses_fixture"}}"#;
        assert_eq!(
            parse_event(unknown, "ses_fixture").expect("correlated unknown is preserved"),
            Event::Unknown("provider.future".to_owned())
        );
    }

    fn fixture_response(name: &str) -> Response {
        Response {
            status: 200,
            body: std::fs::read(format!("{RANGE_ROOT}/{name}")).expect("range fixture reads"),
            next_cursor: None,
        }
    }
}
