use super::SessionState;
use crate::handle::QwenProcessCancellation;
use crate::working_resource::{QwenWorkingResourceLease, combine_cleanup};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::CancellationScope;
use swallowtail_runtime::{
    BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome, JoinedTask,
    RuntimeFailure, RuntimeTurnId,
};

pub(crate) struct ActiveTurn {
    pub(super) turn_id: RuntimeTurnId,
    pub(super) task: Option<Box<dyn JoinedTask>>,
    pub(super) cancellation: Arc<QwenProcessCancellation>,
    pub(super) terminal: Arc<AtomicBool>,
    pub(super) working_resource: Option<QwenWorkingResourceLease>,
}

pub(crate) type ActiveSlot = Arc<Mutex<Option<ActiveTurn>>>;

pub(super) struct SessionCancellation {
    active: ActiveSlot,
    state: Arc<Mutex<SessionState>>,
    requested: AtomicBool,
}

impl SessionCancellation {
    pub(super) fn new(active: ActiveSlot, state: Arc<Mutex<SessionState>>) -> Self {
        Self {
            active,
            state,
            requested: AtomicBool::new(false),
        }
    }
}

impl CancellationControl for SessionCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::InteractiveSession
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let requested = !self.requested.swap(true, Ordering::SeqCst);
        self.state
            .lock()
            .expect("Qwen session lock poisoned")
            .usable = false;
        let active = self
            .active
            .lock()
            .expect("Qwen active turn lock poisoned")
            .as_ref()
            .map(|turn| Arc::clone(&turn.cancellation));
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

pub(super) async fn reap_finished(
    active: &ActiveSlot,
    state: &Arc<Mutex<SessionState>>,
) -> Result<(), RuntimeFailure> {
    let turn = {
        let mut active = active.lock().expect("Qwen active turn lock poisoned");
        if active
            .as_ref()
            .is_some_and(|turn| turn.terminal.load(Ordering::SeqCst))
        {
            active.take()
        } else {
            None
        }
    };
    if let Some(mut turn) = turn {
        let join_result = match turn.task.take() {
            Some(task) => task.join().await,
            None => Ok(()),
        };
        let resource_cleanup = match turn.working_resource.take() {
            Some(lease) => lease.release().await,
            None => CleanupOutcome::NotApplicable,
        };
        let cleanup = combine_cleanup(
            match &join_result {
                Ok(()) => CleanupOutcome::Clean,
                Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
            },
            resource_cleanup,
        );
        if let Err(error) = join_result {
            state.lock().expect("Qwen session lock poisoned").usable = false;
            return Err(error);
        }
        if let Some(diagnostic) = cleanup.diagnostic() {
            state.lock().expect("Qwen session lock poisoned").usable = false;
            return Err(RuntimeFailure::new(diagnostic.clone()));
        }
    }
    Ok(())
}

pub(super) async fn join_turn(
    active: &ActiveSlot,
    turn_id: &RuntimeTurnId,
    state: &Arc<Mutex<SessionState>>,
) -> CleanupOutcome {
    let (task, working_resource) = {
        let mut active = active.lock().expect("Qwen active turn lock poisoned");
        match active.as_mut() {
            Some(turn) if &turn.turn_id == turn_id => {
                (turn.task.take(), turn.working_resource.take())
            }
            _ => return CleanupOutcome::NotApplicable,
        }
    };
    let task_cleanup = match task {
        Some(task) => match task.join().await {
            Ok(()) => CleanupOutcome::Clean,
            Err(error) => {
                state.lock().expect("Qwen session lock poisoned").usable = false;
                CleanupOutcome::Failed(error.diagnostic().clone())
            }
        },
        None => CleanupOutcome::NotApplicable,
    };
    let resource_cleanup = match working_resource {
        Some(lease) => lease.release().await,
        None => CleanupOutcome::NotApplicable,
    };
    let cleanup = combine_cleanup(task_cleanup, resource_cleanup);
    if matches!(
        cleanup,
        CleanupOutcome::Degraded(_) | CleanupOutcome::Failed(_)
    ) {
        state.lock().expect("Qwen session lock poisoned").usable = false;
    }
    let mut active = active.lock().expect("Qwen active turn lock poisoned");
    if active.as_ref().is_some_and(|turn| &turn.turn_id == turn_id) {
        *active = None;
    }
    cleanup
}

pub(super) async fn close_active(active: &ActiveSlot) -> CleanupOutcome {
    let (cancellation, task, working_resource) = {
        let mut active = active.lock().expect("Qwen active turn lock poisoned");
        match active.as_mut() {
            Some(turn) => (
                (!turn.terminal.load(Ordering::SeqCst)).then(|| Arc::clone(&turn.cancellation)),
                turn.task.take(),
                turn.working_resource.take(),
            ),
            None => return CleanupOutcome::Clean,
        }
    };
    if let Some(cancellation) = cancellation {
        let _ = cancellation.request().await;
    }
    let task_cleanup = match task {
        Some(task) => match task.join().await {
            Ok(()) => CleanupOutcome::Clean,
            Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
        },
        None => CleanupOutcome::NotApplicable,
    };
    let resource_cleanup = match working_resource {
        Some(lease) => lease.release().await,
        None => CleanupOutcome::NotApplicable,
    };
    *active.lock().expect("Qwen active turn lock poisoned") = None;
    combine_cleanup(task_cleanup, resource_cleanup)
}
