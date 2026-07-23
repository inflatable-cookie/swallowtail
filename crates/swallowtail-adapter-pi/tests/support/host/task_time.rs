use super::{FixtureHost, Shared, fixture_failure};
use futures_executor::block_on;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, JoinedTask, MonotonicInstant, RuntimeFailure,
    ScopeId, ScopedTaskService, TimeService,
};

impl TimeService for FixtureHost {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(
            self.shared
                .time
                .lock()
                .expect("Pi fixture time lock poisoned")
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

impl FixtureHost {
    pub fn advance_time(&self, ticks: u64) {
        let waiters = {
            let mut time = self
                .shared
                .time
                .lock()
                .expect("Pi fixture time lock poisoned");
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
            .expect("Pi fixture time lock poisoned");
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
struct ThreadTask(Mutex<Option<JoinHandle<()>>>);

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask(Mutex::new(Some(thread::spawn(
            move || block_on(task),
        ))))))
    }
}

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            self.0
                .lock()
                .expect("Pi fixture task lock poisoned")
                .take()
                .expect("Pi fixture task joins once")
                .join()
                .map_err(|_| fixture_failure())
        })
    }
}
