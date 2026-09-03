use super::super::host::{SdkFixtureHost, Shared};
use std::sync::Arc;
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, MonotonicInstant, TimeService,
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
