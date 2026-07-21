#[test]
fn timeout_aborts_and_joins_without_becoming_cancellation() {
    let server = FixtureServer::start(StreamFixture::WaitForAbort);
    let fixture = Fixture::new(server.endpoint(), "host.remote-authoritative");
    let driver = OpenCodeHttpDriver::new();
    let mut session = block_on(driver.open_session(
        fixture.plan(DriverRole::InteractiveSession),
        OpenSessionRequest::new(
            RequestId::new("deadline-session").expect("request id is valid"),
            fixture.resource.clone(),
            None,
        ),
        fixture.services(),
    ))
    .expect("session opens");
    let deadline = fixture.thread.deadline_after(Duration::from_millis(30));
    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("deadline-turn").expect("turn id is valid"),
                OperationContent::new("wait").expect("content is valid"),
            )
            .with_deadline(deadline),
            fixture.services(),
        ),
    )
    .expect("turn starts");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert!(matches!(
        block_on(turn.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(matches!(
        block_on(session.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(
        server
            .requests()
            .iter()
            .any(|request| request.contains("/abort?directory="))
    );
}

#[test]
fn explicit_cancellation_stays_cancelled_and_uses_abort() {
    let server = FixtureServer::start(StreamFixture::WaitForAbort);
    let fixture = Fixture::new(server.endpoint(), "host.local");
    let driver = OpenCodeHttpDriver::new();
    let mut session = block_on(driver.open_session(
        fixture.plan(DriverRole::InteractiveSession),
        OpenSessionRequest::new(
            RequestId::new("cancel-session").expect("request id is valid"),
            fixture.resource.clone(),
            None,
        ),
        fixture.services(),
    ))
    .expect("session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("cancel-turn").expect("turn id is valid"),
            OperationContent::new("wait").expect("content is valid"),
        ),
        fixture.services(),
    ))
    .expect("turn starts");
    block_on(turn.cancellation().request()).expect("cancellation succeeds");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert!(matches!(
        block_on(turn.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(matches!(
        block_on(session.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(
        server
            .requests()
            .iter()
            .any(|request| request.contains("/abort?directory="))
    );
}


