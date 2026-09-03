//! Correlated `canUseTool` admission for the Claude Agent SDK sidecar route.
//!
//! Admission is the consumer's decision, never the sidecar's. Every request
//! is bounded, correlated to one turn, answered at most once, and fails
//! closed: a consumer failure, an abandoned turn, or a closed exchange all
//! deny. Only the tool name crosses the boundary; no tool input, provider
//! payload, or path is exposed.

use super::connection::SdkConnection;
use super::failure::failure;
use super::wire::ClaudeAgentSdkToolDecision;
use futures_core::Stream;
use serde_json::{Value, json};
use std::collections::{BTreeMap, VecDeque};
use std::pin::Pin;
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{ExtensionNamespace, ProviderExtension, ProviderRequestRef};
use swallowtail_runtime::{
    BoxCallbackStream, BoxFuture, CallbackAbandonment, CallbackExchange, CallbackId,
    CallbackOperationId, CallbackPayload, CallbackRequest, CallbackResponder, CallbackResponse,
    CallbackResult, Deadline, RuntimeFailure, RuntimeTurnId,
};

const ADMISSION_NAMESPACE: &str = "claude-agent-sdk/can-use-tool";
const CALLBACK_CAPACITY: usize = 8;
const CALLBACK_BYTES: usize = 4 * 1024;

/// Returns the extension namespace for Claude Agent SDK tool admission.
///
/// This is deliberately route-local: the SDK route's admission semantics are
/// not proved portable by a second provider, so it does not reuse the ACP
/// permission namespace.
#[must_use]
pub fn claude_agent_sdk_tool_admission_namespace() -> ExtensionNamespace {
    ExtensionNamespace::new(ADMISSION_NAMESPACE).expect("static namespace is valid")
}

struct Pending {
    sidecar_id: String,
    operation_id: CallbackOperationId,
}

struct State {
    requests: VecDeque<CallbackRequest>,
    pending: BTreeMap<CallbackId, Pending>,
    sidecar_ids: BTreeMap<String, CallbackId>,
    closed: bool,
    waiter: Option<Waker>,
}

pub(crate) struct AdmissionHub {
    state: Arc<Mutex<State>>,
}

impl AdmissionHub {
    pub(crate) fn new(connection: Weak<SdkConnection>) -> (Self, CallbackExchange) {
        let state = Arc::new(Mutex::new(State {
            requests: VecDeque::new(),
            pending: BTreeMap::new(),
            sidecar_ids: BTreeMap::new(),
            closed: false,
            waiter: None,
        }));
        let requests: BoxCallbackStream = Box::pin(AdmissionStream {
            state: Arc::clone(&state),
        });
        let responder: Arc<dyn CallbackResponder> = Arc::new(AdmissionResponder {
            state: Arc::clone(&state),
            connection,
        });
        (
            Self {
                state: Arc::clone(&state),
            },
            CallbackExchange::new(requests, responder),
        )
    }

    /// Enqueues one bounded admission request for the active turn. `None`
    /// reports a closed exchange, which is a race with the turn's own end
    /// rather than a protocol failure.
    pub(crate) fn enqueue(
        &self,
        turn_id: &RuntimeTurnId,
        event_sequence: u64,
        deadline: Option<Deadline>,
        sidecar_id: &str,
        tool_name: &str,
    ) -> Result<Option<CallbackId>, RuntimeFailure> {
        let payload = serde_json::to_vec(&json!({"toolName": tool_name}))
            .map_err(|_| admission_failure("payload was invalid"))?;
        let callback_id = CallbackId::new(format!(
            "{}:can-use-tool:{event_sequence}",
            turn_id.as_str()
        ))
        .map_err(|_| admission_failure("identity was invalid"))?;
        let provider_ref = ProviderRequestRef::new(format!("claude-agent-sdk:{sidecar_id}"))
            .map_err(|_| admission_failure("identity was invalid"))?;
        let request = CallbackRequest::extension(
            callback_id.clone(),
            turn_id.clone(),
            event_sequence,
            deadline,
            ProviderExtension::new(claude_agent_sdk_tool_admission_namespace(), payload),
            CALLBACK_BYTES,
        )
        .map_err(|_| admission_failure("request was invalid"))?
        .with_provider_request_ref(provider_ref);

        let mut state = self.state.lock().expect("SDK admission lock poisoned");
        if state.closed {
            return Ok(None);
        }
        if state.pending.len() >= CALLBACK_CAPACITY || state.requests.len() >= CALLBACK_CAPACITY {
            return Err(admission_failure("capacity was exceeded"));
        }
        if state
            .sidecar_ids
            .insert(sidecar_id.to_owned(), callback_id.clone())
            .is_some()
        {
            return Err(admission_failure("request id was reused"));
        }
        state.pending.insert(
            callback_id.clone(),
            Pending {
                sidecar_id: sidecar_id.to_owned(),
                operation_id: CallbackOperationId::Turn(turn_id.clone()),
            },
        );
        state.requests.push_back(request);
        wake(&mut state);
        Ok(Some(callback_id))
    }

