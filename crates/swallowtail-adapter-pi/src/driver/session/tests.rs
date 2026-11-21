use super::{ActiveTask, settle_active_task};
use crate::connection::PiConnection;
use crate::driver::input::SharedAttachmentMaterialization;
use crate::turn::ActiveTurn;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DeadlineObservation, HostServices, JoinedTask,
    MonotonicInstant, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    RuntimeFailure, RuntimeTurnId, ScopeId, ScopedTaskService, TimeService,
};

/// Child that accepts every control call and never produces output.
struct QuietProcess;

impl ProcessHandle for QuietProcess {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async { Ok(None) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

struct RefusingTasks;

impl ScopedTaskService for RefusingTasks {
    fn spawn(
        &self,
        _scope: ScopeId,
        _task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Err(RuntimeFailure::new(SafeDiagnostic::new(
            "fixture.spawn_refused",
            "fixture refuses task spawns",
        )))
    }
}

struct FixedClock;

impl TimeService for FixedClock {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::ready(DeadlineObservation::new(
            deadline,
            deadline.instant(),
        )))
    }
}

/// Deadline task whose join reports a failure.
struct FailingJoin;

impl JoinedTask for FailingJoin {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "fixture.join_failed",
                "fixture deadline task failed to join",
            )))
        })
    }
}

fn connection() -> Arc<PiConnection> {
    PiConnection::new(
        Arc::new(QuietProcess),
        Arc::new(RefusingTasks),
        Arc::new(FixedClock),
        HostServices::new(ExecutionHostId::new("pi.session.fixture").expect("host id is valid")),
    )
}

fn active_turn(
    connection: &Arc<PiConnection>,
    deadline_task: Option<Box<dyn JoinedTask>>,
) -> ActiveTask {
    let (turn, _events, _callbacks, _terminal) = ActiveTurn::new(
        RuntimeTurnId::new("turn-settle").expect("turn id is valid"),
        Arc::new(AtomicU32::new(0)),
        Arc::downgrade(connection),
        false,
    )
    .expect("turn state is valid");
    ActiveTask {
        turn,
        deadline_task,
        attachment: SharedAttachmentMaterialization::default(),
    }
}

#[test]
fn a_deadline_task_that_fails_to_join_is_reported_at_session_close() {
    let connection = connection();
    let active = active_turn(&connection, Some(Box::new(FailingJoin)));

    let cleanup = futures_executor::block_on(settle_active_task(Some(active), &connection));

    assert!(
        matches!(&cleanup, CleanupOutcome::Failed(diagnostic) if diagnostic.message().contains("deadline_join_failed")),
        "a deadline task that does not join is a cleanup failure, got {cleanup:?}"
    );
}

#[test]
fn a_session_without_an_active_turn_settles_clean() {
    let connection = connection();

    assert_eq!(
        futures_executor::block_on(settle_active_task(None, &connection)),
        CleanupOutcome::NotApplicable
    );
}
