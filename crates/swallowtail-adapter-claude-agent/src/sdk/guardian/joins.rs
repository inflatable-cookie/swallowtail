//! Joining a host task inside a caller deadline, without ever blocking the
//! caller's executor thread.
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
//! A host that offers neither the finished observation nor relinquishment
//! leaves ordinary join-and-drop ownership in place, and the result is reported
//! as neither joined nor transferred. That is the fail-closed reading: without
//! the observation there is no evidence the task ended, and this route never
//! reports cleanup truth the host cannot support.

use crate::sdk::bounded::HostBound;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    HostServices, JoinedTask, ScopeId, ScopedTaskService, TaskRelinquishOutcome,
};

type Slot = Arc<Mutex<Option<Box<dyn JoinedTask>>>>;

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

impl TaskEvidence {
    /// True only for an actual join.
    pub(crate) const fn joined(self) -> bool {
        matches!(self, Self::Joined)
    }
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
    if waited.is_none() {
        let Some(task) = held else {
            return TaskEvidence::Unresolved;
        };
        return match owner.transfer(task) {
            Ok(TaskRelinquishOutcome::AcceptedForReap) => TaskEvidence::Relinquished,
            // The host kept ownership where it was. Ordinary join-and-drop
            // rules apply to the handle, and nothing here is join evidence.
            Err(_) => TaskEvidence::Unresolved,
        };
    }
    // Finished: this join cannot block on task work.
    let Some(task) = held else {
        return TaskEvidence::Unresolved;
    };
    if task.join().await.is_ok() {
        TaskEvidence::Joined
    } else {
        TaskEvidence::Unresolved
    }
}

#[cfg(test)]
mod joins_tests;
