#[test]
fn prepared_sessions_preserve_oh_my_pi_rpc_policy_in_both_host_topologies() {
    for host_value in ["fixture.pi.prepared.local", "fixture.pi.prepared.remote"] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let discovery = FixtureHost::version_probe("17.2.9");
        let prepared = block_on(prepare_oh_my_pi_rpc(
            preparation_input(host_id.clone()),
            probe(),
            discovery.services(host_id.clone()),
        ))
        .expect("OhMyPi prepares");
        assert_eq!(discovery.process_arguments(), ["--version"]);

        let profile = prepared
            .prepare_session(OhMyPiSessionProfileInput::new(
                RequestId::new("pi-prepared-open").expect("valid request"),
                OhMyPiModelSelection::new(
                    ModelRouteId::new("pi.prepared.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("fixture-provider").expect("valid provider"),
                    ModelId::new("fixture-model").expect("valid model"),
                ),
                WorkingResourceRef::new("pi.prepared.workspace").expect("valid resource"),
                SessionOptions::default(),
            ))
            .expect("OhMyPi session profile prepares");

        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::ProviderSuppressed)
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile.request().access_policy(),
            &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        );
        assert_eq!(
            profile.plan().provider_id().map(ProviderId::as_str),
            Some("fixture-provider")
        );
        assert!(profile.plan().harness_rpc_policy().is_some());
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        assert_eq!(
            profile
                .evidence()
                .operation()
                .observable_activity()
                .availability(),
            ObservableActivityAvailability::Available
        );
        assert_eq!(
            profile
                .prepare_working_state_restoration(
                    RuntimeTurnId::new("lost-pi-turn").expect("valid turn")
                )
                .method(),
            swallowtail_runtime::WorkingStateRestorationMethod::FreshSessionReplacement
        );

        let operation = FixtureHost::new(Scenario::Complete);
        let session = block_on(profile.open_session(operation.services(host_id)))
            .expect("prepared OhMyPi session opens");
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn prepared_session_and_run_bind_exact_reasoning_selection() {
    let host_id = ExecutionHostId::new("fixture.omp.reasoning").expect("valid host");
    let discovery = FixtureHost::version_probe("17.2.9");
    let prepared = block_on(prepare_oh_my_pi_rpc(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id.clone()),
    ))
    .expect("OhMyPi prepares");
    let low = ReasoningMode::new("low").expect("valid reasoning mode");
    let session = prepared
        .prepare_session(OhMyPiSessionProfileInput::new(
            RequestId::new("omp-reasoning-session").expect("valid request"),
            model("omp.reasoning.session.route"),
            WorkingResourceRef::new("omp.reasoning.workspace").expect("valid resource"),
            SessionOptions::default().with_reasoning_mode(low.clone()),
        ))
        .expect("reasoning session prepares");
    assert_eq!(session.request().options().reasoning_mode(), Some(&low));
    assert!(session.plan().requirements().capabilities().any(|requirement| {
        requirement.capability() == Capability::ReasoningSelection
            && requirement.constraints().any(|constraint| {
                constraint == &swallowtail_core::CapabilityConstraint::reasoning_mode(low.clone())
            })
    }));
    let operation = FixtureHost::new(Scenario::Complete);
    let opened = block_on(session.open_session(operation.services(host_id)))
        .expect("reasoning session opens");
    assert_eq!(block_on(opened.close()), CleanupOutcome::Clean);
    assert!(operation.inputs().iter().any(|input| {
        input["type"] == "set_thinking_level" && input["level"] == "low"
    }));

    let run = prepared
        .prepare_run(
            OhMyPiRunProfileInput::new(
                RequestId::new("omp-reasoning-run").expect("valid request"),
                model("omp.reasoning.run.route"),
                OperationContent::new("reason carefully").expect("valid content"),
                WorkingResourceRef::new("omp.reasoning.workspace").expect("valid resource"),
                Deadline::at(MonotonicInstant::from_ticks(100_000)),
            )
            .with_reasoning_mode(low.clone()),
        )
        .expect("reasoning run prepares");
    assert_eq!(run.request().policy().reasoning_mode(), Some(&low));

    let unsupported = ReasoningMode::new("turbo").expect("syntactically valid reasoning mode");
    let error = prepared
        .prepare_run(
            OhMyPiRunProfileInput::new(
                RequestId::new("omp-reasoning-invalid").expect("valid request"),
                model("omp.reasoning.invalid.route"),
                OperationContent::new("reason").expect("valid content"),
                WorkingResourceRef::new("omp.reasoning.workspace").expect("valid resource"),
                Deadline::at(MonotonicInstant::from_ticks(100_000)),
            )
            .with_reasoning_mode(unsupported),
        )
        .expect_err("unsupported reasoning is rejected before provider work");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.oh_my_pi.preparation.reasoning_mode_unsupported"
    );
}

