use super::activity::QoderHeadlessActivityProjection;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Finish {
    Completed,
    MaxTurns,
    Aborted,
    Failed,
}

pub(super) struct QoderHeadlessEventParser {
    pending: Vec<u8>,
    stderr: String,
    sequence: u64,
    event_count: usize,
    assistant_output: String,
    final_output: Option<OperationContent>,
    terminal_seen: bool,
    finish: Option<Finish>,
    activity: QoderHeadlessActivityProjection,
}

impl QoderHeadlessEventParser {
    pub(super) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            pending: Vec::new(),
            stderr: String::new(),
            sequence: 1,
            event_count: 0,
            assistant_output: String::new(),
            final_output: None,
            terminal_seen: false,
            finish: None,
            activity: QoderHeadlessActivityProjection::new(operation_id),
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
        Ok((
            events,
            ParsedTerminal::new(
                self.final_output,
                self.activity,
                self.sequence,
                self.terminal_seen,
                self.finish,
            ),
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
        if self.terminal_seen {
            return Err(malformed_stream());
        }
        let record: Value = serde_json::from_slice(line).map_err(|_| malformed_stream())?;
        if record.is_array() || record.get("jsonrpc").is_some() {
            return Err(wrong_wire());
        }
        let record_type = record
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match record_type {
            "system" | "stream_event" => Ok(Vec::new()),
            "assistant" => self.assistant(&record),
            "result" => self.result(&record),
            _ => Err(malformed_stream()),
        }
    }

    fn assistant(&mut self, record: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let text = assistant_text(record)?;
        if text.is_empty() {
            return Ok(Vec::new());
        }
        self.append_output(&text)?;
        let content = OperationContent::new(&text).map_err(|_| malformed_stream())?;
        let mut events = vec![self.event_with(RuntimeEventKind::OutputDelta, content)];
        let observations = self.activity.text_delta(&text)?;
        events.extend(self.activity_events(observations));
        Ok(events)
    }

    fn result(&mut self, record: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let subtype = record
            .get("subtype")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        let is_error = record
            .get("is_error")
            .and_then(Value::as_bool)
            .ok_or_else(malformed_stream)?;
        let mut events = Vec::new();
        match (subtype, is_error) {
            ("success", false) => {
                let text = record
                    .get("result")
                    .and_then(Value::as_str)
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned)
                    .unwrap_or_else(|| self.assistant_output.clone());
                if !text.is_empty() {
                    let content = OperationContent::new(&text).map_err(|_| malformed_stream())?;
                    self.final_output = Some(content.clone());
                    events.push(self.event_with(RuntimeEventKind::OutputAvailable, content));
                }
                self.finish = Some(Finish::Completed);
            }
            ("error_max_turns", true) => {
                self.finish = Some(Finish::MaxTurns);
            }
            ("error_during_execution", true) if is_abort(record) => {
                self.finish = Some(Finish::Aborted);
            }
            ("error_during_execution", true) => {
                self.finish = Some(Finish::Failed);
            }
            _ => return Err(malformed_stream()),
        }
        self.terminal_seen = true;
        Ok(events)
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
    activity: QoderHeadlessActivityProjection,
    sequence: u64,
    terminal_seen: bool,
    finish: Option<Finish>,
}

impl ParsedTerminal {
    const fn new(
        final_output: Option<OperationContent>,
        activity: QoderHeadlessActivityProjection,
        sequence: u64,
        terminal_seen: bool,
        finish: Option<Finish>,
    ) -> Self {
        Self {
            final_output,
            activity,
            sequence,
            terminal_seen,
            finish,
        }
    }

