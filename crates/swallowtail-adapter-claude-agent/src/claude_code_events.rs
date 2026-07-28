#[path = "claude_code_events/terminal.rs"]
mod terminal;
#[path = "claude_code_events/usage.rs"]
mod usage;

use crate::failure::failure;
use serde_json::Value;
use swallowtail_core::{ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    OperationContent, ProviderObservation, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
};
use terminal::ParsedTerminal;
use usage::token_usage;

const MAXIMUM_LINE_BYTES: usize = 1024 * 1024;
const MAXIMUM_EVENT_COUNT: usize = 4096;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

pub(crate) struct ClaudeCodeEventParser {
    model: ModelId,
    session_id: Option<String>,
    pending: Vec<u8>,
    sequence: u64,
    event_count: usize,
    streamed_output_bytes: usize,
    final_output: Option<OperationContent>,
    provider_failure: Option<SafeDiagnostic>,
    init_seen: bool,
    terminal_seen: bool,
}

impl ClaudeCodeEventParser {
    pub(crate) fn new(model: ModelId) -> Self {
        Self {
            model,
            session_id: None,
            pending: Vec::new(),
            sequence: 1,
            event_count: 0,
            streamed_output_bytes: 0,
            final_output: None,
            provider_failure: None,
            init_seen: false,
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
            ParsedTerminal {
                final_output: self.final_output,
                provider_failure: self.provider_failure,
                initialized: self.init_seen,
                terminal_seen: self.terminal_seen,
            },
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
        match payload.get("type").and_then(Value::as_str) {
            Some("system") if payload.get("subtype").and_then(Value::as_str) == Some("init") => {
                self.parse_init(&payload)
            }
            Some("system")
                if !self.init_seen
                    && matches!(
                        payload.get("subtype").and_then(Value::as_str),
                        Some("hook_started" | "hook_response")
                    ) =>
            {
                self.parse_pre_init_hook(&payload)
            }
            Some("assistant") => self.parse_assistant(&payload),
            Some("result") => self.parse_result(&payload),
            Some(_) => {
                self.require_session(&payload)?;
                Ok(vec![self.event(RuntimeEventKind::Progress)])
            }
            None => Err(malformed_stream()),
        }
    }

    fn parse_init(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let session_id = non_empty_string(payload, "session_id").ok_or_else(malformed_stream)?;
        if self.init_seen
            || self
                .session_id
                .as_deref()
                .is_some_and(|observed| observed != session_id)
            || payload.get("model").and_then(Value::as_str) != Some(self.model.as_str())
            || payload.get("permissionMode").and_then(Value::as_str) != Some("plan")
        {
            return Err(malformed_stream());
        }
        self.session_id = Some(session_id.to_owned());
        self.init_seen = true;
        Ok(Vec::new())
    }

    fn parse_pre_init_hook(
        &mut self,
        payload: &Value,
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let session_id = non_empty_string(payload, "session_id").ok_or_else(malformed_stream)?;
        if self
            .session_id
            .as_deref()
            .is_some_and(|observed| observed != session_id)
        {
            return Err(malformed_stream());
        }
        self.session_id = Some(session_id.to_owned());
        Ok(vec![self.event(RuntimeEventKind::Progress)])
    }

    fn parse_assistant(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.require_session(payload)?;
        if let Some(error) = payload.get("error").and_then(Value::as_str)
            && !error.is_empty()
        {
            self.provider_failure = Some(SafeDiagnostic::new(
                "swallowtail.claude_code.headless.provider_failed",
                "Claude Code reported a provider execution failure",
            ));
        }
        let message = payload.get("message").ok_or_else(malformed_stream)?;
        if message.get("model").and_then(Value::as_str) != Some(self.model.as_str()) {
            return Err(malformed_stream());
        }
        let content = message
            .get("content")
            .and_then(Value::as_array)
            .ok_or_else(malformed_stream)?;
        let mut events = Vec::new();
        for block in content {
            if block.get("type").and_then(Value::as_str) == Some("text") {
                let text = block
                    .get("text")
                    .and_then(Value::as_str)
                    .ok_or_else(malformed_stream)?;
                if text.is_empty() {
                    continue;
                }
                self.streamed_output_bytes = self.streamed_output_bytes.saturating_add(text.len());
                if self.streamed_output_bytes > MAXIMUM_OUTPUT_BYTES {
                    return Err(stream_limit());
                }
                let content = OperationContent::new(text).map_err(|_| malformed_stream())?;
                events.push(self.event_with(RuntimeEventKind::OutputDelta, content));
            } else {
                events.push(self.event(RuntimeEventKind::Progress));
            }
        }
        Ok(events)
    }

    fn parse_result(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.require_session(payload)?;
        let subtype = payload
            .get("subtype")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        let is_error = payload
            .get("is_error")
            .and_then(Value::as_bool)
            .ok_or_else(malformed_stream)?;
        let usage = token_usage(payload).ok_or_else(malformed_stream)?;
        if subtype == "success" && !is_error {
            if let Some(result) = payload.get("result").and_then(Value::as_str)
                && !result.is_empty()
            {
                if result.len() > MAXIMUM_OUTPUT_BYTES {
                    return Err(stream_limit());
                }
                self.final_output =
                    Some(OperationContent::new(result).map_err(|_| malformed_stream())?);
            }
        } else {
            self.provider_failure = Some(SafeDiagnostic::new(
                "swallowtail.claude_code.headless.provider_failed",
                "Claude Code reported a provider execution failure",
            ));
        }
        self.terminal_seen = true;
        let mut events = Vec::new();
        if let Some(output) = self.final_output.clone() {
            events.push(self.event_with(RuntimeEventKind::OutputAvailable, output));
        }
        events.push(self.event(RuntimeEventKind::ProviderObservation(
            ProviderObservation::Usage(usage),
        )));
        Ok(events)
    }

    fn require_session(&self, payload: &Value) -> Result<(), RuntimeFailure> {
        if !self.init_seen
            || payload.get("session_id").and_then(Value::as_str) != self.session_id.as_deref()
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
}

fn non_empty_string<'a>(payload: &'a Value, key: &str) -> Option<&'a str> {
    payload
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
}

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn malformed_stream() -> RuntimeFailure {
    failure(
        "swallowtail.claude_code.headless.malformed_stream",
        "Claude Code emitted malformed stream-json output",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.claude_code.headless.stream_limit",
        "Claude Code exceeded the bounded stream-json limits",
    )
}
