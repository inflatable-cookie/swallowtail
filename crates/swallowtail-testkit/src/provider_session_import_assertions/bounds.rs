fn assert_bounds_pagination_and_redaction() {
    let fixture = ProviderSessionImportFixture::local();
    let catalogue = fixture.catalogue_plan("fixture-bounded", standard_bounds());
    let first = fixture
        .candidate(
            &catalogue,
            "fixture-candidate-a",
            "provider/private/session-a",
            ProviderSessionImportAvailability::Available,
        )
        .expect("first candidate is bounded");
    let second = fixture
        .candidate(
            &catalogue,
            "fixture-candidate-b",
            "provider/private/session-b",
            ProviderSessionImportAvailability::Available,
        )
        .expect("second candidate is bounded");
    let request = ProviderSessionCatalogueRequest::from_plan(
        request_id("fixture-page-one"),
        &catalogue,
        None,
    )
    .expect("first page request is valid");
    let first_page = ProviderSessionCatalogueOutcome::new(
        &catalogue,
        &request,
        vec![first.clone(), second],
        Some("private-next-cursor".to_owned()),
        CleanupOutcome::Clean,
    )
    .expect("bounded page is valid");
    assert_eq!(first_page.candidates().len(), 2);
    assert_eq!(
        first_page
            .next_cursor()
            .expect("next cursor exists")
            .observed_candidates(),
        2
    );

    let next_request = ProviderSessionCatalogueRequest::from_plan(
        request_id("fixture-page-two"),
        &catalogue,
        first_page.next_cursor().cloned(),
    )
    .expect("second page request is valid");
    let duplicate = ProviderSessionCatalogueOutcome::new(
        &catalogue,
        &next_request,
        vec![first],
        None,
        CleanupOutcome::Clean,
    )
    .expect_err("cross-page duplicate must fail");
    assert_eq!(
        duplicate.diagnostic().code(),
        "swallowtail.provider_session_catalogue.duplicate_candidate"
    );

    let overflow = (0..3)
        .map(|index| {
            fixture
                .candidate(
                    &catalogue,
                    &format!("fixture-overflow-{index}"),
                    &format!("provider/private/overflow-{index}"),
                    ProviderSessionImportAvailability::Available,
                )
                .expect("overflow candidate is individually bounded")
        })
        .collect::<Vec<_>>();
    let page_failure = ProviderSessionCatalogueOutcome::new(
        &catalogue,
        &request,
        overflow,
        None,
        CleanupOutcome::Clean,
    )
    .expect_err("page-size overflow must fail");
    assert_eq!(
        page_failure.diagnostic().code(),
        "swallowtail.provider_session_catalogue.page_limit_exceeded"
    );

    let tight = fixture.catalogue_plan(
        "fixture-tight",
        provider_session_catalogue_bounds(1, 1, 4, 4, 4),
    );
    let content_failure = fixture
        .candidate(
            &tight,
            "fixture-private-candidate",
            "p",
            ProviderSessionImportAvailability::Available,
        )
        .expect_err("oversized provider content must fail");
    let reference_failure = fixture
        .candidate(
            &tight,
            "fixture-private-reference",
            "provider/private/oversized",
            ProviderSessionImportAvailability::Available,
        )
        .expect_err("oversized provider identity must fail");
    assert_eq!(
        content_failure.diagnostic().code(),
        "swallowtail.provider_session_catalogue.content_limit_exceeded"
    );
    assert_eq!(
        reference_failure.diagnostic().code(),
        "swallowtail.provider_session_catalogue.reference_limit_exceeded"
    );
    let diagnostic = format!("{content_failure:?}{reference_failure:?}");
    assert!(!diagnostic.contains("private provider title"));
    assert!(!diagnostic.contains("provider/private/oversized"));
    let page_debug = format!("{first_page:?}");
    assert!(!page_debug.contains("private-next-cursor"));
    assert!(!page_debug.contains("private provider title"));
    assert!(!page_debug.contains("private provider preview"));
    assert!(!page_debug.contains("provider/private/session"));
}

