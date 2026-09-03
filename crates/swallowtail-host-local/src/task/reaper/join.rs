use std::future::poll_fn;
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};
use swallowtail_runtime::{BoxFuture, RuntimeFailure};

pub(in crate::task) struct ReapJoin {
    completion: Arc<ReapJoinCompletion>,
}

#[derive(Default)]
pub(super) struct ReapJoinCompletion {
    state: Mutex<ReapJoinState>,
}

#[derive(Default)]
struct ReapJoinState {
    outcome: Option<Result<(), RuntimeFailure>>,
    waker: Option<Waker>,
}

impl ReapJoin {
    pub(super) fn new(completion: Arc<ReapJoinCompletion>) -> Self {
        Self { completion }
    }

    pub(in crate::task) fn into_future(self) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(poll_fn(move |context| {
            let mut state = self
                .completion
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(outcome) = state.outcome.take() {
                return Poll::Ready(outcome);
            }
            if state
                .waker
                .as_ref()
                .is_none_or(|waker| !waker.will_wake(context.waker()))
            {
                state.waker = Some(context.waker().clone());
            }
            Poll::Pending
        }))
    }
}

impl ReapJoinCompletion {
    pub(super) fn complete(&self, outcome: Result<(), RuntimeFailure>) {
        let waker = {
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            state.outcome = Some(outcome);
            state.waker.take()
        };
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}
