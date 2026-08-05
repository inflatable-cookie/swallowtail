use crate::activity::OhMyPiActivityProjection;
use crate::callback::CallbackHub;
use crate::connection::OhMyPiConnection;
use crate::failure::failure;
use crate::protocol::OhMyPiAgentEvent;
use std::collections::BTreeSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use swallowtail_runtime::{
    ActivityObservation, ActivityStatus, BoxEventStream, CallbackAbandonment, CallbackExchange,
    CleanupOutcome, OperationContent, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
    RuntimeTurnId, TerminalOutcome, TerminalOutcomeFuture, TerminalOutcomeSender, TerminalStatus,
    TokenUsage, runtime_event_channel, terminal_outcome_channel,
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
        let mut state = self.0.lock().expect("OhMyPi turn-finished lock poisoned");
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
    activity: Mutex<OhMyPiActivityProjection>,
    output: Mutex<String>,
    usage: Mutex<Option<TokenUsage>>,
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
        connection: Weak<OhMyPiConnection>,
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
        let activity = Mutex::new(OhMyPiActivityProjection::new(runtime_id.clone()));
        Ok((
            Arc::new(Self {
                runtime_id,
                events,
                terminal,
                callbacks,
                sequence: AtomicU64::new(1),
                activity,
                output: Mutex::new(String::new()),
                usage: Mutex::new(None),
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

    pub(crate) fn handle_event(&self, event: OhMyPiAgentEvent) -> Result<(), RuntimeFailure> {
        if self.is_finished() {
            return Err(failure(
                "swallowtail.oh_my_pi.rpc.event_after_terminal",
                "OhMyPi RPC emitted an event after the active turn terminated",
            ));
        }
        self.project_activity(&event)?;
        match event {
            OhMyPiAgentEvent::Started | OhMyPiAgentEvent::Progress => self.progress(),
            OhMyPiAgentEvent::OutputDelta(delta) => self.output_delta(delta),
            OhMyPiAgentEvent::ReasoningDelta(delta) => {
                self.content_event(RuntimeEventKind::ReasoningProgress, delta)
            }
            OhMyPiAgentEvent::MessageEnded(Some(usage)) => self.add_usage(usage),
            OhMyPiAgentEvent::MessageEnded(None)
            | OhMyPiAgentEvent::MessageStarted
            | OhMyPiAgentEvent::ReasoningStarted
            | OhMyPiAgentEvent::ReasoningEnded
            | OhMyPiAgentEvent::ToolStarted { .. }
            | OhMyPiAgentEvent::ToolUpdated { .. }
            | OhMyPiAgentEvent::ToolEnded { .. }
            | OhMyPiAgentEvent::CompactionStarted
            | OhMyPiAgentEvent::CompactionEnded
            | OhMyPiAgentEvent::Unknown(_) => Ok(()),
            OhMyPiAgentEvent::ProviderFailed => {
                self.complete_activity(ActivityStatus::Failed)?;
                self.finish(TerminalStatus::ProviderFailed(
                    swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.oh_my_pi.rpc.provider_failed",
                        "OhMyPi RPC reported a downstream provider failure",
                    ),
                ));
                Ok(())
            }
            OhMyPiAgentEvent::RetryObserved => {
                self.complete_activity(ActivityStatus::Failed)?;
                self.finish(TerminalStatus::RuntimeFailed(
                    swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.oh_my_pi.rpc.retry_policy_drift",
                        "OhMyPi RPC retried despite the disabled retry policy",
                    ),
                ));
                Ok(())
            }
            OhMyPiAgentEvent::Settled => {
                let (status, activity_status) = if self.timed_out.load(Ordering::SeqCst) {
                    (TerminalStatus::TimedOut, ActivityStatus::Failed)
                } else if self.cancelled.load(Ordering::SeqCst) {
                    (TerminalStatus::Cancelled, ActivityStatus::Cancelled)
                } else {
                    self.completed_prompts.fetch_add(1, Ordering::SeqCst);
                    (TerminalStatus::Completed, ActivityStatus::Completed)
                };
                self.complete_activity(activity_status)?;
                let usage = *self.usage.lock().expect("OhMyPi usage lock poisoned");
                if matches!(status, TerminalStatus::Completed) && usage.is_none() {
                    self.finish(TerminalStatus::RuntimeFailed(
                        swallowtail_core::SafeDiagnostic::new(
                            "swallowtail.oh_my_pi.rpc.usage_missing",
                            "OhMyPi RPC completed without required token usage",
                        ),
                    ));
                } else {
                    self.finish_with_usage(status, usage);
                }
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
            let mut output = self.output.lock().expect("OhMyPi output lock poisoned");
            if output.len().saturating_add(delta.len()) > MAXIMUM_OUTPUT_BYTES {
                return Err(failure(
                    "swallowtail.oh_my_pi.rpc.output_limit_exceeded",
                    "OhMyPi RPC output exceeded the adapter limit",
                ));
            }
            output.push_str(&delta);
        }
        self.content_event(RuntimeEventKind::OutputDelta, delta)
    }

    fn project_activity(&self, event: &OhMyPiAgentEvent) -> Result<(), RuntimeFailure> {
        let observations = self
            .activity
            .lock()
            .expect("OhMyPi activity lock poisoned")
            .project(event)?;
        self.emit_activity(observations)
    }

    fn complete_activity(&self, status: ActivityStatus) -> Result<(), RuntimeFailure> {
        let observations = self
            .activity
            .lock()
            .expect("OhMyPi activity lock poisoned")
            .complete(status)?;
        self.emit_activity(observations)
    }

    fn emit_activity(
        &self,
        observations: impl IntoIterator<Item = ActivityObservation>,
    ) -> Result<(), RuntimeFailure> {
        for observation in observations {
            self.events.send(RuntimeEvent::new(
                self.next_sequence(),
                RuntimeEventKind::Activity(observation),
            ))?;
        }
        Ok(())
    }

    fn progress(&self) -> Result<(), RuntimeFailure> {
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        self.events
            .send(RuntimeEvent::new(sequence, RuntimeEventKind::Progress))
    }

    fn add_usage(&self, usage: TokenUsage) -> Result<(), RuntimeFailure> {
        let mut aggregate = self.usage.lock().expect("OhMyPi usage lock poisoned");
        *aggregate = Some(match *aggregate {
            Some(current) => current.checked_add_disjoint(usage).ok_or_else(|| {
                failure(
                    "swallowtail.oh_my_pi.rpc.usage_overflow",
                    "OhMyPi RPC token usage exceeded the supported range",
                )
            })?,
            None => usage,
        });
        Ok(())
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
        self.finish_with_usage(status, None);
    }

    fn finish_with_usage(&self, status: TerminalStatus, usage: Option<TokenUsage>) {
        if self.finished.swap(true, Ordering::SeqCst) {
            return;
        }
        self.callbacks.abandon(CallbackAbandonment::TurnTerminated);
        let output = self
            .output
            .lock()
            .expect("OhMyPi output lock poisoned")
            .clone();
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
        if let Some(usage) = usage {
            let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
            let _ = self.events.send(RuntimeEvent::new(
                sequence,
                RuntimeEventKind::ProviderObservation(
                    swallowtail_runtime::ProviderObservation::Usage(usage),
                ),
            ));
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
            .expect("OhMyPi turn-finished lock poisoned");
        signal.finished = true;
        if let Some(waiter) = signal.waiter.take() {
            waiter.wake();
        }
    }
}

fn malformed_ui_sequence() -> RuntimeFailure {
    failure(
        "swallowtail.oh_my_pi.rpc.ui_request_invalid",
        "OhMyPi RPC extension UI request was invalid",
    )
}
