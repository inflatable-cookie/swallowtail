use swallowtail_testkit::{
    ConformanceAssertion, SyntheticProfile, run_harness_rpc_contract_assertions,
    run_long_lived_rpc_profile,
};

#[test]
fn shared_rpc_profile_and_contract_pack_remain_unchanged() {
    let profile = run_long_lived_rpc_profile();
    assert_eq!(profile.profile(), SyntheticProfile::LongLivedRpcHarness);
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::CallbackExchange,
    ] {
        assert!(profile.covers(assertion), "missing {assertion:?}");
    }

    let contract = run_harness_rpc_contract_assertions();
    for assertion in [
        ConformanceAssertion::InterfaceVersionQualified,
        ConformanceAssertion::HarnessPolicyExact,
        ConformanceAssertion::HarnessScheduling,
        ConformanceAssertion::CommandAcknowledgement,
        ConformanceAssertion::HarnessUiRelay,
    ] {
        assert!(contract.covers(assertion), "missing {assertion:?}");
    }
}
