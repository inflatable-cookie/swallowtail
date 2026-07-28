#[path = "headless_events/terminal.rs"]
mod terminal;

use crate::failure::failure;
use serde_json::Value;
use swallowtail_runtime::{OperationContent, RuntimeEvent, RuntimeEventKind, RuntimeFailure};
use terminal::ParsedTerminal;

const MAXIMUM_LINE_BYTES: usize = 1024 * 1024;
const MAXIMUM_EVENT_COUNT: usize = 4096;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

pub(crate) struct KimiHeadlessEventParser {
    pending: Vec<u8>,
    sequence: u64,
    event_count: usize,
    output: String,
    final_output: Option<OperationContent>,
    terminal_seen: bool,
}

impl KimiHeadlessEventParser {
    pub(crate) const fn new() -> Self {
        Self {
            pending: Vec::new(),
            sequence: 1,
            event_count: 0,
            output: String::new(),
            final_output: None,
            terminal_seen: false,
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
            _ => Err(malformed_stream()),
        }
    }

    fn parse_assistant(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let content = payload.get("content").and_then(Value::as_str);
        let tools = payload
            .get("tool_calls")
            .map(validate_tool_calls)
            .transpose()?;
        if content.is_none() && tools != Some(true) {
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
        if tools == Some(true) {
            events.push(self.event(RuntimeEventKind::Progress));
        }
        Ok(events)
    }

    fn parse_tool(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if !non_empty_string(payload, "tool_call_id") || !non_empty_string(payload, "content") {
            return Err(malformed_stream());
        }
        Ok(vec![self.event(RuntimeEventKind::Progress)])
    }

    fn parse_meta(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        match payload.get("type").and_then(Value::as_str) {
            Some("turn.step.retrying") => {
                for key in ["failed_attempt", "next_attempt", "max_attempts", "delay_ms"] {
                    if payload.get(key).and_then(Value::as_u64).is_none() {
                        return Err(malformed_stream());
                    }
                }
                if !non_empty_string(payload, "error_name")
                    || !non_empty_string(payload, "error_message")
                    || payload
                        .get("status_code")
                        .is_some_and(|value| !value.is_u64())
                {
                    return Err(malformed_stream());
                }
                Ok(vec![self.event(RuntimeEventKind::Progress)])
            }
            Some("session.resume_hint") => {
                if !non_empty_string(payload, "session_id")
                    || !non_empty_string(payload, "command")
                    || !non_empty_string(payload, "content")
                {
                    return Err(malformed_stream());
                }
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
            _ => Err(malformed_stream()),
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
}

fn validate_tool_calls(value: &Value) -> Result<bool, RuntimeFailure> {
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
        Ok(true)
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
