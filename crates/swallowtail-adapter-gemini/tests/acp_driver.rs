mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{FixtureHost, Scenario, selection};
use swallowtail_adapter_gemini::GeminiAcpDriver;
use swallowtail_core::{ExecutionHostId, ProviderRequestHandling};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OpenSessionRequest, OperationContent, RequestId,
    RuntimeEventKind, RuntimeTurnId, TerminalStatus, TurnRequest,
};

#[test]
fn success_and_read_callbacks_preserve_local_and_remote_host_authority() {
    for host_id in ["fixture.host.local", "fixture.host.remote-authoritative"] {
        let host_id = ExecutionHostId::new(host_id).expect("valid host id");
        let selected = selection(host_id.clone());
        let host = FixtureHost::new(Scenario::Success);
        let services = host.services(host_id);
        let driver = GeminiAcpDriver::new(
            swallowtail_runtime::EnvironmentRef::new("gemini.fixture.isolated")
                .expect("valid environment"),
            selected.credential,
        );
        let mut session = block_on(driver.open_session(
            selected.plan,
            OpenSessionRequest::new(
                RequestId::new("gemini-open").expect("valid request"),
                selected.resource.clone(),
                None,
            ),
            services.clone(),
        ))
        .expect("session opens");
        assert_eq!(
            session
                .provider_session_ref()
                .expect("provider session exists")
                .as_provider_value(),
            "fixture-session"
        );
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("gemini-turn").expect("valid turn"),
                OperationContent::new("private fixture prompt").expect("valid prompt"),
            ),
            services,
        ))
        .expect("turn starts");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            outcome.output().expect("output exists").as_str(),
            "fixture response."
        );
        let mut events = turn.take_events().expect("events are available");
        let events = block_on(async move {
            let mut seen = Vec::new();
            while let Some(event) = events.next().await {
                seen.push(event.expect("event is valid"));
            }
            seen
        });
        assert!(
            events
                .iter()
                .any(|event| matches!(event.kind(), RuntimeEventKind::OutputDelta))
        );
        assert!(!format!("{events:?}").contains("private fixture prompt"));
        assert!(!format!("{outcome:?}").contains("fixture response"));
        assert_eq!(host.reads(), 1);
        let observed = host.observed_process();
        assert_eq!(observed.arguments, ["--acp", "--approval-mode", "plan"]);
        assert_eq!(observed.environment_count, 1);
        assert_eq!(observed.working_resource, Some(selected.resource));
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(host.releases(), 1);
    }
}

#[test]
fn permission_is_observed_cancelled_and_never_becomes_a_consumer_callback() {
    let (host, mut session, services) = open(Scenario::Permission, "permission");
    let mut turn = start(&mut *session, services, "permission-turn");
    assert!(turn.take_callbacks().is_none());
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderRequestObserved(_)
    ));
    assert!(host.writes().iter().any(|message| {
        message.get("method").and_then(serde_json::Value::as_str) == Some("session/cancel")
    }));
    assert!(host.writes().iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_u64) == Some(900)
            && message["result"]["outcome"]["outcome"] == "cancelled"
    }));
    assert_eq!(
        ProviderRequestHandling::Reject,
        swallowtail_core::ProviderRequestPolicy::reject_all().handling_for(
            &swallowtail_core::ExtensionNamespace::new("acp/session/request-permission")
                .expect("valid namespace")
        )
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn active_turn_cancellation_waits_for_cancelled_prompt_result() {
    let (_host, mut session, services) = open(Scenario::Cancellation, "cancellation");
    let mut turn = start(&mut *session, services, "cancel-turn");
    block_on(turn.cancellation().request()).expect("cancellation is sent");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn disconnect_fails_the_turn_and_session_close_still_joins_cleanup() {
    let (_host, mut session, services) = open(Scenario::Disconnect, "disconnect");
    let mut turn = start(&mut *session, services, "disconnect-turn");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

fn open(
    scenario: Scenario,
    suffix: &str,
) -> (
    FixtureHost,
    Box<dyn swallowtail_runtime::InteractiveSessionHandle>,
    swallowtail_runtime::HostServices,
) {
    let host_id = ExecutionHostId::new(format!("fixture.host.{suffix}")).expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(scenario);
    let services = host.services(host_id);
    let driver = GeminiAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("gemini.fixture.isolated")
            .expect("valid environment"),
        selected.credential,
    );
    let session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new(format!("gemini-{suffix}")).expect("valid request"),
            selected.resource,
            None,
        ),
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
