use swallowtail_core::{CancellationScope, RunRef, SafeDiagnostic};
use swallowtail_runtime::{
    BoxEventStream, CancellationAcknowledgement, CancellationControl, JoinedTask, RequestId,
};

struct RunCancellation {
    cancelled: Arc<AtomicBool>,
}

impl RunCancellation {
    fn new(cancelled: Arc<AtomicBool>) -> Self {
        Self { cancelled }
    }

    fn is_requested(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

impl CancellationControl for RunCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let prior = self.cancelled.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            Ok(if prior {
                CancellationAcknowledgement::AlreadyRequested
            } else {
                CancellationAcknowledgement::Requested
            })
        })
    }
}

struct LlamaCppRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<RunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl LlamaCppRunHandle {
    fn new(
        request_id: RequestId,
        run_id: RuntimeRunId,
        events: BoxEventStream,
        terminal: BoxFuture<'static, TerminalOutcome>,
        cancellation: Arc<RunCancellation>,
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

impl RunHandle for LlamaCppRunHandle {
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
            self.cancellation.cancelled.store(true, Ordering::SeqCst);
            match self.task.join().await {
                Ok(()) => CleanupOutcome::Clean,
                Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.llama_cpp.task_join_failed",
                    "llama.cpp operation task could not be joined",
                )),
            }
        })
    }
}
