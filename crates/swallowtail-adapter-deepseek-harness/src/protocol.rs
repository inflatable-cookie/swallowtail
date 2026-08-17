use crate::activity::DeepSeekHarnessActivityProjection;
use crate::failure::failure;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ActivityOperationId, CleanupOutcome, OperationContent, ProcessExit, ProviderObservation,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, TerminalOutcome, TerminalStatus, TokenUsage,
};

const MAXIMUM_FRAME_BYTES: usize = 64 * 1024;
const MAXIMUM_LIVE_RECORDS: usize = 8192;
const MAXIMUM_STREAM_BYTES: usize = 8 * 1024 * 1024;
const MAXIMUM_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_IDENTIFIER_BYTES: usize = 256;

const KNOWN_EVENT_TYPES: &[&str] = &[
    "turn/start",
    "turn/end",
    "step/start",
    "step/end",
    "user/message",
    "request/header",
    "request/context",
    "assistant/chunk",
    "assistant/message",
    "tool/call",
    "tool/result",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RequestKind {
    Initialize,
    Prompt,
    Shutdown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    AwaitInitialize,
    AwaitPrompt,
    Running,
    Terminal,
    AwaitShutdown,
    Complete,
}

pub(crate) struct ParserOutput {
    pub(crate) events: Vec<RuntimeEvent>,
    pub(crate) writes: Vec<Vec<u8>>,
}

pub(crate) struct JsonRpcParser {
    pending: Vec<u8>,
    stream_bytes: usize,
    record_count: usize,
    runtime_sequence: u64,
    event_sequence: u64,
    phase: Phase,
    pending_request: Option<(i64, RequestKind)>,
    cwd: String,
    provider: String,
    model: String,
    prompt: String,
    session_id: String,
    turn_id: Option<String>,
    step_id: Option<String>,
    step_ended: bool,
    status_running: bool,
    status_idle: bool,
    terminal: Option<TerminalStatus>,
    output_text: String,
    streamed_text: String,
    tool_calls: BTreeSet<String>,
    tool_results: BTreeSet<String>,
    finish_seen: bool,
    usage_seen: bool,
    activity: DeepSeekHarnessActivityProjection,
}

impl JsonRpcParser {
    pub(crate) fn new(
        operation_id: ActivityOperationId,
        cwd: String,
        provider: String,
        model: String,
        prompt: String,
        session_id: String,
    ) -> Self {
        Self {
            pending: Vec::new(),
            stream_bytes: 0,
            record_count: 0,
            runtime_sequence: 1,
            event_sequence: 0,
            phase: Phase::AwaitInitialize,
            pending_request: None,
            cwd,
            provider,
            model,
            prompt,
            session_id,
            turn_id: None,
            step_id: None,
            step_ended: true,
            status_running: false,
            status_idle: false,
            terminal: None,
            output_text: String::new(),
            streamed_text: String::new(),
            tool_calls: BTreeSet::new(),
            tool_results: BTreeSet::new(),
            finish_seen: false,
            usage_seen: false,
            activity: DeepSeekHarnessActivityProjection::new(operation_id),
        }
    }

    pub(crate) fn initialize_request(&mut self) -> Result<Vec<u8>, RuntimeFailure> {
        require(
            self.pending_request.is_none(),
            "initialize request already sent",
        )?;
        self.pending_request = Some((1, RequestKind::Initialize));
        request(
            1,
            "initialize",
            json!({
                "cwd": self.cwd,
                "provider": self.provider,
                "model": self.model
            }),
        )
    }

    pub(crate) fn push(&mut self, bytes: &[u8]) -> Result<ParserOutput, RuntimeFailure> {
        self.stream_bytes = self
            .stream_bytes
            .checked_add(bytes.len())
            .ok_or_else(stream_limit)?;
        if self.stream_bytes > MAXIMUM_STREAM_BYTES {
            return Err(stream_limit());
        }
        self.pending.extend_from_slice(bytes);
        if self.pending.len() > MAXIMUM_FRAME_BYTES && !self.pending.contains(&b'\n') {
            return Err(frame_limit());
        }
        let mut output = ParserOutput {
            events: Vec::new(),
            writes: Vec::new(),
        };
        while let Some(newline) = self.pending.iter().position(|byte| *byte == b'\n') {
            if newline > MAXIMUM_FRAME_BYTES {
                return Err(frame_limit());
            }
            let line: Vec<u8> = self.pending.drain(..=newline).collect();
            let parsed = self.parse_line(trim_newline(&line))?;
            output.events.extend(parsed.events);
            output.writes.extend(parsed.writes);
        }
        if self.pending.len() > MAXIMUM_FRAME_BYTES {
            return Err(frame_limit());
        }
        Ok(output)
    }

    pub(crate) fn finish(mut self, exit: ProcessExit) -> Result<ParsedTerminal, RuntimeFailure> {
        if !self.pending.is_empty() {
            let line = std::mem::take(&mut self.pending);
            return Err(if line.iter().all(u8::is_ascii_whitespace) {
                malformed_stream()
            } else {
                incomplete_stream()
            });
        }
        require(
            self.phase == Phase::Complete
                && self.pending_request.is_none()
                && self.status_idle
                && self.terminal.is_some(),
            "JSON-RPC stream ended before shutdown and idle completion",
        )?;
        let status = if !exit.success() && self.terminal == Some(TerminalStatus::Completed) {
            TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                "swallowtail.deepseek_harness.process_failed",
                "DeepSeek Harness runtime exited unsuccessfully after a completed turn",
            ))
        } else {
            self.terminal.take().ok_or_else(incomplete_stream)?
        };
        let output = if self.output_text.is_empty() {
            None
        } else {
            Some(OperationContent::new(&self.output_text).map_err(|_| output_limit())?)
        };
        Ok(ParsedTerminal { status, output })
    }

    fn parse_line(&mut self, line: &[u8]) -> Result<ParserOutput, RuntimeFailure> {
        if line.is_empty() || line.iter().all(u8::is_ascii_whitespace) {
            return Err(malformed_stream());
        }
        self.record_count = self.record_count.checked_add(1).ok_or_else(record_limit)?;
        if self.record_count > MAXIMUM_LIVE_RECORDS {
            return Err(record_limit());
        }
        let frame: Value = serde_json::from_slice(line).map_err(|_| malformed_stream())?;
        require(
            frame.get("jsonrpc").and_then(Value::as_str) == Some("2.0"),
            "invalid JSON-RPC version",
        )?;
        if frame.get("method").is_some() {
            require(
                frame.get("id").is_none(),
                "server request is outside the admitted protocol",
            )?;
            self.notification(&frame)
        } else {
            require(
                frame.get("id").and_then(Value::as_i64).is_some(),
                "response id is invalid",
            )?;
            self.response(&frame)
        }
    }

    fn response(&mut self, frame: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let id = frame
            .get("id")
            .and_then(Value::as_i64)
            .ok_or_else(malformed_stream)?;
        let (expected_id, kind) = self.pending_request.take().ok_or_else(unmatched_response)?;
        require(
            id == expected_id,
            "JSON-RPC response id does not match the pending request",
        )?;
        if frame.get("error").is_some() {
            return Err(provider_rpc_error());
        }
        let result = frame.get("result").ok_or_else(malformed_stream)?;
        match kind {
            RequestKind::Initialize => {
                require(
                    self.phase == Phase::AwaitInitialize,
                    "duplicate initialize response",
                )?;
                let server_info = result
                    .get("serverInfo")
                    .and_then(Value::as_object)
                    .ok_or_else(malformed_stream)?;
                require(
                    server_info.get("name").and_then(Value::as_str)
                        == Some("deepseek-harness-sdk-runtime"),
                    "initialize server name does not match the expected runtime",
                )?;
                require(
                    server_info.get("version").and_then(Value::as_str).is_some(),
                    "initialize server version is malformed",
                )?;
                self.phase = Phase::AwaitPrompt;
                self.pending_request = Some((2, RequestKind::Prompt));
                Ok(ParserOutput {
                    events: Vec::new(),
                    writes: vec![request(
                        2,
                        "session/prompt",
                        json!({
                            "sessionId": self.session_id,
                            "contentBlocks": [{"type":"text", "text": self.prompt}]
                        }),
                    )?],
                })
            }
            RequestKind::Prompt => {
                require(
                    self.phase == Phase::AwaitPrompt,
                    "duplicate prompt response",
                )?;
                require(
                    result
                        .get("messageId")
                        .and_then(Value::as_str)
                        .is_some_and(|value| !value.is_empty()),
                    "prompt response is not an enqueue receipt",
                )?;
                self.phase = Phase::Running;
                Ok(ParserOutput {
                    events: Vec::new(),
                    writes: Vec::new(),
                })
            }
            RequestKind::Shutdown => {
                require(
                    self.phase == Phase::AwaitShutdown,
                    "unexpected shutdown response",
                )?;
                require(
                    result.as_object().is_some_and(|object| object.is_empty()),
                    "shutdown result is not empty",
                )?;
                self.phase = Phase::Complete;
                Ok(ParserOutput {
                    events: Vec::new(),
                    writes: Vec::new(),
                })
            }
        }
    }

    fn notification(&mut self, frame: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let method = frame
            .get("method")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match method {
            "session.status" => self.status(frame.get("params").ok_or_else(malformed_stream)?),
            "session.event" => {
                self.session_event(frame.get("params").ok_or_else(malformed_stream)?)
            }
            "subagent.started" | "subagent.finished" => self.subagent(method),
            _ => Err(unsupported_notification()),
        }
    }

    fn status(&mut self, params: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let params = params.as_object().ok_or_else(malformed_stream)?;
        self.require_session(params.get("sessionId").and_then(Value::as_str))?;
        let status = params
            .get("status")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match status {
            "running" => {
                require(
                    self.phase == Phase::Running && !self.status_running,
                    "invalid running status",
                )?;
                self.status_running = true;
                Ok(self.progress())
            }
            "idle" => {
                require(
                    self.phase == Phase::Terminal
                        && self.status_running
                        && !self.status_idle
                        && self.terminal.is_some(),
                    "idle arrived before terminal turn/end",
                )?;
                self.status_idle = true;
                self.phase = Phase::AwaitShutdown;
                self.pending_request = Some((3, RequestKind::Shutdown));
                Ok(ParserOutput {
                    events: vec![self.event(RuntimeEventKind::Progress)],
                    writes: vec![request(3, "shutdown", json!({}))?],
                })
            }
            _ => Err(malformed_stream()),
        }
    }

    fn session_event(&mut self, params: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let params = params.as_object().ok_or_else(malformed_stream)?;
        self.require_session(params.get("sessionId").and_then(Value::as_str))?;
        require(
            self.status_running,
            "session event arrived before running status",
        )?;
        require(
            self.terminal.is_none(),
            "session event arrived after terminal",
        )?;
        let event = params
            .get("event")
            .and_then(Value::as_object)
            .ok_or_else(malformed_stream)?;
        let event_type = event
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        let sequence = event
            .get("seq")
            .and_then(Value::as_u64)
            .ok_or_else(malformed_stream)?;
        require(
            sequence == self.event_sequence + 1,
            "session event sequence is not contiguous",
        )?;
        self.event_sequence = sequence;
        require(
            event.get("time").is_some_and(|value| {
                value.as_i64().is_some() || value.as_u64().is_some() || value.as_f64().is_some()
            }),
            "session event time is invalid",
        )?;
        if !KNOWN_EVENT_TYPES.contains(&event_type) {
            require(
                event_type.starts_with("deepseek-harness/")
                    && event.get("ignorable") == Some(&Value::Bool(true)),
                "unknown event is not an explicit namespaced observation",
            )?;
            let observation = self.activity.unknown(event_type)?;
            return Ok(ParserOutput {
                events: vec![self.activity_event(observation)],
                writes: Vec::new(),
            });
        }
        let data = event
            .get("data")
            .and_then(Value::as_object)
            .ok_or_else(malformed_stream)?;
        self.known_event(event_type, data)
    }

    fn subagent(&mut self, method: &str) -> Result<ParserOutput, RuntimeFailure> {
        require(
            self.status_running && self.terminal.is_none(),
            "subagent notification is outside the active run",
        )?;
        let observation = self.activity.unknown(method)?;
        Ok(ParserOutput {
            events: vec![self.activity_event(observation)],
            writes: Vec::new(),
        })
    }

    fn known_event(
        &mut self,
        event_type: &str,
        data: &serde_json::Map<String, Value>,
    ) -> Result<ParserOutput, RuntimeFailure> {
        if event_type == "turn/start" {
            require(self.turn_id.is_none(), "duplicate turn/start")?;
            self.turn_id = Some(identifier(data.get("turn").and_then(Value::as_str))?.to_owned());
            return Ok(self.progress());
        }
        let turn = data.get("turn").and_then(Value::as_str);
        require(turn == self.turn_id.as_deref(), "turn correlation failed")?;
        match event_type {
            "step/start" => {
                require(
                    self.step_id.is_none() && self.step_ended,
                    "step/start overlaps an active step",
                )?;
                self.step_id =
                    Some(identifier(data.get("step").and_then(Value::as_str))?.to_owned());
                self.step_ended = false;
                self.finish_seen = false;
                self.usage_seen = false;
                Ok(self.progress())
            }
            "user/message" => {
                self.require_step(data)?;
                require(
                    data.get("message").and_then(Value::as_object).is_some(),
                    "user message is malformed",
                )?;
                Ok(self.progress())
            }
            "request/header" | "request/context" => {
                self.require_step(data)?;
                self.require_model_binding(data)?;
                Ok(self.progress())
            }
            "assistant/chunk" => {
                self.require_step(data)?;
                self.chunk(data.get("chunk").ok_or_else(malformed_stream)?)
            }
            "assistant/message" => {
                self.require_step(data)?;
                let message = data
                    .get("message")
                    .and_then(Value::as_object)
                    .ok_or_else(malformed_stream)?;
                require(
                    message.get("role").and_then(Value::as_str) == Some("assistant"),
                    "assistant message role is invalid",
                )?;
                let text = assistant_text(message.get("content").ok_or_else(malformed_stream)?)?;
                if self.streamed_text.is_empty() {
                    self.record_output(&text)?;
                } else {
                    require(
                        text == self.streamed_text,
                        "assistant message disagreed with streamed text",
                    )?;
                }
                Ok(ParserOutput {
                    events: if self.streamed_text.is_empty() {
                        vec![self.event_with(RuntimeEventKind::OutputAvailable, &text)?]
                    } else {
                        Vec::new()
                    },
                    writes: Vec::new(),
                })
            }
            "tool/call" => {
                self.require_step(data)?;
                let call_id = identifier(data.get("callId").and_then(Value::as_str))?.to_owned();
                require(
                    self.tool_calls.insert(call_id.clone()),
                    "duplicate tool call id",
                )?;
                let name = identifier(data.get("name").and_then(Value::as_str))?;
                require(
                    data.get("arguments").and_then(Value::as_str).is_some(),
                    "tool arguments are malformed",
                )?;
                let observation = self.activity.tool_started(&call_id, name)?;
                Ok(ParserOutput {
                    events: vec![self.activity_event(observation)],
                    writes: Vec::new(),
                })
            }
            "tool/result" => {
                self.require_step(data)?;
                let call_id = identifier(data.get("callId").and_then(Value::as_str))?.to_owned();
                require(
                    self.tool_calls.contains(&call_id),
                    "tool result has no admitted call",
                )?;
                require(
                    self.tool_results.insert(call_id.clone()),
                    "duplicate tool result",
                )?;
                require(
                    data.get("message").and_then(Value::as_object).is_some(),
                    "tool result message is malformed",
                )?;
                let status = if let Some(error) = data.get("error") {
                    safe_error(error)?;
                    swallowtail_runtime::ActivityStatus::Failed
                } else {
                    swallowtail_runtime::ActivityStatus::Completed
                };
                let observation = self.activity.tool_finished(&call_id, status)?;
                Ok(ParserOutput {
                    events: vec![self.activity_event(observation)],
                    writes: Vec::new(),
                })
            }
            "step/end" => {
                self.require_step(data)?;
                require(
                    data.get("status").and_then(Value::as_str).is_some(),
                    "step status is missing",
                )?;
                self.step_id = None;
                self.step_ended = true;
                Ok(self.progress())
            }
            "turn/end" => {
                require(
                    self.step_id.is_none() && self.step_ended,
                    "turn/end arrived inside a step",
                )?;
                let status = data
                    .get("status")
                    .and_then(Value::as_str)
                    .ok_or_else(malformed_stream)?;
                let terminal = match status {
                    "completed" => TerminalStatus::Completed,
                    "error" => {
                        let error = data.get("error").ok_or_else(malformed_stream)?;
                        safe_error(error)?;
                        TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                            "swallowtail.deepseek_harness.provider_failed",
                            "DeepSeek Harness provider work reported a safe terminal failure",
                        ))
                    }
                    _ => return Err(malformed_stream()),
                };
                self.terminal = Some(terminal.clone());
                self.phase = Phase::Terminal;
                let mut events = vec![self.event(RuntimeEventKind::Progress)];
                events.extend(
                    self.activity
                        .complete(&terminal)?
                        .into_iter()
                        .map(|observation| self.activity_event(observation)),
                );
                Ok(ParserOutput {
                    events,
                    writes: Vec::new(),
                })
            }
            _ => Err(malformed_stream()),
        }
    }

    fn chunk(&mut self, chunk: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let chunk = chunk.as_object().ok_or_else(malformed_stream)?;
        let kind = chunk
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        match kind {
            "block-start" | "block-end" => Ok(ParserOutput {
                events: Vec::new(),
                writes: Vec::new(),
            }),
            "reasoning-delta" => {
                require(
                    chunk.get("delta").and_then(Value::as_str).is_some(),
                    "reasoning delta is malformed",
                )?;
                Ok(ParserOutput {
                    events: vec![self.event(RuntimeEventKind::ReasoningProgress)],
                    writes: Vec::new(),
                })
            }
            "text-delta" => {
                let text = chunk
                    .get("delta")
                    .and_then(Value::as_str)
                    .ok_or_else(malformed_stream)?;
                self.record_streamed_text(text)?;
                let mut events = vec![self.event_with(RuntimeEventKind::OutputDelta, text)?];
                events.extend(
                    self.activity
                        .assistant_delta(text)?
                        .into_iter()
                        .map(|observation| self.activity_event(observation)),
                );
                Ok(ParserOutput {
                    events,
                    writes: Vec::new(),
                })
            }
            "usage" => {
                require(!self.finish_seen, "usage arrived after finish")?;
                let usage = chunk
                    .get("usage")
                    .and_then(Value::as_object)
                    .ok_or_else(malformed_stream)?;
                let input = token_count(usage.get("inputTokens"))?;
                let output = token_count(usage.get("outputTokens"))?;
                let reasoning = token_count_optional(usage.get("reasoningTokens"))?;
                self.usage_seen = true;
                let token_usage =
                    TokenUsage::new(Some(input), Some(output)).with_reasoning_tokens(reasoning);
                Ok(ParserOutput {
                    events: vec![self.event(RuntimeEventKind::ProviderObservation(
                        ProviderObservation::Usage(token_usage),
                    ))],
                    writes: Vec::new(),
                })
            }
            "finish" => {
                require(self.usage_seen, "finish arrived before usage")?;
                require(
                    matches!(
                        chunk.get("finishReason").and_then(Value::as_str),
                        Some("stop" | "tool-calls")
                    ),
                    "unsupported finish reason",
                )?;
                self.finish_seen = true;
                Ok(ParserOutput {
                    events: Vec::new(),
                    writes: Vec::new(),
                })
            }
            "tool-call-delta" => {
                require(
                    chunk.get("delta").and_then(Value::as_str).is_some(),
                    "tool-call delta is malformed",
                )?;
                Ok(ParserOutput {
                    events: Vec::new(),
                    writes: Vec::new(),
                })
            }
            _ => Err(malformed_stream()),
        }
    }

    fn require_session(&self, value: Option<&str>) -> Result<(), RuntimeFailure> {
        require(
            value == Some(self.session_id.as_str()),
            "notification session id does not match the admitted run",
        )
    }

    fn require_step(&self, data: &serde_json::Map<String, Value>) -> Result<(), RuntimeFailure> {
        require(
            self.step_id.is_some() && !self.step_ended,
            "event arrived outside an active step",
        )?;
        require(
            data.get("step").and_then(Value::as_str) == self.step_id.as_deref(),
            "step correlation failed",
        )
    }

    fn require_model_binding(
        &self,
        data: &serde_json::Map<String, Value>,
    ) -> Result<(), RuntimeFailure> {
        if let Some(provider) = data.get("provider").and_then(Value::as_str) {
            require(provider == self.provider, "provider binding mismatch")?;
        }
        if let Some(model) = data.get("model").and_then(Value::as_str) {
            require(model == self.model, "model binding mismatch")?;
        }
        Ok(())
    }

    fn record_streamed_text(&mut self, text: &str) -> Result<(), RuntimeFailure> {
        let next = self
            .streamed_text
            .len()
            .checked_add(text.len())
            .ok_or_else(output_limit)?;
        require(
            next <= MAXIMUM_OUTPUT_BYTES,
            "output exceeds the bounded projection",
        )?;
        self.streamed_text.push_str(text);
        self.output_text = self.streamed_text.clone();
        Ok(())
    }

    fn record_output(&mut self, text: &str) -> Result<(), RuntimeFailure> {
        let next = self
            .output_text
            .len()
            .checked_add(text.len())
            .ok_or_else(output_limit)?;
        require(
            next <= MAXIMUM_OUTPUT_BYTES,
            "output exceeds the bounded projection",
        )?;
        self.output_text.push_str(text);
        if !self.streamed_text.is_empty() && self.output_text != self.streamed_text {
            return Err(output_mismatch());
        }
        Ok(())
    }

    fn progress(&mut self) -> ParserOutput {
        ParserOutput {
            events: vec![self.event(RuntimeEventKind::Progress)],
            writes: Vec::new(),
        }
    }

    fn event(&mut self, kind: RuntimeEventKind) -> RuntimeEvent {
        let sequence = self.runtime_sequence;
        self.runtime_sequence = self.runtime_sequence.saturating_add(1);
        RuntimeEvent::new(sequence, kind)
    }

    fn event_with(
        &mut self,
        kind: RuntimeEventKind,
        text: &str,
    ) -> Result<RuntimeEvent, RuntimeFailure> {
        let content = OperationContent::new(text).map_err(|_| output_limit())?;
        let sequence = self.runtime_sequence;
        self.runtime_sequence = self.runtime_sequence.saturating_add(1);
        Ok(RuntimeEvent::with_content(sequence, kind, content))
    }

    fn activity_event(
        &mut self,
        observation: swallowtail_runtime::ActivityObservation,
    ) -> RuntimeEvent {
        self.event(RuntimeEventKind::Activity(observation))
    }
}

