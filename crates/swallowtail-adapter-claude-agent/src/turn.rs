use crate::failure::{failure, malformed};
use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use swallowtail_core::{
    ExtensionNamespace, ProviderRequestHandling, ProviderRequestPolicy, ProviderRequestRef,
};
use swallowtail_runtime::{
    BoxEventStream, CallbackAbandonment, CallbackExchange, CleanupOutcome, Deadline,
    OperationContent, ProviderRequestObservation, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
    RuntimeTurnId, TerminalOutcome, TerminalOutcomeFuture, TerminalOutcomeSender, TerminalStatus,
    TokenUsage, runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 128;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

mod finished;

use finished::{FinishedSignal, TurnFinishedFuture};
use swallowtail_protocol_acp::{AcpContentBlock, AcpMessageRole, AcpSessionUpdate};

pub(crate) struct ActiveTurn {
    runtime_id: RuntimeTurnId,
    session_id: String,
    events: swallowtail_runtime::RuntimeEventSender,
    terminal: TerminalOutcomeSender,
    sequence: AtomicU64,
    output: Mutex<String>,
    activity: Mutex<crate::acp_activity::AcpActivityProjection>,
    deadline: Option<Deadline>,
    callbacks: crate::permission::CallbackHub,
    connection: Weak<crate::connection::AcpConnection>,
    provider_observation: Mutex<Option<ProviderRequestObservation>>,
    cancelled: AtomicBool,
    timed_out: AtomicBool,
    finished: AtomicBool,
    finish_signal: FinishedSignal,
}

impl ActiveTurn {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        session_id: String,
        deadline: Option<Deadline>,
        provider_requests: &ProviderRequestPolicy,
        connection: Weak<crate::connection::AcpConnection>,
    ) -> Result<
        (
            Arc<Self>,
            BoxEventStream,
            Option<CallbackExchange>,
            TerminalOutcomeFuture,
        ),
        RuntimeFailure,
    > {
        let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let (terminal, future) = terminal_outcome_channel();
        let exchanges_permissions = matches!(
            provider_requests.handling_for(&crate::claude_agent_permission_namespace()),
            ProviderRequestHandling::Exchange
        );
        let (callbacks, callback_exchange) =
            crate::permission::CallbackHub::new(connection.clone(), exchanges_permissions);
        Ok((
            Arc::new(Self {
                runtime_id: runtime_id.clone(),
                session_id,
                events,
                terminal,
                sequence: AtomicU64::new(1),
                output: Mutex::new(String::new()),
                activity: Mutex::new(crate::acp_activity::AcpActivityProjection::new(runtime_id)),
                deadline,
                callbacks,
                connection,
                provider_observation: Mutex::new(None),
                cancelled: AtomicBool::new(false),
                timed_out: AtomicBool::new(false),
                finished: AtomicBool::new(false),
                finish_signal: FinishedSignal::new(),
            }),
            Box::pin(stream),
            Some(callback_exchange),
            future,
        ))
    }

    pub(crate) const fn runtime_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    pub(crate) fn session_id(&self) -> &str {
        &self.session_id
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }

    pub(crate) fn finished_future(&self) -> TurnFinishedFuture {
        self.finish_signal.future()
    }

    pub(crate) fn mark_cancelled(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
        self.callbacks.abandon(CallbackAbandonment::TurnCancelled);
    }

    pub(crate) fn finish_for_session_close(&self) {
        self.mark_cancelled();
        self.finish(TerminalStatus::Cancelled);
    }

    pub(crate) fn timeout(&self) {
        self.timed_out.store(true, Ordering::SeqCst);
        self.finish(TerminalStatus::TimedOut);
    }

    pub(crate) fn observe_permission(
        &self,
        provider_request_id: &Value,
    ) -> Result<(), RuntimeFailure> {
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        let callback_id = swallowtail_runtime::CallbackId::new(format!(
            "{}:permission:{sequence}",
            self.runtime_id.as_str()
        ))
        .map_err(|_| malformed())?;
        let provider_request_ref = ProviderRequestRef::new(match provider_request_id {
            Value::String(value) => format!("acp:{value}"),
            Value::Number(value) => format!("acp:{value}"),
            _ => return Err(malformed()),
        })
        .map_err(|_| malformed())?;
        *self
            .provider_observation
            .lock()
            .expect("permission observation lock poisoned") =
            Some(ProviderRequestObservation::new(
                callback_id,
                ExtensionNamespace::new("acp/session/request-permission")
                    .expect("static namespace is valid"),
                provider_request_ref.clone(),
            ));
        self.mark_cancelled();
        self.emit(
            RuntimeEventKind::ProviderObservation(
                swallowtail_runtime::ProviderObservation::RequestCorrelation(provider_request_ref),
            ),
            None,
        )
    }

    pub(crate) const fn exchanges_permissions(&self) -> bool {
        self.callbacks.exchanges_permissions()
    }

    pub(crate) fn exchange_permission(
        &self,
        provider_request_id: &Value,
        params: &Value,
    ) -> Result<(), RuntimeFailure> {
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        let callback_id = self.callbacks.enqueue_permission(
            &self.runtime_id,
            sequence,
            self.deadline,
            provider_request_id,
            params,
        )?;
        self.events.send(RuntimeEvent::new(
            sequence,
            RuntimeEventKind::CallbackRequested(callback_id),
        ))
    }

    pub(crate) fn exchange_user_input(
        &self,
        provider_request_id: &Value,
        request: swallowtail_runtime::HarnessUserInputRequest,
    ) -> Result<(), RuntimeFailure> {
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        let callback_id = self.callbacks.enqueue_user_input(
            &self.runtime_id,
            sequence,
            self.deadline,
            provider_request_id,
            request,
        )?;
        self.events.send(RuntimeEvent::new(
            sequence,
            RuntimeEventKind::CallbackRequested(callback_id),
        ))
    }

    pub(crate) fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let decoded =
            swallowtail_protocol_acp::decode_session_update(params).map_err(|_| malformed())?;
        if decoded.session_id.as_str() != self.session_id {
            return Err(failure(
                "swallowtail.claude_agent.acp.session_mismatch",
                "Claude Agent update does not match the active session",
            ));
        }
        match &decoded.update {
            AcpSessionUpdate::Message(message) if message.role == AcpMessageRole::Agent => {
                let text = text_content(&message.content)?;
                self.append_output(text)?;
                self.emit_content(RuntimeEventKind::OutputDelta, text)?;
            }
            AcpSessionUpdate::Message(message) if message.role == AcpMessageRole::Thought => {
                self.emit_content(
                    RuntimeEventKind::ReasoningProgress,
                    text_content(&message.content)?,
                )?;
            }
            AcpSessionUpdate::Message(_)
            | AcpSessionUpdate::AvailableCommands(_)
            | AcpSessionUpdate::CurrentMode(_)
            | AcpSessionUpdate::ConfigOptions(_)
            | AcpSessionUpdate::SessionInfo { .. }
            | AcpSessionUpdate::Usage(_) => self.emit(RuntimeEventKind::Progress, None)?,
            AcpSessionUpdate::ToolCall(_)
            | AcpSessionUpdate::ToolCallUpdate(_)
            | AcpSessionUpdate::Plan(_)
            | AcpSessionUpdate::Unknown { .. } => {}
        }
        let observations = self
            .activity
            .lock()
            .expect("turn activity lock poisoned")
            .project(&decoded.update)?;
        for observation in observations {
            self.emit(RuntimeEventKind::Activity(observation), None)?;
        }
        Ok(())
    }

    pub(crate) fn finish_prompt(&self, response: &Value) {
        let stop_reason = match response.get("stopReason").and_then(Value::as_str) {
            Some(stop_reason) => stop_reason,
            None => {
                self.fail(&malformed());
                return;
            }
        };
        let usage = match prompt_usage(response) {
            Ok(usage) => usage,
            Err(error) => {
                self.fail(&error);
                return;
            }
        };
        let status = if let Some(observation) = self
            .provider_observation
            .lock()
            .expect("permission observation lock poisoned")
            .clone()
        {
            TerminalStatus::ProviderRequestObserved(observation)
        } else if self.timed_out.load(Ordering::SeqCst) {
            TerminalStatus::TimedOut
        } else {
            match stop_reason {
                "end_turn" => TerminalStatus::Completed,
                "cancelled" => TerminalStatus::Cancelled,
                "max_tokens" | "max_turn_requests" | "refusal" => {
                    TerminalStatus::ProviderFailed(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.claude_agent.acp.prompt_stopped",
                        "Claude Agent stopped before completing the turn",
                    ))
                }
                _ => TerminalStatus::RuntimeFailed(
                    failure(
                        "swallowtail.claude_agent.acp.stop_reason_unsupported",
                        "Claude Agent returned an unsupported ACP stop reason",
                    )
                    .diagnostic()
                    .clone(),
                ),
            }
        };
        self.finish_with_usage(status, usage);
    }

    pub(crate) fn fail(&self, error: &RuntimeFailure) {
        if let Some(connection) = self.connection.upgrade() {
            connection.emit_protocol_debug(error, "acp.turn.fail");
        }
        self.finish(TerminalStatus::RuntimeFailed(error.diagnostic().clone()));
    }

    fn finish(&self, status: TerminalStatus) {
        self.finish_internal(status, None);
    }

    fn finish_with_usage(&self, status: TerminalStatus, usage: TokenUsage) {
        self.finish_internal(status, Some(usage));
    }

    fn finish_internal(&self, status: TerminalStatus, usage: Option<TokenUsage>) {
        if self.finished.swap(true, Ordering::SeqCst) {
            return;
        }
        self.callbacks.abandon(CallbackAbandonment::TurnTerminated);
        if let Ok(observations) = self
            .activity
            .lock()
            .expect("turn activity lock poisoned")
            .complete(&status)
        {
            for observation in observations {
                let _ = self.emit(RuntimeEventKind::Activity(observation), None);
            }
        }
        let output = self
            .output
            .lock()
            .expect("turn output lock poisoned")
            .clone();
        let mut outcome = TerminalOutcome::new(status, CleanupOutcome::NotApplicable);
        if let Ok(content) = OperationContent::new(output) {
            let _ = self.emit(RuntimeEventKind::OutputAvailable, Some(content.clone()));
            outcome = outcome.with_output(content);
        }
        if let Some(usage) = usage {
            let _ = self.emit(
                RuntimeEventKind::ProviderObservation(
                    swallowtail_runtime::ProviderObservation::Usage(usage),
                ),
                None,
            );
        }
        self.events.mark_terminal();
        let _ = self.terminal.complete(outcome);
        self.finish_signal.finish();
    }

    fn append_output(&self, text: &str) -> Result<(), RuntimeFailure> {
        let mut output = self.output.lock().expect("turn output lock poisoned");
        if output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(failure(
                "swallowtail.claude_agent.acp.output_limit_exceeded",
                "Claude Agent output exceeded the adapter limit",
            ));
        }
        output.push_str(text);
        Ok(())
    }

    fn emit_content(&self, kind: RuntimeEventKind, text: &str) -> Result<(), RuntimeFailure> {
        match OperationContent::new(text) {
            Ok(content) => self.emit(kind, Some(content)),
            Err(_) if text.trim().is_empty() => Ok(()),
            Err(_) => Err(malformed()),
        }
    }

    fn emit(
        &self,
        kind: RuntimeEventKind,
        content: Option<OperationContent>,
    ) -> Result<(), RuntimeFailure> {
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        self.events.send(match content {
            Some(content) => RuntimeEvent::with_content(sequence, kind, content),
            None => RuntimeEvent::new(sequence, kind),
        })
    }
}

