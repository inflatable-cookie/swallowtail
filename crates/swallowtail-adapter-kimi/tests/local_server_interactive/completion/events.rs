#[test]
fn later_releases_ignore_unsolicited_global_events_from_other_sessions() {
    for version in [
        "0.29.1", "0.29.2", "0.30.0", "0.31.0", "0.31.1", "0.36.1", "0.37.2", "0.38.0",
    ] {
        let server =
            InteractiveFixtureServer::start_with_version(InteractiveScenario::GlobalNoise, version);
        let host = FixtureHost::for_endpoint(server.endpoint());
        let execution_host = id(
            ExecutionHostId::new,
            &format!("fixture.kimi.global-noise.{version}"),
        );
        let services = host.services(execution_host.clone(), false);
        let prepared = prepare(execution_host, services.clone(), version);
        let profile = session_profile(
            &prepared,
            KimiLocalServerPermissionMode::Auto,
            "global-noise",
        );
        let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
        let mut turn =
            block_on(session.start_turn(turn("global-noise-turn"), services.clone()))
                .expect("turn starts");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome exists"),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            outcome.output().expect("output exists").as_str(),
            "fixture result"
        );
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(block_on(close_session(session, services)), CleanupOutcome::Clean);
    }
}
