use crate::host_id;
use crate::sdk_support::{
    SdkFixtureHost, SdkScenario, cleanup_request, prepared_session, prepared_session_for,
    turn_request,
};
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkPermissionMode, ClaudeAgentSdkSessionProfile, ClaudeAgentSdkTool,
};
use swallowtail_runtime::{CallbackPayload, CallbackResponse, CallbackResult, TerminalStatus};

fn terminal_status(scenario: SdkScenario) -> TerminalStatus {
    let host = host_id("claude-agent-sdk.fixture.framing");
    let fixture = SdkFixtureHost::new(scenario);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    let _ = block_on(turn.close());
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
    terminal.status().clone()
}

#[test]
fn unqualified_malformed_truncated_and_terminal_records_all_fail_closed() {
    for scenario in [
        SdkScenario::UnknownEvent,
        SdkScenario::Malformed,
        SdkScenario::Disconnect,
        SdkScenario::TerminalRecord,
        SdkScenario::ToolOrderingDrift,
    ] {
        let status = terminal_status(scenario);
        let TerminalStatus::RuntimeFailed(diagnostic) = &status else {
            panic!("{scenario:?} must fail the turn, got {status:?}");
        };
        assert!(
            !diagnostic.message().contains("/fixture/"),
            "{scenario:?} diagnostic must not carry a host path"
        );
    }
}

#[test]
fn tool_admission_crosses_the_wire_as_a_correlated_consumer_decision() {
    let host = host_id("claude-agent-sdk.fixture.admission");
    let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");

    let mut callbacks = turn.take_callbacks().expect("turn exposes tool admission");
    let mut requests = callbacks
        .take_requests()
        .expect("admission requests are available once");
    let request = block_on(requests.next())
        .expect("one admission request arrives")
        .expect("admission request is healthy");
    let swallowtail_runtime::CallbackRequestKind::Extension(extension) = request.kind() else {
        panic!("tool admission is a route-local extension callback");
    };
    assert_eq!(
        extension.namespace().as_str(),
        "claude-agent-sdk/can-use-tool"
    );
    let payload: serde_json::Value =
        serde_json::from_slice(extension.payload()).expect("payload is JSON");
    assert_eq!(payload, serde_json::json!({"toolName": "Read"}));
    assert_eq!(payload.as_object().expect("object").len(), 1);

    block_on(
        callbacks.responder().respond(CallbackResponse::new(
            request.callback_id().clone(),
            swallowtail_runtime::RuntimeTurnId::new("turn-1").expect("valid turn"),
            CallbackResult::Success(
                CallbackPayload::new(br#"{"decision":"allow"}"#.to_vec(), 4096)
                    .expect("payload fits the bound"),
            ),
        )),
    )
    .expect("admission response reaches the sidecar");

    let decision = fixture
        .inputs()
        .into_iter()
        .find(|value| value["type"] == "callback_response")
        .expect("a callback response crosses the wire");
    assert_eq!(decision["id"], "cb-1");
    assert_eq!(decision["decision"], "allow");
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
}

#[test]
fn bash_admission_crosses_the_wire_with_its_bounded_command_view() {
    let host = host_id("claude-agent-sdk.fixture.bash-admission");
    let fixture = SdkFixtureHost::new(SdkScenario::BashAdmission);
    let profile = ClaudeAgentSdkSessionProfile::new(
        [ClaudeAgentSdkTool::Bash],
        ClaudeAgentSdkPermissionMode::Default,
    )
    .expect("explicit Bash profile is admissible");
    let prepared = prepared_session_for(host.clone(), profile);
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "inspect it"), services))
        .expect("SDK sidecar turn starts");

    let mut callbacks = turn.take_callbacks().expect("turn exposes tool admission");
    let mut requests = callbacks
        .take_requests()
        .expect("admission requests are available once");
    let request = block_on(requests.next())
        .expect("one admission request arrives")
        .expect("admission request is healthy");
    let swallowtail_runtime::CallbackRequestKind::Extension(extension) = request.kind() else {
        panic!("tool admission is a route-local extension callback");
    };
    let payload: serde_json::Value =
        serde_json::from_slice(extension.payload()).expect("payload is JSON");
    assert_eq!(payload["toolName"], "Bash");
    assert_eq!(payload["command"], "git status --porcelain");
    assert_eq!(payload["commandByteLength"], 22);
    assert_eq!(payload["description"], "inspect the working tree");
    assert_eq!(payload["truncated"], false);
    assert_eq!(payload.as_object().expect("object").len(), 5);

    block_on(
        callbacks.responder().respond(CallbackResponse::new(
            request.callback_id().clone(),
            swallowtail_runtime::RuntimeTurnId::new("turn-1").expect("valid turn"),
            CallbackResult::Success(
                CallbackPayload::new(br#"{"decision":"allow"}"#.to_vec(), 4096)
                    .expect("payload fits the bound"),
            ),
        )),
    )
    .expect("admission response reaches the sidecar");

    let decision = fixture
        .inputs()
        .into_iter()
        .find(|value| value["type"] == "callback_response")
        .expect("a callback response crosses the wire");
    assert_eq!(decision["id"], "cb-1");
    assert_eq!(decision["decision"], "allow");
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup));
}

