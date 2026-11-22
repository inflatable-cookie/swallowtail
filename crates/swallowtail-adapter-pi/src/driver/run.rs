use super::session::PiSessionHandle;
use super::validation::validate_run;
use super::{PiRpcDriver, failure};
use crate::connection::PiConnection;
use crate::driver::{ABORT_ACKNOWLEDGEMENT_GRACE_TICKS, grace_deadline, run_before_deadline};
use crate::turn::ActiveTurn;
use serde_json::json;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, HarnessConfigurationPosture, PreflightPlan, ResourceAccess, RunRef,
    SafeDiagnostic, SessionAccessPolicy, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackExchange, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, Deadline, HostServices, InteractiveSessionHandle, JoinedTask,
    OpenSessionRequest, RequestId, RunHandle, RuntimeFailure, RuntimeRunId, RuntimeTurnId,
    SessionPlanAgreement, StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TurnHandle,
    TurnRequest, terminal_outcome_channel,
};

impl StructuredRunDriver for PiRpcDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let version_features = validate_run(&plan, &request, &services, &self.credential)?;
            let run_id = RuntimeRunId::new(format!("pi-rpc:run:{}", request.request_id().as_str()))
                .map_err(|_| malformed_run_id())?;
            let turn_id =
                RuntimeTurnId::new(format!("pi-rpc:run:{}", request.request_id().as_str()))
                    .map_err(|_| malformed_run_id())?;
            let task_scope = swallowtail_runtime::ScopeId::new(format!(
                "pi-rpc:run-cleanup:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed_run_id())?;
            let open_request = OpenSessionRequest::new(
                request.request_id().clone(),
                request
                    .working_resource()
                    .expect("validated Pi working resource")
                    .clone(),
                request.deadline(),
                SessionPlanAgreement::explicit(
                    SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                    Some(SessionProviderStatePolicy::Prohibited),
                    Some(HarnessConfigurationPosture::ProviderSuppressed),
                ),
            );
            let mut session = self
                .start_session(plan, open_request, services.clone(), version_features)
                .await?;
            let turn_request = TurnRequest::new(turn_id, request.content().clone())
                .with_deadline(request.deadline().expect("validated Pi run deadline"))
                .with_attachments(request.attachments().cloned());
            let mut turn = match session.start_turn(turn_request, services.clone()).await {
                Ok(turn) => turn,
                Err(error) => {
                    let _ = session.close_run().await;
                    return Err(error);
                }
            };
            let events = match turn.take_events() {
                Some(events) => events,
                None => {
                    drop(turn);
                    let _ = session.close_run().await;
                    return Err(failure(
                        "swallowtail.pi.rpc.run_events_missing",
                        "Pi RPC structured run did not expose its event stream",
                    ));
                }
            };
            let callbacks = turn.take_callbacks();
            let turn_terminal = match turn.take_terminal_outcome() {
                Some(terminal) => terminal,
                None => {
                    drop(turn);
                    let _ = session.close_run().await;
                    return Err(failure(
                        "swallowtail.pi.rpc.run_terminal_missing",
                        "Pi RPC structured run did not expose its terminal outcome",
                    ));
                }
            };
            let active = {
                let slot = session.active.lock().expect("Pi active-task lock poisoned");
                slot.as_ref().map(|active| Arc::clone(&active.turn))
            };
            let active = match active {
                Some(active) => active,
                None => {
                    drop(turn);
                    let _ = session.close_run().await;
                    return Err(failure(
                        "swallowtail.pi.rpc.run_active_missing",
                        "Pi RPC structured run lost its active prompt",
                    ));
                }
            };
            let cancellation = Arc::new(PiRunCancellation {
                connection: Arc::clone(&session.connection),
                turn: active,
                requested: AtomicBool::new(false),
                attempts: AtomicU64::new(0),
            });
            let pending = Arc::new(Mutex::new(Some(PiRunResources {
                turn,
                session,
                terminal: turn_terminal,
            })));
            let (terminal_sender, terminal) = terminal_outcome_channel();
            let run_cleanup = Arc::new(Mutex::new(None));
            let task_cleanup = Arc::clone(&run_cleanup);
            let task_pending = Arc::clone(&pending);
            let task = services.task().expect("validated Pi task service").spawn(
                task_scope,
                Box::pin(async move {
                    let resources = task_pending
                        .lock()
                        .expect("Pi pending run lock poisoned")
                        .take()
                        .expect("Pi pending run exists");
                    let outcome = resources.terminal.await;
                    drop(resources.turn);
                    let session_cleanup = resources.session.close_run().await;
                    let cleanup = CleanupOutcome::merge(outcome.cleanup().clone(), session_cleanup);
                    *task_cleanup.lock().expect("Pi run cleanup lock poisoned") =
                        Some(cleanup.clone());
                    let finished = copy_outcome_with_cleanup(outcome, cleanup);
                    let _ = terminal_sender.complete(finished);
                }),
            );
            let task = match task {
                Ok(task) => task,
                Err(error) => {
                    let resources = pending.lock().expect("Pi pending run lock poisoned").take();
                    if let Some(resources) = resources {
                        drop(resources.turn);
                        let _ = resources.session.close_run().await;
                    }
                    return Err(error);
                }
            };
            Ok(Box::new(PiRunHandle {
                request_id: request.request_id().clone(),
                run_id,
                events: Some(events),
                callbacks,
                terminal: Some(Box::pin(terminal)),
                cancellation,
                task,
                cleanup: run_cleanup,
            }) as Box<dyn RunHandle>)
        })
    }
}

