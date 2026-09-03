//! Transferring an enclosing guardian task inside a caller deadline, without
//! ever blocking the caller's executor thread.
//!
//! `JoinedTask::join` is allowed to be a blocking observation: the local host's
//! handle owns its worker thread, so its join future can park the thread it is
//! polled on, and dropping an unfinished handle joins as well. Racing that
//! future against a deadline is therefore not a bound at all — the first poll
//! can already overrun it.
//!
//! So the wait uses the trait's own non-blocking seam instead:
//! [`JoinedTask::is_finished`] with [`JoinedTask::register_waker`]. `join` is
//! called only once the task reports finished, where it cannot block on task
//! work. The handle itself is held in a slot the bounded wait only borrows, so
//! expiry never drops it.
//!
//! A task still running when the caller's deadline expires is handed back to
//! the host through [`ScopedTaskService::relinquish`], with the exact execution
//! host the operation selected and the exact `ScopeId` the task was spawned
//! under. The host reaps it autonomously. `AcceptedForReap` is
//! ownership-transfer evidence only: it is never reported as a join and never
//! strengthens a cleanup outcome. The route makes no claim about, and never
//! invokes, the host's own outer reaper shutdown — that lifecycle belongs to
//! the execution host, outside this task tree.
//!
//! Every task transferred here was started through
//! [`ScopedTaskService::spawn_reapable`] under a reservation this operation
//! acquired *before* it took a credential, a working resource, a process, a
//! task, or any provider work. That pre-admission is what makes the transfer
//! non-fallible while the work is still unfinished: the host has already
//! committed the reap lane, so neither a shutdown race nor exhausted capacity
//! can refuse it. Refusal is therefore reachable only when the worker has
//! already ended — the reservation settles as the task's own body returns — and
//! joining an ended worker cannot block. [`transfer_or_join`] states that
//! reading explicitly rather than dropping the handle, because dropping an
//! unfinished handle is itself the synchronous join this module exists to
//! avoid.

use crate::sdk::bounded::HostBound;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    HostServices, JoinedTask, RuntimeFailure, ScopeId, ScopedTaskService, TaskReapReservation,
    TaskRelinquishOutcome,
};

type Slot = Arc<Mutex<Option<Box<dyn JoinedTask>>>>;

/// Reserves host authority to start and later transfer one task under `scope`.
///
/// This runs before the operation acquires anything, so an unsupported,
/// closing, or capacity-exhausted host refuses here rather than after a
/// credential, a resource, a process, or provider work already exists.
pub(crate) fn reserve_reap(
    services: &HostServices,
    scope: &ScopeId,
) -> Result<Box<dyn TaskReapReservation>, RuntimeFailure> {
    let service = services.task().ok_or_else(|| {
        crate::sdk::failure::failure(
            "swallowtail.claude-agent.sdk.task_service_missing",
            "Claude Agent SDK sidecar requires a host task service",
        )
    })?;
    service.reserve_reap(scope.clone())
}

/// Starts one guardian task under an already-granted reservation.
pub(crate) fn spawn_reserved(
    services: &HostServices,
    reservation: Box<dyn TaskReapReservation>,
    task: swallowtail_runtime::BoxFuture<'static, ()>,
) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
    let service = services.task().ok_or_else(|| {
        crate::sdk::failure::failure(
            "swallowtail.claude-agent.sdk.task_service_missing",
            "Claude Agent SDK sidecar requires a host task service",
        )
    })?;
    service.spawn_reapable(reservation, task)
}

/// What actually happened to one scoped task, kept distinct on purpose.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TaskEvidence {
    /// The task finished and was joined. The only join evidence here.
    Joined,
    /// The owning host accepted the unfinished task for its own reaping. Not a
    /// join, and never a stronger cleanup outcome.
    Relinquished,
    /// Neither happened: ordinary join-and-drop ownership stayed in place.
    Unresolved,
}

/// The exact host authority one scoped task was created under.
pub(crate) struct TaskOwner<'a> {
    services: &'a HostServices,
    expected_host: &'a ExecutionHostId,
    scope: &'a ScopeId,
}

impl<'a> TaskOwner<'a> {
    pub(crate) const fn new(
        services: &'a HostServices,
        expected_host: &'a ExecutionHostId,
        scope: &'a ScopeId,
    ) -> Self {
        Self {
            services,
            expected_host,
            scope,
        }
    }

    /// Transfers an unfinished task to its owning host, through the exact
    /// selected host and the exact scope it was spawned under. A refusal hands
    /// the task back: ownership is unchanged, and ordinary join-and-drop rules
    /// still apply to it.
    pub(crate) fn transfer(
        &self,
        task: Box<dyn JoinedTask>,
    ) -> Result<TaskRelinquishOutcome, Box<dyn JoinedTask>> {
        let mut held = Some(task);
        if self
            .services
            .require_execution_host(self.expected_host)
            .is_err()
        {
            return Err(held.take().expect("task was not transferred"));
        }
        let Some(service) = self.services.task() else {
            return Err(held.take().expect("task was not transferred"));
        };
        match ScopedTaskService::relinquish(service.as_ref(), self.scope, &mut held) {
            Ok(outcome) => Ok(outcome),
            Err(_) => Err(held.take().expect("a refused transfer returns the task")),
        }
    }
}

/// Resolves when the task in the slot reports finished, using only the trait's
/// non-blocking observation. Dropping this future leaves the handle in the
/// slot, so an expired wait never joins by dropping.
struct TaskFinished(Slot);

impl Future for TaskFinished {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let slot = self.0.lock().expect("SDK task join slot poisoned");
        let Some(task) = slot.as_ref() else {
            return Poll::Ready(());
        };
        if task.is_finished() {
            return Poll::Ready(());
        }
        task.register_waker(context.waker());
        // Registering can race the task's own final notification; the second
        // observation closes that race without sleeping or blocking.
        if task.is_finished() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

/// Hands an unfinished reservation-backed task to its owning host.
///
/// A refusal is not answered by dropping the handle. Under the pre-admitted
/// reservation the host cannot refuse work that is still running, so every
/// reachable refusal means the worker already ended; the ordinary join that
/// follows therefore observes a finished worker and cannot block the caller.
/// Only that join is reported as join evidence.
async fn transfer_or_join(owner: &TaskOwner<'_>, task: Box<dyn JoinedTask>) -> TaskEvidence {
    match owner.transfer(task) {
        Ok(TaskRelinquishOutcome::AcceptedForReap) => TaskEvidence::Relinquished,
        Err(task) => {
            if task.join().await.is_ok() {
                TaskEvidence::Joined
            } else {
                TaskEvidence::Unresolved
            }
        }
    }
}

/// Joins `task` inside `bounded`, or hands it to its owning host when the
/// caller's deadline arrives first.
pub(crate) async fn bounded_join(
    bounded: &HostBound,
    owner: &TaskOwner<'_>,
    task: Box<dyn JoinedTask>,
) -> TaskEvidence {
    let slot: Slot = Arc::new(Mutex::new(Some(task)));
    let waited = bounded.run(TaskFinished(Arc::clone(&slot))).await;
    let held = slot.lock().expect("SDK task join slot poisoned").take();
    let Some(task) = held else {
        return TaskEvidence::Unresolved;
    };
    if waited.is_none() {
        return transfer_or_join(owner, task).await;
    }
    // Finished: this join cannot block on task work.
    if task.join().await.is_ok() {
        TaskEvidence::Joined
    } else {
        TaskEvidence::Unresolved
    }
}

#[cfg(test)]
mod joins_tests;
