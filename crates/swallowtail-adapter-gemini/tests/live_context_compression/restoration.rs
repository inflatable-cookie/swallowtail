#[test]
fn selected_default_sliding_window_survives_fresh_restoration_and_composes() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let operation = prepared
        .prepare_live_session(
            GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                RequestId::new("context-compression-restoration").expect("request id is valid"),
                None,
            )
            .with_reasoning_mode(mode("high"))
            .with_maximum_output_tokens(maximum(1_024))
            .with_context_window_compression(compression()),
        )
        .expect("composed compression prepares");
    let interrupted = RuntimeTurnId::new("context-compression-interrupted").expect("turn id");
    let restoration = operation.prepare_working_state_restoration(interrupted.clone());
    let restored = block_on(restoration.restore(fixture.services())).expect("replacement opens");
    let WorkingStateRestorationOutcome::RealtimeSessionReplaced(replacement) = restored else {
        panic!("fresh realtime replacement expected");
    };
    assert_eq!(replacement.interrupted_turn_id(), &interrupted);
    let (_, mut replacement) = replacement.into_parts();
    for turn in 1..=2 {
        let response = start_turn(&mut replacement, &fixture, turn);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(replacement.close()), CleanupOutcome::Clean);

    let frames = fixture.server.frames();
    assert_eq!(
        compression_values(&frames),
        vec![Some(json!({"slidingWindow": {}})); 2]
    );
    assert_eq!(
        levels(&frames),
        vec![Some("HIGH".to_owned()), Some("HIGH".to_owned())]
    );
    assert_eq!(maxima(&frames), vec![Some(1_024), Some(1_024)]);
    assert_eq!(handles(&frames)[0], None);
    assert!(handles(&frames)[1].is_some());
}
