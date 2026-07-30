use crate::failure::failure;
use crate::protocol::{
    PendingProviderRequest, ProviderRequestKind, callback_response, question_request,
};
use crate::transport::CurlTransport;
use futures_core::Stream;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{ExtensionNamespace, ProviderExtension, ProviderRequestRef};
use swallowtail_runtime::{
    BoxCallbackStream, BoxFuture, CallbackAbandonment, CallbackExchange, CallbackId,
    CallbackOperationId, CallbackRequest, CallbackResponder, CallbackResponse, HostServices,
    RuntimeFailure, ScopeId,
};

const CALLBACK_CAPACITY: usize = 16;
const CALLBACK_BYTES: usize = 64 * 1024;

pub(crate) fn permission_namespace() -> ExtensionNamespace {
    ExtensionNamespace::new("opencode/permission").expect("static namespace is valid")
}

pub(crate) fn question_namespace() -> ExtensionNamespace {
    ExtensionNamespace::new("opencode/question").expect("static namespace is valid")
}

struct Pending {
    provider_id: String,
    kind: ProviderRequestKind,
    operation_id: CallbackOperationId,
    user_input: Option<swallowtail_runtime::HarnessUserInputRequest>,
}

struct State {
    requests: VecDeque<CallbackRequest>,
    pending: BTreeMap<CallbackId, Pending>,
    provider_ids: BTreeSet<String>,
    closed: bool,
    next_id: u64,
    waiter: Option<Waker>,
}

#[derive(Clone)]
pub(super) struct CallbackHub {
    state: Arc<Mutex<State>>,
}

struct ResponseContext {
    state: Arc<Mutex<State>>,
    scope: ScopeId,
    directory: String,
    endpoint: String,
    services: HostServices,
    transport: CurlTransport,
}

impl CallbackHub {
    pub(super) fn new(
        scope: ScopeId,
        directory: String,
        endpoint: String,
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
        let requests: BoxCallbackStream = Box::pin(RequestStream {
            state: Arc::clone(&state),
        });
        let responder: Arc<dyn CallbackResponder> = Arc::new(ResponseContext {
            state: Arc::clone(&state),
            scope,
            directory,
            endpoint,
            services,
            transport,
        });
        (Self { state }, CallbackExchange::new(requests, responder))
    }

    pub(super) fn enqueue(
        &self,
        operation_id: CallbackOperationId,
        sequence: u64,
        deadline: Option<swallowtail_runtime::Deadline>,
        provider: PendingProviderRequest,
    ) -> Result<CallbackId, RuntimeFailure> {
        let mut state = self.state.lock().expect("OpenCode callback lock poisoned");
        if state.closed {
            return Err(closed());
        }
        if state.pending.len() >= CALLBACK_CAPACITY || state.requests.len() >= CALLBACK_CAPACITY {
            return Err(callback_failure(
                "capacity",
                "OpenCode callback capacity was exceeded",
            ));
        }
        if !state.provider_ids.insert(provider.id.clone()) {
            return Err(callback_failure(
                "provider_id_reused",
                "OpenCode reused a provider request identity",
            ));
        }
        let callback_id = CallbackId::new(format!("opencode-callback-{}", state.next_id))
            .map_err(|_| malformed())?;
        state.next_id += 1;
        let namespace = match provider.kind {
            ProviderRequestKind::Permission => permission_namespace(),
            ProviderRequestKind::Question { .. } => question_namespace(),
        };
        let provider_ref = ProviderRequestRef::new(&provider.id).map_err(|_| malformed())?;
        let user_input = match provider.kind {
            ProviderRequestKind::Permission => None,
            ProviderRequestKind::Question { .. } => Some(question_request(&provider.payload)?),
        };
        let request = match (&operation_id, &user_input) {
            (CallbackOperationId::Turn(turn_id), Some(user_input)) => {
                CallbackRequest::harness_user_input(
                    callback_id.clone(),
                    turn_id.clone(),
                    sequence,
                    deadline,
                    user_input.clone(),
                )
            }
            (CallbackOperationId::Run(run_id), Some(user_input)) => {
                CallbackRequest::run_harness_user_input(
                    callback_id.clone(),
                    run_id.clone(),
                    sequence,
                    deadline,
                    user_input.clone(),
                )
            }
            (CallbackOperationId::Turn(turn_id), None) => CallbackRequest::extension(
                callback_id.clone(),
                turn_id.clone(),
                sequence,
                deadline,
                ProviderExtension::new(namespace, provider.payload),
                CALLBACK_BYTES,
            )
            .map_err(|_| malformed())?,
            (CallbackOperationId::Run(run_id), None) => CallbackRequest::run_extension(
                callback_id.clone(),
                run_id.clone(),
                sequence,
                deadline,
                ProviderExtension::new(namespace, provider.payload),
                CALLBACK_BYTES,
            )
            .map_err(|_| malformed())?,
        }
        .with_provider_request_ref(provider_ref);
        state.pending.insert(
            callback_id.clone(),
            Pending {
                provider_id: provider.id,
                kind: provider.kind,
                operation_id,
                user_input,
            },
        );
        state.requests.push_back(request);
        wake(&mut state);
        Ok(callback_id)
    }

