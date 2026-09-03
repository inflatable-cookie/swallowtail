use std::sync::{Arc, Weak};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    BoxFuture, JoinedTask, RuntimeFailure, ScopeId, ScopedTaskService, TaskReapReservation,
    TaskRelinquishOutcome,
};

mod joined;
mod reaper;
mod reservation;

use joined::{LocalJoinedTask, task_failure};
use reaper::ReaperSupervisor;
use reservation::LocalReapReservation;

pub(crate) const DEFAULT_TASK_REAP_CAPACITY: usize = 64;

/// Per-host task service whose returned handles always own their worker thread.
///
/// Dropping a handle joins the worker thread; the task still completes and
/// its effects stay deterministic, but the dropping consumer thread blocks
/// until the task ends. A task that waits on an external condition can block
/// its dropper indefinitely; consumers that need a bounded shutdown must
/// bound the task itself (for example through the host deadline service) or
/// reserve reap authority before effects, bind it with
/// [`ScopedTaskService::spawn_reapable`], and transfer the unfinished handle
/// through [`ScopedTaskService::relinquish`] before dropping it. Reservations
/// are available only when the service was created by an owning host
/// composition. Service clones carry weak handoff access only; the outer host
/// lifecycle must call its explicit reaper shutdown after operation work.
/// Acceptance for reap is not join evidence.
#[derive(Clone)]
pub struct LocalScopedTaskService {
    execution_host_id: ExecutionHostId,
    reaper: Weak<ReaperSupervisor>,
}

impl LocalScopedTaskService {
    /// Creates an ordinary scoped task service for one exact execution host.
    ///
    /// This standalone service preserves joined-task behavior but has no outer
    /// lifecycle owner, so reservation and relinquishment fail closed. Use the
    /// local host composition when host-owned reap is required.
    #[must_use]
    pub const fn new(execution_host_id: ExecutionHostId) -> Self {
        Self {
            execution_host_id,
            reaper: Weak::new(),
        }
    }

    /// Returns the execution host identity bound to spawned tasks.
    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    pub(crate) fn with_reaper_owner(
        execution_host_id: ExecutionHostId,
        capacity: usize,
    ) -> (Self, LocalTaskReaperOwner) {
        let owner = LocalTaskReaperOwner::new(capacity);
        let service = Self {
            execution_host_id,
            reaper: Arc::downgrade(&owner.reaper),
        };
        (service, owner)
    }

    fn spawn_task(
        &self,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<LocalJoinedTask, RuntimeFailure> {
        LocalJoinedTask::spawn(self.execution_host_id.clone(), scope, task)
    }
}

#[derive(Clone)]
pub(crate) struct LocalTaskReaperOwner {
    reaper: Arc<ReaperSupervisor>,
}

impl LocalTaskReaperOwner {
    fn new(capacity: usize) -> Self {
        Self {
            reaper: Arc::new(ReaperSupervisor::new(capacity)),
        }
    }

    pub(crate) fn shutdown(&self) -> Result<(), RuntimeFailure> {
        self.reaper.shutdown()
    }
}

impl ScopedTaskService for LocalScopedTaskService {
    fn spawn(
        &self,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        self.spawn_task(scope, task)
            .map(|task| Box::new(task) as Box<dyn JoinedTask>)
    }

    fn reserve_reap(&self, scope: ScopeId) -> Result<Box<dyn TaskReapReservation>, RuntimeFailure> {
        let reaper = self.reaper.upgrade().ok_or_else(|| {
            task_failure(
                "swallowtail.local_task.reap_reservation_unsupported",
                "Local task service has no host-owned reap lifecycle",
            )
        })?;
        let permit = reaper.reserve()?;
        Ok(Box::new(LocalReapReservation {
            execution_host_id: self.execution_host_id.clone(),
            scope,
            permit: Some(permit),
        }))
    }

    fn spawn_reapable(
        &self,
        reservation: Box<dyn TaskReapReservation>,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        let reservation = reservation
            .into_any()
            .downcast::<LocalReapReservation>()
            .map_err(|_| {
                task_failure(
                    "swallowtail.local_task.reap_reservation_foreign",
                    "Reap reservation was not issued by the local task host",
                )
            })?;
        (*reservation)
            .bind(self, task)
            .map(|task| Box::new(task) as Box<dyn JoinedTask>)
    }

    fn relinquish(
        &self,
        scope: &ScopeId,
        task: &mut Option<Box<dyn JoinedTask>>,
    ) -> Result<TaskRelinquishOutcome, RuntimeFailure> {
        let joined_task = task.as_deref_mut().ok_or_else(|| {
            task_failure(
                "swallowtail.local_task.already_relinquished",
                "Local task ownership was already transferred",
            )
        })?;
        joined_task.relinquish_to_host(&self.execution_host_id, scope)?;
        drop(task.take());
        Ok(TaskRelinquishOutcome::AcceptedForReap)
    }
}

#[cfg(test)]
mod tests;
