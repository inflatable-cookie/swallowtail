//! The fixture's scoped-task authority, modelled on the real local host.

use std::sync::{Arc, Mutex};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    BoxFuture, JoinedTask, RuntimeFailure, ScopeId, ScopedTaskService, TaskReapReservation,
    TaskRelinquishOutcome,
};

/// The fixture's scoped-task authority. It models the real local host: a
/// handle owns its worker thread, `join` blocks, dropping an unfinished handle
/// joins, and an unfinished handle can only leave the route through an exact
/// host, exact scope transfer backed by a reservation taken before the work
/// started. Acceptance moves the worker to a retained reaper owned by this
/// fixture, so nothing is ever discarded or detached.
pub(super) struct ThreadTaskService {
    execution_host_id: ExecutionHostId,
    reaper: Arc<FixtureReaper>,
}

/// The outer owner of transferred work, standing in for the host lifecycle
/// outside the task tree.
#[derive(Default)]
pub struct FixtureReaper {
    accepting: Mutex<bool>,
    relinquished: Arc<Mutex<Vec<String>>>,
    retained: Mutex<Vec<std::thread::JoinHandle<()>>>,
}

impl FixtureReaper {
    fn reserve(&self) -> Result<(), RuntimeFailure> {
        if !*self
            .accepting
            .lock()
            .expect("SDK fixture reaper lock poisoned")
        {
            return Err(fixture_task_failure());
        }
        Ok(())
    }

    fn retain(&self, scope: &ScopeId, worker: std::thread::JoinHandle<()>) {
        self.relinquished
            .lock()
            .expect("SDK fixture relinquish lock poisoned")
            .push(scope.as_str().to_owned());
        self.retained
            .lock()
            .expect("SDK fixture reaper lock poisoned")
            .push(worker);
    }

    /// Closes admission and joins every retained worker, exactly as the outer
    /// host owner does. Transferred work is joined here, not discarded.
    pub fn shutdown(&self) {
        *self
            .accepting
            .lock()
            .expect("SDK fixture reaper lock poisoned") = false;
        let retained = std::mem::take(
            &mut *self
                .retained
                .lock()
                .expect("SDK fixture reaper lock poisoned"),
        );
        for worker in retained {
            let _ = worker.join();
        }
    }

    /// Refuses every further reservation without shutting the lane, so a
    /// preflight refusal can be observed before any effect.
    pub fn close_admission(&self) {
        *self
            .accepting
            .lock()
            .expect("SDK fixture reaper lock poisoned") = false;
    }
}

impl ThreadTaskService {
    pub(super) fn new(
        execution_host_id: ExecutionHostId,
    ) -> (Self, Arc<Mutex<Vec<String>>>, Arc<FixtureReaper>) {
        let reaper = Arc::new(FixtureReaper {
            accepting: Mutex::new(true),
            relinquished: Arc::new(Mutex::new(Vec::new())),
            retained: Mutex::new(Vec::new()),
        });
        let relinquished = Arc::clone(&reaper.relinquished);
        (
            Self {
                execution_host_id,
                reaper: Arc::clone(&reaper),
            },
            relinquished,
            reaper,
        )
    }

    fn start(
        &self,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
        reserved: bool,
    ) -> Box<dyn JoinedTask> {
        let state = Arc::new(Mutex::new(TaskState::default()));
        let thread_state = Arc::clone(&state);
        // The worker handle is retained, never discarded.
        let worker = std::thread::spawn(move || {
            futures_executor::block_on(task);
            let mut state = thread_state.lock().expect("SDK fixture task lock poisoned");
            state.finished = true;
            if let Some(waiter) = state.waiter.take() {
                waiter.wake();
            }
        });
        Box::new(ThreadTask {
            state,
            worker: Some(worker),
            execution_host_id: self.execution_host_id.clone(),
            scope,
            reserved,
            reaper: Arc::clone(&self.reaper),
        })
    }
}

