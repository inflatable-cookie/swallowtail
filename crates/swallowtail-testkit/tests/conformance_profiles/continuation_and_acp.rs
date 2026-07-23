use super::*;

#[test]
fn locally_continued_profile_proves_contract_030_boundaries() {
    let reports = run_all_synthetic_profiles();
    let direct = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::LocallyContinuedDirectSession)
        .expect("locally continued profile report exists");

    for assertion in [
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::DirectSessionNoResource,
        ConformanceAssertion::ExplicitAttemptAuthorization,
        ConformanceAssertion::ConsumerToolExchange,
        ConformanceAssertion::PrivateContinuationBounded,
        ConformanceAssertion::ProviderCachePosture,
        ConformanceAssertion::RequestScopedLeaseLifecycle,
        ConformanceAssertion::NoImplicitSessionRecovery,
    ] {
        assert!(direct.covers(assertion));
    }
}

#[test]
fn long_lived_acp_profile_proves_process_callback_and_topology_boundaries() {
    let reports = run_all_synthetic_profiles();
    let acp = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::LongLivedAcpHarness)
        .expect("ACP profile report exists");

    for assertion in [
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::ProcessLifecycle,
        ConformanceAssertion::WorkingResourceCallback,
        ConformanceAssertion::HostTopologyPreserved,
    ] {
        assert!(acp.covers(assertion));
    }
    assert!(!acp.covers(ConformanceAssertion::CallbackExchange));
}
