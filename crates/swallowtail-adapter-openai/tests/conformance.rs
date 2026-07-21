use swallowtail_testkit::{ConformanceAssertion, run_hosted_direct_api_profile};

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
