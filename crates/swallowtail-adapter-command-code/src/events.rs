use crate::activity::CommandCodeHeadlessActivityProjection;
use crate::failure::failure;
use serde_json::Value;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ActivityObservation, ActivityOperationId, CleanupOutcome, OperationContent, ProcessExit,
    ProviderObservation, RuntimeEvent, RuntimeEventKind, RuntimeFailure, TerminalOutcome,
    TerminalStatus, TokenUsage,
};

const MAXIMUM_LINE_BYTES: usize = 1024 * 1024;
const MAXIMUM_EVENT_COUNT: usize = 4096;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

/// Lifecycle-only event types that are recognized but never projected.
///
/// `run_end` is included here and its `result` field (which may carry the
/// private `nextState`) is never inspected.
const IGNORED_LIFECYCLE_EVENT_TYPES: &[&str] = &[
    "turn_start",
    "turn_end",
    "message_start",
    "message_update",
    "message_end",
    "model_request_start",
    "model_trace",
    "run_end",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ResultSubtype {
    Success,
    Error,
    MaxTurns,
}

pub(crate) struct CommandCodeHeadlessEventParser {
    pending: Vec<u8>,
    sequence: u64,
    event_count: usize,
    expected_session_id: Option<String>,
    session_id: Option<String>,
    run_start_seen: bool,
    assistant_output: String,
    final_output: Option<OperationContent>,
    terminal_seen: bool,
    result_subtype: Option<ResultSubtype>,
    credit_signal: bool,
    activity: CommandCodeHeadlessActivityProjection,
}

impl CommandCodeHeadlessEventParser {
    pub(crate) fn with_expected_session(
        operation_id: ActivityOperationId,
        expected_session_id: Option<String>,
    ) -> Self {
        Self {
            pending: Vec::new(),
            sequence: 1,
            event_count: 0,
            expected_session_id,
            session_id: None,
            run_start_seen: false,
            assistant_output: String::new(),
            final_output: None,
            terminal_seen: false,
            result_subtype: None,
            credit_signal: false,
            activity: CommandCodeHeadlessActivityProjection::new(operation_id),
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
            ParsedTerminal::new(
                self.final_output,
                self.terminal_seen,
                self.result_subtype,
                self.credit_signal,
            ),
            self.session_id,
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
        let record: Value = serde_json::from_slice(line).map_err(|_| malformed_stream())?;
        let record_type = record
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match record_type {
            "event" => {
                let event = record.get("event").ok_or_else(malformed_stream)?;
                let event_type = event
                    .get("type")
                    .and_then(Value::as_str)
                    .ok_or_else(malformed_stream)?;
                self.dispatch_event(event_type, event)
            }
            "result" => self.result(&record),
            _ => Err(malformed_stream()),
        }
    }

    fn dispatch_event(
        &mut self,
        event_type: &str,
        event: &Value,
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        match event_type {
            "run_start" => self.run_start(event),
            "thinking_start" => {
                let observations = self.activity.thought_start()?;
                Ok(self.activity_events(observations))
            }
            "thinking_delta" => self.thinking_delta(event),
            "thinking_end" => {
                let observations = self.activity.thought_end()?;
                Ok(self.activity_events(observations))
            }
            "text_delta" => self.text_delta(event),
            "tool_queued" => self.tool_queued(event),
            "tool_running" => self.tool_running(event),
            "tool_completed" => self.tool_completed(event),
            "model_request_end" => self.model_request_end(event),
            "run_error" => self.run_error(event),
            other if IGNORED_LIFECYCLE_EVENT_TYPES.contains(&other) => Ok(Vec::new()),
            other => {
                let observations = self.activity.unknown(other)?;
                Ok(self.activity_events(observations))
            }
        }
    }

    fn run_start(&mut self, event: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if self.run_start_seen {
            return Err(malformed_stream());
        }
        let session_id = required_text(event, "sessionId")?;
        if self
            .expected_session_id
            .as_deref()
            .is_some_and(|expected| expected != session_id)
        {
            return Err(session_mismatch());
        }
        self.session_id = Some(session_id.to_owned());
        self.run_start_seen = true;
        Ok(Vec::new())
    }

    fn thinking_delta(&mut self, event: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let text = required_string(event, "delta")?.to_owned();
        let content = OperationContent::new(&text).map_err(|_| malformed_stream())?;
        let mut events = vec![self.event_with(RuntimeEventKind::ReasoningProgress, content)];
        let observations = self.activity.thought_delta(&text)?;
        events.extend(self.activity_events(observations));
        Ok(events)
    }

    fn text_delta(&mut self, event: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let text = required_string(event, "delta")?.to_owned();
        if self.assistant_output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(output_limit());
        }
        self.assistant_output.push_str(&text);
        let content = OperationContent::new(&text).map_err(|_| malformed_stream())?;
        let mut events = vec![self.event_with(RuntimeEventKind::OutputDelta, content)];
        let observations = self.activity.text_delta(&text)?;
        events.extend(self.activity_events(observations));
        Ok(events)
    }

    fn tool_queued(&mut self, event: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let tool_call_id = bounded_identifier(required_text(event, "toolCallId")?)?.to_owned();
        let tool_name = event
            .get("toolName")
            .and_then(Value::as_str)
            .map(str::to_owned);
        let observations = self
            .activity
            .tool_queued(&tool_call_id, tool_name.as_deref())?;
        Ok(self.activity_events(observations))
    }

    fn tool_running(&mut self, event: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let tool_call_id = bounded_identifier(required_text(event, "toolCallId")?)?.to_owned();
        let observations = self.activity.tool_running(&tool_call_id)?;
        Ok(self.activity_events(observations))
    }

    fn tool_completed(&mut self, event: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let tool_call_id = bounded_identifier(required_text(event, "toolCallId")?)?.to_owned();
        let observations = self.activity.tool_completed(&tool_call_id)?;
        Ok(self.activity_events(observations))
    }

    fn model_request_end(&mut self, event: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        match usage(event)? {
            Some(usage) => Ok(vec![self.event(RuntimeEventKind::ProviderObservation(
                ProviderObservation::Usage(usage),
            ))]),
            None => Ok(Vec::new()),
        }
    }

    fn run_error(&mut self, event: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if let Some(message) = event.get("message").and_then(Value::as_str)
            && mentions_insufficient_credit(message)
        {
            self.credit_signal = true;
        }
        Ok(Vec::new())
    }

    fn result(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if let Some(session_id) = payload.get("sessionId").and_then(Value::as_str)
            && self
                .session_id
                .as_deref()
                .is_some_and(|expected| expected != session_id)
        {
            return Err(malformed_stream());
        }
        let subtype = required_text(payload, "subtype")?.to_owned();
        let events = match subtype.as_str() {
            "success" => self.result_success(payload)?,
            "error" => {
                self.result_subtype = Some(ResultSubtype::Error);
                if payload
                    .get("error")
                    .and_then(Value::as_str)
                    .is_some_and(mentions_insufficient_credit)
                {
                    self.credit_signal = true;
                }
                let status = TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                    "swallowtail.command_code.headless.result_error",
                    "",
                ));
                let observations = self.activity.complete(&status)?;
                self.activity_events(observations)
            }
            "max_turns" => {
                self.result_subtype = Some(ResultSubtype::MaxTurns);
                let status = TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                    "swallowtail.command_code.headless.max_turns",
                    "",
                ));
                let observations = self.activity.complete(&status)?;
                self.activity_events(observations)
            }
            _ => return Err(malformed_stream()),
        };
        self.terminal_seen = true;
        Ok(events)
    }

    fn result_success(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let final_text = required_string(payload, "finalText")?.to_owned();
        if !self.assistant_output.is_empty() && self.assistant_output != final_text {
            return Err(output_mismatch());
        }
        let output = OperationContent::new(&final_text).map_err(|_| malformed_stream())?;
        self.final_output = Some(output.clone());
        self.result_subtype = Some(ResultSubtype::Success);
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
    subtype: Option<ResultSubtype>,
    credit_signal: bool,
}

