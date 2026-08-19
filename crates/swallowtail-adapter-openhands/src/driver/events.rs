use super::activity::OpenHandsActivityProjection;
use crate::failure::failure;
use serde_json::Value;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ActivityObservation, ActivityOperationId, CleanupOutcome, OperationContent, ProcessExit,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, TerminalOutcome, TerminalStatus,
};

const MAXIMUM_EVENT_COUNT: usize = 4096;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Finish {
    Completed,
    Bounded,
    Aborted,
    Failed,
}

pub(super) struct OpenHandsEventParser {
    sequence: u64,
    event_count: usize,
    assistant_output: String,
    final_output: Option<OperationContent>,
    terminal_seen: bool,
    finish: Option<Finish>,
    activity: OpenHandsActivityProjection,
}

impl OpenHandsEventParser {
    pub(super) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            sequence: 1,
            event_count: 0,
            assistant_output: String::new(),
            final_output: None,
            terminal_seen: false,
            finish: None,
            activity: OpenHandsActivityProjection::new(operation_id),
        }
    }

    pub(super) fn push_event(
        &mut self,
        record: &Value,
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.event_count = self.event_count.saturating_add(1);
        if self.event_count > MAXIMUM_EVENT_COUNT {
            return Err(stream_limit());
        }
        if self.terminal_seen {
            return Ok(Vec::new());
        }
        if record.is_array() || record.get("jsonrpc").is_some() {
            return Err(wrong_wire());
        }
        let kind = record
            .get("kind")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match kind {
            "StreamingDeltaEvent" => self.delta(record),
            "MessageEvent" => self.message(record),
            "FinishAction" => self.finish_action(record),
            "ConversationStateUpdateEvent" => self.state(record),
            "InterruptEvent" | "PauseEvent" => {
                self.finish = Some(Finish::Aborted);
                self.terminal_seen = true;
                Ok(Vec::new())
            }
            "AgentErrorEvent" | "ConversationErrorEvent" | "ServerErrorEvent" => {
                self.finish = Some(Finish::Failed);
                self.terminal_seen = true;
                Ok(Vec::new())
            }
            _ => Err(malformed_stream()),
        }
    }

    pub(super) fn finish(self) -> Result<(Vec<RuntimeEvent>, ParsedTerminal), RuntimeFailure> {
        Ok((
            Vec::new(),
            ParsedTerminal::new(
                self.final_output,
                self.activity,
                self.sequence,
                self.terminal_seen,
                self.finish,
            ),
        ))
    }

    fn delta(&mut self, record: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let text = record
            .get("content")
            .and_then(Value::as_str)
            .unwrap_or_default();
        self.emit_text(text)
    }

    fn message(&mut self, record: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let source = record
            .get("source")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if source != "agent" {
            return Ok(Vec::new());
        }
        self.emit_text(&message_text(record)?)
    }

    fn finish_action(&mut self, record: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let text = record
            .get("message")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !text.is_empty() {
            self.append_output(text)?;
            let content = OperationContent::new(text).map_err(|_| malformed_stream())?;
            self.final_output = Some(content);
        }
        Ok(Vec::new())
    }

    fn state(&mut self, record: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let key = record
            .get("key")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if key != "execution_status" {
            return Ok(Vec::new());
        }
        let status = match record.get("value") {
            Some(Value::String(value)) => value.as_str(),
            _ => return Ok(Vec::new()),
        };
        match status {
            "finished" => {
                if self.final_output.is_none() && !self.assistant_output.is_empty() {
                    let content = OperationContent::new(&self.assistant_output)
                        .map_err(|_| malformed_stream())?;
                    self.final_output = Some(content);
                }
                self.finish = Some(Finish::Completed);
                self.terminal_seen = true;
            }
            "stuck" => {
                self.finish = Some(Finish::Bounded);
                self.terminal_seen = true;
            }
            "error" => {
                self.finish = Some(Finish::Failed);
                self.terminal_seen = true;
            }
            "idle" | "running" | "paused" | "waiting_for_confirmation" | "deleting" => {}
            _ => return Err(malformed_stream()),
        }
        Ok(Vec::new())
    }

    fn emit_text(&mut self, text: &str) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
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
    activity: OpenHandsActivityProjection,
    sequence: u64,
    terminal_seen: bool,
    finish: Option<Finish>,
}

