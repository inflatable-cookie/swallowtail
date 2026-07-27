use super::fixture::{id, prepare, session_profile, turn};
use crate::interactive_support::{InteractiveFixtureServer, InteractiveScenario};
use crate::lifecycle_support::FixtureHost;
use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_adapter_kimi::KimiLocalServerPermissionMode;
use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, JoinedTask, MonotonicInstant, ProviderCancellationOutcome,
    RuntimeFailure, ScopeId, ScopedTaskService, TerminalOutcome, TerminalStatus,
};

#[test]
fn cancellation_resync_and_disconnect_keep_distinct_terminal_truth() {
    let cancelled = run_scenario(InteractiveScenario::Cancel, true);
    assert_eq!(cancelled.status(), &TerminalStatus::Cancelled);
    assert_eq!(
        cancelled.provider_cancellation(),
        Some(ProviderCancellationOutcome::Confirmed)
    );

    let resync = run_scenario(InteractiveScenario::Resync, false);
    assert!(matches!(
        resync.status(),
        TerminalStatus::RuntimeFailed(diagnostic)
            if diagnostic.code() == "swallowtail.kimi.local_server.websocket_resync_required"
    ));

    let disconnected = run_scenario(InteractiveScenario::Disconnect, false);
    assert!(matches!(
        disconnected.status(),
        TerminalStatus::RuntimeFailed(diagnostic)
            if diagnostic.code() == "swallowtail.kimi.local_server.websocket_disconnected"
    ));

    let undeclared = run_scenario(InteractiveScenario::UnexpectedApproval, false);
    assert!(matches!(
        undeclared.status(),
        TerminalStatus::RuntimeFailed(diagnostic)
            if diagnostic.code() == "swallowtail.kimi.local_server.provider_request_rejected"
    ));
}

#[test]
fn turn_deadline_aborts_provider_work_and_reports_timeout() {
    let server = InteractiveFixtureServer::start(InteractiveScenario::Cancel);
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.deadline");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services.clone(), "0.29.0");
    let profile = session_profile(&prepared, KimiLocalServerPermissionMode::Auto, "deadline");
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    let request =
        turn("turn-deadline").with_deadline(Deadline::at(MonotonicInstant::from_ticks(10)));
    let mut turn = block_on(session.start_turn(request, services)).expect("turn starts");
    host.set_now(10);
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn task_admission_failure_precedes_websocket_and_prompt_effects() {
    let server = InteractiveFixtureServer::start(InteractiveScenario::Complete);
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.task-rejection");
    let services = host
        .services(execution_host.clone(), false)
        .with_task(Arc::new(RejectingTaskService));
    let prepared = prepare(execution_host, services.clone(), "0.29.0");
    let profile = session_profile(
        &prepared,
        KimiLocalServerPermissionMode::Auto,
        "task-rejection",
    );
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    let requests_before_turn = server.requests();
    let failure = match block_on(session.start_turn(turn("turn-task-rejection"), services)) {
        Ok(_) => panic!("task rejection must stop the turn"),
        Err(failure) => failure,
    };
    assert_eq!(failure.diagnostic().code(), "fixture.kimi.task_rejected");
    assert_eq!(server.requests(), requests_before_turn);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

fn run_scenario(scenario: InteractiveScenario, cancel: bool) -> TerminalOutcome {
    let server = InteractiveFixtureServer::start(scenario);
    let host = FixtureHost::for_endpoint(server.endpoint());
    let execution_host = id(ExecutionHostId::new, "fixture.kimi.failure");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare(execution_host, services.clone(), "0.29.0");
    let profile = session_profile(&prepared, KimiLocalServerPermissionMode::Auto, "failure");
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    let mut turn =
        block_on(session.start_turn(turn("turn-failure"), services)).expect("turn starts");
    if cancel {
        block_on(turn.cancellation().request()).expect("cancellation is acknowledged");
    }
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    let _ = block_on(turn.close());
    let _ = block_on(session.close());
    outcome
}

struct RejectingTaskService;

impl ScopedTaskService for RejectingTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        _task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Err(RuntimeFailure::new(SafeDiagnostic::new(
            "fixture.kimi.task_rejected",
            "Fixture rejected task admission",
        )))
    }
}
