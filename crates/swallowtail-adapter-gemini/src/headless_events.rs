use crate::failure::failure;
use crate::headless_activity::GeminiHeadlessActivityProjection;
#[path = "headless_events/terminal.rs"]
mod terminal;
#[path = "headless_events/usage.rs"]
mod usage;
use serde_json::Value;
use swallowtail_core::{ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityObservation, ActivityOperationId, ActivityStatus, OperationContent,
    ProviderObservation, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
};
use terminal::ParsedTerminal;
use usage::token_usage;

const MAXIMUM_LINE_BYTES: usize = 1024 * 1024;
const MAXIMUM_EVENT_COUNT: usize = 4096;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

pub(crate) struct GeminiHeadlessEventParser {
    model: ModelId,
    session_id: String,
    pending: Vec<u8>,
    sequence: u64,
    event_count: usize,
    output: String,
    final_output: Option<OperationContent>,
    provider_failure: Option<SafeDiagnostic>,
    init_seen: bool,
    terminal_seen: bool,
    activity: GeminiHeadlessActivityProjection,
}

impl GeminiHeadlessEventParser {
    pub(crate) fn new(
        model: ModelId,
        session_id: String,
        operation_id: ActivityOperationId,
    ) -> Self {
        Self {
            model,
            session_id,
            pending: Vec::new(),
            sequence: 1,
            event_count: 0,
            output: String::new(),
            final_output: None,
            provider_failure: None,
            init_seen: false,
            terminal_seen: false,
            activity: GeminiHeadlessActivityProjection::new(operation_id),
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
        let event_type = payload
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match event_type {
            "init" => self.parse_init(&payload),
            "message" => self.parse_message(&payload),
            "tool_use" => self.parse_tool_use(&payload),
            "tool_result" => self.parse_tool_result(&payload),
            "error" => self.parse_error(&payload),
            "result" => self.parse_result(&payload),
            _ => {
                self.require_init()?;
                let activity = self.activity.unknown(event_type)?;
                Ok(self.activity_events(activity))
            }
        }
    }

    fn parse_init(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if self.init_seen
            || payload.get("session_id").and_then(Value::as_str) != Some(&self.session_id)
            || payload.get("model").and_then(Value::as_str) != Some(self.model.as_str())
        {
            return Err(malformed_stream());
        }
        self.init_seen = true;
        Ok(Vec::new())
    }

    fn parse_message(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.require_init()?;
        let role = payload
            .get("role")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        let content = payload
            .get("content")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        if !matches!(role, "user" | "assistant")
            || payload
                .get("delta")
                .is_some_and(|value| !value.is_boolean())
        {
            return Err(malformed_stream());
        }
        if role == "user" || content.is_empty() {
            return Ok(Vec::new());
        }
        if self.output.len().saturating_add(content.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(stream_limit());
        }
        self.output.push_str(content);
        let content = OperationContent::new(content).map_err(|_| malformed_stream())?;
        let activity = self.activity.assistant_delta(content.as_str())?;
        let mut events = vec![self.event_with(RuntimeEventKind::OutputDelta, content)];
        events.extend(self.activity_events(activity));
        Ok(events)
    }

    fn parse_tool_use(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.require_init()?;
        let name = payload
            .get("tool_name")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(malformed_stream)?;
        let tool_id = payload
            .get("tool_id")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(malformed_stream)?;
        if !payload.get("parameters").is_some_and(Value::is_object) {
            return Err(malformed_stream());
        }
        let activity = self.activity.tool_use(tool_id, name)?;
        Ok(self.activity_events(activity))
    }

    fn parse_tool_result(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.require_init()?;
        let status = payload
            .get("status")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        let tool_id = payload
            .get("tool_id")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(malformed_stream)?;
        if !matches!(status, "success" | "error") {
            return Err(malformed_stream());
        }
        let activity = self.activity.tool_result(tool_id, status == "error")?;
        Ok(self.activity_events(activity))
    }

    fn parse_error(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.require_init()?;
        let severity = payload
            .get("severity")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        if !matches!(severity, "warning" | "error") || !non_empty_string(payload, "message") {
            return Err(malformed_stream());
        }
        let activity = self.activity.warning()?;
        Ok(self.activity_events(activity))
    }

    fn parse_result(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.require_init()?;
        let status = payload
            .get("status")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        if !matches!(status, "success" | "error") {
            return Err(malformed_stream());
        }
        let usage = token_usage(payload).ok_or_else(malformed_stream)?;
        if status == "error" {
            let error = payload.get("error").ok_or_else(malformed_stream)?;
            if !non_empty_string(error, "type") || !non_empty_string(error, "message") {
                return Err(malformed_stream());
            }
            self.provider_failure = Some(
                SafeDiagnostic::new(
                    "swallowtail.gemini.headless.provider_failed",
                    "Gemini headless reported a provider execution failure",
                )
                .with_failure_classification(
                    swallowtail_core::FailureClassification::new(
                        swallowtail_core::FailureOrigin::Provider,
                        swallowtail_core::FailureKind::Unknown,
                        swallowtail_core::FailureRecovery::Unknown,
                    ),
                ),
            );
        }
        let activity = self.activity.complete(if status == "error" {
            ActivityStatus::Failed
        } else {
            ActivityStatus::Completed
        })?;
        self.terminal_seen = true;
        let mut events = self.activity_events(activity);
        if !self.output.is_empty() {
            let output = OperationContent::new(std::mem::take(&mut self.output))
                .map_err(|_| malformed_stream())?;
            self.final_output = Some(output.clone());
            events.push(self.event_with(RuntimeEventKind::OutputAvailable, output));
        }
        events.push(self.event(RuntimeEventKind::ProviderObservation(
            ProviderObservation::Usage(usage),
        )));
        Ok(events)
    }

    fn require_init(&self) -> Result<(), RuntimeFailure> {
        if self.init_seen {
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
        "swallowtail.gemini.headless.malformed_stream",
        "Gemini CLI emitted malformed stream-json output",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.headless.stream_limit",
        "Gemini CLI exceeded the bounded stream-json limit",
    )
}