    /// Abandons every outstanding admission request. Outstanding requests are
    /// denied on the wire by the sidecar's own close path.
    pub(crate) fn abandon(&self, _reason: CallbackAbandonment) {
        let mut state = self.state.lock().expect("SDK admission lock poisoned");
        state.closed = true;
        state.requests.clear();
        state.pending.clear();
        state.sidecar_ids.clear();
        wake(&mut state);
    }
}

struct AdmissionResponder {
    state: Arc<Mutex<State>>,
    connection: Weak<SdkConnection>,
}

impl CallbackResponder for AdmissionResponder {
    fn respond(&self, response: CallbackResponse) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let state = Arc::clone(&self.state);
        let connection = self.connection.clone();
        Box::pin(async move {
            let (sidecar_id, decision) = claim(&state, &response)?;
            let connection = connection.upgrade().ok_or_else(closed)?;
            connection.respond_admission(&sidecar_id, decision).await
        })
    }
}

fn claim(
    state: &Arc<Mutex<State>>,
    response: &CallbackResponse,
) -> Result<(String, ClaudeAgentSdkToolDecision), RuntimeFailure> {
    let mut state = state.lock().expect("SDK admission lock poisoned");
    if state.closed {
        return Err(closed());
    }
    let pending = state
        .pending
        .get(response.callback_id())
        .ok_or_else(|| admission_failure("response is unknown or was already used"))?;
    if &pending.operation_id != response.operation_id() {
        return Err(admission_failure("response belongs to a different turn"));
    }
    let sidecar_id = pending.sidecar_id.clone();
    let decision = match response.result() {
        CallbackResult::Failure { .. } => ClaudeAgentSdkToolDecision::Deny,
        CallbackResult::Success(payload) => decode_decision(payload)?,
        CallbackResult::UserInput(_) => {
            return Err(admission_failure("response carried user input"));
        }
    };
    state
        .pending
        .remove(response.callback_id())
        .expect("validated admission remains pending");
    state.sidecar_ids.remove(&sidecar_id);
    Ok((sidecar_id, decision))
}

fn decode_decision(
    payload: &CallbackPayload,
) -> Result<ClaudeAgentSdkToolDecision, RuntimeFailure> {
    let value: Value = serde_json::from_slice(payload.as_bytes())
        .map_err(|_| admission_failure("response was malformed"))?;
    let object = value
        .as_object()
        .filter(|object| object.len() == 1)
        .ok_or_else(|| admission_failure("response was malformed"))?;
    match object.get("decision").and_then(Value::as_str) {
        Some("allow") => Ok(ClaudeAgentSdkToolDecision::Allow),
        Some("deny") => Ok(ClaudeAgentSdkToolDecision::Deny),
        _ => Err(admission_failure("decision was outside the offered set")),
    }
}

struct AdmissionStream {
    state: Arc<Mutex<State>>,
}

impl Stream for AdmissionStream {
    type Item = Result<CallbackRequest, RuntimeFailure>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = self.state.lock().expect("SDK admission lock poisoned");
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

fn wake(state: &mut State) {
    if let Some(waiter) = state.waiter.take() {
        waiter.wake();
    }
}

fn admission_failure(detail: &str) -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.tool_admission_invalid",
        format!("Claude Agent SDK tool admission {detail}"),
    )
}

fn closed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.tool_admission_closed",
        "Claude Agent SDK tool admission exchange is closed",
    )
}
