//! A derived turn scope is fallible, and the driver must reach that decision
//! before the first provider side effect. A rejected scope that ran after
//! `set_active_turn` or `session/prompt` would wedge every later turn on the
//! same session, so this proves the refusal leaves the session usable.

use super::support::selection::{open_request, selection};
use super::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::ClaudeAgentAcpDriver;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    CleanupOutcome, EnvironmentRef, InteractiveSessionDriver, OperationContent, RuntimeTurnId,
    TerminalStatus, TurnRequest,
};

const OVERLONG_TURN_BYTES: usize = 512;

#[test]
fn an_overlong_turn_scope_is_refused_before_the_prompt_and_leaves_the_session_usable() {
    let host_id = ExecutionHostId::new("fixture.host.turn-scope").expect("valid host");
    let selected = selection(host_id.clone(), "0.61.0");
    let host = FixtureHost::new(Scenario::Success, "0.61.0");
    let services = host.services(host_id);
    let driver = ClaudeAgentAcpDriver::new(
        EnvironmentRef::new("claude-agent.fixture.environment").expect("valid environment"),
        selected.credential,
    );
    let mut session = block_on(driver.open_session(
        selected.plan,
        open_request("turn-scope-open", selected.resource),
        services.clone(),
    ))
    .expect("session opens");

    let overlong = RuntimeTurnId::new("t".repeat(OVERLONG_TURN_BYTES)).expect("identity is valid");
    let error = match block_on(session.start_turn(
        TurnRequest::new(
            overlong,
            OperationContent::new("private fixture prompt").expect("valid prompt"),
        ),
        services.clone(),
    )) {
        Ok(_turn) => panic!("an overlong turn scope must be refused"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude_agent.acp.scope_invalid"
    );
    assert!(
        !host.writes().iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/prompt")
        }),
        "no prompt reaches the provider for a refused scope"
    );

    // The session is not wedged: a bounded turn still starts and completes.
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("bounded-turn").expect("valid turn"),
            OperationContent::new("private fixture prompt").expect("valid prompt"),
        ),
        services.clone(),
    ))
    .expect("the session still accepts a bounded turn");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
    assert_eq!(host.resource_releases(), 1);
    assert_eq!(host.credential_releases(), 1);
}
