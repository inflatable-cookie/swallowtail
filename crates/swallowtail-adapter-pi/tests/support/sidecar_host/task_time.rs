use super::{Shared, SidecarFixtureHost, fixture_failure};
use futures_channel::oneshot;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, JoinedTask, MonotonicInstant, RuntimeFailure,
    ScopeId, ScopedTaskService, TimeService,
};

impl TimeService for SidecarFixtureHost {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(
            self.shared
                .time
                .lock()
                .expect("sidecar fixture time lock poisoned")
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

impl SidecarFixtureHost {
    pub fn advance_time(&self, ticks: u64) {
        let waiters = {
            let mut time = self
                .shared
                .time
                .lock()
                .expect("sidecar fixture time lock poisoned");
            time.now = ticks;
            time.fire_through = Some(ticks);
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
            .expect("sidecar fixture time lock poisoned");
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

pub(super) struct ThreadTaskService {
    shared: Arc<Shared>,
    reject_deadline_spawn: bool,
}
struct ThreadTask {
    thread: std::thread::JoinHandle<()>,
    completed: oneshot::Receiver<()>,
}

impl ThreadTaskService {
    pub(super) fn new(shared: Arc<Shared>, reject_deadline_spawn: bool) -> Self {
        Self {
            shared,
            reject_deadline_spawn,
        }
    }
}

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        let spawn = self.shared.task_spawns.fetch_add(1, Ordering::SeqCst) + 1;
        if self.reject_deadline_spawn && spawn == 2 {
            return Err(fixture_failure());
        }
        let (finished, completed) = oneshot::channel();
        let thread = std::thread::spawn(move || {
            futures_executor::block_on(task);
            let _ = finished.send(());
        });
        Ok(Box::new(ThreadTask { thread, completed }))
    }
}

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            // Let the caller's cleanup deadline advance while the fake pump is held.
            let _ = self.completed.await;
            self.thread.join().map_err(|_| fixture_failure())
        })
    }
}
