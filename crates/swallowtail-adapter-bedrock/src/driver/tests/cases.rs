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
