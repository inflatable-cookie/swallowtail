#[path = "callbacks/response.rs"]
mod response;

use self::response::ResponseContext;
use super::access::SecretMaterial;
use crate::failure::failure;
use crate::local_server::protocol::PendingProviderRequest;
use crate::local_server::transport::CurlTransport;
use futures_core::Stream;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::pin::Pin;
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{ExtensionNamespace, ProviderExtension, ProviderRequestRef};
use swallowtail_runtime::{
    BoxCallbackStream, CallbackAbandonment, CallbackExchange, CallbackId, CallbackOperationId,
    CallbackRequest, CallbackResponder, HarnessQuestionId, HarnessQuestionOptionId,
    HarnessUserInputChoiceMode, HarnessUserInputOption, HarnessUserInputQuestion,
    HarnessUserInputQuestionKind, HarnessUserInputRequest, HostServices, OperationContent,
    RuntimeFailure, RuntimeTurnId, ScopeId,
};

const CALLBACK_CAPACITY: usize = 32;
const CALLBACK_BYTES: usize = 256 * 1024;
const APPROVAL_NAMESPACE: &str = "kimi.local-server/approval-v1";
const QUESTION_NAMESPACE: &str = "kimi.local-server/question-v1";

pub(in crate::local_server) fn approval_namespace() -> ExtensionNamespace {
    ExtensionNamespace::new(APPROVAL_NAMESPACE).expect("static namespace is valid")
}

pub(in crate::local_server) fn question_namespace() -> ExtensionNamespace {
    ExtensionNamespace::new(QUESTION_NAMESPACE).expect("static namespace is valid")
}

#[derive(Clone, Copy)]
pub(super) enum ProviderCallbackKind {
    Approval,
    Question,
}

struct PendingCallback {
    operation_id: CallbackOperationId,
    provider_id: String,
    kind: ProviderCallbackKind,
    user_input: Option<HarnessUserInputRequest>,
}

struct State {
    requests: VecDeque<CallbackRequest>,
    pending: BTreeMap<CallbackId, PendingCallback>,
    provider_ids: BTreeSet<String>,
    closed: bool,
    next_id: u64,
    waiter: Option<Waker>,
}

pub(super) struct CallbackHub {
    state: Arc<Mutex<State>>,
}

impl CallbackHub {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        scope: ScopeId,
        provider_session_id: String,
        endpoint: String,
        secret: Weak<SecretMaterial>,
        services: HostServices,
        transport: CurlTransport,
    ) -> (Self, CallbackExchange) {
        let state = Arc::new(Mutex::new(State {
            requests: VecDeque::new(),
            pending: BTreeMap::new(),
            provider_ids: BTreeSet::new(),
            closed: false,
            next_id: 1,
            waiter: None,
        }));
        let requests: BoxCallbackStream = Box::pin(CallbackStream {
            state: Arc::clone(&state),
        });
        let responder: Arc<dyn CallbackResponder> = Arc::new(ResponseContext {
            state: Arc::clone(&state),
            scope,
            provider_session_id,
            endpoint,
            secret,
            services,
            transport,
        });
        (Self { state }, CallbackExchange::new(requests, responder))
    }

    pub(super) fn enqueue(
        &self,
        turn_id: &RuntimeTurnId,
        event_sequence: u64,
        deadline: Option<swallowtail_runtime::Deadline>,
        kind: ProviderCallbackKind,
        provider: PendingProviderRequest,
    ) -> Result<CallbackId, RuntimeFailure> {
        let mut state = self.state.lock().expect("callback state lock poisoned");
        if state.closed {
            return Err(closed());
        }
        if !state.provider_ids.insert(provider.id.clone()) {
            return Err(failure(
                "swallowtail.kimi.local_server.callback_provider_id_reused",
                "Kimi local server reused a pending interaction id",
            ));
        }
        if state.pending.len() >= CALLBACK_CAPACITY || state.requests.len() >= CALLBACK_CAPACITY {
            return Err(failure(
                "swallowtail.kimi.local_server.callback_capacity",
                "Kimi local-server callback capacity was exceeded",
            ));
        }
        let callback_id = CallbackId::new(format!("kimi-local-callback-{}", state.next_id))
            .map_err(|_| closed())?;
        state.next_id += 1;
        let namespace = match kind {
            ProviderCallbackKind::Approval => approval_namespace(),
            ProviderCallbackKind::Question => question_namespace(),
        };
        let provider_ref = ProviderRequestRef::new(&provider.id).map_err(|_| malformed())?;
        let user_input = match kind {
            ProviderCallbackKind::Approval => None,
            ProviderCallbackKind::Question => Some(question_request(&provider.payload)?),
        };
        let request = match &user_input {
            Some(user_input) => CallbackRequest::harness_user_input(
                callback_id.clone(),
                turn_id.clone(),
                event_sequence,
                deadline,
                user_input.clone(),
            ),
            None => CallbackRequest::extension(
                callback_id.clone(),
                turn_id.clone(),
                event_sequence,
                deadline,
                ProviderExtension::new(namespace, provider.payload),
                CALLBACK_BYTES,
            )
            .map_err(|_| malformed())?,
        }
        .with_provider_request_ref(provider_ref);
        state.pending.insert(
            callback_id.clone(),
            PendingCallback {
                operation_id: CallbackOperationId::Turn(turn_id.clone()),
                provider_id: provider.id,
                kind,
                user_input,
            },
        );
        state.requests.push_back(request);
        wake(&mut state);
        Ok(callback_id)
    }

    pub(super) fn abandon(&self, _reason: CallbackAbandonment) {
        let mut state = self.state.lock().expect("callback state lock poisoned");
        state.closed = true;
        state.pending.clear();
        state.requests.clear();
        wake(&mut state);
    }
}