struct PiRunResources {
    turn: Box<dyn TurnHandle>,
    session: PiSessionHandle,
    terminal: BoxFuture<'static, TerminalOutcome>,
}

struct PiRunCancellation {
    connection: Arc<PiConnection>,
    turn: Arc<ActiveTurn>,
    requested: AtomicBool,
    /// Counts abort attempts so a retry after a refused abort carries a fresh
    /// command id; the connection rejects reused ids.
    attempts: AtomicU64,
}

impl PiRunCancellation {
    fn request_with_deadline(
        &self,
        deadline: Option<Deadline>,
    ) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst) || self.turn.is_finished();
        Box::pin(async move {
            if already {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            self.turn.mark_abort_in_flight();
            let attempt = self.attempts.fetch_add(1, Ordering::SeqCst);
            let id = if attempt == 0 {
                format!("abort:{}", self.turn.runtime_id().as_str())
            } else {
                format!("abort:{}:{attempt}", self.turn.runtime_id().as_str())
            };
            let abort = self.connection.abort_command(
                id.clone(),
                json!({"id": id, "type": "abort"}),
                Arc::clone(&self.turn),
            );
            let response = run_before_deadline(
                abort,
                deadline,
                self.connection.services(),
                "swallowtail.pi.rpc.close_abort_timed_out",
                "Pi RPC close timed out awaiting abort acknowledgement",
            )
            .await;
            match response {
                Ok(response) if response.success => {
                    self.turn.mark_abort_acknowledged();
                    Ok(CancellationAcknowledgement::Requested)
                }
                Ok(_) => {
                    // A refused abort is not acknowledged, so a later request
                    // (including the close escalation) may try again.
                    self.requested.store(false, Ordering::SeqCst);
                    Err(failure(
                        "swallowtail.pi.rpc.abort_rejected",
                        "Pi RPC rejected native abort",
                    ))
                }
                Err(error) => {
                    let _ = self.turn.reconcile_abort_response(false);
                    self.requested.store(false, Ordering::SeqCst);
                    Err(error)
                }
            }
        })
    }
}

impl CancellationControl for PiRunCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        self.request_with_deadline(None)
    }
}

struct PiRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    callbacks: Option<CallbackExchange>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<PiRunCancellation>,
    task: Box<dyn JoinedTask>,
    cleanup: Arc<Mutex<Option<CleanupOutcome>>>,
}

