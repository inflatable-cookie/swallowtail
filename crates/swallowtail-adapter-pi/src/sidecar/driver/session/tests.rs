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

    let (arm_tx, arm_rx) = futures_channel::oneshot::channel();
    let _ = arm_tx.send(());
    let task = spawn_deadline(
        &services,
        Arc::clone(&connection),
        Arc::clone(&turn),
        Deadline::at(MonotonicInstant::from_ticks(1)),
        arm_rx,
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

#[test]
fn sidecar_unacknowledged_prompt_and_abort_do_not_hang_and_invalidate_connection() {
    use super::{PiSdkSidecarSessionHandle, spawn_deadline};
    use crate::sidecar::connection::SidecarConnection;
    use crate::sidecar::driver::handle::SessionCancellation;
    use crate::sidecar::turn::SidecarActiveTurn;
    use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use swallowtail_core::{
        ConfiguredInstanceId, ExecutionHostId, ModelId, ModelRouteId, SessionAccessPolicy,
        SessionRef,
    };
    use swallowtail_runtime::{
        Deadline, DeadlineObservation, HostServices, InteractiveSessionHandle, MonotonicInstant,
        OperationContent, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
        RequestId, RuntimeSessionId, RuntimeTurnId, ScopeId, ScopedTaskService,
        SessionResumeBinding, TerminalStatus, TimeService, TurnRequest, WorkingResourceRef,
    };

    struct SilentProcess {
        writes: AtomicUsize,
        force_stops: AtomicUsize,
    }

    impl ProcessHandle for SilentProcess {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.writes.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(std::future::pending())
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

    let host = ExecutionHostId::new("pi.sidecar.unack-prompt").expect("host id");
    let process = Arc::new(SilentProcess {
        writes: AtomicUsize::new(0),
        force_stops: AtomicUsize::new(0),
    });
    let services = HostServices::new(host.clone())
        .with_task(Arc::new(InlineTaskService))
        .with_time(Arc::new(FixedClock));
    let connection = SidecarConnection::new(
        Arc::clone(&process) as Arc<dyn ProcessHandle>,
        services.clone(),
    );
    let active: super::ActiveSlot = Arc::new(Mutex::new(None));
    let binding = SessionResumeBinding::new(
        SessionRef::new("sidecar-session-1").expect("session ref"),
        ConfiguredInstanceId::new("pi.sidecar.inst").expect("instance id"),
        host.clone(),
        ModelRouteId::new("pi.route").expect("route id"),
        ModelId::new("pi-model").expect("model id"),
        WorkingResourceRef::new("workspace.main").expect("resource"),
        SessionAccessPolicy::read_only(),
    );
    let mut session = PiSdkSidecarSessionHandle {
        request_id: RequestId::new("req-sidecar-unack").expect("req id"),
        runtime_id: RuntimeSessionId::new("pi-sdk-sidecar:unack").expect("session id"),
        execution_host_id: host,
        binding,
        connection: Arc::clone(&connection),
        cancellation: SessionCancellation::new(Arc::clone(&connection), Arc::clone(&active)),
        pump_task: None,
        services: services.clone(),
        resource: None,
        credential: None,
        active,
        completed_prompts: Arc::new(AtomicU32::new(0)),
        image_attachments: false,
    };
    let req = TurnRequest::new(
        RuntimeTurnId::new("sidecar-unack-1").expect("turn id"),
        OperationContent::new("hello").expect("content"),
    )
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1)));
    let err = futures_executor::block_on(session.start_turn(req, services.clone()))
        .err()
        .expect("unacknowledged sidecar prompt must time out");
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.prompt_timed_out"
    );
    assert_eq!(
        process.writes.load(Ordering::SeqCst),
        1,
        "prompt write succeeded"
    );
    assert_eq!(
        process.force_stops.load(Ordering::SeqCst),
        1,
        "timed-out sidecar prompt must force-stop the uncertain process"
    );
    let req2 = TurnRequest::new(
        RuntimeTurnId::new("sidecar-unack-2").expect("turn id"),
        OperationContent::new("next").expect("content"),
    )
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1)));
    let err2 = futures_executor::block_on(session.start_turn(req2, services.clone()))
        .err()
        .expect("subsequent turn must be rejected on invalidated sidecar connection");
    assert_eq!(
        err2.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.prompt_timed_out"
    );

    // Active sidecar turn deadline fires, abort write succeeds but is never
    // acknowledged: spawn_deadline bounds the wait, closes, and force-stops.
    let process2 = Arc::new(SilentProcess {
        writes: AtomicUsize::new(0),
        force_stops: AtomicUsize::new(0),
    });
    let connection2 = SidecarConnection::new(
        Arc::clone(&process2) as Arc<dyn ProcessHandle>,
        services.clone(),
    );
    let (turn2, _events2, terminal2) = SidecarActiveTurn::new(
        RuntimeTurnId::new("sidecar-unack-abort").expect("turn id"),
        Arc::new(AtomicU32::new(0)),
    )
    .expect("sidecar turn constructs");
    connection2
        .set_active_turn(Arc::clone(&turn2))
        .expect("turn attaches");
    let (arm_tx, arm_rx) = futures_channel::oneshot::channel();
    let _ = arm_tx.send(());
    let task = spawn_deadline(
        &services,
        Arc::clone(&connection2),
        Arc::clone(&turn2),
        Deadline::at(MonotonicInstant::from_ticks(1)),
        arm_rx,
    )
    .expect("deadline task spawns");
    futures_executor::block_on(task.join()).expect("deadline task joins without hanging");
    assert_eq!(
        process2.writes.load(Ordering::SeqCst),
        1,
        "abort write succeeded"
    );
    assert_eq!(
        process2.force_stops.load(Ordering::SeqCst),
        1,
        "unacknowledged sidecar deadline abort must escalate to force_stop"
    );
    assert_eq!(
        futures_executor::block_on(terminal2).status(),
        &TerminalStatus::TimedOut
    );
}
