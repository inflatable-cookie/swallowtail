use swallowtail_core::PreflightDimension;
use swallowtail_testkit::{
    ConformanceAssertion, PreflightFixtureCase, SyntheticProfile,
    assert_preflight_rejection_without_side_effects, run_all_synthetic_profiles,
};

const COMMON_ASSERTIONS: [ConformanceAssertion; 14] = [
    ConformanceAssertion::PreflightBeforeSideEffects,
    ConformanceAssertion::BoundSelection,
    ConformanceAssertion::StalePlanRejected,
    ConformanceAssertion::OrderedEvents,
    ConformanceAssertion::SingleTerminalOutcome,
    ConformanceAssertion::SemanticOverflowFails,
    ConformanceAssertion::CancellationAndTimeoutDistinct,
    ConformanceAssertion::CleanupRemainsVisible,
    ConformanceAssertion::ExternalOwnershipPreserved,
    ConformanceAssertion::Redaction,
    ConformanceAssertion::ScopedInputs,
    ConformanceAssertion::SchemaTransportOnly,
    ConformanceAssertion::ExtensionPolicyExplicit,
    ConformanceAssertion::NoImplicitFallback,
];

#[test]
fn all_five_profiles_cover_every_common_contract_assertion() {
    let reports = run_all_synthetic_profiles();
    assert_eq!(reports.len(), 5);

    for report in &reports {
        for assertion in COMMON_ASSERTIONS {
            assert!(
                report.covers(assertion),
                "{:?} missed {assertion:?}",
                report.profile()
            );
        }
    }
}

#[test]
fn each_profile_proves_its_shape_specific_boundary() {
    let reports = run_all_synthetic_profiles();
    let expected = [
        (
            SyntheticProfile::OneShotStructuredCli,
            ConformanceAssertion::ProcessLifecycle,
        ),
        (
            SyntheticProfile::LongLivedRpcHarness,
            ConformanceAssertion::SessionLifecycle,
        ),
        (
            SyntheticProfile::HostedDirectApi,
            ConformanceAssertion::HostedApiNeedsNoProcess,
        ),
        (
            SyntheticProfile::AttachedSelfHosted,
            ConformanceAssertion::AttachedServiceNeverStopped,
        ),
        (
            SyntheticProfile::OwnedSelfHosted,
            ConformanceAssertion::OwnedServiceStops,
        ),
    ];

    for (profile, assertion) in expected {
        let report = reports
            .iter()
            .find(|report| report.profile() == profile)
            .expect("profile report exists");
        assert!(report.covers(assertion));
    }
}

#[test]
fn long_lived_rpc_profile_proves_callback_exchange() {
    let reports = run_all_synthetic_profiles();
    let rpc = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::LongLivedRpcHarness)
        .expect("RPC profile report exists");

    assert!(rpc.covers(ConformanceAssertion::CallbackExchange));
}

#[test]
fn deliberate_violations_name_the_exact_dimension_before_effects() {
    for (case, dimension) in [
        (
            PreflightFixtureCase::MissingModelRoute,
            PreflightDimension::ModelRoute,
        ),
        (
            PreflightFixtureCase::RejectedAccess,
            PreflightDimension::Access,
        ),
        (
            PreflightFixtureCase::RejectedOwnership,
            PreflightDimension::Ownership,
        ),
        (
            PreflightFixtureCase::WrongExecutionHost,
            PreflightDimension::Topology,
        ),
    ] {
        assert_preflight_rejection_without_side_effects(case, dimension);
    }
}
