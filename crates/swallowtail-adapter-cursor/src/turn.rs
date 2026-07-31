use crate::activity::{AcpActivityProjection, text_content};
use crate::failure::{failure, malformed};
use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{ExtensionNamespace, ProviderRequestRef};
use swallowtail_protocol_acp::{AcpMessageRole, AcpSessionUpdate};
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
    activity: Mutex<AcpActivityProjection>,
    provider_observation: Mutex<Option<ProviderRequestObservation>>,
    cancelled: AtomicBool,
    timed_out: AtomicBool,
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
                runtime_id: runtime_id.clone(),
                session_id,
                events,
                terminal,
                sequence: AtomicU64::new(1),
                output: Mutex::new(String::new()),
                activity: Mutex::new(AcpActivityProjection::new(runtime_id)),
                provider_observation: Mutex::new(None),
                cancelled: AtomicBool::new(false),
                timed_out: AtomicBool::new(false),
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
        let observation = ProviderRequestObservation::new(
            callback_id,
            ExtensionNamespace::new("acp/session/request-permission")
                .expect("static namespace is valid"),
            provider_request_ref.clone(),
        );
        *self
            .provider_observation
            .lock()
            .expect("permission observation lock poisoned") = Some(observation);
        self.mark_cancelled();
        self.emit(
            RuntimeEventKind::ProviderObservation(
                swallowtail_runtime::ProviderObservation::RequestCorrelation(provider_request_ref),
            ),
            None,
        )
    }

    pub(crate) fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let decoded =
            swallowtail_protocol_acp::decode_session_update(params).map_err(|_| malformed())?;
        self.verify_session(decoded.session_id.as_str())?;
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
            | AcpSessionUpdate::Plan(_)
            | AcpSessionUpdate::AvailableCommands(_)
            | AcpSessionUpdate::CurrentMode(_)
            | AcpSessionUpdate::ConfigOptions(_)
            | AcpSessionUpdate::SessionInfo { .. }
            | AcpSessionUpdate::Usage(_) => self.emit(RuntimeEventKind::Progress, None)?,
            AcpSessionUpdate::ToolCall(_)
            | AcpSessionUpdate::ToolCallUpdate(_)
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

    pub(crate) fn finish_prompt(&self, stop_reason: &str) {
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
                        "swallowtail.cursor.acp.prompt_stopped",
                        "Cursor Agent stopped before completing the turn",
                    ))
                }
                _ => TerminalStatus::RuntimeFailed(
                    failure(
                        "swallowtail.cursor.acp.stop_reason_unsupported",
                        "Cursor Agent returned an unsupported ACP stop reason",
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
        self.events.mark_terminal();
        let _ = self.terminal.complete(outcome);
    }

    fn append_output(&self, text: &str) -> Result<(), RuntimeFailure> {
        let mut output = self.output.lock().expect("turn output lock poisoned");
        if output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(failure(
                "swallowtail.cursor.acp.output_limit_exceeded",
                "Cursor Agent output exceeded the adapter limit",
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

    fn verify_session(&self, session_id: &str) -> Result<(), RuntimeFailure> {
        if session_id == self.session_id {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.cursor.acp.session_mismatch",
                "Cursor Agent update does not match the active session",
            ))
        }
    }
}
