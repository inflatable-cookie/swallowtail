use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::{CancellationScope, RunRef, SafeDiagnostic};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    JoinedTask, ProcessHandle, RequestId, RunHandle, RuntimeFailure, RuntimeRunId, TerminalOutcome,
};

pub(crate) struct ClaudeCodeCancellation {
    process: Arc<dyn ProcessHandle>,
    requested: AtomicBool,
}

impl ClaudeCodeCancellation {
    pub(crate) fn new(process: Arc<dyn ProcessHandle>) -> Self {
        Self {
            process,
            requested: AtomicBool::new(false),
        }
    }

    pub(crate) fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }
}

impl CancellationControl for ClaudeCodeCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already_requested = self.requested.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            if already_requested {
                Ok(CancellationAcknowledgement::AlreadyRequested)
            } else {
                self.process.force_stop().await?;
                Ok(CancellationAcknowledgement::Requested)
            }
        })
    }
}

pub(crate) struct ClaudeCodeRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<ClaudeCodeCancellation>,
    task: Box<dyn JoinedTask>,
    task_join_code: &'static str,
    task_join_message: &'static str,
}

impl ClaudeCodeRunHandle {
    pub(crate) fn new(
        request_id: RequestId,
        run_id: RuntimeRunId,
        events: BoxEventStream,
        terminal: BoxFuture<'static, TerminalOutcome>,
        cancellation: Arc<ClaudeCodeCancellation>,
        task: Box<dyn JoinedTask>,
    ) -> Self {
        Self {
            request_id,
            run_id,
            events: Some(events),
            terminal: Some(terminal),
            cancellation,
            task,
            task_join_code: "swallowtail.claude_code.headless.task_join_failed",
            task_join_message: "Claude Code headless operation task could not be joined",
        }
    }

    pub(crate) fn new_response_only(
        request_id: RequestId,
        run_id: RuntimeRunId,
        events: BoxEventStream,
        terminal: BoxFuture<'static, TerminalOutcome>,
        cancellation: Arc<ClaudeCodeCancellation>,
        task: Box<dyn JoinedTask>,
    ) -> Self {
        Self {
            request_id,
            run_id,
            events: Some(events),
            terminal: Some(terminal),
            cancellation,
            task,
            task_join_code: "swallowtail.claude_code.response_only.task_join_failed",
            task_join_message: "Claude Code response-only task could not be joined",
        }
    }
}

impl RunHandle for ClaudeCodeRunHandle {
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

    fn close(mut self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            // A run still in flight is cancelled first, so close cannot wait on
            // process output the consumer no longer reads.
            if !self.task.is_finished() {
                let _ = self.cancellation.request().await;
            }
            let joined = self.task.join().await;
            let pump_cleanup = take_ready_cleanup(&mut self.terminal);
            let join_cleanup = match joined {
                Ok(()) => CleanupOutcome::Clean,
                Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                    self.task_join_code,
                    self.task_join_message,
                )),
            };
            match pump_cleanup {
                Some(pump_cleanup) => strongest(pump_cleanup, join_cleanup),
                None => join_cleanup,
            }
        })
    }
}

/// Reads the pump's published cleanup when the terminal outcome is already
/// settled, without waiting on anything the pump has not published yet.
fn take_ready_cleanup(
    terminal: &mut Option<BoxFuture<'static, TerminalOutcome>>,
) -> Option<CleanupOutcome> {
    let terminal = terminal.as_mut()?;
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    match terminal.as_mut().poll(&mut context) {
        std::task::Poll::Ready(outcome) => Some(outcome.cleanup().clone()),
        std::task::Poll::Pending => None,
    }
}

