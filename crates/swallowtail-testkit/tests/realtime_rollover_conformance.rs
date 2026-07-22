use swallowtail_testkit::{
    ConformanceAssertion, SyntheticProfile, run_all_synthetic_profiles,
    run_realtime_rollover_boundary_assertions,
};

#[test]
fn planned_rollover_is_an_assertion_pack_over_the_existing_realtime_profile() {
    assert_eq!(run_all_synthetic_profiles().len(), 11);
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
        assert!(report.covers(assertion));
    }
}
