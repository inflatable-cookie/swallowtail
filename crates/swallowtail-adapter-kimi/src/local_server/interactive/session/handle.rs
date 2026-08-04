use super::super::access::merge;
use super::super::websocket::SubscriptionControl;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{CancellationScope, TurnRef};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackExchange, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, OperationDetachmentAcknowledgement, OperationDetachmentControl, RuntimeFailure,
    RuntimeTurnId, TerminalOutcome, TurnHandle,
};

pub(in crate::local_server) type ActiveSlot = Arc<Mutex<Option<ActiveTurn>>>;

pub(in crate::local_server) struct ActiveTurn {
    pub(in crate::local_server) task: Option<Box<dyn swallowtail_runtime::JoinedTask>>,
    pub(in crate::local_server) cancellation: Arc<TurnCancellation>,
    pub(in crate::local_server) terminal: Arc<AtomicBool>,
}

pub(in crate::local_server::interactive) struct SessionCancellation {
    active: ActiveSlot,
    pub(super) requested: AtomicBool,
}

impl SessionCancellation {
    pub(in crate::local_server::interactive) fn new(active: ActiveSlot) -> Self {
        Self {
            active,
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
        let active = self
            .active
            .lock()
            .expect("active turn lock poisoned")
            .as_ref()
            .map(|turn| Arc::clone(&turn.cancellation));
        Box::pin(async move {
            if let Some(active) = active {
                let _ = active.request().await?;
            }
            Ok(if already {
                CancellationAcknowledgement::AlreadyRequested
            } else {
                CancellationAcknowledgement::Requested
            })
        })
    }
}

pub(in crate::local_server) struct TurnCancellation {
    pub(super) control: Mutex<SubscriptionControl>,
    pub(super) session_id: String,
    pub(super) prompt_id: String,
    pub(super) requested: AtomicBool,
}

pub(in crate::local_server) struct TurnDetachment {
    pub(super) cancellation: Arc<TurnCancellation>,
    pub(super) terminal: Arc<AtomicBool>,
    pub(super) checkpoint_ready: Arc<AtomicBool>,
    pub(super) requested: AtomicBool,
}

impl TurnDetachment {
    pub(super) fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }
}

impl OperationDetachmentControl for TurnDetachment {
    fn scope(&self) -> swallowtail_core::OperationDetachmentScope {
        swallowtail_core::OperationDetachmentScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<OperationDetachmentAcknowledgement, RuntimeFailure>> {
        Box::pin(async move {
            if self.cancellation.requested.load(Ordering::SeqCst) {
                return Err(crate::failure::failure(
                    "swallowtail.kimi.local_server.detachment_cancelled",
                    "Kimi local-server turn cancellation already won operation disposition",
                ));
            }
            if self.is_requested() {
                return Ok(OperationDetachmentAcknowledgement::AlreadyRequested);
            }
            if self.terminal.load(Ordering::SeqCst) {
                return Err(crate::failure::failure(
                    "swallowtail.kimi.local_server.detachment_terminal",
                    "Kimi local-server turn already reached local terminal state",
                ));
            }
            if !self.checkpoint_ready.load(Ordering::SeqCst) {
                return Err(crate::failure::failure(
                    "swallowtail.kimi.local_server.detachment_checkpoint_unavailable",
                    "Kimi local-server turn has no recoverable operation checkpoint",
                ));
            }
            let already = self.requested.swap(true, Ordering::SeqCst);
            if self.cancellation.requested.load(Ordering::SeqCst) {
                return Err(crate::failure::failure(
                    "swallowtail.kimi.local_server.detachment_cancelled",
                    "Kimi local-server turn cancellation won operation disposition",
                ));
            }
            let control = self
                .cancellation
                .control
                .lock()
                .expect("subscription control lock poisoned")
                .clone();
            control.close()?;
            Ok(if already {
                OperationDetachmentAcknowledgement::AlreadyRequested
            } else {
                OperationDetachmentAcknowledgement::Requested
            })
        })
    }
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
            let control = self
                .control
                .lock()
                .expect("subscription control lock poisoned")
                .clone();
            control.abort(&self.session_id, &self.prompt_id).await?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

pub(in crate::local_server::interactive) struct KimiTurnHandle {
    pub(super) runtime_id: RuntimeTurnId,
    pub(super) provider_ref: Option<TurnRef>,
    pub(super) events: Option<BoxEventStream>,
    pub(super) callbacks: Option<CallbackExchange>,
    pub(super) terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    pub(super) cancellation: Arc<TurnCancellation>,
    pub(super) detachment: Option<Arc<TurnDetachment>>,
    pub(super) terminal_flag: Arc<AtomicBool>,
    pub(super) active: ActiveSlot,
}

impl TurnHandle for KimiTurnHandle {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    fn provider_turn_ref(&self) -> Option<&TurnRef> {
        self.provider_ref.as_ref()
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

    fn detachment(&self) -> Option<&dyn OperationDetachmentControl> {
        self.detachment
            .as_deref()
            .map(|control| control as &dyn OperationDetachmentControl)
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if !self.terminal_flag.load(Ordering::SeqCst)
                && !self
                    .detachment
                    .as_ref()
                    .is_some_and(|detachment| detachment.is_requested())
            {
                let _ = self.cancellation.request().await;
            }
            join(&self.active).await
        })
    }
}

pub(in crate::local_server::interactive) async fn reap(
    active: &ActiveSlot,
) -> Result<(), RuntimeFailure> {
    let task = {
        let mut active = active.lock().expect("active turn lock poisoned");
        if active
            .as_ref()
            .is_some_and(|turn| turn.terminal.load(Ordering::SeqCst))
        {
            active.as_mut().and_then(|turn| turn.task.take())
        } else {
            None
        }
    };
    if let Some(task) = task {
        task.join().await?;
        *active.lock().expect("active turn lock poisoned") = None;
    }
    Ok(())
}

async fn join(active: &ActiveSlot) -> CleanupOutcome {
    let task = active
        .lock()
        .expect("active turn lock poisoned")
        .as_mut()
        .and_then(|turn| turn.task.take());
    let cleanup = match task {
        Some(task) => match task.join().await {
            Ok(()) => CleanupOutcome::Clean,
            Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
        },
        None => CleanupOutcome::NotApplicable,
    };
    *active.lock().expect("active turn lock poisoned") = None;
    cleanup
}

pub(in crate::local_server::interactive) async fn close_active(
    active: &ActiveSlot,
) -> CleanupOutcome {
    let cancellation = active
        .lock()
        .expect("active turn lock poisoned")
        .as_ref()
        .filter(|turn| !turn.terminal.load(Ordering::SeqCst))
        .map(|turn| Arc::clone(&turn.cancellation));
    let cancellation = match cancellation {
        Some(cancellation) => match cancellation.request().await {
            Ok(_) => CleanupOutcome::Clean,
            Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
        },
        None => CleanupOutcome::NotApplicable,
    };
    merge(cancellation, join(active).await)
}
