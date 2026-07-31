use crate::failure::failure;
use crate::headless_activity::CursorHeadlessActivityProjection;
use serde_json::Value;
use swallowtail_core::{ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityObservation, ActivityOperationId, CleanupOutcome, OperationContent, ProcessExit,
    ProviderObservation, RuntimeEvent, RuntimeEventKind, RuntimeFailure, TerminalOutcome,
    TerminalStatus, TokenUsage,
};

const MAXIMUM_LINE_BYTES: usize = 1024 * 1024;
const MAXIMUM_EVENT_COUNT: usize = 4096;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

pub(crate) struct CursorHeadlessEventParser {
    pending: Vec<u8>,
    sequence: u64,
    event_count: usize,
    session_id: Option<String>,
    model: ModelId,
    assistant_output: String,
    final_output: Option<OperationContent>,
    terminal_seen: bool,
    activity: CursorHeadlessActivityProjection,
}

impl CursorHeadlessEventParser {
    pub(crate) fn new(operation_id: ActivityOperationId, model: ModelId) -> Self {
        Self {
            pending: Vec::new(),
            sequence: 1,
            event_count: 0,
            session_id: None,
            model,
            assistant_output: String::new(),
            final_output: None,
            terminal_seen: false,
            activity: CursorHeadlessActivityProjection::new(operation_id),
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
        self.event_count = self.event_count.saturating_add(1);
        if self.event_count > MAXIMUM_EVENT_COUNT || self.terminal_seen {
            return Err(stream_limit());
        }
        let payload: Value = serde_json::from_slice(line).map_err(|_| malformed_stream())?;
        let event_type = payload
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match event_type {
            "system" => self.system(&payload),
            "user" => {
                self.validate_session(&payload)?;
                Ok(Vec::new())
            }
            "assistant" => self.assistant(&payload),
            "thinking" => self.thinking(&payload),
            "tool_call" => self.tool_call(&payload),
            "result" => self.result(&payload),
            other => {
                self.validate_session(&payload)?;
                let observations = self.activity.unknown(other)?;
                Ok(self.activity_events(observations))
            }
        }
    }

    fn system(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if self.session_id.is_some()
            || payload.get("subtype").and_then(Value::as_str) != Some("init")
            || payload.get("model").and_then(Value::as_str) != Some(self.model.as_str())
            || payload.get("permissionMode").and_then(Value::as_str) != Some("default")
        {
            return Err(malformed_stream());
        }
        self.session_id = Some(session_id(payload)?.to_owned());
        Ok(Vec::new())
    }

    fn assistant(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.validate_session(payload)?;
        let text = text_content(payload)?;
        if self.assistant_output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(stream_limit());
        }
        self.assistant_output.push_str(&text);
        let mut events = if text.is_empty() {
            Vec::new()
        } else {
            vec![self.event_with(
                RuntimeEventKind::OutputDelta,
                OperationContent::new(&text).map_err(|_| malformed_stream())?,
            )]
        };
        let observations = self.activity.assistant(&text)?;
        events.extend(self.activity_events(observations));
        Ok(events)
    }

    fn thinking(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.validate_session(payload)?;
        let subtype = payload
            .get("subtype")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        let observations = match subtype {
            "delta" => {
                let text = payload
                    .get("text")
                    .and_then(Value::as_str)
                    .ok_or_else(malformed_stream)?;
                let content = OperationContent::new(text).map_err(|_| malformed_stream())?;
                let mut events =
                    vec![self.event_with(RuntimeEventKind::ReasoningProgress, content)];
                let observations = self.activity.thought_delta(text)?;
                events.extend(self.activity_events(observations));
                return Ok(events);
            }
            "completed" => self.activity.thought_completed()?,
            _ => return Err(malformed_stream()),
        };
        Ok(self.activity_events(observations))
    }

    fn tool_call(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.validate_session(payload)?;
        let subtype = payload
            .get("subtype")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        let call_id = payload
            .get("call_id")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(malformed_stream)?;
        let observations = match subtype {
            "started" => self.activity.tool_started(call_id, tool_case(payload))?,
            "completed" => self.activity.tool_completed(call_id)?,
            _ => return Err(malformed_stream()),
        };
        Ok(self.activity_events(observations))
    }

