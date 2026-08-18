#[test]
fn timeout_aborts_and_joins_without_becoming_cancellation() {
    let server = FixtureServer::start(StreamFixture::WaitForAbort);
    let fixture = Fixture::new(server.endpoint(), "host.remote-authoritative");
    let driver = OpenCodeHttpDriver::new();
    let mut session = block_on(driver.open_session(
        fixture.plan(DriverRole::InteractiveSession),
        open_session_request("deadline-session", fixture.resource.clone()),
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
        open_session_request("cancel-session", fixture.resource.clone()),
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

#[test]
fn admitted_detachment_closes_only_the_local_attachment_without_aborting_provider_work() {
    let server = FixtureServer::start(StreamFixture::WaitForAbort);
    let fixture = Fixture::new(server.endpoint(), "host.detachment");
    let driver = OpenCodeHttpDriver::new();
    let session_plan = fixture.detachable_session_plan();
    let mut session = block_on(driver.open_session(
        session_plan.clone(),
        open_detachable_session_request("detach-session", fixture.resource.clone()),
        fixture.services(),
    ))
    .expect("detachable session opens");
    let persisted = session
        .resume_binding()
        .expect("detachable session has a durable binding")
        .export_persisted(&session_plan)
        .expect("binding exports before shutdown");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("detach-turn").expect("turn id is valid"),
            OperationContent::new("continue remotely").expect("content is valid"),
        ),
        fixture.services(),
    ))
    .expect("turn starts");
    let terminal = turn
        .take_terminal_outcome()
        .expect("terminal outcome is available");
    let detachment = turn.detachment().expect("detachment is admitted");
    assert_eq!(
        detachment.scope(),
        swallowtail_core::OperationDetachmentScope::ActiveTurn
    );
    assert_eq!(
        block_on(detachment.request()).expect("detachment succeeds"),
        OperationDetachmentAcknowledgement::Requested
    );
    assert_eq!(
        block_on(detachment.request()).expect("detachment is idempotent"),
        OperationDetachmentAcknowledgement::AlreadyRequested
    );
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Detached);
    assert_eq!(
        block_on(turn.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    assert_eq!(
        block_on(session.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    let shutdown_requests = server.requests();
    assert!(!shutdown_requests.iter().any(|request| request.contains("/abort?")));
    assert!(!shutdown_requests
        .iter()
        .any(|request| request.starts_with("GET /session/status?")));
    assert!(!shutdown_requests
        .iter()
        .any(|request| request.starts_with("DELETE ")));

    let restored = SessionResumeBinding::restore_persisted(
        &PersistedSessionResumeBinding::from_bytes(persisted.as_bytes())
            .expect("persisted bytes survive restart"),
        &session_plan,
        &fixture.resource,
        &SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
    )
    .expect("detached session binding restores");
    let reconciliation_preflight = fixture.plan(DriverRole::ProviderSessionReconciliation);
    let reconciliation_plan = ProviderSessionReconciliationPlan::new(
        reconciliation_preflight,
        ProviderSessionReconciliationAgreement::new(
            restored,
            RuntimeTurnId::new("detach-turn").unwrap(),
            None,
            ProviderSessionReconciliationBounds::new(
                std::num::NonZeroU32::new(4).unwrap(),
                std::num::NonZeroU64::new(1024).unwrap(),
            ),
            None,
        ),
    )
    .expect("reconciliation plan binds the restored session");
    let reconciliation_request = ProviderSessionReconciliationRequest::from_plan(
        RequestId::new("reconcile-detached-turn").unwrap(),
        &reconciliation_plan,
    )
    .unwrap();
    let reconciliation = block_on(driver.reconcile_provider_session(
        reconciliation_plan,
        reconciliation_request,
        fixture.services(),
    ))
    .expect("detached provider session reconciles after restart");
    assert_eq!(
        reconciliation.state(),
        swallowtail_runtime::InterruptedTurnState::Active
    );
    assert_eq!(
        reconciliation.attribution(),
        swallowtail_runtime::InterruptedTurnAttribution::ProviderSession
    );

    let requests = server.requests();
    assert!(
        requests
            .iter()
            .any(|request| request.contains("/prompt_async?directory="))
    );
    assert!(!requests.iter().any(|request| request.contains("/abort?")));
    assert!(!requests.iter().any(|request| request.starts_with("DELETE ")));
}

#[test]
fn ordinary_session_plan_does_not_expose_detachment() {
    let server = FixtureServer::start(StreamFixture::Success);
    let fixture = Fixture::new(server.endpoint(), "host.no-detachment");
    let driver = OpenCodeHttpDriver::new();
    let mut session = block_on(driver.open_session(
        fixture.plan(DriverRole::InteractiveSession),
        open_session_request("ordinary-session", fixture.resource.clone()),
        fixture.services(),
    ))
    .expect("session opens");
    let turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("ordinary-turn").expect("turn id is valid"),
            OperationContent::new("complete").expect("content is valid"),
        ),
        fixture.services(),
    ))
    .expect("turn starts");
    assert!(turn.detachment().is_none());
    assert_eq!(block_on(turn.close()), swallowtail_runtime::CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), swallowtail_runtime::CleanupOutcome::Clean);
}

