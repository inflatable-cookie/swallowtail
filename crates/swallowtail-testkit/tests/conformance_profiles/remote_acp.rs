use swallowtail_testkit::{ConformanceAssertion, SyntheticProfile, run_all_synthetic_profiles};

#[test]
fn profile_adds_network_affinity_version_and_no_recovery_boundaries() {
    let reports = run_all_synthetic_profiles();
    let remote = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::RemoteAcpHarness)
        .expect("remote ACP profile report exists");

    for assertion in [
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::CallbackExchange,
        ConformanceAssertion::HostTopologyPreserved,
        ConformanceAssertion::RemoteAcpConnectionLifecycle,
        ConformanceAssertion::RemoteAcpAffinityScoped,
        ConformanceAssertion::RemoteAcpNoRecovery,
        ConformanceAssertion::RemoteAcpVersionAxesSeparate,
    ] {
        assert!(remote.covers(assertion), "missing {assertion:?}");
    }
}