impl ParsedTerminal {
    const fn new(
        final_output: Option<OperationContent>,
        terminal_seen: bool,
        subtype: Option<ResultSubtype>,
        credit_signal: bool,
    ) -> Self {
        Self {
            final_output,
            terminal_seen,
            subtype,
            credit_signal,
        }
    }

    pub(crate) fn outcome(self, exit: ProcessExit) -> TerminalOutcome {
        if !exit.success() {
            return TerminalOutcome::new(
                TerminalStatus::ProviderFailed(self.exit_diagnostic(exit)),
                CleanupOutcome::Clean,
            );
        }
        if !self.terminal_seen {
            return TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.command_code.headless.incomplete_stream",
                    "Command Code headless stream ended without a terminal result",
                )),
                CleanupOutcome::Clean,
            );
        }
        match self.subtype {
            Some(ResultSubtype::Success) => {
                let outcome = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean);
                match self.final_output {
                    Some(output) => outcome.with_output(output),
                    None => outcome,
                }
            }
            Some(ResultSubtype::MaxTurns) => TerminalOutcome::new(
                TerminalStatus::ProviderFailed(max_turns_diagnostic()),
                CleanupOutcome::Clean,
            ),
            Some(ResultSubtype::Error) | None => TerminalOutcome::new(
                TerminalStatus::ProviderFailed(self.provider_error_diagnostic()),
                CleanupOutcome::Clean,
            ),
        }
    }

    fn exit_diagnostic(&self, exit: ProcessExit) -> SafeDiagnostic {
        if self.credit_signal || exit.code() == Some(10) {
            return quota_exhausted_diagnostic();
        }
        match self.subtype {
            Some(ResultSubtype::MaxTurns) => max_turns_diagnostic(),
            Some(ResultSubtype::Error) => self.provider_error_diagnostic(),
            Some(ResultSubtype::Success) | None => SafeDiagnostic::new(
                "swallowtail.command_code.headless.process_failed",
                match exit.code() {
                    Some(code) => format!("Command Code process exited with status {code}"),
                    None => "Command Code process exited unsuccessfully".to_owned(),
                },
            )
            .with_failure_classification(swallowtail_core::FailureClassification::new(
                swallowtail_core::FailureOrigin::Harness,
                swallowtail_core::FailureKind::Unknown,
                swallowtail_core::FailureRecovery::Unknown,
            )),
        }
    }

    fn provider_error_diagnostic(&self) -> SafeDiagnostic {
        if self.credit_signal {
            return quota_exhausted_diagnostic();
        }
        SafeDiagnostic::new(
            "swallowtail.command_code.headless.provider_failed",
            "Command Code reported a failed headless run",
        )
        .with_failure_classification(swallowtail_core::FailureClassification::new(
            swallowtail_core::FailureOrigin::Provider,
            swallowtail_core::FailureKind::Unknown,
            swallowtail_core::FailureRecovery::Unknown,
        ))
    }
}

