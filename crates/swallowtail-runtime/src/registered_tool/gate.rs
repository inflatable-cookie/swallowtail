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

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};

#[derive(Default)]
struct GateInner {
    held: bool,
    waiters: VecDeque<Waker>,
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
        Acquire { gate: self }
    }

    fn release(&self) {
        let waiters = {
            let mut inner = self.locked();
            inner.held = false;
            std::mem::take(&mut inner.waiters)
        };
        for waiter in waiters {
            waiter.wake();
        }
    }
}

/// Future that resolves once this task owns the admission gate.
pub(super) struct Acquire<'gate> {
    gate: &'gate AdmissionGate,
}

impl<'gate> Future for Acquire<'gate> {
    type Output = AdmissionPermit<'gate>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.gate.locked();
        if inner.held {
            let waker = context.waker();
            if !inner.waiters.iter().any(|waiting| waiting.will_wake(waker)) {
                inner.waiters.push_back(waker.clone());
            }
            return Poll::Pending;
        }
        inner.held = true;
        drop(inner);
        Poll::Ready(AdmissionPermit { gate: self.gate })
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
