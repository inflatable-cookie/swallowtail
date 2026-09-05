use crate::host_id;
use crate::sdk_support::{
    CleanupEvent, SdkFixtureHost, SdkScenario, cleanup_request, prepared_session,
    prepared_session_with, turn_request,
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
    let services_for_cleanup = services.clone();
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
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
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

#[test]
fn an_effective_model_other_than_the_selected_one_fails_closed() {
    // The plan binds the model; running Claude's ambient default instead is a
    // silent substitution, so open confirms the effective model from the
    // runtime's own init evidence.
    let (code, _) = open_failure(SdkScenario::ModelMismatch);
    assert_eq!(code, "swallowtail.claude-agent.sdk.open_mismatch");
}

#[test]
fn the_selected_model_crosses_the_wire_on_open() {
    let host = host_id("claude-agent-sdk.fixture.model");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    let open = fixture
        .inputs()
        .into_iter()
        .find(|value| value["command"] == "open")
        .expect("open command is sent");
    assert_eq!(open["params"]["model"], "claude-sonnet-5");
    // Open carries exactly the cwd, model, admitted tool set, and permission
    // mode. The default profile is the unchanged read-only one.
    assert_eq!(open["params"].as_object().expect("params").len(), 4);
    assert_eq!(
        open["params"]["tools"],
        serde_json::json!(["Read", "Glob", "Grep"])
    );
    assert_eq!(open["params"]["permissionMode"], "default");
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
}

#[test]
fn an_open_that_never_reaches_readiness_expires_on_the_host_deadline() {
    // The sidecar holds its open response; only the host clock ends the wait.
    let host = host_id("claude-agent-sdk.fixture.open-deadline");
    let fixture = SdkFixtureHost::new(SdkScenario::OpenHold).with_immediate_time();
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("an unbounded open must fail on the host deadline");
    };
    let code = error.diagnostic().code().to_owned();
    fixture.wait_for_cleanup(CleanupEvent::ProcessWait);
    fixture.release_process_hold();
    fixture.wait_for_cleanup(CleanupEvent::CredentialRelease);
    fixture.reaper().shutdown();
    assert_eq!(
        code,
        "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed"
    );
}

#[cfg(windows)]
#[test]
fn windows_is_an_unsupported_platform_for_this_route() {
    use swallowtail_adapter_claude_agent::sdk::claude_agent_sdk_addable_route_descriptor;
    let host = host_id("claude-agent-sdk.fixture.windows");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    assert_eq!(
        claude_agent_sdk_addable_route_descriptor(&services).availability(),
        swallowtail_core::AddableRouteAvailability::Unsupported
    );
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("an unprovable descendant-tree platform must refuse to open");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.unsupported_input"
    );
}

#[cfg(not(windows))]
#[test]
fn a_platform_with_retained_tree_ownership_admits_the_route() {
    use swallowtail_adapter_claude_agent::sdk::claude_agent_sdk_addable_route_descriptor;
    let host = host_id("claude-agent-sdk.fixture.platform");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let services = fixture.services(host);
    assert_eq!(
        claude_agent_sdk_addable_route_descriptor(&services).availability(),
        swallowtail_core::AddableRouteAvailability::Available
    );
}
