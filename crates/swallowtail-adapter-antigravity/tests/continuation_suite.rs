mod plan;
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use support::{FixtureHost, ImmediateTime};
use swallowtail_adapter_antigravity::AntigravityHeadlessDriver;
use swallowtail_core::{CancellationScope, ExecutionHostId};
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, EnvironmentRef,
    InteractiveSessionDriver, MonotonicInstant, OpenSessionRequest, OperationContent, RequestId,
    RuntimeTurnId, TerminalStatus, TurnRequest, WorkingResourceRef,
};

const FIRST: &str = include_str!("fixtures/antigravity-cli-1.1.9/continuation-first.jsonl");
const SECOND: &str = include_str!("fixtures/antigravity-cli-1.1.9/continuation-second.jsonl");
const MISMATCH: &str = include_str!("fixtures/antigravity-cli-1.1.9/continuation-mismatch.jsonl");

#[test]
fn later_turn_uses_only_the_exact_private_conversation_id() {
    for host_value in [
        "fixture.antigravity.continuation.local",
        "fixture.antigravity.continuation.remote",
    ] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let plan = plan::continuation_plan(host_id.clone(), "antigravity.fixture.executable");
        let host = FixtureHost::scripted(&[FIRST, SECOND]);
        let services = host.services(host_id);
        let mut session = block_on(
            driver().open_session(
                plan.clone(),
                OpenSessionRequest::from_plan(
                    &plan,
                    RequestId::new("antigravity-continuation").expect("valid request"),
                    WorkingResourceRef::new("workspace.main").expect("valid resource"),
                    None,
                )
                .expect("request from plan"),
                services.clone(),
            ),
        )
        .expect("session opens");
        assert!(session.provider_session_ref().is_none());
        assert!(session.resume_binding().is_none());

        for (index, prompt) in ["first prompt", "second prompt"].into_iter().enumerate() {
            let mut turn =
                block_on(session.start_turn(turn_request(index + 1, prompt), services.clone()))
                    .expect("turn starts");
            let mut events = turn.take_events().expect("event stream");
            let terminal = block_on(
                turn.take_terminal_outcome()
                    .expect("terminal outcome is available"),
            );
            while block_on(events.next()).is_some() {}
            assert_eq!(terminal.status(), &TerminalStatus::Completed);
            assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        }

        let observations = host.observations();
        assert_eq!(observations.len(), 2);
        assert!(
            !observations[0]
                .arguments
                .iter()
                .any(|argument| argument == "--conversation" || argument == "--continue")
        );
        assert!(
            observations[1]
                .arguments
                .windows(2)
                .any(|pair| { pair == ["--conversation", "fixture-conversation"] })
        );
        assert!(
            !observations[1]
                .arguments
                .iter()
                .any(|argument| argument == "--continue")
        );
        assert!(observations.iter().all(|observation| {
            observation
                .arguments
                .windows(2)
                .any(|pair| pair == ["--mode", "plan"])
                && !observation
                    .arguments
                    .iter()
                    .any(|argument| argument == "--dangerously-skip-permissions")
        }));
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn a_mismatched_conversation_invalidates_the_runtime_handle_without_fallback() {
    let host_id = ExecutionHostId::new("fixture.antigravity.mismatch").expect("valid host");
    let plan = plan::continuation_plan(host_id.clone(), "antigravity.fixture.executable");
    let host = FixtureHost::scripted(&[FIRST, MISMATCH]);
    let services = host.services(host_id);
    let mut session = block_on(
        driver().open_session(
            plan.clone(),
            OpenSessionRequest::from_plan(
                &plan,
                RequestId::new("antigravity-mismatch").expect("valid request"),
                WorkingResourceRef::new("workspace.main").expect("valid resource"),
                None,
            )
            .expect("request from plan"),
            services.clone(),
        ),
    )
    .expect("session opens");

    let first = completed_turn(&mut session, turn_request(1, "first"), services.clone());
    assert_eq!(first.status(), &TerminalStatus::Completed);
    let second = completed_turn(&mut session, turn_request(2, "second"), services.clone());
    let TerminalStatus::RuntimeFailed(diagnostic) = second.status() else {
        panic!("mismatch remains a runtime failure");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.antigravity.headless.conversation_mismatch"
    );
    assert!(!format!("{second:?}").contains("different-private-conversation"));

    let error = match block_on(session.start_turn(turn_request(3, "third"), services)) {
        Ok(_) => panic!("invalidated session must reject later turns"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.antigravity.headless.session_unusable"
    );
    assert_eq!(host.observations().len(), 2);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn active_turn_cancellation_force_stops_joins_and_invalidates_continuation() {
    let host_id = ExecutionHostId::new("fixture.antigravity.cancel").expect("valid host");
    let plan = plan::continuation_plan(host_id.clone(), "antigravity.fixture.executable");
    let host = FixtureHost::held_open();
    let services = host.services(host_id);
    let mut session = block_on(
        driver().open_session(
            plan.clone(),
            OpenSessionRequest::from_plan(
                &plan,
                RequestId::new("antigravity-cancel").expect("valid request"),
                WorkingResourceRef::new("workspace.main").expect("valid resource"),
                None,
            )
            .expect("request from plan"),
            services.clone(),
        ),
    )
    .expect("session opens");
    let mut turn = block_on(session.start_turn(turn_request(1, "cancel"), services.clone()))
        .expect("turn starts");
    assert_eq!(turn.cancellation().scope(), CancellationScope::ActiveTurn);
    assert_eq!(
        block_on(turn.cancellation().request()).expect("cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert!(host.force_stopped());
    assert!(host.waited());
    assert!(host.joined());
    assert!(block_on(session.start_turn(turn_request(2, "later"), services)).is_err());
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn missing_first_conversation_identity_fails_closed() {
    let host_id = ExecutionHostId::new("fixture.antigravity.missing").expect("valid host");
    let plan = plan::continuation_plan(host_id.clone(), "antigravity.fixture.executable");
    let missing = FIRST.replace("fixture-conversation", "");
    let host = FixtureHost::scripted(&[&missing]);
    let services = host.services(host_id);
    let mut session = block_on(
        driver().open_session(
            plan.clone(),
            OpenSessionRequest::from_plan(
                &plan,
                RequestId::new("antigravity-missing").expect("valid request"),
                WorkingResourceRef::new("workspace.main").expect("valid resource"),
                None,
            )
            .expect("request from plan"),
            services.clone(),
        ),
    )
    .expect("session opens");
    let terminal = completed_turn(&mut session, turn_request(1, "first"), services.clone());
    let TerminalStatus::RuntimeFailed(diagnostic) = terminal.status() else {
        panic!("missing identity remains a runtime failure");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.antigravity.headless.malformed_stream"
    );
    assert!(block_on(session.start_turn(turn_request(2, "later"), services)).is_err());
    assert_eq!(host.observations().len(), 1);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn turn_deadline_is_terminal_joined_and_invalidates_continuation() {
    let host_id = ExecutionHostId::new("fixture.antigravity.deadline").expect("valid host");
    let plan = plan::continuation_plan(host_id.clone(), "antigravity.fixture.executable");
    let host = FixtureHost::held_open();
    let services = host.services_with_time(host_id, Arc::new(ImmediateTime));
    let mut session = block_on(
        driver().open_session(
            plan.clone(),
            OpenSessionRequest::from_plan(
                &plan,
                RequestId::new("antigravity-deadline").expect("valid request"),
                WorkingResourceRef::new("workspace.main").expect("valid resource"),
                None,
            )
            .expect("request from plan"),
            services.clone(),
        ),
    )
    .expect("session opens");
    let terminal = completed_turn(&mut session, turn_request(1, "deadline"), services.clone());
    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    assert!(host.force_stopped());
    assert!(host.waited());
    assert!(host.joined());
    assert!(block_on(session.start_turn(turn_request(2, "later"), services)).is_err());
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

fn driver() -> AntigravityHeadlessDriver {
    AntigravityHeadlessDriver::new(
        EnvironmentRef::new("antigravity.fixture.environment").expect("valid environment"),
    )
}

fn turn_request(index: usize, prompt: &str) -> TurnRequest {
    TurnRequest::new(
        RuntimeTurnId::new(format!("antigravity-turn-{index}")).expect("valid turn id"),
        OperationContent::new(prompt).expect("valid prompt"),
    )
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)))
}

fn completed_turn(
    session: &mut Box<dyn swallowtail_runtime::InteractiveSessionHandle>,
    request: TurnRequest,
    services: swallowtail_runtime::HostServices,
) -> swallowtail_runtime::TerminalOutcome {
    let mut turn = block_on(session.start_turn(request, services)).expect("turn starts");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    terminal
}