impl RunHandle for PiRunHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn run_id(&self) -> &RuntimeRunId {
        &self.run_id
    }

    fn provider_run_ref(&self) -> Option<&RunRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        self.callbacks.take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let cancellation = match self
                .cancellation
                .request_with_deadline(grace_deadline(
                    self.cancellation.connection.services(),
                    ABORT_ACKNOWLEDGEMENT_GRACE_TICKS,
                ))
                .await
            {
                Ok(_) => CleanupOutcome::NotApplicable,
                Err(error) => {
                    // Any rejected or failed native abort leaves the run live, so
                    // the join below would wait for provider completion. Escalate
                    // to a forced stop so the RPC process ends and the join can settle.
                    let _ = self.cancellation.connection.force_stop().await;
                    CleanupOutcome::Failed(error.diagnostic().clone())
                }
            };
            let join = match self.task.join().await {
                Ok(()) => self
                    .cleanup
                    .lock()
                    .expect("Pi run cleanup lock poisoned")
                    .take()
                    .unwrap_or(CleanupOutcome::Clean),
                Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.pi.rpc.run_join_failed",
                    "Pi RPC structured-run cleanup task did not join",
                )),
            };
            CleanupOutcome::merge(cancellation, join)
        })
    }
}

fn copy_outcome_with_cleanup(outcome: TerminalOutcome, cleanup: CleanupOutcome) -> TerminalOutcome {
    let mut finished = TerminalOutcome::new(outcome.status().clone(), cleanup);
    if let Some(output) = outcome.output().cloned() {
        finished = finished.with_output(output);
    }
    if let Some(cancellation) = outcome.provider_cancellation() {
        finished = finished.with_provider_cancellation(cancellation);
    }
    for (resource, deletion) in outcome.remote_resource_deletions() {
        finished = finished.with_remote_resource_deletion(resource, deletion);
    }
    finished
}

