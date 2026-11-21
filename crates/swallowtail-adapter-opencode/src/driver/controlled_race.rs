/// The control signal that interrupted a provider request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Interrupt {
    Cancelled,
    Deadline,
}

/// What a race does once a control signal fires.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OnInterrupt {
    /// Return at once. The caller reports the interrupt and drops the work,
    /// after the abort flag has told the transport to stop.
    Abandon,
    /// Set the abort flag and keep polling the work to completion, so the
    /// caller can still observe its result.
    Drain,
}

/// The outcome of one request raced against its controls.
struct Race<T> {
    /// The work's value, or `None` when an [`OnInterrupt::Abandon`] race
    /// was interrupted before the work completed.
    value: Option<T>,
    /// The first control signal that fired, if any.
    interrupt: Option<Interrupt>,
}

/// Polls `work` until it completes or a control signal fires.
///
/// Completed work always wins over a control observed on the same poll, so a
/// fast response is never reported as cancelled or timed out. Cancellation is
/// optional because some operations have only a deadline. The first interrupt
/// sets `abort`, which the transport reads to stop the request.
async fn race_controls<T, F>(
    work: F,
    cancellation: Option<&swallowtail_runtime::ImmediateCancellation>,
    deadline: Option<swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation>>,
    abort: &AtomicBool,
    on_interrupt: OnInterrupt,
) -> Race<T>
where
    F: Future<Output = T>,
{
    let mut work = std::pin::pin!(work);
    let mut cancellation_wait = cancellation.map(swallowtail_runtime::ImmediateCancellation::wait_requested);
    let mut deadline = deadline;
    let mut interrupt = None;
    let value = poll_fn(|context| {
        if let Poll::Ready(value) = work.as_mut().poll(context) {
            return Poll::Ready(Some(value));
        }
        if interrupt.is_none() {
            let fired = if cancellation_wait
                .as_mut()
                .is_some_and(|wait| wait.as_mut().poll(context).is_ready())
            {
                Some(Interrupt::Cancelled)
            } else if deadline
                .as_mut()
                .is_some_and(|wait| wait.as_mut().poll(context).is_ready())
            {
                Some(Interrupt::Deadline)
            } else {
                None
            };
            if let Some(kind) = fired {
                interrupt = Some(kind);
                abort.store(true, Ordering::SeqCst);
                match on_interrupt {
                    OnInterrupt::Abandon => return Poll::Ready(None),
                    // Re-poll at once so the work observes the abort flag.
                    OnInterrupt::Drain => context.waker().wake_by_ref(),
                }
            }
        }
        Poll::Pending
    })
    .await;
    Race { value, interrupt }
}

#[cfg(test)]
mod controlled_race_tests {
    use super::{Interrupt, OnInterrupt, race_controls};
    use std::future::{Future, pending, poll_fn, ready};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::task::{Context, Poll, Waker};
    use swallowtail_runtime::{BoxFuture, Deadline, DeadlineObservation, MonotonicInstant};

    fn block_on<F: Future>(future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        let mut context = Context::from_waker(Waker::noop());
        for _ in 0..128 {
            if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
                return value;
            }
        }
        panic!("fixture future did not resolve within bounded polls");
    }

    fn deadline_reached() -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(ready(DeadlineObservation::new(
            Deadline::at(MonotonicInstant::from_ticks(1)),
            MonotonicInstant::from_ticks(2),
        )))
    }

    fn deadline_not_reached() -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(pending())
    }

    #[test]
    fn completed_work_wins_over_a_deadline_observed_on_the_same_poll() {
        let abort = AtomicBool::new(false);

        let race = block_on(race_controls(
            ready(7_u8),
            None,
            Some(deadline_reached()),
            &abort,
            OnInterrupt::Abandon,
        ));

        assert_eq!(race.value, Some(7));
        assert_eq!(race.interrupt, None);
        assert!(!abort.load(Ordering::SeqCst));
    }

    #[test]
    fn abandon_returns_at_the_deadline_and_sets_the_abort_flag() {
        let abort = AtomicBool::new(false);

        let race = block_on(race_controls(
            pending::<u8>(),
            None,
            Some(deadline_reached()),
            &abort,
            OnInterrupt::Abandon,
        ));

        assert_eq!(race.value, None);
        assert_eq!(race.interrupt, Some(Interrupt::Deadline));
        assert!(abort.load(Ordering::SeqCst));
    }

    #[test]
    fn drain_keeps_the_work_result_and_reports_the_interrupt() {
        let mut yielded = false;
        let work = poll_fn(move |_| {
            if yielded {
                Poll::Ready(9_u8)
            } else {
                yielded = true;
                Poll::Pending
            }
        });
        let abort = AtomicBool::new(false);

        let race = block_on(race_controls(
            work,
            None,
            Some(deadline_reached()),
            &abort,
            OnInterrupt::Drain,
        ));

        assert_eq!(race.value, Some(9));
        assert_eq!(race.interrupt, Some(Interrupt::Deadline));
        assert!(abort.load(Ordering::SeqCst));
    }

    #[test]
    fn an_undisturbed_race_never_sets_the_abort_flag() {
        let abort = AtomicBool::new(false);

        let race = block_on(race_controls(
            ready(1_u8),
            None,
            Some(deadline_not_reached()),
            &abort,
            OnInterrupt::Drain,
        ));

        assert_eq!(race.value, Some(1));
        assert_eq!(race.interrupt, None);
        assert!(!abort.load(Ordering::SeqCst));
    }
}
