use super::LocalScopedTaskService;
use super::joined::{LocalJoinedTask, task_failure};
use super::reaper::ReapPermit;
use std::any::Any;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{BoxFuture, RuntimeFailure, ScopeId, TaskReapReservation};

pub(super) struct LocalReapReservation {
    pub(super) execution_host_id: ExecutionHostId,
    pub(super) scope: ScopeId,
    pub(super) permit: Option<ReapPermit>,
}

impl LocalReapReservation {
    pub(super) fn bind(
        mut self,
        service: &LocalScopedTaskService,
        task: BoxFuture<'static, ()>,
    ) -> Result<LocalJoinedTask, RuntimeFailure> {
        if self.execution_host_id != service.execution_host_id {
            return Err(task_failure(
                "swallowtail.local_task.reap_reservation_host_mismatch",
                "Reap reservation belongs to a different execution host",
            ));
        }
        let reaper = service.reaper.upgrade().ok_or_else(|| {
            task_failure(
                "swallowtail.local_task.reap_reservation_unsupported",
                "Local task service has no host-owned reap lifecycle",
            )
        })?;
        let permit = self.permit.take().ok_or_else(|| {
            task_failure(
                "swallowtail.local_task.reap_reservation_released",
                "Reap reservation was already released or bound",
            )
        })?;
        if !permit.belongs_to(&reaper) {
            return Err(task_failure(
                "swallowtail.local_task.reap_reservation_host_mismatch",
                "Reap reservation belongs to a different local host lifecycle",
            ));
        }
        LocalJoinedTask::spawn_reapable(
            service.execution_host_id.clone(),
            self.scope.clone(),
            task,
            permit,
        )
    }
}

impl std::fmt::Debug for LocalReapReservation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("LocalReapReservation")
            .finish_non_exhaustive()
    }
}

impl TaskReapReservation for LocalReapReservation {
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send> {
        self
    }
}
