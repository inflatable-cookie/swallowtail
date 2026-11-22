use super::settle_deadline_and_attachment;
use crate::sidecar::driver::input::SharedAttachmentMaterialization;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{BoxFuture, CleanupOutcome, JoinedTask, RuntimeFailure};

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

#[test]
fn a_sidecar_deadline_task_that_fails_to_join_is_reported_at_session_close() {
    let cleanup = futures_executor::block_on(settle_deadline_and_attachment(
        Some(Box::new(FailingJoin)),
        SharedAttachmentMaterialization::default(),
    ));

    assert!(
        matches!(&cleanup, CleanupOutcome::Failed(diagnostic) if diagnostic.message().contains("deadline_join_failed")),
        "a deadline task that does not join is a cleanup failure, got {cleanup:?}"
    );
}

#[test]
fn a_sidecar_session_without_a_deadline_task_settles_clean() {
    assert_eq!(
        futures_executor::block_on(settle_deadline_and_attachment(
            None,
            SharedAttachmentMaterialization::default(),
        )),
        CleanupOutcome::NotApplicable
    );
}

#[test]
fn sidecar_spawn_deadline_escalates_failed_abort_to_close_and_force_stop() {
    use super::spawn_deadline;
    use crate::sidecar::connection::SidecarConnection;
    use crate::sidecar::turn::SidecarActiveTurn;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
    use swallowtail_core::ExecutionHostId;
    use swallowtail_runtime::{
        Deadline, DeadlineObservation, HostServices, MonotonicInstant, ProcessExit, ProcessHandle,
        ProcessInputChunk, ProcessOutputChunk, RuntimeTurnId, ScopeId, ScopedTaskService,
        TerminalStatus, TimeService,
    };

    struct RefusingAbortProcess {
        force_stops: AtomicUsize,
    }

    impl ProcessHandle for RefusingAbortProcess {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "fixture.abort_write_failed",
                    "abort write failed",
                )))
            })
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
            self.force_stops.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
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

    struct InlineTaskService;
    struct InlineTask(Option<BoxFuture<'static, ()>>);
    impl JoinedTask for InlineTask {
        fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let fut = self.0.take();
            Box::pin(async move {
                if let Some(fut) = fut {
                    fut.await;
                }
                Ok(())
            })
        }
    }
    impl ScopedTaskService for InlineTaskService {
        fn spawn(
            &self,
            _scope: ScopeId,
            task: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            Ok(Box::new(InlineTask(Some(task))))
        }
    }

    let process = Arc::new(RefusingAbortProcess {
        force_stops: AtomicUsize::new(0),
    });
    let services = HostServices::new(
        ExecutionHostId::new("pi.sidecar.deadline-escalate").expect("host id is valid"),
    )
    .with_task(Arc::new(InlineTaskService))
    .with_time(Arc::new(FixedClock));
    let connection = SidecarConnection::new(
        Arc::clone(&process) as Arc<dyn ProcessHandle>,
        services.clone(),
    );
    let (turn, _events, terminal) = SidecarActiveTurn::new(
        RuntimeTurnId::new("sidecar-deadline-escalate").expect("turn id is valid"),
        Arc::new(AtomicU32::new(0)),
    )
    .expect("sidecar turn constructs");
    connection
        .set_active_turn(Arc::clone(&turn))
        .expect("turn attaches");

    let task = spawn_deadline(
        &services,
        Arc::clone(&connection),
        Arc::clone(&turn),
        Deadline::at(MonotonicInstant::from_ticks(1)),
    )
    .expect("deadline task spawns");
    futures_executor::block_on(task.join()).expect("deadline task joins");

    assert_eq!(
        process.force_stops.load(Ordering::SeqCst),
        1,
        "failed sidecar deadline abort must escalate to force_stop"
    );
    assert!(
        turn.is_finished(),
        "sidecar turn must finish after deadline abort escalation"
    );
    let outcome = futures_executor::block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
}