#[test]
fn prepared_runs_preserve_the_one_prompt_rpc_projection_in_both_host_topologies() {
    for host_value in ["fixture.pi.run.local", "fixture.pi.run.remote"] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let discovery = FixtureHost::version_probe("17.2.9");
        let prepared = block_on(prepare_oh_my_pi_rpc(
            preparation_input(host_id.clone()),
            probe(),
            discovery.services(host_id.clone()),
        ))
        .expect("OhMyPi prepares");
        let run = prepared
            .prepare_run(OhMyPiRunProfileInput::new(
                RequestId::new("pi-prepared-run").expect("valid request"),
                OhMyPiModelSelection::new(
                    ModelRouteId::new("pi.prepared.run.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("fixture-provider").expect("valid provider"),
                    ModelId::new("fixture-model").expect("valid model"),
                ),
                OperationContent::new("fixture private prompt").expect("valid content"),
                WorkingResourceRef::new("pi.prepared.workspace").expect("valid resource"),
                Deadline::at(MonotonicInstant::from_ticks(100_000)),
            ))
            .expect("OhMyPi run profile prepares");
        assert_eq!(
            run.plan().requirements().driver_role(),
            swallowtail_core::DriverRole::StructuredRun
        );
        assert_eq!(
            run.request().policy().provider_retention(),
            ProviderRetentionPolicy::Prohibited
        );
        assert_prepared_operation_evidence_matches_plan(run.evidence().operation(), run.plan());
        assert_eq!(
            run.evidence()
                .operation()
                .observable_activity()
                .availability(),
            ObservableActivityAvailability::Available
        );

        let operation = FixtureHost::new(Scenario::Complete);
        let mut handle =
            block_on(run.start_run(operation.services(host_id))).expect("prepared run starts");
        let mut events = handle.take_events().expect("events are available");
        let terminal = handle
            .take_terminal_outcome()
            .expect("terminal outcome is available");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("runtime event succeeds");
            }
            terminal.await
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn prepared_sessions_and_runs_dispatch_one_bounded_png_and_release_it() {
    let session_host_id =
        ExecutionHostId::new("fixture.pi.attachment.session").expect("valid host");
    let session_discovery = FixtureHost::version_probe("17.2.9");
    let session_prepared = block_on(prepare_oh_my_pi_rpc(
        preparation_input(session_host_id.clone()),
        probe(),
        session_discovery.services(session_host_id.clone()),
    ))
    .expect("OhMyPi prepares");
    let session_profile = session_prepared
        .prepare_session(
            OhMyPiSessionProfileInput::new(
                RequestId::new("pi-image-session").expect("valid request"),
                model("pi.image.session.route"),
                WorkingResourceRef::new("pi.image.workspace").expect("valid resource"),
                SessionOptions::default(),
            )
            .with_image_attachments(),
        )
        .expect("image session prepares");
    assert!(
        session_profile
            .plan()
            .requirements()
            .capabilities()
            .any(|required| required.capability() == Capability::Attachments)
    );
    let session_host = FixtureHost::new(Scenario::Complete);
    let services = session_host.services(session_host_id);
    let mut session =
        block_on(session_profile.open_session(services.clone())).expect("image session opens");
    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("pi-image-turn").expect("valid turn"),
                OperationContent::new("inspect image").expect("valid content"),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(100_000)))
            .with_attachments([image("pi.image.session")]),
            services,
        ),
    )
    .expect("image turn starts");
    let terminal = turn
        .take_terminal_outcome()
        .expect("terminal outcome is available");
    assert_eq!(block_on(terminal).status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_prompt_image(&session_host);
    assert_eq!(
        session_host
            .cleanup_events()
            .iter()
            .filter(|event| **event == support::CleanupEvent::AttachmentRelease)
            .count(),
        1
    );

    let run_host_id = ExecutionHostId::new("fixture.pi.attachment.run").expect("valid host");
    let run_discovery = FixtureHost::version_probe("17.2.9");
    let run_prepared = block_on(prepare_oh_my_pi_rpc(
        preparation_input(run_host_id.clone()),
        probe(),
        run_discovery.services(run_host_id.clone()),
    ))
    .expect("OhMyPi prepares");
    let run = run_prepared
        .prepare_run(
            OhMyPiRunProfileInput::new(
                RequestId::new("pi-image-run").expect("valid request"),
                model("pi.image.run.route"),
                OperationContent::new("inspect image").expect("valid content"),
                WorkingResourceRef::new("pi.image.workspace").expect("valid resource"),
                Deadline::at(MonotonicInstant::from_ticks(100_000)),
            )
            .with_attachments([image("pi.image.run")]),
        )
        .expect("image run prepares");
    let run_host = FixtureHost::new(Scenario::Complete);
    let mut handle =
        block_on(run.start_run(run_host.services(run_host_id))).expect("image run starts");
    let terminal = handle
        .take_terminal_outcome()
        .expect("terminal outcome is available");
    assert_eq!(block_on(terminal).status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert_prompt_image(&run_host);
    assert_eq!(
        run_host
            .cleanup_events()
            .iter()
            .filter(|event| **event == support::CleanupEvent::AttachmentRelease)
            .count(),
        1
    );
}
