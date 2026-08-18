use swallowtail_adapter_opencode::{
    OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION, opencode_http_claim,
};
use swallowtail_core::InterfaceVersion;
use swallowtail_testkit::{
    ClosedSemanticWindowCase, ConformanceAssertion, SyntheticProfile,
    assert_closed_semantic_compatibility_window, assert_provider_session_management_contract,
    assert_unverified_newer_execution, run_attached_network_harness_profile,
    run_structured_harness_native_boundary_assertions,
};

#[test]
fn provider_neutral_attached_network_profile_covers_opencode_boundaries() {
    let report = run_attached_network_harness_profile();
    assert_eq!(report.profile(), SyntheticProfile::AttachedNetworkHarness);
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::StalePlanRejected,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::ExternalOwnershipPreserved,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::DelegatedAuthentication,
        ConformanceAssertion::AttachedNetworkHarnessLifecycle,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn provider_neutral_closed_window_assertion_covers_opencode_range() {
    let case = ClosedSemanticWindowCase::new(
        version(OPENCODE_BASELINE_VERSION),
        version(OPENCODE_LATEST_QUALIFIED_VERSION),
    )
    .with_accepted([
        version("1.14.49"),
        version("1.15.7"),
        version("1.17.10"),
        version("1.18.0"),
        version("1.18.5"),
        version("1.18.10"),
        version("1.18.18"),
    ])
    .with_rejected([
        version("1.14.47"),
        version("1.14.52"),
        version("1.15.8"),
        version("1.16.1"),
        version("1.17.21"),
        version("1.18.11-rc.1"),
        version("1.18.19"),
    ]);
    assert_closed_semantic_compatibility_window(&opencode_http_claim(), &case);
    assert_unverified_newer_execution(&opencode_http_claim(), &version("1.18.19"));
}

#[test]
fn provider_neutral_management_contract_covers_opencode_delete_boundaries() {
    assert_provider_session_management_contract();
}

#[test]
fn provider_neutral_projection_pack_covers_opencode_temporary_session_deletion() {
    let report = run_structured_harness_native_boundary_assertions();
    for assertion in [
        ConformanceAssertion::AmbientHarnessAuthority,
        ConformanceAssertion::OwnedRemoteDeletionTruth,
        ConformanceAssertion::NativeBudgetIndependent,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
