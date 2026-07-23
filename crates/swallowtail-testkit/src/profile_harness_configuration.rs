use crate::{ConformanceAssertion, ConformanceReport, ProfilePreflightFixture, SyntheticProfile};
use swallowtail_core::HarnessConfigurationPosture;

pub(crate) fn run() -> Vec<ConformanceReport> {
    [
        (
            SyntheticProfile::OneShotStructuredCli,
            HarnessConfigurationPosture::Ambient,
        ),
        (
            SyntheticProfile::LongLivedRpcHarness,
            HarnessConfigurationPosture::ProviderSuppressed,
        ),
    ]
    .into_iter()
    .map(|(profile, posture)| {
        let mut fixture = if posture == HarnessConfigurationPosture::ProviderSuppressed {
            ProfilePreflightFixture::harness_rpc_contract()
        } else {
            ProfilePreflightFixture::new(profile)
        };
        fixture.bind_harness_configuration(posture);
        let plan = fixture
            .preflight()
            .expect("exact harness configuration posture passes preflight");
        assert_eq!(plan.harness_configuration_posture(), Some(posture));
        if posture == HarnessConfigurationPosture::ProviderSuppressed {
            let binding = plan
                .interface_versions()
                .next()
                .expect("provider-suppressed posture binds an exact interface version");
            assert!(
                plan.classify_interface_version(binding).is_some(),
                "provider-suppressed interface version is qualified"
            );
        }

        let mut report = ConformanceReport::new(profile);
        report.record(ConformanceAssertion::HarnessConfigurationExact);
        report
    })
    .collect()
}
