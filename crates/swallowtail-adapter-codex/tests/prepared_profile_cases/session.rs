use super::*;

#[test]
fn session_resume_agreement_is_derived_and_unsupported_deadlines_fail_in_preparation() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.145.0", &recording, false);
    let profile = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("open").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default(),
        ))
        .expect("read-only session prepares");
    let binding = support::session_resume_binding(profile.plan(), "thread-1");
    let resume = profile
        .resume_request(RequestId::new("resume").unwrap(), binding.clone())
        .expect("resume request derives immutable agreement");
    assert_eq!(resume.access_policy(), profile.request().access_policy());
    assert_eq!(
        resume.harness_configuration_posture(),
        profile.request().harness_configuration_posture()
    );
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let handle = block_on(
        profile
            .resume_session(
                RequestId::new("bound-resume").unwrap(),
                binding,
                support::host_services(process),
            )
            .expect("bound resume prepares"),
    )
    .expect("bound resume opens");
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.methods().contains(&"thread/resume".to_owned()));
    assert!(state.waited());

    let failure = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("deadline").unwrap(),
            model(),
            working_resource(),
            Some(Deadline::at(MonotonicInstant::from_ticks(200))),
            SessionOptions::default(),
        ))
        .expect_err("unsupported session deadline fails during preparation");
    assert_eq!(failure.stage(), PreparationStage::Preflight);
}

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
    assert!(matches!(
        profile
            .evidence()
            .operation()
            .interface_compatibility()
            .next()
            .expect("Codex version evidence is present")
            .assessment(),
        swallowtail_core::InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}
