#[test]
fn selected_default_sliding_window_survives_one_planned_rollover() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let selected = compression();
    let operation = prepared
        .prepare_live_session(
            GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                RequestId::new("context-compression-rollover").expect("request id is valid"),
                None,
            )
            .with_context_window_compression(selected),
        )
        .expect("selected compression prepares");
    assert_eq!(
        operation.evidence().context_window_compression(),
        Some(selected)
    );
    assert_eq!(
        operation.plan().protocol_facade_id().as_str(),
        GEMINI_LIVE_FACADE_REVISION
    );
    assert_eq!(
        operation.plan().model_id().expect("model bound").as_str(),
        "gemini-3.1-flash-live-preview"
    );

    let mut session = block_on(operation.open_session(fixture.services())).expect("session opens");
    for turn in 1..=2 {
        let response = start_turn(&mut session, &fixture, turn);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let frames = fixture.server.frames();
    let setup_frames = raw_setup_frames(&frames);
    assert_eq!(
        setup_frames[0],
        include_str!("../fixtures/gemini-live-2026-07-22/client-setup-compression-initial.json")
            .trim()
    );
    assert_eq!(
        setup_frames[1],
        include_str!("../fixtures/gemini-live-2026-07-22/client-setup-compression-resume.json")
            .trim()
    );
    assert_eq!(
        compression_values(&frames),
        vec![Some(json!({"slidingWindow": {}})); 2]
    );
    assert_eq!(handles(&frames)[0], None);
    assert_eq!(
        handles(&frames)[1],
        Some("fixture-private-handle-2".to_owned())
    );
    assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
}

#[test]
fn omitted_compression_keeps_prior_initial_and_resume_setup_bytes() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let operation = prepared
        .prepare_live_session(GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
            RequestId::new("context-compression-omitted").expect("request id is valid"),
            None,
        ))
        .expect("omitted compression prepares");
    assert_eq!(operation.evidence().context_window_compression(), None);

    let mut session = block_on(operation.open_session(fixture.services())).expect("session opens");
    for turn in 1..=2 {
        let response = start_turn(&mut session, &fixture, turn);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let frames = fixture.server.frames();
    let setup_frames = raw_setup_frames(&frames);
    assert_eq!(compression_values(&frames), vec![None, None]);
    assert_eq!(
        setup_frames[0],
        include_str!("../fixtures/gemini-live-2026-07-22/client-setup-initial.json").trim()
    );
    assert_eq!(
        setup_frames[1],
        include_str!("../fixtures/gemini-live-2026-07-22/client-setup-resume.json").trim()
    );
}
