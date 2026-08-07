fn assert_lifecycle_failures_remain_distinct() {
    let fixture = ProviderSessionImportFixture::local();
    let catalogue = fixture.catalogue_plan("fixture-lifecycle", standard_bounds());
    let request = ProviderSessionCatalogueRequest::from_plan(
        request_id("fixture-cancelled"),
        &catalogue,
        None,
    )
    .expect("catalogue request is valid");
    poll_immediate(request.cancellation().request()).expect("cancellation is recorded");
    assert!(request.cancellation().is_requested());
    assert_eq!(
        request.cancellation().scope(),
        CancellationScope::ProviderSessionCatalogue
    );

    let lifecycle = [
        ProviderSessionOperationFailure::new(
            ProviderSessionOperationFailureStage::Cancelled,
            SafeDiagnostic::new("fixture.cancelled", "Catalogue was cancelled"),
        ),
        ProviderSessionOperationFailure::new(
            ProviderSessionOperationFailureStage::TimedOut,
            SafeDiagnostic::new("fixture.timed_out", "Catalogue deadline elapsed"),
        ),
        ProviderSessionOperationFailure::new(
            ProviderSessionOperationFailureStage::CatalogueDispatch,
            SafeDiagnostic::new("fixture.disconnected", "Catalogue transport disconnected"),
        ),
    ];
    assert_eq!(
        lifecycle.map(|failure| failure.stage()),
        [
            ProviderSessionOperationFailureStage::Cancelled,
            ProviderSessionOperationFailureStage::TimedOut,
            ProviderSessionOperationFailureStage::CatalogueDispatch,
        ]
    );

    let cleanup = ProviderSessionCatalogueOutcome::new(
        &catalogue,
        &request,
        Vec::new(),
        None,
        CleanupOutcome::Failed(SafeDiagnostic::new(
            "fixture.cleanup_failed",
            "Fixture cleanup failed",
        )),
    )
    .expect_err("cleanup failure must prevent success");
    assert_eq!(
        cleanup.stage(),
        ProviderSessionOperationFailureStage::Cleanup
    );
}

