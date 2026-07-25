use super::support::selection::{open_request, selection};
use super::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::ClaudeAgentAcpDriver;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    CleanupOutcome, Deadline, EnvironmentRef, InteractiveSessionDriver, MonotonicInstant,
    OperationContent, RuntimeTurnId, TerminalStatus, TurnRequest,
};

#[test]
fn deadline_times_out_cancels_and_joins_before_cleanup() {
    let host_id = ExecutionHostId::new("fixture.host.deadline").expect("valid host");
    let selected = selection(host_id.clone(), "0.61.0");
    let host = FixtureHost::new(Scenario::Cancellation, "0.61.0").with_immediate_deadline();
    let services = host.services(host_id);
    let driver = ClaudeAgentAcpDriver::new(
        EnvironmentRef::new("claude-agent.fixture.environment").expect("valid environment"),
        selected.credential,
    );
    let mut session = block_on(driver.open_session(
        selected.plan,
        open_request("deadline-open", selected.resource),
        services.clone(),
    ))
    .expect("session opens");
    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("deadline-turn").expect("valid turn"),
                OperationContent::new("private fixture prompt").expect("valid prompt"),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1))),
            services,
        ),
    )
    .expect("turn starts");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert!(host.writes().iter().any(|message| {
        message.get("method").and_then(serde_json::Value::as_str) == Some("session/cancel")
    }));
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(host.resource_releases(), 1);
    assert_eq!(host.credential_releases(), 1);
}
