use crate::headless_activity::AntigravityActivityProjection;
use serde_json::Value;
use swallowtail_core::{ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityObservation, ActivityOperationId, OperationContent, ProviderObservation, RuntimeEvent,
    RuntimeEventKind, RuntimeFailure, TerminalStatus,
};

const MAXIMUM_LINE_BYTES: usize = 1024 * 1024;
const MAXIMUM_EVENT_COUNT: usize = 4096;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_SUBAGENTS: usize = 64;
type SubagentEvidence = (String, Option<String>);

mod helpers;
mod terminal;
use helpers::*;
pub(crate) use terminal::ParsedTerminal;

pub(crate) struct AntigravityEventParser {
    pending: Vec<u8>,
    sequence: u64,
    event_count: usize,
    conversation_id: Option<String>,
    expected_conversation_id: Option<String>,
    model: ModelId,
    schema_expected: bool,
    assistant_output: String,
    final_output: Option<OperationContent>,
    terminal_status: Option<TerminalStatus>,
    terminal_seen: bool,
    activity: AntigravityActivityProjection,
}

impl AntigravityEventParser {
    pub(crate) fn with_expected_conversation(
        operation_id: ActivityOperationId,
        model: ModelId,
        schema_expected: bool,
        expected_conversation_id: Option<String>,
    ) -> Self {
        Self {
            pending: Vec::new(),
            sequence: 1,
            event_count: 0,
            conversation_id: None,
            expected_conversation_id,
            model,
            schema_expected,
            assistant_output: String::new(),
            final_output: None,
            terminal_status: None,
            terminal_seen: false,
            activity: AntigravityActivityProjection::new(operation_id),
        }
    }

