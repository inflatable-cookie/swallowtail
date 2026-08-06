use crate::activity::MuseActivityProjection;
use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_core::{ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityObservation, ActivityOperationId, CleanupOutcome, OperationContent, ProcessExit,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, TerminalOutcome, TerminalStatus,
};

pub(crate) const MAXIMUM_RECORD_BYTES: usize = 1024 * 1024;
pub(crate) const MAXIMUM_STREAM_BYTES: usize = 8 * 1024 * 1024;
pub(crate) const MAXIMUM_RECORDS: usize = 4096;
const MAXIMUM_UNKNOWN_PAYLOAD_BYTES: usize = 64 * 1024;
const MAXIMUM_OUTPUT_BYTES: usize = 256 * 1024;
const MAXIMUM_IDENTITY_BYTES: usize = 256;

pub(crate) struct MuseEventParser {
    pending: Vec<u8>,
    stream_bytes: usize,
    record_count: usize,
    expected_sequence: u64,
    runtime_sequence: u64,
    session_id: Option<String>,
    command_id: Option<String>,
    run_linked: bool,
    run_started: bool,
    model_seen: bool,
    expected_model: ModelId,
    linked_tasks: BTreeSet<String>,
    output: String,
    final_output: Option<OperationContent>,
    terminal: Option<TerminalStatus>,
    terminal_seen: bool,
    activity: MuseActivityProjection,
}

impl MuseEventParser {
    pub(crate) fn new(operation_id: ActivityOperationId, expected_model: ModelId) -> Self {
        Self {
            pending: Vec::new(),
            stream_bytes: 0,
            record_count: 0,
            expected_sequence: 1,
            runtime_sequence: 1,
            session_id: None,
            command_id: None,
            run_linked: false,
            run_started: false,
            model_seen: false,
            expected_model,
            linked_tasks: BTreeSet::new(),
            output: String::new(),
            final_output: None,
            terminal: None,
            terminal_seen: false,
            activity: MuseActivityProjection::new(operation_id),
        }
    }

