use std::num::{NonZeroU32, NonZeroU64};

fn history_bounds(page_items: u32, snapshot_items: u32) -> ProviderSessionHistoryBounds {
    ProviderSessionHistoryBounds::new(
        NonZeroU32::new(page_items).unwrap(),
        NonZeroU64::new(4096).unwrap(),
        NonZeroU32::new(64).unwrap(),
        NonZeroU32::new(snapshot_items).unwrap(),
    )
}

#[test]
fn newest_first_history_pages_over_session_messages_without_control_dispatch() {
    let server = FixtureServer::start_with_version(StreamFixture::Success, "1.18.10");
    let fixture = Fixture::new_with_version(server.endpoint(), "host.session-history", "1.18.10");
    let plan = fixture.plan(DriverRole::ProviderSessionHistory);
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        fixture.resource.clone(),
        SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
    );
    let plan = ProviderSessionHistoryPlan::new(
        plan,
        ProviderSessionHistoryAgreement::new(
            ProviderSessionHistoryId::new("opencode-history-first").unwrap(),
            binding,
            history_bounds(2, 8),
            None,
        ),
    )
    .unwrap();
    let request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-first-page").unwrap(),
        &plan,
        None,
    )
    .unwrap();

    let first = block_on(OpenCodeHttpDriver::new().page_provider_session_history(
        plan.clone(),
        request,
        fixture.services(),
    ))
    .expect("first history page succeeds");

    assert_eq!(first.fetched_count(), 2);
    assert!(first.has_older());
    assert_eq!(first.total(), ProviderSessionHistoryTotal::Exact(4));
    assert_eq!(
        first
            .items()
            .filter_map(|item| item.content().map(OperationContent::as_str))
            .collect::<Vec<_>>(),
        ["Later question.", "Later answer."]
    );

    let older_cursor = first.older_cursor().expect("older cursor").clone();
    let older_request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-older-page").unwrap(),
        &plan,
        Some(older_cursor),
    )
    .unwrap();
    let older = block_on(OpenCodeHttpDriver::new().page_provider_session_history(
        plan,
        older_request,
        fixture.services(),
    ))
    .expect("older history page succeeds");

    assert_eq!(older.fetched_count(), 2);
    assert!(!older.has_older());
    assert!(older.older_cursor().is_none());
    assert_eq!(
        older
            .items()
            .filter_map(|item| item.content().map(OperationContent::as_str))
            .collect::<Vec<_>>(),
        ["Earlier question.", "Earlier answer."]
    );

    let requests = server.requests();
    assert!(requests.iter().any(|request| request.starts_with("GET /global/health")));
    assert!(requests.iter().any(|request| request.contains("/session/ses_fixture")));
    assert!(requests.iter().any(|request| request.contains("/session/") && request.contains("/message")));
    assert!(!requests.iter().any(|request| request.starts_with("POST ")));
    assert!(!requests.iter().any(|request| request.starts_with("DELETE ")));
}

#[test]
fn history_snapshot_overflow_fails_closed() {
    let server = FixtureServer::start_with_version(StreamFixture::Success, "1.18.10");
    let fixture = Fixture::new_with_version(server.endpoint(), "host.history-overflow", "1.18.10");
    let plan = fixture.plan(DriverRole::ProviderSessionHistory);
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        fixture.resource.clone(),
        SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
    );
    let plan = ProviderSessionHistoryPlan::new(
        plan,
        ProviderSessionHistoryAgreement::new(
            ProviderSessionHistoryId::new("opencode-history-overflow").unwrap(),
            binding,
            history_bounds(2, 2),
            None,
        ),
    )
    .unwrap();
    let request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-overflow").unwrap(),
        &plan,
        None,
    )
    .unwrap();

    let failure = block_on(OpenCodeHttpDriver::new().page_provider_session_history(
        plan,
        request,
        fixture.services(),
    ))
    .expect_err("snapshot overflow fails closed");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.provider_session_history.snapshot_limit_exceeded"
    );
    let requests = server.requests();
    assert!(!requests.iter().any(|request| request.starts_with("POST ")));
    assert!(!requests.iter().any(|request| request.starts_with("DELETE ")));
}