fn malformed_run_id() -> RuntimeFailure {
    failure(
        "swallowtail.pi.rpc.run_id_invalid",
        "Pi RPC structured-run identity was invalid",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::close_tests::{AbortGraceExpires, SilentAbort};
    use futures_util::FutureExt;
    use std::sync::atomic::{AtomicU32, AtomicUsize};
    use swallowtail_core::ExecutionHostId;
    use swallowtail_runtime::{
        Deadline, DeadlineObservation, MonotonicInstant, ProcessExit, ProcessHandle,
        ProcessInputChunk, ProcessOutputChunk, ScopeId, ScopedTaskService, TerminalStatus,
        TimeService,
    };

    struct RefusingStdinProcess {
        force_stops: AtomicUsize,
        stopped: Mutex<Option<futures_channel::oneshot::Sender<()>>>,
    }

    impl ProcessHandle for RefusingStdinProcess {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "test.abort_write_failed",
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
            if let Some(tx) = self.stopped.lock().expect("stopped lock").take() {
                let _ = tx.send(());
            }
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    struct DummyTaskService;

    impl ScopedTaskService for DummyTaskService {
        fn spawn(
            &self,
            _scope: ScopeId,
            _task: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "test.unused",
                "unused",
            )))
        }
    }

    struct DummyTimeService;

    impl TimeService for DummyTimeService {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(0)
        }

        fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            Box::pin(std::future::pending())
        }
    }

    struct WaitForStopTask {
        stopped: futures_channel::oneshot::Receiver<()>,
    }

    impl JoinedTask for WaitForStopTask {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let stopped = self.stopped;
            Box::pin(async move {
                stopped.await.map_err(|_| {
                    RuntimeFailure::new(SafeDiagnostic::new(
                        "test.not_force_stopped",
                        "force_stop was not called",
                    ))
                })
            })
        }
    }

    #[test]
    fn silent_abort_during_run_close_expires_before_force_stop_and_join() {
        struct JoinAfterForceStop(Arc<SilentAbort>);

        impl JoinedTask for JoinAfterForceStop {
            fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
                assert!(
                    self.0.force_stopped.load(Ordering::SeqCst),
                    "force stop must precede join"
                );
                Box::pin(async { Ok(()) })
            }
        }

        let process = Arc::new(SilentAbort::default());
        let clock = Arc::new(AbortGraceExpires);
        let services =
            HostServices::new(ExecutionHostId::new("pi.rpc.run-close-test").expect("host id"))
                .with_time(clock.clone());
        let connection =
            PiConnection::new(process.clone(), Arc::new(DummyTaskService), clock, services);
        let (turn, events, callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("pi-run-silent-abort").expect("turn id"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn constructs");
        let handle = Box::new(PiRunHandle {
            request_id: RequestId::new("pi-run-silent-abort-req").expect("request id"),
            run_id: RuntimeRunId::new("pi-run-silent-abort-run").expect("run id"),
            events: Some(events),
            callbacks: Some(callbacks),
            terminal: Some(Box::pin(terminal)),
            cancellation: Arc::new(PiRunCancellation {
                connection,
                turn,
                requested: AtomicBool::new(false),
                attempts: AtomicU64::new(0),
            }),
            task: Box::new(JoinAfterForceStop(process.clone())),
            cleanup: Arc::new(Mutex::new(None)),
        });
        let cleanup = handle
            .close()
            .now_or_never()
            .expect("close settles after grace expires");
        assert_eq!(
            cleanup.diagnostic().map(SafeDiagnostic::code),
            Some("swallowtail.pi.rpc.close_abort_timed_out")
        );
        assert_eq!(process.writes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn abort_write_error_does_not_mark_cancelled_and_close_escalates_to_force_stop() {
        let (stopped_tx, stopped_rx) = futures_channel::oneshot::channel();
        let process = Arc::new(RefusingStdinProcess {
            force_stops: AtomicUsize::new(0),
            stopped: Mutex::new(Some(stopped_tx)),
        });
        let services =
            HostServices::new(ExecutionHostId::new("pi.rpc.run-cancel-test").expect("host id"));
        let connection = PiConnection::new(
            Arc::clone(&process) as Arc<dyn ProcessHandle>,
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services,
        );
        let (turn, events, callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("pi-run-abort-err").expect("turn id"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn constructs");
        let cancellation = Arc::new(PiRunCancellation {
            connection: Arc::clone(&connection),
            turn: Arc::clone(&turn),
            requested: AtomicBool::new(false),
            attempts: AtomicU64::new(0),
        });
        let mut handle = Box::new(PiRunHandle {
            request_id: RequestId::new("pi-run-abort-err-req").expect("request id"),
            run_id: RuntimeRunId::new("pi-run-abort-err-run").expect("run id"),
            events: Some(events),
            callbacks: Some(callbacks),
            terminal: Some(Box::pin(terminal)),
            cancellation,
            task: Box::new(WaitForStopTask {
                stopped: stopped_rx,
            }),
            cleanup: Arc::new(Mutex::new(None)),
        });
        let terminal_fut = handle
            .take_terminal_outcome()
            .expect("terminal outcome future is present");

        // 1. A failed abort write must return the error and must NOT mark the
        // turn as cancelled; subsequent connection EOF resolves cancel_rejected.
        let abort_err = futures_executor::block_on(handle.cancellation().request())
            .expect_err("failed abort write returns error");
        assert_eq!(abort_err.diagnostic().code(), "test.abort_write_failed");

        // 2. Closing the run handle after a transport/write abort error must
        // escalate to force_stop() so the task joins instead of hanging.
        let cleanup = futures_executor::block_on(handle.close());
        assert_eq!(process.force_stops.load(Ordering::SeqCst), 1);
        assert!(
            matches!(cleanup, CleanupOutcome::Failed(ref diag) if diag.code() == "test.abort_write_failed"),
            "expected abort_write_failed cleanup failure, got {cleanup:?}"
        );

        // 3. When the turn settles via connection close after the rejected
        // abort, it must NOT misreport TerminalStatus::Cancelled.
        turn.fail_connection(SafeDiagnostic::new("test.eof", "eof"));
        let outcome = futures_executor::block_on(terminal_fut);
        assert!(
            matches!(
                outcome.status(),
                TerminalStatus::RuntimeFailed(diag) if diag.code() == "swallowtail.pi.rpc.cancel_rejected"
            ),
            "rejected abort must not misreport TerminalStatus::Cancelled, got {:?}",
            outcome.status()
        );
    }

    #[test]
    fn negative_abort_reply_followed_by_acknowledged_retry_and_eof_resolves_cancelled() {
        use std::collections::VecDeque;
        use swallowtail_runtime::ProcessOutputStream;

        struct ScriptedRetryProcess {
            writes: AtomicUsize,
            outputs: Mutex<VecDeque<futures_channel::oneshot::Receiver<Option<Vec<u8>>>>>,
            senders: Mutex<VecDeque<futures_channel::oneshot::Sender<Option<Vec<u8>>>>>,
        }

        impl ScriptedRetryProcess {
            fn new() -> Self {
                let mut outputs = VecDeque::new();
                let mut senders = VecDeque::new();
                for _ in 0..3 {
                    let (tx, rx) = futures_channel::oneshot::channel();
                    outputs.push_back(rx);
                    senders.push_back(tx);
                }
                Self {
                    writes: AtomicUsize::new(0),
                    outputs: Mutex::new(outputs),
                    senders: Mutex::new(senders),
                }
            }

            fn send_eof(&self) {
                if let Some(tx) = self.senders.lock().expect("senders lock").pop_front() {
                    let _ = tx.send(None);
                }
            }
        }

        impl ProcessHandle for ScriptedRetryProcess {
            fn write_stdin(
                &self,
                chunk: ProcessInputChunk,
            ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                let parsed: serde_json::Value =
                    serde_json::from_slice(chunk.bytes()).expect("valid JSON command");
                let id = parsed["id"].as_str().expect("id").to_owned();
                let attempt = self.writes.fetch_add(1, Ordering::SeqCst);
                let success = attempt >= 1;
                let mut reply = serde_json::to_vec(&json!({
                    "type": "response",
                    "id": id,
                    "command": "abort",
                    "success": success,
                }))
                .expect("serialize reply");
                reply.push(b'\n');
                if let Some(tx) = self.senders.lock().expect("senders lock").pop_front() {
                    let _ = tx.send(Some(reply));
                }
                Box::pin(async { Ok(()) })
            }

            fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }

            fn read_output(
                &self,
            ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
                let rx = self.outputs.lock().expect("outputs lock").pop_front();
                Box::pin(async move {
                    match rx {
                        Some(rx) => match rx.await.ok().flatten() {
                            Some(bytes) => Ok(Some(ProcessOutputChunk::new(
                                ProcessOutputStream::Stdout,
                                bytes,
                            ))),
                            None => Ok(None),
                        },
                        None => Ok(None),
                    }
                })
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

        let process = Arc::new(ScriptedRetryProcess::new());
        let services =
            HostServices::new(ExecutionHostId::new("pi.rpc.abort-retry-eof").expect("host id"));
        let connection = PiConnection::new(
            Arc::clone(&process) as Arc<dyn ProcessHandle>,
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services,
        );
        let (turn, _events, _callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("pi-run-retry-eof").expect("turn id"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn constructs");
        connection
            .set_active_turn(Arc::clone(&turn))
            .expect("turn attaches");
        let cancellation = Arc::new(PiRunCancellation {
            connection: Arc::clone(&connection),
            turn: Arc::clone(&turn),
            requested: AtomicBool::new(false),
            attempts: AtomicU64::new(0),
        });

        let pump_conn = Arc::clone(&connection);
        let pump_thread = std::thread::spawn(move || {
            futures_executor::block_on(pump_conn.pump());
        });

        // 1. First abort receives a negative reply -> Err(abort_rejected).
        let first = futures_executor::block_on(cancellation.request())
            .expect_err("first abort is rejected");
        assert_eq!(
            first.diagnostic().code(),
            "swallowtail.pi.rpc.abort_rejected"
        );

        // 2. Retry succeeds -> Ok(Requested) and supersedes the earlier refusal.
        let retry = futures_executor::block_on(cancellation.request())
            .expect("retry abort is acknowledged");
        assert_eq!(retry, CancellationAcknowledgement::Requested);

        // 3. EOF arrives before any agent_end event -> TerminalStatus::Cancelled.
        process.send_eof();
        pump_thread.join().expect("pump thread joins");

        let outcome = futures_executor::block_on(terminal);
        assert_eq!(
            outcome.status(),
            &TerminalStatus::Cancelled,
            "acknowledged retry followed by EOF must resolve TerminalStatus::Cancelled"
        );
    }

    #[test]
    fn run_close_surfaces_session_cleanup_failure() {
        use crate::driver::handle::SessionCancellation;
        use crate::selection::PiRpcVersionFeatures;
        use swallowtail_runtime::RuntimeSessionId;

        struct QuietProcess;
        impl ProcessHandle for QuietProcess {
            fn write_stdin(
                &self,
                _: ProcessInputChunk,
            ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn read_output(
                &self,
            ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
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

        struct FailingPumpTask;
        impl JoinedTask for FailingPumpTask {
            fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
                Box::pin(async {
                    Err(RuntimeFailure::new(SafeDiagnostic::new(
                        "test.pump_join_failed",
                        "pump task failed to join",
                    )))
                })
            }
        }

        struct InlineJoinedTask(Option<BoxFuture<'static, ()>>);
        impl JoinedTask for InlineJoinedTask {
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

        let host = ExecutionHostId::new("pi.rpc.run-cleanup-fail").expect("host id");
        let services = HostServices::new(host.clone()).with_time(Arc::new(DummyTimeService));
        let connection = PiConnection::new(
            Arc::new(QuietProcess),
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services.clone(),
        );
        let active = Arc::new(Mutex::new(None));
        let session = PiSessionHandle {
            request_id: RequestId::new("pi-run-cleanup-req").expect("request id"),
            runtime_id: RuntimeSessionId::new("pi-rpc:run-cleanup").expect("session id"),
            execution_host_id: host,
            connection: Arc::clone(&connection),
            cancellation: SessionCancellation::new(Arc::clone(&connection), Arc::clone(&active)),
            pump_task: Some(Box::new(FailingPumpTask)),
            services: services.clone(),
            resource: None,
            credential: None,
            active,
            completed_prompts: Arc::new(AtomicU32::new(0)),
            version_features: PiRpcVersionFeatures {
                command_dispositions: false,
                aborted_settled: false,
            },
            image_attachments: false,
        };
        let (turn, events, callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("pi-run-cleanup-turn").expect("turn id"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn constructs");
        turn.fail_connection(SafeDiagnostic::new("test.done", "already finished"));

        let run_cleanup = Arc::new(Mutex::new(None));
        let task_cleanup = Arc::clone(&run_cleanup);
        let (terminal_sender, run_terminal) = terminal_outcome_channel();
        let cleanup_task = Box::new(InlineJoinedTask(Some(Box::pin(async move {
            let outcome = terminal.await;
            let session_cleanup = session.close_run().await;
            let cleanup = CleanupOutcome::merge(outcome.cleanup().clone(), session_cleanup);
            *task_cleanup.lock().expect("Pi run cleanup lock poisoned") = Some(cleanup.clone());
            let finished = copy_outcome_with_cleanup(outcome, cleanup);
            let _ = terminal_sender.complete(finished);
        }))));

        let handle = Box::new(PiRunHandle {
            request_id: RequestId::new("pi-run-cleanup-req").expect("request id"),
            run_id: RuntimeRunId::new("pi-run-cleanup-run").expect("run id"),
            events: Some(events),
            callbacks: Some(callbacks),
            terminal: Some(Box::pin(run_terminal)),
            cancellation: Arc::new(PiRunCancellation {
                connection,
                turn,
                requested: AtomicBool::new(false),
                attempts: AtomicU64::new(0),
            }),
            task: cleanup_task,
            cleanup: run_cleanup,
        });

        let cleanup = futures_executor::block_on(handle.close());
        assert!(
            matches!(
                cleanup,
                CleanupOutcome::Failed(ref diag)
                    if diag.code() == "swallowtail.pi.rpc.cleanup_failed"
                        && diag.message().contains("pump_join_failed")
            ),
            "PiRunHandle::close must surface session.close_run() failure instead of Clean, got {cleanup:?}"
        );
    }
}
