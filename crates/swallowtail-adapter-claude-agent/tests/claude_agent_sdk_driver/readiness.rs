use crate::host_id;
use crate::sdk_support::{
    CleanupEvent, SdkFixtureHost, SdkScenario, prepared_session, prepared_session_with,
    turn_request,
};
use futures_executor::block_on;
use swallowtail_runtime::SessionOptions;

fn open_failure(scenario: SdkScenario) -> (String, Vec<CleanupEvent>) {
    let host = host_id("claude-agent-sdk.fixture.readiness");
    let fixture = SdkFixtureHost::new(scenario);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("open must fail closed");
    };
    (
        error.diagnostic().code().to_owned(),
        fixture.cleanup_events(),
    )
}

#[test]
fn an_api_key_access_profile_never_becomes_a_subscription_session() {
    let (code, cleanup) = open_failure(SdkScenario::AccountApiKeySource);
    assert_eq!(code, "swallowtail.claude-agent.sdk.account_not_ready");
    // Failing open still terminates the tree and releases both leases.
    assert!(cleanup.contains(&CleanupEvent::ProcessForceStop));
    assert!(cleanup.contains(&CleanupEvent::ResourceRelease));
    assert!(cleanup.contains(&CleanupEvent::CredentialRelease));
}

#[test]
fn a_delegated_cloud_provider_is_not_first_party_readiness() {
    let (code, _) = open_failure(SdkScenario::AccountNotFirstParty);
    assert_eq!(code, "swallowtail.claude-agent.sdk.account_not_ready");
}

#[test]
fn account_identity_fields_are_refused_rather_than_recorded() {
    let (code, _) = open_failure(SdkScenario::AccountIdentityLeak);
    assert_eq!(code, "swallowtail.claude-agent.sdk.account_not_ready");
}

#[test]
fn off_point_identity_resource_and_tool_sets_fail_closed() {
    for scenario in [
        SdkScenario::IdentityMismatch,
        SdkScenario::CwdMismatch,
        SdkScenario::ToolsWidened,
    ] {
        let (code, _) = open_failure(scenario);
        assert_eq!(
            code, "swallowtail.claude-agent.sdk.open_mismatch",
            "{scenario:?} must fail closed"
        );
    }
}

#[test]
fn an_interrupt_receipt_requires_a_runtime_advertised_capability() {
    let host = host_id("claude-agent-sdk.fixture.receipt");
    let fixture = SdkFixtureHost::new(SdkScenario::UnadvertisedInterruptReceipt);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    let error = block_on(turn.cancellation().request())
        .expect_err("an unadvertised receipt must fail closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.interrupt_receipt_unadvertised"
    );
    let _ = block_on(session.close());
}

#[test]
fn preparation_admits_no_session_options_in_this_layer() {
    let host = host_id("claude-agent-sdk.fixture.options");
    let options = SessionOptions::default().with_reasoning_mode(
        swallowtail_core::ReasoningMode::new("high").expect("valid reasoning mode"),
    );
    let Err(failure) = prepared_session_with(host, options) else {
        panic!("reasoning selection is a later layer");
    };
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.claude-agent.sdk.preparation.unsupported_options"
    );
}
