use crate::failure::failure;
use futures_channel::oneshot;
use std::collections::BTreeSet;
use std::sync::Mutex;
use swallowtail_runtime::{
    BoxFuture, DirectToolCallId, DirectToolResult, DirectToolResultSubmitter, RuntimeFailure,
};

enum SubmitState {
    AwaitingCall,
    Waiting(BTreeSet<DirectToolCallId>),
    Submitted,
    Abandoned,
}

pub(in crate::driver) struct ResultSubmitter {
    state: Mutex<SubmitState>,
    sender: Mutex<Option<oneshot::Sender<Vec<DirectToolResult>>>>,
}

impl ResultSubmitter {
    pub(super) fn new() -> (Self, oneshot::Receiver<Vec<DirectToolResult>>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self {
                state: Mutex::new(SubmitState::AwaitingCall),
                sender: Mutex::new(Some(sender)),
            },
            receiver,
        )
    }

    pub(super) fn open(&self, call_id: DirectToolCallId) -> Result<(), RuntimeFailure> {
        let mut state = self.state.lock().expect("tool result state lock poisoned");
        if !matches!(*state, SubmitState::AwaitingCall) {
            return Err(exchange_failure(
                "Tool-result exchange was not awaiting a call",
            ));
        }
        *state = SubmitState::Waiting(BTreeSet::from([call_id]));
        Ok(())
    }

    pub(super) fn abandon(&self) {
        *self.state.lock().expect("tool result state lock poisoned") = SubmitState::Abandoned;
        self.sender
            .lock()
            .expect("tool result sender lock poisoned")
            .take();
    }
}

impl DirectToolResultSubmitter for ResultSubmitter {
    fn submit(&self, results: Vec<DirectToolResult>) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = {
            let mut state = self.state.lock().expect("tool result state lock poisoned");
            let SubmitState::Waiting(expected) = &*state else {
                return Box::pin(async {
                    Err(exchange_failure(
                        "Tool results arrived outside the active consumer wait",
                    ))
                });
            };
            let supplied: BTreeSet<_> = results
                .iter()
                .map(|result| result.call_id().clone())
                .collect();
            if supplied != *expected || supplied.len() != results.len() {
                return Box::pin(async {
                    Err(exchange_failure(
                        "Tool results did not match every exact pending call",
                    ))
                });
            }
            let sender = self
                .sender
                .lock()
                .expect("tool result sender lock poisoned")
                .take();
            *state = SubmitState::Submitted;
            sender
                .ok_or_else(|| exchange_failure("Tool results were already submitted"))
                .and_then(|sender| {
                    sender
                        .send(results)
                        .map_err(|_| exchange_failure("Tool-result wait ended before submission"))
                })
        };
        Box::pin(async move { result })
    }
}

fn exchange_failure(message: &'static str) -> RuntimeFailure {
    failure("swallowtail.deepseek.tool_exchange_invalid", message)
}
