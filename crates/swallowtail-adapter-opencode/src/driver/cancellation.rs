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

/// Maximum time spent waiting for a best-effort `/abort` HTTP exchange during
/// deadline/timeout cleanup before abandoning the stalled abort request.
const ABORT_CLEANUP_BUDGET_TICKS: u64 = 500;

impl TurnCancellation {
    fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }

    async fn request_with_deadline(
        &self,
        abort_deadline: Option<Deadline>,
    ) -> Result<CancellationAcknowledgement, RuntimeFailure> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        self.stream_cancelled.store(true, Ordering::SeqCst);
        if let Some(callbacks) = &self.callbacks {
            callbacks.abandon(swallowtail_runtime::CallbackAbandonment::TurnCancelled);
        }
        if already {
            return Ok(CancellationAcknowledgement::AlreadyRequested);
        }
        let abort_request = abort(&self.session_id, &self.directory)?;
        let abort_cancelled = Arc::new(AtomicBool::new(false));
        let response = complete_before_deadline(
            self.transport.request(
                self.scope.clone(),
                self.endpoint.clone(),
                abort_request,
                &self.services,
                Arc::clone(&abort_cancelled),
            ),
            abort_deadline,
            &self.services,
            abort_cancelled,
            "swallowtail.opencode.abort_timed_out",
            "OpenCode abort request timed out",
        )
        .await?;
        require_abort_success(&response)?;
        Ok(CancellationAcknowledgement::Requested)
    }

    async fn request_bounded(&self) -> Result<CancellationAcknowledgement, RuntimeFailure> {
        let abort_deadline = self.services.time().map(|time| {
            Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
                time.now().ticks().saturating_add(ABORT_CLEANUP_BUDGET_TICKS),
            ))
        });
        self.request_with_deadline(abort_deadline).await
    }
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        Box::pin(async move { self.request_with_deadline(None).await })
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

/// Maps the cancellation request a close makes into its cleanup outcome. A
/// refused request leaves the turn or run unconfirmed, so its failure is
/// reported instead of discarded.
fn cancellation_cleanup(
    result: Result<swallowtail_runtime::CancellationAcknowledgement, RuntimeFailure>,
) -> CleanupOutcome {
    match result {
        Ok(_) => CleanupOutcome::NotApplicable,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    }
}
