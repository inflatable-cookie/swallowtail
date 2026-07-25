use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

#[derive(Default)]
struct FinishedState {
    finished: bool,
    waiter: Option<Waker>,
}

pub(crate) struct FinishedSignal(Arc<Mutex<FinishedState>>);

impl FinishedSignal {
    pub(crate) fn new() -> Self {
        Self(Arc::new(Mutex::new(FinishedState::default())))
    }

    pub(crate) fn future(&self) -> TurnFinishedFuture {
        TurnFinishedFuture(Arc::clone(&self.0))
    }

    pub(crate) fn finish(&self) {
        let mut state = self.0.lock().expect("turn-finished lock poisoned");
        state.finished = true;
        if let Some(waiter) = state.waiter.take() {
            waiter.wake();
        }
    }
}

pub(crate) struct TurnFinishedFuture(Arc<Mutex<FinishedState>>);

impl Future for TurnFinishedFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().expect("turn-finished lock poisoned");
        if state.finished {
            Poll::Ready(())
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}