    pub(crate) fn push(&mut self, bytes: &[u8]) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.stream_bytes = self
            .stream_bytes
            .checked_add(bytes.len())
            .ok_or_else(stream_limit)?;
        if self.stream_bytes > MAXIMUM_STREAM_BYTES {
            return Err(stream_limit());
        }
        self.pending.extend_from_slice(bytes);
        let mut events = Vec::new();
        while let Some(newline) = self.pending.iter().position(|byte| *byte == b'\n') {
            if newline > MAXIMUM_RECORD_BYTES {
                return Err(record_limit());
            }
            let line: Vec<_> = self.pending.drain(..=newline).collect();
            events.extend(self.parse_line(trim_newline(&line))?);
        }
        if self.pending.len() > MAXIMUM_RECORD_BYTES {
            return Err(record_limit());
        }
        Ok(events)
    }

    pub(crate) fn finish(mut self) -> Result<(Vec<RuntimeEvent>, ParsedTerminal), RuntimeFailure> {
        let mut events = Vec::new();
        if !self.pending.is_empty() {
            let line = std::mem::take(&mut self.pending);
            events.extend(self.parse_line(&line)?);
        }
        if !self.run_linked || !self.run_started || !self.model_seen || !self.terminal_seen {
            return Ok((
                events,
                ParsedTerminal::incomplete(self.final_output, self.terminal),
            ));
        }
        Ok((
            events,
            ParsedTerminal::complete(self.final_output, self.terminal),
        ))
    }

    fn parse_line(&mut self, line: &[u8]) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if line.is_empty() || line.iter().all(u8::is_ascii_whitespace) {
            return Err(malformed());
        }
        self.record_count = self.record_count.checked_add(1).ok_or_else(record_limit)?;
        if self.record_count > MAXIMUM_RECORDS {
            return Err(record_limit());
        }
        let record: Value = serde_json::from_slice(line).map_err(|_| malformed())?;
        self.validate_envelope(&record)?;
        let payload_type = required_text(&record, "payload_type")?;
        if self.terminal_seen && is_known_payload_type(payload_type) {
            return Err(post_terminal());
        }
        let payload = record.get("payload").ok_or_else(malformed)?;
        let event_id = bounded_identity(required_text(&record, "id")?)?;

        match payload_type {
            "runtime.command.accepted" => {
                let events = self.command_accepted(payload)?;
                if required_text(&record, "causation_id")?
                    != self.command_id.as_deref().expect("command was accepted")
                {
                    return Err(causation_mismatch());
                }
                Ok(events)
            }
            "session.run.linked" => self.run_linked(payload),
            "run.model.configured" => self.model_configured(payload),
            "turn.input.user" => self.ordinary_run_payload(payload).map(|()| Vec::new()),
            "run.lifecycle.started" => self.run_started(payload),
            "run.output.delta" => self.output_delta(payload),
            "run.terminal.completed" | "run.terminal.failed" | "run.terminal.cancelled" => {
                self.terminal(payload_type, payload)
            }
            "task.stream.linked" => self.task_linked(payload),
            value if value.starts_with("task.lifecycle.") => self.task_lifecycle(value, payload),
            value => self.unknown(event_id, value, payload),
        }
    }

    fn validate_envelope(&mut self, record: &Value) -> Result<(), RuntimeFailure> {
        if record.get("schema_version").and_then(Value::as_u64) != Some(1)
            || record.get("payload_schema_version").and_then(Value::as_u64) != Some(1)
            || !record.get("recorded_at").is_some_and(Value::is_u64)
            || !matches!(
                required_text(record, "record_type")?,
                "reconciliation" | "event" | "status"
            )
            || !matches!(
                required_text(record, "durability")?,
                "durable" | "ephemeral"
            )
        {
            return Err(malformed());
        }
        let sequence = record
            .get("sequence")
            .and_then(Value::as_u64)
            .ok_or_else(malformed)?;
        if sequence != self.expected_sequence {
            return Err(sequence_mismatch());
        }
        self.expected_sequence = self
            .expected_sequence
            .checked_add(1)
            .ok_or_else(sequence_mismatch)?;

        let stream = record.get("stream").ok_or_else(malformed)?;
        if required_text(stream, "kind")? != "session" {
            return Err(session_mismatch());
        }
        let session_id = bounded_identity(required_text(stream, "id")?)?;
        match &self.session_id {
            Some(expected) if expected != session_id => return Err(session_mismatch()),
            None => self.session_id = Some(session_id.to_owned()),
            _ => {}
        }
        let causation = bounded_identity(required_text(record, "causation_id")?)?;
        if let Some(command_id) = &self.command_id
            && command_id != causation
        {
            return Err(causation_mismatch());
        }
        Ok(())
    }

    fn command_accepted(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if self.record_count != 1 || self.command_id.is_some() {
            return Err(malformed());
        }
        let command_id = bounded_identity(required_text(payload, "command_id")?)?;
        if required_text(payload, "command_kind")? != "turn.submit" {
            return Err(malformed());
        }
        self.command_id = Some(command_id.to_owned());
        Ok(Vec::new())
    }

    fn run_linked(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if self.run_linked {
            return Err(malformed());
        }
        self.ordinary_run_payload(payload)?;
        self.run_linked = true;
        Ok(Vec::new())
    }

    fn model_configured(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if !self.run_linked || self.model_seen {
            return Err(malformed());
        }
        self.ordinary_run_payload(payload)?;
        if required_text(payload, "provider_id")? != "meta"
            || required_text(payload, "model_id")? != self.expected_model.as_str()
            || bounded_identity(required_text(payload, "profile_id")?).is_err()
        {
            return Err(model_mismatch());
        }
        self.model_seen = true;
        Ok(Vec::new())
    }

    fn run_started(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if !self.run_linked || self.run_started {
            return Err(malformed());
        }
        self.ordinary_run_payload(payload)?;
        self.run_started = true;
        Ok(Vec::new())
    }

    fn output_delta(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if !self.run_started {
            return Err(malformed());
        }
        self.ordinary_run_payload(payload)?;
        let text = required_text(payload, "text")?;
        if self.output.len().saturating_add(text.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(output_limit());
        }
        self.output.push_str(text);
        let content = OperationContent::new(text).map_err(|_| malformed())?;
        Ok(vec![self.runtime_event_with(
            RuntimeEventKind::OutputDelta,
            content,
        )])
    }

    fn task_linked(&mut self, payload: &Value) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.ordinary_run_payload(payload)?;
        let task_id = bounded_identity(required_text(payload, "task_id")?)?;
        let stream = payload.get("task_stream").ok_or_else(malformed)?;
        if required_text(stream, "kind")? != "task"
            || required_text(stream, "id")? != task_id
            || !self.linked_tasks.insert(task_id.to_owned())
        {
            return Err(task_mismatch());
        }
        Ok(Vec::new())
    }

    fn task_lifecycle(
        &mut self,
        payload_type: &str,
        payload: &Value,
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.ordinary_run_payload(payload)?;
        let task_id = bounded_identity(required_text(payload, "task_id")?)?;
        if !self.linked_tasks.contains(task_id) {
            return Err(task_mismatch());
        }
        let task_stream = payload.get("task_stream").ok_or_else(malformed)?;
        let event = payload.get("event").ok_or_else(malformed)?;
        let lifecycle = payload_type
            .strip_prefix("task.lifecycle.")
            .ok_or_else(malformed)?;
        if required_text(task_stream, "kind")? != "task"
            || required_text(task_stream, "id")? != task_id
            || required_text(event, "kind")? != lifecycle
            || required_text(event, "task_id")? != task_id
        {
            return Err(task_mismatch());
        }
        let observations = self.activity.task(task_id, lifecycle)?;
        if matches!(
            lifecycle,
            "completed" | "failed" | "cancelled" | "timed_out" | "rejected"
        ) {
            self.linked_tasks.remove(task_id);
        }
        Ok(self.activity_events(observations))
    }

    fn unknown(
        &mut self,
        event_id: &str,
        payload_type: &str,
        payload: &Value,
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        let bytes = serde_json::to_vec(payload).map_err(|_| malformed())?;
        if bytes.len() > MAXIMUM_UNKNOWN_PAYLOAD_BYTES {
            return Err(unknown_limit());
        }
        let observation = self.activity.unknown(event_id, payload_type)?;
        Ok(vec![
            self.runtime_event(RuntimeEventKind::Activity(observation)),
        ])
    }

    fn terminal(
        &mut self,
        payload_type: &str,
        payload: &Value,
    ) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        if !self.run_started {
            return Err(malformed());
        }
        self.ordinary_run_payload(payload)?;
        let terminal = required_text(payload, "terminal")?;
        let expected_terminal = payload_type
            .strip_prefix("run.terminal.")
            .ok_or_else(malformed)?;
        if terminal != expected_terminal {
            return Err(malformed());
        }
        let status = match terminal {
            "completed" => TerminalStatus::Completed,
            "cancelled" => TerminalStatus::Cancelled,
            "failed" => TerminalStatus::ProviderFailed(
                SafeDiagnostic::new(
                    "swallowtail.muse_code.headless.provider_failed",
                    "Muse Code reported a failed headless run",
                )
                .with_failure_classification(
                    swallowtail_core::FailureClassification::new(
                        swallowtail_core::FailureOrigin::Provider,
                        swallowtail_core::FailureKind::Unknown,
                        swallowtail_core::FailureRecovery::Unknown,
                    ),
                ),
            ),
            _ => return Err(malformed()),
        };
        let text = payload
            .get("text")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if text.len() > MAXIMUM_OUTPUT_BYTES {
            return Err(output_limit());
        }
        let mut events = Vec::new();
        if status == TerminalStatus::Completed {
            if !self.output.is_empty() && self.output != text {
                return Err(output_mismatch());
            }
            let output = OperationContent::new(text).map_err(|_| malformed())?;
            self.final_output = Some(output.clone());
            events.push(self.runtime_event_with(RuntimeEventKind::OutputAvailable, output));
        }
        let observations = self.activity.complete(&status)?;
        events.extend(self.activity_events(observations));
        self.terminal = Some(status);
        self.terminal_seen = true;
        Ok(events)
    }

    fn ordinary_run_payload(&self, payload: &Value) -> Result<(), RuntimeFailure> {
        let command_id = self.command_id.as_deref().ok_or_else(malformed)?;
        if required_text(payload, "command_id")? != command_id {
            return Err(causation_mismatch());
        }
        let stream = payload.get("run_stream").ok_or_else(malformed)?;
        if required_text(stream, "kind")? != "run" || required_text(stream, "id")? != command_id {
            return Err(run_mismatch());
        }
        Ok(())
    }

    fn runtime_event(&mut self, kind: RuntimeEventKind) -> RuntimeEvent {
        let sequence = self.runtime_sequence;
        self.runtime_sequence = self.runtime_sequence.saturating_add(1);
        RuntimeEvent::new(sequence, kind)
    }

    fn runtime_event_with(
        &mut self,
        kind: RuntimeEventKind,
        content: OperationContent,
    ) -> RuntimeEvent {
        let sequence = self.runtime_sequence;
        self.runtime_sequence = self.runtime_sequence.saturating_add(1);
        RuntimeEvent::with_content(sequence, kind, content)
    }

    fn activity_events(
        &mut self,
        observations: impl IntoIterator<Item = ActivityObservation>,
    ) -> Vec<RuntimeEvent> {
        observations
            .into_iter()
            .map(|observation| self.runtime_event(RuntimeEventKind::Activity(observation)))
            .collect()
    }
}

