use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, MonotonicInstant, TimeService,
};

pub struct PendingTimeService;

impl TimeService for PendingTimeService {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(PendingDeadline)
    }
}

struct PendingDeadline;

impl Future for PendingDeadline {
    type Output = DeadlineObservation;

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

pub struct ImmediateTimeService;

impl TimeService for ImmediateTimeService {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(1_000)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
    }
}

pub struct ControllableTimeService {
    state: std::sync::Arc<DeadlineFire>,
}

struct DeadlineFire {
    fired: std::sync::atomic::AtomicBool,
    waker: std::sync::Mutex<Option<std::task::Waker>>,
}

impl ControllableTimeService {
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(DeadlineFire {
                fired: std::sync::atomic::AtomicBool::new(false),
                waker: std::sync::Mutex::new(None),
            }),
        }
    }

    pub fn fire(&self) {
        self.state
            .fired
            .store(true, std::sync::atomic::Ordering::SeqCst);
        if let Some(waker) = self
            .state
            .waker
            .lock()
            .expect("deadline waker lock is available")
            .take()
        {
            waker.wake();
        }
    }
}

impl TimeService for ControllableTimeService {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        let state = std::sync::Arc::clone(&self.state);
        Box::pin(std::future::poll_fn(move |context| {
            if state.fired.load(std::sync::atomic::Ordering::SeqCst) {
                return Poll::Ready(DeadlineObservation::new(deadline, deadline.instant()));
            }
            *state
                .waker
                .lock()
                .expect("deadline waker lock is available") = Some(context.waker().clone());
            if state.fired.load(std::sync::atomic::Ordering::SeqCst) {
                Poll::Ready(DeadlineObservation::new(deadline, deadline.instant()))
            } else {
                Poll::Pending
            }
        }))
    }
}
