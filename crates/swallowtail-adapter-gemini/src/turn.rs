use crate::failure::{failure, malformed};
use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{ExtensionNamespace, ProviderRequestRef};
use swallowtail_runtime::{
    BoxEventStream, CleanupOutcome, OperationContent, ProviderRequestObservation, RuntimeEvent,
    RuntimeEventKind, RuntimeFailure, RuntimeTurnId, TerminalOutcome, TerminalOutcomeFuture,
    TerminalOutcomeSender, TerminalStatus, runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 128;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

pub(crate) struct ActiveTurn {
    runtime_id: RuntimeTurnId,
    session_id: String,
    events: swallowtail_runtime::RuntimeEventSender,
    terminal: TerminalOutcomeSender,
    sequence: AtomicU64,
    output: Mutex<String>,
    provider_observation: Mutex<Option<ProviderRequestObservation>>,
    cancelled: AtomicBool,
    finished: AtomicBool,
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
                finished: AtomicBool::new(false),
            }),
            Box::pin(stream),
            future,
        ))
    }

    pub(crate) fn session_id(&self) -> &str {
        &self.session_id
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }

    pub(crate) fn mark_cancelled(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub(crate) fn observe_permission(
        &self,
        provider_request_id: &Value,
    ) -> Result<ProviderRequestObservation, RuntimeFailure> {
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
        let observation = ProviderRequestObservation::new(
            callback_id,
            ExtensionNamespace::new("acp/session/request-permission")
                .expect("static namespace is valid"),
            provider_request_ref.clone(),
        );
        *self
            .provider_observation
            .lock()
            .expect("permission observation lock poisoned") = Some(observation.clone());
        self.mark_cancelled();
        self.emit(
            RuntimeEventKind::ProviderObservation(
                swallowtail_runtime::ProviderObservation::RequestCorrelation(provider_request_ref),
            ),
            None,
        )?;
        Ok(observation)
    }

    pub(crate) fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        self.verify_session(params)?;
        let update = params.get("update").ok_or_else(malformed)?;
        match update.get("sessionUpdate").and_then(Value::as_str) {
            Some("agent_message_chunk") => {
                let text = text_content(update)?;
                self.append_output(text)?;
                self.emit_content(RuntimeEventKind::OutputDelta, text)
            }
            Some("agent_thought_chunk") => {
                let text = text_content(update)?;
                self.emit_content(RuntimeEventKind::ReasoningProgress, text)
            }
            Some(
                "tool_call"
                | "tool_call_update"
                | "plan"
                | "user_message_chunk"
                | "available_commands_update"
                | "config_option_update"
                | "usage_update",
            ) => self.emit(RuntimeEventKind::Progress, None),
            Some("current_mode_update") => {
                let mode = update
                    .get("currentModeId")
                    .or_else(|| update.get("modeId"))
                    .and_then(Value::as_str)
                    .ok_or_else(malformed)?;
                if mode != "plan" {
                    return Err(failure(
                        "swallowtail.gemini.acp.mode_widened",
                        "Gemini CLI changed the read-only session mode",
                    ));
                }
                self.emit(RuntimeEventKind::Progress, None)
            }
            _ => Err(failure(
                "swallowtail.gemini.acp.update_unsupported",
                "Gemini CLI returned an unsupported ACP session update",
            )),
        }
    }

    pub(crate) fn finish_prompt(&self, stop_reason: &str) {
        let status = if let Some(observation) = self
            .provider_observation
            .lock()
            .expect("permission observation lock poisoned")
            .clone()
        {
            TerminalStatus::ProviderRequestObserved(observation)
        } else {
            match stop_reason {
                "end_turn" => TerminalStatus::Completed,
                "cancelled" if self.cancelled.load(Ordering::SeqCst) => TerminalStatus::Cancelled,
                "cancelled" => TerminalStatus::Cancelled,
                "max_tokens" | "max_turn_requests" | "refusal" => {
                    TerminalStatus::ProviderFailed(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.gemini.acp.prompt_stopped",
                        "Gemini CLI stopped before completing the turn",
                    ))
                }
                _ => TerminalStatus::RuntimeFailed(
                    failure(
                        "swallowtail.gemini.acp.stop_reason_unsupported",
                        "Gemini CLI returned an unsupported ACP stop reason",
                    )
                    .diagnostic()
                    .clone(),
                ),
            }
        };
        self.finish(status);
    }

    pub(crate) fn fail(&self, error: &RuntimeFailure) {
        self.finish(TerminalStatus::RuntimeFailed(error.diagnostic().clone()));
    }

    fn finish(&self, status: TerminalStatus) {
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
        self.events.mark_terminal();
        let _ = self.terminal.complete(outcome);
    }

    fn append_output(&self, text: &str) -> Result<(), RuntimeFailure> {
        let mut output = self.output.lock().expect("turn output lock poisoned");
        if output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(failure(
                "swallowtail.gemini.acp.output_limit_exceeded",
                "Gemini CLI output exceeded the adapter limit",
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
        let event = match content {
            Some(content) => RuntimeEvent::with_content(sequence, kind, content),
            None => RuntimeEvent::new(sequence, kind),
        };
        self.events.send(event)
    }

    fn verify_session(&self, params: &Value) -> Result<(), RuntimeFailure> {
        if params.get("sessionId").and_then(Value::as_str) == Some(&self.session_id) {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.gemini.acp.session_mismatch",
                "Gemini CLI update does not match the active session",
            ))
        }
    }
}

fn text_content(update: &Value) -> Result<&str, RuntimeFailure> {
    let content = update.get("content").ok_or_else(malformed)?;
    if content.get("type").and_then(Value::as_str) != Some("text") {
        return Err(failure(
            "swallowtail.gemini.acp.content_unsupported",
            "Gemini CLI returned unsupported ACP content",
        ));
    }
    content
        .get("text")
        .and_then(Value::as_str)
        .ok_or_else(malformed)
}

#[cfg(test)]
include!("turn_tests.rs");
