//! One active Claude Agent SDK sidecar turn: streamed events, bounded
//! output, correlated tool admission, and one terminal outcome.

mod events;
mod finished;

use self::finished::FinishedState;
use super::activity::SdkActivityProjection;
use super::failure::failure;
use super::permission::AdmissionHub;
use super::wire::{ClaudeAgentSdkBashCommandView, ClaudeAgentSdkEvent};
use std::collections::BTreeMap;
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

/// What one admission request from the sidecar turned into.
pub(crate) enum AdmissionDisposition {
    /// Delegated to the consumer as a bounded correlated callback.
    Delegated,
    /// The turn ended before the request was read. The sidecar had already
    /// written it, so this is a race rather than a protocol violation, and it
    /// is denied on the wire.
    RacedTurnEnd,
}

pub(crate) struct SdkActiveTurn {
    connection: Weak<super::connection::SdkConnection>,
    runtime_id: RuntimeTurnId,
    events: swallowtail_runtime::RuntimeEventSender,
    terminal: TerminalOutcomeSender,
    sequence: AtomicU64,
    activity: Mutex<SdkActivityProjection>,
    output: Mutex<String>,
    admission: AdmissionHub,
    deadline: Option<Deadline>,
    cancelled: AtomicBool,
    timed_out: AtomicBool,
    finished: AtomicBool,
    finish_signal: Arc<Mutex<FinishedState>>,
    /// The session's admitted tool set. Admission never widens per turn.
    profile: crate::sdk::profile::ClaudeAgentSdkSessionProfile,
    admitted_mcp_tools: Vec<String>,
}

pub(super) struct TurnEndedDiagnostic {
    pub(super) stop_reason: String,
    pub(super) failed: bool,
    pub(super) subtype: Option<String>,
    pub(super) num_turns: Option<u64>,
    pub(super) duration_ms: Option<u64>,
    pub(super) error_text_present: bool,
    pub(super) error_text_type: String,
    pub(super) result_field_presence: BTreeMap<String, bool>,
    /// Validated numeric provider HTTP status, if the result carried one.
    pub(super) api_error_status: Option<u64>,
    /// Bounded terminal reason, if the result carried one.
    pub(super) terminal_reason: Option<String>,
    /// Latest validated active-turn rate-limit status, if any turn-owned
    /// notice arrived before this result. Advisory only.
    pub(super) rate_limit_status: Option<super::wire::ClaudeAgentSdkRateLimitStatus>,
}

impl SdkActiveTurn {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        connection: Weak<super::connection::SdkConnection>,
        deadline: Option<Deadline>,
        profile: crate::sdk::profile::ClaudeAgentSdkSessionProfile,
        admitted_mcp_tools: Vec<String>,
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
        let (admission, exchange) = AdmissionHub::new(connection.clone());
        Ok((
            Arc::new(Self {
                connection,
                activity: Mutex::new(SdkActivityProjection::new(runtime_id.clone())),
                runtime_id,
                events,
                terminal,
                sequence: AtomicU64::new(1),
                output: Mutex::new(String::new()),
                admission,
                deadline,
                cancelled: AtomicBool::new(false),
                timed_out: AtomicBool::new(false),
                finished: AtomicBool::new(false),
                finish_signal: Arc::new(Mutex::new(FinishedState::default())),
                profile,
                admitted_mcp_tools,
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

    /// Records that the host deadline, not the consumer, ended this turn.
    pub(crate) fn mark_timed_out(&self) {
        self.timed_out.store(true, Ordering::SeqCst);
    }

    pub(crate) fn finished_future(&self) -> finished::TurnFinishedFuture {
        finished::TurnFinishedFuture::new(Arc::clone(&self.finish_signal))
    }

    pub(crate) fn abandon_admissions(&self, reason: CallbackAbandonment) {
        self.admission.abandon(reason);
    }

    pub(crate) fn handle_admission(
        &self,
        sidecar_id: &str,
        tool_name: &str,
        bash_command: Option<&ClaudeAgentSdkBashCommandView>,
    ) -> Result<AdmissionDisposition, RuntimeFailure> {
        // The session's admitted set is an allow-list on both sides of the
        // wire. A tool outside it never reaches the consumer, and its arrival
        // is itself a transport failure rather than a decision to delegate.
        // This is checked first, so an out-of-set request stays fatal even
        // when it races the turn's end.
        let Some(tool) = crate::sdk::profile::ClaudeAgentSdkTool::parse(tool_name) else {
            if !self
                .admitted_mcp_tools
                .iter()
                .any(|admitted| admitted == tool_name)
            {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.admission_tool_unadmitted",
                    "Claude Agent SDK sidecar requested admission for a tool outside the admitted set",
                ));
            }
            if bash_command.is_some() {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.admission_bash_view_unexpected",
                    "Claude Agent SDK non-Bash admission carried a Bash command view",
                ));
            }
            if self.is_finished() {
                return Ok(AdmissionDisposition::RacedTurnEnd);
            }
            let sequence = self.next_sequence();
            let Some(callback_id) = self.admission.enqueue(
                &self.runtime_id,
                sequence,
                self.deadline,
                sidecar_id,
                tool_name,
                bash_command,
            )?
            else {
                return Ok(AdmissionDisposition::RacedTurnEnd);
            };
            self.events.send(RuntimeEvent::new(
                sequence,
                RuntimeEventKind::CallbackRequested(callback_id),
            ))?;
            return Ok(AdmissionDisposition::Delegated);
        };
        if !self.profile.admits(tool) {
            return Err(failure(
                "swallowtail.claude-agent.sdk.admission_tool_unadmitted",
                "Claude Agent SDK sidecar requested admission for a tool outside the admitted set",
            ));
        }
        if tool == crate::sdk::profile::ClaudeAgentSdkTool::Bash {
            if bash_command.is_none() {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.admission_bash_view_missing",
                    "Claude Agent SDK Bash admission omitted its command view",
                ));
            }
        } else if bash_command.is_some() {
            return Err(failure(
                "swallowtail.claude-agent.sdk.admission_bash_view_unexpected",
                "Claude Agent SDK non-Bash admission carried a Bash command view",
            ));
        }
        if self.is_finished() {
            return Ok(AdmissionDisposition::RacedTurnEnd);
        }
        let sequence = self.next_sequence();
        let Some(callback_id) = self.admission.enqueue(
            &self.runtime_id,
            sequence,
            self.deadline,
            sidecar_id,
            tool_name,
            bash_command,
        )?
        else {
            return Ok(AdmissionDisposition::RacedTurnEnd);
        };
        self.events.send(RuntimeEvent::new(
            sequence,
            RuntimeEventKind::CallbackRequested(callback_id),
        ))?;
        Ok(AdmissionDisposition::Delegated)
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

    pub(super) fn add_stderr_to_diagnostic(&self, diagnostic: SafeDiagnostic) -> SafeDiagnostic {
        self.connection
            .upgrade()
            .map_or(diagnostic.clone(), |connection| {
                connection.diagnostic_with_stderr(diagnostic)
            })
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
        let mut signal = self
            .finish_signal
            .lock()
            .expect("SDK sidecar turn-finished lock poisoned");
        signal.finished = true;
        if let Some(waiter) = signal.waiter.take() {
            waiter.wake();
        }
    }
}

