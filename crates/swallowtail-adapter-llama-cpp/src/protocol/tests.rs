mod tests {
    use super::*;
    use serde_json::{Value, json};

    const FIXTURE_ROOT: &str = "../../tests/fixtures/llama-cpp-b9910-openai-chat";
    const PROTOCOL: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/llama-cpp-b9910-openai-chat/protocol.json"));
    const READY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/llama-cpp-b9910-openai-chat/health-ready.json"));
    const LOADING: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/llama-cpp-b9910-openai-chat/health-loading.json"));
    const PROPERTIES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/llama-cpp-b9910-openai-chat/properties.json"));
    const MODELS: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/llama-cpp-b9910-openai-chat/models.json"));
    const CHAT_REQUEST: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/llama-cpp-b9910-openai-chat/chat-request.json"));
    const SUCCESS: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/llama-cpp-b9910-openai-chat/success.sse"));
    const MIDSTREAM_ERROR: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/llama-cpp-b9910-openai-chat/midstream-error.sse"));
    const UNSUPPORTED: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/llama-cpp-b9910-openai-chat/unsupported-semantics.sse"));
    const DISCONNECT: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/llama-cpp-b9910-openai-chat/disconnect.sse"));

    fn response(status: u32, body: &[u8]) -> Response {
        Response {
            status,
            body: body.to_vec(),
        }
    }

    #[test]
    fn fixture_separates_artifact_server_deployment_facade_and_route() {
        let fixture: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture parses");
        assert_eq!(fixture["fixture_schema"], 1);
        assert_eq!(fixture["server"]["release"], "b9910");
        assert_eq!(fixture["server"]["commit"], "f5525f7e7a7e7cbecd386144299493ea40499bd3");
        assert_eq!(fixture["artifact"]["size_bytes"], 1_185_376);
        assert_eq!(fixture["artifact"]["bundled_by_swallowtail"], false);
        assert_eq!(fixture["deployment"]["ownership"], "attached_external");
        assert_eq!(fixture["deployment"]["endpoint_source"], "host_approved_grant");
        assert_eq!(fixture["facade"]["id"], "llama.cpp.openai-chat-completions.b9910");
        assert_eq!(fixture["route"]["model_alias"], "swallowtail-fixture-stories260k");
        assert_eq!(fixture["cleanup"]["stop_server"], false);
        assert!(FIXTURE_ROOT.ends_with("llama-cpp-b9910-openai-chat"));
    }

    #[test]
    fn readiness_distinguishes_loading_from_ready() {
        assert_eq!(parse_health(&response(200, READY)).expect("ready parses"), Readiness::Ready);
        assert_eq!(parse_health(&response(503, LOADING)).expect("loading parses"), Readiness::Loading);
        assert_eq!(Request::health().path, "/health");
    }

    #[test]
    fn properties_bind_exact_build_template_and_text_only_capability_evidence() {
        let evidence = parse_properties(&response(200, PROPERTIES), ATTACHED_VERSION)
            .expect("properties parse");
        assert_eq!(evidence.model_alias, "swallowtail-fixture-stories260k");
        assert_eq!(evidence.chat_template, "chatml");
        assert!(evidence.chat_template_capabilities.supports_string_content);
        assert!(evidence.chat_template_capabilities.supports_system_role);
        assert!(!evidence.chat_template_capabilities.supports_tools);
        assert!(!evidence.chat_template_capabilities.supports_tool_calls);
        assert_eq!(Request::properties().path, "/props");

        let mut wrong: Value = serde_json::from_slice(PROPERTIES).expect("properties JSON parses");
        wrong["build_info"] = json!("b10069-178a6c449");
        let error = parse_properties(
            &response(200, &serde_json::to_vec(&wrong).expect("JSON serializes")),
            ATTACHED_VERSION,
        )
            .expect_err("unobserved version fails");
        assert_eq!(error.diagnostic().code(), "swallowtail.llama_cpp.version_mismatch");
    }

    #[test]
    fn single_model_catalogue_preserves_alias_without_claiming_provider_or_limits() {
        let entries = parse_models(&response(200, MODELS)).expect("models parse");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id().as_str(), "swallowtail-fixture-stories260k");
        assert_eq!(entries[0].provider_id(), None);
        assert_eq!(entries[0].metadata().token_limits(), None);
        assert_eq!(Request::models().path, "/v1/models");
    }

    #[test]
    fn chat_request_is_one_text_attempt_with_an_explicit_output_bound() {
        let request = Request::chat(
            "swallowtail-fixture-stories260k",
            &OperationContent::new("Fixture prompt").expect("content is valid"),
            8,
        )
        .expect("request builds");
        assert_eq!(request.method, Method::Post);
        assert_eq!(request.path, "/v1/chat/completions");
        let actual: Value = serde_json::from_slice(request.body.as_deref().expect("body exists"))
            .expect("request JSON parses");
        let expected: Value = serde_json::from_str(CHAT_REQUEST).expect("fixture JSON parses");
        assert_eq!(actual, expected);
        assert_eq!(serde_json::from_str::<Value>(PROTOCOL).expect("manifest parses")["inference"]["maximum_attempts"], 1);
    }

    #[test]
    fn stream_preserves_role_text_finish_usage_and_done_order() {
        let events = decode(SUCCESS).expect("success stream decodes");
        assert_eq!(events[0], Event::RoleStart);
        assert_eq!(events[1], Event::OutputDelta("Fixture ".to_owned()));
        assert_eq!(events[2], Event::OutputDelta("output".to_owned()));
        assert_eq!(events[3], Event::Finished("length".to_owned()));
        assert!(matches!(events[4], Event::Usage(usage) if usage.input_tokens() == Some(12) && usage.output_tokens() == Some(2)));
        assert_eq!(events[5], Event::Done);
    }

    #[test]
    fn midstream_error_is_terminal_and_unobserved_semantics_fail_closed() {
        let error_events = decode(MIDSTREAM_ERROR).expect("error stream decodes");
        assert_eq!(error_events[0], Event::RoleStart);
        assert_eq!(error_events[1], Event::OutputDelta("partial".to_owned()));
        assert_eq!(error_events[2], Event::ProviderFailed);

        let mut decoder = SseDecoder::default();
        let frames = decoder.push(UNSUPPORTED).expect("SSE frames decode");
        decoder.finish().expect("stream is complete");
        let error = parse_event(&frames[0]).expect_err("reasoning is outside the fixture");
        assert_eq!(error.diagnostic().code(), "swallowtail.llama_cpp.content_semantics_unsupported");
    }

    #[test]
    fn incomplete_and_oversized_streams_fail_without_raw_payload_diagnostics() {
        let mut decoder = SseDecoder::default();
        let prefix = decoder.push(DISCONNECT).expect("complete prefix decodes");
        assert_eq!(prefix.len(), 1);
        let error = decoder.finish().expect_err("partial frame fails");
        assert_eq!(error.diagnostic().code(), "swallowtail.llama_cpp.sse_disconnected");

        let mut oversized = SseDecoder::default();
        let error = oversized.push(&vec![b'x'; MAX_SSE_BYTES + 1]).expect_err("oversized input fails");
        assert_eq!(error.diagnostic().code(), "swallowtail.llama_cpp.sse_limit");
        assert!(!error.to_string().contains("raw-provider-payload"));
    }

    fn decode(bytes: &[u8]) -> Result<Vec<Event>, RuntimeFailure> {
        let mut decoder = SseDecoder::default();
        let frames = decoder.push(bytes)?;
        decoder.finish()?;
        frames.iter().map(parse_event).collect()
    }
}
