use crate::host_id;
use crate::sdk_support::{SdkFixtureHost, SdkScenario, prepared_session, turn_request};
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_runtime::{CallbackPayload, CallbackResponse, CallbackResult, TerminalStatus};

fn terminal_status(scenario: SdkScenario) -> TerminalStatus {
    let host = host_id("claude-agent-sdk.fixture.framing");
    let fixture = SdkFixtureHost::new(scenario);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    let _ = block_on(turn.close());
    let _ = block_on(session.close());
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
    let _ = block_on(session.close());
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
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    fixture.emit(serde_json::json!({"type": "event", "event": "output_delta", "delta": "x"}));
    let outcome = block_on(session.close());
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
    let _ = block_on(session.close());
}
