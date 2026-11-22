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

#[test]
fn sidecar_dropping_pending_start_turn_on_join_on_drop_host_finishes_cleanup_without_deadlock() {
    use super::PiSdkSidecarSessionHandle;
    use crate::sidecar::connection::SidecarConnection;
    use crate::sidecar::driver::handle::SessionCancellation;
    use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, Waker};
    use swallowtail_core::{
        ConfiguredInstanceId, ExecutionHostId, ModelId, ModelRouteId, SessionAccessPolicy,
        SessionRef,
    };
    use swallowtail_runtime::{
        Deadline, DeadlineObservation, HostServices, InteractiveSessionHandle, MonotonicInstant,
        OperationContent, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
        RequestId, RuntimeSessionId, RuntimeTurnId, ScopeId, ScopedTaskService,
        SessionCleanupRequest, SessionResumeBinding, TimeService, TurnRequest, WorkingResourceRef,
    };

    struct SilentProcess {
        writes: AtomicUsize,
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
            Box::pin(async { Ok(()) })
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    struct PendingClock;
    impl TimeService for PendingClock {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(0)
        }
        fn wait_until(&self, _: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            Box::pin(std::future::pending())
        }
    }

    struct JoinOnDropThread(Option<std::thread::JoinHandle<()>>);
    impl Drop for JoinOnDropThread {
        fn drop(&mut self) {
            if let Some(handle) = self.0.take() {
                let _ = handle.join();
            }
        }
    }
    impl JoinedTask for JoinOnDropThread {
        fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let handle = self.0.take();
            Box::pin(async move {
                if let Some(handle) = handle {
                    let _ = handle.join();
                }
                Ok(())
            })
        }
    }
    struct JoinOnDropTaskService;
    impl ScopedTaskService for JoinOnDropTaskService {
        fn spawn(
            &self,
            _scope: ScopeId,
            task: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            Ok(Box::new(JoinOnDropThread(Some(std::thread::spawn(
                move || {
                    futures_executor::block_on(task);
                },
            )))))
        }
    }

    let host = ExecutionHostId::new("pi.sidecar.drop-pending-start").expect("host id");
    let process = Arc::new(SilentProcess {
        writes: AtomicUsize::new(0),
    });
    let services = HostServices::new(host.clone())
        .with_task(Arc::new(JoinOnDropTaskService))
        .with_time(Arc::new(PendingClock));
    let connection = SidecarConnection::new(
        Arc::clone(&process) as Arc<dyn ProcessHandle>,
        services.clone(),
    );
    let active: super::ActiveSlot = Arc::new(Mutex::new(None));
    let binding = SessionResumeBinding::new(
        SessionRef::new("sidecar-session-drop").expect("session ref"),
        ConfiguredInstanceId::new("pi.sidecar.inst").expect("instance id"),
        host.clone(),
        ModelRouteId::new("pi.route").expect("route id"),
        ModelId::new("pi-model").expect("model id"),
        WorkingResourceRef::new("workspace.main").expect("resource"),
        SessionAccessPolicy::read_only(),
    );
    let mut session = PiSdkSidecarSessionHandle {
        request_id: RequestId::new("req-sidecar-drop").expect("req id"),
        runtime_id: RuntimeSessionId::new("pi-sdk-sidecar:drop").expect("session id"),
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

    {
        let req = TurnRequest::new(
            RuntimeTurnId::new("sidecar-drop-pending").expect("turn id"),
            OperationContent::new("hello").expect("content"),
        )
        .with_deadline(Deadline::at(MonotonicInstant::from_ticks(10)));
        let mut fut = session.start_turn(req, services.clone());
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        assert!(matches!(fut.as_mut().poll(&mut cx), Poll::Pending));
        assert_eq!(process.writes.load(Ordering::SeqCst), 1);
    }

    let cleanup = futures_executor::block_on(Box::new(session).close(
        SessionCleanupRequest::new(Deadline::at(MonotonicInstant::from_ticks(100))),
        services,
    ));
    assert_eq!(cleanup, CleanupOutcome::Clean);
}

