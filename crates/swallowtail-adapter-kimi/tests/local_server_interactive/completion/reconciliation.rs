#[test]
fn persisted_checkpoint_reconciles_the_exact_completed_turn_after_restart() {
    let first_server = InteractiveFixtureServer::start(InteractiveScenario::Complete);
    let first_host = FixtureHost::for_endpoint(first_server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.reconciliation");
    let first_services = first_host.services(execution_host.clone(), false);
    let first_prepared = prepare(execution_host.clone(), first_services.clone(), "0.29.0");
    let first_profile = session_profile(
        &first_prepared,
        KimiLocalServerPermissionMode::Auto,
        "reconciliation-source",
    );
    let mut session =
        block_on(first_profile.open_session(first_services.clone())).expect("source session opens");
    let binding = session
        .resume_binding()
        .expect("source binding exists")
        .clone();
    let mut turn =
        block_on(session.start_turn(turn("reconciliation-runtime-turn"), first_services.clone()))
            .expect("source turn starts");
    let events = block_on(
        turn.take_events()
            .expect("source event stream exists")
            .collect::<Vec<_>>(),
    );
    let checkpoint = events
        .iter()
        .filter_map(|event| event.as_ref().ok())
        .filter_map(|event| event.reconciliation_checkpoint())
        .find(|checkpoint| !checkpoint.cursor().is_empty())
        .expect("source checkpoint exists")
        .clone();
    let persisted = checkpoint
        .export_persisted(first_profile.plan(), &binding)
        .expect("checkpoint persists under source plan");
    assert_eq!(
        block_on(
            turn.take_terminal_outcome()
                .expect("source terminal exists")
        )
        .status(),
        &TerminalStatus::Completed
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(
        block_on(close_session(session, first_services)),
        CleanupOutcome::Clean
    );
    drop(first_server);

    let second_server = InteractiveFixtureServer::start(InteractiveScenario::ReconcileComplete);
    let second_host = FixtureHost::for_endpoint(second_server.endpoint());
    let second_services = second_host.services(execution_host.clone(), false);
    let second_prepared = prepare(execution_host, second_services.clone(), "0.29.0");
    let wrong_host_id = id(ExecutionHostId::new, "fixture.kimi.reconciliation.foreign");
    let wrong_services = second_host.services(wrong_host_id.clone(), false);
    let wrong_prepared = prepare(wrong_host_id, wrong_services, "0.29.0");
    let mismatch = wrong_prepared
        .prepare_session_reconciliation(KimiLocalServerReconciliationInput::new(
            id(RequestId::new, "foreign-reconciliation-request"),
            KimiModelSelection::new(
                id(ModelRouteId::new, "fixture.kimi.route"),
                id(ModelRouteRevision::new, "1"),
                id(ModelId::new, "kimi-k2.5"),
            ),
            binding.clone(),
            persisted.clone(),
            ProviderSessionReconciliationBounds::new(
                NonZeroU32::new(16).expect("bound is non-zero"),
                NonZeroU64::new(4096).expect("bound is non-zero"),
            ),
        ))
        .expect_err("cross-host checkpoint rejects");
    assert_eq!(
        mismatch.diagnostic().safe().code(),
        "swallowtail.provider_operation_checkpoint.attachment_mismatch"
    );
    let input = KimiLocalServerReconciliationInput::new(
        id(RequestId::new, "reconciliation-request"),
        KimiModelSelection::new(
            id(ModelRouteId::new, "fixture.kimi.route"),
            id(ModelRouteRevision::new, "1"),
            id(ModelId::new, "kimi-k2.5"),
        ),
        binding,
        persisted,
        ProviderSessionReconciliationBounds::new(
            NonZeroU32::new(16).expect("bound is non-zero"),
            NonZeroU64::new(4096).expect("bound is non-zero"),
        ),
    );
    let legacy = second_prepared
        .prepare_working_state_restoration(input.clone())
        .expect("read-only restoration still prepares");
    assert_eq!(
        legacy.method(),
        WorkingStateRestorationMethod::ProviderSessionReconciliation
    );
    let session = session_profile(
        &second_prepared,
        KimiLocalServerPermissionMode::Auto,
        "reconciliation-attachment",
    );
    let restoration = second_prepared
        .prepare_session_reconciliation(input)
        .expect("reconciliation prepares")
        .prepare_settled_session_restoration(session, id(RequestId::new, "reconciliation-resume"))
        .expect("settled restoration prepares");
    assert_eq!(
        restoration.attachment_kind(),
        SettledSessionAttachmentKind::Resume
    );
    let restored = block_on(restoration.restore(second_services.clone())).unwrap_or_else(|error| {
        panic!(
            "reconciliation executes: {error:?}; requests={:?}",
            second_server.requests()
        )
    });
    let SettledSessionRestorationOutcome::Attached(attached) = restored else {
        panic!("completed Kimi turn must resume its session");
    };
    let (outcome, attachment) = attached.into_parts();

    assert_eq!(
        outcome.attribution(),
        swallowtail_runtime::InterruptedTurnAttribution::ExactProviderTurn
    );
    assert_eq!(
        outcome.state(),
        swallowtail_runtime::InterruptedTurnState::Completed
    );
    assert_eq!(
        outcome
            .provider_turn_ref()
            .expect("exact provider turn remains bound")
            .as_provider_value(),
        "7"
    );
    let SettledSessionAttachment::Resumed(resumed) = attachment else {
        panic!("Kimi local-server settled attachment is replay-free resume");
    };
    assert_eq!(
        block_on(close_session(resumed, second_services)),
        CleanupOutcome::Clean
    );
    assert!(
        second_server
            .requests()
            .iter()
            .any(|request| { request.contains("GET /api/v1/sessions/interactive-session") })
    );
    assert!(second_server.requests().iter().all(|request| {
        !request.contains("/prompts")
            && !request.contains(":archive")
            && !request.contains("/approvals/")
            && !request.contains("/questions/")
    }));
}

#[test]
fn attached_turn_detaches_without_abort_and_reconciles_as_exact_active_work() {
    let first_server = InteractiveFixtureServer::start(InteractiveScenario::Detach);
    let first_host = FixtureHost::for_endpoint(first_server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.detachment");
    let first_services = first_host.services(execution_host.clone(), false);
    let first_prepared = prepare(execution_host.clone(), first_services.clone(), "0.29.0");
    let first_profile = first_prepared
        .prepare_session(super::fixture::session_input(
            "detachment-source",
            swallowtail_adapter_kimi::KimiLocalServerSessionConfiguration::new(
                KimiLocalServerPermissionMode::Auto,
            )
            .with_active_turn_detachment(),
        ))
        .expect("detachment profile prepares");
    let mut session = block_on(first_profile.open_session(first_services.clone()))
        .expect("detachment session opens");
    let binding = session
        .resume_binding()
        .expect("detachment binding exists")
        .clone();
    let mut turn = block_on(session.start_turn(
        turn("detachment-runtime-turn"),
        first_services.clone(),
    ))
        .expect("detachment turn starts");
    let mut events = turn.take_events().expect("detachment event stream exists");
    let checkpoint = block_on(async {
        loop {
            let event = events
                .next()
                .await
                .expect("event stream remains open")
                .expect("event is valid");
            if let Some(checkpoint) = event.reconciliation_checkpoint() {
                break checkpoint.clone();
            }
        }
    });
    let persisted = checkpoint
        .export_persisted(first_profile.plan(), &binding)
        .expect("detachment checkpoint persists");
    let detachment = turn.detachment().expect("detachment control exists");
    assert_eq!(
        block_on(detachment.request()).expect("detachment requests"),
        OperationDetachmentAcknowledgement::Requested
    );
    assert_eq!(
        block_on(detachment.request()).expect("detachment is idempotent"),
        OperationDetachmentAcknowledgement::AlreadyRequested
    );
    assert_eq!(
        block_on(
            turn.take_terminal_outcome()
                .expect("detachment terminal exists")
        )
        .status(),
        &TerminalStatus::Detached
    );
    drop(events);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(
        block_on(close_session(session, first_services)),
        CleanupOutcome::Clean
    );
    let first_requests = first_server.finish();
    assert!(first_requests.contains(&"WS observer closed".to_owned()));
    assert!(!first_requests.contains(&"WS unexpected control text".to_owned()));

    let second_server = InteractiveFixtureServer::start(InteractiveScenario::ReconcileActive);
    let second_host = FixtureHost::for_endpoint(second_server.endpoint());
    let second_services = second_host.services(execution_host.clone(), false);
    let second_prepared = prepare(execution_host, second_services.clone(), "0.29.0");
    let reconciliation = second_prepared
        .prepare_session_reconciliation(KimiLocalServerReconciliationInput::new(
            id(RequestId::new, "detachment-reconciliation"),
            KimiModelSelection::new(
                id(ModelRouteId::new, "fixture.kimi.route"),
                id(ModelRouteRevision::new, "1"),
                id(ModelId::new, "kimi-k2.5"),
            ),
            binding,
            persisted,
            ProviderSessionReconciliationBounds::new(
                NonZeroU32::new(16).expect("bound is non-zero"),
                NonZeroU64::new(4096).expect("bound is non-zero"),
            ),
        ))
        .expect("detached reconciliation prepares");
    let outcome =
        block_on(reconciliation.execute(second_services)).expect("detached turn reconciles");
    assert_eq!(
        outcome.state(),
        swallowtail_runtime::InterruptedTurnState::Active
    );
    assert_eq!(
        outcome.attribution(),
        swallowtail_runtime::InterruptedTurnAttribution::ExactProviderTurn
    );
    assert!(
        second_server
            .requests()
            .iter()
            .all(|request| { !request.contains("/prompts") && !request.contains("abort") })
    );
}