pub(super) fn provider_diagnostic() -> SafeDiagnostic {
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
pub(super) fn provider_turn_ended_diagnostic(fields: TurnEndedDiagnostic) -> SafeDiagnostic {
    let TurnEndedDiagnostic {
        stop_reason,
        failed,
        subtype,
        num_turns,
        duration_ms,
        error_text_present,
        error_text_type,
        result_field_presence,
        api_error_status,
        terminal_reason,
        rate_limit_status,
    } = fields;
    let fields = result_field_presence
        .into_iter()
        .map(|(field, present)| format!("{field}={present}"))
        .collect::<Vec<_>>()
        .join(",");
    let subtype = subtype.as_deref().unwrap_or("<null>");
    let terminal_reason = terminal_reason.as_deref().unwrap_or("<null>");
    let rate_limit_status = rate_limit_status
        .map(|status| status.as_str().to_owned())
        .unwrap_or_else(|| "<null>".to_owned());
    let (code, summary) = provider_status_summary(api_error_status);
    SafeDiagnostic::new(
        code,
        format!(
            "{summary}; turn_ended: subtype={subtype}; stopReason={stop_reason}; isError={failed}; numTurns={}; durationMs={}; errorTextPresent={error_text_present}; errorTextType={error_text_type}; apiErrorStatus={}; terminalReason={terminal_reason}; rateLimitStatus={rate_limit_status}; resultFieldPresence=[{fields}]",
            optional_number(num_turns),
            optional_number(duration_ms),
            optional_number(api_error_status),
        ),
    )
    .with_failure_classification(provider_status_classification(api_error_status))
}

/// Stable route code and human summary for one validated provider HTTP
/// status. Only `402` proves a billing/entitlement outcome under the
/// provider's documented error contract; `400` and `429` each mix
/// spend-limit and non-billing causes, so they keep distinct mixed route
/// codes but no quota classification. Absent or unlisted statuses stay
/// generic. No status authorizes retry, fallback, replay, or account
/// mutation.
fn provider_status_summary(api_error_status: Option<u64>) -> (&'static str, &'static str) {
    match api_error_status {
        Some(402) => (
            "swallowtail.claude-agent.sdk.provider_billing_unavailable",
            "Claude Agent SDK sidecar reported a downstream provider billing/entitlement failure",
        ),
        Some(400) => (
            "swallowtail.claude-agent.sdk.provider_invalid_request_or_spend_limit",
            "Claude Agent SDK sidecar reported a downstream provider failure: invalid request or spend limit",
        ),
        Some(429) => (
            "swallowtail.claude-agent.sdk.provider_rate_or_spend_limit",
            "Claude Agent SDK sidecar reported a downstream provider failure: rate or spend limit",
        ),
        _ => (
            "swallowtail.claude-agent.sdk.provider_failed",
            "Claude Agent SDK sidecar reported a downstream provider failure",
        ),
    }
}

/// Portable classification for one validated provider HTTP status. Only the
/// documented `402` billing status classifies beyond generic provider
/// failure; mixed and unknown statuses never become `QuotaExhausted`.
fn provider_status_classification(api_error_status: Option<u64>) -> FailureClassification {
    match api_error_status {
        Some(402) => FailureClassification::new(
            FailureOrigin::Provider,
            FailureKind::EntitlementUnavailable,
            FailureRecovery::ConfigurationChangeRequired,
        ),
        _ => FailureClassification::new(
            FailureOrigin::Provider,
            FailureKind::Unknown,
            FailureRecovery::Unknown,
        ),
    }
}

fn optional_number(value: Option<u64>) -> String {
    value.map_or_else(|| "<null>".to_owned(), |value| value.to_string())
}