#[test]
fn sidecar_already_expired_prompt_deadline_makes_zero_writes_and_keeps_session_usable() {
    use super::PiSdkSidecarSessionHandle;
    use crate::sidecar::connection::SidecarConnection;
    use crate::sidecar::driver::handle::SessionCancellation;
    use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use swallowtail_core::{
        ConfiguredInstanceId, ExecutionHostId, ModelId, ModelRouteId, SessionAccessPolicy,
        SessionRef,
    };
    use swallowtail_runtime::{
        Deadline, DeadlineObservation, HostServices, InteractiveSessionHandle, MonotonicInstant,
        OperationContent, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
        ProcessOutputStream, RequestId, RuntimeSessionId, RuntimeTurnId, ScopeId,
        ScopedTaskService, SessionResumeBinding, TimeService, TurnRequest, WorkingResourceRef,
    };

    struct RespondingProcess {
        writes: AtomicUsize,
        force_stops: AtomicUsize,
        output: Mutex<(bool, Option<Vec<u8>>)>,
        written: std::sync::Condvar,
    }
    impl ProcessHandle for RespondingProcess {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.writes.fetch_add(1, Ordering::SeqCst);
            let mut state = self.output.lock().expect("output lock");
            state.0 = true;
            self.written.notify_all();
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            let mut state = self.output.lock().expect("output lock");
            while !state.0 {
                state = self.written.wait(state).expect("output lock");
            }
            let line = state.1.take();
            Box::pin(async move {
                Ok(line.map(|bytes| ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes)))
            })
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

    struct AtTenClock;
    impl TimeService for AtTenClock {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(10)
        }
        fn wait_until(&self, _: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            Box::pin(std::future::pending())
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

    let host = ExecutionHostId::new("pi.sidecar.expired-deadline").expect("host id");
    let process = Arc::new(RespondingProcess {
        writes: AtomicUsize::new(0),
        force_stops: AtomicUsize::new(0),
        output: Mutex::new((
            false,
            Some(
                b"{\"type\":\"response\",\"id\":\"prompt:sidecar-valid\",\"command\":\"prompt\",\"success\":true}\n"
                    .to_vec(),
            ),
        )),
        written: std::sync::Condvar::new(),
    });
    let services = HostServices::new(host.clone())
        .with_task(Arc::new(InlineTaskService))
        .with_time(Arc::new(AtTenClock));
    let connection = SidecarConnection::new(
        Arc::clone(&process) as Arc<dyn ProcessHandle>,
        services.clone(),
    );
    let active: super::ActiveSlot = Arc::new(Mutex::new(None));
    let binding = SessionResumeBinding::new(
        SessionRef::new("sidecar-session-expired").expect("session ref"),
        ConfiguredInstanceId::new("pi.sidecar.inst").expect("instance id"),
        host.clone(),
        ModelRouteId::new("pi.route").expect("route id"),
        ModelId::new("pi-model").expect("model id"),
        WorkingResourceRef::new("workspace.main").expect("resource"),
        SessionAccessPolicy::read_only(),
    );
    let mut session = PiSdkSidecarSessionHandle {
        request_id: RequestId::new("req-sidecar-expired").expect("req id"),
        runtime_id: RuntimeSessionId::new("pi-sdk-sidecar:expired").expect("session id"),
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

    let expired_req = TurnRequest::new(
        RuntimeTurnId::new("sidecar-expired").expect("turn id"),
        OperationContent::new("too late").expect("content"),
    )
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(5)));
    let err = futures_executor::block_on(session.start_turn(expired_req, services.clone()))
        .err()
        .expect("expired deadline must fail");
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.prompt_timed_out"
    );
    assert_eq!(process.writes.load(Ordering::SeqCst), 0);
    assert_eq!(process.force_stops.load(Ordering::SeqCst), 0);

    let pump_conn = Arc::clone(&connection);
    let pump_thread = std::thread::spawn(move || futures_executor::block_on(pump_conn.pump()));
    let valid_req = TurnRequest::new(
        RuntimeTurnId::new("sidecar-valid").expect("turn id"),
        OperationContent::new("valid").expect("content"),
    )
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(50)));
    let turn = futures_executor::block_on(session.start_turn(valid_req, services))
        .expect("sidecar session must still accept a valid turn after pre-dispatch timeout");
    assert_eq!(process.writes.load(Ordering::SeqCst), 1);
    assert_eq!(process.force_stops.load(Ordering::SeqCst), 0);
    drop(turn);
    pump_thread.join().expect("pump joins");
}

