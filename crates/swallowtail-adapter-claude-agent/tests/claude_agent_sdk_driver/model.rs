//! Provider-free model catalogue and mid-session model-change proofs.

use crate::host_id;
use crate::sdk_support::{
    SdkFixtureHost, SdkScenario, cleanup_request, prepared_session, turn_request,
};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle;
use swallowtail_runtime::{InteractiveSessionHandle, MonotonicInstant};

fn finish_first_turn(
    session: &mut ClaudeAgentSdkSessionHandle,
    services: swallowtail_runtime::HostServices,
) {
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("first turn starts and confirms the model");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(
        terminal.status(),
        &swallowtail_runtime::TerminalStatus::Completed
    );
    let _ = block_on(turn.close());
}

fn model_deadline() -> swallowtail_runtime::Deadline {
    swallowtail_runtime::Deadline::at(MonotonicInstant::from_ticks(10_000))
}

#[test]
fn supported_models_are_open_time_evidence_and_a_change_requires_confirmation() {
    let host = host_id("claude-agent-sdk.fixture.model-confirmed");
    let fixture = SdkFixtureHost::new(SdkScenario::ModelChangeConfirmed);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone())).expect("opens");
    assert_eq!(
        session.supported_models(),
        ["claude-sonnet-5", "claude-opus-5"]
    );
    finish_first_turn(&mut session, services.clone());
    assert_eq!(session.effective_model(), "claude-sonnet-5");

    let changed = block_on(session.set_model("claude-opus-5", services, model_deadline()))
        .expect("fake SDK reports the effective model");
    assert_eq!(changed, "claude-opus-5");
    assert_eq!(session.effective_model(), "claude-opus-5");
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn an_unconfirmed_model_change_keeps_the_previous_effective_model() {
    let host = host_id("claude-agent-sdk.fixture.model-unconfirmed");
    let fixture = SdkFixtureHost::new(SdkScenario::ModelChangeUnconfirmed);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone())).expect("opens");
    finish_first_turn(&mut session, services.clone());
    let error = block_on(session.set_model("claude-opus-5", services, model_deadline()))
        .expect_err("acceptance without a model value is not confirmation");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.model_change_unconfirmed"
    );
    assert_eq!(session.effective_model(), "claude-sonnet-5");
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn a_supported_model_rejection_is_typed_unconfirmed() {
    let host = host_id("claude-agent-sdk.fixture.model-rejected");
    let fixture = SdkFixtureHost::new(SdkScenario::ModelChangeRejected);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone())).expect("opens");
    finish_first_turn(&mut session, services.clone());
    let error = block_on(session.set_model("claude-opus-5", services, model_deadline()))
        .expect_err("a provider rejection supplies no model confirmation");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.model_change_unconfirmed"
    );
    assert_eq!(session.effective_model(), "claude-sonnet-5");
    assert!(
        fixture
            .inputs()
            .into_iter()
            .any(|input| input["command"] == "set_model"),
        "the supported model reaches Query.setModel before its rejection"
    );
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn an_unsupported_model_is_rejected_before_the_sidecar_call() {
    let host = host_id("claude-agent-sdk.fixture.model-rejected");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone())).expect("opens");
    finish_first_turn(&mut session, services.clone());
    let error = block_on(session.set_model("claude-opus-5", services, model_deadline()))
        .expect_err("the model is absent from supportedModels");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.model_unsupported"
    );
    assert_eq!(session.effective_model(), "claude-sonnet-5");
    assert!(
        fixture
            .inputs()
            .into_iter()
            .all(|input| input["command"] != "set_model"),
        "unsupported model must not reach Query.setModel"
    );
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}
