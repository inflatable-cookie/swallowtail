use swallowtail_testkit::{
    ConformanceAssertion, SyntheticProfile, run_hosted_direct_api_profile,
    run_realtime_media_direct_session_profile,
};

#[test]
fn provider_neutral_hosted_profile_covers_openai_background_boundaries() {
    let report = run_hosted_direct_api_profile();
    for assertion in [
        ConformanceAssertion::HostedApiNeedsNoProcess,
        ConformanceAssertion::HostedEndpointCredentialBinding,
        ConformanceAssertion::DirectRunNoResource,
        ConformanceAssertion::DirectRunOutputBound,
        ConformanceAssertion::ProviderEvidenceSeparated,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn provider_neutral_realtime_profile_covers_openai_realtime_boundaries() {
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
