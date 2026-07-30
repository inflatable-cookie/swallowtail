use crate::connection::PiConnection;
use crate::failure::failure;
use crate::protocol::PiUiDialogMethod;
use futures_core::Stream;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::pin::Pin;
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use swallowtail_runtime::{
    BoxCallbackStream, BoxFuture, CallbackAbandonment, CallbackExchange, CallbackId,
    CallbackOperationId, CallbackRequest, CallbackResponder, CallbackResponse, CallbackResult,
    HarnessUserInputRequest, RuntimeFailure,
};

mod deadline;

pub(crate) use deadline::CallbackFinishedFuture;

const CALLBACK_CAPACITY: usize = 16;

struct PendingCallback {
    provider_id: String,
    operation_id: CallbackOperationId,
    method: PiUiDialogMethod,
    user_input: HarnessUserInputRequest,
    waiter: Option<Waker>,
}

struct CallbackState {
    requests: VecDeque<CallbackRequest>,
    pending: BTreeMap<CallbackId, PendingCallback>,
    provider_ids: BTreeSet<String>,
    expired: BTreeSet<CallbackId>,
    closed: bool,
    waiter: Option<Waker>,
}

pub(crate) struct CallbackHub {
    state: Arc<Mutex<CallbackState>>,
}

impl CallbackHub {
    pub(crate) fn new(connection: Weak<PiConnection>) -> (Self, CallbackExchange) {
        let state = Arc::new(Mutex::new(CallbackState {
            requests: VecDeque::new(),
            pending: BTreeMap::new(),
            provider_ids: BTreeSet::new(),
            expired: BTreeSet::new(),
            closed: false,
            waiter: None,
        }));
        let requests: BoxCallbackStream = Box::pin(CallbackStream {
            state: Arc::clone(&state),
        });
        let responder: Arc<dyn CallbackResponder> = Arc::new(PiCallbackResponder {
            state: Arc::clone(&state),
            connection,
        });
        (Self { state }, CallbackExchange::new(requests, responder))
    }

    pub(crate) fn enqueue(
        &self,
        request: CallbackRequest,
        provider_id: String,
        method: PiUiDialogMethod,
        user_input: HarnessUserInputRequest,
    ) -> Result<(), RuntimeFailure> {
        let mut state = self.state.lock().expect("Pi callback lock poisoned");
        if state.closed {
            return Err(callback_closed());
        }
        if !state.provider_ids.insert(provider_id.clone()) {
            return Err(failure(
                "swallowtail.pi.rpc.callback_id_reused",
                "Pi RPC reused an extension UI request id",
            ));
        }
        if state.pending.len() >= CALLBACK_CAPACITY || state.requests.len() >= CALLBACK_CAPACITY {
            return Err(failure(
                "swallowtail.pi.rpc.callback_capacity_exceeded",
                "Pi RPC exceeded the bounded callback capacity",
            ));
        }
        state.pending.insert(
            request.callback_id().clone(),
            PendingCallback {
                provider_id,
                operation_id: request.operation_id().clone(),
                method,
                user_input,
                waiter: None,
            },
        );
        state.requests.push_back(request);
        wake(&mut state);
        Ok(())
    }

    pub(crate) fn abandon(&self, _reason: CallbackAbandonment) {
        let mut state = self.state.lock().expect("Pi callback lock poisoned");
        state.closed = true;
        state.requests.clear();
        for pending in state.pending.values_mut() {
            if let Some(waiter) = pending.waiter.take() {
                waiter.wake();
            }
        }
        state.pending.clear();
        wake(&mut state);
    }
}

struct CallbackStream {
    state: Arc<Mutex<CallbackState>>,
}

