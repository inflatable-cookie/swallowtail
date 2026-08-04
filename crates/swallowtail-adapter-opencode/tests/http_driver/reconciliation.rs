#[test]
fn interrupted_session_reconciliation_clears_idle_work_without_claiming_terminal_truth() {
    let server = FixtureServer::start_with_version(StreamFixture::Success, "1.18.10");
    let fixture = Fixture::new_with_version(server.endpoint(), "host.reconciliation", "1.18.10");
    let plan = fixture.plan(DriverRole::ProviderSessionReconciliation);
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        fixture.resource.clone(),
        SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
    );
    let plan = ProviderSessionReconciliationPlan::new(
        plan,
        ProviderSessionReconciliationAgreement::new(
            binding,
            RuntimeTurnId::new("turn-before-restart").unwrap(),
            None,
            ProviderSessionReconciliationBounds::new(
                std::num::NonZeroU32::new(4).unwrap(),
                std::num::NonZeroU64::new(1024).unwrap(),
            ),
            None,
        ),
    )
    .unwrap();
    let request = ProviderSessionReconciliationRequest::from_plan(
        RequestId::new("reconcile-after-restart").unwrap(),
        &plan,
    )
    .unwrap();

    let invalid_terminal = swallowtail_runtime::ProviderSessionReconciliationOutcome::new(
        &plan,
        &request,
        swallowtail_runtime::ProviderSessionReconciliationObservation::session_scoped(
            swallowtail_runtime::InterruptedTurnState::Completed,
            Vec::new(),
            true,
        ),
        swallowtail_runtime::CleanupOutcome::Clean,
    )
    .expect_err("session-scoped evidence cannot claim terminal truth");
    assert_eq!(
        invalid_terminal.diagnostic().code(),
        "swallowtail.provider_session_reconciliation.terminal_attribution_required"
    );

    let outcome = block_on(OpenCodeHttpDriver::new().reconcile_provider_session(
        plan,
        request,
        fixture.services(),
    ))
    .expect("reconciliation succeeds");

    assert_eq!(
        outcome.state(),
        swallowtail_runtime::InterruptedTurnState::InactiveUnresolved
    );
    assert_eq!(
        outcome.attribution(),
        swallowtail_runtime::InterruptedTurnAttribution::ProviderSession
    );
    assert!(outcome.provider_turn_ref().is_none());
    assert!(outcome.replay_complete());
    assert_eq!(
        outcome
            .replay()
            .filter_map(|item| item.content().map(OperationContent::as_str))
            .collect::<Vec<_>>(),
        [
            "Earlier question.",
            "Earlier answer.",
            "Later question.",
            "Later answer."
        ]
    );
    assert!(server.requests().iter().any(|request| request.starts_with("GET /session/status?")));
    assert!(!server.requests().iter().any(|request| request.starts_with("POST ")));
    assert!(!server.requests().iter().any(|request| request.starts_with("DELETE ")));
}

#[test]
fn active_reconciliation_observes_without_aborting_or_resuming_provider_work() {
    let server = FixtureServer::start_with_version(StreamFixture::ReconciliationActive, "1.18.10");
    let fixture = Fixture::new_with_version(
        server.endpoint(),
        "host.active-reconciliation",
        "1.18.10",
    );
    let plan = fixture.plan(DriverRole::ProviderSessionReconciliation);
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        fixture.resource.clone(),
        SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
    );
    let plan = ProviderSessionReconciliationPlan::new(
        plan,
        ProviderSessionReconciliationAgreement::new(
            binding,
            RuntimeTurnId::new("active-turn-before-restart").unwrap(),
            None,
            ProviderSessionReconciliationBounds::new(
                std::num::NonZeroU32::new(4).unwrap(),
                std::num::NonZeroU64::new(1024).unwrap(),
            ),
            None,
        ),
    )
    .unwrap();
    let request = ProviderSessionReconciliationRequest::from_plan(
        RequestId::new("observe-active-after-restart").unwrap(),
        &plan,
    )
    .unwrap();

    let outcome = block_on(OpenCodeHttpDriver::new().reconcile_provider_session(
        plan,
        request,
        fixture.services(),
    ))
    .expect("active reconciliation succeeds");

    assert_eq!(
        outcome.state(),
        swallowtail_runtime::InterruptedTurnState::Active
    );
    assert!(outcome.replay_complete());
    assert_eq!(outcome.replay().count(), 4);
    let requests = server.requests();
    assert!(!requests.iter().any(|request| request.contains("/abort")));
    assert!(!requests.iter().any(|request| request.contains("/prompt_async")));
}
