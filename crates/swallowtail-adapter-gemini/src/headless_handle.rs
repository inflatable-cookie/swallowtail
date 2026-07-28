use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::{CancellationScope, RunRef, SafeDiagnostic};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    JoinedTask, ProcessHandle, RequestId, RunHandle, RuntimeFailure, RuntimeRunId, TerminalOutcome,
};

pub(crate) struct GeminiHeadlessCancellation {
    process: Arc<dyn ProcessHandle>,
    requested: AtomicBool,
}

impl GeminiHeadlessCancellation {
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

impl CancellationControl for GeminiHeadlessCancellation {
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

pub(crate) struct GeminiHeadlessRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    provider_run_ref: RunRef,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<GeminiHeadlessCancellation>,
    task: Box<dyn JoinedTask>,
}

impl GeminiHeadlessRunHandle {
    pub(crate) fn new(
        request_id: RequestId,
        run_id: RuntimeRunId,
        provider_run_ref: RunRef,
        events: BoxEventStream,
        terminal: BoxFuture<'static, TerminalOutcome>,
        cancellation: Arc<GeminiHeadlessCancellation>,
        task: Box<dyn JoinedTask>,
    ) -> Self {
        Self {
            request_id,
            run_id,
            provider_run_ref,
            events: Some(events),
            terminal: Some(terminal),
            cancellation,
            task,
        }
    }
}

impl RunHandle for GeminiHeadlessRunHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn run_id(&self) -> &RuntimeRunId {
        &self.run_id
    }

    fn provider_run_ref(&self) -> Option<&RunRef> {
        Some(&self.provider_run_ref)
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
            match self.task.join().await {
                Ok(()) => CleanupOutcome::Clean,
                Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.gemini.headless.task_join_failed",
                    "Gemini headless operation task could not be joined",
                )),
            }
        })
    }
}
