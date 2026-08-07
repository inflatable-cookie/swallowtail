fn assert_topology_and_prepared_evidence() {
    for fixture in fixtures() {
        let catalogue = fixture.catalogue_plan("fixture-catalogue", standard_bounds());
        let prepared_catalogue = fixture.prepared_catalogue(catalogue.clone());
        assert!(
            prepared_catalogue
                .operation()
                .matches_plan(catalogue.preflight())
        );

        let request = ProviderSessionCatalogueRequest::from_plan(
            request_id("fixture-catalogue-request"),
            &catalogue,
            None,
        )
        .expect("catalogue request is valid");
        let host = RecordingHostServices::for_host(
            fixture.topology().execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        validate_provider_session_catalogue_execution(&catalogue, &request, host.services())
            .expect("catalogue execution retains exact host authority");

        let candidate = fixture
            .candidate(
                &catalogue,
                "fixture-candidate",
                "provider/private/session",
                ProviderSessionImportAvailability::Available,
            )
            .expect("candidate is bounded");
        let import = fixture
            .import_plan(catalogue, candidate)
            .expect("import plan is valid");
        let prepared_import = fixture.prepared_import(import.clone());
        assert!(prepared_import.operation().matches_plan(import.preflight()));
        assert_eq!(
            prepared_import.plan().preflight().execution_host_id(),
            fixture.topology().execution_host_id()
        );
        let request =
            ProviderSessionImportRequest::from_plan(request_id("fixture-import-request"), &import)
                .expect("import request is valid");
        validate_provider_session_import_execution(&import, &request, host.services())
            .expect("import execution retains exact host authority");
        assert!(host.calls().is_empty());
    }
}

