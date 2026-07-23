use swallowtail_testkit::{
    ConformanceAssertion, SyntheticProfile, run_attached_runtime_boundary_assertions,
};

#[test]
fn provider_neutral_attached_runtime_pack_covers_ollama_boundaries() {
    let report = run_attached_runtime_boundary_assertions();
    assert_eq!(report.profile(), SyntheticProfile::AttachedSelfHosted);
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
        ConformanceAssertion::AttachedServiceNeverStopped,
        ConformanceAssertion::AttachedRuntimeBinding,
        ConformanceAssertion::RuntimeManagedResidency,
        ConformanceAssertion::ClosedCompatibilityWindow,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}