/// Keeps the stronger of two cleanup observations.
fn strongest(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
    match (left, right) {
        (CleanupOutcome::Failed(diagnostic), _) | (_, CleanupOutcome::Failed(diagnostic)) => {
            CleanupOutcome::Failed(diagnostic)
        }
        (CleanupOutcome::Degraded(diagnostic), _) | (_, CleanupOutcome::Degraded(diagnostic)) => {
            CleanupOutcome::Degraded(diagnostic)
        }
        (CleanupOutcome::Clean, _) | (_, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        (CleanupOutcome::NotApplicable, CleanupOutcome::NotApplicable) => {
            CleanupOutcome::NotApplicable
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ClaudeCodeCancellation, ClaudeCodeRunHandle};
    use futures_executor::block_on;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use swallowtail_core::SafeDiagnostic;
    use swallowtail_runtime::{
        BoxFuture, CleanupOutcome, JoinedTask, ProcessExit, ProcessHandle, ProcessInputChunk,
        ProcessOutputChunk, RequestId, RunHandle, RuntimeFailure, RuntimeRunId, TerminalOutcome,
        TerminalStatus, runtime_event_channel, terminal_outcome_channel,
    };

    fn unused(what: &str) -> RuntimeFailure {
        RuntimeFailure::new(SafeDiagnostic::new(
            "fixture.unused",
            format!("{what} is unused"),
        ))
    }

    #[derive(Default)]
    struct FixtureProcess {
        forced: AtomicUsize,
    }

    impl ProcessHandle for FixtureProcess {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Err(unused("stdin write")) })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Err(unused("stdin close")) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Err(unused("output read")) })
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Err(unused("graceful stop")) })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.forced.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Err(unused("wait")) })
        }
    }

    struct FixtureTask {
        finished: bool,
        join_fails: bool,
    }

    impl JoinedTask for FixtureTask {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async move {
                if self.join_fails {
                    Err(unused("task join"))
                } else {
                    Ok(())
                }
            })
        }

        fn is_finished(&self) -> bool {
            self.finished
        }
    }

    fn handle(
        terminal: Option<TerminalOutcome>,
        finished: bool,
        join_fails: bool,
    ) -> (Box<ClaudeCodeRunHandle>, Arc<FixtureProcess>) {
        let process = Arc::new(FixtureProcess::default());
        let (events, stream) = runtime_event_channel(1).expect("event channel");
        drop(events);
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        match terminal {
            Some(outcome) => terminal_sender
                .complete(outcome)
                .expect("the fixture publishes one terminal outcome"),
            // A pump still running holds its sender. Keeping it alive leaves
            // the terminal future pending, exactly as an active run does.
            None => std::mem::forget(terminal_sender),
        }
        let cancellation = Arc::new(ClaudeCodeCancellation::new(
            Arc::clone(&process) as Arc<dyn ProcessHandle>
        ));
        let handle = ClaudeCodeRunHandle::new(
            RequestId::new("fixture-request").expect("request"),
            RuntimeRunId::new("fixture-run").expect("run"),
            Box::pin(stream),
            Box::pin(terminal_future),
            cancellation,
            Box::new(FixtureTask {
                finished,
                join_fails,
            }),
        );
        (Box::new(handle), process)
    }

    #[test]
    fn close_propagates_a_pump_cleanup_failure() {
        let diagnostic = SafeDiagnostic::new("fixture.pump.cleanup_failed", "pump cleanup failed");
        let (handle, process) = handle(
            Some(TerminalOutcome::new(
                TerminalStatus::Completed,
                CleanupOutcome::Failed(diagnostic.clone()),
            )),
            true,
            false,
        );
        assert_eq!(block_on(handle.close()), CleanupOutcome::Failed(diagnostic));
        assert_eq!(
            process.forced.load(Ordering::SeqCst),
            0,
            "a finished run is not force-stopped again"
        );
    }

    #[test]
    fn close_cancels_a_running_pump_before_joining() {
        let (handle, process) = handle(None, false, false);
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert_eq!(
            process.forced.load(Ordering::SeqCst),
            1,
            "an active run is cancelled before the pump is joined"
        );
    }

    #[test]
    fn close_reports_a_task_join_failure() {
        let (handle, _process) = handle(None, true, true);
        match block_on(handle.close()) {
            CleanupOutcome::Failed(diagnostic) => assert_eq!(
                diagnostic.code(),
                "swallowtail.claude_code.headless.task_join_failed"
            ),
            other => panic!("expected a join failure, observed {other:?}"),
        }
    }
}
