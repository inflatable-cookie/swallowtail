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
/// Maximum number of recent provider callback ids retained for duplicate rejection.
const MAXIMUM_TRACKED_PROVIDER_IDS: usize = 1024;

pub(super) struct BoundedProviderIds {
    entries: BTreeSet<String>,
    order: VecDeque<String>,
    capacity: usize,
}

impl BoundedProviderIds {
    pub(super) fn new(capacity: usize) -> Self {
        Self {
            entries: BTreeSet::new(),
            order: VecDeque::new(),
            capacity,
        }
    }

    fn insert(&mut self, id: String) -> bool {
        if self.entries.contains(&id) {
            return false;
        }
        while self.entries.len() >= self.capacity {
            if let Some(oldest) = self.order.pop_front() {
                self.entries.remove(&oldest);
            } else {
                break;
            }
        }
        self.entries.insert(id.clone());
        self.order.push_back(id);
        true
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.order.clear();
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.entries.len()
    }
}

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
    provider_key: String,
    operation_id: CallbackOperationId,
    kind: PendingKind,
}

struct State {
    requests: VecDeque<CallbackRequest>,
    pending: BTreeMap<CallbackId, PendingCallback>,
    provider_ids: BoundedProviderIds,
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
            provider_ids: BoundedProviderIds::new(MAXIMUM_TRACKED_PROVIDER_IDS),
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
        if state
            .pending
            .values()
            .any(|pending| pending.provider_key == provider_key)
            || !state.provider_ids.insert(provider_key.clone())
        {
            return Err(failure(
                "swallowtail.claude_agent.acp.permission_provider_id_reused",
                "Claude Agent reused a permission request id",
            ));
        }
        state.pending.insert(
            callback_id.clone(),
            PendingCallback {
                provider_id: provider_id.clone(),
                provider_key,
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
        if state
            .pending
            .values()
            .any(|pending| pending.provider_key == provider_key)
            || !state.provider_ids.insert(provider_key.clone())
        {
            return Err(failure(
                "swallowtail.claude_agent.acp.callback_provider_id_reused",
                "Claude Agent reused a callback request id",
            ));
        }
        state.pending.insert(
            callback_id.clone(),
            PendingCallback {
                provider_id: provider_id.clone(),
                provider_key,
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
        state.provider_ids.clear();
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

#[cfg(test)]
mod tests {
    use super::*;
    use swallowtail_runtime::{
        HarnessQuestionId, HarnessQuestionOptionId, HarnessUserInputChoiceMode,
        HarnessUserInputOption, HarnessUserInputQuestion, HarnessUserInputQuestionKind,
        OperationContent,
    };

    fn permission_params() -> Value {
        json!({
            "toolCall": { "toolCallId": "tool-1" },
            "options": [
                { "optionId": "allow-1", "kind": "allow_once" },
                { "optionId": "reject-1", "kind": "reject_once" }
            ]
        })
    }

    fn user_input_request() -> HarnessUserInputRequest {
        HarnessUserInputRequest::new(
            vec![
                HarnessUserInputQuestion::new(
                    HarnessQuestionId::new("q1").expect("id"),
                    OperationContent::new("header").expect("content"),
                    OperationContent::new("prompt").expect("content"),
                    HarnessUserInputQuestionKind::Choice {
                        mode: HarnessUserInputChoiceMode::Single,
                        allow_other: false,
                    },
                    vec![
                        HarnessUserInputOption::new(
                            HarnessQuestionOptionId::new("a").expect("id"),
                            OperationContent::new("A").expect("content"),
                            None,
                        ),
                        HarnessUserInputOption::new(
                            HarnessQuestionOptionId::new("b").expect("id"),
                            OperationContent::new("B").expect("content"),
                            None,
                        ),
                    ],
                )
                .expect("question"),
            ],
            None,
            4,
            4,
            4096,
        )
        .expect("request")
    }

    #[test]
    fn provider_ids_stays_bounded_and_rejects_recent_duplicates() {
        let mut ids = BoundedProviderIds::new(3);
        assert!(ids.insert("number:1".to_owned()));
        assert!(ids.insert("number:2".to_owned()));
        assert!(ids.insert("number:3".to_owned()));
        assert!(!ids.insert("number:2".to_owned()));
        assert!(ids.insert("number:4".to_owned()));
        assert_eq!(ids.len(), 3);
        assert!(!ids.insert("number:4".to_owned()));
        assert!(ids.insert("number:1".to_owned()));
        assert_eq!(ids.len(), 3);
        ids.clear();
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn outstanding_pending_provider_id_stays_reserved_across_cache_turnover() {
        let state = Arc::new(Mutex::new(State {
            requests: VecDeque::new(),
            pending: BTreeMap::new(),
            provider_ids: BoundedProviderIds::new(2),
            closed: false,
            waiter: None,
        }));
        let hub = CallbackHub {
            state,
            exchanges_permissions: true,
        };
        let turn_id = RuntimeTurnId::new("turn-turnover").expect("turn id");
        hub.enqueue_permission(&turn_id, 1, None, &json!(100), &permission_params())
            .expect("first permission enqueues");
        // Cycle two more provider IDs so "number:100" is evicted from the
        // 2-entry recent-ID cache while callback 100 remains pending.
        hub.enqueue_permission(&turn_id, 2, None, &json!(200), &permission_params())
            .expect("second permission enqueues");
        hub.enqueue_permission(&turn_id, 3, None, &json!(300), &permission_params())
            .expect("third permission enqueues");

        let dup_permission = hub
            .enqueue_permission(&turn_id, 4, None, &json!(100), &permission_params())
            .expect_err("pending provider id must remain reserved for permissions");
        assert_eq!(
            dup_permission.diagnostic().code(),
            "swallowtail.claude_agent.acp.permission_provider_id_reused"
        );
        let dup_user_input = hub
            .enqueue_user_input(&turn_id, 5, None, &json!(100), user_input_request())
            .expect_err("pending provider id must remain reserved for user input");
        assert_eq!(
            dup_user_input.diagnostic().code(),
            "swallowtail.claude_agent.acp.callback_provider_id_reused"
        );
    }
}
