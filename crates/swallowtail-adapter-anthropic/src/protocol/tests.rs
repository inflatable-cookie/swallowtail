mod tests {
    use super::*;

    const SUCCESS: &[u8] = include_bytes!(
        "../../tests/fixtures/anthropic-2023-06-01/success.sse"
    );
    const UNKNOWN: &[u8] = include_bytes!(
        "../../tests/fixtures/anthropic-2023-06-01/unknown-event.sse"
    );
    const ERROR: &[u8] = include_bytes!(
        "../../tests/fixtures/anthropic-2023-06-01/midstream-error.sse"
    );
    const DISCONNECT: &[u8] = include_bytes!(
        "../../tests/fixtures/anthropic-2023-06-01/disconnect.sse"
    );

    #[test]
    fn messages_effort_is_additive_and_does_not_enable_thinking() {
        let content = OperationContent::new("fixture prompt").expect("content is valid");
        let reasoning = ReasoningMode::new("xhigh").expect("reasoning is valid");
        let request = Request::message(
            "claude-opus-4-7",
            &content,
            64,
            None,
            None,
            Some(&reasoning),
        )
        .expect("message request serializes");
        let raw = request.body.expect("request body exists");
        let body: serde_json::Value = serde_json::from_slice(&raw).expect("request body parses");
        assert_eq!(body["output_config"]["effort"], "xhigh");
        assert!(body.get("thinking").is_none());
        assert_eq!(
            raw,
            br#"{"max_tokens":64,"messages":[{"content":"fixture prompt","role":"user"}],"model":"claude-opus-4-7","output_config":{"effort":"xhigh"},"stream":true}"#
        );
    }

    #[test]
    fn absent_message_effort_keeps_existing_serialized_body() {
        let content = OperationContent::new("fixture prompt").expect("content is valid");
        let request = Request::message("claude-fixture-primary", &content, 64, None, None, None)
            .expect("message request serializes");
        assert_eq!(
            request.body.expect("request body exists"),
            br#"{"max_tokens":64,"messages":[{"content":"fixture prompt","role":"user"}],"model":"claude-fixture-primary","stream":true}"#
        );
    }

    #[test]
    fn direct_messages_effort_is_additive() {
        let reasoning = ReasoningMode::new("max").expect("reasoning is valid");
        let request = Request::direct_message(
            "claude-opus-4-7",
            serde_json::json!([]),
            &[],
            64,
            Some(&reasoning),
        )
        .expect("direct message request serializes");
        let body: serde_json::Value =
            serde_json::from_slice(&request.body.expect("request body exists"))
                .expect("request body parses");
        assert_eq!(body["output_config"]["effort"], "max");
        assert!(body.get("thinking").is_none());
        assert_eq!(
            serde_json::to_vec(&body).expect("request body reserializes"),
            br#"{"max_tokens":64,"messages":[],"model":"claude-opus-4-7","output_config":{"effort":"max"},"stream":true,"tool_choice":{"type":"auto"},"tools":[]}"#
        );
    }

    #[test]
    fn production_decoder_preserves_success_order_and_usage() {
        let frames = decode(SUCCESS).expect("success decodes");
        let events: Vec<_> = frames
            .iter()
            .map(|frame| parse_event(frame).expect("event parses"))
            .collect();
        assert!(matches!(events[0], Event::MessageStart { .. }));
        assert!(matches!(events[3], Event::OutputDelta(ref text) if text == "Hello"));
        assert!(matches!(events[6], Event::Usage(usage, _) if usage.output_tokens() == Some(3)));
        assert_eq!(events.last(), Some(&Event::MessageStop));
    }

    #[test]
    fn production_decoder_ignores_top_level_unknown_and_keeps_stream_errors() {
        let unknown = decode(UNKNOWN).expect("unknown stream decodes");
        assert_eq!(parse_event(&unknown[1]).expect("unknown parses"), Event::Unknown);

        let error = decode(ERROR).expect("error stream decodes");
        assert_eq!(
            parse_event(error.last().expect("error exists")).expect("error parses"),
            Event::ProviderFailed(ProviderErrorKind::Overloaded)
        );
    }

    #[test]
    fn production_decoder_rejects_partial_frames() {
        let mut decoder = SseDecoder::default();
        decoder.push(DISCONNECT).expect("complete prefix parses");
        let error = decoder.finish().expect_err("partial frame fails");
        assert_eq!(error.diagnostic().code(), "swallowtail.anthropic.sse_disconnected");
    }

    #[test]
    fn typed_provider_errors_keep_portable_meaning() {
        let error = provider_failure(ProviderErrorKind::RateLimited, "fixture");
        let classification = error.diagnostic().failure_classification();

        assert_eq!(classification.origin(), FailureOrigin::Provider);
        assert_eq!(classification.kind(), FailureKind::RateLimited);
        assert_eq!(classification.recovery(), FailureRecovery::RetryMaySucceed);
        assert_eq!(error.diagnostic().code(), "swallowtail.anthropic.rate_limited");
    }

    fn decode(bytes: &[u8]) -> Result<Vec<SseFrame>, RuntimeFailure> {
        let mut decoder = SseDecoder::default();
        let frames = decoder.push(bytes)?;
        decoder.finish()?;
        Ok(frames)
    }
}
