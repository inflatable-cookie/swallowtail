//! Terminal signal for one turn, so a host deadline can race real completion
//! instead of polling.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

#[derive(Default)]
pub(super) struct FinishedState {
    pub(super) finished: bool,
    pub(super) waiter: Option<Waker>,
}

/// Resolves when the turn reaches its terminal outcome.
pub(crate) struct TurnFinishedFuture(Arc<Mutex<FinishedState>>);

impl TurnFinishedFuture {
    pub(super) fn new(state: Arc<Mutex<FinishedState>>) -> Self {
        Self(state)
    }
}

impl Future for TurnFinishedFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self
            .0
            .lock()
            .expect("SDK sidecar turn-finished lock poisoned");
        if state.finished {
            Poll::Ready(())
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}