fn prompt_usage(response: &Value) -> Result<TokenUsage, RuntimeFailure> {
    let usage = response.get("usage").ok_or_else(malformed)?;
    let input = usage
        .get("inputTokens")
        .and_then(Value::as_u64)
        .ok_or_else(malformed)?;
    let output = usage
        .get("outputTokens")
        .and_then(Value::as_u64)
        .ok_or_else(malformed)?;
    let cache_read = usage
        .get("cachedReadTokens")
        .and_then(Value::as_u64)
        .ok_or_else(malformed)?;
    let cache_write = usage
        .get("cachedWriteTokens")
        .and_then(Value::as_u64)
        .ok_or_else(malformed)?;
    let total = usage
        .get("totalTokens")
        .and_then(Value::as_u64)
        .ok_or_else(malformed)?;
    let calculated = input
        .checked_add(output)
        .and_then(|value| value.checked_add(cache_read))
        .and_then(|value| value.checked_add(cache_write))
        .ok_or_else(malformed)?;
    if calculated != total {
        return Err(malformed());
    }
    Ok(TokenUsage::new(Some(input), Some(output))
        .with_cache_tokens(Some(cache_read), Some(cache_write)))
}

fn text_content(content: &AcpContentBlock) -> Result<&str, RuntimeFailure> {
    match content {
        AcpContentBlock::Text(text) => Ok(text.as_str()),
        _ => Err(malformed()),
    }
}
