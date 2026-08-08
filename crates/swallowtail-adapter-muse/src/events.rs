use self::terminal::ParsedTerminal;
use self::validation::{
    bounded_identity, causation_mismatch, is_known_payload_type, malformed, model_mismatch,
    output_limit, output_mismatch, post_terminal, record_limit, required_text, run_mismatch,
    sequence_mismatch, session_mismatch, stream_limit, task_mismatch, trim_newline, unknown_limit,
};
use crate::activity::MuseActivityProjection;
use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_core::{ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityObservation, ActivityOperationId, OperationContent, RuntimeEvent, RuntimeEventKind,
    RuntimeFailure, TerminalStatus,
};

mod terminal;
mod validation;

#[cfg(test)]
mod tests;

pub(crate) const MAXIMUM_RECORD_BYTES: usize = 1024 * 1024;
pub(crate) const MAXIMUM_STREAM_BYTES: usize = 8 * 1024 * 1024;
pub(crate) const MAXIMUM_RECORDS: usize = 4096;
const MAXIMUM_UNKNOWN_PAYLOAD_BYTES: usize = 64 * 1024;
const MAXIMUM_OUTPUT_BYTES: usize = 256 * 1024;

pub(crate) struct MuseEventParser {
    pending: Vec<u8>,
    stream_bytes: usize,
    record_count: usize,
    expected_sequence: u64,
    runtime_sequence: u64,
    session_id: Option<String>,
    command_id: Option<String>,
    run_linked: bool,
    run_started: bool,
    model_seen: bool,
    expected_model: ModelId,
    linked_tasks: BTreeSet<String>,
    output: String,
    final_output: Option<OperationContent>,
    terminal: Option<TerminalStatus>,
    terminal_seen: bool,
    activity: MuseActivityProjection,
}

impl MuseEventParser {
    pub(crate) fn new(operation_id: ActivityOperationId, expected_model: ModelId) -> Self {
        Self {
            pending: Vec::new(),
            stream_bytes: 0,
            record_count: 0,
            expected_sequence: 1,
            runtime_sequence: 1,
            session_id: None,
            command_id: None,
            run_linked: false,
            run_started: false,
            model_seen: false,
            expected_model,
            linked_tasks: BTreeSet::new(),
            output: String::new(),
            final_output: None,
            terminal: None,
            terminal_seen: false,
            activity: MuseActivityProjection::new(operation_id),
        }
    }

