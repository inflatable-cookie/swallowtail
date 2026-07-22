use swallowtail_testkit::{ConformanceAssertion, SyntheticProfile, run_all_synthetic_profiles};

#[test]
fn realtime_media_profile_adds_contract_026_without_replacing_existing_profiles() {
    let reports = run_all_synthetic_profiles();
    let existing = [
        SyntheticProfile::OneShotStructuredCli,
        SyntheticProfile::LongLivedRpcHarness,
        SyntheticProfile::LongLivedAcpHarness,
        SyntheticProfile::PersistentAcpHarness,
        SyntheticProfile::AttachedNetworkHarness,
        SyntheticProfile::HostedDirectApi,
        SyntheticProfile::ProviderManagedRemoteHarness,
        SyntheticProfile::ConnectionScopedDirectSession,
        SyntheticProfile::AttachedSelfHosted,
        SyntheticProfile::OwnedSelfHosted,
    ];
    assert!(
        existing
            .into_iter()
            .all(|profile| reports.iter().any(|report| report.profile() == profile))
    );

    let media = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::RealtimeMediaDirectSession)
        .expect("realtime-media profile exists");
    for assertion in [
        ConformanceAssertion::RealtimeMediaBoundary,
        ConformanceAssertion::RealtimeMediaOrdering,
        ConformanceAssertion::RealtimeMediaInterruptionEndsSession,
        ConformanceAssertion::ConnectionScopedLeaseLifecycle,
    ] {
        assert!(media.covers(assertion));
    }
}