fn quota_exhausted_diagnostic() -> SafeDiagnostic {
    SafeDiagnostic::new(
        "swallowtail.command_code.headless.quota_exhausted",
        "Command Code reported insufficient credits for the headless run",
    )
    .with_failure_classification(swallowtail_core::FailureClassification::new(
        swallowtail_core::FailureOrigin::Provider,
        swallowtail_core::FailureKind::QuotaExhausted,
        swallowtail_core::FailureRecovery::Unknown,
    ))
}

fn max_turns_diagnostic() -> SafeDiagnostic {
    SafeDiagnostic::new(
        "swallowtail.command_code.headless.max_turns",
        "Command Code reached its bounded maximum-turns limit",
    )
    .with_failure_classification(swallowtail_core::FailureClassification::new(
        swallowtail_core::FailureOrigin::Provider,
        swallowtail_core::FailureKind::InvalidRequest,
        swallowtail_core::FailureRecovery::Unknown,
    ))
}

fn mentions_insufficient_credit(text: &str) -> bool {
    text.to_ascii_lowercase().contains("credit")
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

fn required_text<'a>(value: &'a Value, key: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(malformed_stream)
}

/// Like [`required_text`] but tolerates a legitimately empty string (for
/// example, an empty `finalText` or a zero-length streamed delta).
fn required_string<'a>(value: &'a Value, key: &str) -> Result<&'a str, RuntimeFailure> {
    value.get(key).and_then(Value::as_str).ok_or_else(malformed_stream)
}

fn bounded_identifier(value: &str) -> Result<&str, RuntimeFailure> {
    if value.is_empty() || value.len() > 256 || value.chars().any(char::is_control) {
        Err(malformed_stream())
    } else {
        Ok(value)
    }
}

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn malformed_stream() -> RuntimeFailure {
    failure(
        "swallowtail.command_code.headless.malformed_stream",
        "Command Code emitted malformed headless stream output",
    )
}

fn session_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.command_code.headless.session_mismatch",
        "Command Code resumed session identity did not match the private continuity handle",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.command_code.headless.stream_limit",
        "Command Code exceeded the bounded headless stream limit",
    )
}

fn output_limit() -> RuntimeFailure {
    failure(
        "swallowtail.command_code.headless.output_limit",
        "Command Code exceeded the bounded headless output limit",
    )
}

fn output_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.command_code.headless.output_mismatch",
        "Command Code final text did not match its streamed output deltas",
    )
}

#[cfg(test)]
#[path = "events_tests.rs"]
mod tests;
