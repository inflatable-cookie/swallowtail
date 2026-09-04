#[test]
fn prepared_structured_run_is_private_and_deletes_its_session_on_both_host_topologies() {
    for host_id in ["opencode.run.local", "opencode.run.remote-authoritative"] {
        let fixture = PreparedFixture::new(host_id, "1.18.10");
        let prepared = fixture.prepared();
        let run = prepared
            .prepare_run(OpenCodeRunProfileInput::new(
                RequestId::new("prepared-run").unwrap(),
                fixture.model(),
                OperationContent::new("fixture private prompt").unwrap(),
                fixture.resource.clone(),
            ))
            .expect("structured run prepares");
        assert_eq!(
            run.plan().requirements().driver_role(),
            DriverRole::StructuredRun
        );
        assert_eq!(
            run.request().policy().provider_retention(),
            ProviderRetentionPolicy::TemporaryAllowed
        );
        assert_prepared_operation_evidence_matches_plan(run.evidence().operation(), run.plan());
        assert_eq!(
            run.evidence()
                .operation()
                .observable_activity()
                .availability(),
            ObservableActivityAvailability::Available
        );

        let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
        assert!(handle.provider_run_ref().is_none());
        assert_eq!(
            handle.cancellation().scope(),
            swallowtail_core::CancellationScope::StructuredRun
        );
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
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
            Some(RemoteResourceDeletionOutcome::Confirmed)
        );
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 2);

        let requests = fixture.server.requests();
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.starts_with("POST /session?directory="))
                .count(),
            1
        );
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.contains("/prompt_async?directory="))
                .count(),
            1
        );
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.starts_with("DELETE /session/ses_fixture?directory="))
                .count(),
            1
        );
        let prompt = requests
            .iter()
            .position(|request| request.contains("/prompt_async?directory="))
            .expect("prompt request observed");
        let delete = requests
            .iter()
            .position(|request| request.starts_with("DELETE /session/ses_fixture?directory="))
            .expect("delete request observed");
        assert!(prompt < delete);
    }
}

#[test]
fn prepared_generation_controls_use_exact_catalogue_evidence_and_zero_retry_dispatch() {
    for (host_id, version) in [
        ("opencode.controls.local", "1.18.10"),
        ("opencode.controls.remote-authoritative", "1.18.28"),
    ] {
        let fixture = PreparedFixture::new(host_id, version);
        let prepared = fixture.prepared();
        let mut models = block_on(
            prepared
                .prepare_catalogue(OpenCodeCatalogueProfileInput::new(
                    RequestId::new(format!("controls-catalogue-{host_id}"))
                        .expect("request id is valid"),
                ))
                .expect("catalogue prepares")
                .list_models(fixture.services()),
        )
        .expect("catalogue succeeds");
        let reasoning = ReasoningMode::new("high").expect("reasoning is valid");
        let run = prepared
            .prepare_run(
                OpenCodeRunProfileInput::new(
                    RequestId::new(format!("controls-run-{host_id}")).expect("request id is valid"),
                    fixture.model().with_catalogue_entry(models.remove(0)),
                    OperationContent::new("Return one fixture result").expect("content is valid"),
                    fixture.resource.clone(),
                )
                .with_reasoning_mode(reasoning.clone())
                .with_structured_output(schema()),
            )
            .expect("generation controls prepare");
        assert!(run.plan().requirements().capabilities().any(|requirement| {
            requirement.capability() == Capability::ReasoningSelection
                && requirement
                    .constraints()
                    .eq([&CapabilityConstraint::ReasoningMode(reasoning.clone())])
        }));
        assert!(run.plan().requirements().capabilities().any(|requirement| {
            requirement.capability() == Capability::StructuredOutput
                && requirement.constraints().any(|constraint| {
                    constraint
                        == &CapabilityConstraint::StructuredOutputEnforcement(
                            StructuredOutputEnforcement::HarnessValidated,
                        )
                })
        }));
        let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
        let outcome = block_on(
            handle
                .take_terminal_outcome()
                .expect("terminal outcome exists"),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

        let request_count = fixture.server.requests().len();
        let error = prepared
            .prepare_run(
                OpenCodeRunProfileInput::new(
                    RequestId::new(format!("controls-missing-{host_id}"))
                        .expect("request id is valid"),
                    fixture.model(),
                    OperationContent::new("No catalogue evidence").expect("content is valid"),
                    fixture.resource.clone(),
                )
                .with_reasoning_mode(reasoning.clone()),
            )
            .expect_err("missing catalogue evidence fails");
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.opencode.preparation.catalogue_evidence_missing"
        );
        assert_eq!(fixture.server.requests().len(), request_count);
    }
}