    fn result(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.validate_session(payload)?;
        if payload.get("subtype").and_then(Value::as_str) != Some("success")
            || payload.get("is_error").and_then(Value::as_bool) != Some(false)
            || payload.get("duration_ms").and_then(Value::as_u64).is_none()
            || payload
                .get("duration_api_ms")
                .and_then(Value::as_u64)
                .is_none()
            || payload
                .get("request_id")
                .and_then(Value::as_str)
                .is_none_or(str::is_empty)
        {
            return Err(malformed_stream());
        }
        let result = payload
            .get("result")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        if !self.assistant_output.is_empty() && self.assistant_output != result {
            return Err(malformed_stream());
        }
        let output = OperationContent::new(result).map_err(|_| malformed_stream())?;
        self.final_output = Some(output.clone());
        self.terminal_seen = true;
        let mut events = vec![self.event_with(RuntimeEventKind::OutputAvailable, output)];
        if let Some(usage) = usage(payload)? {
            events.push(self.event(RuntimeEventKind::ProviderObservation(
                ProviderObservation::Usage(usage),
            )));
        }
        let observations = self.activity.complete(&TerminalStatus::Completed)?;
        events.extend(self.activity_events(observations));
        Ok(events)
    }

    fn validate_session(&self, payload: &Value) -> Result<(), RuntimeFailure> {
        if session_id(payload)? == self.session_id.as_deref().ok_or_else(malformed_stream)? {
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

pub(crate) struct ParsedTerminal {
    final_output: Option<OperationContent>,
    terminal_seen: bool,
}

impl ParsedTerminal {
    const fn new(final_output: Option<OperationContent>, terminal_seen: bool) -> Self {
        Self {
            final_output,
            terminal_seen,
        }
    }

    pub(crate) fn outcome(self, exit: ProcessExit) -> TerminalOutcome {
        let status = if !exit.success() {
            TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                "swallowtail.cursor.headless.process_failed",
                match exit.code() {
                    Some(code) => format!("Cursor headless process exited with status {code}"),
                    None => "Cursor headless process exited unsuccessfully".to_owned(),
                },
            ))
        } else if !self.terminal_seen {
            TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                "swallowtail.cursor.headless.incomplete_stream",
                "Cursor headless stream ended without a terminal result",
            ))
        } else {
            TerminalStatus::Completed
        };
        let outcome = TerminalOutcome::new(status, CleanupOutcome::Clean);
        match self.final_output {
            Some(output) => outcome.with_output(output),
            None => outcome,
        }
    }
}

fn text_content(payload: &Value) -> Result<String, RuntimeFailure> {
    payload
        .pointer("/message/content")
        .and_then(Value::as_array)
        .ok_or_else(malformed_stream)?
        .iter()
        .filter(|block| block.get("type").and_then(Value::as_str) == Some("text"))
        .map(|block| {
            block
                .get("text")
                .and_then(Value::as_str)
                .ok_or_else(malformed_stream)
        })
        .collect()
}

fn tool_case(payload: &Value) -> Option<&str> {
    let tool = payload.get("tool_call")?;
    tool.get("tool")
        .and_then(|value| value.get("case"))
        .and_then(Value::as_str)
        .or_else(|| tool.as_object()?.keys().next().map(String::as_str))
}

fn usage(payload: &Value) -> Result<Option<TokenUsage>, RuntimeFailure> {
    let Some(usage) = payload.get("usage") else {
        return Ok(None);
    };
    let input = usage
        .get("inputTokens")
        .and_then(Value::as_u64)
        .ok_or_else(malformed_stream)?;
    let output = usage
        .get("outputTokens")
        .and_then(Value::as_u64)
        .ok_or_else(malformed_stream)?;
    let cache_read = optional_u64(usage, "cacheReadTokens")?;
    let cache_write = optional_u64(usage, "cacheWriteTokens")?;
    Ok(Some(
        TokenUsage::new(Some(input), Some(output)).with_cache_tokens(cache_read, cache_write),
    ))
}

fn optional_u64(value: &Value, key: &str) -> Result<Option<u64>, RuntimeFailure> {
    value
        .get(key)
        .map(|value| value.as_u64().ok_or_else(malformed_stream))
        .transpose()
}

fn session_id(payload: &Value) -> Result<&str, RuntimeFailure> {
    payload
        .get("session_id")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(malformed_stream)
}

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn malformed_stream() -> RuntimeFailure {
    failure(
        "swallowtail.cursor.headless.malformed_stream",
        "Cursor Agent emitted malformed headless stream output",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.cursor.headless.stream_limit",
        "Cursor Agent exceeded the bounded headless stream limit",
    )
}
