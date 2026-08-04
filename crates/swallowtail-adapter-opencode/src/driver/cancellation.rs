struct TurnCancellation {
    scope: ScopeId,
    session_id: String,
    directory: String,
    endpoint: String,
    services: HostServices,
    transport: CurlTransport,
    stream_cancelled: Arc<AtomicBool>,
    requested: AtomicBool,
    callbacks: Option<callback::CallbackHub>,
}

impl TurnCancellation {
    fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        self.stream_cancelled.store(true, Ordering::SeqCst);
        if let Some(callbacks) = &self.callbacks {
            callbacks.abandon(swallowtail_runtime::CallbackAbandonment::TurnCancelled);
        }
        Box::pin(async move {
            if already {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            let response = self
                .transport
                .request(
                    self.scope.clone(),
                    self.endpoint.clone(),
                    abort(&self.session_id, &self.directory),
                    &self.services,
                    Arc::new(AtomicBool::new(false)),
                )
                .await?;
            require_abort_success(&response)?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

struct TurnDetachment {
    stream_cancelled: Arc<AtomicBool>,
    terminal: Arc<AtomicBool>,
    cancellation: Arc<TurnCancellation>,
    requested: AtomicBool,
}

impl TurnDetachment {
    fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }
}

impl OperationDetachmentControl for TurnDetachment {
    fn scope(&self) -> swallowtail_core::OperationDetachmentScope {
        swallowtail_core::OperationDetachmentScope::ActiveTurn
    }

    fn request(
        &self,
    ) -> BoxFuture<'_, Result<OperationDetachmentAcknowledgement, RuntimeFailure>> {
        Box::pin(async move {
            if self.cancellation.is_requested() {
                return Err(failure(
                    "swallowtail.opencode.detachment_cancelled",
                    "OpenCode turn cancellation already won operation disposition",
                ));
            }
            if self.is_requested() {
                return Ok(OperationDetachmentAcknowledgement::AlreadyRequested);
            }
            if self.terminal.load(Ordering::SeqCst) {
                return Err(failure(
                    "swallowtail.opencode.detachment_terminal",
                    "OpenCode turn already reached local terminal state",
                ));
            }
            let already = self.requested.swap(true, Ordering::SeqCst);
            if self.cancellation.is_requested() {
                return Err(failure(
                    "swallowtail.opencode.detachment_cancelled",
                    "OpenCode turn cancellation won operation disposition",
                ));
            }
            self.stream_cancelled.store(true, Ordering::SeqCst);
            Ok(if already {
                OperationDetachmentAcknowledgement::AlreadyRequested
            } else {
                OperationDetachmentAcknowledgement::Requested
            })
        })
    }
}
