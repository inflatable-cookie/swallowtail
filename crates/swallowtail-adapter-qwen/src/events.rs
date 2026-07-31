use crate::activity::QwenActivityProjection;
use crate::validation::failure;
mod terminal;
mod value;

use self::terminal::ParsedTerminal;
use self::value::{session_id, token_usage};
use serde_json::Value;
use swallowtail_core::{InterfaceVersion, ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityObservation, ActivityOperationId, OperationContent, ProviderObservation, RuntimeEvent,
    RuntimeEventKind, RuntimeFailure,
};

const MAXIMUM_LINE_BYTES: usize = 1024 * 1024;
const MAXIMUM_EVENT_COUNT: usize = 4096;

pub(crate) struct QwenEventParser {
    model: ModelId,
    expected_version: InterfaceVersion,
    expected_session_id: Option<String>,
    pending: Vec<u8>,
    sequence: u64,
    event_count: usize,
    session_id: Option<String>,
    assistant_output: Option<OperationContent>,
    final_output: Option<OperationContent>,
    provider_failure: Option<SafeDiagnostic>,
    terminal_seen: bool,
    activity: QwenActivityProjection,
}

impl QwenEventParser {
    pub(crate) fn with_expected_session(
        model: ModelId,
        expected_version: InterfaceVersion,
        expected_session_id: Option<String>,
        operation_id: ActivityOperationId,
    ) -> Self {
        Self {
            model,
            expected_version,
            expected_session_id,
            pending: Vec::new(),
            sequence: 1,
            event_count: 0,
            session_id: None,
            assistant_output: None,
            final_output: None,
            provider_failure: None,
            terminal_seen: false,
            activity: QwenActivityProjection::new(operation_id),
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

    pub(crate) fn finish(
        mut self,
    ) -> Result<(Vec<RuntimeEvent>, ParsedTerminal, Option<String>), RuntimeFailure> {
        let mut events = Vec::new();
        if !self.pending.is_empty() {
            let line = std::mem::take(&mut self.pending);
            events.extend(self.parse_line(&line)?);
        }
        Ok((
            events,
            ParsedTerminal::new(self.final_output, self.provider_failure, self.terminal_seen),
            self.session_id,
        ))
    }

    fn parse_line(&mut self, line: &[u8]) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if line.iter().all(u8::is_ascii_whitespace) {
            return Ok(Vec::new());
        }
        self.event_count += 1;
        if self.event_count > MAXIMUM_EVENT_COUNT {
            return Err(stream_limit());
        }
        if self.terminal_seen {
            return Err(malformed_stream());
        }

        let payload: Value = serde_json::from_slice(line).map_err(|_| malformed_stream())?;
        let event_type = payload
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match event_type {
            "system" => self.parse_system(&payload),
            "stream_event" => self.parse_partial(&payload),
            "assistant" => self.parse_assistant(&payload),
            "result" => self.parse_result(&payload),
            _ => {
                self.validate_session(&payload)?;
                let provider_ref = payload.get("uuid").and_then(Value::as_str);
                let activity = self.activity.unknown(event_type, provider_ref)?;
                Ok(self.activity_events(activity))
            }
        }
    }

    fn parse_system(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let subtype = payload
            .get("subtype")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        if subtype == "session_start" {
            if self.session_id.is_some()
                || payload.get("model").and_then(Value::as_str) != Some(self.model.as_str())
                || payload.get("permission_mode").and_then(Value::as_str) != Some("default")
                || payload.get("qwen_code_version").and_then(Value::as_str)
                    != Some(self.expected_version.as_str())
            {
                return Err(malformed_stream());
            }
            let session_id = session_id(payload)?.to_owned();
            if self
                .expected_session_id
                .as_deref()
                .is_some_and(|expected| expected != session_id)
            {
                return Err(malformed_stream());
            }
            self.session_id = Some(session_id);
            Ok(Vec::new())
        } else {
            self.validate_session(payload)?;
            let provider_ref = payload.get("uuid").and_then(Value::as_str);
            let activity = self
                .activity
                .unknown(&format!("system.{subtype}"), provider_ref)?;
            Ok(self.activity_events(activity))
        }
    }

    fn parse_partial(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.validate_session(payload)?;
        let event = payload.get("event").ok_or_else(malformed_stream)?;
        let event_type = event
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match event_type {
            "message_start" => {
                let message = event.get("message").ok_or_else(malformed_stream)?;
                let activity = self.activity.message_started(message)?;
                Ok(self.activity_events(activity))
            }
            "content_block_start" => {
                let index = event
                    .get("index")
                    .and_then(Value::as_u64)
                    .ok_or_else(malformed_stream)?;
                let block = event.get("content_block").ok_or_else(malformed_stream)?;
                let activity = self.activity.block_started(index, block)?;
                Ok(self.activity_events(activity))
            }
            "content_block_delta" => {
                let index = event
                    .get("index")
                    .and_then(Value::as_u64)
                    .ok_or_else(malformed_stream)?;
                let delta = event.get("delta").ok_or_else(malformed_stream)?;
                let activity = self.activity.block_updated(index, delta)?;
                let mut events = Vec::new();
                if delta.get("type").and_then(Value::as_str) == Some("text_delta") {
                    let text = delta
                        .get("text")
                        .and_then(Value::as_str)
                        .ok_or_else(malformed_stream)?;
                    let content = OperationContent::new(text).map_err(|_| malformed_stream())?;
                    events.push(self.event_with(RuntimeEventKind::Progress, content));
                }
                events.extend(self.activity_events(activity));
                Ok(events)
            }
            "content_block_stop" => {
                let index = event
                    .get("index")
                    .and_then(Value::as_u64)
                    .ok_or_else(malformed_stream)?;
                let activity = self.activity.block_completed(index)?;
                Ok(self.activity_events(activity))
            }
            "message_stop" => {
                let activity = self.activity.message_completed()?;
                Ok(self.activity_events(activity))
            }
            other => {
                let provider_ref = payload.get("uuid").and_then(Value::as_str);
                let activity = self
                    .activity
                    .unknown(&format!("stream-event.{other}"), provider_ref)?;
                Ok(self.activity_events(activity))
            }
        }
    }

    fn parse_assistant(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.validate_session(payload)?;
        if payload.pointer("/message/model").and_then(Value::as_str) != Some(self.model.as_str()) {
            return Err(malformed_stream());
        }
        let blocks = payload
            .pointer("/message/content")
            .and_then(Value::as_array)
            .ok_or_else(malformed_stream)?;
        let text = blocks
            .iter()
            .filter(|block| block.get("type").and_then(Value::as_str) == Some("text"))
            .map(|block| {
                block
                    .get("text")
                    .and_then(Value::as_str)
                    .ok_or_else(malformed_stream)
            })
            .collect::<Result<String, _>>()?;
        if !text.is_empty() {
            self.assistant_output =
                Some(OperationContent::new(text).map_err(|_| malformed_stream())?);
        }
        let activity = self
            .activity
            .completed_assistant(payload.get("message").ok_or_else(malformed_stream)?)?;
        Ok(self.activity_events(activity))
    }

    fn parse_result(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.validate_session(payload)?;
        self.activity.ensure_idle()?;
        self.terminal_seen = true;
        let subtype = payload
            .get("subtype")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        let is_error = payload
            .get("is_error")
            .and_then(Value::as_bool)
            .ok_or_else(malformed_stream)?;
        let usage = token_usage(payload).ok_or_else(malformed_stream)?;

        match (subtype, is_error) {
            ("success", false) => {
                let result = payload
                    .get("result")
                    .and_then(Value::as_str)
                    .ok_or_else(malformed_stream)?;
                if result.is_empty() {
                    if self.assistant_output.is_some() {
                        return Err(malformed_stream());
                    }
                    Ok(vec![self.event(RuntimeEventKind::ProviderObservation(
                        ProviderObservation::Usage(usage),
                    ))])
                } else {
                    let content = OperationContent::new(result).map_err(|_| malformed_stream())?;
                    if self
                        .assistant_output
                        .as_ref()
                        .is_some_and(|assistant| assistant != &content)
                    {
                        return Err(malformed_stream());
                    }
                    self.final_output = Some(content.clone());
                    Ok(vec![
                        self.event_with(RuntimeEventKind::OutputAvailable, content),
                        self.event(RuntimeEventKind::ProviderObservation(
                            ProviderObservation::Usage(usage),
                        )),
                    ])
                }
            }
            ("error_max_turns" | "error_during_execution", true) => {
                self.provider_failure = Some(SafeDiagnostic::new(
                    "swallowtail.qwen.headless.provider_failed",
                    "Qwen Code reported a provider execution failure",
                ));
                Ok(vec![self.event(RuntimeEventKind::ProviderObservation(
                    ProviderObservation::Usage(usage),
                ))])
            }
            _ => Err(malformed_stream()),
        }
    }

    fn validate_session(&self, payload: &Value) -> Result<(), RuntimeFailure> {
        let expected = self.session_id.as_deref().ok_or_else(malformed_stream)?;
        if session_id(payload)? == expected {
            Ok(())
        } else {
            Err(malformed_stream())
        }
    }

    fn event(&mut self, kind: RuntimeEventKind) -> RuntimeEvent {
        let sequence = self.sequence;
        self.sequence += 1;
        RuntimeEvent::new(sequence, kind)
    }

    fn event_with(&mut self, kind: RuntimeEventKind, content: OperationContent) -> RuntimeEvent {
        let sequence = self.sequence;
        self.sequence += 1;
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

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn malformed_stream() -> RuntimeFailure {
    failure(
        "swallowtail.qwen.headless.malformed_stream",
        "Qwen Code emitted malformed stream output",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.qwen.headless.stream_limit",
        "Qwen Code exceeded the bounded stream-output limit",
    )
}
