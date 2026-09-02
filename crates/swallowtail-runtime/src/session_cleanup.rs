#![deny(missing_docs)]

use crate::{BoxFuture, CleanupOutcome, Deadline, HostServices};
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_core::{ExecutionHostId, SafeDiagnostic};

/// Caller-selected hard boundary for interactive-session cleanup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SessionCleanupRequest {
    deadline: Deadline,
}

impl SessionCleanupRequest {
    /// Creates a cleanup request with one absolute host-monotonic deadline.
    #[must_use]
    pub const fn new(deadline: Deadline) -> Self {
        Self { deadline }
    }

    /// Returns the hard deadline for the public cleanup operation.
    #[must_use]
    pub const fn deadline(self) -> Deadline {
        self.deadline
    }
}

/// Bounds one interactive-session cleanup future by caller-supplied host time.
///
/// The cleanup future is abandoned and reported as failed when the host
/// observes the deadline. A ready cleanup result is accepted only while a
/// fresh host-time observation remains before the boundary.
#[must_use]
pub fn bound_session_cleanup(
    expected_execution_host_id: ExecutionHostId,
    request: SessionCleanupRequest,
    services: HostServices,
    mut cleanup: BoxFuture<'static, CleanupOutcome>,
) -> BoxFuture<'static, CleanupOutcome> {
    let host_check = services.require_execution_host(&expected_execution_host_id);
    let time = services.time().cloned();
    Box::pin(async move {
        if let Err(error) = host_check {
            return CleanupOutcome::Failed(error.diagnostic().clone());
        }
        let Some(time) = time else {
            return cleanup_failure(
                "swallowtail.session_cleanup.time_service_missing",
                "Interactive-session cleanup requires the caller's host time service",
            );
        };
        if time.now() >= request.deadline().instant() {
            return deadline_expired();
        }
        let mut deadline = None;
        poll_fn(|context| {
            if let Poll::Ready(outcome) = cleanup.as_mut().poll(context) {
                if time.now() < request.deadline().instant() {
                    return Poll::Ready(outcome);
                }
                return Poll::Ready(deadline_expired());
            }
            let deadline = deadline.get_or_insert_with(|| time.wait_until(request.deadline()));
            if deadline.as_mut().poll(context).is_ready() {
                Poll::Ready(deadline_expired())
            } else {
                Poll::Pending
            }
        })
        .await
    })
}

fn deadline_expired() -> CleanupOutcome {
    cleanup_failure(
        "swallowtail.session_cleanup.deadline_expired",
        "Interactive-session cleanup exceeded the caller's hard deadline",
    )
}

