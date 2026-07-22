use swallowtail_testkit::{
    ConformanceAssertion, SyntheticProfile, run_all_synthetic_profiles,
    run_realtime_media_direct_session_profile, run_realtime_rollover_boundary_assertions,
};

#[test]
fn unchanged_realtime_profile_covers_gemini_live_boundaries() {
    let report = run_realtime_media_direct_session_profile();
    assert_eq!(
        report.profile(),
        SyntheticProfile::RealtimeMediaDirectSession
    );
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::HostedEndpointCredentialBinding,
        ConformanceAssertion::DirectSessionNoResource,
        ConformanceAssertion::ConnectionScopedLeaseLifecycle,
        ConformanceAssertion::ProviderEvidenceSeparated,
        ConformanceAssertion::RealtimeMediaBoundary,
        ConformanceAssertion::RealtimeMediaOrdering,
        ConformanceAssertion::RealtimeMediaInterruptionEndsSession,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn separate_rollover_assertions_remain_bounded_and_provider_neutral() {
    let report = run_realtime_rollover_boundary_assertions();
    assert_eq!(
        report.profile(),
        SyntheticProfile::RealtimeMediaDirectSession
    );
    for assertion in [
        ConformanceAssertion::PlannedConnectionRollover,
        ConformanceAssertion::RolloverNoReplay,
        ConformanceAssertion::RolloverCleanupOrdered,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn rollover_does_not_create_a_twelfth_common_profile() {
    let reports = run_all_synthetic_profiles();
    assert_eq!(reports.len(), 11);
    assert_eq!(
        reports
            .iter()
            .filter(|report| report.profile() == SyntheticProfile::RealtimeMediaDirectSession)
            .count(),
        1
    );
    for report in reports {
        assert!(report.covers(ConformanceAssertion::PreflightBeforeSideEffects));
        assert!(report.covers(ConformanceAssertion::NoImplicitFallback));
        assert!(report.covers(ConformanceAssertion::Redaction));
    }
}