pub(crate) struct ParsedTerminal {
    status: TerminalStatus,
    output: Option<OperationContent>,
}

impl ParsedTerminal {
    pub(crate) fn outcome(self, cleanup: CleanupOutcome) -> TerminalOutcome {
        let outcome = TerminalOutcome::new(self.status, cleanup);
        match self.output {
            Some(output) => outcome.with_output(output),
            None => outcome,
        }
    }
}

fn request(id: i64, method: &str, params: Value) -> Result<Vec<u8>, RuntimeFailure> {
    let mut bytes = serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params,
    }))
    .map_err(|_| malformed_stream())?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn assistant_text(content: &Value) -> Result<String, RuntimeFailure> {
    let blocks = content.as_array().ok_or_else(malformed_stream)?;
    let mut text = String::new();
    for block in blocks {
        let block = block.as_object().ok_or_else(malformed_stream)?;
        match block.get("type").and_then(Value::as_str) {
            Some("text") => text.push_str(
                block
                    .get("text")
                    .and_then(Value::as_str)
                    .ok_or_else(malformed_stream)?,
            ),
            Some("reasoning") => {
                require(
                    block
                        .get("text")
                        .and_then(Value::as_str)
                        .is_some_and(str::is_empty),
                    "reasoning content was retained",
                )?;
            }
            Some("tool-call") | Some("tool-result") => {}
            _ => return Err(malformed_stream()),
        }
        require(
            text.len() <= MAXIMUM_OUTPUT_BYTES,
            "assistant output exceeds the bounded projection",
        )?;
    }
    Ok(text)
}

