#[path = "headless_events/terminal.rs"]
mod terminal;

use crate::failure::failure;
use crate::headless_activity::KimiHeadlessActivityProjection;
use crate::selection::KimiHeadlessBehavior;
use serde_json::Value;
use swallowtail_core::InterfaceVersion;
use swallowtail_runtime::{
    ActivityObservation, ActivityOperationId, OperationContent, RuntimeEvent, RuntimeEventKind,
    RuntimeFailure,
};
use terminal::ParsedTerminal;

const MAXIMUM_LINE_BYTES: usize = 1024 * 1024;
const MAXIMUM_EVENT_COUNT: usize = 4096;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

pub(crate) struct KimiHeadlessEventParser {
    stream_behavior: KimiHeadlessBehavior,
    expected_version: Option<String>,
    version_preamble_seen: bool,
    pending: Vec<u8>,
    sequence: u64,
    event_count: usize,
    output: String,
    final_output: Option<OperationContent>,
    terminal_seen: bool,
    last_retry: Option<(u64, u64, u64)>,
    activity: KimiHeadlessActivityProjection,
}

impl KimiHeadlessEventParser {
    pub(crate) fn new(
        operation_id: ActivityOperationId,
        stream_behavior: KimiHeadlessBehavior,
        executable_version: &InterfaceVersion,
    ) -> Self {
        let expected_version = match stream_behavior {
            KimiHeadlessBehavior::StreamJsonV2 => Some(executable_version.as_str().to_owned()),
            KimiHeadlessBehavior::StreamJsonV1 => None,
        };
        Self {
            stream_behavior,
            expected_version,
            version_preamble_seen: false,
            pending: Vec::new(),
            sequence: 1,
            event_count: 0,
            output: String::new(),
            final_output: None,
            terminal_seen: false,
            last_retry: None,
            activity: KimiHeadlessActivityProjection::new(operation_id),
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
        if self.stream_behavior == KimiHeadlessBehavior::StreamJsonV2 && !self.version_preamble_seen
        {
            return Err(malformed_stream());
        }
        let mut events = Vec::new();
        if !self.pending.is_empty() {
            let line = std::mem::take(&mut self.pending);
            events.extend(self.parse_line(&line)?);
        }
        Ok((
            events,
            ParsedTerminal::new(self.final_output, self.terminal_seen),
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
        match payload.get("role").and_then(Value::as_str) {
            Some("assistant") => self.parse_assistant(&payload),
            Some("tool") => self.parse_tool(&payload),
            Some("meta") => self.parse_meta(&payload),
            Some(role) if !role.trim().is_empty() => {
                self.ensure_turn_output_allowed()?;
                let activity = self.activity.unknown(&format!("role.{role}"))?;
                Ok(self.activity_events(activity))
            }
            _ => Err(malformed_stream()),
        }
    }

    fn parse_assistant(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.ensure_turn_output_allowed()?;
        let content = payload.get("content").and_then(Value::as_str);
        let tools = payload
            .get("tool_calls")
            .map(validate_tool_calls)
            .transpose()?
            .unwrap_or_default();
        if content.is_none() && tools.is_empty() {
            return Err(malformed_stream());
        }
        let mut events = Vec::new();
        if let Some(content) = content.filter(|content| !content.is_empty()) {
            if self.output.len().saturating_add(content.len()) > MAXIMUM_OUTPUT_BYTES {
                return Err(stream_limit());
            }
            self.output.push_str(content);
            events.push(self.event_with(
                RuntimeEventKind::OutputDelta,
                OperationContent::new(content).map_err(|_| malformed_stream())?,
            ));
        }
        let activity = self.activity.assistant(content, &tools)?;
        events.extend(self.activity_events(activity));
        Ok(events)
    }

    fn parse_tool(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.ensure_turn_output_allowed()?;
        if !non_empty_string(payload, "tool_call_id") || !non_empty_string(payload, "content") {
            return Err(malformed_stream());
        }
        let tool_id = payload["tool_call_id"].as_str().expect("validated");
        let activity = self.activity.tool_result(tool_id)?;
        Ok(self.activity_events(activity))
    }

    fn parse_meta(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        match payload.get("type").and_then(Value::as_str) {
            Some("turn.step.retrying") => {
                self.ensure_turn_output_allowed()?;
                for key in ["failed_attempt", "next_attempt", "max_attempts", "delay_ms"] {
                    if payload.get(key).and_then(Value::as_u64).is_none() {
                        return Err(malformed_stream());
                    }
                }
                let failed = payload["failed_attempt"].as_u64().expect("validated");
                let next = payload["next_attempt"].as_u64().expect("validated");
                let maximum = payload["max_attempts"].as_u64().expect("validated");
                if failed == 0
                    || next != failed.saturating_add(1)
                    || next > maximum
                    || self
                        .last_retry
                        .is_some_and(|(_, previous_next, previous_maximum)| {
                            failed != previous_next || maximum != previous_maximum
                        })
                {
                    return Err(malformed_stream());
                }
                if !non_empty_string(payload, "error_name")
                    || !non_empty_string(payload, "error_message")
                    || payload
                        .get("status_code")
                        .is_some_and(|value| !value.is_u64())
                {
                    return Err(malformed_stream());
                }
                self.last_retry = Some((failed, next, maximum));
                let activity = self.activity.retry()?;
                Ok(self.activity_events(activity))
            }
            Some("session.resume_hint") => {
                self.ensure_turn_output_allowed()?;
                if !non_empty_string(payload, "session_id")
                    || !non_empty_string(payload, "command")
                    || !non_empty_string(payload, "content")
                {
                    return Err(malformed_stream());
                }
                self.activity.ensure_idle()?;
                self.terminal_seen = true;
                let mut events = Vec::new();
                if !self.output.is_empty() {
                    let output = OperationContent::new(std::mem::take(&mut self.output))
                        .map_err(|_| malformed_stream())?;
                    self.final_output = Some(output.clone());
                    events.push(self.event_with(RuntimeEventKind::OutputAvailable, output));
                }
                Ok(events)
            }
            Some("system.version") => match self.stream_behavior {
                KimiHeadlessBehavior::StreamJsonV1 => Err(malformed_stream()),
                KimiHeadlessBehavior::StreamJsonV2 => {
                    if self.version_preamble_seen {
                        return Err(malformed_stream());
                    }
                    let observed = payload
                        .get("version")
                        .and_then(Value::as_str)
                        .filter(|value| !value.trim().is_empty());
                    if observed != self.expected_version.as_deref() {
                        return Err(malformed_stream());
                    }
                    self.version_preamble_seen = true;
                    Ok(Vec::new())
                }
            },
            Some(event_type) if !event_type.trim().is_empty() => {
                self.ensure_turn_output_allowed()?;
                let activity = self.activity.unknown(event_type)?;
                Ok(self.activity_events(activity))
            }
            _ => Err(malformed_stream()),
        }
    }

    fn ensure_turn_output_allowed(&self) -> Result<(), RuntimeFailure> {
        if self.stream_behavior == KimiHeadlessBehavior::StreamJsonV2 && !self.version_preamble_seen
        {
            Err(malformed_stream())
        } else {
            Ok(())
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

fn validate_tool_calls(value: &Value) -> Result<Vec<(String, String)>, RuntimeFailure> {
    let calls = value.as_array().ok_or_else(malformed_stream)?;
    if calls.is_empty()
        || calls.iter().any(|call| {
            call.get("type").and_then(Value::as_str) != Some("function")
                || !non_empty_string(call, "id")
                || call.get("function").is_none_or(|function| {
                    !non_empty_string(function, "name") || !non_empty_string(function, "arguments")
                })
        })
    {
        Err(malformed_stream())
    } else {
        calls
            .iter()
            .map(|call| {
                Ok((
                    call["id"].as_str().expect("validated").to_owned(),
                    call["function"]["name"]
                        .as_str()
                        .expect("validated")
                        .to_owned(),
                ))
            })
            .collect()
    }
}

fn non_empty_string(payload: &Value, key: &str) -> bool {
    payload
        .get(key)
        .and_then(Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
}

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn malformed_stream() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.headless.malformed_stream",
        "Kimi Code emitted malformed stream-json output",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.headless.stream_limit",
        "Kimi Code exceeded the bounded stream-json limit",
    )
}
