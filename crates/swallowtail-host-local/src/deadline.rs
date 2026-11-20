use crate::host::LocalProcessHost;
use async_io::Timer;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant, SystemTime};
use swallowtail_core::{CatalogTimestamp, SafeDiagnostic};
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, MonotonicInstant, RuntimeFailure, TimeService,
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
        Box::pin(DeadlineWait::new(self.monotonic_origin, deadline, now))
    }

    fn catalog_now(&self) -> Result<CatalogTimestamp, RuntimeFailure> {
        let elapsed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| observation_clock_failure())?;
        let seconds = i64::try_from(elapsed.as_secs()).map_err(|_| observation_clock_failure())?;
        CatalogTimestamp::new(seconds, elapsed.subsec_nanos())
            .map_err(|_| observation_clock_failure())
    }
}

fn observation_clock_failure() -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.local_host.observation_clock_failed",
        "Local host could not read the observation clock",
    ))
}

struct DeadlineWait {
    origin: Instant,
    deadline: Deadline,
    timer: Timer,
}

impl DeadlineWait {
    fn new(origin: Instant, deadline: Deadline, now: MonotonicInstant) -> Self {
        Self {
            origin,
            deadline,
            timer: timer_for(origin, deadline, now),
        }
    }
}

impl Future for DeadlineWait {
    type Output = DeadlineObservation;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match Pin::new(&mut self.timer).poll(context) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(_) => {
                    let now = monotonic_instant(self.origin);
                    if now >= self.deadline.instant() {
                        return Poll::Ready(DeadlineObservation::new(self.deadline, now));
                    }
                    self.timer = timer_for(self.origin, self.deadline, now);
                }
            }
        }
    }
}

fn timer_for(origin: Instant, deadline: Deadline, now: MonotonicInstant) -> Timer {
    let target = origin
        .checked_add(Duration::from_nanos(deadline.instant().ticks()))
        .or_else(|| {
            let remaining =
                Duration::from_nanos(deadline.instant().ticks().saturating_sub(now.ticks()));
            Instant::now().checked_add(remaining)
        });
    match target {
        Some(target) => Timer::at(target),
        None => Timer::after(Duration::from_secs(86_400 * 365 * 50)),
    }
}

fn monotonic_instant(origin: Instant) -> MonotonicInstant {
    let ticks = u64::try_from(origin.elapsed().as_nanos()).unwrap_or(u64::MAX);
    MonotonicInstant::from_ticks(ticks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::task::Waker;

    #[test]
    fn deadline_wait_completes_and_drops_without_thread_churn() {
        let origin = Instant::now();
        let now = monotonic_instant(origin);
        let short = Deadline::at(MonotonicInstant::from_ticks(
            now.ticks().saturating_add(5_000_000),
        ));
        let observation = futures_executor::block_on(DeadlineWait::new(origin, short, now));
        assert_eq!(observation.deadline(), short);
        assert!(observation.observed_at() >= short.instant());

        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let started = Instant::now();
        for _ in 0..500 {
            let current = monotonic_instant(origin);
            let distant = Deadline::at(MonotonicInstant::from_ticks(
                current.ticks().saturating_add(60_000_000_000),
            ));
            let mut wait = DeadlineWait::new(origin, distant, current);
            assert!(Pin::new(&mut wait).poll(&mut context).is_pending());
            drop(wait);
        }
        assert!(started.elapsed() < Duration::from_secs(1));

        // Saturation/overflow deadlines do not panic on timer setup or drop.
        let overflow = Deadline::at(MonotonicInstant::from_ticks(u64::MAX));
        let mut wait = DeadlineWait::new(origin, overflow, monotonic_instant(origin));
        assert!(Pin::new(&mut wait).poll(&mut context).is_pending());
    }
}
