use crate::callback::CallbackHub;
use crate::connection::PiConnection;
use crate::failure::failure;
use crate::protocol::PiAgentEvent;
use std::collections::BTreeSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use swallowtail_runtime::{
    BoxEventStream, CallbackAbandonment, CallbackExchange, CleanupOutcome, OperationContent,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeTurnId, TerminalOutcome,
    TerminalOutcomeFuture, TerminalOutcomeSender, TerminalStatus, runtime_event_channel,
    terminal_outcome_channel,
};

mod scheduling;
mod ui;

pub(crate) use ui::CallbackTimer;

const EVENT_CAPACITY: usize = 256;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_DIALOG_BYTES: usize = 16 * 1024;
const MAXIMUM_DIALOG_OPTIONS: usize = 32;

#[derive(Default)]
struct FinishedState {
    finished: bool,
    waiter: Option<Waker>,
}

pub(crate) struct TurnFinishedFuture(Arc<Mutex<FinishedState>>);

impl Future for TurnFinishedFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().expect("Pi turn-finished lock poisoned");
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
    events: swallowtail_runtime::RuntimeEventSender,
    terminal: TerminalOutcomeSender,
    callbacks: CallbackHub,
    sequence: AtomicU64,
    output: Mutex<String>,
    ui_ids: Mutex<BTreeSet<String>>,
    steering_scheduled: AtomicBool,
    follow_up_scheduled: AtomicBool,
    cancelled: AtomicBool,
    timed_out: AtomicBool,
    finished: AtomicBool,
    completed_prompts: Arc<AtomicU32>,
    finish_signal: Arc<Mutex<FinishedState>>,
}

impl ActiveTurn {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        completed_prompts: Arc<AtomicU32>,
        connection: Weak<PiConnection>,
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
                events,
                terminal,
                callbacks,
                sequence: AtomicU64::new(1),
                output: Mutex::new(String::new()),
                ui_ids: Mutex::new(BTreeSet::new()),
                steering_scheduled: AtomicBool::new(false),
                follow_up_scheduled: AtomicBool::new(false),
                cancelled: AtomicBool::new(false),
                timed_out: AtomicBool::new(false),
                finished: AtomicBool::new(false),
                completed_prompts,
                finish_signal: Arc::new(Mutex::new(FinishedState::default())),
            }),
            Box::pin(stream),
            exchange,
            future,
        ))
    }

    pub(crate) const fn runtime_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    pub(crate) fn finished_future(&self) -> TurnFinishedFuture {
        TurnFinishedFuture(Arc::clone(&self.finish_signal))
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }

    pub(crate) fn handle_event(&self, event: PiAgentEvent) -> Result<(), RuntimeFailure> {
        if self.is_finished() {
            return Err(failure(
                "swallowtail.pi.rpc.event_after_terminal",
                "Pi RPC emitted an event after the active turn terminated",
            ));
        }
        match event {
            PiAgentEvent::Started | PiAgentEvent::Progress => self.progress(),
            PiAgentEvent::OutputDelta(delta) => self.output_delta(delta),
            PiAgentEvent::ReasoningDelta(delta) => {
                self.content_event(RuntimeEventKind::ReasoningProgress, delta)
            }
            PiAgentEvent::ProviderFailed => {
                self.finish(TerminalStatus::ProviderFailed(
                    swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.pi.rpc.provider_failed",
                        "Pi RPC reported a downstream provider failure",
                    ),
                ));
                Ok(())
            }
            PiAgentEvent::RetryObserved => {
                self.finish(TerminalStatus::RuntimeFailed(
                    swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.pi.rpc.retry_policy_drift",
                        "Pi RPC retried despite the disabled retry policy",
                    ),
                ));
                Ok(())
            }
            PiAgentEvent::Settled => {
                let status = if self.timed_out.load(Ordering::SeqCst) {
                    TerminalStatus::TimedOut
                } else if self.cancelled.load(Ordering::SeqCst) {
                    TerminalStatus::Cancelled
                } else {
                    self.completed_prompts.fetch_add(1, Ordering::SeqCst);
                    TerminalStatus::Completed
                };
                self.finish(status);
                Ok(())
            }
        }
    }

    pub(crate) fn mark_cancelled(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
        self.callbacks.abandon(CallbackAbandonment::TurnCancelled);
    }

    pub(crate) fn mark_timed_out(&self) {
        self.timed_out.store(true, Ordering::SeqCst);
        self.callbacks.abandon(CallbackAbandonment::TimedOut);
    }

    pub(crate) fn fail_connection(&self, diagnostic: swallowtail_core::SafeDiagnostic) {
        let status = if self.timed_out.load(Ordering::SeqCst) {
            TerminalStatus::TimedOut
        } else if self.cancelled.load(Ordering::SeqCst) {
            TerminalStatus::Cancelled
        } else {
            TerminalStatus::RuntimeFailed(diagnostic)
        };
        self.finish(status);
    }

    fn output_delta(&self, delta: String) -> Result<(), RuntimeFailure> {
        {
            let mut output = self.output.lock().expect("Pi output lock poisoned");
            if output.len().saturating_add(delta.len()) > MAXIMUM_OUTPUT_BYTES {
                return Err(failure(
                    "swallowtail.pi.rpc.output_limit_exceeded",
                    "Pi RPC output exceeded the adapter limit",
                ));
            }
            output.push_str(&delta);
        }
        self.content_event(RuntimeEventKind::OutputDelta, delta)
    }

    fn progress(&self) -> Result<(), RuntimeFailure> {
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        self.events
            .send(RuntimeEvent::new(sequence, RuntimeEventKind::Progress))
    }

    fn content_event(&self, kind: RuntimeEventKind, value: String) -> Result<(), RuntimeFailure> {
        let content = OperationContent::new(value).map_err(|_| malformed_ui_sequence())?;
        let sequence = self.next_sequence();
        self.events
            .send(RuntimeEvent::with_content(sequence, kind, content))
    }

    fn next_sequence(&self) -> u64 {
        self.sequence.fetch_add(1, Ordering::SeqCst)
    }

    fn finish(&self, status: TerminalStatus) {
        if self.finished.swap(true, Ordering::SeqCst) {
            return;
        }
        self.callbacks.abandon(CallbackAbandonment::TurnTerminated);
        let output = self.output.lock().expect("Pi output lock poisoned").clone();
        if !output.is_empty() {
            let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
            if let Ok(content) = OperationContent::new(output.clone()) {
                let _ = self.events.send(RuntimeEvent::with_content(
                    sequence,
                    RuntimeEventKind::OutputAvailable,
                    content,
                ));
            }
        }
        self.events.mark_terminal();
        let mut outcome = TerminalOutcome::new(status, CleanupOutcome::NotApplicable);
        if let Ok(content) = OperationContent::new(output)
            && content.byte_len() != 0
        {
            outcome = outcome.with_output(content);
        }
        let _ = self.terminal.complete(outcome);
        let mut signal = self
            .finish_signal
            .lock()
            .expect("Pi turn-finished lock poisoned");
        signal.finished = true;
        if let Some(waiter) = signal.waiter.take() {
            waiter.wake();
        }
    }
}

fn malformed_ui_sequence() -> RuntimeFailure {
    failure(
        "swallowtail.pi.rpc.ui_request_invalid",
        "Pi RPC extension UI request was invalid",
    )
}
