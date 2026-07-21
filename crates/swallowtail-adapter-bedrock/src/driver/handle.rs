struct RunCancellation {
    requested: AtomicBool,
    signal: watch::Sender<bool>,
}

impl RunCancellation {
    fn request_signal(&self) -> bool {
        let prior = self.requested.swap(true, Ordering::SeqCst);
        let _ = self.signal.send(true);
        prior
    }

    fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }
}

impl swallowtail_runtime::CancellationControl for RunCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<swallowtail_runtime::CancellationAcknowledgement, RuntimeFailure>> {
        let prior = self.request_signal();
        Box::pin(async move {
            Ok(if prior {
                swallowtail_runtime::CancellationAcknowledgement::AlreadyRequested
            } else {
                swallowtail_runtime::CancellationAcknowledgement::Requested
            })
        })
    }
}

struct BedrockRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<RunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for BedrockRunHandle {
    fn request_id(&self) -> &RequestId { &self.request_id }
    fn run_id(&self) -> &RuntimeRunId { &self.run_id }
    fn provider_run_ref(&self) -> Option<&RunRef> { None }
    fn take_events(&mut self) -> Option<BoxEventStream> { self.events.take() }
    fn cancellation(&self) -> &dyn swallowtail_runtime::CancellationControl { self.cancellation.as_ref() }
    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> { self.terminal.take() }
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            self.cancellation.request_signal();
            match self.task.join().await {
                Ok(()) => CleanupOutcome::Clean,
                Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new("swallowtail.bedrock.task_join_failed", "Bedrock Runtime operation task could not be joined")),
            }
        })
    }
}