fn cleanup_failure(code: &'static str, message: &'static str) -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(code, message))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeadlineObservation, MonotonicInstant, TimeService};
    use futures_channel::oneshot;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, Wake, Waker};

    struct ControlledTime {
        now: MonotonicInstant,
        now_calls: AtomicUsize,
        receiver: Mutex<Option<oneshot::Receiver<MonotonicInstant>>>,
    }

    impl TimeService for ControlledTime {
        fn now(&self) -> MonotonicInstant {
            self.now_calls.fetch_add(1, Ordering::SeqCst);
            self.now
        }

        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            let receiver = self
                .receiver
                .lock()
                .expect("controlled-time lock poisoned")
                .take()
                .expect("deadline wait requested once");
            Box::pin(async move {
                let observed_at = receiver.await.expect("test supplies deadline observation");
                DeadlineObservation::new(deadline, observed_at)
            })
        }
    }

    struct WakeFlag(AtomicBool);

    struct StalledStage(Arc<AtomicBool>);

    impl Future for StalledStage {
        type Output = CleanupOutcome;

        fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Pending
        }
    }

    impl Drop for StalledStage {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    impl Wake for WakeFlag {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    fn host_id(value: &str) -> ExecutionHostId {
        ExecutionHostId::new(value).expect("fixture host id is valid")
    }

    fn deadline(ticks: u64) -> Deadline {
        Deadline::at(MonotonicInstant::from_ticks(ticks))
    }

    #[test]
    fn request_preserves_the_exact_caller_deadline() {
        let request = SessionCleanupRequest::new(deadline(20));

        assert_eq!(request.deadline(), deadline(20));
    }

    #[test]
    fn missing_or_cross_host_time_fails_without_polling_cleanup() {
        let expected = host_id("host.expected");
        let missing = HostServices::new(expected.clone());
        let missing_outcome = poll_ready(bound_session_cleanup(
            expected.clone(),
            SessionCleanupRequest::new(deadline(20)),
            missing,
            Box::pin(async { panic!("cleanup must not be polled without host time") }),
        ));
        assert_eq!(
            missing_outcome.diagnostic().map(SafeDiagnostic::code),
            Some("swallowtail.session_cleanup.time_service_missing")
        );

        let other = host_id("host.other");
        let cross_host = HostServices::new(other);
        let cross_host_outcome = poll_ready(bound_session_cleanup(
            expected.clone(),
            SessionCleanupRequest::new(deadline(20)),
            cross_host,
            Box::pin(async { panic!("cleanup must not be polled across hosts") }),
        ));
        assert_eq!(
            cross_host_outcome.diagnostic().map(SafeDiagnostic::code),
            Some("swallowtail.execution_host_mismatch")
        );
    }

    #[test]
    fn already_elapsed_deadline_fails_without_polling_cleanup() {
        let expected = host_id("host.elapsed");
        let (_sender, receiver) = oneshot::channel();
        let services = HostServices::new(expected.clone()).with_time(Arc::new(ControlledTime {
            now: MonotonicInstant::from_ticks(20),
            now_calls: AtomicUsize::new(0),
            receiver: Mutex::new(Some(receiver)),
        }));

        let outcome = poll_ready(bound_session_cleanup(
            expected.clone(),
            SessionCleanupRequest::new(deadline(20)),
            services,
            Box::pin(async { panic!("cleanup must not be polled after the boundary") }),
        ));

        assert_deadline_failure(&outcome);
    }

    #[test]
    fn every_stalled_cleanup_stage_returns_at_the_same_deadline() {
        for stage in [
            "interrupt",
            "escalation",
            "task_join",
            "credential_release",
            "resource_release",
        ] {
            let expected = host_id(&format!("host.{stage}"));
            let (sender, receiver) = oneshot::channel();
            let dropped = Arc::new(AtomicBool::new(false));
            let services =
                HostServices::new(expected.clone()).with_time(Arc::new(ControlledTime {
                    now: MonotonicInstant::from_ticks(10),
                    now_calls: AtomicUsize::new(0),
                    receiver: Mutex::new(Some(receiver)),
                }));
            let mut cleanup = bound_session_cleanup(
                expected.clone(),
                SessionCleanupRequest::new(deadline(20)),
                services,
                Box::pin(StalledStage(Arc::clone(&dropped))),
            );
            let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
            let waker = Waker::from(Arc::clone(&wake));
            let mut context = Context::from_waker(&waker);

            assert!(cleanup.as_mut().poll(&mut context).is_pending(), "{stage}");
            sender
                .send(MonotonicInstant::from_ticks(20))
                .expect("deadline receiver remains live");
            assert!(wake.0.load(Ordering::SeqCst), "{stage}");
            let Poll::Ready(outcome) = cleanup.as_mut().poll(&mut context) else {
                panic!("{stage} remained pending after the hard deadline");
            };
            assert_deadline_failure(&outcome);
            assert!(dropped.load(Ordering::SeqCst), "{stage}");
        }
    }

    #[test]
    fn cleanup_finishing_before_the_deadline_preserves_its_outcome() {
        let expected = host_id("host.clean");
        let (_sender, receiver) = oneshot::channel();
        let time = Arc::new(ControlledTime {
            now: MonotonicInstant::from_ticks(10),
            now_calls: AtomicUsize::new(0),
            receiver: Mutex::new(Some(receiver)),
        });
        let services = HostServices::new(expected.clone()).with_time(time.clone());

        let outcome = poll_ready(bound_session_cleanup(
            expected.clone(),
            SessionCleanupRequest::new(deadline(20)),
            services,
            Box::pin(async { CleanupOutcome::Clean }),
        ));

        assert_eq!(outcome, CleanupOutcome::Clean);
        assert_eq!(time.now_calls.load(Ordering::SeqCst), 2);
    }

    fn poll_ready<T>(mut future: Pin<Box<dyn Future<Output = T> + Send>>) -> T {
        let wake = Arc::new(WakeFlag(AtomicBool::new(false)));
        let waker = Waker::from(wake);
        let mut context = Context::from_waker(&waker);
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("fixture future unexpectedly remained pending"),
        }
    }

    fn assert_deadline_failure(outcome: &CleanupOutcome) {
        assert_eq!(
            outcome.diagnostic().map(SafeDiagnostic::code),
            Some("swallowtail.session_cleanup.deadline_expired")
        );
    }
}