fn safe_error(value: &Value) -> Result<(), RuntimeFailure> {
    let error = value.as_object().ok_or_else(malformed_stream)?;
    require(
        error.keys().all(|key| key == "code" || key == "name"),
        "provider error contains an unbounded field",
    )?;
    require(
        error.get("code").is_some() || error.get("name").is_some(),
        "provider error has no safe identity",
    )?;
    for value in error.values() {
        require(
            value.as_str().is_some_and(|text| !text.is_empty()),
            "provider error identity is malformed",
        )?;
    }
    Ok(())
}

fn token_count(value: Option<&Value>) -> Result<u64, RuntimeFailure> {
    value.and_then(Value::as_u64).ok_or_else(malformed_stream)
}

fn token_count_optional(value: Option<&Value>) -> Result<Option<u64>, RuntimeFailure> {
    match value {
        Some(value) => Ok(Some(token_count(Some(value))?)),
        None => Ok(None),
    }
}

fn identifier(value: Option<&str>) -> Result<&str, RuntimeFailure> {
    let value = value.ok_or_else(malformed_stream)?;
    require(
        !value.is_empty()
            && value.len() <= MAXIMUM_IDENTIFIER_BYTES
            && !value.chars().any(char::is_control),
        "provider identity is malformed",
    )?;
    Ok(value)
}

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    if line.ends_with(b"\r") {
        &line[..line.len() - 1]
    } else {
        line
    }
}

