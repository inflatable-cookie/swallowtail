#[test]
fn recoverable_run_emits_separate_authorities_before_message_and_restores_them_exactly() {
    let source = Fixture::new();
    let prepared = prepare_anthropic_managed_agent(source.preparation_input(), &source.services())
        .expect("managed integration prepares");
    let run = prepared
        .prepare_managed_run(
            source
                .prepared_run_input("prepared-cross-process", [])
                .with_cross_process_recovery(),
        )
        .expect("recoverable run prepares");
    let source_plan = run.plan().clone();
    let (handle, events, outcome) = complete_prepared(run.start_run(source.services()));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    let recovery = events
        .iter()
        .find(|event| {
            event.run_reconciliation_checkpoint().is_some()
                && event.recovered_resource_cleanup_binding().is_some()
        })
        .expect("recovery authorities are emitted together");
    let checkpoint = recovery
        .run_reconciliation_checkpoint()
        .expect("checkpoint exists")
        .export_persisted(&source_plan)
        .expect("checkpoint persists");
    let cleanup_binding = recovery
        .recovered_resource_cleanup_binding()
        .expect("cleanup binding exists")
        .export_persisted(&source_plan)
        .expect("cleanup binding persists");
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let requests = source.server.requests();
    let session_create = requests
        .iter()
        .position(|request| request.method == "POST" && request.target == "/v1/sessions")
        .expect("session creation exists");
    let message = requests
        .iter()
        .position(|request| {
            request.method == "POST" && request.target == "/v1/sessions/session_fixture/events"
        })
        .expect("message submission exists");
    assert!(session_create < message);

    for fixture_kind in [
        ManagedStreamFixture::Recovered,
        ManagedStreamFixture::RecoveredPaginated,
    ] {
        let recovered = Fixture::with_stream(fixture_kind);
        let prepared =
            prepare_anthropic_managed_agent(recovered.preparation_input(), &recovered.services())
                .expect("recovery integration prepares");
        let reconciliation = prepared
            .prepare_run_reconciliation(AnthropicManagedRunReconciliationInput::new(
                RequestId::new("reconcile-managed").expect("request id is valid"),
                managed_model(),
                checkpoint.clone(),
                NonZeroU64::new(4 * 1024).expect("bound is non-zero"),
                Some(recovered.deadline()),
            ))
            .expect("reconciliation prepares");
        let observation = block_on(reconciliation.reconcile(recovered.services()))
            .expect("reconciliation succeeds");
        assert_eq!(
            observation.observation().state(),
            InterruptedRunState::Completed
        );
        assert_eq!(
            observation
                .observation()
                .output()
                .expect("recovered output exists")
                .as_str(),
            "Recovered fixture complete."
        );
        let read_only_requests = recovered.server.requests();
        assert!(
            read_only_requests
                .iter()
                .all(|request| request.method == "GET")
        );
        assert_eq!(
            read_only_requests
                .iter()
                .filter(|request| request.target.contains("/events?"))
                .count(),
            if fixture_kind == ManagedStreamFixture::RecoveredPaginated {
                2
            } else {
                1
            }
        );

        let cleanup = prepared
            .prepare_recovered_cleanup(AnthropicManagedRecoveredCleanupInput::new(
                RequestId::new("cleanup-managed").expect("request id is valid"),
                managed_model(),
                cleanup_binding.clone(),
                Some(recovered.deadline()),
            ))
            .expect("cleanup prepares");
        let cleaned = block_on(cleanup.cleanup(recovered.services())).expect("cleanup completes");
        assert_eq!(
            cleaned.effect(),
            ProviderRecoveredResourceCleanupEffect::Applied
        );
        assert_delete_order(&recovered);
        let state = recovered.server.state();
        assert!(state.session_deleted && state.environment_deleted);
    }

    let active = Fixture::with_stream(ManagedStreamFixture::RecoveredActive);
    let prepared = prepare_anthropic_managed_agent(active.preparation_input(), &active.services())
        .expect("active recovery integration prepares");
    let cleanup = prepared
        .prepare_recovered_cleanup(AnthropicManagedRecoveredCleanupInput::new(
            RequestId::new("cleanup-active-managed").expect("request id is valid"),
            managed_model(),
            cleanup_binding.clone(),
            Some(active.deadline()),
        ))
        .expect("active cleanup prepares");
    let preserved = block_on(cleanup.cleanup(active.services())).expect("active state is observed");
    assert_eq!(
        preserved.effect(),
        ProviderRecoveredResourceCleanupEffect::RejectedActiveOrUnknown
    );
    assert!(
        active
            .server
            .requests()
            .iter()
            .all(|request| request.method == "GET")
    );
    assert!(!active.server.state().session_deleted);
    assert!(!active.server.state().environment_deleted);

    let partial = Fixture::with_stream(ManagedStreamFixture::RecoveredSessionDeleteFailure);
    let prepared =
        prepare_anthropic_managed_agent(partial.preparation_input(), &partial.services())
            .expect("partial recovery integration prepares");
    let cleanup = prepared
        .prepare_recovered_cleanup(AnthropicManagedRecoveredCleanupInput::new(
            RequestId::new("cleanup-partial-managed").expect("request id is valid"),
            managed_model(),
            cleanup_binding.clone(),
            Some(partial.deadline()),
        ))
        .expect("partial cleanup prepares");
    let uncertain =
        block_on(cleanup.cleanup(partial.services())).expect("partial truth is preserved");
    assert_eq!(
        uncertain.effect(),
        ProviderRecoveredResourceCleanupEffect::UnconfirmedAfterEffect
    );
    assert!(!partial.server.state().environment_deleted);
    assert!(
        !partial
            .server
            .requests()
            .iter()
            .any(|request| request.target == "/v1/environments/env_fixture")
    );

    let cancelled = Fixture::with_stream(ManagedStreamFixture::Recovered);
    let prepared =
        prepare_anthropic_managed_agent(cancelled.preparation_input(), &cancelled.services())
            .expect("cancelled recovery integration prepares");
    let cleanup = prepared
        .prepare_recovered_cleanup(AnthropicManagedRecoveredCleanupInput::new(
            RequestId::new("cleanup-cancelled-managed").expect("request id is valid"),
            managed_model(),
            cleanup_binding.clone(),
            Some(cancelled.deadline()),
        ))
        .expect("cancelled cleanup prepares");
    block_on(cleanup.request().cancellation().request()).expect("cancellation is accepted");
    let stopped = block_on(cleanup.cleanup(cancelled.services())).expect("cancellation is truth");
    assert_eq!(
        stopped.effect(),
        ProviderRecoveredResourceCleanupEffect::FailedBeforeEffect
    );
    assert!(cancelled.server.requests().is_empty());

    let expired = Fixture::with_stream(ManagedStreamFixture::Recovered);
    let prepared =
        prepare_anthropic_managed_agent(expired.preparation_input(), &expired.services())
            .expect("expired recovery integration prepares");
    let cleanup = prepared
        .prepare_recovered_cleanup(AnthropicManagedRecoveredCleanupInput::new(
            RequestId::new("cleanup-expired-managed").expect("request id is valid"),
            managed_model(),
            cleanup_binding,
            Some(Deadline::at(MonotonicInstant::from_ticks(0))),
        ))
        .expect("expired cleanup prepares");
    let stopped = block_on(cleanup.cleanup(expired.services())).expect("deadline is truth");
    assert_eq!(
        stopped.effect(),
        ProviderRecoveredResourceCleanupEffect::FailedBeforeEffect
    );
    assert!(expired.server.requests().is_empty());

    let cancelled = Fixture::with_stream(ManagedStreamFixture::Recovered);
    let prepared =
        prepare_anthropic_managed_agent(cancelled.preparation_input(), &cancelled.services())
            .expect("cancelled reconciliation integration prepares");
    let reconciliation = prepared
        .prepare_run_reconciliation(AnthropicManagedRunReconciliationInput::new(
            RequestId::new("reconcile-cancelled-managed").expect("request id is valid"),
            managed_model(),
            checkpoint.clone(),
            NonZeroU64::new(4 * 1024).expect("bound is non-zero"),
            Some(cancelled.deadline()),
        ))
        .expect("cancelled reconciliation prepares");
    block_on(reconciliation.request().cancellation().request())
        .expect("reconciliation cancellation is accepted");
    let error = block_on(reconciliation.reconcile(cancelled.services()))
        .expect_err("cancelled reconciliation rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.anthropic.managed.reconciliation_cancelled"
    );
    assert!(cancelled.server.requests().is_empty());

    let expired = Fixture::with_stream(ManagedStreamFixture::Recovered);
    let prepared =
        prepare_anthropic_managed_agent(expired.preparation_input(), &expired.services())
            .expect("expired reconciliation integration prepares");
    let reconciliation = prepared
        .prepare_run_reconciliation(AnthropicManagedRunReconciliationInput::new(
            RequestId::new("reconcile-expired-managed").expect("request id is valid"),
            managed_model(),
            checkpoint,
            NonZeroU64::new(4 * 1024).expect("bound is non-zero"),
            Some(Deadline::at(MonotonicInstant::from_ticks(0))),
        ))
        .expect("expired reconciliation prepares");
    let error = block_on(reconciliation.reconcile(expired.services()))
        .expect_err("expired reconciliation rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.anthropic.managed.reconciliation_timed_out"
    );
    assert!(expired.server.requests().is_empty());
}

fn managed_model() -> AnthropicManagedModelSelection {
    AnthropicManagedModelSelection::new(
        ModelRouteId::new("anthropic-managed-fixture").expect("route id is valid"),
        ModelRouteRevision::new("prepared-1").expect("route revision is valid"),
        ModelId::new("claude-fixture-model").expect("model id is valid"),
    )
}
