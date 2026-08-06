use crate::connection::AcpConnection;
use crate::failure::{failure, malformed};
use futures_core::Stream;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::pin::Pin;
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{ExtensionNamespace, ProviderExtension, ProviderRequestRef};
use swallowtail_runtime::{
    BoxCallbackStream, CallbackAbandonment, CallbackExchange, CallbackId, CallbackOperationId,
    CallbackRequest, CallbackResponder, Deadline, HarnessUserInputRequest, RuntimeFailure,
    RuntimeTurnId,
};

mod response;

use response::PermissionCallbackResponder;

const PERMISSION_NAMESPACE: &str = "acp/session/request-permission";
const CALLBACK_CAPACITY: usize = 16;
const CALLBACK_BYTES: usize = 64 * 1024;
const MAXIMUM_OPTION_ID_BYTES: usize = 512;

#[must_use]
/// Returns the extension namespace for Claude Agent permission callbacks.
pub fn claude_agent_permission_namespace() -> ExtensionNamespace {
    ExtensionNamespace::new(PERMISSION_NAMESPACE).expect("static namespace is valid")
}

enum PendingKind {
    Permission {
        options: BTreeSet<String>,
        reject_option_id: String,
    },
    UserInput(HarnessUserInputRequest),
}

struct PendingCallback {
    provider_id: Value,
    operation_id: CallbackOperationId,
    kind: PendingKind,
}

struct State {
    requests: VecDeque<CallbackRequest>,
    pending: BTreeMap<CallbackId, PendingCallback>,
    provider_ids: BTreeSet<String>,
    closed: bool,
    waiter: Option<Waker>,
}

pub(crate) struct CallbackHub {
    state: Arc<Mutex<State>>,
    exchanges_permissions: bool,
}

impl CallbackHub {
    pub(crate) fn new(
        connection: Weak<AcpConnection>,
        exchanges_permissions: bool,
    ) -> (Self, CallbackExchange) {
        let state = Arc::new(Mutex::new(State {
            requests: VecDeque::new(),
            pending: BTreeMap::new(),
            provider_ids: BTreeSet::new(),
            closed: false,
            waiter: None,
        }));
        let requests: BoxCallbackStream = Box::pin(PermissionCallbackStream {
            state: Arc::clone(&state),
        });
        let responder: Arc<dyn CallbackResponder> = Arc::new(PermissionCallbackResponder {
            state: Arc::clone(&state),
            connection,
        });
        (
            Self {
                state,
                exchanges_permissions,
            },
            CallbackExchange::new(requests, responder),
        )
    }

    pub(crate) const fn exchanges_permissions(&self) -> bool {
        self.exchanges_permissions
    }

    pub(crate) fn enqueue_permission(
        &self,
        turn_id: &RuntimeTurnId,
        event_sequence: u64,
        deadline: Option<Deadline>,
        provider_id: &Value,
        params: &Value,
    ) -> Result<CallbackId, RuntimeFailure> {
        let provider_key = provider_request_key(provider_id)?;
        let tool_call = params
            .get("toolCall")
            .and_then(Value::as_object)
            .filter(|tool| {
                tool.get("toolCallId")
                    .and_then(Value::as_str)
                    .is_some_and(|id| !id.is_empty())
            })
            .ok_or_else(malformed)?;
        let options = params
            .get("options")
            .and_then(Value::as_array)
            .filter(|options| !options.is_empty() && options.len() <= 32)
            .ok_or_else(malformed)?;
        let mut accepted = BTreeSet::new();
        let mut exposed = Vec::new();
        let mut reject_option_id = None;
        for option in options {
            let Some(option_id) = option.get("optionId").and_then(Value::as_str) else {
                return Err(malformed());
            };
            if option_id.is_empty() || option_id.len() > MAXIMUM_OPTION_ID_BYTES {
                return Err(malformed());
            }
            match option.get("kind").and_then(Value::as_str) {
                Some("allow_once") => {}
                Some("reject_once") => {
                    reject_option_id = Some(option_id.to_owned());
                }
                Some("allow_always" | "reject_always") => continue,
                Some(_) => return Err(malformed()),
                None => return Err(malformed()),
            }
            if !accepted.insert(option_id.to_owned()) {
                return Err(malformed());
            }
            exposed.push(option.clone());
        }
        let reject_option_id = reject_option_id.ok_or_else(|| {
            failure(
                "swallowtail.claude_agent.acp.permission_rejection_unavailable",
                "Claude Agent permission request offered no one-shot rejection",
            )
        })?;
        if accepted.is_empty() {
            return Err(malformed());
        }

        let payload = serde_json::to_vec(&json!({
            "toolCall": tool_call,
            "options": exposed,
        }))
        .map_err(|_| malformed())?;
        let provider_ref =
            ProviderRequestRef::new(format!("acp:{provider_key}")).map_err(|_| malformed())?;
        let callback_id =
            CallbackId::new(format!("{}:permission:{event_sequence}", turn_id.as_str()))
                .map_err(|_| malformed())?;
        let request = CallbackRequest::extension(
            callback_id.clone(),
            turn_id.clone(),
            event_sequence,
            deadline,
            ProviderExtension::new(claude_agent_permission_namespace(), payload),
            CALLBACK_BYTES,
        )
        .map_err(|_| malformed())?
        .with_provider_request_ref(provider_ref);

        let mut state = self
            .state
            .lock()
            .expect("permission callback lock poisoned");
        if state.closed {
            return Err(closed());
        }
        if state.pending.len() >= CALLBACK_CAPACITY || state.requests.len() >= CALLBACK_CAPACITY {
            return Err(failure(
                "swallowtail.claude_agent.acp.permission_callback_capacity",
                "Claude Agent permission callback capacity was exceeded",
            ));
        }
        if !state.provider_ids.insert(provider_key) {
            return Err(failure(
                "swallowtail.claude_agent.acp.permission_provider_id_reused",
                "Claude Agent reused a permission request id",
            ));
        }
        state.pending.insert(
            callback_id.clone(),
            PendingCallback {
                provider_id: provider_id.clone(),
                operation_id: CallbackOperationId::Turn(turn_id.clone()),
                kind: PendingKind::Permission {
                    options: accepted,
                    reject_option_id,
                },
            },
        );
        state.requests.push_back(request);
        wake(&mut state);
        Ok(callback_id)
    }