fn is_known_payload_type(payload_type: &str) -> bool {
    matches!(
        payload_type,
        "runtime.command.accepted"
            | "session.run.linked"
            | "run.model.configured"
            | "turn.input.user"
            | "run.lifecycle.started"
            | "run.output.delta"
            | "run.terminal.completed"
            | "run.terminal.failed"
            | "run.terminal.cancelled"
            | "task.stream.linked"
    ) || payload_type.starts_with("task.lifecycle.")
}

pub(crate) struct ParsedTerminal {
    output: Option<OperationContent>,
    status: Option<TerminalStatus>,
    complete: bool,
}

impl ParsedTerminal {
    fn complete(output: Option<OperationContent>, status: Option<TerminalStatus>) -> Self {
        Self {
            output,
            status,
            complete: true,
        }
    }

    fn incomplete(output: Option<OperationContent>, status: Option<TerminalStatus>) -> Self {
        Self {
            output,
            status,
            complete: false,
        }
    }

    pub(crate) fn outcome(self, exit: ProcessExit) -> TerminalOutcome {
        let status = if !self.complete {
            TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                "swallowtail.muse_code.headless.incomplete_stream",
                "Muse Code stream ended without its complete correlated lifecycle",
            ))
        } else if !exit.success() && self.status == Some(TerminalStatus::Completed) {
            TerminalStatus::ProviderFailed(
                SafeDiagnostic::new(
                    "swallowtail.muse_code.headless.process_failed",
                    match exit.code() {
                        Some(code) => format!("Muse Code exited with status {code}"),
                        None => "Muse Code exited unsuccessfully".to_owned(),
                    },
                )
                .with_failure_classification(
                    swallowtail_core::FailureClassification::new(
                        swallowtail_core::FailureOrigin::Harness,
                        swallowtail_core::FailureKind::Unknown,
                        swallowtail_core::FailureRecovery::Unknown,
                    ),
                ),
            )
        } else {
            self.status.unwrap_or_else(|| {
                TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                    "swallowtail.muse_code.headless.invalid_terminal",
                    "Muse Code terminal status could not be classified",
                ))
            })
        };
        let outcome = TerminalOutcome::new(status, CleanupOutcome::Clean);
        match self.output {
            Some(output) => outcome.with_output(output),
            None => outcome,
        }
    }
}

