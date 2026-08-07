#[test]
fn both_qualified_behavior_segments_execute() {
    let claim = swallowtail_adapter_grok::grok_build_acp_claim();
    for candidate in ["0.2.114", "0.2.117"] {
        let version = swallowtail_core::InterfaceVersion::new(candidate).expect("version");
        assert!(claim.supports(&version));
        let host = FixtureHost::with_version(Scenario::Success, candidate);
        let host_id = ExecutionHostId::new(format!("fixture.host.grok.{candidate}"))
            .expect("host");
        let mut run = start_run(host_id, &host, candidate, None);
        let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn unverified_newer_executes_without_becoming_guaranteed_support() {
    let claim = swallowtail_adapter_grok::grok_build_acp_claim();
    let version = swallowtail_core::InterfaceVersion::new("0.2.118").expect("version");
    assert!(!claim.supports(&version));
    assert!(claim.permits(&version));
    assert!(matches!(
        claim.assess(&version),
        swallowtail_core::InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    let host = FixtureHost::with_version(Scenario::Success, "0.2.118");
    let host_id = ExecutionHostId::new("fixture.host.grok.unverified").expect("host");
    let mut run = start_run(host_id, &host, "0.2.118", None);
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
}

#[test]
fn excluded_version_is_rejected_before_an_attachment_can_be_planned() {
    let claim = swallowtail_adapter_grok::grok_build_acp_claim();
    let version = swallowtail_core::InterfaceVersion::new("0.2.113").expect("version");
    assert!(!claim.permits(&version));
    assert!(matches!(
        claim.assess(&version),
        swallowtail_core::InterfaceCompatibilityAssessment::Incompatible
    ));
}

