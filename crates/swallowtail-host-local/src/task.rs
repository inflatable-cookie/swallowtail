use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, Weak};
use std::task::Waker;
use std::thread::{self, JoinHandle};
use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{
    BoxFuture, JoinedTask, RuntimeFailure, ScopeId, ScopedTaskService, TaskRelinquishOutcome,
};

mod reaper;

use reaper::ReaperSupervisor;

/// Per-host task service whose returned handles always own their worker thread.
///
/// Dropping a handle joins the worker thread; the task still completes and
/// its effects stay deterministic, but the dropping consumer thread blocks
/// until the task ends. A task that waits on an external condition can block
/// its dropper indefinitely; consumers that need a bounded shutdown must
/// bound the task itself (for example through the host deadline service) or
/// transfer an unfinished handle through [`ScopedTaskService::relinquish`]
/// before dropping it. Service clones share ownership of every accepted
/// task's reaper; dropping the final clone joins all retained reapers.
/// Acceptance for reap is not join evidence.
#[derive(Clone)]
pub struct LocalScopedTaskService {
    execution_host_id: ExecutionHostId,
    reaper: SharedReaper,
}

impl LocalScopedTaskService {
    /// Creates a scoped task service for one exact execution host.
    #[must_use]
    pub const fn new(execution_host_id: ExecutionHostId) -> Self {
        Self {
            execution_host_id,
            reaper: SharedReaper::new(),
        }
    }

    /// Returns the execution host identity bound to spawned tasks.
    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    fn reaper(&self) -> &Arc<ReaperSupervisor> {
        self.reaper.get()
    }

    fn spawn_task(
        &self,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<LocalJoinedTask, RuntimeFailure> {
        let signal = Arc::new(JoinSignal::default());
        let worker_signal = Arc::clone(&signal);
        let worker = thread::Builder::new()
            .name("swallowtail-local-task".to_owned())
            .spawn(move || {
                let _notification = NotifyOnDrop(worker_signal);
                futures_executor::block_on(task);
            })
            .map_err(|_| {
                task_failure(
                    "swallowtail.local_task.spawn_failed",
                    "Local task could not be started",
                )
            })?;
        Ok(LocalJoinedTask {
            execution_host_id: self.execution_host_id.clone(),
            scope,
            worker: Some(worker),
            signal,
            reaped: Arc::new(AtomicBool::new(false)),
            reaper: Arc::downgrade(self.reaper()),
        })
    }
}

struct SharedReaper(OnceLock<Arc<ReaperSupervisor>>);

impl SharedReaper {
    const fn new() -> Self {
        Self(OnceLock::new())
    }

    fn get(&self) -> &Arc<ReaperSupervisor> {
        self.0.get_or_init(|| Arc::new(ReaperSupervisor::default()))
    }
}

impl Clone for SharedReaper {
    fn clone(&self) -> Self {
        Self(OnceLock::from(Arc::clone(self.get())))
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

struct LocalJoinedTask {
    execution_host_id: ExecutionHostId,
    scope: ScopeId,
    worker: Option<JoinHandle<()>>,
    signal: Arc<JoinSignal>,
    reaped: Arc<AtomicBool>,
    reaper: Weak<ReaperSupervisor>,
}

impl JoinedTask for LocalJoinedTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let worker = self.worker.take();
        let reaped = Arc::clone(&self.reaped);
        Box::pin(async move { reap_worker(worker, &reaped) })
    }

    fn is_finished(&self) -> bool {
        self.worker
            .as_ref()
            .is_none_or(std::thread::JoinHandle::is_finished)
    }

    fn register_waker(&self, waker: &Waker) {
        self.signal.register(waker);
    }

    fn relinquish_to_host(
        &mut self,
        execution_host_id: &ExecutionHostId,
        scope: &ScopeId,
    ) -> Result<(), RuntimeFailure> {
        if &self.execution_host_id != execution_host_id {
            return Err(task_failure(
                "swallowtail.local_task.execution_host_mismatch",
                "Local task belongs to a different execution host",
            ));
        }
        if &self.scope != scope {
            return Err(task_failure(
                "swallowtail.local_task.scope_mismatch",
                "Local task belongs to a different operation scope",
            ));
        }
        let worker = self.worker.as_ref().ok_or_else(|| {
            task_failure(
                "swallowtail.local_task.already_relinquished",
                "Local task ownership was already transferred",
            )
        })?;
        if worker.is_finished() {
            return Err(task_failure(
                "swallowtail.local_task.already_finished",
                "Finished local tasks must use ordinary join",
            ));
        }

        let reaper = self.reaper.upgrade().ok_or_else(|| {
            task_failure(
                "swallowtail.local_task.reaper_unavailable",
                "Owning local task service is no longer available",
            )
        })?;
        let worker = self.worker.take().ok_or_else(|| {
            task_failure(
                "swallowtail.local_task.already_relinquished",
                "Local task ownership was already transferred",
            )
        })?;
        if let Err((error, worker)) = reaper.accept(worker, Arc::clone(&self.reaped)) {
            self.worker = Some(worker);
            return Err(error);
        }
        Ok(())
    }
}

impl Drop for LocalJoinedTask {
    fn drop(&mut self) {
        if let Some(worker) = self.worker.take() {
            // Joining on drop is deliberate: a dropped task still completes
            // and its effects stay deterministic. The flip side is that the
            // consumer thread blocks until the task ends; see the service
            // docs for the bounded-shutdown guidance. Bounding or detaching
            // here would let a dropped task keep running silently.
            let _ = reap_worker(Some(worker), &self.reaped);
        }
    }
}

struct NotifyOnDrop(Arc<JoinSignal>);

impl Drop for NotifyOnDrop {
    fn drop(&mut self) {
        self.0.notify();
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

fn reap_worker(worker: Option<JoinHandle<()>>, reaped: &AtomicBool) -> Result<(), RuntimeFailure> {
    let outcome = join_worker(worker);
    reaped.store(true, Ordering::Release);
    outcome
}

fn task_failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

#[derive(Default)]
struct JoinSignal {
    finished: AtomicBool,
    wakers: Mutex<Vec<Waker>>,
}

impl JoinSignal {
    fn notify(&self) {
        self.finished.store(true, Ordering::Release);
        let wakers = self
            .wakers
            .lock()
            .expect("local task waker lock poisoned")
            .drain(..)
            .collect::<Vec<_>>();
        for waker in wakers {
            waker.wake();
        }
    }

    fn register(&self, waker: &Waker) {
        if self.finished.load(Ordering::Acquire) {
            waker.wake_by_ref();
            return;
        }
        let mut registered = self.wakers.lock().expect("local task waker lock poisoned");
        if !registered.iter().any(|current| current.will_wake(waker)) {
            registered.push(waker.clone());
        }
        if self.finished.load(Ordering::Acquire) {
            let wakers = registered.drain(..).collect::<Vec<_>>();
            drop(registered);
            for waker in wakers {
                waker.wake();
            }
        }
    }
}

#[cfg(test)]
mod tests;
