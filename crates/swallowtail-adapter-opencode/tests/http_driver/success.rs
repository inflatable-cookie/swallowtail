#[test]
fn catalogue_and_read_only_session_run_against_the_frozen_http_fixture() {
    let server = FixtureServer::start(StreamFixture::Success);
    let fixture = Fixture::new(server.endpoint(), "host.local");
    let driver = OpenCodeHttpDriver::new();

    let models = block_on(driver.list_models(
        fixture.plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("catalogue").expect("request id is valid")),
        fixture.services(),
    ))
    .expect("catalogue succeeds");
    assert_eq!(models.len(), 1);
    assert_eq!(
        models[0].provider_id().expect("provider").as_str(),
        "anthropic"
    );
    assert_eq!(models[0].id().as_str(), "claude-sonnet");

    let mut session = block_on(driver.open_session(
        fixture.plan(DriverRole::InteractiveSession),
        OpenSessionRequest::new(
            RequestId::new("session").expect("request id is valid"),
            fixture.resource.clone(),
            None,
        ),
        fixture.services(),
    ))
    .expect("session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-1").expect("turn id is valid"),
            OperationContent::new("fixture input").expect("content is valid"),
        ),
        fixture.services(),
    ))
    .expect("turn starts");
    let mut events = turn.take_events().expect("event stream is available");
    let outcome = turn
        .take_terminal_outcome()
        .expect("terminal outcome is available");
    let observed = block_on(async {
        let mut kinds = Vec::new();
        while let Some(event) = events.next().await {
            kinds.push(event.expect("event succeeds").kind().clone());
        }
        (kinds, outcome.await)
    });
    assert_eq!(observed.1.status(), &TerminalStatus::Completed);
    assert_eq!(
        observed.1.output().expect("snapshot output").as_str(),
        "hello world"
    );
    assert_eq!(observed.0[0], RuntimeEventKind::Started);
    assert!(observed.0.contains(&RuntimeEventKind::OutputDelta));
    assert!(observed.0.contains(&RuntimeEventKind::OutputAvailable));
    assert!(matches!(
        block_on(turn.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(matches!(
        block_on(session.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));

    let requests = server.requests();
    for route in [
        "/global/health",
        "/provider",
        "/session?directory=",
        "/event?directory=",
        "/session/ses_fixture/prompt_async?directory=",
    ] {
        assert!(
            requests.iter().any(|request| request.contains(route)),
            "missing {route}"
        );
    }
    assert!(!requests.iter().any(|request| {
        request.contains("/dispose")
            || request.contains("/delete")
            || request.contains("/share")
            || request.contains("/config")
    }));
}


