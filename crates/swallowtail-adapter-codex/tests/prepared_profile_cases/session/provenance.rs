#[test]
fn prepared_profile_keeps_exact_version_and_access_provenance_visible() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.146.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_catalogue(RequestId::new("catalogue-evidence").unwrap(), None)
        .expect("catalogue prepares");

    assert_eq!(
        profile
            .evidence()
            .observation()
            .version()
            .version()
            .as_str(),
        "0.146.0"
    );
    assert_eq!(
        profile.evidence().access().provenance(),
        &swallowtail_runtime::AccessEvidenceProvenance::CallerAsserted
    );
    swallowtail_testkit::assert_prepared_operation_evidence_matches_plan(
        profile.evidence().operation(),
        profile.plan(),
    );
    assert_eq!(
        profile
            .evidence()
            .operation()
            .observable_activity()
            .availability(),
        swallowtail_core::ObservableActivityAvailability::NotApplicable
    );
    assert_eq!(
        profile.evidence().operation().binding().driver_role(),
        DriverRole::ModelCatalog
    );
    assert_eq!(
        profile
            .evidence()
            .operation()
            .interface_compatibility()
            .count(),
        1
    );
    let compatibility = profile
        .evidence()
        .operation()
        .interface_compatibility()
        .next()
        .expect("Codex version evidence is present");
    let swallowtail_core::InterfaceCompatibilityAssessment::Qualified(matched) =
        compatibility.assessment()
    else {
        panic!("Codex 0.146.0 must be qualified");
    };
    assert_eq!(
        matched.support_status(),
        swallowtail_core::InterfaceSupportStatus::Maintained
    );
}