/// One fixture reservation. Opaque to the route, and bound to the exact host
/// and scope it was issued for.
#[derive(Debug)]
struct FixtureReservation {
    execution_host_id: ExecutionHostId,
    scope: ScopeId,
}

impl TaskReapReservation for FixtureReservation {
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any + Send> {
        self
    }
}

/// Completion state for one fixture task.
#[derive(Default)]
struct TaskState {
    finished: bool,
    waiter: Option<std::task::Waker>,
}

struct ThreadTask {
    state: Arc<Mutex<TaskState>>,
    worker: Option<std::thread::JoinHandle<()>>,
    execution_host_id: ExecutionHostId,
    scope: ScopeId,
    reserved: bool,
    reaper: Arc<FixtureReaper>,
}

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(self.start(scope, task, false))
    }

    fn reserve_reap(&self, scope: ScopeId) -> Result<Box<dyn TaskReapReservation>, RuntimeFailure> {
        self.reaper.reserve()?;
        Ok(Box::new(FixtureReservation {
            execution_host_id: self.execution_host_id.clone(),
            scope,
        }))
    }

    fn spawn_reapable(
        &self,
        reservation: Box<dyn TaskReapReservation>,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        let reservation = reservation
            .into_any()
            .downcast::<FixtureReservation>()
            .map_err(|_| fixture_task_failure())?;
        if reservation.execution_host_id != self.execution_host_id {
            return Err(fixture_task_failure());
        }
        Ok(self.start(reservation.scope.clone(), task, true))
    }

    fn relinquish(
        &self,
        scope: &ScopeId,
        task: &mut Option<Box<dyn JoinedTask>>,
    ) -> Result<TaskRelinquishOutcome, RuntimeFailure> {
        let held = task.as_deref_mut().ok_or_else(fixture_task_failure)?;
        held.relinquish_to_host(&self.execution_host_id, scope)?;
        drop(task.take());
        Ok(TaskRelinquishOutcome::AcceptedForReap)
    }
}

fn fixture_task_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "swallowtail.fixture_task.unavailable",
        "Fixture task ownership was already transferred",
    ))
}

impl JoinedTask for ThreadTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        // Blocking on purpose, exactly like the local host's own handle. The
        // caller's bound must be spent on the finished observation, never here.
        let worker = self.worker.take();
        Box::pin(async move {
            match worker {
                Some(worker) => worker.join().map_err(|_| fixture_task_failure()),
                None => Err(fixture_task_failure()),
            }
        })
    }

    fn is_finished(&self) -> bool {
        self.state
            .lock()
            .expect("SDK fixture task lock poisoned")
            .finished
    }

    fn register_waker(&self, waker: &std::task::Waker) {
        let mut state = self.state.lock().expect("SDK fixture task lock poisoned");
        if state.finished {
            waker.wake_by_ref();
        } else {
            state.waiter = Some(waker.clone());
        }
    }

    fn relinquish_to_host(
        &mut self,
        execution_host_id: &ExecutionHostId,
        scope: &ScopeId,
    ) -> Result<(), RuntimeFailure> {
        // Exact host and exact scope, like the local host. Anything else keeps
        // ordinary ownership rather than transferring it.
        if &self.execution_host_id != execution_host_id || &self.scope != scope {
            return Err(fixture_task_failure());
        }
        // An unreserved task cannot be upgraded through a late check, and a
        // finished task uses ordinary join.
        if !self.reserved || self.is_finished() {
            return Err(fixture_task_failure());
        }
        let worker = self.worker.take().ok_or_else(fixture_task_failure)?;
        self.reaper.retain(&self.scope, worker);
        Ok(())
    }
}

impl Drop for ThreadTask {
    fn drop(&mut self) {
        // Join-on-drop, exactly like the local host. A route that drops an
        // unfinished handle blocks here, which is the counterexample this
        // fixture must be able to reproduce.
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
