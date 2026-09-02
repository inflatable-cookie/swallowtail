//! One active Claude Agent SDK sidecar turn: streamed events, bounded
//! output, correlated tool admission, and one terminal outcome.

use super::activity::SdkActivityProjection;
use super::failure::failure;
use super::permission::AdmissionHub;
use super::wire::ClaudeAgentSdkEvent;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use swallowtail_core::{
    FailureClassification, FailureKind, FailureOrigin, FailureRecovery, SafeDiagnostic,
};
use swallowtail_runtime::{
    ActivityStatus, BoxEventStream, CallbackAbandonment, CallbackExchange, CleanupOutcome,
    Deadline, OperationContent, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeTurnId,
    TerminalOutcome, TerminalOutcomeFuture, TerminalOutcomeSender, TerminalStatus,
    runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 256;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

pub(crate) struct SdkActiveTurn {
    runtime_id: RuntimeTurnId,
    events: swallowtail_runtime::RuntimeEventSender,
    terminal: TerminalOutcomeSender,
    sequence: AtomicU64,
    activity: Mutex<SdkActivityProjection>,
    output: Mutex<String>,
    admission: AdmissionHub,
    deadline: Option<Deadline>,
    cancelled: AtomicBool,
    finished: AtomicBool,
}

impl SdkActiveTurn {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        connection: Weak<super::connection::SdkConnection>,
        deadline: Option<Deadline>,
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
        let (admission, exchange) = AdmissionHub::new(connection);
        Ok((
            Arc::new(Self {
                activity: Mutex::new(SdkActivityProjection::new(runtime_id.clone())),
                runtime_id,
                events,
                terminal,
                sequence: AtomicU64::new(1),
                output: Mutex::new(String::new()),
                admission,
                deadline,
                cancelled: AtomicBool::new(false),
                finished: AtomicBool::new(false),
            }),
            Box::pin(stream),
            exchange,
            future,
        ))
    }

    pub(crate) const fn runtime_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }

    pub(crate) fn mark_cancelled(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub(crate) fn abandon_admissions(&self, reason: CallbackAbandonment) {
        self.admission.abandon(reason);
    }

    pub(crate) fn handle_admission(
        &self,
        sidecar_id: &str,
        tool_name: &str,
    ) -> Result<(), RuntimeFailure> {
        if self.is_finished() {
            return Err(failure(
                "swallowtail.claude-agent.sdk.admission_after_terminal",
                "Claude Agent SDK sidecar requested admission after the turn terminated",
            ));
        }
        let sequence = self.next_sequence();
        let callback_id = self.admission.enqueue(
            &self.runtime_id,
            sequence,
            self.deadline,
            sidecar_id,
            tool_name,
        )?;
        self.events.send(RuntimeEvent::new(
            sequence,
            RuntimeEventKind::CallbackRequested(callback_id),
        ))
    }

    pub(crate) fn handle_event(&self, event: ClaudeAgentSdkEvent) -> Result<(), RuntimeFailure> {
        if self.is_finished() {
            return Err(failure(
                "swallowtail.claude-agent.sdk.event_after_terminal",
                "Claude Agent SDK sidecar emitted an event after the active turn terminated",
            ));
        }
        self.project_activity(&event)?;
        match event {
            ClaudeAgentSdkEvent::TurnStarted | ClaudeAgentSdkEvent::Progress => self.progress(),
            ClaudeAgentSdkEvent::OutputDelta(delta) => self.output_delta(delta),
            ClaudeAgentSdkEvent::ToolStarted { .. } | ClaudeAgentSdkEvent::ToolEnded { .. } => {
                Ok(())
            }
            ClaudeAgentSdkEvent::TurnFailed => {
                self.complete_activity(ActivityStatus::Failed)?;
                self.finish(TerminalStatus::ProviderFailed(provider_diagnostic()));
                Ok(())
            }
            ClaudeAgentSdkEvent::TurnEnded {
                stop_reason,
                failed,
            } => {
                let (status, activity_status) = if failed || stop_reason != "success" {
                    (
                        TerminalStatus::ProviderFailed(provider_diagnostic()),
                        ActivityStatus::Failed,
                    )
                } else if self.cancelled.load(Ordering::SeqCst) {
                    (TerminalStatus::Cancelled, ActivityStatus::Cancelled)
                } else {
                    (TerminalStatus::Completed, ActivityStatus::Completed)
                };
                self.complete_activity(activity_status)?;
                self.finish(status);
                Ok(())
            }
        }
    }

    pub(crate) fn fail_connection(&self, diagnostic: SafeDiagnostic) {
        let status = if self.cancelled.load(Ordering::SeqCst) {
            TerminalStatus::Cancelled
        } else {
            TerminalStatus::RuntimeFailed(diagnostic)
        };
        self.finish(status);
    }

    fn output_delta(&self, delta: String) -> Result<(), RuntimeFailure> {
        {
            let mut output = self
                .output
                .lock()
                .expect("SDK sidecar output lock poisoned");
            if output.len().saturating_add(delta.len()) > MAXIMUM_OUTPUT_BYTES {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.output_limit_exceeded",
                    "Claude Agent SDK sidecar output exceeded the adapter limit",
                ));
            }
            output.push_str(&delta);
        }
        let content = OperationContent::new(delta).map_err(|_| {
            failure(
                "swallowtail.claude-agent.sdk.event_content_invalid",
                "Claude Agent SDK sidecar event content was invalid",
            )
        })?;
        let sequence = self.next_sequence();
        self.events.send(RuntimeEvent::with_content(
            sequence,
            RuntimeEventKind::OutputDelta,
            content,
        ))
    }

    fn project_activity(&self, event: &ClaudeAgentSdkEvent) -> Result<(), RuntimeFailure> {
        let observations = self
            .activity
            .lock()
            .expect("SDK sidecar activity lock poisoned")
            .project(event)?;
        self.emit_activity(observations)
    }

    fn complete_activity(&self, status: ActivityStatus) -> Result<(), RuntimeFailure> {
        let observations = self
            .activity
            .lock()
            .expect("SDK sidecar activity lock poisoned")
            .complete(status)?;
        self.emit_activity(observations)
    }

    fn emit_activity(
        &self,
        observations: Vec<swallowtail_runtime::ActivityObservation>,
    ) -> Result<(), RuntimeFailure> {
        for observation in observations {
            let sequence = self.next_sequence();
            self.events.send(RuntimeEvent::new(
                sequence,
                RuntimeEventKind::Activity(observation),
            ))?;
        }
        Ok(())
    }

    fn progress(&self) -> Result<(), RuntimeFailure> {
        let sequence = self.next_sequence();
        self.events
            .send(RuntimeEvent::new(sequence, RuntimeEventKind::Progress))
    }

    fn next_sequence(&self) -> u64 {
        self.sequence.fetch_add(1, Ordering::SeqCst)
    }

    fn finish(&self, status: TerminalStatus) {
        if self.finished.swap(true, Ordering::SeqCst) {
            return;
        }
        self.admission.abandon(CallbackAbandonment::TurnTerminated);
        let output = self
            .output
            .lock()
            .expect("SDK sidecar output lock poisoned")
            .clone();
        let mut outcome = TerminalOutcome::new(status, CleanupOutcome::NotApplicable);
        if let Ok(content) = OperationContent::new(output)
            && content.byte_len() != 0
        {
            let sequence = self.next_sequence();
            let _ = self.events.send(RuntimeEvent::with_content(
                sequence,
                RuntimeEventKind::OutputAvailable,
                content.clone(),
            ));
            outcome = outcome.with_output(content);
        }
        self.events.mark_terminal();
        let _ = self.terminal.complete(outcome);
    }
}

fn provider_diagnostic() -> SafeDiagnostic {
    SafeDiagnostic::new(
        "swallowtail.claude-agent.sdk.provider_failed",
        "Claude Agent SDK sidecar reported a downstream provider failure",
    )
    .with_failure_classification(FailureClassification::new(
        FailureOrigin::Provider,
        FailureKind::Unknown,
        FailureRecovery::Unknown,
    ))
}
