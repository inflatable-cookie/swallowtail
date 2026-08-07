use crate::app_server_activity::AppServerActivityProjection;
use crate::callback_exchange::CallbackHub;
use crate::rpc::RpcConnection;
use crate::rpc::failure;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use swallowtail_core::ProviderRequestRef;
use swallowtail_runtime::{
    BoxEventStream, CallbackAbandonment, CallbackExchange, CleanupOutcome, Deadline,
    OperationContent, ProviderRequestObservation, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
    RuntimeTurnId, TerminalOutcome, TerminalOutcomeFuture, TerminalOutcomeSender, TerminalStatus,
    runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 256;
const MAX_CALLBACK_ARGUMENT_BYTES: usize = 1_048_576;
const MAX_ADMITTED_CHILD_THREADS: usize = 256;

pub(crate) enum ProviderRequestDisposition {
    Exchange,
    Observed(ProviderRequestObservation),
}

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
    activity: Mutex<AppServerActivityProjection>,
    admitted_child_threads: Mutex<BTreeSet<String>>,
    active_child_turns: Mutex<BTreeMap<String, String>>,
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
                activity: Mutex::new(AppServerActivityProjection::new(
                    runtime_id.clone(),
                    provider_thread_id.clone(),
                )),
                runtime_id,
                provider_thread_id,
                admitted_child_threads: Mutex::new(BTreeSet::new()),
                active_child_turns: Mutex::new(BTreeMap::new()),
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
}


mod notifications;
mod provider_requests;
#[cfg(test)]
mod tests;

impl ActiveTurn {
    fn verify_turn(&self, params: &Value) -> Result<(), RuntimeFailure> {
        if required_text(params, "threadId")? != self.provider_thread_id {
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
        self.admitted_child_threads
            .lock()
            .expect("admitted child threads lock poisoned")
            .clear();
        self.active_child_turns
            .lock()
            .expect("active child turns lock poisoned")
            .clear();
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

pub(crate) fn canonical_provider_request_id(
    value: &Value,
) -> Result<ProviderRequestRef, RuntimeFailure> {
    match value {
        Value::String(value) => {
            ProviderRequestRef::new(value.clone()).map_err(|_| malformed_notification())
        }
        Value::Number(value) => {
            let value = value.as_i64().ok_or_else(malformed_notification)?;
            Ok(ProviderRequestRef::from_signed_integer(value))
        }
        _ => Err(malformed_notification()),
    }
}

pub(super) fn activity_notification(method: &str) -> bool {
    method.starts_with("item/")
        || matches!(
            method,
            "turn/plan/updated"
                | "turn/diff/updated"
                | "thread/compacted"
                | "hook/started"
                | "hook/completed"
                | "serverRequest/resolved"
        )
}

pub(super) fn required_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
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

pub(super) fn child_turn_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.child_turn_id_mismatch",
        "Codex app-server child activity does not match its active child turn",
    )
}

pub(super) fn child_lifecycle_after_terminal() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.child_lifecycle_after_terminal",
        "Codex app-server emitted child lifecycle after operation termination",
    )
}
