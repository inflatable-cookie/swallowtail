//! The per-kernel serialized admission gate.
//!
//! Contract 063 requires one serialized admission point. Because the live
//! consumer verdict is asynchronous, "serialized" has to span an `await`: the
//! pre-verdict checks, the awaited validation, and the final state commit must
//! run as one critical section that no other dispatch, progress, or delivery
//! admission can interleave with.
//!
//! This gate is an executor-neutral asynchronous mutex. It holds no lock across
//! the await itself; it holds *admission* across it, so at most one admission
//! sequence is in flight per kernel at any time.
//!
//! Waiters queue under monotonically increasing ids, so release wakes the
//! oldest live waiter. A waiter that is dropped before it acquires removes its
//! own entry; if it had been handed the wake-up and the gate is still free, it
//! passes that wake-up to the next waiter so no admission is stranded.

use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};

#[derive(Default)]
struct GateInner {
    held: bool,
    next_id: u64,
    waiters: BTreeMap<u64, Waker>,
}

/// One kernel's exclusive admission gate.
#[derive(Default)]
pub(super) struct AdmissionGate {
    inner: Mutex<GateInner>,
}

impl AdmissionGate {
    fn locked(&self) -> std::sync::MutexGuard<'_, GateInner> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Waits for exclusive admission for one complete sequence.
    pub(super) fn acquire(&self) -> Acquire<'_> {
        Acquire {
            gate: self,
            waiter_id: None,
        }
    }

    fn release(&self) {
        let next = {
            let mut inner = self.locked();
            inner.held = false;
            inner.waiters.pop_first().map(|(_, waker)| waker)
        };
        if let Some(waker) = next {
            waker.wake();
        }
    }

    /// Removes a departing waiter and, if the gate is free, wakes the next one.
    fn depart(&self, waiter_id: u64) {
        let next = {
            let mut inner = self.locked();
            inner.waiters.remove(&waiter_id);
            if inner.held {
                None
            } else {
                inner.waiters.pop_first().map(|(_, waker)| waker)
            }
        };
        if let Some(waker) = next {
            waker.wake();
        }
    }
}

/// Future that resolves once this task owns the admission gate.
///
/// Dropping it before it resolves deregisters its waiter, so cancelled
/// admissions never leave wakers behind on a long-lived kernel.
pub(super) struct Acquire<'gate> {
    gate: &'gate AdmissionGate,
    waiter_id: Option<u64>,
}

impl<'gate> Future for Acquire<'gate> {
    type Output = AdmissionPermit<'gate>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let gate = self.gate;
        let mut inner = gate.locked();
        if inner.held {
            let id = match self.waiter_id {
                Some(id) => id,
                None => {
                    let id = inner.next_id;
                    inner.next_id += 1;
                    self.waiter_id = Some(id);
                    id
                }
            };
            match inner.waiters.get_mut(&id) {
                Some(waker) if waker.will_wake(context.waker()) => {}
                _ => {
                    inner.waiters.insert(id, context.waker().clone());
                }
            }
            return Poll::Pending;
        }
        inner.held = true;
        if let Some(id) = self.waiter_id.take() {
            inner.waiters.remove(&id);
        }
        drop(inner);
        Poll::Ready(AdmissionPermit { gate })
    }
}

impl Drop for Acquire<'_> {
    fn drop(&mut self) {
        if let Some(id) = self.waiter_id.take() {
            self.gate.depart(id);
        }
    }
}

/// Exclusive admission held for one complete admission sequence.
///
/// Dropping the permit ends the critical section and wakes the next waiter.
pub(super) struct AdmissionPermit<'gate> {
    gate: &'gate AdmissionGate,
}

impl Drop for AdmissionPermit<'_> {
    fn drop(&mut self) {
        self.gate.release();
    }
}

