use crate::failure::{failure, malformed};
use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{ExtensionNamespace, ProviderRequestRef};
use swallowtail_runtime::{
    BoxEventStream, CleanupOutcome, OperationContent, ProviderRequestObservation, RuntimeEvent,
    RuntimeEventKind, RuntimeFailure, RuntimeTurnId, TerminalOutcome, TerminalOutcomeFuture,
    TerminalOutcomeSender, TerminalStatus, TokenUsage, runtime_event_channel,
    terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 128;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

mod finished;

use finished::{FinishedSignal, TurnFinishedFuture};

pub(crate) struct ActiveTurn {
    runtime_id: RuntimeTurnId,
    session_id: String,
    events: swallowtail_runtime::RuntimeEventSender,
    terminal: TerminalOutcomeSender,
    sequence: AtomicU64,
    output: Mutex<String>,
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
    ) -> Result<(Arc<Self>, BoxEventStream, TerminalOutcomeFuture), RuntimeFailure> {
        let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let (terminal, future) = terminal_outcome_channel();
        Ok((
            Arc::new(Self {
                runtime_id,
                session_id,
                events,
                terminal,
                sequence: AtomicU64::new(1),
                output: Mutex::new(String::new()),
                provider_observation: Mutex::new(None),
                cancelled: AtomicBool::new(false),
                timed_out: AtomicBool::new(false),
                finished: AtomicBool::new(false),
                finish_signal: FinishedSignal::new(),
            }),
            Box::pin(stream),
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

    pub(crate) fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        if params.get("sessionId").and_then(Value::as_str) != Some(&self.session_id) {
            return Err(failure(
                "swallowtail.claude_agent.acp.session_mismatch",
                "Claude Agent update does not match the active session",
            ));
        }
        let update = params.get("update").ok_or_else(malformed)?;
        match update.get("sessionUpdate").and_then(Value::as_str) {
            Some("agent_message_chunk") => {
                let text = text_content(update)?;
                self.append_output(text)?;
                self.emit_content(RuntimeEventKind::OutputDelta, text)
            }
            Some("agent_thought_chunk") => {
                self.emit_content(RuntimeEventKind::ReasoningProgress, text_content(update)?)
            }
            Some(
                "tool_call"
                | "tool_call_update"
                | "plan"
                | "usage_update"
                | "config_option_update"
                | "current_mode_update"
                | "available_commands_update",
            ) => self.emit(RuntimeEventKind::Progress, None),
            _ => Err(failure(
                "swallowtail.claude_agent.acp.update_unsupported",
                "Claude Agent returned an unsupported ACP session update",
            )),
        }
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

fn text_content(update: &Value) -> Result<&str, RuntimeFailure> {
    update
        .get("content")
        .filter(|content| content.get("type").and_then(Value::as_str) == Some("text"))
        .and_then(|content| content.get("text"))
        .and_then(Value::as_str)
        .ok_or_else(malformed)
}
