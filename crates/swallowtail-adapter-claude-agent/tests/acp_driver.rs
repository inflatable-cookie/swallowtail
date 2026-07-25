#[path = "acp_driver/conformance.rs"]
mod conformance;
#[path = "acp_driver/deadline.rs"]
mod deadline;
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::selection::{open_request, selection};
use support::{FixtureHost, Scenario};
use swallowtail_adapter_claude_agent::ClaudeAgentAcpDriver;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    CleanupOutcome, EnvironmentRef, InteractiveSessionDriver, OperationContent, RuntimeEventKind,
    RuntimeTurnId, TerminalStatus, TurnRequest,
};

#[test]
fn qualified_milestones_and_unverified_newer_use_one_exact_read_only_session_shape() {
    for (host_name, host_suffix) in [
        ("fixture.host.local", "local"),
        ("fixture.host.remote-authoritative", "remote-authoritative"),
    ] {
        for version in ["0.53.0", "0.54.1", "0.60.0", "0.61.0", "0.62.0"] {
            let host_id = ExecutionHostId::new(host_name).expect("valid host");
            let selected = selection(host_id.clone(), version);
            let host = FixtureHost::new(Scenario::Success, version);
            let services = host.services(host_id);
            let driver = driver(selected.credential);
            let mut session = block_on(driver.open_session(
                selected.plan,
                open_request(
                    format!("open-{host_suffix}-{version}"),
                    selected.resource.clone(),
                ),
                services.clone(),
            ))
            .expect("session opens");
            assert_eq!(
                session
                    .provider_session_ref()
                    .expect("provider session exists")
                    .as_provider_value(),
                "claude-agent-session-fixture"
            );
            let mut turn = start(
                &mut *session,
                services,
                &format!("turn-{host_suffix}-{version}"),
            );
            let outcome = block_on(
                turn.take_terminal_outcome()
                    .expect("terminal outcome exists"),
            );
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
            assert_eq!(
                outcome.output().expect("output exists").as_str(),
                "fixture response."
            );
            let mut events = turn.take_events().expect("events exist");
            let events = block_on(async move {
                let mut values = Vec::new();
                while let Some(event) = events.next().await {
                    values.push(event.expect("event is valid"));
                }
                values
            });
            assert!(
                events
                    .iter()
                    .any(|event| { matches!(event.kind(), RuntimeEventKind::ReasoningProgress) })
            );
            assert!(
                events
                    .iter()
                    .any(|event| { matches!(event.kind(), RuntimeEventKind::OutputDelta) })
            );
            assert!(!format!("{events:?}").contains("private fixture prompt"));
            assert!(!format!("{outcome:?}").contains("fixture response"));
            assert_eq!(host.reads(), 1);
            assert_eq!(host.credential_acquires(), 1);
            let process = host.observed_process();
            assert_eq!(process.executable, "claude-agent.fixture.executable");
            assert!(process.arguments.is_empty());
            assert_eq!(process.environment_count, 1);
            assert_eq!(process.working_resource, Some(selected.resource));
            assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
            assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
            assert_eq!(host.resource_releases(), 1);
            assert_eq!(host.credential_releases(), 1);
        }
    }
}

#[test]
fn permission_is_rejected_then_cancelled_and_never_becomes_a_consumer_callback() {
    let (host, mut session, services) = open(Scenario::Permission, "0.61.0", "permission");
    let mut turn = start(&mut *session, services, "permission-turn");
    assert!(turn.take_callbacks().is_none());
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderRequestObserved(_)
    ));
    let writes = host.writes();
    let rejection = writes
        .iter()
        .position(|message| {
            message.get("id").and_then(serde_json::Value::as_u64) == Some(900)
                && message["result"]["outcome"]["optionId"] == "reject-once"
        })
        .expect("permission rejection was sent");
    let cancellation = writes
        .iter()
        .position(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/cancel")
        })
        .expect("turn cancellation was sent");
    assert!(rejection < cancellation);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn cancellation_disconnect_drift_and_access_mismatch_fail_without_leaks() {
    let (_host, mut session, services) = open(Scenario::Cancellation, "0.61.0", "cancel");
    let mut turn = start(&mut *session, services, "cancel-turn");
    block_on(turn.cancellation().request()).expect("cancellation is sent");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let (_host, mut session, services) = open(Scenario::Disconnect, "0.61.0", "disconnect");
    let mut turn = start(&mut *session, services, "disconnect-turn");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let host_id = ExecutionHostId::new("fixture.host.drift").expect("valid host");
    let selected = selection(host_id.clone(), "0.61.0");
    let host = FixtureHost::new(Scenario::ModelDrift, "0.61.0");
    let services = host.services(host_id);
    let error = match block_on(driver(selected.credential).open_session(
        selected.plan,
        open_request("drift-open", selected.resource),
        services,
    )) {
        Ok(_) => panic!("model drift must fail"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude_agent.acp.model_mismatch"
    );
    assert_eq!(host.resource_releases(), 1);
    assert_eq!(host.credential_releases(), 1);

    let host_id = ExecutionHostId::new("fixture.host.access").expect("valid host");
    let selected = selection(host_id.clone(), "0.61.0");
    let host = FixtureHost::new(Scenario::Success, "0.61.0");
    let wrong = swallowtail_core::CredentialRef::new("wrong.credential").expect("valid ref");
    let error = match block_on(driver(wrong).open_session(
        selected.plan,
        open_request("access-open", selected.resource),
        host.services(host_id),
    )) {
        Ok(_) => panic!("credential substitution must fail"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude_agent.acp.access_profile_rejected"
    );
    assert_eq!(host.credential_acquires(), 0);
}

fn open(
    scenario: Scenario,
    version: &str,
    suffix: &str,
) -> (
    FixtureHost,
    Box<dyn swallowtail_runtime::InteractiveSessionHandle>,
    swallowtail_runtime::HostServices,
) {
    let host_id = ExecutionHostId::new(format!("fixture.host.{suffix}")).expect("valid host");
    let selected = selection(host_id.clone(), version);
    let host = FixtureHost::new(scenario, version);
    let services = host.services(host_id);
    let session = block_on(driver(selected.credential).open_session(
        selected.plan,
        open_request(format!("open-{suffix}"), selected.resource),
        services.clone(),
    ))
    .expect("session opens");
    (host, session, services)
}

fn start(
    session: &mut dyn swallowtail_runtime::InteractiveSessionHandle,
    services: swallowtail_runtime::HostServices,
    turn_id: &str,
) -> Box<dyn swallowtail_runtime::TurnHandle> {
    block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new(turn_id).expect("valid turn"),
            OperationContent::new("private fixture prompt").expect("valid prompt"),
        ),
        services,
    ))
    .expect("turn starts")
}

fn driver(credential: swallowtail_core::CredentialRef) -> ClaudeAgentAcpDriver {
    ClaudeAgentAcpDriver::new(
        EnvironmentRef::new("claude-agent.fixture.environment").expect("valid environment"),
        credential,
    )
}
