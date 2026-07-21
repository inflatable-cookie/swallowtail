use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{RunRef, SafeDiagnostic};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, JoinedTask,
    RequestId, RunHandle, RuntimeRunId, TerminalOutcome,
};

struct RunCancellation {
    requested: AtomicBool,
    active_connection: Mutex<Arc<AtomicBool>>,
}

impl RunCancellation {
    fn new(connection: Arc<AtomicBool>) -> Self {
        Self {
            requested: AtomicBool::new(false),
            active_connection: Mutex::new(connection),
        }
    }

    fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }

    fn install(&self, connection: Arc<AtomicBool>) {
        if self.is_requested() {
            connection.store(true, Ordering::SeqCst);
        }
        *self
            .active_connection
            .lock()
            .expect("OpenAI active connection lock poisoned") = connection;
    }

    fn stop_active(&self) {
        self.active_connection
            .lock()
            .expect("OpenAI active connection lock poisoned")
            .store(true, Ordering::SeqCst);
    }
}

impl CancellationControl for RunCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let prior = self.requested.swap(true, Ordering::SeqCst);
        self.stop_active();
        Box::pin(async move {
            Ok(if prior {
                CancellationAcknowledgement::AlreadyRequested
            } else {
                CancellationAcknowledgement::Requested
            })
        })
    }
}

struct OpenAiRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    provider_run_ref: RunRef,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<RunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for OpenAiRunHandle {
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
            self.cancellation.requested.store(true, Ordering::SeqCst);
            self.cancellation.stop_active();
            match self.task.join().await {
                Ok(()) => CleanupOutcome::Clean,
                Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.openai.task_join_failed",
                    "OpenAI operation task could not be joined",
                )),
            }
        })
    }
}
