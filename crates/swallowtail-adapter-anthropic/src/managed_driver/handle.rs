struct ManagedRunCancellation {
    requested: AtomicBool,
    active_connection: Mutex<Arc<AtomicBool>>,
    callbacks: ManagedCallbackHub,
}

impl ManagedRunCancellation {
    fn new(connection: Arc<AtomicBool>, callbacks: ManagedCallbackHub) -> Self {
        Self {
            requested: AtomicBool::new(false),
            active_connection: Mutex::new(connection),
            callbacks,
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
            .expect("managed active connection lock poisoned") = connection;
    }

    fn stop_active(&self) {
        self.active_connection
            .lock()
            .expect("managed active connection lock poisoned")
            .store(true, Ordering::SeqCst);
    }
}

impl CancellationControl for ManagedRunCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let prior = self.requested.swap(true, Ordering::SeqCst);
        self.stop_active();
        self.callbacks.abandon(CallbackAbandonment::TurnCancelled);
        Box::pin(async move {
            Ok(if prior {
                CancellationAcknowledgement::AlreadyRequested
            } else {
                CancellationAcknowledgement::Requested
            })
        })
    }
}

struct ManagedRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    callbacks: Option<CallbackExchange>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<ManagedRunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for ManagedRunHandle {
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
            self.cancellation.requested.store(true, Ordering::SeqCst);
            self.cancellation.stop_active();
            self.cancellation
                .callbacks
                .abandon(CallbackAbandonment::Closed);
            match self.task.join().await {
                Ok(()) => CleanupOutcome::Clean,
                Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.anthropic.managed.task_join_failed",
                    "Anthropic Managed Agents operation task could not be joined",
                )),
            }
        })
    }
}
