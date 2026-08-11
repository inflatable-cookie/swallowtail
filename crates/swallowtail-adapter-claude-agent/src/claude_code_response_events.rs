use crate::failure::failure;
use serde_json::Value;
use swallowtail_core::{
    ActivityContentStream, ActivityDisclosure, FailureClassification, FailureKind, FailureOrigin,
    FailureRecovery, ModelId, ProviderActivityRef, SafeDiagnostic,
};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityOperationId,
    ActivityStatus, CleanupOutcome, OperationContent, ProcessExit, ProviderObservation,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, TerminalOutcome, TerminalStatus, TokenUsage,
};

const MAXIMUM_LINE_BYTES: usize = 1024 * 1024;
const MAXIMUM_EVENT_COUNT: usize = 4096;
const MAXIMUM_OUTPUT_BYTES: usize = 64 * 1024;

pub(crate) struct ClaudeCodeResponseEventParser {
    model: ModelId,
    session_id: Option<String>,
    pending: Vec<u8>,
    sequence: u64,
    event_count: usize,
    assistant_text: Option<String>,
    final_output: Option<OperationContent>,
    provider_failure: Option<SafeDiagnostic>,
    init_seen: bool,
    terminal_seen: bool,
    operation_id: ActivityOperationId,
}

impl ClaudeCodeResponseEventParser {
    pub(crate) fn new(model: ModelId, operation_id: ActivityOperationId) -> Self {
        Self {
            model,
            session_id: None,
            pending: Vec::new(),
            sequence: 1,
            event_count: 0,
            assistant_text: None,
            final_output: None,
            provider_failure: None,
            init_seen: false,
            terminal_seen: false,
            operation_id,
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
        exit: ProcessExit,
    ) -> Result<(Vec<RuntimeEvent>, TerminalOutcome), RuntimeFailure> {
        let mut events = Vec::new();
        if !self.pending.is_empty() {
            let line = std::mem::take(&mut self.pending);
            events.extend(self.parse_line(&line)?);
        }
        let status = match exit.code() {
            Some(130) => harness_failure(
                "swallowtail.claude_code.response_only.process_interrupted",
                "Claude Code response-only execution was interrupted outside Swallowtail cancellation",
                FailureKind::TransportInterrupted,
                FailureRecovery::RetryMaySucceed,
            ),
            _ if self.provider_failure.is_some() => TerminalStatus::ProviderFailed(
                self.provider_failure.expect("checked provider failure"),
            ),
            _ if !exit.success() => harness_failure(
                "swallowtail.claude_code.response_only.process_failed",
                "Claude Code response-only execution exited unsuccessfully",
                FailureKind::Unknown,
                FailureRecovery::Unknown,
            ),
            _ if !self.init_seen || !self.terminal_seen || self.final_output.is_none() => {
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.claude_code.response_only.incomplete_stream",
                    "Claude Code response-only execution ended without complete text evidence",
                ))
            }
            _ => TerminalStatus::Completed,
        };
        let outcome = TerminalOutcome::new(status, CleanupOutcome::Clean);
        Ok((
            events,
            match self.final_output {
                Some(output) => outcome.with_output(output),
                None => outcome,
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
            Some("assistant") => self.parse_assistant(&payload),
            Some("result") => self.parse_result(&payload),
            Some("rate_limit_event") => {
                self.require_session(&payload)?;
                Ok(Vec::new())
            }
            _ => Err(malformed_stream()),
        }
    }

    fn parse_init(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let session_id = non_empty_string(payload, "session_id").ok_or_else(malformed_stream)?;
        let tools = payload
            .get("tools")
            .and_then(Value::as_array)
            .ok_or_else(malformed_stream)?;
        let mcp = payload
            .get("mcp_servers")
            .and_then(Value::as_array)
            .ok_or_else(malformed_stream)?;
        if self.init_seen
            || payload.get("model").and_then(Value::as_str) != Some(self.model.as_str())
            || payload.get("permissionMode").and_then(Value::as_str) != Some("default")
            || payload.get("claude_code_version").and_then(Value::as_str)
                != Some(crate::CLAUDE_CODE_RESPONSE_ONLY_VERSION)
            || !tools.is_empty()
            || !mcp.is_empty()
        {
            return Err(malformed_stream());
        }
        self.session_id = Some(session_id.to_owned());
        self.init_seen = true;
        Ok(Vec::new())
    }

    fn parse_assistant(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.require_session(payload)?;
        if self.assistant_text.is_some()
            || payload.get("error").is_some_and(|value| !value.is_null())
        {
            return Err(malformed_stream());
        }
        let message = payload.get("message").ok_or_else(malformed_stream)?;
        if message.get("role").and_then(Value::as_str) != Some("assistant")
            || message.get("model").and_then(Value::as_str) != Some(self.model.as_str())
            || message
                .get("stop_reason")
                .is_some_and(|value| !value.is_null())
        {
            return Err(malformed_stream());
        }
        let blocks = message
            .get("content")
            .and_then(Value::as_array)
            .ok_or_else(malformed_stream)?;
        if blocks.is_empty()
            || blocks
                .iter()
                .any(|block| block.get("type").and_then(Value::as_str) != Some("text"))
        {
            return Err(malformed_stream());
        }
        let mut text = String::new();
        let mut events = Vec::new();
        for block in blocks {
            let delta = block
                .get("text")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .ok_or_else(malformed_stream)?;
            if text.len().saturating_add(delta.len()) > MAXIMUM_OUTPUT_BYTES {
                return Err(stream_limit());
            }
            text.push_str(delta);
            events.push(self.event_with(
                RuntimeEventKind::OutputDelta,
                OperationContent::new(delta).map_err(|_| malformed_stream())?,
            ));
        }
        let activity = self.assistant_activity(message, &text)?;
        self.assistant_text = Some(text);
        events.push(self.event(RuntimeEventKind::Activity(activity)));
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
        let turns = payload
            .get("num_turns")
            .and_then(Value::as_u64)
            .ok_or_else(malformed_stream)?;
        if turns != 1
            || payload
                .get("structured_output")
                .is_some_and(|value| !value.is_null())
        {
            return Err(malformed_stream());
        }
        let usage = token_usage(payload).ok_or_else(malformed_stream)?;
        let mut events = Vec::new();
        if subtype == "success" && !is_error {
            let result = payload
                .get("result")
                .and_then(Value::as_str)
                .filter(|value| !value.is_empty())
                .ok_or_else(malformed_stream)?;
            if result.len() > MAXIMUM_OUTPUT_BYTES || self.assistant_text.as_deref() != Some(result)
            {
                return Err(malformed_stream());
            }
            let output = OperationContent::new(result).map_err(|_| malformed_stream())?;
            self.final_output = Some(output.clone());
            events.push(self.event_with(RuntimeEventKind::OutputAvailable, output));
        } else {
            self.provider_failure = Some(
                SafeDiagnostic::new(
                    "swallowtail.claude_code.response_only.provider_failed",
                    "Claude Code reported a response-only provider failure",
                )
                .with_failure_classification(FailureClassification::new(
                    FailureOrigin::Provider,
                    FailureKind::Unknown,
                    FailureRecovery::Unknown,
                )),
            );
        }
        self.terminal_seen = true;
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

    fn assistant_activity(
        &self,
        message: &Value,
        text: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let message_id = non_empty_string(message, "id").ok_or_else(malformed_stream)?;
        let content = ActivityContent::new(
            OperationContent::new(text).map_err(|_| malformed_stream())?,
            MAXIMUM_OUTPUT_BYTES,
        )
        .map_err(|_| malformed_stream())?;
        ActivityObservation::new(
            ActivityId::new("claude-code-response-only:assistant:1")
                .map_err(|_| malformed_stream())?,
            self.operation_id.clone(),
            ActivityKind::AssistantMessage,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
        )
        .map_err(|_| malformed_stream())?
        .with_provider_activity_ref(
            ProviderActivityRef::new(message_id).map_err(|_| malformed_stream())?,
        )
        .with_content(ActivityContentUpdate::new(
            ActivityContentChangeKind::ReplacementSnapshot,
            ActivityContentStream::FinalAnswerText,
            content,
        ))
        .map_err(|_| malformed_stream())
    }
}

fn token_usage(payload: &Value) -> Option<TokenUsage> {
    let usage = payload.get("usage")?;
    Some(
        TokenUsage::new(
            Some(usage.get("input_tokens")?.as_u64()?),
            Some(usage.get("output_tokens")?.as_u64()?),
        )
        .with_cache_tokens(
            optional_u64(usage, "cache_read_input_tokens")?,
            optional_u64(usage, "cache_creation_input_tokens")?,
        ),
    )
}

fn optional_u64(value: &Value, key: &str) -> Option<Option<u64>> {
    match value.get(key) {
        None | Some(Value::Null) => Some(None),
        Some(value) => value.as_u64().map(Some),
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
        "swallowtail.claude_code.response_only.malformed_stream",
        "Claude Code emitted malformed response-only stream-json output",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.claude_code.response_only.stream_limit",
        "Claude Code exceeded the bounded response-only stream-json limits",
    )
}

fn harness_failure(
    code: &'static str,
    message: &'static str,
    kind: FailureKind,
    recovery: FailureRecovery,
) -> TerminalStatus {
    TerminalStatus::ProviderFailed(
        SafeDiagnostic::new(code, message).with_failure_classification(FailureClassification::new(
            FailureOrigin::Harness,
            kind,
            recovery,
        )),
    )
}
