use super::activity::SidecarActivityProjection;
use super::failure::failure;
use super::wire::PiSdkSidecarEvent;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{
    FailureClassification, FailureKind, FailureOrigin, FailureRecovery, SafeDiagnostic,
};
use swallowtail_runtime::{
    ActivityStatus, BoxEventStream, CleanupOutcome, OperationContent, RuntimeEvent,
    RuntimeEventKind, RuntimeFailure, RuntimeTurnId, TerminalOutcome, TerminalOutcomeFuture,
    TerminalOutcomeSender, TerminalStatus, TokenUsage, runtime_event_channel,
    terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 256;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

#[derive(Default)]
struct FinishedState {
    finished: bool,
    waiter: Option<Waker>,
}

pub(crate) struct TurnFinishedFuture(Arc<Mutex<FinishedState>>);

impl Future for TurnFinishedFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().expect("sidecar turn-finished lock poisoned");
        if state.finished {
            Poll::Ready(())
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

pub(crate) struct SidecarActiveTurn {
    runtime_id: RuntimeTurnId,
    events: swallowtail_runtime::RuntimeEventSender,
    terminal: TerminalOutcomeSender,
    sequence: AtomicU64,
    activity: Mutex<SidecarActivityProjection>,
    output: Mutex<String>,
    usage: Mutex<Option<TokenUsage>>,
    steering_scheduled: AtomicBool,
    follow_up_scheduled: AtomicBool,
    cancelled: AtomicBool,
    timed_out: AtomicBool,
    finished: AtomicBool,
    completed_prompts: Arc<AtomicU32>,
    finish_signal: Arc<Mutex<FinishedState>>,
}

impl SidecarActiveTurn {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        completed_prompts: Arc<AtomicU32>,
    ) -> Result<(Arc<Self>, BoxEventStream, TerminalOutcomeFuture), RuntimeFailure> {
        let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let (terminal, future) = terminal_outcome_channel();
        let activity = Mutex::new(SidecarActivityProjection::new(runtime_id.clone()));
        Ok((
            Arc::new(Self {
                runtime_id,
                events,
                terminal,
                sequence: AtomicU64::new(1),
                activity,
                output: Mutex::new(String::new()),
                usage: Mutex::new(None),
                steering_scheduled: AtomicBool::new(false),
                follow_up_scheduled: AtomicBool::new(false),
                cancelled: AtomicBool::new(false),
                timed_out: AtomicBool::new(false),
                finished: AtomicBool::new(false),
                completed_prompts,
                finish_signal: Arc::new(Mutex::new(FinishedState::default())),
            }),
            Box::pin(stream),
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

    pub(crate) fn handle_event(&self, event: PiSdkSidecarEvent) -> Result<(), RuntimeFailure> {
        if self.is_finished() {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.event_after_terminal",
                "Pi SDK sidecar emitted an event after the active turn terminated",
            ));
        }
        if matches!(event, PiSdkSidecarEvent::ReplayItem { .. }) {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.replay_unexpected",
                "Pi SDK sidecar emitted replay evidence inside a fresh session",
            ));
        }
        self.project_activity(&event)?;
        match event {
            PiSdkSidecarEvent::Started | PiSdkSidecarEvent::Progress => self.progress(),
            PiSdkSidecarEvent::OutputDelta(delta) => self.output_delta(delta),
            PiSdkSidecarEvent::ReasoningDelta(delta) => {
                self.content_event(RuntimeEventKind::ReasoningProgress, delta)
            }
            PiSdkSidecarEvent::MessageEnded { stop_reason, usage } => {
                if stop_reason == "error" {
                    self.complete_activity(ActivityStatus::Failed)?;
                    self.finish(TerminalStatus::ProviderFailed(
                        SafeDiagnostic::new(
                            "swallowtail.pi.sdk-sidecar.provider_failed",
                            "Pi SDK sidecar reported a downstream provider failure",
                        )
                        .with_failure_classification(
                            FailureClassification::new(
                                FailureOrigin::Provider,
                                FailureKind::Unknown,
                                FailureRecovery::Unknown,
                            ),
                        ),
                    ));
                    Ok(())
                } else if let Some(usage) = usage {
                    self.add_usage(usage)
                } else {
                    Ok(())
                }
            }
            PiSdkSidecarEvent::TurnStarted
            | PiSdkSidecarEvent::TurnEnded
            | PiSdkSidecarEvent::Ended
            | PiSdkSidecarEvent::MessageStarted
            | PiSdkSidecarEvent::ReasoningStarted
            | PiSdkSidecarEvent::ReasoningEnded
            | PiSdkSidecarEvent::ToolStarted { .. }
            | PiSdkSidecarEvent::ToolUpdated { .. }
            | PiSdkSidecarEvent::ToolEnded { .. } => Ok(()),
            PiSdkSidecarEvent::Settled => {
                let (status, activity_status) = if self.timed_out.load(Ordering::SeqCst) {
                    (TerminalStatus::TimedOut, ActivityStatus::Failed)
                } else if self.cancelled.load(Ordering::SeqCst) {
                    (TerminalStatus::Cancelled, ActivityStatus::Cancelled)
                } else {
                    self.completed_prompts.fetch_add(1, Ordering::SeqCst);
                    (TerminalStatus::Completed, ActivityStatus::Completed)
                };
                self.complete_activity(activity_status)?;
                let usage = *self.usage.lock().expect("sidecar usage lock poisoned");
                if matches!(status, TerminalStatus::Completed) && usage.is_none() {
                    self.finish(TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                        "swallowtail.pi.sdk-sidecar.usage_missing",
                        "Pi SDK sidecar completed without required token usage",
                    )));
                } else {
                    self.finish_with_usage(status, usage);
                }
                Ok(())
            }
            PiSdkSidecarEvent::ReplayItem { .. } => unreachable!("replay events fail closed above"),
        }
    }

    pub(crate) fn mark_cancelled(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub(crate) fn mark_timed_out(&self) {
        self.timed_out.store(true, Ordering::SeqCst);
    }

    pub(crate) fn fail_connection(&self, diagnostic: SafeDiagnostic) {
        let status = if self.timed_out.load(Ordering::SeqCst) {
            TerminalStatus::TimedOut
        } else if self.cancelled.load(Ordering::SeqCst) {
            TerminalStatus::Cancelled
        } else {
            TerminalStatus::RuntimeFailed(diagnostic)
        };
        self.finish(status);
    }

    pub(crate) fn reserve_scheduling(&self, class: swallowtail_core::HarnessMessageClass) -> bool {
        match class {
            swallowtail_core::HarnessMessageClass::Steering => {
                !self.steering_scheduled.swap(true, Ordering::SeqCst)
            }
            swallowtail_core::HarnessMessageClass::FollowUp => {
                !self.follow_up_scheduled.swap(true, Ordering::SeqCst)
            }
            swallowtail_core::HarnessMessageClass::Prompt => false,
        }
    }

    pub(crate) fn release_scheduling(&self, class: swallowtail_core::HarnessMessageClass) {
        match class {
            swallowtail_core::HarnessMessageClass::Steering => {
                self.steering_scheduled.store(false, Ordering::SeqCst);
            }
            swallowtail_core::HarnessMessageClass::FollowUp => {
                self.follow_up_scheduled.store(false, Ordering::SeqCst);
            }
            swallowtail_core::HarnessMessageClass::Prompt => {}
        }
    }

    fn output_delta(&self, delta: String) -> Result<(), RuntimeFailure> {
        {
            let mut output = self.output.lock().expect("sidecar output lock poisoned");
            if output.len().saturating_add(delta.len()) > MAXIMUM_OUTPUT_BYTES {
                return Err(failure(
                    "swallowtail.pi.sdk-sidecar.output_limit_exceeded",
                    "Pi SDK sidecar output exceeded the adapter limit",
                ));
            }
            output.push_str(&delta);
        }
        self.content_event(RuntimeEventKind::OutputDelta, delta)
    }

    fn project_activity(&self, event: &PiSdkSidecarEvent) -> Result<(), RuntimeFailure> {
        let observations = self
            .activity
            .lock()
            .expect("sidecar activity lock poisoned")
            .project(event)?;
        for observation in observations {
            self.events.send(RuntimeEvent::new(
                self.next_sequence(),
                RuntimeEventKind::Activity(observation),
            ))?;
        }
        Ok(())
    }

    fn complete_activity(&self, status: ActivityStatus) -> Result<(), RuntimeFailure> {
        let observations = self
            .activity
            .lock()
            .expect("sidecar activity lock poisoned")
            .complete(status)?;
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
        let mut aggregate = self.usage.lock().expect("sidecar usage lock poisoned");
        *aggregate = Some(match *aggregate {
            Some(current) => current.checked_add_disjoint(usage).ok_or_else(|| {
                failure(
                    "swallowtail.pi.sdk-sidecar.usage_overflow",
                    "Pi SDK sidecar token usage exceeded the supported range",
                )
            })?,
            None => usage,
        });
        Ok(())
    }

    fn content_event(&self, kind: RuntimeEventKind, value: String) -> Result<(), RuntimeFailure> {
        let content = OperationContent::new(value).map_err(|_| {
            failure(
                "swallowtail.pi.sdk-sidecar.event_content_invalid",
                "Pi SDK sidecar event content was invalid",
            )
        })?;
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
        let output = self
            .output
            .lock()
            .expect("sidecar output lock poisoned")
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
            .expect("sidecar turn-finished lock poisoned");
        signal.finished = true;
        if let Some(waiter) = signal.waiter.take() {
            waiter.wake();
        }
    }
}
