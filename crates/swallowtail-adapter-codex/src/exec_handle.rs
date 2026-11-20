use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::{CancellationScope, RunRef, SafeDiagnostic};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    JoinedTask, ProcessHandle, RequestId, RunHandle, RuntimeFailure, RuntimeRunId, TerminalOutcome,
};

pub(crate) struct ProcessCancellation {
    process: Arc<dyn ProcessHandle>,
    requested: AtomicBool,
    finished: AtomicBool,
}

impl ProcessCancellation {
    pub(crate) fn new(process: Arc<dyn ProcessHandle>) -> Self {
        Self {
            process,
            requested: AtomicBool::new(false),
            finished: AtomicBool::new(false),
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
    task: Box<dyn JoinedTask>,
}

impl CodexExecRunHandle {
    pub(crate) fn new(
        request_id: RequestId,
        run_id: RuntimeRunId,
        events: BoxEventStream,
        terminal: BoxFuture<'static, TerminalOutcome>,
        cancellation: Arc<ProcessCancellation>,
        task: Box<dyn JoinedTask>,
    ) -> Self {
        Self {
            request_id,
            run_id,
            events: Some(events),
            terminal: Some(terminal),
            cancellation,
            task,
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
            // Cancel an active run first: joining it otherwise waits for the child to exit.
            if !self.cancellation.is_finished() {
                let _ = self.cancellation.request().await;
            }
            match self.task.join().await {
                Ok(()) => CleanupOutcome::Clean,
                Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.codex.exec.task_join_failed",
                    "Codex exec operation task could not be joined",
                )),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::sync::atomic::AtomicUsize;
    use swallowtail_runtime::{
        ProcessExit, ProcessInputChunk, ProcessOutputChunk, RuntimeEvent, TerminalStatus,
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
        let terminal = Box::pin(async {
            TerminalOutcome::new(TerminalStatus::Cancelled, CleanupOutcome::Clean)
        });
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
            Box::new(RunTask { stopped }),
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
}