    pub(crate) fn enqueue_user_input(
        &self,
        turn_id: &RuntimeTurnId,
        event_sequence: u64,
        deadline: Option<Deadline>,
        provider_id: &Value,
        request: HarnessUserInputRequest,
    ) -> Result<CallbackId, RuntimeFailure> {
        let provider_key = provider_request_key(provider_id)?;
        let provider_ref =
            ProviderRequestRef::new(format!("acp:{provider_key}")).map_err(|_| malformed())?;
        let callback_id =
            CallbackId::new(format!("{}:elicitation:{event_sequence}", turn_id.as_str()))
                .map_err(|_| malformed())?;
        let callback = CallbackRequest::harness_user_input(
            callback_id.clone(),
            turn_id.clone(),
            event_sequence,
            deadline,
            request.clone(),
        )
        .with_provider_request_ref(provider_ref);

        let mut state = self.state.lock().expect("callback lock poisoned");
        if state.closed {
            return Err(closed());
        }
        if state.pending.len() >= CALLBACK_CAPACITY || state.requests.len() >= CALLBACK_CAPACITY {
            return Err(failure(
                "swallowtail.claude_agent.acp.callback_capacity",
                "Claude Agent callback capacity was exceeded",
            ));
        }
        if !state.provider_ids.insert(provider_key) {
            return Err(failure(
                "swallowtail.claude_agent.acp.callback_provider_id_reused",
                "Claude Agent reused a callback request id",
            ));
        }
        state.pending.insert(
            callback_id.clone(),
            PendingCallback {
                provider_id: provider_id.clone(),
                operation_id: CallbackOperationId::Turn(turn_id.clone()),
                kind: PendingKind::UserInput(request),
            },
        );
        state.requests.push_back(callback);
        wake(&mut state);
        Ok(callback_id)
    }

    pub(crate) fn abandon(&self, _reason: CallbackAbandonment) {
        let mut state = self
            .state
            .lock()
            .expect("permission callback lock poisoned");
        state.closed = true;
        state.requests.clear();
        state.pending.clear();
        wake(&mut state);
    }
}

struct PermissionCallbackStream {
    state: Arc<Mutex<State>>,
}

impl Stream for PermissionCallbackStream {
    type Item = Result<CallbackRequest, RuntimeFailure>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = self
            .state
            .lock()
            .expect("permission callback lock poisoned");
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

fn provider_request_key(value: &Value) -> Result<String, RuntimeFailure> {
    match value {
        Value::String(value) if !value.is_empty() => Ok(format!("string:{value}")),
        Value::Number(value) => Ok(format!("number:{value}")),
        _ => Err(malformed()),
    }
}

fn wake(state: &mut State) {
    if let Some(waiter) = state.waiter.take() {
        waiter.wake();
    }
}

fn closed() -> RuntimeFailure {
    failure(
        "swallowtail.claude_agent.acp.permission_callback_closed",
        "Claude Agent permission callback exchange is closed",
    )
}
