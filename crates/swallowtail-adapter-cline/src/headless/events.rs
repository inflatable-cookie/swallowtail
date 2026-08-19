use super::activity::ClineHeadlessActivityProjection;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Finish {
    Completed,
    Aborted,
    Failed,
}

pub(super) struct ClineHeadlessEventParser {
    pending: Vec<u8>,
    stderr_pending: Vec<u8>,
    sequence: u64,
    event_count: usize,
    assistant_output: String,
    final_output: Option<OperationContent>,
    terminal_seen: bool,
    finish: Option<Finish>,
    activity: ClineHeadlessActivityProjection,
}

impl ClineHeadlessEventParser {
    pub(super) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            pending: Vec::new(),
            stderr_pending: Vec::new(),
            sequence: 1,
            event_count: 0,
            assistant_output: String::new(),
            final_output: None,
            terminal_seen: false,
            finish: None,
            activity: ClineHeadlessActivityProjection::new(operation_id),
        }
    }

    pub(super) fn push_stdout(
        &mut self,
        bytes: &[u8],
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.pending.extend_from_slice(bytes);
        self.drain_pending(Stream::Stdout)
    }

    pub(super) fn push_stderr(
        &mut self,
        bytes: &[u8],
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.stderr_pending.extend_from_slice(bytes);
        self.drain_pending(Stream::Stderr)
    }

    fn drain_pending(&mut self, stream: Stream) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let mut events = Vec::new();
        loop {
            let newline = match stream {
                Stream::Stdout => self.pending.iter().position(|byte| *byte == b'\n'),
                Stream::Stderr => self.stderr_pending.iter().position(|byte| *byte == b'\n'),
            };
            let Some(newline) = newline else {
                break;
            };
            if newline > MAXIMUM_LINE_BYTES {
                return Err(stream_limit());
            }
            let line = match stream {
                Stream::Stdout => self.pending.drain(..=newline).collect::<Vec<_>>(),
                Stream::Stderr => self.stderr_pending.drain(..=newline).collect::<Vec<_>>(),
            };
            events.extend(self.parse_line(trim_newline(&line), stream)?);
        }
        let pending_len = match stream {
            Stream::Stdout => self.pending.len(),
            Stream::Stderr => self.stderr_pending.len(),
        };
        if pending_len > MAXIMUM_LINE_BYTES {
            return Err(stream_limit());
        }
        Ok(events)
    }

    pub(super) fn finish(mut self) -> Result<(Vec<RuntimeEvent>, ParsedTerminal), RuntimeFailure> {
        let mut events = Vec::new();
        if !self.pending.is_empty() {
            let line = std::mem::take(&mut self.pending);
            events.extend(self.parse_line(&line, Stream::Stdout)?);
        }
        if !self.stderr_pending.is_empty() {
            let line = std::mem::take(&mut self.stderr_pending);
            events.extend(self.parse_line(&line, Stream::Stderr)?);
        }
        Ok((
            events,
            ParsedTerminal::new(self.final_output, self.terminal_seen, self.finish),
        ))
    }

    fn parse_line(
        &mut self,
        line: &[u8],
        stream: Stream,
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if line.iter().all(u8::is_ascii_whitespace) {
            return Ok(Vec::new());
        }
        self.event_count = self.event_count.saturating_add(1);
        if self.event_count > MAXIMUM_EVENT_COUNT {
            return Err(stream_limit());
        }
        if self.terminal_seen {
            return Err(stream_limit());
        }
        let record: Value = serde_json::from_slice(line).map_err(|_| malformed_stream())?;
        if record.get("jsonrpc").is_some() {
            return Err(wrong_wire());
        }
        let record_type = record
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match (stream, record_type) {
            (Stream::Stderr, "error") => {
                self.finish = Some(Finish::Failed);
                Ok(Vec::new())
            }
            (Stream::Stderr, _) => Ok(Vec::new()),
            (Stream::Stdout, "ask" | "say") => Err(wrong_wire()),
            (
                Stream::Stdout,
                "run_start" | "run_abort_requested" | "team_event" | "team_restored",
            ) => Ok(Vec::new()),
            (Stream::Stdout, "agent_event") => self.agent_event(&record),
            (Stream::Stdout, "run_result") => self.run_result(&record),
            (Stream::Stdout, "run_aborted") => self.run_aborted(),
            (Stream::Stdout, "error") => {
                self.finish = Some(Finish::Failed);
                Ok(Vec::new())
            }
            (Stream::Stdout, _) => Err(malformed_stream()),
        }
    }

    fn agent_event(&mut self, record: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let event = record.get("event").ok_or_else(malformed_stream)?;
        let event_type = event
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match event_type {
            "content_start" => self.content_start(event),
            "content_end" => self.content_end(event),
            "iteration_start" | "iteration_end" | "done" | "notice" | "error" => Ok(Vec::new()),
            _ => Ok(Vec::new()),
        }
    }

    fn content_start(&mut self, event: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        match event.get("contentType").and_then(Value::as_str) {
            Some("text") => {
                let text = event.get("text").and_then(Value::as_str).unwrap_or("");
                if text.is_empty() {
                    return Ok(Vec::new());
                }
                self.append_output(text)?;
                let content = OperationContent::new(text).map_err(|_| malformed_stream())?;
                let mut events = vec![self.event_with(RuntimeEventKind::OutputDelta, content)];
                let observations = self.activity.text_delta(text)?;
                events.extend(self.activity_events(observations));
                Ok(events)
            }
            Some("reasoning") => {
                let text = event.get("reasoning").and_then(Value::as_str).unwrap_or("");
                if text.is_empty() {
                    return Ok(Vec::new());
                }
                let content = OperationContent::new(text).map_err(|_| malformed_stream())?;
                let mut events =
                    vec![self.event_with(RuntimeEventKind::ReasoningProgress, content)];
                let observations = self.activity.thought_delta(text)?;
                events.extend(self.activity_events(observations));
                Ok(events)
            }
            Some("tool") => {
                let tool_name = event
                    .get("toolName")
                    .and_then(Value::as_str)
                    .filter(|value| !value.is_empty())
                    .ok_or_else(malformed_stream)?;
                let observations = self.activity.tool_start(tool_name)?;
                Ok(self.activity_events(observations))
            }
            _ => Err(malformed_stream()),
        }
    }

    fn content_end(&mut self, event: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        match event.get("contentType").and_then(Value::as_str) {
            Some("tool") => {
                let tool_name = event
                    .get("toolName")
                    .and_then(Value::as_str)
                    .filter(|value| !value.is_empty())
                    .ok_or_else(malformed_stream)?;
                let observations = self.activity.tool_end(tool_name)?;
                Ok(self.activity_events(observations))
            }
            Some("text" | "reasoning") | None => Ok(Vec::new()),
            _ => Err(malformed_stream()),
        }
    }

    fn run_result(&mut self, record: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let finish_reason = record
            .get("finishReason")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        let mut events = Vec::new();
        match finish_reason {
            "completed" => {
                if let Some(text) = record.get("text").and_then(Value::as_str) {
                    if self.assistant_output.is_empty() {
                        self.append_output(text)?;
                    }
                    let output = OperationContent::new(text).map_err(|_| malformed_stream())?;
                    self.final_output = Some(output.clone());
                    events.push(self.event_with(RuntimeEventKind::OutputAvailable, output));
                }
                self.finish = Some(Finish::Completed);
                let observations = self.activity.complete(&TerminalStatus::Completed)?;
                events.extend(self.activity_events(observations));
            }
            "aborted" => {
                self.finish = Some(Finish::Aborted);
                let observations = self.activity.complete(&TerminalStatus::Cancelled)?;
                events.extend(self.activity_events(observations));
            }
            _ => {
                self.finish = Some(Finish::Failed);
                let status = TerminalStatus::ProviderFailed(provider_failed());
                let observations = self.activity.complete(&status)?;
                events.extend(self.activity_events(observations));
            }
        }
        self.terminal_seen = true;
        Ok(events)
    }

    fn run_aborted(&mut self) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.finish = Some(Finish::Aborted);
        self.terminal_seen = true;
        let observations = self.activity.complete(&TerminalStatus::Cancelled)?;
        Ok(self.activity_events(observations))
    }

    fn append_output(&mut self, text: &str) -> Result<(), RuntimeFailure> {
        if self.assistant_output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(output_limit());
        }
        self.assistant_output.push_str(text);
        Ok(())
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

