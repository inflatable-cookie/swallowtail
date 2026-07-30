#[test]
fn prepared_catalogue_and_session_stay_separate_on_both_host_topologies() {
    for host_id in [
        "opencode.prepared.local",
        "opencode.prepared.remote-authoritative",
    ] {
        let fixture = PreparedFixture::new(host_id, "1.18.10");
        let prepared = fixture.prepared();
        assert_eq!(
            prepared.instance().ownership(),
            InstanceOwnership::ExternalAttached
        );
        assert_eq!(
            prepared.instance().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(prepared.server().binding().version().as_str(), "1.18.10");
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 1);

        let catalogue = prepared
            .prepare_catalogue(OpenCodeCatalogueProfileInput::new(
                RequestId::new("prepared-catalogue").unwrap(),
            ))
            .expect("catalogue prepares");
        assert_eq!(
            catalogue.plan().requirements().driver_role(),
            DriverRole::ModelCatalog
        );
        assert!(catalogue.plan().model_route_id().is_none());
        assert!(catalogue.plan().provider_id().is_none());
        assert_prepared_operation_evidence_matches_plan(
            catalogue.evidence().operation(),
            catalogue.plan(),
        );
        assert_eq!(
            catalogue
                .evidence()
                .operation()
                .observable_activity()
                .availability(),
            ObservableActivityAvailability::NotApplicable
        );
        let models =
            block_on(catalogue.list_models(fixture.services())).expect("catalogue succeeds");
        assert_eq!(models.len(), 1);
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 2);

        let session = prepared
            .prepare_session(OpenCodeSessionProfileInput::new(
                RequestId::new("prepared-session").unwrap(),
                fixture.model(),
                fixture.resource.clone(),
            ))
            .expect("session prepares");
        assert_eq!(
            session.plan().requirements().driver_role(),
            DriverRole::InteractiveSession
        );
        assert_eq!(
            session.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(session.plan().provider_id().unwrap().as_str(), "anthropic");
        assert_eq!(
            session.request().working_resource(),
            Some(&fixture.resource)
        );
        assert_prepared_operation_evidence_matches_plan(
            session.evidence().operation(),
            session.plan(),
        );
        assert_eq!(
            session
                .evidence()
                .operation()
                .observable_activity()
                .availability(),
            ObservableActivityAvailability::Available
        );
        let handle = block_on(session.open_session(fixture.services())).expect("session opens");
        assert_eq!(
            handle.provider_session_ref().unwrap().as_provider_value(),
            "ses_fixture"
        );
        let binding = handle
            .resume_binding()
            .expect("prepared session returns a continuity binding")
            .clone();
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 3);

        let loaded = block_on(
            session
                .load_session(
                    RequestId::new("prepared-load").unwrap(),
                    binding.clone(),
                    fixture.services(),
                )
                .expect("prepared load derives"),
        )
        .expect("prepared session loads");
        assert_eq!(
            loaded
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
        let (_, loaded_handle) = loaded.into_parts();
        assert_eq!(
            loaded_handle
                .management_binding()
                .expect("loaded session returns management authority")
                .origin(),
            swallowtail_core::ProviderSessionBindingOrigin::Loaded
        );
        assert_eq!(block_on(loaded_handle.close()), CleanupOutcome::Clean);

        let resumed = block_on(
            session
                .resume_session(
                    RequestId::new("prepared-resume").unwrap(),
                    binding,
                    fixture.services(),
                )
                .expect("prepared resume derives"),
        )
        .expect("prepared session resumes");
        assert_eq!(
            resumed
                .management_binding()
                .expect("resumed session returns management authority")
                .origin(),
            swallowtail_core::ProviderSessionBindingOrigin::Resumed
        );
        assert_eq!(block_on(resumed.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.releases.load(Ordering::SeqCst), 5);

        let requests = fixture.server.requests();
        assert!(!requests.iter().any(|request| {
            request.contains("/dispose")
                || request.contains("/delete")
                || request.contains("/share")
                || request.contains("/config")
        }));
    }
}

