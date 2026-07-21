use crate::failure::{failure, malformed};
use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{
    BoxEventStream, CleanupOutcome, OperationContent, RuntimeEvent, RuntimeEventKind,
    RuntimeFailure, RuntimeTurnId, TerminalOutcome, TerminalOutcomeFuture, TerminalOutcomeSender,
    TerminalStatus, runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 128;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

pub(crate) struct ActiveTurn {
    session_id: String,
    events: swallowtail_runtime::RuntimeEventSender,
    terminal: TerminalOutcomeSender,
    sequence: AtomicU64,
    output: Mutex<String>,
    cancelled: AtomicBool,
    finished: AtomicBool,
}

impl ActiveTurn {
    pub(crate) fn new(
        _runtime_id: RuntimeTurnId,
        session_id: String,
    ) -> Result<(Arc<Self>, BoxEventStream, TerminalOutcomeFuture), RuntimeFailure> {
        let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let (terminal, future) = terminal_outcome_channel();
        Ok((
            Arc::new(Self {
                session_id,
                events,
                terminal,
                sequence: AtomicU64::new(1),
                output: Mutex::new(String::new()),
                cancelled: AtomicBool::new(false),
                finished: AtomicBool::new(false),
            }),
            Box::pin(stream),
            future,
        ))
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }

    pub(crate) fn mark_cancelled(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub(crate) fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        if params.get("sessionId").and_then(Value::as_str) != Some(&self.session_id) {
            return Err(failure(
                "swallowtail.kimi.acp.session_mismatch",
                "Kimi Code update does not match the bound session",
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
                | "user_message_chunk"
                | "available_commands_update"
                | "config_option_update"
                | "current_mode_update"
                | "usage_update",
            ) => self.emit(RuntimeEventKind::Progress, None),
            _ => Err(failure(
                "swallowtail.kimi.acp.update_unsupported",
                "Kimi Code returned an unsupported ACP session update",
            )),
        }
    }

    pub(crate) fn finish_prompt(&self, stop_reason: &str) {
        let status = match stop_reason {
            "end_turn" => TerminalStatus::Completed,
            "cancelled" => TerminalStatus::Cancelled,
            "max_tokens" | "max_turn_requests" | "refusal" => {
                TerminalStatus::ProviderFailed(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.kimi.acp.prompt_stopped",
                    "Kimi Code stopped before completing the turn",
                ))
            }
            _ => TerminalStatus::RuntimeFailed(
                failure(
                    "swallowtail.kimi.acp.stop_reason_unsupported",
                    "Kimi Code returned an unsupported ACP stop reason",
                )
                .diagnostic()
                .clone(),
            ),
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
                "swallowtail.kimi.acp.output_limit_exceeded",
                "Kimi Code output exceeded the adapter limit",
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

fn text_content(update: &Value) -> Result<&str, RuntimeFailure> {
    update
        .get("content")
        .filter(|content| content.get("type").and_then(Value::as_str) == Some("text"))
        .and_then(|content| content.get("text"))
        .and_then(Value::as_str)
        .ok_or_else(malformed)
}