fn required_text<'a>(value: &'a Value, key: &str) -> Result<&'a str, RuntimeFailure> {
    value.get(key).and_then(Value::as_str).ok_or_else(malformed)
}

fn bounded_identity(value: &str) -> Result<&str, RuntimeFailure> {
    if value.trim().is_empty()
        || value.len() > MAXIMUM_IDENTITY_BYTES
        || value.chars().any(char::is_control)
    {
        Err(malformed())
    } else {
        Ok(value)
    }
}

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn malformed() -> RuntimeFailure {
    fail(
        "malformed_stream",
        "Muse Code emitted malformed headless JSONL",
    )
}

fn record_limit() -> RuntimeFailure {
    fail(
        "record_limit",
        "Muse Code exceeded the headless record bound",
    )
}

fn stream_limit() -> RuntimeFailure {
    fail(
        "stream_limit",
        "Muse Code exceeded the headless stream bound",
    )
}

fn unknown_limit() -> RuntimeFailure {
    fail(
        "unknown_payload_limit",
        "Muse Code emitted an oversized unknown event payload",
    )
}

fn output_limit() -> RuntimeFailure {
    fail(
        "output_limit",
        "Muse Code exceeded the headless output bound",
    )
}

fn sequence_mismatch() -> RuntimeFailure {
    fail(
        "sequence_mismatch",
        "Muse Code event sequence is not contiguous",
    )
}