fn require(condition: bool, message: &'static str) -> Result<(), RuntimeFailure> {
    if condition {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.deepseek_harness.invalid_stream",
            message,
        ))
    }
}

fn malformed_stream() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.malformed_stream",
        "DeepSeek Harness emitted malformed JSON-RPC",
    )
}

fn frame_limit() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.frame_limit",
        "DeepSeek Harness exceeded the JSON-RPC frame bound",
    )
}

fn record_limit() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.record_limit",
        "DeepSeek Harness exceeded the live notification bound",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.stream_limit",
        "DeepSeek Harness exceeded the JSON-RPC stream bound",
    )
}

fn output_limit() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.output_limit",
        "DeepSeek Harness exceeded the bounded output projection",
    )
}

fn output_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.output_mismatch",
        "DeepSeek Harness assistant message disagreed with streamed text",
    )
}

fn incomplete_stream() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.incomplete_stream",
        "DeepSeek Harness stream ended before shutdown and idle completion",
    )
}

fn unmatched_response() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.response_unmatched",
        "DeepSeek Harness emitted a response without a pending request",
    )
}

fn provider_rpc_error() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.provider_rpc_error",
        "DeepSeek Harness returned a safe JSON-RPC provider error",
    )
}

fn unsupported_notification() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.notification_unsupported",
        "DeepSeek Harness emitted an unsupported JSON-RPC notification",
    )
}

