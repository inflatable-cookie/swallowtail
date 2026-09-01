#[test]
fn selected_compression_composes_with_every_admitted_thinking_and_output_selection() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    for (thinking_index, thinking) in [
        None,
        Some("minimal"),
        Some("low"),
        Some("medium"),
        Some("high"),
    ]
    .into_iter()
    .enumerate()
    {
        for (maximum_index, maximum_value) in [None, Some(1), Some(1_024), Some(65_536)]
            .into_iter()
            .enumerate()
        {
            let mut input = GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                RequestId::new(format!(
                    "context-compression-compose-{thinking_index}-{maximum_index}"
                ))
                .expect("request id is valid"),
                None,
            )
            .with_context_window_compression(compression());
            if let Some(thinking) = thinking {
                input = input.with_reasoning_mode(mode(thinking));
            }
            if let Some(maximum_value) = maximum_value {
                input = input.with_maximum_output_tokens(maximum(maximum_value));
            }
            let operation = prepared
                .prepare_live_session(input)
                .expect("admitted composition prepares");
            assert_eq!(
                operation.evidence().context_window_compression(),
                Some(compression())
            );
            assert_eq!(
                operation.plan().protocol_facade_id().as_str(),
                GEMINI_LIVE_FACADE_REVISION
            );
            assert_eq!(
                operation.request().maximum_output_tokens(),
                maximum_value.map(maximum)
            );
            let expected_mode = thinking.map(mode);
            assert_eq!(operation.request().reasoning_mode(), expected_mode.as_ref());
        }
    }
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}