    pub(crate) fn push(&mut self, bytes: &[u8]) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.stream_bytes = self
            .stream_bytes
            .checked_add(bytes.len())
            .ok_or_else(stream_limit)?;
        if self.stream_bytes > MAXIMUM_STREAM_BYTES {
            return Err(stream_limit());
        }
        self.pending.extend_from_slice(bytes);
        let mut events = Vec::new();
        while let Some(newline) = self.pending.iter().position(|byte| *byte == b'\n') {
            if newline > MAXIMUM_RECORD_BYTES {
                return Err(record_limit());
            }
            let line: Vec<_> = self.pending.drain(..=newline).collect();
            events.extend(self.parse_line(trim_newline(&line))?);
        }
        if self.pending.len() > MAXIMUM_RECORD_BYTES {
            return Err(record_limit());
        }
        Ok(events)
    }

    pub(crate) fn finish(mut self) -> Result<(Vec<RuntimeEvent>, ParsedTerminal), RuntimeFailure> {
        let mut events = Vec::new();
        if !self.pending.is_empty() {
            let line = std::mem::take(&mut self.pending);
            events.extend(self.parse_line(&line)?);
        }
        if !self.run_linked || !self.run_started || !self.model_seen || !self.terminal_seen {
            return Ok((
                events,
                ParsedTerminal::incomplete(self.final_output, self.terminal),
            ));
        }
        Ok((
            events,
            ParsedTerminal::complete(self.final_output, self.terminal),
        ))
    }

    fn parse_line(&mut self, line: &[u8]) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if line.is_empty() || line.iter().all(u8::is_ascii_whitespace) {
            return Err(malformed());
        }
        self.record_count = self.record_count.checked_add(1).ok_or_else(record_limit)?;
        if self.record_count > MAXIMUM_RECORDS {
            return Err(record_limit());
        }
        let record: Value = serde_json::from_slice(line).map_err(|_| malformed())?;
        self.validate_envelope(&record)?;
        let payload_type = required_text(&record, "payload_type")?;
        if self.terminal_seen && is_known_payload_type(payload_type) {
            return Err(post_terminal());
        }
        let payload = record.get("payload").ok_or_else(malformed)?;
        let event_id = bounded_identity(required_text(&record, "id")?)?;

        match payload_type {
            "runtime.command.accepted" => {
                let events = self.command_accepted(payload)?;
                if required_text(&record, "causation_id")?
                    != self.command_id.as_deref().ok_or_else(malformed)?
                {
                    return Err(causation_mismatch());
                }
                Ok(events)
            }
            "session.run.linked" => self.run_linked(payload),
            "run.model.configured" => self.model_configured(payload),
            "turn.input.user" => self.ordinary_run_payload(payload).map(|()| Vec::new()),
            "run.lifecycle.started" => self.run_started(payload),
            "run.output.delta" => self.output_delta(payload),
            "run.terminal.completed" | "run.terminal.failed" | "run.terminal.cancelled" => {
                self.terminal(payload_type, payload)
            }
            "task.stream.linked" => self.task_linked(payload),
            value if value.starts_with("task.lifecycle.") => self.task_lifecycle(value, payload),
            value => self.unknown(event_id, value, payload),
        }
    }

    fn validate_envelope(&mut self, record: &Value) -> Result<(), RuntimeFailure> {
        if record.get("schema_version").and_then(Value::as_u64) != Some(1)
            || record.get("payload_schema_version").and_then(Value::as_u64) != Some(1)
            || !record.get("recorded_at").is_some_and(Value::is_u64)
            || !matches!(
                required_text(record, "record_type")?,
                "reconciliation" | "event" | "status"
            )
            || !matches!(
                required_text(record, "durability")?,
                "durable" | "ephemeral"
            )
        {
            return Err(malformed());
        }
        let sequence = record
            .get("sequence")
            .and_then(Value::as_u64)
            .ok_or_else(malformed)?;
        if sequence != self.expected_sequence {
            return Err(sequence_mismatch());
        }
        self.expected_sequence = self
            .expected_sequence
            .checked_add(1)
            .ok_or_else(sequence_mismatch)?;

        let stream = record.get("stream").ok_or_else(malformed)?;
        if required_text(stream, "kind")? != "session" {
            return Err(session_mismatch());
        }
        let session_id = bounded_identity(required_text(stream, "id")?)?;
        match &self.session_id {
            Some(expected) if expected != session_id => return Err(session_mismatch()),
            None => self.session_id = Some(session_id.to_owned()),
            _ => {}
        }
        let causation = bounded_identity(required_text(record, "causation_id")?)?;
        if let Some(command_id) = &self.command_id
            && command_id != causation
        {
            return Err(causation_mismatch());
        }
        Ok(())
    }

    fn command_accepted(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if self.record_count != 1 || self.command_id.is_some() {
            return Err(malformed());
        }
        let command_id = bounded_identity(required_text(payload, "command_id")?)?;
        if required_text(payload, "command_kind")? != "turn.submit" {
            return Err(malformed());
        }
        self.command_id = Some(command_id.to_owned());
        Ok(Vec::new())
    }

    fn run_linked(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if self.run_linked {
            return Err(malformed());
        }
        self.ordinary_run_payload(payload)?;
        self.run_linked = true;
        Ok(Vec::new())
    }

    fn model_configured(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if !self.run_linked || self.model_seen {
            return Err(malformed());
        }
        self.ordinary_run_payload(payload)?;
        if required_text(payload, "provider_id")? != "meta"
            || required_text(payload, "model_id")? != self.expected_model.as_str()
            || bounded_identity(required_text(payload, "profile_id")?).is_err()
        {
            return Err(model_mismatch());
        }
        self.model_seen = true;
        Ok(Vec::new())
    }

    fn run_started(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if !self.run_linked || self.run_started {
            return Err(malformed());
        }
        self.ordinary_run_payload(payload)?;
        self.run_started = true;
        Ok(Vec::new())
    }

    fn output_delta(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if !self.run_started {
            return Err(malformed());
        }
        self.ordinary_run_payload(payload)?;
        let text = required_text(payload, "text")?;
        if self.output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(output_limit());
        }
        self.output.push_str(text);
        let content = OperationContent::new(text).map_err(|_| malformed())?;
        Ok(vec![self.runtime_event_with(
            RuntimeEventKind::OutputDelta,
            content,
        )])
    }

    fn task_linked(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.ordinary_run_payload(payload)?;
        let task_id = bounded_identity(required_text(payload, "task_id")?)?;
        let stream = payload.get("task_stream").ok_or_else(malformed)?;
        if required_text(stream, "kind")? != "task"
            || required_text(stream, "id")? != task_id
            || !self.linked_tasks.insert(task_id.to_owned())
        {
            return Err(task_mismatch());
        }
        Ok(Vec::new())
    }

    fn task_lifecycle(
        &mut self,
        payload_type: &str,
        payload: &Value,
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.ordinary_run_payload(payload)?;
        let task_id = bounded_identity(required_text(payload, "task_id")?)?;
        if !self.linked_tasks.contains(task_id) {
            return Err(task_mismatch());
        }
        let task_stream = payload.get("task_stream").ok_or_else(malformed)?;
        let event = payload.get("event").ok_or_else(malformed)?;
        let lifecycle = payload_type
            .strip_prefix("task.lifecycle.")
            .ok_or_else(malformed)?;
        if required_text(task_stream, "kind")? != "task"
            || required_text(task_stream, "id")? != task_id
            || required_text(event, "kind")? != lifecycle
            || required_text(event, "task_id")? != task_id
        {
            return Err(task_mismatch());
        }
        let observations = self.activity.task(task_id, lifecycle)?;
        if matches!(
            lifecycle,
            "completed" | "failed" | "cancelled" | "timed_out" | "rejected"
        ) {
            self.linked_tasks.remove(task_id);
        }
        Ok(self.activity_events(observations))
    }

    fn unknown(
        &mut self,
        event_id: &str,
        payload_type: &str,
        payload: &Value,
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let bytes = serde_json::to_vec(payload).map_err(|_| malformed())?;
        if bytes.len() > MAXIMUM_UNKNOWN_PAYLOAD_BYTES {
            return Err(unknown_limit());
        }
        let observation = self.activity.unknown(event_id, payload_type)?;
        Ok(vec![
            self.runtime_event(RuntimeEventKind::Activity(observation)),
        ])
    }

    fn terminal(
        &mut self,
        payload_type: &str,
        payload: &Value,
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if !self.run_started {
            return Err(malformed());
        }
        self.ordinary_run_payload(payload)?;
        let terminal = required_text(payload, "terminal")?;
        let expected_terminal = payload_type
            .strip_prefix("run.terminal.")
            .ok_or_else(malformed)?;
        if terminal != expected_terminal {
            return Err(malformed());
        }
        let status = match terminal {
            "completed" => TerminalStatus::Completed,
            "cancelled" => TerminalStatus::Cancelled,
            "failed" => TerminalStatus::ProviderFailed(
                SafeDiagnostic::new(
                    "swallowtail.muse_code.headless.provider_failed",
                    "Muse Code reported a failed headless run",
                )
                .with_failure_classification(
                    swallowtail_core::FailureClassification::new(
                        swallowtail_core::FailureOrigin::Provider,
                        swallowtail_core::FailureKind::Unknown,
                        swallowtail_core::FailureRecovery::Unknown,
                    ),
                ),
            ),
            _ => return Err(malformed()),
        };
        let text = payload
            .get("text")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if text.len() > MAXIMUM_OUTPUT_BYTES {
            return Err(output_limit());
        }
        let mut events = Vec::new();
        if status == TerminalStatus::Completed {
            if !self.output.is_empty() && self.output != text {
                return Err(output_mismatch());
            }
            let output = OperationContent::new(text).map_err(|_| malformed())?;
            self.final_output = Some(output.clone());
            events.push(self.runtime_event_with(RuntimeEventKind::OutputAvailable, output));
        }
        let observations = self.activity.complete(&status)?;
        events.extend(self.activity_events(observations));
        self.terminal = Some(status);
        self.terminal_seen = true;
        Ok(events)
    }

    fn ordinary_run_payload(&self, payload: &Value) -> Result<(), RuntimeFailure> {
        let command_id = self.command_id.as_deref().ok_or_else(malformed)?;
        if required_text(payload, "command_id")? != command_id {
            return Err(causation_mismatch());
        }
        let stream = payload.get("run_stream").ok_or_else(malformed)?;
        if required_text(stream, "kind")? != "run" || required_text(stream, "id")? != command_id {
            return Err(run_mismatch());
        }
        Ok(())
    }

    fn runtime_event(&mut self, kind: RuntimeEventKind) -> RuntimeEvent {
        let sequence = self.runtime_sequence;
        self.runtime_sequence = self.runtime_sequence.saturating_add(1);
        RuntimeEvent::new(sequence, kind)
    }

    fn runtime_event_with(
        &mut self,
        kind: RuntimeEventKind,
        content: OperationContent,
    ) -> RuntimeEvent {
        let sequence = self.runtime_sequence;
        self.runtime_sequence = self.runtime_sequence.saturating_add(1);
        RuntimeEvent::with_content(sequence, kind, content)
    }

    fn activity_events(
        &mut self,
        observations: impl IntoIterator<Item = ActivityObservation>,
    ) -> Vec<RuntimeEvent> {
        observations
            .into_iter()
            .map(|observation| self.runtime_event(RuntimeEventKind::Activity(observation)))
            .collect()
    }
}
