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

