use super::super::access::merge;
use super::super::websocket::SubscriptionControl;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{CancellationScope, TurnRef};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackExchange, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, RuntimeFailure, RuntimeTurnId, TerminalOutcome, TurnHandle,
};

pub(in crate::local_server::interactive) type ActiveSlot = Arc<Mutex<Option<ActiveTurn>>>;

pub(in crate::local_server::interactive) struct ActiveTurn {
    pub(super) task: Option<Box<dyn swallowtail_runtime::JoinedTask>>,
    pub(super) cancellation: Arc<TurnCancellation>,
    pub(super) terminal: Arc<AtomicBool>,
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

pub(in crate::local_server::interactive) struct TurnCancellation {
    pub(super) control: SubscriptionControl,
    pub(super) session_id: String,
    pub(super) prompt_id: String,
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
            self.control
                .abort(&self.session_id, &self.prompt_id)
                .await?;
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

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if !self.terminal_flag.load(Ordering::SeqCst) {
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
