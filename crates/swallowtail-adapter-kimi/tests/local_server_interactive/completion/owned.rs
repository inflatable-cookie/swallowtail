#[test]
fn owned_session_joins_transport_and_then_its_foreground_child() {
    let server = InteractiveFixtureServer::start(InteractiveScenario::Complete);
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.owned-interactive");
    let services = host.services(execution_host.clone(), true);
    let owned = block_on(start_kimi_local_server_owned(
        KimiLocalServerOwnedInput::new(
            attached_input(execution_host, "0.29.0"),
            id(InstanceTargetRef::new, "fixture.kimi.executable"),
        ),
        probe(),
        services.clone(),
    ))
    .expect("owned local server starts");
    let detachment_error = owned
        .prepared()
        .prepare_session(super::fixture::session_input(
            "owned-detachment",
            swallowtail_adapter_kimi::KimiLocalServerSessionConfiguration::new(
                KimiLocalServerPermissionMode::Auto,
            )
            .with_active_turn_detachment(),
        ))
        .expect_err("owned topology rejects detachment");
    assert_eq!(
        detachment_error.diagnostic().safe().code(),
        "swallowtail.kimi.local_server.preparation.detachment_unsupported"
    );
    let profile = session_profile(
        owned.prepared(),
        KimiLocalServerPermissionMode::Auto,
        "owned-session",
    );
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    let mut turn = block_on(session.start_turn(turn("owned-turn"), services)).expect("turn starts");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(host.credential_releases(), 2);
    assert!(!host.process_stopped_and_joined());
    assert_eq!(block_on(owned.close()), CleanupOutcome::Clean);
    assert!(host.process_stopped_and_joined());
}
