fn complete(fixture: &Fixture, mode: FakeMode) -> (Box<dyn RunHandle>, TerminalOutcome, Vec<RuntimeEvent>, Arc<AtomicUsize>) {
    let calls = Arc::new(AtomicUsize::new(0));
    let driver = BedrockDirectDriver::with_executor(
        fixture.binding(), Arc::new(FakeExecutor { mode, calls: Arc::clone(&calls) }),
    );
    let mut run = block_on(driver.start_run(fixture.plan(), fixture.request("run"), fixture.services())).expect("run starts");
    let mut events = run.take_events().expect("events are available");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    let (events, outcome) = block_on(async {
        let mut collected = Vec::new();
        while let Some(event) = events.next().await { collected.push(event.expect("event succeeds")); }
        (collected, terminal.await)
    });
    (run, outcome, events, calls)
}

#[test]
fn local_and_remote_authoritative_hosts_complete_through_the_same_driver() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture = Fixture::new(host);
        let (run, outcome, events, calls) = complete(&fixture, FakeMode::Success);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.output().expect("output exists").as_str(), "Hello Bedrock");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
        assert!(events.iter().any(|event| matches!(event.kind(), RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_)))));
        assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
    }
}

#[test]
fn cancellation_wakes_private_executor_and_joins_cleanup() {
    let fixture = Fixture::new("host.cancel");
    let calls = Arc::new(AtomicUsize::new(0));
    let driver = BedrockDirectDriver::with_executor(
        fixture.binding(), Arc::new(FakeExecutor { mode: FakeMode::WaitForCancellation, calls: Arc::clone(&calls) }),
    );
    let mut run = block_on(driver.start_run(fixture.plan(), fixture.request("cancel"), fixture.services())).expect("run starts");
    let _events = run.take_events().expect("events are available");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    for _ in 0..100 {
        if calls.load(Ordering::SeqCst) == 1 { break; }
        thread::yield_now();
    }
    block_on(run.cancellation().request()).expect("cancellation succeeds");
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn deadline_cancels_sdk_work_and_reports_timeout() {
    let fixture = Fixture::new("host.deadline");
    let driver = BedrockDirectDriver::with_executor(
        fixture.binding(), Arc::new(FakeExecutor { mode: FakeMode::WaitForCancellation, calls: Arc::new(AtomicUsize::new(0)) }),
    );
    let request = fixture.request("deadline").with_deadline(Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(100)));
    let mut run = block_on(driver.start_run(fixture.plan(), request, fixture.services())).expect("run starts");
    let _events = run.take_events().expect("events are available");
    let outcome = block_on(run.take_terminal_outcome().expect("terminal is available"));
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn binding_and_input_mismatches_fail_before_sdk_work() {
    let fixture = Fixture::new("host.binding");
    let calls = Arc::new(AtomicUsize::new(0));
    let driver = BedrockDirectDriver::with_executor(
        fixture.binding(), Arc::new(FakeExecutor { mode: FakeMode::Success, calls: Arc::clone(&calls) }),
    );
    let request = StructuredRunRequest::new(
        RequestId::new("missing-limit").expect("request is valid"),
        OperationContent::new("fixture prompt").expect("content is valid"),
        swallowtail_runtime::OperationPolicy::offline(),
    );
    let error = block_on(driver.start_run(fixture.plan(), request, fixture.services())).err().expect("missing limit fails");
    assert_eq!(error.diagnostic().code(), "swallowtail.bedrock.output_limit_missing");
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[test]
fn internal_stream_failure_reports_provider_failed_and_does_not_deadlock_on_full_channel() {
    let fixture = Fixture::new("host.internal-failure");
    let (run, outcome, _events, calls) = complete(&fixture, FakeMode::EmptyDeltaThenFlood);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    match outcome.status() {
        TerminalStatus::ProviderFailed(diagnostic) => {
            assert_eq!(diagnostic.code(), "swallowtail.bedrock.activity_invalid");
        }
        other => panic!("expected ProviderFailed, got {other:?}"),
    }
    assert_eq!(fixture.acquires.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn empty_output_reports_provider_failed_instead_of_cancelled() {
    let fixture = Fixture::new("host.empty-output");
    let (run, outcome, _events, _calls) = complete(&fixture, FakeMode::EmptyOutput);
    match outcome.status() {
        TerminalStatus::ProviderFailed(diagnostic) => {
            assert_eq!(diagnostic.code(), "swallowtail.bedrock.empty_output");
        }
        other => panic!("expected ProviderFailed, got {other:?}"),
    }
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn oversized_output_is_bounded_and_reports_provider_failed() {
    let fixture = Fixture::new("host.oversized-output");
    let (run, outcome, _events, _calls) = complete(&fixture, FakeMode::OversizedOutput);
    match outcome.status() {
        TerminalStatus::ProviderFailed(diagnostic) => {
            assert_eq!(
                diagnostic.code(),
                "swallowtail.bedrock.output_limit_exceeded"
            );
        }
        other => panic!("expected ProviderFailed, got {other:?}"),
    }
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn early_start_run_errors_do_not_leak_credential_lease() {
    let fixture = Fixture::new("host.lease-leak");
    let driver = BedrockDirectDriver::with_executor(
        fixture.binding(),
        Arc::new(FakeExecutor {
            mode: FakeMode::Success,
            calls: Arc::new(AtomicUsize::new(0)),
        }),
    );

    let overflow_request = StructuredRunRequest::new(
        RequestId::new("overflow-limit").expect("request is valid"),
        OperationContent::new("fixture prompt").expect("content is valid"),
        swallowtail_runtime::OperationPolicy::offline(),
    )
    .with_maximum_output_tokens(NonZeroU64::new(i32::MAX as u64 + 1).expect("nonzero"));
    let error = block_on(driver.start_run(fixture.plan(), overflow_request, fixture.services()))
        .err()
        .expect("overflowing output limit must fail");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.bedrock.output_limit_invalid"
    );
    assert_eq!(
        fixture.acquires.load(Ordering::SeqCst),
        fixture.releases.load(Ordering::SeqCst)
    );

    let failing_services = fixture
        .services()
        .with_task(Arc::new(FailingTaskService) as Arc<dyn ScopedTaskService>);
    let error = block_on(driver.start_run(
        fixture.plan(),
        fixture.request("spawn-fail"),
        failing_services,
    ))
    .err()
    .expect("spawn failure must fail");
    assert_eq!(error.diagnostic().code(), "fixture.spawn_failed");
    assert_eq!(fixture.acquires.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
}

#[test]
fn events_stream_is_marked_terminal_before_terminal_outcome_completes() {
    let fixture = Fixture::new("host.terminal-order");
    let driver = BedrockDirectDriver::with_executor(
        fixture.binding(),
        Arc::new(FakeExecutor {
            mode: FakeMode::Success,
            calls: Arc::new(AtomicUsize::new(0)),
        }),
    );
    let mut run = block_on(driver.start_run(
        fixture.plan(),
        fixture.request("terminal-order"),
        fixture.services(),
    ))
    .expect("run starts");
    let mut events = run.take_events().expect("events are available");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);

    // Every event and the terminal EOF must already be ready without blocking.
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(waker);
    let mut saw_end = false;
    for _ in 0..64 {
        match events.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(_))) => {}
            Poll::Ready(None) => {
                saw_end = true;
                break;
            }
            Poll::Ready(Some(Err(error))) => panic!("unexpected event error: {error:?}"),
            Poll::Pending => panic!("event stream was not marked terminal before outcome"),
        }
    }
    assert!(saw_end, "event stream must reach terminal EOF immediately");
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}
