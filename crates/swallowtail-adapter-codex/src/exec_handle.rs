use crate::process_close::{AppServerTask, CloseBudget};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{CancellationScope, RunRef, SafeDiagnostic};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    ProcessHandle, RequestId, RunHandle, RuntimeFailure, RuntimeRunId, TerminalOutcome,
};

pub(crate) struct ProcessCancellation {
    process: Arc<dyn ProcessHandle>,
    requested: AtomicBool,
    finished: AtomicBool,
    pump_aborted: AtomicBool,
    pump_waker: Mutex<Option<Waker>>,
}

impl ProcessCancellation {
    pub(crate) fn new(process: Arc<dyn ProcessHandle>) -> Self {
        Self {
            process,
            requested: AtomicBool::new(false),
            finished: AtomicBool::new(false),
            pump_aborted: AtomicBool::new(false),
            pump_waker: Mutex::new(None),
        }
    }

    pub(crate) fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }

    /// Records that the pump has produced its terminal outcome, so a later
    /// close does not cancel a run that already finished on its own.
    pub(crate) fn mark_finished(&self) {
        self.finished.store(true, Ordering::SeqCst);
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }

    /// Signals a stuck exec pump loop to exit cleanly after a refused
    /// `force_stop()` so `close()` can always join the pump task.
    pub(crate) fn abort_pump(&self) {
        self.pump_aborted.store(true, Ordering::SeqCst);
        if let Some(waker) = self
            .pump_waker
            .lock()
            .expect("exec pump-abort lock poisoned")
            .take()
        {
            waker.wake();
        }
    }

    pub(crate) fn poll_pump_aborted(&self, context: &mut Context<'_>) -> Poll<()> {
        if self.pump_aborted.load(Ordering::SeqCst) {
            return Poll::Ready(());
        }
        *self
            .pump_waker
            .lock()
            .expect("exec pump-abort lock poisoned") = Some(context.waker().clone());
        if self.pump_aborted.load(Ordering::SeqCst) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl CancellationControl for ProcessCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        Box::pin(async move {
            if self.requested.swap(true, Ordering::SeqCst) {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            if let Err(error) = self.process.force_stop().await {
                // A refused stop leaves nothing requested, so a retry may try again.
                self.requested.store(false, Ordering::SeqCst);
                return Err(error);
            }
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

pub(crate) struct CodexExecRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<ProcessCancellation>,
    task: AppServerTask,
    close_budget: CloseBudget,
}

impl CodexExecRunHandle {
    pub(crate) fn new(
        request_id: RequestId,
        run_id: RuntimeRunId,
        events: BoxEventStream,
        terminal: BoxFuture<'static, TerminalOutcome>,
        cancellation: Arc<ProcessCancellation>,
        task: AppServerTask,
        close_budget: CloseBudget,
    ) -> Self {
        Self {
            request_id,
            run_id,
            events: Some(events),
            terminal: Some(terminal),
            cancellation,
            task,
            close_budget,
        }
    }
}

impl RunHandle for CodexExecRunHandle {
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

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if self.cancellation.is_finished() {
                return match self.task.join().await {
                    Ok(()) => CleanupOutcome::Clean,
                    Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                        "swallowtail.codex.exec.task_join_failed",
                        "Codex exec operation task could not be joined",
                    )),
                };
            }
            let cancellation = Arc::clone(&self.cancellation);
            let stop_future = async move { cancellation.request().await.map(|_| ()) };
            let abort_handle = Arc::clone(&self.cancellation);
            let outcome = self
                .task
                .close_bounded(self.close_budget, Some(stop_future), move || {
                    abort_handle.abort_pump();
                })
                .await;
            let stop_cleanup = match outcome.stop {
                Some(Ok(())) if !outcome.aborted => CleanupOutcome::Clean,
                Some(Ok(())) => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.codex.exec.process_cleanup_failed",
                    "Codex exec process cleanup failed",
                )),
                Some(Err(error)) => CleanupOutcome::Failed(error.diagnostic().clone()),
                None => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.codex.exec.stop_timed_out",
                    "Codex exec process stop timed out",
                )),
            };
            let join_cleanup = match outcome.join {
                Ok(()) => CleanupOutcome::Clean,
                Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.codex.exec.task_join_failed",
                    "Codex exec operation task could not be joined",
                )),
            };
            CleanupOutcome::merge(stop_cleanup, join_cleanup)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exec_events::ExecEventParser;
    use crate::exec_input::{ExecMaterializations, SharedExecMaterializations};
    use crate::exec_pump::pump;
    use crate::process_close::close_budget;
    use std::sync::atomic::AtomicUsize;
    use swallowtail_core::ExecutionHostId;
    use swallowtail_runtime::{
        HostServices, JoinedTask, ProcessExit, ProcessInputChunk, ProcessOutputChunk, RuntimeEvent,
        ScopeId, ScopedTaskService, TerminalStatus, runtime_event_channel,
        terminal_outcome_channel,
    };

    /// Child that exits only when force-stopped; a join before any stop is refused.
    #[derive(Default)]
    struct StubbornRun {
        forced: AtomicUsize,
        stopped: Mutex<Option<futures_channel::oneshot::Sender<()>>>,
    }

    impl ProcessHandle for StubbornRun {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
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
            self.forced.fetch_add(1, Ordering::SeqCst);
            if let Some(stopped) = self.stopped.lock().expect("stop lock").take() {
                let _ = stopped.send(());
            }
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    /// Joins only after the run has been force-stopped, so a close that does not
    /// cancel first fails the join instead of hanging the test.
    struct RunTask {
        stopped: futures_channel::oneshot::Receiver<()>,
    }

    impl JoinedTask for RunTask {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let stopped = self.stopped;
            Box::pin(async move {
                stopped.await.map_err(|_| {
                    RuntimeFailure::new(SafeDiagnostic::new("test.not_cancelled", "not cancelled"))
                })
            })
        }
    }

    fn active_handle(process: &Arc<StubbornRun>) -> Box<CodexExecRunHandle> {
        let (stopped_sender, stopped) = futures_channel::oneshot::channel();
        *process.stopped.lock().expect("stop lock") = Some(stopped_sender);
        let (_finish_tx, finish_rx) = futures_channel::oneshot::channel();
        drop(_finish_tx);
        let terminal = Box::pin(async {
            TerminalOutcome::new(TerminalStatus::Cancelled, CleanupOutcome::Clean)
        });
        let services =
            HostServices::new(ExecutionHostId::new("codex.exec.handle-test").expect("host id"));
        Box::new(CodexExecRunHandle::new(
            RequestId::new("exec-handle-close").expect("request id is valid"),
            RuntimeRunId::new("exec-handle-close-run").expect("run id is valid"),
            Box::pin(futures_util::stream::empty::<
                Result<RuntimeEvent, RuntimeFailure>,
            >()),
            terminal,
            Arc::new(ProcessCancellation::new(
                Arc::clone(process) as Arc<dyn ProcessHandle>
            )),
            AppServerTask::from_parts(
                Box::new(RunTask { stopped }),
                finish_rx,
                ScopeId::new("codex-exec-handle-test").expect("scope id"),
            ),
            close_budget(&services, None),
        ))
    }

    #[test]
    fn closing_an_active_exec_handle_cancels_the_run_before_joining() {
        let process = Arc::new(StubbornRun::default());
        let outcome = futures_executor::block_on(active_handle(&process).close());
        assert!(matches!(outcome, CleanupOutcome::Clean));
        assert_eq!(process.forced.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn closing_a_finished_exec_handle_does_not_cancel_the_run() {
        let process = Arc::new(StubbornRun::default());
        let handle = active_handle(&process);
        handle.cancellation.mark_finished();
        // The finished run never receives a stop; release the join by hand.
        process.stopped.lock().expect("stop lock").take();
        let outcome = futures_executor::block_on(handle.close());
        assert!(matches!(outcome, CleanupOutcome::Failed(_)));
        assert_eq!(process.forced.load(Ordering::SeqCst), 0);
    }

    struct RefusingOpenStdoutProcess;

    impl ProcessHandle for RefusingOpenStdoutProcess {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
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
            Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "test.force_stop_refused",
                    "force stop refused",
                )))
            })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
    }

    struct ThreadTaskService;

    struct ThreadJoin(Option<std::thread::JoinHandle<()>>);

    impl JoinedTask for ThreadJoin {
        fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let handle = self.0.take();
            Box::pin(async move {
                if let Some(handle) = handle {
                    handle.join().map_err(|_| {
                        RuntimeFailure::new(SafeDiagnostic::new("test.join_panicked", "panicked"))
                    })?;
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
            let handle = std::thread::spawn(move || {
                futures_executor::block_on(task);
            });
            Ok(Box::new(ThreadJoin(Some(handle))))
        }
    }

    #[test]
    fn closing_active_exec_handle_when_force_stop_is_refused_aborts_pump_and_joins() {
        let process: Arc<dyn ProcessHandle> = Arc::new(RefusingOpenStdoutProcess);
        let services = HostServices::new(
            ExecutionHostId::new("codex.exec.refused-stop-test").expect("host id"),
        );
        let run_id = RuntimeRunId::new("exec-refused-stop").expect("run id");
        let scope = ScopeId::new("codex-exec-refused-stop").expect("scope id");
        let (event_sender, event_stream) = runtime_event_channel(8).expect("event channel");
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(ProcessCancellation::new(Arc::clone(&process)));
        let parser = ExecEventParser::new(run_id.clone(), semver::Version::new(0, 145, 0));
        let task = AppServerTask::spawn(
            Arc::new(ThreadTaskService),
            scope,
            Box::pin({
                let pump_cancellation = Arc::clone(&cancellation);
                let finish_marker = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                let services = services.clone();
                async move {
                    let outcome = pump(
                        process,
                        event_sender.clone(),
                        pump_cancellation,
                        None,
                        SharedExecMaterializations::new(ExecMaterializations::new(None, None)),
                        parser,
                        services,
                    )
                    .await;
                    finish_marker.mark_finished();
                    let _ = terminal_sender.complete(outcome);
                    event_sender.mark_terminal();
                }
            }),
        )
        .expect("pump task spawns");
        let handle = Box::new(CodexExecRunHandle::new(
            RequestId::new("exec-refused-stop-req").expect("request id"),
            run_id,
            Box::pin(event_stream),
            Box::pin(terminal_future),
            cancellation,
            task,
            close_budget(&services, None),
        ));
        let cleanup = futures_executor::block_on(handle.close());
        assert!(
            matches!(cleanup, CleanupOutcome::Failed(ref diag) if diag.code() == "test.force_stop_refused"),
            "expected force_stop_refused cleanup failure, got {cleanup:?}"
        );
    }

    struct PastClock;

    impl swallowtail_runtime::TimeService for PastClock {
        fn now(&self) -> swallowtail_runtime::MonotonicInstant {
            swallowtail_runtime::MonotonicInstant::from_ticks(0)
        }

        fn wait_until(
            &self,
            deadline: swallowtail_runtime::Deadline,
        ) -> BoxFuture<'static, swallowtail_runtime::DeadlineObservation> {
            Box::pin(async move {
                swallowtail_runtime::DeadlineObservation::new(
                    deadline,
                    swallowtail_runtime::MonotonicInstant::from_ticks(u64::MAX),
                )
            })
        }
    }

    struct PendingStopOpenStdoutProcess;

    impl ProcessHandle for PendingStopOpenStdoutProcess {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
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
            Box::pin(std::future::pending())
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
    }

    struct AcceptedStopOpenStdoutProcess {
        forced: AtomicUsize,
    }

    impl ProcessHandle for AcceptedStopOpenStdoutProcess {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
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
            self.forced.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }
        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
    }

    fn spawn_pump_handle(
        process: Arc<dyn ProcessHandle>,
        services: HostServices,
        id: &str,
    ) -> Box<CodexExecRunHandle> {
        let run_id = RuntimeRunId::new(id).expect("run id");
        let scope = ScopeId::new(format!("codex-exec-{id}")).expect("scope id");
        let (event_sender, event_stream) = runtime_event_channel(8).expect("event channel");
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(ProcessCancellation::new(Arc::clone(&process)));
        let parser = ExecEventParser::new(run_id.clone(), semver::Version::new(0, 145, 0));
        let task = AppServerTask::spawn(
            Arc::new(ThreadTaskService),
            scope,
            Box::pin({
                let pump_cancellation = Arc::clone(&cancellation);
                let finish_marker = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                let services = services.clone();
                async move {
                    let outcome = pump(
                        process,
                        event_sender.clone(),
                        pump_cancellation,
                        None,
                        SharedExecMaterializations::new(ExecMaterializations::new(None, None)),
                        parser,
                        services,
                    )
                    .await;
                    finish_marker.mark_finished();
                    let _ = terminal_sender.complete(outcome);
                    event_sender.mark_terminal();
                }
            }),
        )
        .expect("pump task spawns");
        Box::new(CodexExecRunHandle::new(
            RequestId::new(format!("{id}-req")).expect("request id"),
            run_id,
            Box::pin(event_stream),
            Box::pin(terminal_future),
            cancellation,
            task,
            close_budget(&services, None),
        ))
    }

    #[test]
    fn closing_active_exec_handle_with_pending_stop_times_out_aborts_pump_and_joins() {
        let process: Arc<dyn ProcessHandle> = Arc::new(PendingStopOpenStdoutProcess);
        let services = HostServices::new(
            ExecutionHostId::new("codex.exec.pending-stop-test").expect("host id"),
        )
        .with_time(Arc::new(PastClock));
        let handle = spawn_pump_handle(process, services, "exec-pending-stop");
        let cleanup = futures_executor::block_on(handle.close());
        assert!(
            matches!(cleanup, CleanupOutcome::Failed(ref diag) if diag.code() == "swallowtail.codex.exec.stop_timed_out"),
            "expected stop_timed_out cleanup failure, got {cleanup:?}"
        );
    }

    #[test]
    fn closing_active_exec_handle_with_accepted_stop_and_open_stdout_aborts_pump_and_joins() {
        let process = Arc::new(AcceptedStopOpenStdoutProcess {
            forced: AtomicUsize::new(0),
        });
        let services = HostServices::new(
            ExecutionHostId::new("codex.exec.accepted-stop-open-stdout").expect("host id"),
        )
        .with_time(Arc::new(PastClock));
        let handle = spawn_pump_handle(
            Arc::clone(&process) as Arc<dyn ProcessHandle>,
            services,
            "exec-accepted-open",
        );
        let cleanup = futures_executor::block_on(handle.close());
        assert_eq!(process.forced.load(Ordering::SeqCst), 1);
        assert!(
            matches!(cleanup, CleanupOutcome::Failed(ref diag) if diag.code() == "swallowtail.codex.exec.process_cleanup_failed"),
            "expected process_cleanup_failed when accepted stop leaves stdout open, got {cleanup:?}"
        );
    }

    #[test]
    fn closing_active_exec_handle_with_ready_error_blocking_work_does_not_re_poll_and_aborts_pump()
    {
        struct ReadyErrorBlocking;

        impl swallowtail_runtime::BlockingWorkService for ReadyErrorBlocking {
            fn run(
                &self,
                _scope: ScopeId,
                _job: swallowtail_runtime::BlockingJob,
            ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
                Box::pin(async {
                    Err(RuntimeFailure::new(SafeDiagnostic::new(
                        "test.blocking_failed",
                        "blocking failed",
                    )))
                })
            }
        }

        let process = Arc::new(AcceptedStopOpenStdoutProcess {
            forced: AtomicUsize::new(0),
        });
        let services = HostServices::new(
            ExecutionHostId::new("codex.exec.blocking-ready-err").expect("host id"),
        )
        .with_blocking_work(Arc::new(ReadyErrorBlocking));
        let handle = spawn_pump_handle(
            Arc::clone(&process) as Arc<dyn ProcessHandle>,
            services,
            "exec-blocking-err",
        );
        let cleanup = futures_executor::block_on(handle.close());
        assert_eq!(process.forced.load(Ordering::SeqCst), 1);
        assert!(
            matches!(cleanup, CleanupOutcome::Failed(ref diag) if diag.code() == "swallowtail.codex.exec.process_cleanup_failed"),
            "expected process_cleanup_failed without re-poll panic, got {cleanup:?}"
        );
    }

    #[test]
    fn closing_active_exec_handle_with_accepted_stop_and_open_stdout_without_timing_aborts_pump() {
        let process = Arc::new(AcceptedStopOpenStdoutProcess {
            forced: AtomicUsize::new(0),
        });
        let services = HostServices::new(
            ExecutionHostId::new("codex.exec.clockless-open-stdout").expect("host id"),
        );
        let handle = spawn_pump_handle(
            Arc::clone(&process) as Arc<dyn ProcessHandle>,
            services,
            "exec-clockless-open",
        );
        let cleanup = futures_executor::block_on(handle.close());
        assert_eq!(process.forced.load(Ordering::SeqCst), 1);
        assert!(
            matches!(cleanup, CleanupOutcome::Failed(ref diag) if diag.code() == "swallowtail.codex.exec.process_cleanup_failed"),
            "expected process_cleanup_failed on clockless host with open stdout, got {cleanup:?}"
        );
    }
}