pub(super) struct ParsedTerminal {
    final_output: Option<OperationContent>,
    terminal_seen: bool,
    finish: Option<Finish>,
}

impl ParsedTerminal {
    const fn new(
        final_output: Option<OperationContent>,
        terminal_seen: bool,
        finish: Option<Finish>,
    ) -> Self {
        Self {
            final_output,
            terminal_seen,
            finish,
        }
    }

    pub(super) fn outcome(self, exit: ProcessExit) -> TerminalOutcome {
        if self.finish == Some(Finish::Aborted) {
            return TerminalOutcome::new(TerminalStatus::Cancelled, CleanupOutcome::Clean);
        }
        if !exit.success() || self.finish == Some(Finish::Failed) {
            return TerminalOutcome::new(
                TerminalStatus::ProviderFailed(provider_failed()),
                CleanupOutcome::Clean,
            );
        }
        if !self.terminal_seen || self.finish != Some(Finish::Completed) {
            return TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.cline.headless.incomplete_stream",
                    "Cline headless stream ended without a terminal result",
                )),
                CleanupOutcome::Clean,
            );
        }
        let outcome = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean);
        match self.final_output {
            Some(output) => outcome.with_output(output),
            None => outcome,
        }
    }
}

#[derive(Clone, Copy)]
enum Stream {
    Stdout,
    Stderr,
}

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn malformed_stream() -> RuntimeFailure {
    failure(
        "swallowtail.cline.headless.malformed_stream",
        "Cline emitted malformed headless stream output",
    )
}