#[cfg(test)]
mod tests {
    use super::{AdmissionGate, AdmissionPermit};
    use std::future::Future;
    use std::pin::pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll, Wake, Waker};

    #[derive(Default)]
    struct CountingWake {
        wakes: AtomicUsize,
    }

    impl Wake for CountingWake {
        fn wake(self: Arc<Self>) {
            self.wakes.fetch_add(1, Ordering::SeqCst);
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.wakes.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn poll_once<F: Future>(future: &mut std::pin::Pin<&mut F>, waker: &Waker) -> Poll<F::Output> {
        future.as_mut().poll(&mut Context::from_waker(waker))
    }

    fn permit_of<'gate>(poll: Poll<AdmissionPermit<'gate>>) -> AdmissionPermit<'gate> {
        match poll {
            Poll::Ready(permit) => permit,
            Poll::Pending => panic!("expected the gate to admit this waiter"),
        }
    }

    #[test]
    fn dropped_waiter_deregisters_its_waker() {
        let gate = AdmissionGate::default();
        let holder_wake = Arc::new(CountingWake::default());
        let holder_waker = Waker::from(Arc::clone(&holder_wake));
        let held = permit_of(poll_once(&mut pin!(gate.acquire()), &holder_waker));

        let waiter_wake = Arc::new(CountingWake::default());
        let waiter_waker = Waker::from(Arc::clone(&waiter_wake));
        {
            let mut waiter = pin!(gate.acquire());
            assert!(poll_once(&mut waiter, &waiter_waker).is_pending());
            assert_eq!(gate.locked().waiters.len(), 1);
        }
        assert!(gate.locked().waiters.is_empty());

        drop(held);
        assert_eq!(waiter_wake.wakes.load(Ordering::SeqCst), 0);
        assert!(gate.locked().waiters.is_empty());
    }

    #[test]
    fn release_wakes_exactly_one_waiter_in_arrival_order() {
        let gate = AdmissionGate::default();
        let holder_waker = Waker::from(Arc::new(CountingWake::default()));
        let held = permit_of(poll_once(&mut pin!(gate.acquire()), &holder_waker));

        let first = Arc::new(CountingWake::default());
        let second = Arc::new(CountingWake::default());
        let first_waker = Waker::from(Arc::clone(&first));
        let second_waker = Waker::from(Arc::clone(&second));
        let mut first_future = pin!(gate.acquire());
        let mut second_future = pin!(gate.acquire());
        assert!(poll_once(&mut first_future, &first_waker).is_pending());
        assert!(poll_once(&mut second_future, &second_waker).is_pending());

        drop(held);
        assert_eq!(first.wakes.load(Ordering::SeqCst), 1);
        assert_eq!(second.wakes.load(Ordering::SeqCst), 0);

        let admitted = permit_of(poll_once(&mut first_future, &first_waker));
        assert!(poll_once(&mut second_future, &second_waker).is_pending());
        drop(admitted);
        assert_eq!(second.wakes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn woken_waiter_dropped_before_polling_passes_the_wake_on() {
        let gate = AdmissionGate::default();
        let holder_waker = Waker::from(Arc::new(CountingWake::default()));
        let held = permit_of(poll_once(&mut pin!(gate.acquire()), &holder_waker));

        let second = Arc::new(CountingWake::default());
        let second_waker = Waker::from(Arc::clone(&second));
        let first_waker = Waker::from(Arc::new(CountingWake::default()));
        let mut second_future = pin!(gate.acquire());
        {
            let mut first_future = pin!(gate.acquire());
            assert!(poll_once(&mut first_future, &first_waker).is_pending());
            assert!(poll_once(&mut second_future, &second_waker).is_pending());
            drop(held);
            // The first waiter is woken, then cancelled before it polls again.
        }
        assert_eq!(second.wakes.load(Ordering::SeqCst), 1);

        let admitted = permit_of(poll_once(&mut second_future, &second_waker));
        drop(admitted);
    }

    #[test]
    fn repeated_polls_do_not_duplicate_a_waiter() {
        let gate = AdmissionGate::default();
        let holder_waker = Waker::from(Arc::new(CountingWake::default()));
        let held = permit_of(poll_once(&mut pin!(gate.acquire()), &holder_waker));

        let waiter_waker = Waker::from(Arc::new(CountingWake::default()));
        let mut waiter = pin!(gate.acquire());
        for _ in 0..3 {
            assert!(poll_once(&mut waiter, &waiter_waker).is_pending());
        }
        assert_eq!(gate.locked().waiters.len(), 1);
        drop(held);
        let admitted = permit_of(poll_once(&mut waiter, &waiter_waker));
        assert!(gate.locked().waiters.is_empty());
        drop(admitted);
    }
}
