use crate::transport::SocketCloser;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::CancellationScope;
use swallowtail_runtime::{
    BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome, JoinedTask,
    RuntimeFailure, RuntimeTurnId,
};

pub(super) struct ActiveTurn {
    pub(super) turn_id: RuntimeTurnId,
    pub(super) task: Option<Box<dyn JoinedTask>>,
    pub(super) cancellation: Arc<TurnCancellation>,
    pub(super) terminal: Arc<AtomicBool>,
}

pub(super) type ActiveSlot = Arc<Mutex<Option<ActiveTurn>>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CancelReason {
    None,
    Cancelled,
    TimedOut,
    Finished,
}

pub(super) struct TurnCancellation {
    closer: SocketCloser,
    chain_valid: Arc<AtomicBool>,
    reason: AtomicU8,
}

impl TurnCancellation {
    pub(super) fn new(closer: SocketCloser, chain_valid: Arc<AtomicBool>) -> Self {
        Self {
            closer,
            chain_valid,
            reason: AtomicU8::new(0),
        }
    }

    pub(super) fn timeout(&self) {
        if self
            .reason
            .compare_exchange(0, 2, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            self.chain_valid.store(false, Ordering::SeqCst);
            self.closer.shutdown();
        }
    }

    pub(super) fn abort(&self) {
        self.chain_valid.store(false, Ordering::SeqCst);
        self.closer.shutdown();
    }

    pub(super) fn finish(&self) -> CancelReason {
        let _ = self
            .reason
            .compare_exchange(0, 3, Ordering::SeqCst, Ordering::SeqCst);
        self.reason()
    }

    pub(super) fn reason(&self) -> CancelReason {
        match self.reason.load(Ordering::SeqCst) {
            1 => CancelReason::Cancelled,
            2 => CancelReason::TimedOut,
            3 => CancelReason::Finished,
            _ => CancelReason::None,
        }
    }
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let requested = self
            .reason
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok();
        if requested {
            self.chain_valid.store(false, Ordering::SeqCst);
            self.closer.shutdown();
        }
        Box::pin(async move {
            Ok(if requested {
                CancellationAcknowledgement::Requested
            } else {
                CancellationAcknowledgement::AlreadyRequested
            })
        })
    }
}

pub(super) struct SessionCancellation {
    closer: SocketCloser,
    active: ActiveSlot,
    requested: AtomicBool,
}

impl SessionCancellation {
    pub(super) fn new(closer: SocketCloser, active: ActiveSlot) -> Self {
        Self {
            closer,
            active,
            requested: AtomicBool::new(false),
        }
    }

    pub(super) fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }
}

impl CancellationControl for SessionCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::InteractiveSession
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let requested = !self.requested.swap(true, Ordering::SeqCst);
        let active = self
            .active
            .lock()
            .expect("active turn lock poisoned")
            .as_ref()
            .map(|turn| Arc::clone(&turn.cancellation));
        if requested {
            self.closer.shutdown();
        }
        Box::pin(async move {
            if let Some(active) = active {
                let _ = active.request().await?;
            }
            Ok(if requested {
                CancellationAcknowledgement::Requested
            } else {
                CancellationAcknowledgement::AlreadyRequested
            })
        })
    }
}

pub(super) async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let task = {
        let mut state = active.lock().expect("active turn lock poisoned");
        if state
            .as_ref()
            .is_some_and(|turn| turn.terminal.load(Ordering::SeqCst))
        {
            state.as_mut().and_then(|turn| turn.task.take())
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

pub(super) async fn join_active(active: &ActiveSlot) -> CleanupOutcome {
    let task = active
        .lock()
        .expect("active turn lock poisoned")
        .as_mut()
        .and_then(|turn| turn.task.take());
    let cleanup = match task {
        Some(task) => cleanup_from_result(task.join().await),
        None => CleanupOutcome::NotApplicable,
    };
    *active.lock().expect("active turn lock poisoned") = None;
    cleanup
}

pub(super) async fn join_turn(active: &ActiveSlot, turn_id: &RuntimeTurnId) -> CleanupOutcome {
    let task = {
        let mut state = active.lock().expect("active turn lock poisoned");
        match state.as_mut() {
            Some(turn) if &turn.turn_id == turn_id => turn.task.take(),
            _ => return CleanupOutcome::NotApplicable,
        }
    };
    let cleanup = match task {
        Some(task) => cleanup_from_result(task.join().await),
        None => CleanupOutcome::NotApplicable,
    };
    let mut state = active.lock().expect("active turn lock poisoned");
    if state.as_ref().is_some_and(|turn| &turn.turn_id == turn_id) {
        *state = None;
    }
    cleanup
}

pub(super) async fn close_active(active: &ActiveSlot) -> CleanupOutcome {
    let cancellation = active
        .lock()
        .expect("active turn lock poisoned")
        .as_ref()
        .filter(|turn| !turn.terminal.load(Ordering::SeqCst))
        .map(|turn| Arc::clone(&turn.cancellation));
    let cancellation = match cancellation {
        Some(cancellation) => cleanup_from_result(cancellation.request().await.map(|_| ())),
        None => CleanupOutcome::NotApplicable,
    };
    merge_cleanup(cancellation, join_active(active).await)
}

pub(super) fn cleanup_from_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    match result {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    }
}

pub(super) fn merge_cleanup(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        (CleanupOutcome::Clean, _) => current,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => next,
        _ => current,
    }
}
