#[test]
fn attached_prepared_session_streams_and_preserves_exact_bindings() {
    let server = InteractiveFixtureServer::start(InteractiveScenario::Complete);
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.interactive");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services.clone(), "0.29.0");
    let profile = session_profile(&prepared, KimiLocalServerPermissionMode::Manual, "complete");
    assert_eq!(
        profile.evidence().observable_activity().availability(),
        ObservableActivityAvailability::Available
    );
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");

    assert_eq!(
        session
            .provider_session_ref()
            .expect("provider session is bound")
            .as_provider_value(),
        "interactive-session"
    );
    assert_eq!(
        session
            .management_binding()
            .expect("management binding is returned")
            .origin(),
        ProviderSessionBindingOrigin::Created
    );
    let resume = session
        .resume_binding()
        .expect("resume binding is returned")
        .clone();
    let mut turn = block_on(session.start_turn(turn("turn-complete"), services.clone()))
        .unwrap_or_else(|error| panic!("turn starts: {error:?}; requests={:?}", server.requests()));
    let events = block_on(
        turn.take_events()
            .expect("event stream exists")
            .collect::<Vec<_>>(),
    );
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output exists").as_str(),
        "fixture result"
    );
    assert!(events.iter().all(Result::is_ok));
    assert!(
        events
            .iter()
            .filter_map(|event| event.as_ref().ok())
            .any(|event| {
                event.reconciliation_checkpoint().is_some_and(|checkpoint| {
                    checkpoint.provider_session_ref().as_provider_value() == "interactive-session"
                        && checkpoint.provider_turn_ref().as_provider_value() == "7"
                        && checkpoint.runtime_turn_id().as_str() == "turn-complete"
                })
            })
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(
        block_on(close_session(session, services.clone())),
        CleanupOutcome::Clean
    );

    let resumed = block_on(
        profile
            .resume_session(id(RequestId::new, "resume-request"), resume, services.clone())
            .expect("resume prepares"),
    )
    .expect("session resumes");
    assert_eq!(
        resumed
            .management_binding()
            .expect("resumed management binding exists")
            .origin(),
        ProviderSessionBindingOrigin::Resumed
    );
    let management = resumed
        .management_binding()
        .expect("management binding remains available")
        .clone();
    assert_eq!(block_on(close_session(resumed, services)), CleanupOutcome::Clean);
    let archive = prepared
        .prepare_archive_session(KimiLocalServerSessionManagementInput::new(
            id(RequestId::new, "archive-after-close"),
            management,
        ))
        .expect("archive prepares after the attachment closes");
    let archived = block_on(
        archive.execute(host.services(id(ExecutionHostId::new, "fixture.kimi.interactive"), false)),
    )
    .expect("archive executes");
    assert_eq!(
        archived.effect().truth(),
        ProviderSessionEffectTruth::Applied
    );
    assert_eq!(host.credential_releases(), 4);
    let requests = server.requests();
    assert!(requests.iter().any(|request| request.starts_with("WS ")));
    assert!(requests.iter().any(|request| {
        request.contains("POST /api/v1/sessions/interactive-session/prompts")
            && request.contains(r#""permission_mode":"manual""#)
    }));
    let resume_index = requests
        .iter()
        .position(|request| request.contains("GET /api/v1/sessions/interactive-session"))
        .expect("resume lookup was observed");
    let archive_index = requests
        .iter()
        .position(|request| request.contains(":archive"))
        .expect("explicit archive was observed");
    assert!(resume_index < archive_index);
}
