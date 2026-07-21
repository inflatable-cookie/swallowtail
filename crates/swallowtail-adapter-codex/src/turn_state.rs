use crate::callback_exchange::CallbackHub;
use crate::rpc::RpcConnection;
use crate::rpc::failure;
use crate::session_access::provider_request_namespace;
use serde_json::Value;
use std::collections::BTreeSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{ProviderExtension, ProviderRequestHandling, ProviderRequestRef};
use swallowtail_runtime::{
    BoxEventStream, CallbackAbandonment, CallbackExchange, CallbackId, CallbackPayload,
    CallbackRequest, CleanupOutcome, Deadline, OperationContent, ProviderRequestObservation,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeTurnId, TerminalOutcome,
    TerminalOutcomeFuture, TerminalOutcomeSender, TerminalStatus, runtime_event_channel,
    terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 256;
const MAX_CALLBACK_ARGUMENT_BYTES: usize = 1_048_576;

#[derive(Default)]
struct TurnFinishedState {
    finished: bool,
    waiter: Option<Waker>,
}

pub(crate) struct TurnFinishedFuture(Arc<Mutex<TurnFinishedState>>);

impl Future for TurnFinishedFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().expect("turn-finished lock poisoned");
        if state.finished {
            Poll::Ready(())
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

pub(crate) struct ActiveTurn {
    runtime_id: RuntimeTurnId,
    provider_thread_id: String,
    provider_id: Mutex<Option<String>>,
    deadline: Option<Deadline>,
    declared_tools: BTreeSet<String>,
    provider_requests: swallowtail_core::ProviderRequestPolicy,
    callbacks: CallbackHub,
    events: swallowtail_runtime::RuntimeEventSender,
    terminal: TerminalOutcomeSender,
    sequence: AtomicU64,
    final_output: Mutex<Option<OperationContent>>,
    delta_output: Mutex<String>,
    cancelled: AtomicBool,
    timed_out: AtomicBool,
    finished: AtomicBool,
    finish_signal: Arc<Mutex<TurnFinishedState>>,
}

impl ActiveTurn {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        deadline: Option<Deadline>,
        declared_tools: BTreeSet<String>,
        provider_requests: swallowtail_core::ProviderRequestPolicy,
        provider_thread_id: String,
        connection: Weak<RpcConnection>,
    ) -> Result<
        (
            Arc<Self>,
            BoxEventStream,
            CallbackExchange,
            TerminalOutcomeFuture,
        ),
        RuntimeFailure,
    > {
        let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let (terminal, future) = terminal_outcome_channel();
        let (callbacks, exchange) = CallbackHub::new(connection);
        Ok((
            Arc::new(Self {
                runtime_id,
                provider_thread_id,
                provider_id: Mutex::new(None),
                deadline,
                declared_tools,
                provider_requests,
                callbacks,
                events,
                terminal,
                sequence: AtomicU64::new(1),
                final_output: Mutex::new(None),
                delta_output: Mutex::new(String::new()),
                cancelled: AtomicBool::new(false),
                timed_out: AtomicBool::new(false),
                finished: AtomicBool::new(false),
                finish_signal: Arc::new(Mutex::new(TurnFinishedState::default())),
            }),
            Box::pin(stream),
            exchange,
            future,
        ))
    }

    pub(crate) const fn runtime_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    pub(crate) fn set_provider_id(&self, provider_id: &str) -> Result<(), RuntimeFailure> {
        let mut current = self
            .provider_id
            .lock()
            .expect("provider turn id lock poisoned");
        if current
            .as_ref()
            .is_some_and(|existing| existing != provider_id)
        {
            return Err(failure(
                "swallowtail.codex.app_server.turn_id_mismatch",
                "Codex app-server changed the active turn id",
            ));
        }
        *current = Some(provider_id.to_owned());
        Ok(())
    }

    pub(crate) fn mark_cancelled(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
        self.callbacks.abandon(CallbackAbandonment::TurnCancelled);
    }

    pub(crate) fn mark_timed_out(&self) {
        self.timed_out.store(true, Ordering::SeqCst);
        self.callbacks.abandon(CallbackAbandonment::TimedOut);
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }

    pub(crate) fn is_stopping(&self) -> bool {
        self.is_finished()
            || self.cancelled.load(Ordering::SeqCst)
            || self.timed_out.load(Ordering::SeqCst)
    }

    pub(crate) fn finished_future(&self) -> TurnFinishedFuture {
        TurnFinishedFuture(Arc::clone(&self.finish_signal))
    }

    pub(crate) fn take_abandoned_provider_requests(&self) -> Vec<Value> {
        self.callbacks.take_abandoned_provider_requests()
    }

    pub(crate) fn handle_tool_call(
        &self,
        provider_request_id: Value,
        params: &Value,
        callback_id: CallbackId,
    ) -> Result<(), RuntimeFailure> {
        self.verify_turn(params)?;
        if params
            .get("namespace")
            .is_some_and(|value| !value.is_null())
        {
            return Err(failure(
                "swallowtail.codex.app_server.callback_namespace_unsupported",
                "Codex namespaced dynamic tool callbacks are unsupported",
            ));
        }
        let provider_call_id = required_text(params, "callId")?.to_owned();
        let tool_name = required_text(params, "tool")?;
        if !self.declared_tools.contains(tool_name) {
            return Err(failure(
                "swallowtail.codex.app_server.callback_tool_undeclared",
                "Codex app-server requested an undeclared dynamic tool",
            ));
        }
        let arguments = params.get("arguments").ok_or_else(malformed_notification)?;
        let argument_bytes = serde_json::to_vec(arguments).map_err(|_| malformed_notification())?;
        let payload =
            CallbackPayload::new(argument_bytes, MAX_CALLBACK_ARGUMENT_BYTES).map_err(|_| {
                failure(
                    "swallowtail.codex.app_server.callback_arguments_too_large",
                    "Codex dynamic tool arguments exceeded the adapter limit",
                )
            })?;
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        let request = CallbackRequest::tool_call(
            callback_id.clone(),
            self.runtime_id.clone(),
            sequence,
            self.deadline,
            tool_name,
            payload,
        )
        .map_err(|_| malformed_notification())?;
        self.callbacks
            .enqueue(request, provider_request_id, provider_call_id)?;
        self.events.send(RuntimeEvent::new(
            sequence,
            RuntimeEventKind::CallbackRequested(callback_id),
        ))
    }

    pub(crate) fn handle_provider_request(
        &self,
        provider_request_id: &Value,
        method: &str,
        params: &Value,
        callback_id: CallbackId,
    ) -> Result<ProviderRequestObservation, RuntimeFailure> {
        let namespace = provider_request_namespace(method).ok_or_else(|| {
            failure(
                "swallowtail.codex.app_server.callback_unsupported",
                "Codex app-server requested an unsupported client callback",
            )
        })?;
        if self.provider_requests.handling_for(&namespace)
            != ProviderRequestHandling::ObserveAndStop
        {
            return Err(failure(
                "swallowtail.codex.app_server.callback_unsupported",
                "Codex app-server requested an undeclared provider callback",
            ));
        }
        self.verify_provider_request(params)?;
        let payload = serde_json::to_vec(params).map_err(|_| malformed_notification())?;
        let provider_request_ref =
            ProviderRequestRef::new(provider_request_value(provider_request_id))
                .map_err(|_| malformed_notification())?;
        let request = CallbackRequest::extension(
            callback_id.clone(),
            self.runtime_id.clone(),
            self.sequence.fetch_add(1, Ordering::SeqCst),
            self.deadline,
            ProviderExtension::new(namespace.clone(), payload),
            MAX_CALLBACK_ARGUMENT_BYTES,
        )
        .map_err(|_| {
            failure(
                "swallowtail.codex.app_server.callback_arguments_too_large",
                "Codex provider request exceeded the adapter limit",
            )
        })?
        .with_provider_request_ref(provider_request_ref.clone());
        let sequence = request.event_sequence();
        self.callbacks.observe_and_close(request)?;
        self.events.send(RuntimeEvent::new(
            sequence,
            RuntimeEventKind::CallbackRequested(callback_id.clone()),
        ))?;
        Ok(ProviderRequestObservation::new(
            callback_id,
            namespace,
            provider_request_ref,
        ))
    }

    pub(crate) fn handle_notification(
        &self,
        method: &str,
        params: &Value,
    ) -> Result<(), RuntimeFailure> {
        match method {
            "turn/started" => {
                self.verify_turn(params)?;
                self.emit(RuntimeEventKind::Progress, None)
            }
            "item/agentMessage/delta" => {
                self.verify_turn(params)?;
                let delta = required_text(params, "delta")?;
                self.delta_output
                    .lock()
                    .expect("turn delta lock poisoned")
                    .push_str(delta);
                match OperationContent::new(delta) {
                    Ok(content) => self.emit(RuntimeEventKind::OutputDelta, Some(content)),
                    Err(_) if delta.trim().is_empty() => Ok(()),
                    Err(_) => Err(malformed_notification()),
                }
            }
            "item/completed" => {
                self.verify_turn(params)?;
                let item = params.get("item").ok_or_else(malformed_notification)?;
                if item.get("type").and_then(Value::as_str) == Some("agentMessage") {
                    let text = required_text(item, "text")?;
                    match OperationContent::new(text) {
                        Ok(content) => {
                            *self.final_output.lock().expect("turn output lock poisoned") =
                                Some(content.clone());
                            self.emit(RuntimeEventKind::OutputAvailable, Some(content))
                        }
                        Err(_) if text.trim().is_empty() => {
                            self.emit(RuntimeEventKind::Progress, None)
                        }
                        Err(_) => Err(malformed_notification()),
                    }
                } else {
                    self.emit(RuntimeEventKind::Progress, None)
                }
            }
            "turn/completed" => {
                self.verify_turn(params)?;
                let turn = params.get("turn").ok_or_else(malformed_notification)?;
                let status = required_text(turn, "status")?;
                let terminal = match status {
                    "completed" => TerminalStatus::Completed,
                    "interrupted" if self.timed_out.load(Ordering::SeqCst) => {
                        TerminalStatus::TimedOut
                    }
                    "interrupted" => TerminalStatus::Cancelled,
                    "failed" => {
                        TerminalStatus::ProviderFailed(swallowtail_core::SafeDiagnostic::new(
                            "swallowtail.codex.app_server.turn_failed",
                            "Codex app-server turn failed",
                        ))
                    }
                    _ => TerminalStatus::RuntimeFailed(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.codex.app_server.unknown_turn_status",
                        "Codex app-server returned an unknown turn status",
                    )),
                };
                self.finish(terminal, CleanupOutcome::NotApplicable);
                Ok(())
            }
            "error" => {
                self.finish(
                    TerminalStatus::ProviderFailed(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.codex.app_server.provider_error",
                        "Codex app-server reported a provider error",
                    )),
                    CleanupOutcome::NotApplicable,
                );
                Ok(())
            }
            _ => self.emit(RuntimeEventKind::ProgressSnapshot, None),
        }
    }

    fn verify_turn(&self, params: &Value) -> Result<(), RuntimeFailure> {
        if params
            .get("threadId")
            .and_then(Value::as_str)
            .is_some_and(|thread_id| thread_id != self.provider_thread_id)
        {
            return Err(failure(
                "swallowtail.codex.app_server.session_id_mismatch",
                "Codex app-server event belongs to a different provider session",
            ));
        }
        if let Some(turn_id) = params
            .get("turnId")
            .and_then(Value::as_str)
            .or_else(|| params.get("turn")?.get("id")?.as_str())
        {
            self.set_provider_id(turn_id)
        } else {
            Ok(())
        }
    }

    fn verify_provider_request(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let thread_id = required_text(params, "threadId")?;
        if thread_id != self.provider_thread_id {
            return Err(failure(
                "swallowtail.codex.app_server.session_id_mismatch",
                "Codex provider request belongs to a different session",
            ));
        }
        let turn_id = required_text(params, "turnId")?;
        self.set_provider_id(turn_id)
    }

    fn emit(
        &self,
        kind: RuntimeEventKind,
        content: Option<OperationContent>,
    ) -> Result<(), RuntimeFailure> {
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        let event = match content {
            Some(content) => RuntimeEvent::with_content(sequence, kind, content),
            None => RuntimeEvent::new(sequence, kind),
        };
        self.events.send(event)
    }

    pub(crate) fn finish(&self, status: TerminalStatus, cleanup: CleanupOutcome) {
        if self.finished.swap(true, Ordering::SeqCst) {
            return;
        }
        let abandonment = match &status {
            TerminalStatus::Cancelled => CallbackAbandonment::TurnCancelled,
            TerminalStatus::TimedOut => CallbackAbandonment::TimedOut,
            TerminalStatus::ProviderRequestObserved(_) => CallbackAbandonment::TurnTerminated,
            _ => CallbackAbandonment::TurnTerminated,
        };
        self.callbacks.abandon(abandonment);
        let output = self
            .final_output
            .lock()
            .expect("turn output lock poisoned")
            .clone()
            .or_else(|| {
                let delta = self
                    .delta_output
                    .lock()
                    .expect("turn delta lock poisoned")
                    .clone();
                OperationContent::new(delta).ok()
            });
        let outcome = TerminalOutcome::new(status, cleanup);
        let outcome = match output {
            Some(output) => outcome.with_output(output),
            None => outcome,
        };
        let _ = self.terminal.complete(outcome);
        self.events.mark_terminal();
        let mut signal = self
            .finish_signal
            .lock()
            .expect("turn-finished lock poisoned");
        signal.finished = true;
        if let Some(waiter) = signal.waiter.take() {
            waiter.wake();
        }
    }
}

fn provider_request_value(value: &Value) -> String {
    value
        .as_str()
        .map(str::to_owned)
        .unwrap_or_else(|| value.to_string())
}

fn required_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(malformed_notification)
}

pub(crate) fn malformed_notification() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.malformed_notification",
        "Codex app-server returned a malformed notification",
    )
}
