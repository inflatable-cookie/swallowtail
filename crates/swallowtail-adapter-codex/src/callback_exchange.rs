use crate::rpc::{RpcConnection, failure};
use futures_core::Stream;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::pin::Pin;
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use swallowtail_runtime::{
    BoxCallbackStream, BoxFuture, CallbackAbandonment, CallbackExchange, CallbackFailureKind,
    CallbackId, CallbackRequest, CallbackResponder, CallbackResponse, CallbackResult,
    RuntimeFailure, RuntimeTurnId,
};

const CALLBACK_CAPACITY: usize = 64;

struct PendingCallback {
    provider_request_id: Value,
    turn_id: RuntimeTurnId,
}

struct CallbackState {
    requests: VecDeque<CallbackRequest>,
    pending: BTreeMap<CallbackId, PendingCallback>,
    provider_call_ids: BTreeSet<String>,
    abandoned_provider_requests: Vec<Value>,
    closed: bool,
    waiter: Option<Waker>,
}

pub(crate) struct CallbackHub {
    state: Arc<Mutex<CallbackState>>,
}

impl CallbackHub {
    pub(crate) fn new(connection: Weak<RpcConnection>) -> (Self, CallbackExchange) {
        let state = Arc::new(Mutex::new(CallbackState {
            requests: VecDeque::new(),
            pending: BTreeMap::new(),
            provider_call_ids: BTreeSet::new(),
            abandoned_provider_requests: Vec::new(),
            closed: false,
            waiter: None,
        }));
        let stream: BoxCallbackStream = Box::pin(CallbackRequestStream {
            state: Arc::clone(&state),
        });
        let responder: Arc<dyn CallbackResponder> = Arc::new(CodexCallbackResponder {
            state: Arc::clone(&state),
            connection,
        });
        (Self { state }, CallbackExchange::new(stream, responder))
    }

    pub(crate) fn enqueue(
        &self,
        request: CallbackRequest,
        provider_request_id: Value,
        provider_call_id: String,
    ) -> Result<(), RuntimeFailure> {
        let mut state = self.state.lock().expect("callback state lock poisoned");
        if state.closed {
            return Err(callback_closed());
        }
        if !state.provider_call_ids.insert(provider_call_id) {
            return Err(failure(
                "swallowtail.codex.app_server.callback_provider_id_reused",
                "Codex app-server reused a dynamic tool call id",
            ));
        }
        if state.pending.len() >= CALLBACK_CAPACITY || state.requests.len() >= CALLBACK_CAPACITY {
            return Err(failure(
                "swallowtail.codex.app_server.callback_capacity_exceeded",
                "Codex app-server exceeded the bounded callback capacity",
            ));
        }
        state.pending.insert(
            request.callback_id().clone(),
            PendingCallback {
                provider_request_id,
                turn_id: request.turn_id().clone(),
            },
        );
        state.requests.push_back(request);
        wake(&mut state);
        Ok(())
    }

    pub(crate) fn observe_and_close(&self, request: CallbackRequest) -> Result<(), RuntimeFailure> {
        let mut state = self.state.lock().expect("callback state lock poisoned");
        if state.closed {
            return Err(callback_closed());
        }
        if state.requests.len() >= CALLBACK_CAPACITY {
            return Err(failure(
                "swallowtail.codex.app_server.callback_capacity_exceeded",
                "Codex app-server exceeded the bounded callback capacity",
            ));
        }
        state.requests.push_back(request);
        state.closed = true;
        wake(&mut state);
        Ok(())
    }

    pub(crate) fn abandon(&self, _reason: CallbackAbandonment) {
        let mut state = self.state.lock().expect("callback state lock poisoned");
        if state.closed {
            return;
        }
        state.closed = true;
        state.requests.clear();
        let pending = std::mem::take(&mut state.pending);
        state
            .abandoned_provider_requests
            .extend(pending.into_values().map(|value| value.provider_request_id));
        wake(&mut state);
    }

    pub(crate) fn take_abandoned_provider_requests(&self) -> Vec<Value> {
        std::mem::take(
            &mut self
                .state
                .lock()
                .expect("callback state lock poisoned")
                .abandoned_provider_requests,
        )
    }
}

struct CallbackRequestStream {
    state: Arc<Mutex<CallbackState>>,
}