#[test]
fn cancellation_wins_before_detachment_and_still_aborts_provider_work() {
    let server = FixtureServer::start(StreamFixture::WaitForAbort);
    let fixture = Fixture::new(server.endpoint(), "host.detachment-cancelled");
    let driver = OpenCodeHttpDriver::new();
    let mut session = block_on(driver.open_session(
        fixture.detachable_session_plan(),
        open_detachable_session_request("detach-cancel-session", fixture.resource.clone()),
        fixture.services(),
    ))
    .expect("detachable session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("detach-cancel-turn").unwrap(),
            OperationContent::new("cancel").unwrap(),
        ),
        fixture.services(),
    ))
    .expect("turn starts");
    block_on(turn.cancellation().request()).expect("cancellation succeeds");
    let failure = block_on(
        turn.detachment()
            .expect("detachment control exists")
            .request(),
    )
    .expect_err("cancellation rejects later detachment");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.opencode.detachment_cancelled"
    );
    let outcome = block_on(turn.take_terminal_outcome().unwrap());
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), swallowtail_runtime::CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), swallowtail_runtime::CleanupOutcome::Clean);
    assert!(server.requests().iter().any(|request| request.contains("/abort?")));
}

#[test]
fn completed_turn_rejects_detachment_without_new_provider_work() {
    let server = FixtureServer::start(StreamFixture::Success);
    let fixture = Fixture::new(server.endpoint(), "host.detachment-terminal");
    let driver = OpenCodeHttpDriver::new();
    let mut session = block_on(driver.open_session(
        fixture.detachable_session_plan(),
        open_detachable_session_request("detach-terminal-session", fixture.resource.clone()),
        fixture.services(),
    ))
    .expect("detachable session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("detach-terminal-turn").unwrap(),
            OperationContent::new("complete").unwrap(),
        ),
        fixture.services(),
    ))
    .expect("turn starts");
    let outcome = block_on(turn.take_terminal_outcome().unwrap());
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    let failure = block_on(
        turn.detachment()
            .expect("detachment control exists")
            .request(),
    )
    .expect_err("terminal turn rejects detachment");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.opencode.detachment_terminal"
    );
    assert_eq!(block_on(turn.close()), swallowtail_runtime::CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), swallowtail_runtime::CleanupOutcome::Clean);
    assert!(!server.requests().iter().any(|request| request.contains("/abort?")));
}

#[test]
fn low_level_unverified_newer_plan_cannot_inherit_detachment() {
    let server = FixtureServer::start_with_version(StreamFixture::Success, "1.18.19");
    let fixture = Fixture::new_with_version(
        server.endpoint(),
        "host.detachment-newer",
        "1.18.19",
    );
    let failure = block_on(OpenCodeHttpDriver::new().open_session(
        fixture.detachable_session_plan(),
        open_detachable_session_request("detach-newer-session", fixture.resource.clone()),
        fixture.services(),
    ))
    .err()
    .expect("unverified newer plan cannot inherit detachment");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.opencode.detachment_version_unsupported"
    );
    assert!(server.requests().is_empty());
}
