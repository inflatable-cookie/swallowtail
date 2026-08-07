#[test]
fn exact_interrupted_turn_reconciliation_projects_active_and_terminal_truth() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let session = prepared_app
        .prepare_read_only_session(session_input("reconciliation-session"))
        .expect("read-only session prepares");
    let session_plan = session.plan();
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("thread-provider-import").unwrap(),
        session_plan.instance_id().clone(),
        session_plan.execution_host_id().clone(),
        session_plan.model_route_id().unwrap().clone(),
        session_plan.model_id().unwrap().clone(),
        working_resource(),
        session.request().access_policy().clone(),
    );
    for (provider_turn, expected) in [
        ("turn-1", swallowtail_runtime::InterruptedTurnState::Active),
        (
            "turn-2",
            swallowtail_runtime::InterruptedTurnState::Completed,
        ),
    ] {
        let restoration = prepared_app
            .prepare_working_state_restoration(
                CodexSessionReconciliationInput::new(
                    RequestId::new(format!("reconcile-{provider_turn}")).unwrap(),
                    model(),
                    binding.clone(),
                    RuntimeTurnId::new(format!("runtime-{provider_turn}")).unwrap(),
                    ProviderSessionReconciliationBounds::new(
                        NonZeroU32::new(8).unwrap(),
                        NonZeroU64::new(4096).unwrap(),
                    ),
                )
                .with_provider_turn_ref(swallowtail_core::TurnRef::new(provider_turn).unwrap()),
            )
            .expect("exact restoration prepares");
        assert_eq!(
            restoration.method(),
            WorkingStateRestorationMethod::ProviderSessionReconciliation
        );
        let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
            ThreadCatalogueMode::Available,
        ));
        let restored = block_on(restoration.restore(host_services_with(
            process,
            &recording,
            [HostServiceKind::WorkingResource],
        )))
        .expect("exact turn reconciles");
        let WorkingStateRestorationOutcome::SessionReconciled(outcome) = restored else {
            panic!("Codex must preserve session reconciliation truth");
        };
        assert_eq!(outcome.state(), expected);
        assert_eq!(
            outcome.attribution(),
            swallowtail_runtime::InterruptedTurnAttribution::ExactProviderTurn
        );
        assert_eq!(
            outcome.provider_turn_ref().unwrap().as_provider_value(),
            provider_turn
        );
        assert!(outcome.replay_complete());
        assert_eq!(outcome.replay().count(), 2);
        let read = state
            .messages()
            .into_iter()
            .find(|message| message["method"] == "thread/read")
            .expect("thread/read is captured");
        assert_eq!(read["params"]["threadId"], "thread-provider-import");
        assert_eq!(read["params"]["includeTurns"], true);
        assert!(
            !state
                .methods()
                .iter()
                .any(|method| method == "turn/interrupt")
        );
    }

    let missing = prepared_app
        .prepare_session_reconciliation(
            CodexSessionReconciliationInput::new(
                RequestId::new("reconcile-missing-turn").unwrap(),
                model(),
                binding,
                RuntimeTurnId::new("runtime-missing-turn").unwrap(),
                ProviderSessionReconciliationBounds::new(
                    NonZeroU32::new(8).unwrap(),
                    NonZeroU64::new(4096).unwrap(),
                ),
            )
            .with_provider_turn_ref(swallowtail_core::TurnRef::new("turn-missing").unwrap()),
        )
        .unwrap();
    let (process, _) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    let failure = block_on(missing.reconcile(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect_err("missing exact turn fails closed");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.app_server.reconciliation_turn_missing"
    );
}

#[test]
fn active_codex_reconciliation_is_observed_without_starting_prepared_load() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let session = prepared_app
        .prepare_read_only_session(session_input("settled-load-session"))
        .expect("read-only session prepares");
    let plan = session.plan();
    let binding = SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("thread-provider-import").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        working_resource(),
        session.request().access_policy().clone(),
    );
    let reconciliation = prepared_app
        .prepare_session_reconciliation(
            CodexSessionReconciliationInput::new(
                RequestId::new("settled-reconciliation").unwrap(),
                model(),
                binding,
                RuntimeTurnId::new("runtime-active-turn").unwrap(),
                ProviderSessionReconciliationBounds::new(
                    NonZeroU32::new(8).unwrap(),
                    NonZeroU64::new(4096).unwrap(),
                ),
            )
            .with_provider_turn_ref(swallowtail_core::TurnRef::new("turn-1").unwrap()),
        )
        .expect("reconciliation prepares");
    let restoration = reconciliation
        .prepare_settled_session_restoration(session, RequestId::new("settled-load").unwrap())
        .expect("settled restoration prepares");
    assert_eq!(
        restoration.attachment_kind(),
        SettledSessionAttachmentKind::Load
    );

    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    let restored = block_on(restoration.restore(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("active reconciliation succeeds");
    let SettledSessionRestorationOutcome::Observed(outcome) = restored else {
        panic!("active Codex turn must not attach");
    };
    assert_eq!(
        outcome.state(),
        swallowtail_runtime::InterruptedTurnState::Active
    );
    assert_eq!(
        state
            .methods()
            .into_iter()
            .filter(|method| method == "thread/read")
            .count(),
        1
    );
    assert!(
        !state
            .methods()
            .iter()
            .any(|method| method == "thread/resume")
    );
}