    pub(super) fn finalize(
        mut self,
        exit: ProcessExit,
    ) -> Result<(Vec<RuntimeEvent>, TerminalOutcome), RuntimeFailure> {
        let status = match self.finish {
            Some(Finish::Aborted) => TerminalStatus::Cancelled,
            Some(Finish::MaxTurns) => TerminalStatus::ProviderFailed(limit_failed()),
            Some(Finish::Failed) => TerminalStatus::ProviderFailed(provider_failed()),
            Some(Finish::Completed) if self.terminal_seen => {
                if exit.success() {
                    TerminalStatus::Completed
                } else {
                    TerminalStatus::ProviderFailed(provider_failed())
                }
            }
            Some(Finish::Completed) | None => {
                if !exit.success() {
                    TerminalStatus::ProviderFailed(provider_failed())
                } else {
                    TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                        "swallowtail.qoder.headless.incomplete_stream",
                        "Qoder headless stream ended without a terminal result",
                    ))
                }
            }
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
    let Some(content) = record.pointer("/message/content").and_then(Value::as_array) else {
        return Ok(String::new());
    };
    let mut text = String::new();
    for block in content {
        if block.get("type").and_then(Value::as_str) != Some("text") {
            continue;
        }
        if let Some(part) = block.get("text").and_then(Value::as_str) {
            text.push_str(part);
        }
    }
    Ok(text)
}

fn is_abort(record: &Value) -> bool {
    if record.get("terminal_reason").and_then(Value::as_str) == Some("aborted_streaming") {
        return true;
    }
    record
        .get("errors")
        .and_then(Value::as_array)
        .is_some_and(|errors| {
            errors
                .iter()
                .any(|error| error.as_str() == Some("Operation aborted"))
        })
}

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn malformed_stream() -> RuntimeFailure {
    failure(
        "swallowtail.qoder.headless.malformed_stream",
        "Qoder emitted malformed headless stream output",
    )
}

fn wrong_wire() -> RuntimeFailure {
    failure(
        "swallowtail.qoder.headless.wrong_wire",
        "Qoder headless received ACP, SDK, or dump-at-end framing",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.qoder.headless.stream_limit",
        "Qoder exceeded the bounded headless stream limit",
    )
}

fn output_limit() -> RuntimeFailure {
    failure(
        "swallowtail.qoder.headless.output_limit",
        "Qoder exceeded the bounded headless output limit",
    )
}

fn provider_failed() -> SafeDiagnostic {
    SafeDiagnostic::new(
        "swallowtail.qoder.headless.provider_failed",
        "Qoder reported a failed headless run",
    )
    .with_failure_classification(swallowtail_core::FailureClassification::new(
        swallowtail_core::FailureOrigin::Harness,
        swallowtail_core::FailureKind::Unknown,
        swallowtail_core::FailureRecovery::Unknown,
    ))
}

fn limit_failed() -> SafeDiagnostic {
    SafeDiagnostic::new(
        "swallowtail.qoder.headless.max_turns",
        "Qoder reached the configured conversation limit",
    )
    .with_failure_classification(swallowtail_core::FailureClassification::new(
        swallowtail_core::FailureOrigin::Harness,
        swallowtail_core::FailureKind::Unknown,
        swallowtail_core::FailureRecovery::Unknown,
    ))
}

#[cfg(test)]
mod tests {
    use super::QoderHeadlessEventParser;
    use swallowtail_runtime::{ActivityOperationId, ProcessExit, RuntimeRunId, TerminalStatus};

    const SUCCESS: &str = include_str!("../../tests/fixtures/qoder-headless-1.1.25/success.jsonl");
    const ABORT: &str = include_str!("../../tests/fixtures/qoder-headless-1.1.25/abort.jsonl");
    const LIMIT: &str = include_str!("../../tests/fixtures/qoder-headless-1.1.25/limit.jsonl");
    const JSON_DUMP: &str =
        include_str!("../../tests/fixtures/qoder-headless-1.1.25/json-dump.json");

    fn parser() -> QoderHeadlessEventParser {
        QoderHeadlessEventParser::new(ActivityOperationId::Run(
            RuntimeRunId::new("qoder-headless:fixture").expect("run id"),
        ))
    }