#[test]
fn history_pagination_timeout_and_cancellation_report_session_history_codes() {
    use futures_channel::oneshot;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Mutex};
    use swallowtail_runtime::{BoxFuture, Deadline, DeadlineObservation, MonotonicInstant, TimeService};

    #[derive(Default)]
    struct GatedClock {
        ticks: AtomicU64,
        waiters: Mutex<Vec<(Deadline, oneshot::Sender<DeadlineObservation>)>>,
    }

    impl GatedClock {
        fn advance_to(&self, ticks: u64) {
            self.ticks.store(ticks, Ordering::SeqCst);
            let now = MonotonicInstant::from_ticks(ticks);
            let mut waiters = self.waiters.lock().expect("clock waiters lock");
            let remaining = std::mem::take(&mut *waiters);
            for (deadline, sender) in remaining {
                if ticks >= deadline.instant().ticks() {
                    let _ = sender.send(DeadlineObservation::new(deadline, now));
                } else {
                    waiters.push((deadline, sender));
                }
            }
        }
    }

    impl TimeService for GatedClock {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(self.ticks.load(Ordering::SeqCst))
        }

        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            let current = self.ticks.load(Ordering::SeqCst);
            if current >= deadline.instant().ticks() {
                let now = MonotonicInstant::from_ticks(current);
                return Box::pin(async move { DeadlineObservation::new(deadline, now) });
            }
            let (sender, receiver) = oneshot::channel();
            self.waiters
                .lock()
                .expect("clock waiters lock")
                .push((deadline, sender));
            Box::pin(async move {
                receiver
                    .await
                    .unwrap_or_else(|_| DeadlineObservation::new(deadline, deadline.instant()))
            })
        }
    }

    // 1. Timeout gated on second-page dispatch reports `session_history.timed_out`.
    {
        let mut server = FixtureServer::start_with_version(StreamFixture::HoldHistoryPage, "1.18.10");
        let gate = server.delete_response_gate();
        let clock = Arc::new(GatedClock::default());
        let fixture =
            Fixture::new_with_version(server.endpoint(), "host.history-timeout", "1.18.10");
        let plan = fixture.plan(DriverRole::ProviderSessionHistory);
        let binding = SessionResumeBinding::new(
            swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
            plan.instance_id().clone(),
            plan.execution_host_id().clone(),
            plan.model_route_id().unwrap().clone(),
            plan.model_id().unwrap().clone(),
            fixture.resource.clone(),
            SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
        );
        let plan = ProviderSessionHistoryPlan::new(
            plan,
            ProviderSessionHistoryAgreement::new(
                ProviderSessionHistoryId::new("opencode-history-timeout").unwrap(),
                binding,
                history_bounds(2, 8),
                Some(Deadline::at(MonotonicInstant::from_ticks(100))),
            ),
        )
        .unwrap();
        let request = ProviderSessionHistoryRequest::from_plan(
            RequestId::new("history-timeout").unwrap(),
            &plan,
            None,
        )
        .unwrap();
        let trigger_clock = Arc::clone(&clock);
        let trigger_gate = gate.clone();
        let trigger_thread = std::thread::spawn(move || {
            trigger_gate.wait_for_dispatch();
            trigger_clock.advance_to(100);
        });

        let failure = block_on(OpenCodeHttpDriver::new().page_provider_session_history(
            plan,
            request,
            fixture.services().with_time(clock),
        ))
        .expect_err("pending second page must time out");
        gate.release();
        trigger_thread.join().expect("trigger thread joins");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.opencode.session_history.timed_out"
        );
        assert!(
            server
                .requests()
                .iter()
                .any(|req| req.contains("/session/ses_fixture/message?")
                    && req.contains("before=cursor-older")),
            "timeout must fire after the second-page request entered the handler"
        );
        server.shutdown();
    }

    // 2. Cancellation gated on second-page dispatch reports `session_history.cancelled`.
    {
        let mut server = FixtureServer::start_with_version(StreamFixture::HoldHistoryPage, "1.18.10");
        let gate = server.delete_response_gate();
        let fixture =
            Fixture::new_with_version(server.endpoint(), "host.history-cancel", "1.18.10");
        let plan = fixture.plan(DriverRole::ProviderSessionHistory);
        let binding = SessionResumeBinding::new(
            swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
            plan.instance_id().clone(),
            plan.execution_host_id().clone(),
            plan.model_route_id().unwrap().clone(),
            plan.model_id().unwrap().clone(),
            fixture.resource.clone(),
            SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
        );
        let plan = ProviderSessionHistoryPlan::new(
            plan,
            ProviderSessionHistoryAgreement::new(
                ProviderSessionHistoryId::new("opencode-history-cancel").unwrap(),
                binding,
                history_bounds(2, 8),
                None,
            ),
        )
        .unwrap();
        let request = ProviderSessionHistoryRequest::from_plan(
            RequestId::new("history-cancel").unwrap(),
            &plan,
            None,
        )
        .unwrap();
        let cancellation = request.cancellation().clone();
        let cancel_gate = gate.clone();
        let cancel_thread = std::thread::spawn(move || {
            cancel_gate.wait_for_dispatch();
            let _ = block_on(swallowtail_runtime::CancellationControl::request(
                cancellation.as_ref(),
            ));
        });

        let failure = block_on(OpenCodeHttpDriver::new().page_provider_session_history(
            plan,
            request,
            fixture.services(),
        ))
        .expect_err("pending second page must cancel");
        gate.release();
        cancel_thread.join().expect("cancel thread joins");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.opencode.session_history.cancelled"
        );
        assert!(
            server
                .requests()
                .iter()
                .any(|req| req.contains("/session/ses_fixture/message?")
                    && req.contains("before=cursor-older")),
            "cancellation must fire after the second-page request entered the handler"
        );
        server.shutdown();
    }
}
