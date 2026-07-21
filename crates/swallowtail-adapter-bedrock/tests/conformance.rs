use swallowtail_testkit::{ConformanceAssertion, run_hosted_direct_api_profile};

#[test]
fn provider_neutral_hosted_profile_covers_bedrock_runtime_boundaries() {
    let report = run_hosted_direct_api_profile();
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::HostedApiNeedsNoProcess,
        ConformanceAssertion::HostedEndpointCredentialBinding,
        ConformanceAssertion::DirectRunNoResource,
        ConformanceAssertion::DirectRunOutputBound,
        ConformanceAssertion::ProviderEvidenceSeparated,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}
