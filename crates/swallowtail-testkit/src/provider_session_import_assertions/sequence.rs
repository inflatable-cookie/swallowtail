fn assert_import_load_and_resume_sequence() {
    for fixture in fixtures() {
        let catalogue = fixture.catalogue_plan("fixture-continuation", standard_bounds());
        let selected = fixture
            .candidate(
                &catalogue,
                "fixture-continuation-candidate",
                "provider/private/continuation",
                ProviderSessionImportAvailability::Available,
            )
            .expect("continuation candidate is valid");
        let import = fixture
            .import_plan(catalogue, selected)
            .expect("continuation import is valid");
        let request = ProviderSessionImportRequest::from_plan(
            request_id("fixture-continuation-import"),
            &import,
        )
        .expect("continuation import request is valid");
        let outcome = ProviderSessionImportOutcome::new(
            &import,
            &request,
            ProviderSessionImportRevalidation::new(
                import.agreement().candidate_id().clone(),
                session_ref("provider/private/continuation"),
                fixture.topology().working_resource().clone(),
                ProviderSessionActivityState::Inactive,
                ProviderSessionImportAvailability::Available,
            ),
            CleanupOutcome::Clean,
        )
        .expect("matching revalidation issues one binding");
        assert_eq!(
            outcome.binding().origin(),
            ProviderSessionBindingOrigin::ExplicitlyImported
        );
        assert!(outcome.binding().matches_attachment(
            import.preflight(),
            fixture.topology().working_resource(),
            &fixture.access_policy(),
        ));

        let load = LoadSessionRequest::from_plan(
            import.preflight(),
            request_id("fixture-load-imported"),
            outcome.binding().clone(),
            fixture.topology().working_resource().clone(),
            None,
        )
        .expect("imported binding enters ordinary load");
        let resume = ResumeSessionRequest::from_plan(
            import.preflight(),
            request_id("fixture-resume-imported"),
            outcome.binding().clone(),
            fixture.topology().working_resource().clone(),
            None,
        )
        .expect("imported binding enters ordinary resume");
        let events = Arc::new(Mutex::new(Vec::new()));
        let driver = ContinuationFixtureDriver {
            events: Arc::clone(&events),
        };
        let host = RecordingHostServices::for_host(
            fixture.topology().execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        let loaded = poll_immediate(driver.load_session(
            import.preflight().clone(),
            load,
            host.services().clone(),
        ))
        .expect("load returns replay and a ready handle");
        assert_eq!(
            loaded
                .replay()
                .map(SessionReplayItem::sequence)
                .collect::<Vec<_>>(),
            [0, 1]
        );
        let (_replay, loaded_handle) = loaded.into_parts();
        poll_immediate(loaded_handle.close());
        let resumed = poll_immediate(driver.resume_session(
            import.preflight().clone(),
            resume,
            host.services().clone(),
        ))
        .expect("resume returns a ready handle without replay");
        poll_immediate(resumed.close());
        assert_eq!(
            *events.lock().expect("fixture event lock is valid"),
            [
                ContinuationEvent::Replay(0),
                ContinuationEvent::Replay(1),
                ContinuationEvent::ReadyAfterLoad,
                ContinuationEvent::ReadyAfterResume,
            ]
        );
        assert!(host.calls().is_empty());
    }
}

