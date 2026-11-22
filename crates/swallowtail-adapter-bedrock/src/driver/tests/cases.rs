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
fn lazy_blocking_work_host_starts_on_first_poll_and_completes_without_deadline() {
    let fixture = Fixture::new("host.lazy-blocking");
    let started = Arc::new(AtomicBool::new(false));
    let calls = Arc::new(AtomicUsize::new(0));
    let services = fixture.services().with_blocking_work(Arc::new(
        LazyBlockingWorkService {
            started: Arc::clone(&started),
        },
    ) as Arc<dyn BlockingWorkService>);
    let driver = BedrockDirectDriver::with_executor(
        fixture.binding(),
        Arc::new(FakeExecutor {
            mode: FakeMode::Success,
            calls: Arc::clone(&calls),
        }),
    );
    let mut run = block_on(driver.start_run(
        fixture.plan(),
        fixture.request("lazy-blocking"),
        services,
    ))
    .expect("run starts");
    let mut events = run.take_events().expect("events are available");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    let (events, outcome) = block_on(async {
        let mut collected = Vec::new();
        while let Some(event) = events.next().await {
            collected.push(event.expect("event succeeds"));
        }
        (collected, terminal.await)
    });
    assert!(started.load(Ordering::SeqCst));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output exists").as_str(),
        "Hello Bedrock"
    );
    assert!(!events.is_empty());
    assert_eq!(fixture.acquires.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn events_stream_is_marked_terminal_before_terminal_outcome_completes() {
    use std::sync::Condvar;
    use std::task::{Context, Wake, Waker};

    struct GatedSuccessExecutor {
        gate: Mutex<Option<oneshot::Receiver<()>>>,
    }

    impl SdkExecutor for GatedSuccessExecutor {
        fn execute(
            &self,
            _invocation: SdkInvocation,
            mut updates: mpsc::Sender<Result<StreamUpdate, RuntimeFailure>>,
            _cancelled: watch::Receiver<bool>,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let gate = self
                .gate
                .lock()
                .expect("gate lock poisoned")
                .take()
                .expect("gate receiver available");
            Box::pin(async move {
                let _ = gate.await;
                for update in [
                    StreamUpdate::MessageStarted,
                    StreamUpdate::TextDelta("Hello ".to_owned()),
                    StreamUpdate::TextDelta("Bedrock".to_owned()),
                    StreamUpdate::ContentBlockStopped,
                    StreamUpdate::MessageStopped(crate::StopKind::EndTurn),
                    StreamUpdate::Usage(crate::TokenUsage {
                        input: 2,
                        output: 2,
                        total: 4,
                    }),
                ] {
                    updates
                        .send(Ok(update))
                        .await
                        .map_err(|_| failure("fixture.receiver_closed", "Fixture receiver closed"))?;
                }
                Ok(())
            })
        }
    }

    struct TerminalOrderWaker {
        events: Mutex<Option<BoxEventStream>>,
        saw_eof: AtomicBool,
        saw_pending: AtomicBool,
        notified: (Mutex<bool>, Condvar),
    }

    impl Wake for TerminalOrderWaker {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            if let Some(mut events) = self
                .events
                .lock()
                .expect("events lock poisoned")
                .take()
            {
                let noop = Waker::noop();
                let mut cx = Context::from_waker(noop);
                for _ in 0..64 {
                    match events.as_mut().poll_next(&mut cx) {
                        Poll::Ready(Some(Ok(_))) => {}
                        Poll::Ready(None) => {
                            self.saw_eof.store(true, Ordering::SeqCst);
                            break;
                        }
                        Poll::Ready(Some(Err(error))) => {
                            panic!("unexpected event error: {error:?}");
                        }
                        Poll::Pending => {
                            self.saw_pending.store(true, Ordering::SeqCst);
                            break;
                        }
                    }
                }
            }
            let (lock, condvar) = &self.notified;
            *lock.lock().expect("notified lock poisoned") = true;
            condvar.notify_all();
        }
    }

    let fixture = Fixture::new("host.terminal-order");
    let (gate_sender, gate_receiver) = oneshot::channel();
    let driver = BedrockDirectDriver::with_executor(
        fixture.binding(),
        Arc::new(GatedSuccessExecutor {
            gate: Mutex::new(Some(gate_receiver)),
        }),
    );
    let mut run = block_on(driver.start_run(
        fixture.plan(),
        fixture.request("terminal-order"),
        fixture.services(),
    ))
    .expect("run starts");
    let events = run.take_events().expect("events are available");
    let mut terminal = run.take_terminal_outcome().expect("terminal is available");

    // Register a terminal waker that synchronously drains `events` inside
    // `terminal_sender.complete(outcome)`. Because the worker is gated until
    // after registration and we wait on the waker's condvar before polling
    // `terminal` again, this deterministically fails if `mark_terminal()`
    // is called after `complete(outcome)`.
    let observer = Arc::new(TerminalOrderWaker {
        events: Mutex::new(Some(events)),
        saw_eof: AtomicBool::new(false),
        saw_pending: AtomicBool::new(false),
        notified: (Mutex::new(false), Condvar::new()),
    });
    let waker = Waker::from(Arc::clone(&observer));
    let mut context = Context::from_waker(&waker);
    assert!(
        matches!(terminal.as_mut().poll(&mut context), Poll::Pending),
        "terminal future must still be pending before gate release"
    );

    gate_sender.send(()).expect("gate releases worker");
    let (lock, condvar) = &observer.notified;
    let mut done = lock.lock().expect("notified lock poisoned");
    while !*done {
        done = condvar.wait(done).expect("notified condvar poisoned");
    }
    drop(done);

    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert!(
        !observer.saw_pending.load(Ordering::SeqCst),
        "event stream returned Pending during terminal publication"
    );
    assert!(
        observer.saw_eof.load(Ordering::SeqCst),
        "event stream must already be marked terminal when outcome completes"
    );
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}