    #[test]
    fn success_corpus_completes_without_session_or_usage_leak() {
        let mut parser = parser();
        let events = parser
            .push_stdout(SUCCESS.as_bytes())
            .expect("success parses");
        assert!(events.iter().any(|event| {
            event
                .content()
                .is_some_and(|content| content.as_str() == "Qoder display text.")
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
            Some("Qoder display text.")
        );
        assert!(!format!("{events:?}").contains("opaque-fixture-session"));
        assert!(!format!("{outcome:?}").contains("Qoder display text"));
        assert!(!format!("{events:?}").contains("total_cost_usd"));
    }

    #[test]
    fn abort_corpus_cancels() {
        let mut parser = parser();
        parser.push_stdout(ABORT.as_bytes()).expect("abort parses");
        let (_, terminal) = parser.finish().expect("finish");
        let (_, outcome) = terminal
            .finalize(ProcessExit::new(true, Some(0)))
            .expect("finalize");
        assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    }

    #[test]
    fn max_turns_is_bounded_failure_not_end_turn() {
        let mut parser = parser();
        parser.push_stdout(LIMIT.as_bytes()).expect("limit parses");
        let (_, terminal) = parser.finish().expect("finish");
        let (_, outcome) = terminal
            .finalize(ProcessExit::new(true, Some(0)))
            .expect("finalize");
        match outcome.status() {
            TerminalStatus::ProviderFailed(diagnostic) => {
                assert_eq!(diagnostic.code(), "swallowtail.qoder.headless.max_turns");
            }
            other => panic!("expected provider failed, got {other:?}"),
        }
        assert!(!format!("{outcome:?}").contains("Maximum turns exceeded"));
    }

    #[test]
    fn json_dump_and_acp_jsonrpc_are_wrong_or_malformed_wire() {
        let mut dump = parser();
        assert_eq!(
            dump.push_stdout(JSON_DUMP.as_bytes())
                .expect_err("pretty json dump")
                .diagnostic()
                .code(),
            "swallowtail.qoder.headless.malformed_stream"
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
            "swallowtail.qoder.headless.wrong_wire"
        );
        let mut array = parser();
        assert_eq!(
            array
                .push_stdout(b"[]\n")
                .expect_err("array")
                .diagnostic()
                .code(),
            "swallowtail.qoder.headless.wrong_wire"
        );
    }

    #[test]
    fn malformed_missing_type_unknown_and_post_terminal_fail_closed() {
        let mut malformed = parser();
        assert!(malformed.push_stdout(b"{\n").is_err());
        let mut missing_type = parser();
        assert!(
            missing_type
                .push_stdout(
                    br#"{"session_id":"opaque-fixture-session"}
"#
                )
                .is_err()
        );
        let mut unknown = parser();
        assert!(
            unknown
                .push_stdout(
                    br#"{"type":"qoder_future_envelope"}
"#
                )
                .is_err()
        );
        let mut extra = parser();
        extra
            .push_stdout(SUCCESS.as_bytes())
            .expect("success parses");
        assert!(
            extra
                .push_stdout(
                    br#"{"type":"assistant","message":{"content":[{"type":"text","text":"late"}]}}
"#
                )
                .is_err()
        );
    }

    #[test]
    fn stream_event_and_system_init_are_ignored() {
        let mut parser = parser();
        let events = parser
            .push_stdout(
                br#"{"type":"system","subtype":"init","session_id":"opaque-fixture-session"}
{"type":"stream_event","event":{"type":"content_block_delta","delta":{"type":"text_delta","text":"partial"}}}
{"type":"system","subtype":"hook_started","session_id":"opaque-fixture-session"}
"#,
            )
            .expect("unselected envelopes parse");
        assert!(events.is_empty());
        assert!(!format!("{events:?}").contains("opaque-fixture-session"));
        assert!(!format!("{events:?}").contains("partial"));
    }

    #[test]
    fn usage_is_not_copied_into_diagnostics() {
        let mut parser = parser();
        parser
            .push_stdout(
                br#"{"type":"result","subtype":"success","is_error":false,"result":"ok","usage":{"private":"secret-tokens"},"total_cost_usd":1.25}
"#,
            )
            .expect("usage ignored");
        let (_, terminal) = parser.finish().expect("finish");
        let (_, outcome) = terminal
            .finalize(ProcessExit::new(true, Some(0)))
            .expect("finalize");
        assert!(!format!("{outcome:?}").contains("secret-tokens"));
        assert!(!format!("{outcome:?}").contains("total_cost_usd"));
        assert!(!format!("{outcome:?}").contains("1.25"));
    }
}
