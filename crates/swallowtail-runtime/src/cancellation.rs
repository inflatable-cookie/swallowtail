use crate::{BoxFuture, RuntimeFailure};
use std::fmt;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Poll, Waker};
use swallowtail_core::CancellationScope;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Immediate acknowledgement of a local cancellation request.
///
/// This is not evidence that provider work has stopped or reached a terminal
/// outcome.
pub enum CancellationAcknowledgement {
    /// This call recorded the first cancellation request.
    Requested,
    /// Cancellation had already been requested.
    AlreadyRequested,
}

/// Operation-scoped cancellation request surface.
pub trait CancellationControl: Send + Sync {
    /// Returns the operation shape to which cancellation applies.
    fn scope(&self) -> CancellationScope;
    /// Requests cancellation without claiming terminal provider truth.
    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>>;
}

/// In-memory idempotent cancellation signal with waiter notification.
pub struct ImmediateCancellation {
    scope: CancellationScope,
    requested: AtomicBool,
    waiters: Mutex<Vec<Waker>>,
}

impl fmt::Debug for ImmediateCancellation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmediateCancellation")
            .field("scope", &self.scope)
            .field("requested", &self.is_requested())
            .finish()
    }
}

impl ImmediateCancellation {
    #[must_use]
    /// Creates an unrequested signal for an exact cancellation scope.
    pub const fn new(scope: CancellationScope) -> Self {
        Self {
            scope,
            requested: AtomicBool::new(false),
            waiters: Mutex::new(Vec::new()),
        }
    }

    #[must_use]
    /// Returns whether cancellation has been requested.
    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }

    /// Resolves when cancellation is first requested.
    ///
    /// Concurrent waiters are all notified; each registered waker is woken
    /// exactly once when the request is recorded.
    pub fn wait_requested(&self) -> BoxFuture<'_, ()> {
        Box::pin(std::future::poll_fn(|context| {
            if self.is_requested() {
                return Poll::Ready(());
            }
            let mut waiters = self
                .waiters
                .lock()
                .expect("cancellation waiter lock poisoned");
            if self.is_requested() {
                Poll::Ready(())
            } else {
                if !waiters
                    .iter()
                    .any(|waiter| waiter.will_wake(context.waker()))
                {
                    waiters.push(context.waker().clone());
                }
                Poll::Pending
            }
        }))
    }
}

impl CancellationControl for ImmediateCancellation {
    fn scope(&self) -> CancellationScope {
        self.scope
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let acknowledgement = if self.requested.swap(true, Ordering::SeqCst) {
            CancellationAcknowledgement::AlreadyRequested
        } else {
            let mut waiters = self
                .waiters
                .lock()
                .expect("cancellation waiter lock poisoned");
            for waiter in waiters.drain(..) {
                waiter.wake();
            }
            CancellationAcknowledgement::Requested
        };
        Box::pin(async move { Ok(acknowledgement) })
    }
}

#[cfg(test)]
mod tests {
    use super::{CancellationAcknowledgement, CancellationControl, ImmediateCancellation};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::task::{Context, Poll, Wake, Waker};
    use std::thread;
    use std::time::Duration;
    use swallowtail_core::CancellationScope;

    struct CountingWake {
        count: Arc<AtomicUsize>,
        thread: thread::Thread,
    }

    impl Wake for CountingWake {
        fn wake(self: Arc<Self>) {
            self.count.fetch_add(1, Ordering::SeqCst);
            self.thread.unpark();
        }
    }

    #[test]
    fn concurrent_waiters_wake_exactly_once_on_request() {
        let signal = Arc::new(ImmediateCancellation::new(CancellationScope::StructuredRun));
        let registered = Arc::new(Barrier::new(3));
        let mut handles = Vec::new();
        for _ in 0..2 {
            let signal = Arc::clone(&signal);
            let registered = Arc::clone(&registered);
            handles.push(thread::spawn(move || {
                let count = Arc::new(AtomicUsize::new(0));
                let waker = Waker::from(Arc::new(CountingWake {
                    count: Arc::clone(&count),
                    thread: thread::current(),
                }));
                let mut context = Context::from_waker(&waker);
                let mut future = std::pin::pin!(signal.wait_requested());
                assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
                registered.wait();
                // Parked until the shared request wakes this thread; the
                // wake may land before park, which still returns at once.
                thread::park_timeout(Duration::from_secs(5));
                assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(()));
                assert_eq!(
                    count.load(Ordering::SeqCst),
                    1,
                    "each waiter's waker must fire exactly once"
                );
            }));
        }
        registered.wait();
        request_and_expect(&signal);
        for handle in handles {
            handle.join().expect("waiter thread completes");
        }
    }

    #[test]
    fn multiple_requests_wake_registered_waiters_only_once() {
        let signal = Arc::new(ImmediateCancellation::new(CancellationScope::ActiveTurn));
        let count = Arc::new(AtomicUsize::new(0));
        let waker = Waker::from(Arc::new(CountingWake {
            count: Arc::clone(&count),
            thread: thread::current(),
        }));
        let mut context = Context::from_waker(&waker);
        let mut future = std::pin::pin!(signal.wait_requested());
        assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
        request_and_expect(&signal);
        request_and_expect(&signal);
        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(()));
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn waiting_after_request_resolves_immediately_without_registration() {
        let signal = Arc::new(ImmediateCancellation::new(
            CancellationScope::ActiveResponse,
        ));
        request_and_expect(&signal);
        let count = Arc::new(AtomicUsize::new(0));
        let waker = Waker::from(Arc::new(CountingWake {
            count: Arc::clone(&count),
            thread: thread::current(),
        }));
        let mut context = Context::from_waker(&waker);
        let mut future = std::pin::pin!(signal.wait_requested());
        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(()));
        assert_eq!(count.load(Ordering::SeqCst), 0);
    }

    fn request_and_expect(signal: &ImmediateCancellation) -> CancellationAcknowledgement {
        let mut context = Context::from_waker(Waker::noop());
        let mut future = std::pin::pin!(signal.request());
        match future.as_mut().poll(&mut context) {
            Poll::Ready(Ok(acknowledgement)) => acknowledgement,
            Poll::Ready(Err(_)) => panic!("cancellation request must succeed"),
            Poll::Pending => panic!("cancellation request must resolve immediately"),
        }
    }

    #[test]
    fn waiters_do_not_duplicate_after_repoll() {
        let signal = Arc::new(ImmediateCancellation::new(
            CancellationScope::OwnedServingInstance,
        ));
        let count = Arc::new(AtomicUsize::new(0));
        let waker = Waker::from(Arc::new(CountingWake {
            count: Arc::clone(&count),
            thread: thread::current(),
        }));
        let mut context = Context::from_waker(&waker);
        let mut future = std::pin::pin!(signal.wait_requested());
        assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
        request_and_expect(&signal);
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }
}
