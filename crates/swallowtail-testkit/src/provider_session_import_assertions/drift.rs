fn assert_drift_and_stale_targets_fail_closed() {
    let local = ProviderSessionImportFixture::local();
    let remote = ProviderSessionImportFixture::remote_authoritative();
    let local_catalogue = local.catalogue_plan("fixture-local", standard_bounds());
    let remote_catalogue = remote.catalogue_plan("fixture-remote", standard_bounds());
    let local_candidate = local
        .candidate(
            &local_catalogue,
            "fixture-local-candidate",
            "provider/private/local",
            ProviderSessionImportAvailability::Available,
        )
        .expect("local candidate is valid");

    let copied = remote
        .import_plan(remote_catalogue.clone(), local_candidate)
        .expect_err("copied cross-plan candidate must fail");
    assert_eq!(
        copied.diagnostic().code(),
        "swallowtail.provider_session_import.candidate_plan_mismatch"
    );

    let remote_request = ProviderSessionCatalogueRequest::from_plan(
        request_id("fixture-remote-page"),
        &remote_catalogue,
        None,
    )
    .expect("remote request is valid");
    let remote_candidate = remote
        .candidate(
            &remote_catalogue,
            "fixture-remote-candidate",
            "provider/private/remote",
            ProviderSessionImportAvailability::Available,
        )
        .expect("remote candidate is valid");
    let remote_page = ProviderSessionCatalogueOutcome::new(
        &remote_catalogue,
        &remote_request,
        vec![remote_candidate],
        Some("private-remote-cursor".to_owned()),
        CleanupOutcome::Clean,
    )
    .expect("remote page is valid");
    let cross_plan_cursor = ProviderSessionCatalogueRequest::from_plan(
        request_id("fixture-cross-plan-page"),
        &local_catalogue,
        remote_page.next_cursor().cloned(),
    )
    .expect_err("cross-plan cursor must fail before effects");
    assert_eq!(
        cross_plan_cursor.diagnostic().code(),
        "swallowtail.provider_session_catalogue.cursor_plan_mismatch"
    );

    let selected = local
        .candidate(
            &local_catalogue,
            "fixture-selected",
            "provider/private/selected",
            ProviderSessionImportAvailability::Available,
        )
        .expect("selected candidate is valid");
    let import = local
        .import_plan(local_catalogue, selected)
        .expect("import plan is valid");
    let request =
        ProviderSessionImportRequest::from_plan(request_id("fixture-stale-import"), &import)
            .expect("import request is valid");
    for revalidation in [
        ProviderSessionImportRevalidation::new(
            import.agreement().candidate_id().clone(),
            session_ref("provider/private/disappeared"),
            local.topology().working_resource().clone(),
            ProviderSessionActivityState::Inactive,
            ProviderSessionImportAvailability::Available,
        ),
        ProviderSessionImportRevalidation::new(
            import.agreement().candidate_id().clone(),
            session_ref("provider/private/selected"),
            local.topology().working_resource().clone(),
            ProviderSessionActivityState::Active,
            ProviderSessionImportAvailability::Unavailable(
                ProviderSessionImportUnavailableReason::Active,
            ),
        ),
    ] {
        let failure = ProviderSessionImportOutcome::new(
            &import,
            &request,
            revalidation,
            CleanupOutcome::Clean,
        )
        .expect_err("stale or unavailable target must issue no binding");
        assert_eq!(
            failure.stage(),
            ProviderSessionOperationFailureStage::ImportRevalidation
        );
        assert!(!format!("{failure:?}").contains("provider/private"));
    }
}