#[test]
fn sidecar_deadline_abort_after_deferred_settlement_resolves_timed_out() {
    use crate::sidecar::connection::SidecarConnection;
    use crate::sidecar::turn::SidecarActiveTurn;
    use crate::sidecar::wire::PiSdkSidecarEvent;
    use std::collections::VecDeque;
    use std::sync::atomic::AtomicU32;
    use std::sync::{Arc, Condvar, Mutex};
    use swallowtail_core::ExecutionHostId;
    use swallowtail_runtime::{
        BoxFuture, Deadline, DeadlineObservation, HostServices, JoinedTask, MonotonicInstant,
        ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream,
        RuntimeFailure, RuntimeTurnId, ScopeId, ScopedTaskService, TerminalStatus, TimeService,
    };

    struct HeldAbortProcess {
        state: Mutex<(VecDeque<Vec<u8>>, bool)>,
        changed: Condvar,
    }
    impl HeldAbortProcess {
        fn push_line(&self, line: Vec<u8>) {
            let mut state = self.state.lock().expect("process lock");
            state.0.push_back(line);
            self.changed.notify_all();
        }
        fn close_stdout(&self) {
            let mut state = self.state.lock().expect("process lock");
            state.1 = true;
            self.changed.notify_all();
        }
    }
    impl ProcessHandle for HeldAbortProcess {
        fn write_stdin(
            &self,
            chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            let text = String::from_utf8_lossy(chunk.bytes());
            if text.contains("deadline-abort:sidecar-deferred-deadline") {
                self.push_line(
                    b"{\"type\":\"response\",\"id\":\"deadline-abort:sidecar-deferred-deadline\",\"command\":\"abort\",\"success\":true}\n"
                        .to_vec(),
                );
            }
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            let mut state = self.state.lock().expect("process lock");
            loop {
                if let Some(line) = state.0.pop_front() {
                    return Box::pin(async move {
                        Ok(Some(ProcessOutputChunk::new(
                            ProcessOutputStream::Stdout,
                            line,
                        )))
                    });
                }
                if state.1 {
                    return Box::pin(async { Ok(None) });
                }
                state = self.changed.wait(state).expect("process lock");
            }
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.close_stdout();
            Box::pin(async { Ok(()) })
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.close_stdout();
            Box::pin(async { Ok(()) })
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    struct TriggeredTime {
        fire_rx: Mutex<Option<futures_channel::oneshot::Receiver<()>>>,
    }
    impl TimeService for TriggeredTime {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(10)
        }
        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            let rx = self.fire_rx.lock().expect("time lock").take();
            Box::pin(async move {
                match rx {
                    Some(rx) => {
                        let _ = rx.await;
                        DeadlineObservation::new(deadline, MonotonicInstant::from_ticks(10))
                    }
                    None => std::future::pending().await,
                }
            })
        }
    }

    struct ThreadTaskService;
    struct ThreadJoinedTask(Option<std::thread::JoinHandle<()>>);
    impl JoinedTask for ThreadJoinedTask {
        fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let handle = self.0.take();
            Box::pin(async move {
                if let Some(handle) = handle {
                    let _ = handle.join();
                }
                Ok(())
            })
        }
    }
    impl ScopedTaskService for ThreadTaskService {
        fn spawn(
            &self,
            _scope: ScopeId,
            task: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            let handle = std::thread::spawn(move || futures_executor::block_on(task));
            Ok(Box::new(ThreadJoinedTask(Some(handle))))
        }
    }

    let (fire_tx, fire_rx) = futures_channel::oneshot::channel();
    let time: Arc<dyn TimeService> = Arc::new(TriggeredTime {
        fire_rx: Mutex::new(Some(fire_rx)),
    });
    let task_service: Arc<dyn ScopedTaskService> = Arc::new(ThreadTaskService);
    let host = ExecutionHostId::new("pi.sidecar.deferred-deadline").expect("host id");
    let services = HostServices::new(host)
        .with_task(task_service)
        .with_time(time);
    let process = Arc::new(HeldAbortProcess {
        state: Mutex::new((VecDeque::new(), false)),
        changed: Condvar::new(),
    });
    let connection = SidecarConnection::new(
        Arc::clone(&process) as Arc<dyn ProcessHandle>,
        services.clone(),
    );
    let (turn, _events, terminal_fut) = SidecarActiveTurn::new(
        RuntimeTurnId::new("sidecar-deferred-deadline").expect("turn id"),
        Arc::new(AtomicU32::new(0)),
    )
    .expect("turn constructs");
    connection
        .set_active_turn(Arc::clone(&turn))
        .expect("turn attaches");

    let (arm_tx, arm_rx) = futures_channel::oneshot::channel();
    let deadline_task = super::spawn_deadline(
        &services,
        Arc::clone(&connection),
        Arc::clone(&turn),
        Deadline::at(MonotonicInstant::from_ticks(10)),
        arm_rx,
    )
    .expect("deadline task spawns");
    arm_tx.send(()).expect("arm deadline task");

    let pump_conn = Arc::clone(&connection);
    let pump_thread = std::thread::spawn(move || futures_executor::block_on(pump_conn.pump()));

    turn.mark_abort_in_flight();
    let mut manual_abort = Box::pin(connection.abort_command(
        "abort:sidecar-deferred-deadline".to_owned(),
        serde_json::json!({}),
        Arc::clone(&turn),
    ));
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(waker);
    assert!(matches!(
        manual_abort.as_mut().poll(&mut cx),
        std::task::Poll::Pending
    ));
    turn.handle_event(PiSdkSidecarEvent::Settled)
        .expect("sidecar settlement defers cleanly");
    assert!(!turn.is_finished());

    fire_tx.send(()).expect("fire deadline");
    let outcome = futures_executor::block_on(terminal_fut);
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    let manual_result = futures_executor::block_on(manual_abort)
        .expect("superseded manual sidecar abort waiter must wake on deadline abort");
    assert!(manual_result.success);

    process.push_line(
        b"{\"type\":\"response\",\"id\":\"abort:sidecar-deferred-deadline\",\"command\":\"abort\",\"success\":true}\n"
            .to_vec(),
    );

    futures_executor::block_on(deadline_task.join()).expect("deadline task joins");
    process.close_stdout();
    pump_thread.join().expect("pump joins");
}

