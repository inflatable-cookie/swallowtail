use super::*;

#[test]
fn cursors_and_candidates_reject_cross_plan_reuse() {
    let fixture = fixture();
    let first = catalogue_plan(&fixture, "catalogue-a", "resource-a", bounds(128, 128));
    let second = catalogue_plan(&fixture, "catalogue-b", "resource-a", bounds(128, 128));
    let cursor = ProviderSessionCursor::new(
        &first,
        "private-cursor",
        std::collections::BTreeSet::new(),
        std::collections::BTreeSet::new(),
    )
    .expect("cursor is valid");
    let selected = candidate(
        &first,
        "candidate-a",
        ProviderSessionImportAvailability::Available,
    );

    let cursor_error = ProviderSessionCatalogueRequest::from_plan(
        RequestId::new("list-b").expect("request id is valid"),
        &second,
        Some(cursor),
    )
    .expect_err("cross-plan cursor must fail");
    let candidate_error = import_plan(&fixture, second, selected, "resource-a", true)
        .expect_err("cross-plan candidate must fail");

    assert_eq!(
        cursor_error.diagnostic().code(),
        "swallowtail.provider_session_catalogue.cursor_plan_mismatch"
    );
    assert_eq!(
        candidate_error.diagnostic().code(),
        "swallowtail.provider_session_import.candidate_plan_mismatch"
    );
}

#[test]
fn import_requires_complete_continuation_and_exact_resource() {
    let fixture = fixture();
    let source = catalogue_plan(&fixture, "catalogue-a", "resource-a", bounds(128, 128));
    let selected = candidate(
        &source,
        "candidate-a",
        ProviderSessionImportAvailability::Available,
    );
    let missing_resume = import_plan(
        &fixture,
        source.clone(),
        selected.clone(),
        "resource-a",
        false,
    )
    .expect_err("resume capability is required");
    let resource_drift = import_plan(&fixture, source.clone(), selected, "resource-b", true)
        .expect_err("resource drift must fail");
    let unavailable = candidate(
        &source,
        "candidate-b",
        ProviderSessionImportAvailability::Unavailable(
            ProviderSessionImportUnavailableReason::Active,
        ),
    );
    let unavailable_error = import_plan(&fixture, source, unavailable, "resource-a", true)
        .expect_err("unavailable candidate must fail");

    assert_eq!(
        missing_resume.diagnostic().code(),
        "swallowtail.provider_session_import.capability_mismatch"
    );
    assert_eq!(
        resource_drift.diagnostic().code(),
        "swallowtail.provider_session_import.resource_mismatch"
    );
    assert_eq!(
        unavailable_error.diagnostic().code(),
        "swallowtail.provider_session_import.candidate_unavailable"
    );
}

#[test]
fn request_validation_rejects_immutable_plan_drift() {
    let fixture = fixture();
    let first_catalogue = catalogue_plan(&fixture, "catalogue-a", "resource-a", bounds(128, 128));
    let second_catalogue = catalogue_plan(&fixture, "catalogue-b", "resource-a", bounds(128, 128));
    let catalogue_request = ProviderSessionCatalogueRequest::from_plan(
        RequestId::new("list-a").expect("request id is valid"),
        &first_catalogue,
        None,
    )
    .expect("catalogue request is valid");
    let first_candidate = candidate(
        &first_catalogue,
        "candidate-a",
        ProviderSessionImportAvailability::Available,
    );
    let second_candidate = candidate(
        &second_catalogue,
        "candidate-b",
        ProviderSessionImportAvailability::Available,
    );
    let first_import = import_plan(
        &fixture,
        first_catalogue,
        first_candidate,
        "resource-a",
        true,
    )
    .expect("first import is valid");
    let second_import = import_plan(
        &fixture,
        second_catalogue.clone(),
        second_candidate,
        "resource-a",
        true,
    )
    .expect("second import is valid");
    let import_request = ProviderSessionImportRequest::from_plan(
        RequestId::new("import-a").expect("request id is valid"),
        &first_import,
    )
    .expect("import request is valid");

    let catalogue_error =
        validate_provider_session_catalogue_request(&second_catalogue, &catalogue_request)
            .expect_err("catalogue drift must fail");
    let import_error = validate_provider_session_import_request(&second_import, &import_request)
        .expect_err("import drift must fail");

    assert_eq!(
        catalogue_error.diagnostic().code(),
        "swallowtail.provider_session_catalogue.plan_mismatch"
    );
    assert_eq!(
        import_error.diagnostic().code(),
        "swallowtail.provider_session_import.plan_mismatch"
    );
}