impl Stream for CallbackRequestStream {
    type Item = Result<CallbackRequest, RuntimeFailure>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = self.state.lock().expect("callback state lock poisoned");
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

struct CodexCallbackResponder {
    state: Arc<Mutex<CallbackState>>,
    connection: Weak<RpcConnection>,
}

impl CallbackResponder for CodexCallbackResponder {
    fn respond(&self, response: CallbackResponse) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let state = Arc::clone(&self.state);
        let connection = self.connection.clone();
        Box::pin(async move {
            let pending = claim_response(&state, &response)?;
            let connection = connection.upgrade().ok_or_else(callback_closed)?;
            let (result, invalid_success) = provider_result(response.result());
            connection
                .respond_server_request(pending.provider_request_id, result)
                .await?;
            if invalid_success {
                Err(failure(
                    "swallowtail.codex.app_server.callback_result_not_utf8",
                    "Codex dynamic tool result is not valid UTF-8",
                ))
            } else {
                Ok(())
            }
        })
    }
}

fn claim_response(
    state: &Arc<Mutex<CallbackState>>,
    response: &CallbackResponse,
) -> Result<PendingCallback, RuntimeFailure> {
    let mut state = state.lock().expect("callback state lock poisoned");
    if state.closed {
        return Err(callback_closed());
    }
    let pending = state.pending.get(response.callback_id()).ok_or_else(|| {
        failure(
            "swallowtail.codex.app_server.callback_unknown_or_duplicate",
            "Callback response id is unknown or was already used",
        )
    })?;
    if &pending.turn_id != response.turn_id() {
        return Err(failure(
            "swallowtail.codex.app_server.callback_turn_mismatch",
            "Callback response belongs to a different turn",
        ));
    }
    Ok(state
        .pending
        .remove(response.callback_id())
        .expect("validated callback remains pending"))
}

fn provider_result(result: &CallbackResult) -> (Value, bool) {
    match result {
        CallbackResult::Success(payload) => match std::str::from_utf8(payload.as_bytes()) {
            Ok(text) => (dynamic_tool_result(true, text), false),
            Err(_) => (
                dynamic_tool_result(false, "Callback result was not valid text"),
                true,
            ),
        },
        CallbackResult::Failure { kind, detail } => {
            let detail = detail
                .as_ref()
                .and_then(|value| std::str::from_utf8(value.as_bytes()).ok())
                .unwrap_or_else(|| callback_failure_text(*kind));
            (dynamic_tool_result(false, detail), false)
        }
    }
}

fn dynamic_tool_result(success: bool, text: &str) -> Value {
    serde_json::json!({
        "success": success,
        "contentItems": [{"type": "inputText", "text": text}]
    })
}

fn callback_failure_text(kind: CallbackFailureKind) -> &'static str {
    match kind {
        CallbackFailureKind::UnknownDeclaration => "Unknown tool declaration",
        CallbackFailureKind::Unsupported => "Callback kind is unsupported",
        CallbackFailureKind::ConsumerFailed => "Tool execution failed",
        CallbackFailureKind::Cancelled => "Tool execution was cancelled",
        CallbackFailureKind::TimedOut => "Tool execution timed out",
    }
}

fn callback_closed() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.callback_closed",
        "Callback exchange is closed",
    )
}

fn wake(state: &mut CallbackState) {
    if let Some(waiter) = state.waiter.take() {
        waiter.wake();
    }
}

#[cfg(test)]
mod tests {
    use super::provider_result;
    use swallowtail_runtime::{CallbackFailureKind, CallbackPayload, CallbackResult};

    #[test]
    fn consumer_failure_becomes_a_provider_tool_failure() {
        let (result, invalid) = provider_result(&CallbackResult::Failure {
            kind: CallbackFailureKind::ConsumerFailed,
            detail: Some(
                CallbackPayload::new(b"bounded failure".to_vec(), 64).expect("payload is bounded"),
            ),
        });

        assert!(!invalid);
        assert_eq!(result["success"], false);
        assert_eq!(result["contentItems"][0]["text"], "bounded failure");
    }

    #[test]
    fn non_text_success_is_rejected_without_forwarding_raw_bytes() {
        let (result, invalid) = provider_result(&CallbackResult::Success(
            CallbackPayload::new(vec![0xff], 1).expect("payload is bounded"),
        ));

        assert!(invalid);
        assert_eq!(result["success"], false);
        assert_eq!(
            result["contentItems"][0]["text"],
            "Callback result was not valid text"
        );
    }
}