    pub(super) fn abandon(&self, _reason: CallbackAbandonment) {
        let mut state = self.state.lock().expect("OpenCode callback lock poisoned");
        state.closed = true;
        state.requests.clear();
        state.pending.clear();
        wake(&mut state);
    }
}

impl CallbackResponder for ResponseContext {
    fn respond(&self, response: CallbackResponse) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            let request = {
                let mut state = self.state.lock().expect("OpenCode callback lock poisoned");
                if state.closed {
                    return Err(closed());
                }
                let pending = state.pending.get(response.callback_id()).ok_or_else(|| {
                    callback_failure(
                        "unknown_or_duplicate",
                        "OpenCode callback response was unknown or already used",
                    )
                })?;
                if &pending.operation_id != response.operation_id() {
                    return Err(callback_failure(
                        "operation_mismatch",
                        "OpenCode callback response belongs to another operation",
                    ));
                }
                let request = callback_response(
                    &pending.provider_id,
                    pending.kind,
                    pending.user_input.as_ref(),
                    response.result(),
                )?
                .with_directory(&self.directory);
                state
                    .pending
                    .remove(response.callback_id())
                    .expect("validated callback remains pending");
                request
            };
            let response = self
                .transport
                .request(
                    self.scope.clone(),
                    self.endpoint.clone(),
                    request,
                    &self.services,
                    Arc::new(AtomicBool::new(false)),
                )
                .await?;
            if (200..300).contains(&response.status) {
                Ok(())
            } else {
                Err(callback_failure(
                    "provider_rejected",
                    "OpenCode rejected the callback response",
                ))
            }
        })
    }
}

struct RequestStream {
    state: Arc<Mutex<State>>,
}

impl Stream for RequestStream {
    type Item = Result<CallbackRequest, RuntimeFailure>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = self.state.lock().expect("OpenCode callback lock poisoned");
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
    callback_failure("malformed", "OpenCode callback data was malformed")
}

fn closed() -> RuntimeFailure {
    callback_failure("closed", "OpenCode callback exchange is closed")
}

fn callback_failure(suffix: &str, message: &str) -> RuntimeFailure {
    let code = match suffix {
        "capacity" => "swallowtail.opencode.callback_capacity",
        "provider_id_reused" => "swallowtail.opencode.callback_provider_id_reused",
        "unknown_or_duplicate" => "swallowtail.opencode.callback_unknown_or_duplicate",
        "operation_mismatch" => "swallowtail.opencode.callback_operation_mismatch",
        "provider_rejected" => "swallowtail.opencode.callback_provider_rejected",
        "malformed" => "swallowtail.opencode.callback_malformed",
        "closed" => "swallowtail.opencode.callback_closed",
        _ => "swallowtail.opencode.callback_failed",
    };
    failure(code, message)
}
