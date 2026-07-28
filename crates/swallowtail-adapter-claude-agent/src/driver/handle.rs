use super::*;
use crate::driver::session::{ActiveSlot, cleanup_failure, join_active};

pub(super) struct SessionCancellation {
    connection: Arc<AcpConnection>,
    requested: AtomicBool,
}

impl SessionCancellation {
    pub(super) fn new(connection: Arc<AcpConnection>) -> Self {
        Self {
            connection,
            requested: AtomicBool::new(false),
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
                self.connection.cancel_session().await?;
                Ok(CancellationAcknowledgement::Requested)
            }
        })
    }
}

pub(super) struct TurnCancellation {
    pub(super) connection: Arc<AcpConnection>,
    pub(super) session_id: String,
    pub(super) turn: Arc<ActiveTurn>,
    pub(super) requested: AtomicBool,
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
            self.connection
                .notify("session/cancel", json!({"sessionId": self.session_id}))
                .await?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

pub(super) struct ClaudeAgentTurnHandle {
    pub(super) runtime_id: swallowtail_runtime::RuntimeTurnId,
    pub(super) events: Option<BoxEventStream>,
    pub(super) callbacks: Option<swallowtail_runtime::CallbackExchange>,
    pub(super) terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    pub(super) cancellation: TurnCancellation,
    pub(super) active: ActiveSlot,
}

impl TurnHandle for ClaudeAgentTurnHandle {
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
            if let Some(mut active) = active {
                if join_active(&mut active).await.is_err() {
                    cleanup_failure(
                        "turn_join_failed",
                        "Claude Agent ACP turn tasks did not join cleanly",
                    )
                } else {
                    CleanupOutcome::NotApplicable
                }
            } else {
                CleanupOutcome::NotApplicable
            }
        })
    }
}
