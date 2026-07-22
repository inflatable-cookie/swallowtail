use super::worker::WorkerHandle;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::CancellationScope;
use swallowtail_runtime::{
    BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome, JoinedTask,
    RuntimeFailure, RuntimeTurnId,
};

pub(super) struct ActiveResponse {
    pub(super) turn_id: RuntimeTurnId,
    pub(super) task: Option<Box<dyn JoinedTask>>,
    pub(super) cancellation: Arc<ResponseCancellation>,
    pub(super) terminal: Arc<AtomicBool>,
}

pub(super) type ActiveSlot = Arc<Mutex<Option<ActiveResponse>>>;

#[derive(Clone, Default)]
pub(super) struct ConnectionRegistry(Arc<Mutex<Vec<WorkerHandle>>>);

impl ConnectionRegistry {
    pub(super) fn add(&self, worker: WorkerHandle) {
        self.0
            .lock()
            .expect("connection registry poisoned")
            .push(worker);
    }

    pub(super) fn remove(&self, worker: &WorkerHandle) {
        self.0
            .lock()
            .expect("connection registry poisoned")
            .retain(|candidate| !candidate.same_connection(worker));
    }

    pub(super) fn abort_all(&self) {
        for worker in self.0.lock().expect("connection registry poisoned").iter() {
            worker.abort();
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CancelReason {
    None,
    Cancelled,
    TimedOut,
    Finished,
}

pub(super) struct ResponseCancellation {
    connections: ConnectionRegistry,
    reusable: Arc<AtomicBool>,
    reason: AtomicU8,
}

impl ResponseCancellation {
    pub(super) fn new(connections: ConnectionRegistry, reusable: Arc<AtomicBool>) -> Self {
        Self {
            connections,
            reusable,
            reason: AtomicU8::new(0),
        }
    }

    pub(super) fn timeout(&self) -> bool {
        self.request_with_reason(2)
    }

    fn request_with_reason(&self, reason: u8) -> bool {
        if self
            .reason
            .compare_exchange(0, reason, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return false;
        }
        self.reusable.store(false, Ordering::SeqCst);
        self.connections.abort_all();
        true
    }

    pub(super) fn reason(&self) -> CancelReason {
        match self.reason.load(Ordering::SeqCst) {
            1 => CancelReason::Cancelled,
            2 => CancelReason::TimedOut,
            3 => CancelReason::Finished,
            _ => CancelReason::None,
        }
    }

    pub(super) fn finish(&self) {
        let _ = self
            .reason
            .compare_exchange(0, 3, Ordering::SeqCst, Ordering::SeqCst);
    }

    pub(super) fn abort(&self) {
        self.reusable.store(false, Ordering::SeqCst);
        self.connections.abort_all();
    }
}

impl CancellationControl for ResponseCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveResponse
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let first = self.request_with_reason(1);
        Box::pin(async move {
            Ok(if first {
                CancellationAcknowledgement::Requested
            } else {
                CancellationAcknowledgement::AlreadyRequested
            })
        })
    }
}

pub(super) struct SessionCancellation {
    connections: ConnectionRegistry,
    active: ActiveSlot,
    reusable: Arc<AtomicBool>,
    requested: AtomicBool,
}

impl SessionCancellation {
    pub(super) fn new(
        connections: ConnectionRegistry,
        active: ActiveSlot,
        reusable: Arc<AtomicBool>,
    ) -> Self {
        Self {
            connections,
            active,
            reusable,
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
        let first = !self.requested.swap(true, Ordering::SeqCst);
        self.reusable.store(false, Ordering::SeqCst);
        let active = self
            .active
            .lock()
            .expect("active response lock poisoned")
            .as_ref()
            .map(|active| Arc::clone(&active.cancellation));
        let connections = self.connections.clone();
        Box::pin(async move {
            if first {
                if let Some(active) = active {
                    let _ = active.request().await?;
                } else {
                    connections.abort_all();
                }
            }
            Ok(if first {
                CancellationAcknowledgement::Requested
            } else {
                CancellationAcknowledgement::AlreadyRequested
            })
        })
    }
}

pub(super) async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let task = {
        let mut state = active.lock().expect("active response lock poisoned");
        if state
            .as_ref()
            .is_some_and(|response| response.terminal.load(Ordering::SeqCst))
        {
            state.as_mut().and_then(|response| response.task.take())
        } else {
            None
        }
    };
    if let Some(task) = task {
        task.join().await?;
        *active.lock().expect("active response lock poisoned") = None;
    }
    Ok(())
}

pub(super) async fn join_response(active: &ActiveSlot, turn: &RuntimeTurnId) -> CleanupOutcome {
    let task = {
        let mut state = active.lock().expect("active response lock poisoned");
        match state.as_mut() {
            Some(response) if &response.turn_id == turn => response.task.take(),
            _ => return CleanupOutcome::NotApplicable,
        }
    };
    let outcome = match task {
        Some(task) => cleanup(task.join().await),
        None => CleanupOutcome::NotApplicable,
    };
    *active.lock().expect("active response lock poisoned") = None;
    outcome
}

pub(super) async fn join_active(active: &ActiveSlot) -> CleanupOutcome {
    let task = active
        .lock()
        .expect("active response lock poisoned")
        .as_mut()
        .and_then(|response| response.task.take());
    let outcome = match task {
        Some(task) => cleanup(task.join().await),
        None => CleanupOutcome::NotApplicable,
    };
    *active.lock().expect("active response lock poisoned") = None;
    outcome
}

pub(super) fn cleanup(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    match result {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    }
}

pub(super) fn merge(first: CleanupOutcome, second: CleanupOutcome) -> CleanupOutcome {
    match (&first, &second) {
        (CleanupOutcome::Failed(_), _) => first,
        (_, CleanupOutcome::Failed(_)) => second,
        (CleanupOutcome::Degraded(_), _) => first,
        (_, CleanupOutcome::Degraded(_)) => second,
        (CleanupOutcome::Clean, _) => first,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => second,
        _ => first,
    }
}
