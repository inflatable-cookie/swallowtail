#[test]
fn provider_disconnect_and_unknown_event_remain_distinct_and_redacted() {
    let cases = [
        (
            StreamFixture::ProviderError,
            "swallowtail.opencode.provider_failed",
        ),
        (
            StreamFixture::Disconnect,
            "swallowtail.opencode.sse_disconnected",
        ),
        (StreamFixture::Unknown, "swallowtail.opencode.event_unknown"),
        (
            StreamFixture::DuplicateUsage,
            "swallowtail.opencode.usage_duplicate",
        ),
        (
            StreamFixture::MissingUsage,
            "swallowtail.opencode.usage_missing",
        ),
    ];
    for (stream, expected) in cases {
        let server = FixtureServer::start(stream);
        let fixture = Fixture::new(server.endpoint(), "host.local");
        let driver = OpenCodeHttpDriver::new();
        let mut session = block_on(driver.open_session(
            fixture.plan(DriverRole::InteractiveSession),
            open_session_request("failure-session", fixture.resource.clone()),
            fixture.services(),
        ))
        .expect("session opens");
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("failure-turn").expect("turn id is valid"),
                OperationContent::new("fail safely").expect("content is valid"),
            ),
            fixture.services(),
        ))
        .expect("turn starts");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        let diagnostic = match outcome.status() {
            TerminalStatus::ProviderFailed(diagnostic)
            | TerminalStatus::RuntimeFailed(diagnostic) => diagnostic,
            status => panic!("unexpected terminal status: {status:?}"),
        };
        assert_eq!(diagnostic.code(), expected);
        assert!(!format!("{diagnostic:?}").contains("raw-secret-error-sentinel"));
        let _ = block_on(turn.close());
        let _ = block_on(close_session(session, &fixture));
    }
}

#[test]
fn unsupported_session_options_fail_before_network_work() {
    let server = FixtureServer::start(StreamFixture::Success);
    let fixture = Fixture::new(server.endpoint(), "host.local");
    let driver = OpenCodeHttpDriver::new();
    let options = swallowtail_runtime::SessionOptions::default().with_developer_instructions(
        OperationContent::new("private instructions").expect("content is valid"),
    );
    let error = block_on(
        driver.open_session(
            fixture.plan(DriverRole::InteractiveSession),
            open_session_request("unsupported-session", fixture.resource.clone())
            .with_options(options),
            fixture.services(),
        ),
    )
    .err()
    .expect("unsupported options fail");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.opencode.unsupported"
    );
    assert!(server.requests().is_empty());
}