#[test]
fn admission_requests_beyond_the_bound_fail_rather_than_buffer() {
    let status = terminal_status(SdkScenario::ToolAdmissionOverflow);
    let TerminalStatus::RuntimeFailed(diagnostic) = &status else {
        panic!("overflowing admission must fail the turn, got {status:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.tool_admission_invalid"
    );
}

#[test]
fn events_outside_an_active_turn_fail_closed() {
    let host = host_id("claude-agent-sdk.fixture.unsolicited");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    fixture.emit(serde_json::json!({"type": "event", "event": "output_delta", "delta": "x"}));
    let outcome = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
    assert!(
        matches!(outcome, swallowtail_runtime::CleanupOutcome::Degraded(_)),
        "an unsolicited event breaks the wire, so close can only be escalated"
    );
    assert!(
        fixture
            .cleanup_events()
            .contains(&crate::sdk_support::CleanupEvent::ProcessForceStop)
    );
}

#[test]
fn the_launch_surface_carries_only_opaque_host_owned_references() {
    let host = host_id("claude-agent-sdk.fixture.launch");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    let environment = fixture.process_environment();
    assert_eq!(environment, ["claude-agent-sdk.fixture.environment"]);
    let inputs = fixture.inputs();
    let serialized = serde_json::to_string(&inputs).expect("inputs serialize");
    for forbidden in [
        "ANTHROPIC_API_KEY",
        "apiKeyHelper",
        "accessToken",
        "worker_jwt",
        "oauth",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "no outbound record may carry {forbidden}"
        );
    }
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
}

#[test]
fn an_admission_request_outside_the_read_only_set_never_reaches_the_consumer() {
    // The allow-list is enforced on both sides of the wire. Its arrival here is
    // a transport failure, not a decision to delegate to the consumer.
    let status = terminal_status(SdkScenario::UnadmittedToolAdmission);
    let TerminalStatus::RuntimeFailed(diagnostic) = &status else {
        panic!("an unadmitted tool must fail the turn, got {status:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.admission_tool_unadmitted"
    );
}

#[test]
fn closing_a_session_with_a_live_turn_resolves_it_instead_of_waiting_on_its_deadline() {
    // Regression: the turn's host-deadline task waits on completion or expiry,
    // so close must end the turn before joining that task. Joining first waits
    // for an event close itself prevented.
    let host = host_id("claude-agent-sdk.fixture.close-live-turn");
    let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    let terminal = turn
        .take_terminal_outcome()
        .expect("terminal outcome exists");

    let outcome = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
    assert!(
        matches!(outcome, swallowtail_runtime::CleanupOutcome::Degraded(_)),
        "closing over a live turn still reports the tree honestly"
    );
    assert_eq!(block_on(terminal).status(), &TerminalStatus::Cancelled);
}

#[test]
fn a_turn_that_never_ends_resolves_on_the_host_deadline() {
    let host = host_id("claude-agent-sdk.fixture.turn-deadline");
    let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    // The open bound already passed; only the turn's own deadline fires here.
    fixture.fire_deadlines();
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    // The terminal outcome never waits on the receipt, but the interrupt is
    // still requested afterwards.
    fixture.wait_for_command("interrupt");
    let _ = block_on(turn.close());
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
}

#[test]
fn a_query_that_is_never_answered_returns_on_the_turn_deadline() {
    // The public start-turn future is raced against the caller's turn deadline,
    // so a sidecar that stops answering cannot hold it open.
    let host = host_id("claude-agent-sdk.fixture.query-hold");
    let fixture = SdkFixtureHost::new(SdkScenario::QueryHold);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    fixture.fire_deadlines();
    let Err(error) = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
    else {
        panic!("an unanswered query must return on the turn deadline");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.turn_deadline_elapsed"
    );
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup));
}

#[test]
fn an_interrupt_receipt_that_never_arrives_still_returns_requested() {
    // Cancellation is a request, never a claim of provider truth: the wire
    // write is issued, and only the receipt is bounded.
    let host = host_id("claude-agent-sdk.fixture.interrupt-bound");
    let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    fixture.hold_responses();
    fixture.fire_deadlines();
    assert_eq!(
        block_on(turn.cancellation().request()).expect("cancellation is a bounded request"),
        swallowtail_runtime::CancellationAcknowledgement::Requested
    );
    assert!(
        fixture
            .inputs()
            .iter()
            .any(|value| value["command"] == "interrupt"),
        "the interrupt still crosses the wire"
    );
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup));
}