fn session_mismatch() -> RuntimeFailure {
    fail(
        "session_mismatch",
        "Muse Code event belongs to another session",
    )
}

fn causation_mismatch() -> RuntimeFailure {
    fail(
        "causation_mismatch",
        "Muse Code event belongs to another command",
    )
}

fn run_mismatch() -> RuntimeFailure {
    fail("run_mismatch", "Muse Code event belongs to another run")
}

fn task_mismatch() -> RuntimeFailure {
    fail(
        "task_mismatch",
        "Muse Code event belongs to an unknown task",
    )
}

fn model_mismatch() -> RuntimeFailure {
    fail(
        "model_mismatch",
        "Muse Code configured a different provider or model",
    )
}

fn output_mismatch() -> RuntimeFailure {
    fail(
        "output_mismatch",
        "Muse Code terminal output disagrees with its deltas",
    )
}

fn post_terminal() -> RuntimeFailure {
    fail(
        "post_terminal",
        "Muse Code emitted activity after the terminal event",
    )
}

fn fail(suffix: &'static str, message: &'static str) -> RuntimeFailure {
    crate::failure::failure(
        match suffix {
            "malformed_stream" => "swallowtail.muse_code.headless.malformed_stream",
            "record_limit" => "swallowtail.muse_code.headless.record_limit",
            "stream_limit" => "swallowtail.muse_code.headless.stream_limit",
            "unknown_payload_limit" => "swallowtail.muse_code.headless.unknown_payload_limit",
            "output_limit" => "swallowtail.muse_code.headless.output_limit",
            "sequence_mismatch" => "swallowtail.muse_code.headless.sequence_mismatch",
            "session_mismatch" => "swallowtail.muse_code.headless.session_mismatch",
            "causation_mismatch" => "swallowtail.muse_code.headless.causation_mismatch",
            "run_mismatch" => "swallowtail.muse_code.headless.run_mismatch",
            "task_mismatch" => "swallowtail.muse_code.headless.task_mismatch",
            "model_mismatch" => "swallowtail.muse_code.headless.model_mismatch",
            "output_mismatch" => "swallowtail.muse_code.headless.output_mismatch",
            "post_terminal" => "swallowtail.muse_code.headless.post_terminal",
            _ => "swallowtail.muse_code.headless.invalid_stream",
        },
        message,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use swallowtail_runtime::{ActivityKind, RuntimeRunId};

    const META: &str = include_str!("../tests/fixtures/muse-code-0.1.0-R708.1/meta-success.jsonl");
    const ECHO: &str = include_str!("../tests/fixtures/muse-code-0.1.0-R708.1/echo-success.jsonl");
    const UNKNOWN: &str =
        include_str!("../tests/fixtures/muse-code-0.1.0-R708.1/unknown-event.jsonl");

    fn fixture_parser() -> MuseEventParser {
        MuseEventParser::new(
            ActivityOperationId::Run(RuntimeRunId::new("muse-fixture-run").unwrap()),
            ModelId::new(crate::MUSE_SPARK_MODEL_ID).unwrap(),
        )
    }

    #[test]
    fn exact_meta_projection_completes_with_correlated_output() {
        let mut parser = fixture_parser();
        let mut events = Vec::new();
        for chunk in META.as_bytes().chunks(37) {
            events.extend(parser.push(chunk).expect("chunk parses"));
        }
        let (trailing, terminal) = parser.finish().expect("stream finishes");
        events.extend(trailing);
        let outcome = terminal.outcome(ProcessExit::new(true, Some(0)));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            outcome.output().expect("output").as_str(),
            "MUSE_FIXTURE_OK"
        );
        assert!(
            events
                .iter()
                .any(|event| { matches!(event.kind(), RuntimeEventKind::OutputDelta) })
        );
        assert!(events.iter().any(|event| {
            matches!(
                event.kind(),
                RuntimeEventKind::Activity(observation)
                    if matches!(observation.kind(), ActivityKind::Unknown(namespace)
                        if namespace.as_str()
                            == "muse-code.headless.event.session.workspace_branch.observed")
            )
        }));
    }

    #[test]
    fn bounded_unknown_is_namespaced_without_terminal_authority() {
        let mut parser = fixture_parser();
        let events = parser.push(UNKNOWN.as_bytes()).expect("unknown parses");
        let (_, terminal) = parser.finish().expect("stream finishes");
        assert_eq!(
            terminal.outcome(ProcessExit::new(true, Some(0))).status(),
            &TerminalStatus::Completed
        );
        let unknown = events.iter().find_map(|event| match event.kind() {
            RuntimeEventKind::Activity(observation)
                if matches!(observation.kind(), ActivityKind::Unknown(_)) =>
            {
                Some(observation)
            }
            _ => None,
        });
        assert!(unknown.is_some());
    }

    #[test]
    fn exact_task_lifecycle_projects_without_claiming_a_task_list() {
        let mut parser = fixture_parser();
        let events = parser.push(ECHO.as_bytes()).expect("echo lifecycle parses");
        let tasks = events
            .iter()
            .filter_map(|event| match event.kind() {
                RuntimeEventKind::Activity(observation)
                    if observation.kind() == &ActivityKind::Task =>
                {
                    Some(observation)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(tasks.len() >= 9);
        assert!(tasks.iter().all(|task| task.task_list().is_none()));
        assert!(
            tasks
                .iter()
                .all(|task| task.provider_activity_ref().is_some())
        );
    }

    #[test]
    fn reordered_cross_session_post_terminal_and_model_drift_fail_closed() {
        let lines = META.lines().collect::<Vec<_>>();
        let cases = [
            lines
                .iter()
                .enumerate()
                .map(|(index, line)| {
                    if index == 1 {
                        line.replace("\"sequence\":2", "\"sequence\":9")
                    } else {
                        (*line).to_owned()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
                + "\n",
            META.replacen("fixture-session-meta", "fixture-session-foreign", 1),
            format!("{META}{}\n", lines[5]),
            META.replace("muse-spark-1.2", "muse-spark-foreign"),
        ];
        for case in cases {
            let mut parser = fixture_parser();
            assert!(parser.push(case.as_bytes()).is_err());
        }
    }

    #[test]
    fn record_stream_and_unknown_payload_bounds_fail_closed() {
        let mut parser = fixture_parser();
        assert!(parser.push(&vec![b'x'; MAXIMUM_RECORD_BYTES + 1]).is_err());

        let mut parser = fixture_parser();
        assert!(parser.push(&vec![b'x'; MAXIMUM_STREAM_BYTES + 1]).is_err());

        let oversized = UNKNOWN.replace(
            "bounded fixture notice",
            &"x".repeat(MAXIMUM_UNKNOWN_PAYLOAD_BYTES),
        );
        let mut parser = fixture_parser();
        assert!(parser.push(oversized.as_bytes()).is_err());
    }
}