#[cfg(test)]
mod tests {
    use super::JsonRpcParser;
    use swallowtail_runtime::{
        ActivityOperationId, ProcessExit, RuntimeEventKind, RuntimeRunId, TerminalStatus,
    };

    fn parse_fixture(name: &str) -> (JsonRpcParser, Vec<swallowtail_runtime::RuntimeEvent>) {
        parse_fixture_with_server_info_version(name, None)
    }

    fn parse_fixture_with_server_info_version(
        name: &str,
        server_info_version: Option<&str>,
    ) -> (JsonRpcParser, Vec<swallowtail_runtime::RuntimeEvent>) {
        let operation = ActivityOperationId::Run(
            RuntimeRunId::new("deepseek-harness.fixture-run").expect("run id"),
        );
        let session_id = match name {
            "text-success" => "fixture-session-1",
            "tool-success" => "fixture-session-2",
            "tool-error" => "fixture-session-3",
            "missing-key" => "fixture-session-4",
            "unknown-event" => "fixture-session-5",
            _ => panic!("unknown fixture"),
        };
        let mut parser = JsonRpcParser::new(
            operation,
            "<fixture-cwd>".to_owned(),
            "local-ollama".to_owned(),
            "fixture-model".to_owned(),
            "<redacted-prompt>".to_owned(),
            session_id.to_owned(),
        );
        parser.initialize_request().expect("initialize request");
        let bytes = match name {
            "text-success" => include_bytes!(
                "../tests/fixtures/deepseek-harness-runtime-bin-0.1.0rc6/text-success.jsonl"
            )
            .as_slice(),
            "tool-success" => include_bytes!(
                "../tests/fixtures/deepseek-harness-runtime-bin-0.1.0rc6/tool-success.jsonl"
            )
            .as_slice(),
            "tool-error" => include_bytes!(
                "../tests/fixtures/deepseek-harness-runtime-bin-0.1.0rc6/tool-error.jsonl"
            )
            .as_slice(),
            "missing-key" => include_bytes!(
                "../tests/fixtures/deepseek-harness-runtime-bin-0.1.0rc6/missing-key.jsonl"
            )
            .as_slice(),
            "unknown-event" => include_bytes!(
                "../tests/fixtures/deepseek-harness-runtime-bin-0.1.0rc6/unknown-event.jsonl"
            )
            .as_slice(),
            _ => panic!("unknown fixture"),
        };
        let mut output = Vec::new();
        for (line_number, line) in bytes.split_inclusive(|byte| *byte == b'\n').enumerate() {
            let mut frame: serde_json::Value = serde_json::from_slice(line).expect("fixture JSON");
            if let Some(version) = server_info_version
                && frame.get("result").is_some()
                && frame.get("id").and_then(serde_json::Value::as_i64) == Some(1)
            {
                frame["result"]["serverInfo"]["version"] =
                    serde_json::Value::String(version.to_owned());
            }
            if frame.get("method").is_some() && frame.get("id").is_some() {
                continue;
            }
            let frame = if server_info_version.is_some() {
                let mut frame = serde_json::to_vec(&frame).expect("fixture JSON serializes");
                frame.push(b'\n');
                frame
            } else {
                line.to_vec()
            };
            let parsed = parser.push(&frame).unwrap_or_else(|error| {
                panic!("{name} server frame {line_number} failed: {error}")
            });
            output.extend(parsed.events);
        }
        (parser, output)
    }