#[test]
fn close_force_stops_an_unresponsive_sidecar_child_and_releases_leases() {
    use super::PiSdkSidecarSessionHandle;
    use crate::driver::close_tests::shutdown::{Pump, WedgedShutdown};
    use crate::sidecar::connection::SidecarConnection;
    use crate::sidecar::driver::handle::SessionCancellation;
    use std::sync::atomic::AtomicU32;
    use std::sync::{Arc, Mutex};
    use swallowtail_core::{
        ConfiguredInstanceId, ExecutionHostId, ModelId, ModelRouteId, SessionAccessPolicy,
        SessionRef,
    };
    use swallowtail_runtime::{
        RequestId, RuntimeSessionId, SessionResumeBinding, WorkingResourceRef,
    };

    let host = ExecutionHostId::new("pi.sidecar.wedged-close").expect("host id is valid");
    let fixture = Arc::new(WedgedShutdown::new(host.clone()));
    let services = fixture.services();
    let connection = SidecarConnection::new(fixture.clone(), services.clone());
    let active = Arc::new(Mutex::new(None));
    let binding = SessionResumeBinding::new(
        SessionRef::new("sidecar-wedged").expect("session ref is valid"),
        ConfiguredInstanceId::new("pi.sidecar.inst").expect("instance id is valid"),
        host.clone(),
        ModelRouteId::new("pi.route").expect("route id is valid"),
        ModelId::new("pi-model").expect("model id is valid"),
        WorkingResourceRef::new("pi.fixture.workspace").expect("resource ref is valid"),
        SessionAccessPolicy::read_only(),
    );
    let (resource, credential) = WedgedShutdown::leases();
    let session = PiSdkSidecarSessionHandle {
        request_id: RequestId::new("wedged-sidecar-close").expect("request id is valid"),
        runtime_id: RuntimeSessionId::new("pi-sidecar:wedged").expect("session id is valid"),
        execution_host_id: host,
        binding,
        connection: Arc::clone(&connection),
        cancellation: SessionCancellation::new(Arc::clone(&connection), Arc::clone(&active)),
        pump_task: Some(Box::new(Pump(Box::pin(connection.pump())))),
        services: services.clone(),
        resource,
        credential,
        active,
        completed_prompts: Arc::new(AtomicU32::new(0)),
        image_attachments: false,
    };
    fixture.assert_escalation(session, services, true);
}

