use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Waker;
use std::thread::{self, JoinHandle};
use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{BoxFuture, JoinedTask, RuntimeFailure, ScopeId, ScopedTaskService};

/// Per-host task service whose returned handles always own their worker thread.
///
/// Dropping a handle joins the worker thread; the task still completes and
/// its effects stay deterministic, but the dropping consumer thread blocks
/// until the task ends. A task that waits on an external condition can block
/// its dropper indefinitely; consumers that need a bounded shutdown must
/// bound the task itself (for example through the host deadline service) or
/// join explicitly before dropping.
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
        Ok(Box::new(LocalJoinedTask {
            _scope: scope,
            worker: Some(worker),
            signal,
        }))
    }
}

struct LocalJoinedTask {
    _scope: ScopeId,
    worker: Option<JoinHandle<()>>,
    signal: Arc<JoinSignal>,
}

impl JoinedTask for LocalJoinedTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let worker = self.worker.take();
        Box::pin(async move { join_worker(worker) })
    }

    fn is_finished(&self) -> bool {
        self.worker
            .as_ref()
            .is_none_or(std::thread::JoinHandle::is_finished)
    }

    fn register_waker(&self, waker: &Waker) {
        self.signal.register(waker);
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
            let _ = worker.join();
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