    #[test]
    fn text_fixture_binds_handshake_prompt_idle_and_terminal() {
        let (parser, events) = parse_fixture("text-success");
        let terminal = parser
            .finish(ProcessExit::new(true, Some(0)))
            .expect("fixture completes");
        assert_eq!(terminal.status, TerminalStatus::Completed);
        assert!(terminal.output.is_some());
        assert!(
            events
                .iter()
                .any(|event| matches!(event.kind(), RuntimeEventKind::ReasoningProgress))
        );
        assert!(
            events
                .iter()
                .any(|event| matches!(event.kind(), RuntimeEventKind::OutputDelta))
        );
    }

    #[test]
    fn initialize_server_info_version_is_wire_metadata_not_release_qualification() {
        let (parser, _) = parse_fixture_with_server_info_version("text-success", Some("9.9.9"));
        let terminal = parser
            .finish(ProcessExit::new(true, Some(0)))
            .expect("alternate wire version remains compatible");
        assert_eq!(terminal.status, TerminalStatus::Completed);
    }

    #[test]
    fn tool_error_stays_provider_owned_and_loop_can_complete() {
        let (parser, events) = parse_fixture("tool-error");
        let terminal = parser
            .finish(ProcessExit::new(true, Some(0)))
            .expect("tool error fixture completes");
        assert_eq!(terminal.status, TerminalStatus::Completed);
        assert!(
            events
                .iter()
                .any(|event| matches!(event.kind(), RuntimeEventKind::Activity(_)))
        );
    }

    #[test]
    fn missing_credential_is_a_safe_provider_terminal() {
        let (parser, _) = parse_fixture("missing-key");
        let terminal = parser
            .finish(ProcessExit::new(true, Some(0)))
            .expect("missing credential fixture completes");
        match terminal.status {
            TerminalStatus::ProviderFailed(diagnostic) => {
                assert_eq!(
                    diagnostic.code(),
                    "swallowtail.deepseek_harness.provider_failed"
                );
            }
            other => panic!("unexpected terminal status: {other:?}"),
        }
    }

    #[test]
    fn unknown_event_is_observation_only() {
        let (parser, events) = parse_fixture("unknown-event");
        parser
            .finish(ProcessExit::new(true, Some(0)))
            .expect("unknown fixture completes");
        assert!(events.iter().any(|event| {
            matches!(event.kind(), RuntimeEventKind::Activity(activity) if matches!(activity.kind(), swallowtail_runtime::ActivityKind::Unknown(_)))
        }));
    }
}