#[test]
fn sidecar_reap_finished_releases_attachment_lease_when_deadline_join_fails() {
    use super::{ActiveSlot, ActiveTask, reap_finished};
    use crate::sidecar::turn::SidecarActiveTurn;
    use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use swallowtail_runtime::{
        AttachmentDescriptor, AttachmentFileLease, AttachmentRef, AttachmentService,
        MaterializedFileRef, RuntimeTurnId, ScopeId,
    };

    struct TrackingAttachmentService {
        releases: AtomicUsize,
    }
    impl AttachmentService for TrackingAttachmentService {
        fn materialize_file(
            &self,
            _scope: ScopeId,
            _descriptor: AttachmentDescriptor,
        ) -> BoxFuture<'static, Result<AttachmentFileLease, RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "fixture.unused",
                    "unused",
                )))
            })
        }
        fn release_file(&self, _lease: AttachmentFileLease) -> BoxFuture<'static, CleanupOutcome> {
            self.releases.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { CleanupOutcome::Clean })
        }
    }

    let (turn, _events, _terminal) = SidecarActiveTurn::new(
        RuntimeTurnId::new("sidecar-reap-turn").expect("turn id"),
        Arc::new(AtomicU32::new(0)),
    )
    .expect("sidecar turn constructs");
    turn.fail_connection(SafeDiagnostic::new("fixture.done", "finished"));
    let attachment_service = Arc::new(TrackingAttachmentService {
        releases: AtomicUsize::new(0),
    });
    let lease = AttachmentFileLease::operation_scoped(
        ScopeId::new("pi-sdk-sidecar:reap-test").expect("scope id"),
        AttachmentRef::new("att-sidecar-reap-1").expect("attachment ref"),
        MaterializedFileRef::new("/fixture/sidecar-reap-test.png").expect("file ref"),
    );
    let slot: ActiveSlot = Arc::new(Mutex::new(Some(ActiveTask {
        turn,
        deadline_task: Some(Box::new(FailingJoin)),
        attachment: SharedAttachmentMaterialization::new(
            Arc::clone(&attachment_service) as Arc<dyn AttachmentService>,
            lease,
        ),
    })));

    let error = futures_executor::block_on(reap_finished(&slot))
        .expect_err("sidecar deadline join failure must be returned");
    assert_eq!(error.diagnostic().code(), "fixture.join_failed");
    assert_eq!(
        attachment_service.releases.load(Ordering::SeqCst),
        1,
        "sidecar reap_finished must release the attachment lease even when deadline_task.join() fails"
    );
}

