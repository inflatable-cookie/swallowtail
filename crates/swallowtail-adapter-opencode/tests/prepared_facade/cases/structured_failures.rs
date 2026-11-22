#[test]
fn structured_run_disconnect_and_delete_failure_remain_separate() {
    for (suffix, stream_fixture, terminal_code, deletion, cleanup_code) in [
        (
            "disconnect",
            crate::http_support::StreamFixture::Disconnect,
            Some("swallowtail.opencode.sse_disconnected"),
            RemoteResourceDeletionOutcome::Confirmed,
            None,
        ),
        (
            "delete-unconfirmed",
            crate::http_support::StreamFixture::DeleteMalformedSuccess,
            None,
            RemoteResourceDeletionOutcome::Unconfirmed,
            Some("swallowtail.opencode.run_delete_unconfirmed"),
        ),
    ] {
        let fixture = PreparedFixture::new_with_fixture(
            &format!("opencode.run.{suffix}"),
            "1.18.4",
            stream_fixture,
        );
        let prepared = fixture.prepared();
        let run = prepared
            .prepare_run(OpenCodeRunProfileInput::new(
                RequestId::new(format!("run-{suffix}")).unwrap(),
                fixture.model(),
                OperationContent::new("fixture private prompt").unwrap(),
                fixture.resource.clone(),
            ))
            .expect("structured run prepares");
        let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
        let outcome = block_on(
            handle
                .take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        match terminal_code {
            Some(code) => match outcome.status() {
                TerminalStatus::RuntimeFailed(diagnostic) => assert_eq!(diagnostic.code(), code),
                status => panic!("expected runtime failure, got {status:?}"),
            },
            None => assert_eq!(outcome.status(), &TerminalStatus::Completed),
        }
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
            Some(deletion)
        );
        match cleanup_code {
            Some(code) => match outcome.cleanup() {
                CleanupOutcome::Failed(diagnostic) => assert_eq!(diagnostic.code(), code),
                cleanup => panic!("expected failed cleanup, got {cleanup:?}"),
            },
            None => assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean),
        }
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn structured_run_deadline_still_deletes_session_and_releases_access() {
    use futures_channel::oneshot;
    use std::sync::{Arc, Mutex};
    use swallowtail_runtime::{BoxFuture, Deadline, DeadlineObservation, MonotonicInstant, TimeService};

    #[derive(Default)]
    struct Clock {
        state: Mutex<ClockState>,
    }

    #[derive(Default)]
    struct ClockState {
        ticks: u64,
        waiters: Vec<(Deadline, oneshot::Sender<DeadlineObservation>)>,
    }

    impl Clock {
        fn expire(&self, deadline: Deadline) {
            let mut state = self.state.lock().expect("clock lock");
            let now = deadline.instant();
            state.ticks = now.ticks();
            for (waiting, sender) in state.waiters.drain(..) {
                assert_eq!(waiting, deadline, "only the turn deadline is armed");
                let _ = sender.send(DeadlineObservation::new(waiting, now));
            }
        }
    }

    impl TimeService for Clock {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(self.state.lock().expect("clock lock").ticks)
        }

        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            let mut state = self.state.lock().expect("clock lock");
            let now = MonotonicInstant::from_ticks(state.ticks);
            if now >= deadline.instant() {
                return Box::pin(async move { DeadlineObservation::new(deadline, now) });
            }
            let (sender, receiver) = oneshot::channel();
            state.waiters.push((deadline, sender));
            Box::pin(async move { receiver.await.expect("clock remains alive") })
        }
    }

    let fixture = PreparedFixture::new_with_fixture(
        "opencode.run.deadline",
        "1.18.4",
        crate::http_support::StreamFixture::WaitForAbort,
    );
    let prepared = fixture.prepared();
    let clock = Arc::new(Clock::default());
    let deadline = Deadline::at(MonotonicInstant::from_ticks(100));
    let run = prepared
        .prepare_run(
            OpenCodeRunProfileInput::new(
                RequestId::new("run-deadline").unwrap(),
                fixture.model(),
                OperationContent::new("fixture private prompt").unwrap(),
                fixture.resource.clone(),
            )
            .with_deadline(deadline),
        )
        .expect("structured run prepares");
    let mut handle = block_on(run.start_run(fixture.services().with_time(clock.clone())))
        .expect("run starts before the deadline");
    let credential_releases = fixture.releases.load(Ordering::SeqCst);
    assert_eq!(fixture.resource_releases.load(Ordering::SeqCst), 0);
    clock.expire(deadline);

    let outcome = block_on(handle.take_terminal_outcome().expect("run exposes terminal outcome"));
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(
        outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
        Some(RemoteResourceDeletionOutcome::Confirmed)
    );
    assert_eq!(fixture.resource_releases.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.releases.load(Ordering::SeqCst), credential_releases + 1);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn unsupported_structured_input_stops_before_opencode_network_effects() {
    let fixture = PreparedFixture::new("opencode.run.unsupported", "1.18.4");
    let prepared = fixture.prepared();
    let run = prepared
        .prepare_run(OpenCodeRunProfileInput::new(
            RequestId::new("run-unsupported").unwrap(),
            fixture.model(),
            OperationContent::new("fixture private prompt").unwrap(),
            fixture.resource.clone(),
        ))
        .expect("structured run prepares");
    let request_count = fixture.server.requests().len();
    let (_, plan, request) = run.into_parts();
    let request =
        request.with_maximum_output_tokens(NonZeroU64::new(8).expect("non-zero token limit"));
    let error = block_on(
        swallowtail_adapter_opencode::OpenCodeHttpDriver::new().start_run(
            plan,
            request,
            fixture.services(),
        ),
    )
    .err()
    .expect("unsupported run fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.opencode.unsupported"
    );
    assert_eq!(fixture.server.requests().len(), request_count);
}

