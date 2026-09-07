struct SessionCancellation {
    connection: Arc<AcpConnection>,
    requested: AtomicBool,
    registered: Option<Arc<crate::registered_tool::GrokRegisteredToolSession>>,
    services: HostServices,
}

impl SessionCancellation {
    fn new(
        connection: Arc<AcpConnection>,
        registered: Option<Arc<crate::registered_tool::GrokRegisteredToolSession>>,
        services: HostServices,
    ) -> Self {
        Self {
            connection,
            requested: AtomicBool::new(false),
            registered,
            services,
        }
    }
}

impl CancellationControl for SessionCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::InteractiveSession
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            if already {
                Ok(CancellationAcknowledgement::AlreadyRequested)
            } else {
                // Cancellation freezes registered admission before the
                // provider is asked to stop, so no further call is admitted
                // while the turn settles.
                if let Some(registered) = self.registered.as_ref() {
                    registered.freeze(&self.services).await;
                }
                self.connection.cancel_session().await?;
                Ok(CancellationAcknowledgement::Requested)
            }
        })
    }
}

struct TurnCancellation {
    connection: Arc<AcpConnection>,
    session_id: String,
    turn: Arc<ActiveTurn>,
    requested: AtomicBool,
    registered: Option<Arc<crate::registered_tool::GrokRegisteredToolSession>>,
    services: HostServices,
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            if already {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            self.turn.mark_cancelled();
            // Freeze before the provider is told to cancel: admission closes
            // ahead of settling, never after the turn has already stopped.
            if let Some(registered) = self.registered.as_ref() {
                registered.freeze(&self.services).await;
            }
            self.connection
                .notify("session/cancel", json!({"sessionId": self.session_id}))
                .await?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

struct GrokTurnHandle {
    runtime_id: swallowtail_runtime::RuntimeTurnId,
    events: Option<BoxEventStream>,
    callbacks: Option<swallowtail_runtime::CallbackExchange>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: TurnCancellation,
    active: ActiveSlot,
}

impl TurnHandle for GrokTurnHandle {
    fn turn_id(&self) -> &swallowtail_runtime::RuntimeTurnId {
        &self.runtime_id
    }

    fn provider_turn_ref(&self) -> Option<&swallowtail_core::TurnRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn take_callbacks(&mut self) -> Option<swallowtail_runtime::CallbackExchange> {
        self.callbacks.take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if !self.cancellation.turn.is_finished() {
                let _ = self.cancellation.request().await;
                let _ = self.cancellation.connection.cancel_session().await;
            }
            let active = {
                let mut slot = self.active.lock().expect("ACP active-task lock poisoned");
                if slot
                    .as_ref()
                    .is_some_and(|active| Arc::ptr_eq(&active.turn, &self.cancellation.turn))
                {
                    slot.take()
                } else {
                    None
                }
            };
            match active.and_then(|mut active| active.task.take()) {
                Some(task) => match task.join().await {
                    Ok(()) => CleanupOutcome::NotApplicable,
                    Err(_) => cleanup_failure(
                        "swallowtail.grok.acp.turn_join_failed",
                        "Grok Build ACP prompt task did not join",
                    ),
                },
                None => CleanupOutcome::NotApplicable,
            }
        })
    }
}
