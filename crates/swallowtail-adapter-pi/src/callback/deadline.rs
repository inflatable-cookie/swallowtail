use super::{CallbackHub, CallbackState};
use serde_json::{Value, json};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swallowtail_runtime::CallbackId;

pub(crate) struct CallbackFinishedFuture {
    state: Arc<Mutex<CallbackState>>,
    callback_id: CallbackId,
}

impl Future for CallbackFinishedFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().expect("Pi callback lock poisoned");
        match state.pending.get_mut(&self.callback_id) {
            Some(pending) => {
                pending.waiter = Some(context.waker().clone());
                Poll::Pending
            }
            None => Poll::Ready(()),
        }
    }
}

impl CallbackHub {
    pub(crate) fn finished_future(&self, callback_id: CallbackId) -> CallbackFinishedFuture {
        CallbackFinishedFuture {
            state: Arc::clone(&self.state),
            callback_id,
        }
    }

    pub(crate) fn expire(&self, callback_id: &CallbackId) -> Option<Value> {
        let mut state = self.state.lock().expect("Pi callback lock poisoned");
        if state.closed {
            return None;
        }
        let mut pending = state.pending.remove(callback_id)?;
        state.expired.insert(callback_id.clone());
        state
            .requests
            .retain(|request| request.callback_id() != callback_id);
        if let Some(waiter) = pending.waiter.take() {
            waiter.wake();
        }
        Some(json!({
            "type": "extension_ui_response",
            "id": pending.provider_id,
            "cancelled": true
        }))
    }
}
