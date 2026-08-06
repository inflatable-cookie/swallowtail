use std::thread::{self, JoinHandle};
use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{BoxFuture, JoinedTask, RuntimeFailure, ScopeId, ScopedTaskService};

/// Per-host task service whose returned handles always own their worker thread.
#[derive(Clone)]
pub struct LocalScopedTaskService {
    execution_host_id: ExecutionHostId,
}

impl LocalScopedTaskService {
    /// Creates a scoped task service for one exact execution host.
    #[must_use]
    pub const fn new(execution_host_id: ExecutionHostId) -> Self {
        Self { execution_host_id }
    }

    /// Returns the execution host identity bound to spawned tasks.
    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }
}

impl ScopedTaskService for LocalScopedTaskService {
    fn spawn(
        &self,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        let worker = thread::Builder::new()
            .name("swallowtail-local-task".to_owned())
            .spawn(move || futures_executor::block_on(task))
            .map_err(|_| {
                task_failure(
                    "swallowtail.local_task.spawn_failed",
                    "Local task could not be started",
                )
            })?;
        Ok(Box::new(LocalJoinedTask {
            _scope: scope,
            worker: Some(worker),
        }))
    }
}

struct LocalJoinedTask {
    _scope: ScopeId,
    worker: Option<JoinHandle<()>>,
}

impl JoinedTask for LocalJoinedTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let worker = self.worker.take();
        Box::pin(async move { join_worker(worker) })
    }
}

impl Drop for LocalJoinedTask {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn join_worker(worker: Option<JoinHandle<()>>) -> Result<(), RuntimeFailure> {
    let worker = worker.ok_or_else(|| {
        task_failure(
            "swallowtail.local_task.already_joined",
            "Local task was already joined",
        )
    })?;
    worker.join().map_err(|_| {
        task_failure(
            "swallowtail.local_task.panicked",
            "Local task failed while executing",
        )
    })
}

fn task_failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}
