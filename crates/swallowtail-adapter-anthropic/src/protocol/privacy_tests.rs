    #[test]
    fn replay_request_debug_redacts_private_body() {
        const SIGNATURE: &str = "sig_omitted_fixture_private";
        let request = Request::direct_message(
            "claude-opus-4-7",
            RedactedBytes::from_vec(
                format!(
                    r#"[{{"content":[{{"signature":"{SIGNATURE}","thinking":"","type":"thinking"}}],"role":"assistant"}}]"#
                )
                .into_bytes(),
            ),
            &[],
            64,
            None,
            Some(crate::AnthropicThinkingMode::adaptive()),
        )
        .expect("replay request serializes");
        let debug = format!("{request:?}");
        assert!(!debug.contains(SIGNATURE));
        assert_eq!(
            format!("{:?}", request.body.as_ref().expect("body exists")),
            "[redacted]"
        );
        assert!(request
            .body
            .as_deref()
            .expect("body exists")
            .windows(SIGNATURE.len())
            .any(|window| window == SIGNATURE.as_bytes()));
        drop(request);
    }

    #[test]
    fn malformed_private_sse_frame_fails_without_leaking() {
        const SIGNATURE: &str = "sig_omitted_fixture_private";
        let mut decoder = SseDecoder::default();
        let error = decoder
            .push(
                format!(
                    "data: {{\"delta\":{{\"signature\":\"{SIGNATURE}\",\"type\":\"signature_delta\"}},\"index\":0,\"type\":\"content_block_delta\"}}\n\n"
                )
                .as_bytes(),
            )
            .expect_err("nameless private frame fails");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.anthropic.protocol_invalid"
        );
        assert!(!format!("{error:?}").contains(SIGNATURE));
        decoder
            .finish()
            .expect("failed private frame does not remain buffered");
    }

    #[test]
    fn wrong_typed_signature_fails_without_leaking() {
        const SIGNATURE: &str = "sig_omitted_fixture_private";
        let frames = decode(
            format!(
                "event: content_block_delta\ndata: {{\"type\":\"content_block_delta\",\"index\":0,\"delta\":{{\"type\":\"signature_delta\",\"signature\":{{\"nested\":\"{SIGNATURE}\"}}}}}}\n\n"
            )
            .as_bytes(),
        )
        .expect("named frame decodes");
        let error = parse_event(&frames[0]).expect_err("object signature fails");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.anthropic.protocol_invalid"
        );
        assert!(!format!("{error:?}").contains(SIGNATURE));
    }

    #[test]
    fn wrong_typed_redacted_data_fails_without_leaking() {
        const REDACTED: &str = "redacted_fixture_private_data";
        let frames = decode(
            format!(
                "event: content_block_start\ndata: {{\"type\":\"content_block_start\",\"index\":0,\"content_block\":{{\"type\":\"redacted_thinking\",\"data\":{{\"nested\":\"{REDACTED}\"}}}}}}\n\n"
            )
            .as_bytes(),
        )
        .expect("named frame decodes");
        let error = parse_event(&frames[0]).expect_err("object redacted data fails");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.anthropic.protocol_invalid"
        );
        assert!(!format!("{error:?}").contains(REDACTED));
    }
