use crate::{BoxFuture, RuntimeFailure};
use std::collections::BTreeMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, PoisonError};
use std::task::{Context, Poll, Waker};
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

struct WaiterRegistry {
    next_id: u64,
    waiters: BTreeMap<u64, Waker>,
}

/// In-memory idempotent cancellation signal with waiter notification.
pub struct ImmediateCancellation {
    scope: CancellationScope,
    requested: AtomicBool,
    waiters: Mutex<WaiterRegistry>,
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
            waiters: Mutex::new(WaiterRegistry {
                next_id: 0,
                waiters: BTreeMap::new(),
            }),
        }
    }

    #[must_use]
    /// Returns whether cancellation has been requested.
    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }

    fn locked(&self) -> std::sync::MutexGuard<'_, WaiterRegistry> {
        self.waiters.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn remove_waiter(&self, id: u64) {
        self.locked().waiters.remove(&id);
    }

    #[cfg(test)]
    pub(crate) fn pending_waiter_count(&self) -> usize {
        self.locked().waiters.len()
    }

    #[cfg(test)]
    pub(crate) fn try_lock_free(&self) -> bool {
        self.waiters.try_lock().is_ok()
    }

    /// Resolves when cancellation is first requested.
    ///
    /// Concurrent waiters are all notified; each registered waker is woken
    /// exactly once when the request is recorded. Each future registers under
    /// its own waiter id and deregisters on drop, so abandoned futures never
    /// accumulate wakers on a long-lived signal.
    pub fn wait_requested(&self) -> BoxFuture<'_, ()> {
        Box::pin(WaitRequestedFuture {
            signal: self,
            waiter_id: None,
        })
    }
}

struct WaitRequestedFuture<'a> {
    signal: &'a ImmediateCancellation,
    waiter_id: Option<u64>,
}

impl Drop for WaitRequestedFuture<'_> {
    fn drop(&mut self) {
        if let Some(id) = self.waiter_id.take() {
            self.signal.remove_waiter(id);
        }
    }
}

impl Future for WaitRequestedFuture<'_> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.signal.is_requested() {
            if let Some(id) = self.waiter_id.take() {
                self.signal.remove_waiter(id);
            }
            return Poll::Ready(());
        }
        let mut registry = self.signal.locked();
        if self.signal.is_requested() {
            if let Some(id) = self.waiter_id.take() {
                registry.waiters.remove(&id);
            }
            return Poll::Ready(());
        }
        if let Some(id) = self.waiter_id {
            match registry.waiters.get_mut(&id) {
                Some(existing) if existing.will_wake(context.waker()) => {}
                Some(existing) => *existing = context.waker().clone(),
                None => {
                    registry.waiters.insert(id, context.waker().clone());
                }
            }
        } else {
            let id = registry.next_id;
            registry.next_id = registry.next_id.wrapping_add(1);
            registry.waiters.insert(id, context.waker().clone());
            self.waiter_id = Some(id);
        }
        Poll::Pending
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
            // Waiters leave the lock before they are woken: a waker may poll
            // its own future immediately, and that poll must never observe a
            // held lock.
            let waiters: Vec<Waker> = {
                let mut registry = self.locked();
                std::mem::take(&mut registry.waiters)
                    .into_values()
                    .collect()
            };
            for waiter in waiters {
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

    #[test]
    fn dropped_waiters_do_not_accumulate_on_long_lived_signal() {
        let signal = Arc::new(ImmediateCancellation::new(CancellationScope::StructuredRun));
        let dropped_wakes = Arc::new(AtomicUsize::new(0));

        for _ in 0..256 {
            let waker = Waker::from(Arc::new(CountingWake {
                count: Arc::clone(&dropped_wakes),
                thread: thread::current(),
            }));
            let mut context = Context::from_waker(&waker);
            let mut future = std::pin::pin!(signal.wait_requested());
            assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
        }

        assert_eq!(
            signal.pending_waiter_count(),
            0,
            "dropping polled wait_requested futures must remove their wakers"
        );

        let surviving_wakes = Arc::new(AtomicUsize::new(0));
        let surviving_waker = Waker::from(Arc::new(CountingWake {
            count: Arc::clone(&surviving_wakes),
            thread: thread::current(),
        }));
        let mut surviving_context = Context::from_waker(&surviving_waker);
        let mut surviving = std::pin::pin!(signal.wait_requested());
        assert_eq!(
            surviving.as_mut().poll(&mut surviving_context),
            Poll::Pending
        );
        assert_eq!(signal.pending_waiter_count(), 1);

        request_and_expect(&signal);
        assert_eq!(dropped_wakes.load(Ordering::SeqCst), 0);
        assert_eq!(surviving_wakes.load(Ordering::SeqCst), 1);
        assert_eq!(signal.pending_waiter_count(), 0);
        assert_eq!(
            surviving.as_mut().poll(&mut surviving_context),
            Poll::Ready(())
        );
    }

    #[test]
    fn dropping_one_waiter_does_not_drop_another_sharing_the_same_waker() {
        let signal = Arc::new(ImmediateCancellation::new(CancellationScope::ActiveTurn));
        let count = Arc::new(AtomicUsize::new(0));
        let waker = Waker::from(Arc::new(CountingWake {
            count: Arc::clone(&count),
            thread: thread::current(),
        }));
        let mut context = Context::from_waker(&waker);

        let mut first = signal.wait_requested();
        let mut second = signal.wait_requested();
        assert_eq!(first.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(second.as_mut().poll(&mut context), Poll::Pending);
        assert_eq!(signal.pending_waiter_count(), 2);

        drop(first);
        assert_eq!(signal.pending_waiter_count(), 1);

        request_and_expect(&signal);
        assert_eq!(count.load(Ordering::SeqCst), 1);
        assert_eq!(second.as_mut().poll(&mut context), Poll::Ready(()));
    }
}
