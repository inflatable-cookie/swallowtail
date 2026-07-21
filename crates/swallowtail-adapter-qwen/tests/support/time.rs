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
