use crate::host::LocalProcessHost;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, MonotonicInstant, TimeService,
};

impl TimeService for LocalProcessHost {
    fn now(&self) -> MonotonicInstant {
        monotonic_instant(self.monotonic_origin)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        let now = self.now();
        if now >= deadline.instant() {
            return Box::pin(async move { DeadlineObservation::new(deadline, now) });
        }
        Box::pin(DeadlineWait::new(self.monotonic_origin, deadline))
    }
}

struct WaitState {
    cancelled: bool,
    observation: Option<DeadlineObservation>,
    waker: Option<Waker>,
}

struct DeadlineWait {
    shared: Arc<(Mutex<WaitState>, Condvar)>,
    worker: Option<JoinHandle<()>>,
}

impl DeadlineWait {
    fn new(origin: Instant, deadline: Deadline) -> Self {
        let shared = Arc::new((
            Mutex::new(WaitState {
                cancelled: false,
                observation: None,
                waker: None,
            }),
            Condvar::new(),
        ));
        let worker_shared = Arc::clone(&shared);
        let worker = std::thread::spawn(move || observe_deadline(origin, deadline, worker_shared));
        Self {
            shared,
            worker: Some(worker),
        }
    }
}

impl Future for DeadlineWait {
    type Output = DeadlineObservation;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let (state, _) = &*self.shared;
        let mut state = state.lock().unwrap_or_else(|error| error.into_inner());
        if let Some(observation) = state.observation {
            Poll::Ready(observation)
        } else {
            if !state
                .waker
                .as_ref()
                .is_some_and(|waker| waker.will_wake(context.waker()))
            {
                state.waker = Some(context.waker().clone());
            }
            Poll::Pending
        }
    }
}

impl Drop for DeadlineWait {
    fn drop(&mut self) {
        let (state, wake) = &*self.shared;
        {
            let mut state = state.lock().unwrap_or_else(|error| error.into_inner());
            state.cancelled = true;
            wake.notify_all();
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn observe_deadline(origin: Instant, deadline: Deadline, shared: Arc<(Mutex<WaitState>, Condvar)>) {
    let (state, wake) = &*shared;
    let mut state = state.lock().unwrap_or_else(|error| error.into_inner());
    loop {
        if state.cancelled {
            return;
        }
        let now = monotonic_instant(origin);
        if now >= deadline.instant() {
            let observation = DeadlineObservation::new(deadline, monotonic_instant(origin));
            state.observation = Some(observation);
            let waker = state.waker.take();
            drop(state);
            if let Some(waker) = waker {
                waker.wake();
            }
            return;
        }
        let remaining = Duration::from_nanos(deadline.instant().ticks() - now.ticks());
        let (next, _) = wake
            .wait_timeout(state, remaining)
            .unwrap_or_else(|error| error.into_inner());
        state = next;
    }
}

fn monotonic_instant(origin: Instant) -> MonotonicInstant {
    let ticks = u64::try_from(origin.elapsed().as_nanos()).unwrap_or(u64::MAX);
    MonotonicInstant::from_ticks(ticks)
}
