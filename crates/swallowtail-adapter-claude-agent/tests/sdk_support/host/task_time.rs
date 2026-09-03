use super::super::host::{SdkFixtureHost, Shared};
use std::sync::{Arc, Mutex};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, JoinedTask, MonotonicInstant, RuntimeFailure,
    ScopeId, ScopedTaskService, TaskRelinquishOutcome, TimeService,
};

impl TimeService for SdkFixtureHost {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(
            self.shared
                .time
                .lock()
                .expect("SDK fixture time lock poisoned")
                .now,
        )
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(DeadlineFuture {
            shared: Arc::clone(&self.shared),
            deadline,
        })
    }
}

impl SdkFixtureHost {
    /// Fires every armed deadline without moving the clock past them.
    ///
    /// A caller bound then observes expiry inside its stages while the outer
    /// public bound still returns this route's own outcome, which is how a
    /// stalled stage is distinguished from a blown public deadline.
    pub fn fire_deadlines(&self) {
        let waiters = {
            let mut time = self
                .shared
                .time
                .lock()
                .expect("SDK fixture time lock poisoned");
            time.fire_through = Some(u64::MAX);
            std::mem::take(&mut time.waiters)
        };
        for waiter in waiters {
            waiter.wake();
        }
    }

    /// Waits until the sidecar has been sent `command`, bounded so a missing
    /// write fails the test instead of hanging it.
    pub fn wait_for_command(&self, command: &str) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        loop {
            let seen = self
                .shared
                .process
                .lock()
                .expect("SDK fixture state lock poisoned")
                .input
                .iter()
                .any(|value| value["command"] == command);
            if seen {
                return;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "sidecar never received {command}"
            );
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    /// Blocks until the sidecar received a record the predicate accepts, so a
    /// side effect a host task performs on its own thread is observed rather
    /// than raced.
    pub fn wait_for_input(&self, what: &str, accept: impl Fn(&serde_json::Value) -> bool) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        loop {
            let seen = self
                .shared
                .process
                .lock()
                .expect("SDK fixture state lock poisoned")
                .input
                .iter()
                .any(&accept);
            if seen {
                return;
            }
            assert!(
                std::time::Instant::now() < deadline,
                "sidecar never received {what}"
            );
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    /// Fires every host deadline and moves the clock past them, so even the
    /// outer public cleanup bound observes expiry.
    pub fn advance_time(&self) {
        let waiters = {
            let mut time = self
                .shared
                .time
                .lock()
                .expect("SDK fixture time lock poisoned");
            time.now = u64::MAX;
            time.fire_through = Some(u64::MAX);
            std::mem::take(&mut time.waiters)
        };
        for waiter in waiters {
            waiter.wake();
        }
    }
}

struct DeadlineFuture {
    shared: Arc<Shared>,
    deadline: Deadline,
}

impl std::future::Future for DeadlineFuture {
    type Output = DeadlineObservation;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut time = self
            .shared
            .time
            .lock()
            .expect("SDK fixture time lock poisoned");
        if time
            .fire_through
            .is_some_and(|maximum| self.deadline.instant().ticks() <= maximum)
        {
            std::task::Poll::Ready(DeadlineObservation::new(
                self.deadline,
                MonotonicInstant::from_ticks(time.now),
            ))
        } else {
            time.waiters.push(context.waker().clone());
            std::task::Poll::Pending
        }
    }
}

/// The fixture's scoped-task authority. It models the real host: an unfinished
/// handle can only leave the route through exact-host and exact-scope
/// relinquishment, and acceptance is ownership transfer, never a join.
pub(super) struct ThreadTaskService {
    execution_host_id: ExecutionHostId,
    relinquished: Arc<Mutex<Vec<String>>>,
}

impl ThreadTaskService {
    pub(super) fn new(execution_host_id: ExecutionHostId) -> (Self, Arc<Mutex<Vec<String>>>) {
        let relinquished = Arc::new(Mutex::new(Vec::new()));
        (
            Self {
                execution_host_id,
                relinquished: Arc::clone(&relinquished),
            },
            relinquished,
        )
    }
}

/// Completion state for one fixture task.
///
/// `join` here is deliberately blocking, exactly like the local host's own
/// handle: the only non-blocking observation a fixture task offers is
/// `is_finished`/`register_waker`. A route that polls a join before the task
/// reports finished fails loudly here instead of passing against a friendlier
/// fake than production.
#[derive(Default)]
struct TaskState {
    finished: bool,
    waiter: Option<std::task::Waker>,
}

struct ThreadTask {
    state: Arc<Mutex<TaskState>>,
    execution_host_id: ExecutionHostId,
    scope: ScopeId,
    relinquished: Arc<Mutex<Vec<String>>>,
}

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        let state = Arc::new(Mutex::new(TaskState::default()));
        let thread_state = Arc::clone(&state);
        std::thread::spawn(move || {
            futures_executor::block_on(task);
            let mut state = thread_state.lock().expect("SDK fixture task lock poisoned");
            state.finished = true;
            if let Some(waiter) = state.waiter.take() {
                waiter.wake();
            }
        });
        Ok(Box::new(ThreadTask {
            state,
            execution_host_id: self.execution_host_id.clone(),
            scope,
            relinquished: Arc::clone(&self.relinquished),
        }))
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
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            // Blocking on purpose. The bound must be spent on the finished
            // observation, never on this.
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
            while !self.is_finished() {
                assert!(
                    std::time::Instant::now() < deadline,
                    "a fixture task join blocked: the route polled join before the task finished"
                );
                std::thread::yield_now();
            }
            Ok(())
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
        if self.is_finished() {
            return Err(fixture_task_failure());
        }
        self.relinquished
            .lock()
            .expect("SDK fixture relinquish lock poisoned")
            .push(self.scope.as_str().to_owned());
        Ok(())
    }
}
