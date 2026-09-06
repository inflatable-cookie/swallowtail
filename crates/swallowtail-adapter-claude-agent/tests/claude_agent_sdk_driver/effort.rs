//! Provider-free opening-effort selection and confirmation proofs.

use crate::host_id;
use crate::sdk_support::{
    SdkFixtureHost, SdkScenario, cleanup_request, prepared_session_for, turn_request,
};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkEffort, ClaudeAgentSdkEffortOutcome, ClaudeAgentSdkSessionProfile,
};
use swallowtail_runtime::InteractiveSessionHandle;

fn finish_turn(
    session: &mut swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle,
    services: swallowtail_runtime::HostServices,
) {
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("first turn starts");
    let _ = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    let _ = block_on(turn.close());
}

#[test]
fn the_profile_admits_exactly_the_five_sdk_effort_levels() {
    for (value, effort) in [
        ("low", ClaudeAgentSdkEffort::Low),
        ("medium", ClaudeAgentSdkEffort::Medium),
        ("high", ClaudeAgentSdkEffort::High),
        ("xhigh", ClaudeAgentSdkEffort::XHigh),
        ("max", ClaudeAgentSdkEffort::Max),
    ] {
        assert_eq!(
            ClaudeAgentSdkEffort::parse(value).expect("admitted effort"),
            effort
        );
        assert_eq!(effort.as_str(), value);
    }
    assert!(ClaudeAgentSdkEffort::parse("ultracode").is_err());
}

#[test]
fn effort_is_confirmed_from_first_turn_init_when_reported() {
    let host = host_id("claude-agent-sdk.fixture.effort-confirmed");
    let fixture = SdkFixtureHost::new(SdkScenario::EffortConfirmed);
    let profile = ClaudeAgentSdkSessionProfile::read_only().with_effort(ClaudeAgentSdkEffort::High);
    let prepared = prepared_session_for(host.clone(), profile);
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone())).expect("opens");
    assert_eq!(
        session.effort(),
        ClaudeAgentSdkEffortOutcome::RequestedOnly(ClaudeAgentSdkEffort::High)
    );
    finish_turn(&mut session, services);
    assert_eq!(
        session.effort(),
        ClaudeAgentSdkEffortOutcome::Confirmed(ClaudeAgentSdkEffort::High)
    );
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn effort_without_init_evidence_stays_requested_only() {
    let host = host_id("claude-agent-sdk.fixture.effort-unconfirmed");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let profile = ClaudeAgentSdkSessionProfile::read_only().with_effort(ClaudeAgentSdkEffort::Max);
    let prepared = prepared_session_for(host.clone(), profile);
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone())).expect("opens");
    assert_eq!(
        session.effort(),
        ClaudeAgentSdkEffortOutcome::RequestedOnly(ClaudeAgentSdkEffort::Max)
    );
    finish_turn(&mut session, services);
    assert_eq!(
        session.effort(),
        ClaudeAgentSdkEffortOutcome::RequestedOnly(ClaudeAgentSdkEffort::Max)
    );
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn default_profile_omits_effort_from_the_open_wire() {
    let host = host_id("claude-agent-sdk.fixture.effort-default");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = crate::sdk_support::prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("opens");
    let open = fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("open command exists");
    assert!(open["params"].get("effort").is_none());
    assert_eq!(session.effort(), ClaudeAgentSdkEffortOutcome::NotRequested);
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}