impl ParsedTerminal {
    const fn new(
        final_output: Option<OperationContent>,
        activity: OpenHandsActivityProjection,
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
            Some(Finish::Bounded) => TerminalStatus::ProviderFailed(limit_failed()),
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
                        "swallowtail.openhands.agent_server.incomplete_stream",
                        "OpenHands Agent Server ended without a terminal execution status",
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

fn message_text(record: &Value) -> Result<String, RuntimeFailure> {
    let Some(content) = record
        .pointer("/llm_message/content")
        .and_then(Value::as_array)
    else {
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

fn malformed_stream() -> RuntimeFailure {
    failure(
        "swallowtail.openhands.agent_server.malformed_stream",
        "OpenHands Agent Server emitted a malformed event",
    )
}

fn wrong_wire() -> RuntimeFailure {
    failure(
        "swallowtail.openhands.agent_server.wrong_wire",
        "OpenHands Agent Server received ACP, Socket.IO, or non-event framing",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.openhands.agent_server.stream_limit",
        "OpenHands Agent Server exceeded the bounded event limit",
    )
}

fn output_limit() -> RuntimeFailure {
    failure(
        "swallowtail.openhands.agent_server.output_limit",
        "OpenHands Agent Server exceeded the bounded output limit",
    )
}

fn provider_failed() -> SafeDiagnostic {
    SafeDiagnostic::new(
        "swallowtail.openhands.agent_server.provider_failed",
        "OpenHands Agent Server reported a failed conversation",
    )
    .with_failure_classification(swallowtail_core::FailureClassification::new(
        swallowtail_core::FailureOrigin::Harness,
        swallowtail_core::FailureKind::Unknown,
        swallowtail_core::FailureRecovery::Unknown,
    ))
}

fn limit_failed() -> SafeDiagnostic {
    SafeDiagnostic::new(
        "swallowtail.openhands.agent_server.max_iterations",
        "OpenHands Agent Server reached the configured iteration limit",
    )
    .with_failure_classification(swallowtail_core::FailureClassification::new(
        swallowtail_core::FailureOrigin::Harness,
        swallowtail_core::FailureKind::Unknown,
        swallowtail_core::FailureRecovery::Unknown,
    ))
}

#[cfg(test)]
mod tests {
    use super::OpenHandsEventParser;
    use serde_json::Value;
    use swallowtail_runtime::{ActivityOperationId, ProcessExit, RuntimeRunId, TerminalStatus};

    const ACTIVITY: &str =
        include_str!("../../tests/fixtures/openhands-agent-server-1.42.1/activity.jsonl");
    const ABORT: &str =
        include_str!("../../tests/fixtures/openhands-agent-server-1.42.1/abort.jsonl");
    const LIMIT: &str =
        include_str!("../../tests/fixtures/openhands-agent-server-1.42.1/limit.jsonl");
    const ERROR: &str =
        include_str!("../../tests/fixtures/openhands-agent-server-1.42.1/error.jsonl");

    fn parser() -> OpenHandsEventParser {
        OpenHandsEventParser::new(ActivityOperationId::Run(
            RuntimeRunId::new("openhands-agent-server:fixture").expect("run id"),
        ))
    }

    fn push_jsonl(
        parser: &mut OpenHandsEventParser,
        body: &str,
    ) -> Result<Vec<swallowtail_runtime::RuntimeEvent>, swallowtail_runtime::RuntimeFailure> {
        let mut events = Vec::new();
        for line in body.lines().filter(|line| !line.is_empty()) {
            let value: Value = serde_json::from_str(line).expect("jsonl");
            events.extend(parser.push_event(&value)?);
        }
        Ok(events)
    }

    #[test]
    fn success_corpus_completes_without_native_leak() {
        let mut parser = parser();
        let events = push_jsonl(&mut parser, ACTIVITY).expect("success parses");
        assert!(events.iter().any(|event| {
            event
                .content()
                .is_some_and(|content| content.as_str().contains("OpenHands display text."))
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
            Some("OpenHands display text.")
        );
        assert!(!format!("{events:?}").contains("00000000-0000-4000-8000-000000000001"));
        assert!(!format!("{outcome:?}").contains("OpenHands display text"));
    }

    #[test]
    fn abort_corpus_cancels() {
        let mut parser = parser();
        push_jsonl(&mut parser, ABORT).expect("abort parses");
        let (_, terminal) = parser.finish().expect("finish");
        let (_, outcome) = terminal
            .finalize(ProcessExit::new(true, Some(0)))
            .expect("finalize");
        assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    }

    #[test]
    fn stuck_is_bounded_failure_not_end_turn() {
        let mut parser = parser();
        push_jsonl(&mut parser, LIMIT).expect("limit parses");
        let (_, terminal) = parser.finish().expect("finish");
        let (_, outcome) = terminal
            .finalize(ProcessExit::new(true, Some(0)))
            .expect("finalize");
        match outcome.status() {
            TerminalStatus::ProviderFailed(diagnostic) => {
                assert_eq!(
                    diagnostic.code(),
                    "swallowtail.openhands.agent_server.max_iterations"
                );
            }
            other => panic!("expected provider failed, got {other:?}"),
        }
    }

    #[test]
    fn error_kinds_fail_closed_without_native_detail() {
        let mut parser = parser();
        push_jsonl(&mut parser, ERROR).expect("error parses");
        let (_, terminal) = parser.finish().expect("finish");
        let (_, outcome) = terminal
            .finalize(ProcessExit::new(true, Some(0)))
            .expect("finalize");
        assert!(matches!(
            outcome.status(),
            TerminalStatus::ProviderFailed(_)
        ));
        assert!(!format!("{outcome:?}").contains("OpenHands agent error"));
    }

    #[test]
    fn acp_unknown_and_malformed_fail_closed() {
        let mut acp = parser();
        assert_eq!(
            acp.push_event(&serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize"}))
                .expect_err("acp")
                .diagnostic()
                .code(),
            "swallowtail.openhands.agent_server.wrong_wire"
        );
        let mut unknown = parser();
        assert!(
            unknown
                .push_event(&serde_json::json!({"kind":"NotARealEventKind"}))
                .is_err()
        );
        let mut missing = parser();
        assert!(
            missing
                .push_event(&serde_json::json!({"id":"opaque"}))
                .is_err()
        );
    }
}