#[test]
fn sidecar_long_caller_ids_produce_bounded_wire_command_ids() {
    use super::{PiSdkSidecarSessionHandle, spawn_deadline};
    use crate::sidecar::connection::SidecarConnection;
    use crate::sidecar::driver::handle::SessionCancellation;
    use crate::sidecar::turn::SidecarActiveTurn;
    use crate::sidecar::wire::{MAXIMUM_COMMAND_ID_BYTES, PiSdkSidecarRecord, decode_record};
    use std::collections::VecDeque;
    use std::sync::atomic::AtomicU32;
    use std::sync::{Arc, Condvar, Mutex};
    use swallowtail_core::{
        ConfiguredInstanceId, ExecutionHostId, HarnessMessageClass, ModelId, ModelRouteId,
        SessionAccessPolicy, SessionRef,
    };
    use swallowtail_runtime::{
        Deadline, DeadlineObservation, HarnessCommandAcknowledgement, HarnessCommandId,
        HarnessScheduledMessage, HostServices, InteractiveSessionHandle, MonotonicInstant,
        OperationContent, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
        ProcessOutputStream, RequestId, RuntimeSessionId, RuntimeTurnId, ScopeId,
        ScopedTaskService, SessionCleanupRequest, SessionResumeBinding, TimeService, TurnRequest,
        WorkingResourceRef,
    };

    struct EchoingWireProcess {
        state: Mutex<(Vec<serde_json::Value>, VecDeque<Vec<u8>>, bool)>,
        changed: Condvar,
    }
    impl EchoingWireProcess {
        fn new() -> Self {
            Self {
                state: Mutex::new((Vec::new(), VecDeque::new(), false)),
                changed: Condvar::new(),
            }
        }
        fn recorded_ids(&self) -> Vec<(String, String)> {
            self.state
                .lock()
                .expect("process lock")
                .0
                .iter()
                .map(|value| {
                    (
                        value["command"].as_str().expect("command").to_owned(),
                        value["id"].as_str().expect("id").to_owned(),
                    )
                })
                .collect()
        }
    }
    impl ProcessHandle for EchoingWireProcess {
        fn write_stdin(
            &self,
            chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            let parsed: serde_json::Value =
                serde_json::from_slice(chunk.bytes()).expect("outbound command is valid JSON");
            let id = parsed["id"].as_str().expect("id").to_owned();
            let command = parsed["command"].as_str().expect("command").to_owned();
            let mut reply = serde_json::to_vec(&serde_json::json!({
                "type": "response",
                "id": id,
                "command": command,
                "success": true,
            }))
            .expect("serialize reply");
            // Verify the wire decoder itself accepts the echoed response ID.
            assert!(
                matches!(decode_record(&reply), Ok(PiSdkSidecarRecord::Response(_))),
                "wire response for command {command:?} with id {id:?} (len {}) must decode within MAXIMUM_COMMAND_ID_BYTES",
                id.len()
            );
            reply.push(b'\n');
            let mut state = self.state.lock().expect("process lock");
            state.0.push(parsed);
            state.1.push_back(reply);
            self.changed.notify_all();
            Box::pin(async { Ok(()) })
        }
        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            let mut state = self.state.lock().expect("process lock");
            state.2 = true;
            self.changed.notify_all();
            Box::pin(async { Ok(()) })
        }
        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            let mut state = self.state.lock().expect("process lock");
            loop {
                if let Some(line) = state.1.pop_front() {
                    return Box::pin(async move {
                        Ok(Some(ProcessOutputChunk::new(
                            ProcessOutputStream::Stdout,
                            line,
                        )))
                    });
                }
                if state.2 {
                    return Box::pin(async { Ok(None) });
                }
                state = self.changed.wait(state).expect("process lock");
            }
        }
        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }
        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            let mut state = self.state.lock().expect("process lock");
            state.2 = true;
            self.changed.notify_all();
            Box::pin(async { Ok(()) })
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    struct PendingClock;
    impl TimeService for PendingClock {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(0)
        }
        fn wait_until(&self, _: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            Box::pin(std::future::pending())
        }
    }

    struct ReadyClock;
    impl TimeService for ReadyClock {
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

    struct ThreadTaskService;
    struct ThreadJoinedTask(Option<std::thread::JoinHandle<()>>);
    impl JoinedTask for ThreadJoinedTask {
        fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let handle = self.0.take();
            Box::pin(async move {
                if let Some(handle) = handle {
                    let _ = handle.join();
                }
                Ok(())
            })
        }
    }
    impl ScopedTaskService for ThreadTaskService {
        fn spawn(
            &self,
            _scope: ScopeId,
            task: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            let handle = std::thread::spawn(move || futures_executor::block_on(task));
            Ok(Box::new(ThreadJoinedTask(Some(handle))))
        }
    }

    let long_request_id = "r".repeat(160);
    let long_turn_id = "t".repeat(160);
    let long_steer_id = "s".repeat(160);
    let long_follow_up_id = "f".repeat(160);

    let host = ExecutionHostId::new("pi.sidecar.long-ids").expect("host id");
    let process = Arc::new(EchoingWireProcess::new());
    let services = HostServices::new(host.clone())
        .with_task(Arc::new(ThreadTaskService))
        .with_time(Arc::new(PendingClock));
    let connection = SidecarConnection::new(
        Arc::clone(&process) as Arc<dyn ProcessHandle>,
        services.clone(),
    );
    let pump_conn = Arc::clone(&connection);
    let pump_task = Box::new(ThreadJoinedTask(Some(std::thread::spawn(move || {
        futures_executor::block_on(pump_conn.pump());
    }))));

    let active: super::ActiveSlot = Arc::new(Mutex::new(None));
    let binding = SessionResumeBinding::new(
        SessionRef::new("sidecar-long-ids").expect("session ref"),
        ConfiguredInstanceId::new("pi.sidecar.inst").expect("instance id"),
        host.clone(),
        ModelRouteId::new("pi.route").expect("route id"),
        ModelId::new("pi-model").expect("model id"),
        WorkingResourceRef::new("workspace.main").expect("resource"),
        SessionAccessPolicy::read_only(),
    );
    let mut session = PiSdkSidecarSessionHandle {
        request_id: RequestId::new(long_request_id).expect("long request id"),
        runtime_id: RuntimeSessionId::new("pi-sdk-sidecar:long-ids").expect("session id"),
        execution_host_id: host.clone(),
        binding,
        connection: Arc::clone(&connection),
        cancellation: SessionCancellation::new(Arc::clone(&connection), Arc::clone(&active)),
        pump_task: Some(pump_task),
        services: services.clone(),
        resource: None,
        credential: None,
        active,
        completed_prompts: Arc::new(AtomicU32::new(0)),
        image_attachments: false,
    };

    // 1. Prompt, steer, follow_up, abort, and close with >128-byte caller IDs.
    let turn_req = TurnRequest::new(
        RuntimeTurnId::new(long_turn_id.clone()).expect("long turn id"),
        OperationContent::new("hello").expect("content"),
    )
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(100)));
    let mut turn = futures_executor::block_on(session.start_turn(turn_req, services.clone()))
        .expect("prompt with >128-byte turn id succeeds");

    let steer_cmd_id = HarnessCommandId::new(long_steer_id).expect("long steer id");
    let steer_resp =
        futures_executor::block_on(turn.schedule_harness_message(HarnessScheduledMessage::new(
            steer_cmd_id.clone(),
            HarnessMessageClass::Steering,
            OperationContent::new("steer").expect("content"),
        )))
        .expect("steer with >128-byte command id succeeds");
    assert_eq!(steer_resp.command_id(), &steer_cmd_id);
    assert_eq!(
        steer_resp.acknowledgement(),
        HarnessCommandAcknowledgement::Accepted
    );

    let follow_cmd_id = HarnessCommandId::new(long_follow_up_id).expect("long follow-up id");
    let follow_resp =
        futures_executor::block_on(turn.schedule_harness_message(HarnessScheduledMessage::new(
            follow_cmd_id.clone(),
            HarnessMessageClass::FollowUp,
            OperationContent::new("follow").expect("content"),
        )))
        .expect("follow_up with >128-byte command id succeeds");
    assert_eq!(follow_resp.command_id(), &follow_cmd_id);
    assert_eq!(
        follow_resp.acknowledgement(),
        HarnessCommandAcknowledgement::Accepted
    );

    futures_executor::block_on(turn.cancellation().request())
        .expect("abort with >128-byte turn id succeeds");
    drop(turn);

    let cleanup = futures_executor::block_on(Box::new(session).close(
        SessionCleanupRequest::new(Deadline::at(MonotonicInstant::from_ticks(200))),
        services,
    ));
    assert_eq!(cleanup, CleanupOutcome::Clean);

    // 2. Also exercise `deadline-abort` with a >128-byte turn ID.
    let deadline_process = Arc::new(EchoingWireProcess::new());
    let deadline_services = HostServices::new(host)
        .with_task(Arc::new(ThreadTaskService))
        .with_time(Arc::new(ReadyClock));
    let deadline_conn = SidecarConnection::new(
        Arc::clone(&deadline_process) as Arc<dyn ProcessHandle>,
        deadline_services.clone(),
    );
    let pump_deadline_conn = Arc::clone(&deadline_conn);
    let pump_deadline = std::thread::spawn(move || {
        futures_executor::block_on(pump_deadline_conn.pump());
    });
    let (deadline_turn, _events, _terminal) = SidecarActiveTurn::new(
        RuntimeTurnId::new(long_turn_id).expect("long turn id"),
        Arc::new(AtomicU32::new(0)),
    )
    .expect("turn constructs");
    deadline_conn
        .set_active_turn(Arc::clone(&deadline_turn))
        .expect("turn attaches");
    let (arm_tx, arm_rx) = futures_channel::oneshot::channel();
    let _ = arm_tx.send(());
    let deadline_task = spawn_deadline(
        &deadline_services,
        Arc::clone(&deadline_conn),
        deadline_turn,
        Deadline::at(MonotonicInstant::from_ticks(1)),
        arm_rx,
    )
    .expect("deadline task spawns");
    futures_executor::block_on(deadline_task.join()).expect("deadline task joins");
    futures_executor::block_on(deadline_conn.begin_close());
    pump_deadline.join().expect("deadline pump joins");

    let mut all_ids = process.recorded_ids();
    all_ids.extend(deadline_process.recorded_ids());
    for command in ["prompt", "steer", "follow_up", "abort", "close"] {
        assert!(
            all_ids.iter().any(|(cmd, _)| cmd == command),
            "expected command {command:?} to be recorded, got {all_ids:?}"
        );
    }
    assert!(
        deadline_process
            .recorded_ids()
            .iter()
            .any(|(cmd, _)| cmd == "abort"),
        "expected deadline-abort command to be recorded"
    );
    for (command, wire_id) in &all_ids {
        assert!(
            !wire_id.is_empty() && wire_id.len() <= MAXIMUM_COMMAND_ID_BYTES,
            "outbound command {command:?} wire id {wire_id:?} exceeded MAXIMUM_COMMAND_ID_BYTES ({})",
            wire_id.len()
        );
    }
}
