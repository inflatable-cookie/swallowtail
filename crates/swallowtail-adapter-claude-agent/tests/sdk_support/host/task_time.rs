use super::super::host::{SdkFixtureHost, Shared};
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, JoinedTask, MonotonicInstant, RuntimeFailure,
    ScopeId, ScopedTaskService, TimeService,
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

pub(super) struct ThreadTaskService;

/// Completion state for one fixture task.
///
/// The join must be a real future: a guard task that hangs on a stalled host
/// service has to be race-able against the caller's deadline, and a blocking
/// `JoinHandle::join` inside `poll` would deadlock the very bound under test.
#[derive(Default)]
struct TaskState {
    finished: bool,
    waiter: Option<std::task::Waker>,
}

struct ThreadTask(Arc<Mutex<TaskState>>);

struct ThreadTaskJoin(Arc<Mutex<TaskState>>);

impl std::future::Future for ThreadTaskJoin {
    type Output = Result<(), RuntimeFailure>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut state = self.0.lock().expect("SDK fixture task lock poisoned");
        if state.finished {
            std::task::Poll::Ready(Ok(()))
        } else {
            state.waiter = Some(context.waker().clone());
            std::task::Poll::Pending
        }
    }
}

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
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
        Ok(Box::new(ThreadTask(state)))
    }
}

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(ThreadTaskJoin(self.0))
    }
}
