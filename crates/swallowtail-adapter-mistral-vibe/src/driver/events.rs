use super::activity::VibeHeadlessActivityProjection;
use crate::failure::failure;
use serde_json::Value;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ActivityObservation, ActivityOperationId, CleanupOutcome, OperationContent, ProcessExit,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, TerminalOutcome, TerminalStatus,
};

const MAXIMUM_LINE_BYTES: usize = 64 * 1024;
const MAXIMUM_EVENT_COUNT: usize = 4096;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_STDERR_BYTES: usize = 64 * 1024;
const LIMIT_STDERR: &str = "The configured conversation limit was reached";

pub(super) struct VibeHeadlessEventParser {
    pending: Vec<u8>,
    stderr: String,
    sequence: u64,
    event_count: usize,
    assistant_output: String,
    final_output: Option<OperationContent>,
    activity: VibeHeadlessActivityProjection,
}

impl VibeHeadlessEventParser {
    pub(super) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            pending: Vec::new(),
            stderr: String::new(),
            sequence: 1,
            event_count: 0,
            assistant_output: String::new(),
            final_output: None,
            activity: VibeHeadlessActivityProjection::new(operation_id),
        }
    }

    pub(super) fn push_stdout(
        &mut self,
        bytes: &[u8],
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.pending.extend_from_slice(bytes);
        self.drain_pending()
    }

    pub(super) fn push_stderr(
        &mut self,
        bytes: &[u8],
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if self.stderr.len().saturating_add(bytes.len()) > MAXIMUM_STDERR_BYTES {
            return Err(stream_limit());
        }
        self.stderr.push_str(&String::from_utf8_lossy(bytes));
        Ok(Vec::new())
    }

    fn drain_pending(&mut self) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let mut events = Vec::new();
        while let Some(newline) = self.pending.iter().position(|byte| *byte == b'\n') {
            if newline > MAXIMUM_LINE_BYTES {
                return Err(stream_limit());
            }
            let line = self.pending.drain(..=newline).collect::<Vec<_>>();
            events.extend(self.parse_line(trim_newline(&line))?);
        }
        if self.pending.len() > MAXIMUM_LINE_BYTES {
            return Err(stream_limit());
        }
        Ok(events)
    }

    pub(super) fn finish(mut self) -> Result<(Vec<RuntimeEvent>, ParsedTerminal), RuntimeFailure> {
        let mut events = Vec::new();
        if !self.pending.is_empty() {
            let line = std::mem::take(&mut self.pending);
            events.extend(self.parse_line(trim_newline(&line))?);
        }
        let limit = self.stderr.contains(LIMIT_STDERR);
        Ok((
            events,
            ParsedTerminal::new(self.final_output, self.activity, self.sequence, limit),
        ))
    }

    fn parse_line(&mut self, line: &[u8]) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if line.iter().all(u8::is_ascii_whitespace) {
            return Ok(Vec::new());
        }
        self.event_count = self.event_count.saturating_add(1);
        if self.event_count > MAXIMUM_EVENT_COUNT {
            return Err(stream_limit());
        }
        let record: Value = serde_json::from_slice(line).map_err(|_| malformed_stream())?;
        if record.is_array()
            || record.get("jsonrpc").is_some()
            || record.get("teleportUrl").is_some()
            || (record.get("history").is_some() && record.get("type").is_none())
        {
            return Err(wrong_wire());
        }
        let record_type = record
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        if record.get("generationStatus").and_then(Value::as_str) == Some("in_progress") {
            return Ok(Vec::new());
        }
        match record_type {
            "message" => self.message(&record),
            "reasoning" => self.reasoning(&record),
            "effect" => self.effect(&record),
            "callback" | "checkpoint" | "notice" => Ok(Vec::new()),
            _ => Err(malformed_stream()),
        }
    }

    fn message(&mut self, record: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let role = record
            .get("role")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        if role != "assistant" {
            return Ok(Vec::new());
        }
        let text = assistant_text(record)?;
        if text.is_empty() {
            return Ok(Vec::new());
        }
        self.append_output(&text)?;
        let content = OperationContent::new(&text).map_err(|_| malformed_stream())?;
        self.final_output = Some(content.clone());
        let mut events = vec![
            self.event_with(RuntimeEventKind::OutputDelta, content.clone()),
            self.event_with(RuntimeEventKind::OutputAvailable, content),
        ];
        let observations = self.activity.text_delta(&text)?;
        events.extend(self.activity_events(observations));
        Ok(events)
    }

    fn reasoning(&mut self, record: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let text = record.get("text").and_then(Value::as_str).unwrap_or("");
        if text.is_empty() {
            return Ok(Vec::new());
        }
        let content = OperationContent::new(text).map_err(|_| malformed_stream())?;
        let mut events = vec![self.event_with(RuntimeEventKind::ReasoningProgress, content)];
        let observations = self.activity.thought_delta(text)?;
        events.extend(self.activity_events(observations));
        Ok(events)
    }

    fn effect(&mut self, record: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let title = record
            .get("title")
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .ok_or_else(malformed_stream)?;
        let observations = self.activity.tool_completed(title)?;
        Ok(self.activity_events(observations))
    }

    fn append_output(&mut self, text: &str) -> Result<(), RuntimeFailure> {
        if self.assistant_output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(output_limit());
        }
        self.assistant_output.push_str(text);
        Ok(())
    }

    fn event_with(&mut self, kind: RuntimeEventKind, content: OperationContent) -> RuntimeEvent {
        let sequence = self.sequence;
        self.sequence = self.sequence.saturating_add(1);
        RuntimeEvent::with_content(sequence, kind, content)
    }

    fn event(&mut self, kind: RuntimeEventKind) -> RuntimeEvent {
        let sequence = self.sequence;
        self.sequence = self.sequence.saturating_add(1);
        RuntimeEvent::new(sequence, kind)
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

pub(super) struct ParsedTerminal {
    final_output: Option<OperationContent>,
    activity: VibeHeadlessActivityProjection,
    sequence: u64,
    limit: bool,
}

impl ParsedTerminal {
    const fn new(
        final_output: Option<OperationContent>,
        activity: VibeHeadlessActivityProjection,
        sequence: u64,
        limit: bool,
    ) -> Self {
        Self {
            final_output,
            activity,
            sequence,
            limit,
        }
    }

    pub(super) fn finalize(
        mut self,
        exit: ProcessExit,
    ) -> Result<(Vec<RuntimeEvent>, TerminalOutcome), RuntimeFailure> {
        let status = if self.limit {
            TerminalStatus::ProviderFailed(limit_failed())
        } else if !exit.success() {
            TerminalStatus::ProviderFailed(provider_failed())
        } else {
            TerminalStatus::Completed
        };
        let observations = self.activity.complete(&status)?;
        let mut events = Vec::new();
        for observation in observations {
            events.push(RuntimeEvent::new(
                self.sequence,
                RuntimeEventKind::Activity(observation),
            ));
            self.sequence = self.sequence.saturating_add(1);
        }
        let mut outcome = TerminalOutcome::new(status.clone(), CleanupOutcome::Clean);
        if matches!(status, TerminalStatus::Completed)
            && let Some(output) = self.final_output
        {
            outcome = outcome.with_output(output);
        }
        Ok((events, outcome))
    }
}

fn assistant_text(record: &Value) -> Result<String, RuntimeFailure> {
    let Some(content) = record.get("content").and_then(Value::as_array) else {
        return Ok(String::new());
    };
    let mut text = String::new();
    for block in content {
        if block.get("type").and_then(Value::as_str) != Some("text") {
            continue;
        }
        if let Some(part) = block.get("text").and_then(Value::as_str) {
            if !text.is_empty() {
                text.push_str("\n\n");
            }
            text.push_str(part);
        }
    }
    Ok(text)
}

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn malformed_stream() -> RuntimeFailure {
    failure(
        "swallowtail.mistral-vibe.headless.malformed_stream",
        "Mistral Vibe emitted malformed headless stream output",
    )
}

fn wrong_wire() -> RuntimeFailure {
    failure(
        "swallowtail.mistral-vibe.headless.wrong_wire",
        "Mistral Vibe headless received ACP, JSON dump, or teleport framing",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.mistral-vibe.headless.stream_limit",
        "Mistral Vibe exceeded the bounded headless stream limit",
    )
}

fn output_limit() -> RuntimeFailure {
    failure(
        "swallowtail.mistral-vibe.headless.output_limit",
        "Mistral Vibe exceeded the bounded headless output limit",
    )
}

fn provider_failed() -> SafeDiagnostic {
    SafeDiagnostic::new(
        "swallowtail.mistral-vibe.headless.provider_failed",
        "Mistral Vibe reported a failed headless run",
    )
    .with_failure_classification(swallowtail_core::FailureClassification::new(
        swallowtail_core::FailureOrigin::Harness,
        swallowtail_core::FailureKind::Unknown,
        swallowtail_core::FailureRecovery::Unknown,
    ))
}

fn limit_failed() -> SafeDiagnostic {
    SafeDiagnostic::new(
        "swallowtail.mistral-vibe.headless.max_turns",
        "Mistral Vibe reached the configured conversation limit",
    )
    .with_failure_classification(swallowtail_core::FailureClassification::new(
        swallowtail_core::FailureOrigin::Harness,
        swallowtail_core::FailureKind::Unknown,
        swallowtail_core::FailureRecovery::Unknown,
    ))
}

#[cfg(test)]
mod tests {
    use super::VibeHeadlessEventParser;
    use swallowtail_runtime::{ActivityOperationId, ProcessExit, RuntimeRunId, TerminalStatus};

    const SUCCESS: &str =
        include_str!("../../tests/fixtures/mistral-vibe-headless-2.24.2/success.jsonl");
    const ABORT: &str =
        include_str!("../../tests/fixtures/mistral-vibe-headless-2.24.2/abort.jsonl");
    const JSON_DUMP: &str =
        include_str!("../../tests/fixtures/mistral-vibe-headless-2.24.2/json-dump.json");
    const STDERR: &str =
        include_str!("../../tests/fixtures/mistral-vibe-headless-2.24.2/stderr-error.txt");
    const LIMIT: &str =
        include_str!("../../tests/fixtures/mistral-vibe-headless-2.24.2/limit-stderr.txt");

    fn parser() -> VibeHeadlessEventParser {
        VibeHeadlessEventParser::new(ActivityOperationId::Run(
            RuntimeRunId::new("mistral-vibe-headless:fixture").expect("run id"),
        ))
    }

    #[test]
    fn success_corpus_completes_without_acp_or_json_dump() {
        let mut parser = parser();
        let events = parser
            .push_stdout(SUCCESS.as_bytes())
            .expect("success parses");
        assert!(events.iter().any(|event| {
            event
                .content()
                .is_some_and(|content| content.as_str() == "Vibe display text.")
        }));
        let (_, terminal) = parser.finish().expect("finish");
        let (_, outcome) = terminal
            .finalize(ProcessExit::new(true, Some(0)))
            .expect("finalize");
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            outcome
                .output()
                .map(swallowtail_runtime::OperationContent::as_str),
            Some("Vibe display text.")
        );
        assert!(!format!("{events:?}").contains("opaque-fixture-session"));
        assert!(!format!("{outcome:?}").contains("Vibe display text"));
    }

    #[test]
    fn abort_truncated_line_fails_closed() {
        let mut parser = parser();
        assert_eq!(
            parser
                .push_stdout(ABORT.as_bytes())
                .expect_err("truncated abort")
                .diagnostic()
                .code(),
            "swallowtail.mistral-vibe.headless.malformed_stream"
        );
    }

    #[test]
    fn json_dump_and_acp_jsonrpc_are_wrong_wire() {
        let compact: String = JSON_DUMP.chars().filter(|ch| !ch.is_whitespace()).collect();
        let mut dump = parser();
        assert_eq!(
            dump.push_stdout(format!("{compact}\n").as_bytes())
                .expect_err("json dump")
                .diagnostic()
                .code(),
            "swallowtail.mistral-vibe.headless.wrong_wire"
        );
        let mut acp = parser();
        assert_eq!(
            acp.push_stdout(
                br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
"#
            )
            .expect_err("acp")
            .diagnostic()
            .code(),
            "swallowtail.mistral-vibe.headless.wrong_wire"
        );
    }

    #[test]
    fn malformed_and_missing_type_fail_closed() {
        let mut malformed = parser();
        assert!(malformed.push_stdout(b"{\n").is_err());
        let mut missing_type = parser();
        assert!(
            missing_type
                .push_stdout(
                    br#"{"id":"opaque","generationStatus":"completed"}
"#
                )
                .is_err()
        );
    }

    #[test]
    fn stderr_error_and_limit_do_not_leak_native_text() {
        let mut failed = parser();
        failed
            .push_stderr(STDERR.as_bytes())
            .expect("stderr accepted");
        let (_, terminal) = failed.finish().expect("finish");
        let (_, outcome) = terminal
            .finalize(ProcessExit::new(false, Some(1)))
            .expect("finalize");
        assert!(matches!(
            outcome.status(),
            TerminalStatus::ProviderFailed(_)
        ));
        assert!(!format!("{outcome:?}").contains("No prompt provided"));

        let mut limited = parser();
        limited
            .push_stderr(LIMIT.as_bytes())
            .expect("limit stderr accepted");
        let (_, terminal) = limited.finish().expect("finish");
        let (_, outcome) = terminal
            .finalize(ProcessExit::new(false, Some(1)))
            .expect("finalize");
        match outcome.status() {
            TerminalStatus::ProviderFailed(diagnostic) => {
                assert_eq!(
                    diagnostic.code(),
                    "swallowtail.mistral-vibe.headless.max_turns"
                );
            }
            other => panic!("expected provider failed, got {other:?}"),
        }
        assert!(!format!("{outcome:?}").contains("The configured conversation limit was reached"));
    }

    #[test]
    fn effect_detail_is_not_copied_into_diagnostics() {
        let mut parser = parser();
        parser
            .push_stdout(
                br#"{"id":"opaque","sessionId":"opaque-fixture-session","generationStatus":"completed","type":"effect","title":"read","detail":{"private":"secret-path"},"state":{"status":"completed"}}
"#,
            )
            .expect("effect parses");
        let (events, terminal) = parser.finish().expect("finish");
        let (more, outcome) = terminal
            .finalize(ProcessExit::new(true, Some(0)))
            .expect("finalize");
        let events = [events, more].concat();
        assert!(!format!("{events:?}").contains("secret-path"));
        assert!(!format!("{outcome:?}").contains("secret-path"));
    }
}
