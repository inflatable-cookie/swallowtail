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
        let expected: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(format!(
                "{RANGE_ROOT}/generation-controls-prompt-request.json"
            ))
            .expect("fixture reads"),
        )
        .expect("fixture parses");
        assert_eq!(actual, expected);
    }

    #[test]
    fn provider_callback_responses_preserve_one_shot_and_ordered_question_bounds() {
        let once = callback_response(
            "per_fixture",
            ProviderRequestKind::Permission,
            None,
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
            None,
            &CallbackResult::Success(
                CallbackPayload::new(br#"{"reply":"always"}"#, 256).expect("payload is bounded"),
            ),
        )
        .expect_err("persistent response rejects");
        assert_eq!(
            persistent.diagnostic().code(),
            "swallowtail.opencode.callback_malformed"
        );

        let request = question_request(
            br#"{"questions":[{"question":"Choose","header":"Mode","options":[{"label":"Safe","description":"Read only"}],"multiple":false}]}"#,
        )
        .expect("question projects");
        let response = HarnessUserInputResponse::new(
            [HarnessUserInputAnswer::selected(
                HarnessQuestionId::new("question-0").unwrap(),
                [HarnessQuestionOptionId::new("Safe").unwrap()],
                None,
            )],
            1,
            256,
        )
        .unwrap();
        let answers = callback_response(
            "que_fixture",
            ProviderRequestKind::Question { count: 1 },
            Some(&request),
            &CallbackResult::UserInput(response.clone()),
        )
        .expect("ordered answers encode");
        assert_eq!(answers.path, "/question/que_fixture/reply");
        let wrong_count = callback_response(
            "que_fixture",
            ProviderRequestKind::Question { count: 2 },
            Some(&request),
            &CallbackResult::UserInput(response),
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
        let compacted = br#"{"id":"evt","type":"session.compacted","properties":{"sessionID":"ses_fixture"}}"#;
        assert_eq!(
            parse_event(compacted, "ses_fixture").expect("same-session compaction is accepted"),
            Event::Unknown("session.compacted".to_owned())
        );
        let foreign_compaction = br#"{"id":"evt","type":"session.compacted","properties":{"sessionID":"other"}}"#;
        assert_eq!(
            parse_event(foreign_compaction, "ses_fixture")
                .expect("foreign compaction is quarantined"),
            Event::Foreign
        );
    }