    pub(crate) fn push(&mut self, bytes: &[u8]) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.pending.extend_from_slice(bytes);
        let mut events = Vec::new();
        while let Some(newline) = self.pending.iter().position(|byte| *byte == b'\n') {
            if newline > MAXIMUM_LINE_BYTES {
                return Err(stream_limit());
            }
            let line: Vec<_> = self.pending.drain(..=newline).collect();
            events.extend(self.parse_line(trim_newline(&line))?);
        }
        if self.pending.len() > MAXIMUM_LINE_BYTES {
            return Err(stream_limit());
        }
        Ok(events)
    }

    pub(crate) fn finish(mut self) -> Result<(Vec<RuntimeEvent>, ParsedTerminal), RuntimeFailure> {
        let mut events = Vec::new();
        if !self.pending.is_empty() {
            let line = std::mem::take(&mut self.pending);
            events.extend(self.parse_line(&line)?);
        }
        Ok((
            events,
            ParsedTerminal::new(
                self.final_output,
                self.terminal_status,
                self.terminal_seen,
                self.conversation_id,
            ),
        ))
    }

    fn parse_line(&mut self, line: &[u8]) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if line.iter().all(u8::is_ascii_whitespace) {
            return Ok(Vec::new());
        }
        self.event_count = self.event_count.saturating_add(1);
        if self.event_count > MAXIMUM_EVENT_COUNT || self.terminal_seen {
            return Err(stream_limit());
        }
        let payload: Value = serde_json::from_slice(line).map_err(|_| malformed_stream())?;
        match required_text(&payload, "event")? {
            "init" => self.init(&payload),
            "step_update" => self.step_update(&payload),
            "result" => self.result(&payload),
            _ => Err(malformed_stream()),
        }
    }

    fn init(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if self.conversation_id.is_some() {
            return Err(malformed_stream());
        }
        let conversation_id = bounded_identity(required_text(payload, "conversation_id")?)?;
        if self
            .expected_conversation_id
            .as_deref()
            .is_some_and(|expected| expected != conversation_id)
        {
            return Err(conversation_mismatch());
        }
        let init = payload
            .get("init")
            .and_then(Value::as_object)
            .ok_or_else(malformed_stream)?;
        if init.get("model").and_then(Value::as_str) != Some(self.model.as_str())
            || init.get("permission_mode").and_then(Value::as_str) != Some("request-review")
            || !init.get("tools").is_some_and(Value::is_array)
            || !init.get("cwd").is_some_and(Value::is_string)
        {
            return Err(malformed_stream());
        }
        self.conversation_id = Some(conversation_id.to_owned());
        Ok(Vec::new())
    }

    fn step_update(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let step = payload.get("step_update").ok_or_else(malformed_stream)?;
        self.validate_conversation(step)?;
        let index = step
            .get("step_index")
            .and_then(Value::as_u64)
            .ok_or_else(malformed_stream)?;
        let active = match required_text(step, "state")? {
            "ACTIVE" => true,
            "DONE" => false,
            _ => return Err(malformed_stream()),
        };
        let step_type = required_text(step, "step_type")?;
        let text = step
            .get("text_delta")
            .map(|value| {
                value
                    .as_str()
                    .ok_or_else(malformed_stream)
                    .and_then(bounded_text)
            })
            .transpose()?;
        if let Some(text) = text {
            if self.assistant_output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES {
                return Err(stream_limit());
            }
            self.assistant_output.push_str(text);
        }
        let mut events = match step_type {
            "user_input" => Vec::new(),
            "agent_response" => {
                let mut events = if let Some(text) = text.filter(|value| !value.is_empty()) {
                    vec![self.event_with(
                        RuntimeEventKind::OutputDelta,
                        OperationContent::new(text).map_err(|_| malformed_stream())?,
                    )]
                } else {
                    Vec::new()
                };
                let activity = self.activity.assistant(index, active, text)?;
                events.extend(self.activity_events(activity));
                events
            }
            "tool" => {
                let name = bounded_label(required_text(step, "tool_name")?)?;
                let failed = step
                    .pointer("/tool_info/error")
                    .is_some_and(|value| !value.is_null());
                let activity = self.activity.tool(index, active, name, failed)?;
                self.activity_events(activity)
            }
            other => {
                if active {
                    Vec::new()
                } else {
                    let activity = self.activity.unknown(index, bounded_label(other)?)?;
                    self.activity_events(activity)
                }
            }
        };
        if let Some(children) = subagents(step)? {
            let activity = self.activity.subagents(index, children)?;
            events.extend(self.activity_events(activity));
        }
        if let Some(usage) = token_usage(step)? {
            events.push(self.event(RuntimeEventKind::ProviderObservation(
                ProviderObservation::Usage(usage),
            )));
        }
        Ok(events)
    }

    fn result(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let result = payload.get("result").ok_or_else(malformed_stream)?;
        if !result.get("duration_seconds").is_some_and(Value::is_number)
            || result.get("num_turns").and_then(Value::as_u64).is_none()
        {
            return Err(malformed_stream());
        }
        let status_name = required_text(result, "status")?;
        let invalid_model_without_init = status_name == "ERROR"
            && self.conversation_id.is_none()
            && result.get("conversation_id").and_then(Value::as_str) == Some("")
            && result
                .get("error")
                .and_then(Value::as_str)
                .is_some_and(|error| error.contains("invalid model selection"));
        if !invalid_model_without_init {
            self.validate_conversation(result)?;
        }
        let status = match status_name {
            "SUCCESS" => TerminalStatus::Completed,
            "ERROR" => {
                let error = bounded_text(required_text(result, "error")?)?;
                if error.contains("invalid model selection") {
                    TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                        "swallowtail.antigravity.headless.invalid_model",
                        "Antigravity rejected the explicit model selection",
                    ))
                } else {
                    TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                        "swallowtail.antigravity.headless.provider_error",
                        "Antigravity reported a headless provider failure",
                    ))
                }
            }
            "CANCELED" | "INTERRUPTED" => TerminalStatus::Cancelled,
            "INVALID" | "WAITING" | "RUNNING" => {
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.antigravity.headless.invalid_terminal_status",
                    "Antigravity ended with a non-terminal or invalid status",
                ))
            }
            _ => return Err(malformed_stream()),
        };
        let response = result
            .get("response")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        let mut events = Vec::new();
        if status == TerminalStatus::Completed {
            let output = if self.schema_expected {
                let structured = result
                    .get("structured_output")
                    .filter(|value| value.is_object())
                    .ok_or_else(malformed_stream)?;
                if !result.get("json_schema").is_some_and(Value::is_object) {
                    return Err(malformed_stream());
                }
                let response_value: Value =
                    serde_json::from_str(response.trim()).map_err(|_| malformed_stream())?;
                if &response_value != structured {
                    return Err(malformed_stream());
                }
                OperationContent::new(
                    serde_json::to_string(structured).map_err(|_| malformed_stream())?,
                )
                .map_err(|_| malformed_stream())?
            } else {
                if result.get("structured_output").is_some() || result.get("json_schema").is_some()
                {
                    return Err(malformed_stream());
                }
                if !self.assistant_output.is_empty() && self.assistant_output != response {
                    return Err(malformed_stream());
                }
                OperationContent::new(response).map_err(|_| malformed_stream())?
            };
            self.final_output = Some(output.clone());
            events.push(self.event_with(RuntimeEventKind::OutputAvailable, output));
        }
        let usage = token_usage(result)?.ok_or_else(malformed_stream)?;
        events.push(self.event(RuntimeEventKind::ProviderObservation(
            ProviderObservation::Usage(usage),
        )));
        let activity = self.activity.complete(&status)?;
        events.extend(self.activity_events(activity));
        self.terminal_status = Some(status);
        self.terminal_seen = true;
        Ok(events)
    }

    fn validate_conversation(&self, value: &Value) -> Result<(), RuntimeFailure> {
        let observed = bounded_identity(required_text(value, "conversation_id")?)?;
        if Some(observed) == self.conversation_id.as_deref() {
            Ok(())
        } else {
            Err(malformed_stream())
        }
    }

    fn event(&mut self, kind: RuntimeEventKind) -> RuntimeEvent {
        let sequence = self.sequence;
        self.sequence = self.sequence.saturating_add(1);
        RuntimeEvent::new(sequence, kind)
    }

    fn event_with(&mut self, kind: RuntimeEventKind, content: OperationContent) -> RuntimeEvent {
        let sequence = self.sequence;
        self.sequence = self.sequence.saturating_add(1);
        RuntimeEvent::with_content(sequence, kind, content)
    }

    fn activity_events(
        &mut self,
        observations: impl IntoIterator<Item = ActivityObservation>,
    ) -> Vec<RuntimeEvent> {
        observations
            .into_iter()
            .map(|observation| self.event(RuntimeEventKind::Activity(observation)))
            .collect()
    }
}

#[cfg(test)]
mod failure_classification_tests {
    use super::*;
    use swallowtail_runtime::ProcessExit;

    #[test]
    fn opaque_process_exit_remains_harness_unknown() {
        let outcome = ParsedTerminal::new(None, Some(TerminalStatus::Completed), true, None)
            .outcome(ProcessExit::new(false, Some(7)));
        let classification = outcome
            .failure()
            .expect("process exit fails")
            .diagnostic()
            .failure_classification();

        assert_eq!(
            classification.origin(),
            swallowtail_core::FailureOrigin::Harness
        );
        assert_eq!(
            classification.kind(),
            swallowtail_core::FailureKind::Unknown
        );
    }
}