fn wrong_wire() -> RuntimeFailure {
    failure(
        "swallowtail.cline.headless.wrong_wire",
        "Cline headless received ACP or docs ask/say framing",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.cline.headless.stream_limit",
        "Cline exceeded the bounded headless stream limit",
    )
}

fn output_limit() -> RuntimeFailure {
    failure(
        "swallowtail.cline.headless.output_limit",
        "Cline exceeded the bounded headless output limit",
    )
}

fn provider_failed() -> SafeDiagnostic {
    SafeDiagnostic::new(
        "swallowtail.cline.headless.provider_failed",
        "Cline reported a failed headless run",
    )
    .with_failure_classification(swallowtail_core::FailureClassification::new(
        swallowtail_core::FailureOrigin::Harness,
        swallowtail_core::FailureKind::Unknown,
        swallowtail_core::FailureRecovery::Unknown,
    ))
}

#[cfg(test)]
mod tests {
    use super::ClineHeadlessEventParser;
    use swallowtail_runtime::{ActivityOperationId, ProcessExit, RuntimeRunId, TerminalStatus};

    const SUCCESS: &str = include_str!("../../tests/fixtures/cline-headless-3.0.55/success.jsonl");
    const ABORT: &str = include_str!("../../tests/fixtures/cline-headless-3.0.55/abort.jsonl");

    fn parser() -> ClineHeadlessEventParser {
        ClineHeadlessEventParser::new(ActivityOperationId::Run(
            RuntimeRunId::new("cline-headless:fixture").expect("run id"),
        ))
    }

    #[test]
    fn success_corpus_completes_without_ask_say_or_jsonrpc() {
        let mut parser = parser();
        let events = parser
            .push_stdout(SUCCESS.as_bytes())
            .expect("success parses");
        assert!(events.iter().any(|event| {
            event
                .content()
                .is_some_and(|content| content.as_str() == "Cline display text.")
        }));
        let (_, terminal) = parser.finish().expect("finish");
        let outcome = terminal.outcome(ProcessExit::new(true, Some(0)));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            outcome
                .output()
                .map(swallowtail_runtime::OperationContent::as_str),
            Some("Cline display text.")
        );
        assert!(!format!("{events:?}").contains("opaque-fixture-session"));
        assert!(!format!("{outcome:?}").contains("Cline display text"));
    }

    #[test]
    fn abort_corpus_cancels() {
        let mut parser = parser();
        parser.push_stdout(ABORT.as_bytes()).expect("abort parses");
        let (_, terminal) = parser.finish().expect("finish");
        assert_eq!(
            terminal.outcome(ProcessExit::new(true, Some(0))).status(),
            &TerminalStatus::Cancelled
        );
    }

    #[test]
    fn docs_ask_say_and_acp_jsonrpc_are_wrong_wire() {
        let mut ask_parser = parser();
        let ask_say = ask_parser.push_stdout(
            br#"{"type":"say","text":"docs schema","ts":0,"say":"text"}
"#,
        );
        assert_eq!(
            ask_say.expect_err("ask/say").diagnostic().code(),
            "swallowtail.cline.headless.wrong_wire"
        );
        let mut acp_parser = parser();
        let acp = acp_parser.push_stdout(
            br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
"#,
        );
        assert_eq!(
            acp.expect_err("acp").diagnostic().code(),
            "swallowtail.cline.headless.wrong_wire"
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
                    br#"{"ts":"2026-08-18T00:00:00.000Z"}
"#
                )
                .is_err()
        );
    }

    #[test]
    fn usage_is_not_copied_into_diagnostics() {
        let mut parser = parser();
        parser
            .push_stdout(
                br#"{"ts":"2026-08-18T00:00:00.000Z","type":"run_result","finishReason":"completed","text":"ok","usage":{"private":"secret-tokens"}}
"#,
            )
            .expect("usage ignored");
        let (_, terminal) = parser.finish().expect("finish");
        let outcome = terminal.outcome(ProcessExit::new(true, Some(0)));
        assert!(!format!("{outcome:?}").contains("secret-tokens"));
    }
}
