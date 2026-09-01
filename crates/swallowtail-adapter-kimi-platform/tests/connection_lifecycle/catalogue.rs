#[test]
fn catalogue_and_one_k3_attempt_prepare_after_admission() {
    let services = prepared_services();
    let store = MemoryConnectionLifecycleStore::new();
    let (prepared, _, _) = prepared_after_admission(&services, &store);

    let catalogue = prepared
        .prepare_catalogue(KimiPlatformCatalogueProfileInput::new(
            RequestId::new("lifecycle-catalogue").expect("request id is valid"),
        ))
        .expect("catalogue prepares");
    assert_eq!(
        catalogue.plan().requirements().driver_role(),
        DriverRole::ModelCatalog
    );
    assert!(catalogue.plan().model_route_id().is_none());
    assert_prepared_operation_evidence_matches_plan(
        catalogue.evidence().operation(),
        catalogue.plan(),
    );

    let attempt = prepared
        .prepare_inference_attempt(KimiPlatformInferenceAttemptInput::new(
            RequestId::new("lifecycle-attempt").expect("request id is valid"),
            k3_model_selection(),
            OperationContent::new("lifecycle fixture prompt").expect("content is valid"),
            ReasoningMode::new("high").expect("reasoning is valid"),
            NonZeroU64::new(128).expect("output bound is valid"),
        ))
        .expect("attempt prepares");
    assert_eq!(
        attempt.plan().requirements().driver_role(),
        DriverRole::StructuredRun
    );
    assert_eq!(
        attempt.plan().model_id().expect("model").as_str(),
        KIMI_PLATFORM_MODEL_ID
    );
    assert_eq!(attempt.request().tools().len(), 0);
    assert_prepared_operation_evidence_matches_plan(attempt.evidence().operation(), attempt.plan());
}