struct CallbackStream {
    state: Arc<Mutex<State>>,
}

impl Stream for CallbackStream {
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

fn wake(state: &mut State) {
    if let Some(waiter) = state.waiter.take() {
        waiter.wake();
    }
}

fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.callback_malformed",
        "Kimi local-server callback data is malformed",
    )
}

fn question_request(payload: &[u8]) -> Result<HarnessUserInputRequest, RuntimeFailure> {
    let value: serde_json::Value = serde_json::from_slice(payload).map_err(|_| malformed())?;
    let questions = value
        .get("questions")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(malformed)?
        .iter()
        .map(|question| {
            let question = question.as_object().ok_or_else(malformed)?;
            let id = question
                .get("id")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(malformed)?;
            let prompt = question
                .get("question")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(malformed)?;
            let options = question
                .get("options")
                .and_then(serde_json::Value::as_array)
                .ok_or_else(malformed)?
                .iter()
                .map(|option| {
                    let option = option.as_object().ok_or_else(malformed)?;
                    let id = option
                        .get("id")
                        .and_then(serde_json::Value::as_str)
                        .ok_or_else(malformed)?;
                    let label = option
                        .get("label")
                        .and_then(serde_json::Value::as_str)
                        .ok_or_else(malformed)?;
                    Ok(HarnessUserInputOption::new(
                        HarnessQuestionOptionId::new(id).map_err(|_| malformed())?,
                        OperationContent::new(label).map_err(|_| malformed())?,
                        None,
                    ))
                })
                .collect::<Result<Vec<_>, RuntimeFailure>>()?;
            HarnessUserInputQuestion::new(
                HarnessQuestionId::new(id).map_err(|_| malformed())?,
                OperationContent::new(prompt).map_err(|_| malformed())?,
                OperationContent::new(prompt).map_err(|_| malformed())?,
                HarnessUserInputQuestionKind::Choice {
                    mode: HarnessUserInputChoiceMode::Single,
                    allow_other: false,
                },
                options,
            )
            .map_err(|_| malformed())
        })
        .collect::<Result<Vec<_>, _>>()?;
    HarnessUserInputRequest::new(questions, None, 4, 4, CALLBACK_BYTES).map_err(|_| malformed())
}

fn closed() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.callback_closed",
        "Kimi local-server callback exchange is closed",
    )
}
