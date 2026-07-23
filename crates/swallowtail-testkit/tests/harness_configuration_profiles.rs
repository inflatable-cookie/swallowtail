use swallowtail_testkit::{
    ConformanceAssertion, SyntheticProfile, run_harness_configuration_boundary_assertions,
};

#[test]
fn assertion_pack_covers_both_common_harness_shapes() {
    let reports = run_harness_configuration_boundary_assertions();
    assert_eq!(reports.len(), 2);
    for profile in [
        SyntheticProfile::OneShotStructuredCli,
        SyntheticProfile::LongLivedRpcHarness,
    ] {
        let report = reports
            .iter()
            .find(|report| report.profile() == profile)
            .expect("configuration report exists for harness profile");
        assert!(report.covers(ConformanceAssertion::HarnessConfigurationExact));
    }
}