impl Stream for CallbackStream {
    type Item = Result<CallbackRequest, RuntimeFailure>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = self.state.lock().expect("Pi callback lock poisoned");
        if let Some(request) = state.requests.pop_front() {
            Poll::Ready(Some(Ok(request)))
        } else if state.closed {
            Poll::Ready(None)
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

struct PiCallbackResponder {
    state: Arc<Mutex<CallbackState>>,
    connection: Weak<PiConnection>,
}

impl CallbackResponder for PiCallbackResponder {
    fn respond(&self, response: CallbackResponse) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let state = Arc::clone(&self.state);
        let connection = self.connection.clone();
        Box::pin(async move {
            let value = claim_response(&state, &response)?;
            let connection = connection.upgrade().ok_or_else(callback_closed)?;
            connection.write_value(value).await
        })
    }
}

fn claim_response(
    state: &Arc<Mutex<CallbackState>>,
    response: &CallbackResponse,
) -> Result<Value, RuntimeFailure> {
    let mut state = state.lock().expect("Pi callback lock poisoned");
    if state.expired.contains(response.callback_id()) {
        return Err(failure(
            "swallowtail.pi.rpc.callback_expired",
            "Pi RPC callback response arrived after its deadline",
        ));
    }
    if state.closed {
        return Err(callback_closed());
    }
    let pending = state.pending.get(response.callback_id()).ok_or_else(|| {
        failure(
            "swallowtail.pi.rpc.callback_unknown_or_duplicate",
            "Pi RPC callback response is unknown or was already used",
        )
    })?;
    if &pending.operation_id != response.operation_id() {
        return Err(failure(
            "swallowtail.pi.rpc.callback_turn_mismatch",
            "Pi RPC callback response belongs to a different turn",
        ));
    }
    let value = callback_value(pending, response.result())?;
    let mut pending = state
        .pending
        .remove(response.callback_id())
        .expect("validated Pi callback remains pending");
    if let Some(waiter) = pending.waiter.take() {
        waiter.wake();
    }
    Ok(value)
}

fn callback_value(
    pending: &PendingCallback,
    result: &CallbackResult,
) -> Result<Value, RuntimeFailure> {
    match result {
        CallbackResult::Failure { .. } => Ok(json!({
            "type": "extension_ui_response",
            "id": pending.provider_id,
            "cancelled": true
        })),
        CallbackResult::UserInput(response) => {
            if !pending.user_input.accepts(response) {
                return Err(failure(
                    "swallowtail.pi.rpc.callback_result_invalid",
                    "Pi RPC user-input response did not match the pending dialog",
                ));
            }
            let answer = response
                .answers()
                .next()
                .expect("validated response has one answer");
            match pending.method {
                PiUiDialogMethod::Confirm => {
                    let confirmed = answer
                        .selected_options()
                        .next()
                        .map(|id| id.as_str())
                        .ok_or_else(|| {
                            failure(
                                "swallowtail.pi.rpc.callback_confirmation_invalid",
                                "Pi RPC confirmation callback did not select an option",
                            )
                        })?
                        .parse::<bool>()
                        .map_err(|_| {
                            failure(
                                "swallowtail.pi.rpc.callback_confirmation_invalid",
                                "Pi RPC confirmation callback was not boolean",
                            )
                        })?;
                    Ok(json!({
                        "type": "extension_ui_response",
                        "id": pending.provider_id,
                        "confirmed": confirmed
                    }))
                }
                PiUiDialogMethod::Select => {
                    let text = answer
                        .selected_options()
                        .next()
                        .map(|id| id.as_str())
                        .ok_or_else(|| {
                            failure(
                                "swallowtail.pi.rpc.callback_selection_invalid",
                                "Pi RPC selection callback did not select an option",
                            )
                        })?;
                    Ok(json!({
                        "type": "extension_ui_response",
                        "id": pending.provider_id,
                        "value": text
                    }))
                }
                PiUiDialogMethod::Input | PiUiDialogMethod::Editor => {
                    let text = answer.text().map(|text| text.as_str()).ok_or_else(|| {
                        failure(
                            "swallowtail.pi.rpc.callback_input_invalid",
                            "Pi RPC input callback did not contain text",
                        )
                    })?;
                    Ok(json!({
                        "type": "extension_ui_response",
                        "id": pending.provider_id,
                        "value": text
                    }))
                }
            }
        }
        CallbackResult::Success(_) => Err(failure(
            "swallowtail.pi.rpc.callback_result_kind_mismatch",
            "Pi RPC UI callback requires a typed user-input response",
        )),
    }
}

fn callback_closed() -> RuntimeFailure {
    failure(
        "swallowtail.pi.rpc.callback_closed",
        "Pi RPC callback exchange is closed",
    )
}

fn wake(state: &mut CallbackState) {
    if let Some(waiter) = state.waiter.take() {
        waiter.wake();
    }
}
