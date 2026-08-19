#[test]
fn qualified_behavior_segments_execute() {
    let claim = swallowtail_adapter_grok::grok_build_acp_claim();
    for candidate in ["0.2.114", "0.2.117", "1.0.4", "1.0.5"] {
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
    let version = swallowtail_core::InterfaceVersion::new("1.0.6").expect("version");
    assert!(!claim.supports(&version));
    assert!(claim.permits(&version));
    assert!(matches!(
        claim.assess(&version),
        swallowtail_core::InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    let host = FixtureHost::with_version(Scenario::Success, "1.0.6");
    let host_id = ExecutionHostId::new("fixture.host.grok.unverified").expect("host");
    let mut run = start_run(host_id, &host, "1.0.6", None);
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
}

#[test]
fn mid_gap_0_2_later_stables_are_incompatible_after_1_0_milestone() {
    let claim = swallowtail_adapter_grok::grok_build_acp_claim();
    for candidate in ["0.2.118", "0.2.121", "1.0.0", "1.0.3"] {
        let version = swallowtail_core::InterfaceVersion::new(candidate).expect("version");
        assert!(!claim.permits(&version));
        assert!(matches!(
            claim.assess(&version),
            swallowtail_core::InterfaceCompatibilityAssessment::Incompatible
        ));
    }
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
