#[test]
fn version_drift_fails_before_catalogue_or_inference() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        VersionFixture::Drift,
        StreamFixture::Success,
    ));
    let error = block_on(OllamaNativeAttachedDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        run_request("version-drift"),
        fixture.services(),
    ))
    .err()
    .expect("version drift fails");

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.ollama.version_drift"
    );
    assert_eq!(fixture.server.targets(), ["/api/version"]);
    assert_eq!(fixture.server.inference_attempts(), 0);
    assert!(!format!("{error:?}").contains(fixture.server.endpoint()));
}

#[test]
fn missing_output_bound_fails_before_endpoint_work() {
    let fixture = Fixture::new();
    let request = StructuredRunRequest::new(
        RequestId::new("missing-limit").expect("request id is valid"),
        OperationContent::new("Fixture prompt").expect("content is valid"),
        policy(),
    );
    let error = block_on(OllamaNativeAttachedDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .err()
    .expect("missing bound fails");

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.ollama.output_limit_missing"
    );
    assert!(fixture.server.targets().is_empty());
}

#[test]
fn midstream_provider_error_fails_once_and_redacts_payload() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        VersionFixture::Expected,
        StreamFixture::MidstreamError,
    ));
    let (run, events, outcome) = complete_run(&fixture);

    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderFailed(_)
    ));
    assert_eq!(fixture.server.inference_attempts(), 1);
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        swallowtail_runtime::RuntimeEventKind::OutputDelta
    )));
    assert!(!format!("{:?}", outcome.status()).contains("synthetic provider failure"));
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(fixture.server.is_reachable());
}

#[test]
fn malformed_and_disconnected_streams_fail_with_distinct_safe_codes() {
    for (stream, expected) in [
        (
            StreamFixture::Malformed,
            "swallowtail.ollama.protocol_invalid",
        ),
        (
            StreamFixture::Disconnect,
            "swallowtail.ollama.stream_disconnected",
        ),
    ] {
        let fixture =
            Fixture::with_server(FixtureServer::start_with(VersionFixture::Expected, stream));
        let (run, _, outcome) = complete_run(&fixture);
        let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
            panic!("stream failure must remain provider-facing");
        };
        assert_eq!(diagnostic.code(), expected);
        assert!(!format!("{diagnostic:?}").contains("fixture-model"));
        assert_eq!(fixture.server.inference_attempts(), 1);
        assert!(matches!(
            block_on(run.close()),
            swallowtail_runtime::CleanupOutcome::Clean
        ));
    }
}

#[test]
fn cancellation_joins_owned_stream_without_stopping_runtime() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        VersionFixture::Expected,
        StreamFixture::WaitForCancel,
    ));
    let mut run = block_on(OllamaNativeAttachedDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        run_request("cancel-run"),
        fixture.services(),
    ))
    .expect("run starts");
    let _events = run.take_events().expect("events are available");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    for _ in 0..100 {
        if fixture.server.inference_attempts() == 1 {
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    block_on(run.cancellation().request()).expect("cancellation is accepted");
    let outcome = block_on(terminal);

    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(fixture.server.is_reachable());
}

#[test]
fn independent_runs_are_not_serialized_by_the_driver() {
    let waiting = Fixture::with_server(FixtureServer::start_with(
        VersionFixture::Expected,
        StreamFixture::WaitForCancel,
    ));
    let succeeding = Fixture::new();
    let driver = OllamaNativeAttachedDriver::new();
    let mut first = block_on(driver.start_run(
        waiting.plan(DriverRole::StructuredRun),
        run_request("independent-first"),
        waiting.services(),
    ))
    .expect("first run starts");
    let _first_events = first.take_events().expect("first events are available");
    let first_terminal = first
        .take_terminal_outcome()
        .expect("first terminal is available");
    for _ in 0..100 {
        if waiting.server.inference_attempts() == 1 {
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }

    let second = block_on(driver.start_run(
        succeeding.plan(DriverRole::StructuredRun),
        run_request("independent-second"),
        succeeding.services(),
    ))
    .expect("independent second run starts");
    let (second, _, second_outcome) = drain_run(second);
    assert_eq!(second_outcome.status(), &TerminalStatus::Completed);
    assert_eq!(succeeding.server.inference_attempts(), 1);

    block_on(first.cancellation().request()).expect("first cancellation is accepted");
    assert_eq!(
        block_on(first_terminal).status(),
        &TerminalStatus::Cancelled
    );
    assert!(matches!(
        block_on(first.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(matches!(
        block_on(second.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
}

#[test]
fn deadline_remains_distinct_from_cancellation() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        VersionFixture::Expected,
        StreamFixture::WaitForCancel,
    ));
    let request = run_request("deadline-run")
        .with_deadline(fixture.thread.deadline_after(Duration::from_millis(100)));
    let mut run = block_on(OllamaNativeAttachedDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .expect("run starts");
    let _events = run.take_events().expect("events are available");
    let outcome = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(fixture.server.is_reachable());
}

#[test]
fn task_join_failure_remains_visible_from_close() {
    let fixture = Fixture::new();
    let (run, _, outcome) = complete_run_with_services(
        &fixture,
        fixture.services_with_join_failure(),
        "join-failure",
    );

    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Failed(_)
    ));
    assert!(fixture.server.is_reachable());
}
