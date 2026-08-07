#[test]
fn prepared_session_uses_only_the_exact_private_resume_id_on_later_turns() {
    let host_id = ExecutionHostId::new("fixture.qwen.interactive").expect("valid host");
    let (discovery_process, _) = FakeProcessService::completed("0.19.11\n");
    let (discovery_services, _) = host_services_for(
        host_id.clone(),
        discovery_process,
        Arc::new(PendingTimeService),
    );
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery_services,
    ))
    .expect("Qwen prepares");
    let profile = prepared
        .prepare_session(QwenSessionProfileInput::new(
            RequestId::new("qwen-session").expect("valid request"),
            QwenModelSelection::new(
                ModelRouteId::new("qwen.session.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                ModelId::new("qwen3-coder-plus").expect("valid model"),
            ),
            WorkingResourceRef::new("qwen.session.workspace").expect("valid resource"),
        ))
        .expect("Qwen session prepares");
    assert_prepared_operation_evidence_matches_plan(profile.evidence().operation(), profile.plan());
    assert_eq!(
        profile.evidence().observable_activity().availability(),
        ObservableActivityAvailability::Available
    );
    assert_eq!(
        profile
            .prepare_working_state_restoration(
                RuntimeTurnId::new("lost-qwen-turn").expect("valid turn")
            )
            .method(),
        swallowtail_runtime::WorkingStateRestorationMethod::FreshSessionReplacement
    );

    let (process, states) = ScriptedProcessService::completed(&[
        include_str!("../fixtures/qwen-code-v0.19.11/interactive-first-turn.jsonl"),
        include_str!("../fixtures/qwen-code-v0.19.11/interactive-continued-turn.jsonl"),
    ]);
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    assert!(session.provider_session_ref().is_none());
    assert!(session.resume_binding().is_none());

    for (index, content) in ["first prompt", "second prompt"].into_iter().enumerate() {
        let mut turn = block_on(
            session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("qwen-turn-{}", index + 1)).expect("valid turn"),
                    OperationContent::new(content).expect("valid content"),
                )
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
                services.clone(),
            ),
        )
        .expect("turn starts");
        let terminal = block_on(
            turn.take_terminal_outcome()
                .expect("turn terminal is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }

    let first = states[0].request().arguments;
    assert!(!first.iter().any(|argument| argument == "--resume"));
    assert!(!first.iter().any(|argument| argument == "--continue"));
    let second = states[1].request().arguments;
    assert_eq!(
        second
            .windows(2)
            .find(|arguments| arguments[0] == "--resume"),
        Some(
            [
                "--resume".to_owned(),
                "123e4567-e89b-12d3-a456-426614174000".to_owned()
            ]
            .as_slice()
        )
    );
    assert!(!second.iter().any(|argument| argument == "--continue"));
    assert_eq!(states[0].stdin(), b"first prompt");
    assert_eq!(states[1].stdin(), b"second prompt");
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn qwen_session_mismatch_fails_closed_without_starting_another_child() {
    let host_id = ExecutionHostId::new("fixture.qwen.mismatch").expect("valid host");
    let (discovery_process, _) = FakeProcessService::completed("0.19.11\n");
    let (discovery_services, _) = host_services_for(
        host_id.clone(),
        discovery_process,
        Arc::new(PendingTimeService),
    );
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery_services,
    ))
    .expect("Qwen prepares");
    let profile = prepared
        .prepare_session(QwenSessionProfileInput::new(
            RequestId::new("qwen-mismatch").expect("valid request"),
            QwenModelSelection::new(
                ModelRouteId::new("qwen.mismatch.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                ModelId::new("qwen3-coder-plus").expect("valid model"),
            ),
            WorkingResourceRef::new("qwen.mismatch.workspace").expect("valid resource"),
        ))
        .expect("Qwen session prepares");
    let (process, states) = ScriptedProcessService::completed(&[
        include_str!("../fixtures/qwen-code-v0.19.11/interactive-first-turn.jsonl"),
        include_str!("../fixtures/qwen-code-v0.19.11/interactive-session-mismatch.jsonl"),
    ]);
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    for (index, expected) in [
        TerminalStatus::Completed,
        TerminalStatus::RuntimeFailed(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.qwen.headless.malformed_stream",
            "Qwen Code emitted malformed stream output",
        )),
    ]
    .into_iter()
    .enumerate()
    {
        let mut turn = block_on(
            session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("qwen-mismatch-{}", index + 1)).expect("valid turn"),
                    OperationContent::new(format!("prompt {}", index + 1)).expect("valid content"),
                )
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
                services.clone(),
            ),
        )
        .expect("bounded turn starts");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(outcome.status(), &expected);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }
    let error = match block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("qwen-mismatch-3").expect("valid turn"),
                OperationContent::new("must not start").expect("valid content"),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
            services,
        ),
    ) {
        Ok(_) => panic!("mismatched provider session must invalidate the handle"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.qwen.headless.session_unusable"
    );
    assert_eq!(states.len(), 2);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

