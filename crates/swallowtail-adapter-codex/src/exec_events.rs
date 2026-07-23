use serde_json::Value;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, OperationContent, ProviderObservation, RuntimeEvent, RuntimeEventKind,
    RuntimeFailure, TerminalOutcome, TerminalStatus, TokenUsage,
};

pub(crate) struct ExecEventParser {
    pending: Vec<u8>,
    sequence: u64,
    final_output: Option<OperationContent>,
    provider_failure: Option<SafeDiagnostic>,
    completed: bool,
}

impl ExecEventParser {
    pub(crate) fn new() -> Self {
        Self {
            pending: Vec::new(),
            sequence: 1,
            final_output: None,
            provider_failure: None,
            completed: false,
        }
    }

    pub(crate) fn push(&mut self, bytes: &[u8]) -> Result<Vec<RuntimeEvent>, RuntimeFailure> {
        self.pending.extend_from_slice(bytes);
        let mut events = Vec::new();
        while let Some(newline) = self.pending.iter().position(|byte| *byte == b'\n') {
            let line: Vec<_> = self.pending.drain(..=newline).collect();
            if let Some(event) = self.parse_line(trim_newline(&line))? {
                events.push(event);
            }
        }
        Ok(events)
    }

    pub(crate) fn finish(mut self) -> Result<(Vec<RuntimeEvent>, ParsedTerminal), RuntimeFailure> {
        let mut events = Vec::new();
        if !self.pending.is_empty() {
            let line = std::mem::take(&mut self.pending);
            if let Some(event) = self.parse_line(&line)? {
                events.push(event);
            }
        }
        Ok((
            events,
            ParsedTerminal {
                final_output: self.final_output,
                provider_failure: self.provider_failure,
                completed: self.completed,
            },
        ))
    }

    fn parse_line(&mut self, line: &[u8]) -> Result<Option<RuntimeEvent>, RuntimeFailure> {
        if line.iter().all(u8::is_ascii_whitespace) {
            return Ok(None);
        }
        let payload: Value = serde_json::from_slice(line).map_err(|_| malformed_stream())?;
        let event_type = payload
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;

        match event_type {
            "turn.completed" => {
                self.completed = true;
                Ok(token_usage(&payload).map(|usage| {
                    self.event(RuntimeEventKind::ProviderObservation(
                        ProviderObservation::Usage(usage),
                    ))
                }))
            }
            "turn.failed" | "error" => {
                self.provider_failure = Some(SafeDiagnostic::new(
                    "swallowtail.codex.exec.provider_failed",
                    "Codex exec reported a provider failure",
                ));
                Ok(None)
            }
            "item.completed" => {
                let item = payload.get("item").ok_or_else(malformed_stream)?;
                self.parse_item(item)
            }
            _ => Ok(Some(self.event(RuntimeEventKind::Progress))),
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

    fn parse_item(&mut self, item: &Value) -> Result<Option<RuntimeEvent>, RuntimeFailure> {
        match item.get("type").and_then(Value::as_str) {
            Some("web_search") => {
                let query = item
                    .get("query")
                    .and_then(Value::as_str)
                    .or_else(|| item.pointer("/action/query").and_then(Value::as_str));
                Ok(query
                    .and_then(|query| OperationContent::new(query).ok())
                    .map(|content| {
                        self.event_with(RuntimeEventKind::ExternalSearchProgress, content)
                    }))
            }
            Some("reasoning") => {
                let summary = item
                    .get("text")
                    .or_else(|| item.get("summary"))
                    .and_then(Value::as_str);
                Ok(summary
                    .and_then(|summary| OperationContent::new(summary).ok())
                    .map(|content| self.event_with(RuntimeEventKind::ReasoningProgress, content))
                    .or_else(|| Some(self.event(RuntimeEventKind::ReasoningProgress))))
            }
            Some("agent_message") => {
                let text = item
                    .get("text")
                    .and_then(Value::as_str)
                    .ok_or_else(malformed_stream)?;
                let content = OperationContent::new(text).map_err(|_| malformed_stream())?;
                self.final_output = Some(content.clone());
                if serde_json::from_str::<Value>(text).is_ok() {
                    Ok(Some(
                        self.event_with(RuntimeEventKind::OutputAvailable, content),
                    ))
                } else {
                    Ok(Some(self.event_with(RuntimeEventKind::Progress, content)))
                }
            }
            _ => Ok(Some(self.event(RuntimeEventKind::Progress))),
        }
    }
}

fn token_usage(payload: &Value) -> Option<TokenUsage> {
    let input = payload
        .pointer("/usage/input_tokens")
        .and_then(Value::as_u64)?;
    let output = payload
        .pointer("/usage/output_tokens")
        .and_then(Value::as_u64)?;
    Some(TokenUsage::new(Some(input), Some(output)))
}

pub(crate) struct ParsedTerminal {
    final_output: Option<OperationContent>,
    provider_failure: Option<SafeDiagnostic>,
    completed: bool,
}

impl ParsedTerminal {
    pub(crate) fn outcome(self, process_succeeded: bool) -> TerminalOutcome {
        let status = if let Some(failure) = self.provider_failure {
            TerminalStatus::ProviderFailed(failure)
        } else if !process_succeeded {
            TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                "swallowtail.codex.exec.process_failed",
                "Codex exec exited unsuccessfully",
            ))
        } else if !self.completed {
            TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                "swallowtail.codex.exec.incomplete_stream",
                "Codex exec ended without a completion event",
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

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn malformed_stream() -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.codex.exec.malformed_jsonl",
        "Codex exec returned malformed structured output",
    ))
}

#[cfg(test)]
#[path = "exec_events_tests.rs"]
mod tests;
