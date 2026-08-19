use crate::activity::ZcodeActivityProjection;
use crate::failure::failure;
use crate::selection::ZCODE_RELEASE_VERSION;
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
const MAXIMUM_TOOL_RECORDS: usize = 256;
const ADMITTED_PREFERENCE_SCOPES: &[&str] = &["runtime-materialization", "user-execution"];

const CREATE_RESULT_KEYS: &[&str] = &[
    "messages",
    "projection",
    "protocol",
    "runtime",
    "session",
    "settings",
    "slashCommands",
    "todoGroups",
    "todos",
];

const SESSION_KEYS: &[&str] = &[
    "sessionId",
    "sessionKind",
    "status",
    "mode",
    "model",
    "title",
    "traceId",
    "target",
    "workspace",
    "createdAt",
    "updatedAt",
];

const KNOWN_EVENT_TYPES: &[&str] = &[
    "turn.started",
    "model.streaming",
    "tool.updated",
    "turn.completed",
    "turn.failed",
    "session.updated",
    "turn.terminal",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RequestKind {
    Create,
    Subscribe,
    Send,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    AwaitPreferences,
    AwaitCreateResult,
    AwaitSubscribeResult,
    AwaitSendResult,
    Running,
    Complete,
}

#[derive(Debug)]
pub(crate) struct ParserOutput {
    pub(crate) events: Vec<RuntimeEvent>,
    pub(crate) writes: Vec<Vec<u8>>,
}

pub(crate) struct AppServerParser {
    pending: Vec<u8>,
    stream_bytes: usize,
    record_count: usize,
    runtime_sequence: u64,
    event_sequence: u64,
    phase: Phase,
    pending_request: Option<(Value, RequestKind)>,
    cwd: String,
    provider: String,
    model: String,
    prompt: String,
    mode: String,
    session_id: String,
    terminal: Option<TerminalStatus>,
    output_text: String,
    tool_calls: BTreeSet<String>,
    activity: ZcodeActivityProjection,
}

impl AppServerParser {
    pub(crate) fn new(
        operation_id: ActivityOperationId,
        cwd: String,
        provider: String,
        model: String,
        prompt: String,
        mode: String,
    ) -> Self {
        Self {
            pending: Vec::new(),
            stream_bytes: 0,
            record_count: 0,
            runtime_sequence: 1,
            event_sequence: 0,
            phase: Phase::AwaitPreferences,
            pending_request: None,
            cwd,
            provider,
            model,
            prompt,
            mode,
            session_id: String::new(),
            terminal: None,
            output_text: String::new(),
            tool_calls: BTreeSet::new(),
            activity: ZcodeActivityProjection::new(operation_id),
        }
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.phase == Phase::Complete
    }

    pub(crate) fn create_request(&mut self) -> Result<Vec<u8>, RuntimeFailure> {
        require(
            self.pending_request.is_none() && self.phase == Phase::AwaitPreferences,
            "create request already sent",
        )?;
        self.pending_request = Some((json!(1), RequestKind::Create));
        encode_request(
            json!(1),
            "session/create",
            json!({
                "workspace": {
                    "workspacePath": self.cwd,
                    "workspaceKey": self.cwd
                },
                "mode": self.mode
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

    pub(crate) fn finish(mut self, _exit: ProcessExit) -> Result<ParsedTerminal, RuntimeFailure> {
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
                && self.terminal.is_some(),
            "app-server stream ended before turn completion",
        )?;
        let status = self.terminal.take().ok_or_else(incomplete_stream)?;
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
        if frame.get("jsonrpc").is_some() {
            return Err(malformed_stream());
        }
        if frame.get("method").is_some() && frame.get("id").is_some() {
            self.server_request(&frame)
        } else if frame.get("method").is_some() {
            self.notification(&frame)
        } else {
            self.response(&frame)
        }
    }

    fn server_request(&mut self, frame: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let method = frame
            .get("method")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        require(
            method == "session/requestRuntimePreferences",
            "server request is outside the admitted protocol",
        )?;
        let id = frame.get("id").cloned().ok_or_else(malformed_stream)?;
        require(
            id.as_str()
                .is_some_and(|value| identifier(Some(value)).is_ok()),
            "runtime-preferences id is invalid",
        )?;
        let params = frame
            .get("params")
            .and_then(Value::as_object)
            .ok_or_else(malformed_stream)?;
        require(
            params
                .get("scope")
                .and_then(Value::as_str)
                .is_some_and(|scope| ADMITTED_PREFERENCE_SCOPES.contains(&scope)),
            "runtime-preferences scope is invalid",
        )?;
        let session_id = identifier(params.get("sessionId").and_then(Value::as_str))?;
        if self.phase != Phase::AwaitPreferences {
            require(
                session_id == self.session_id,
                "runtime-preferences session does not match the admitted run",
            )?;
            return Ok(ParserOutput {
                events: vec![self.event(RuntimeEventKind::Progress)],
                writes: vec![runtime_preferences_reply(id)?],
            });
        }
        require(
            self.session_id.is_empty(),
            "runtime-preferences duplicated the session id",
        )?;
        self.session_id = session_id.to_owned();
        self.phase = Phase::AwaitCreateResult;
        Ok(ParserOutput {
            events: Vec::new(),
            writes: vec![runtime_preferences_reply(id)?],
        })
    }

    fn response(&mut self, frame: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let id = frame.get("id").cloned().ok_or_else(malformed_stream)?;
        let (expected_id, kind) = self.pending_request.take().ok_or_else(unmatched_response)?;
        require(
            id == expected_id,
            "response id does not match the pending request",
        )?;
        if frame.get("error").is_some() {
            return Err(provider_rpc_error());
        }
        let result = frame.get("result").ok_or_else(malformed_stream)?;
        match kind {
            RequestKind::Create => self.create_result(result),
            RequestKind::Subscribe => self.subscribe_result(result),
            RequestKind::Send => self.send_result(result),
        }
    }

    fn create_result(&mut self, result: &Value) -> Result<ParserOutput, RuntimeFailure> {
        require(
            self.phase == Phase::AwaitCreateResult,
            "create result arrived before runtime-preferences",
        )?;
        let result = result.as_object().ok_or_else(malformed_stream)?;
        for key in CREATE_RESULT_KEYS {
            require(
                result.contains_key(*key),
                "create result is missing a required key",
            )?;
        }
        require(
            protocol_name_is_admitted(result.get("protocol")),
            "create result protocol name is invalid",
        )?;
        let runtime = result
            .get("runtime")
            .and_then(Value::as_object)
            .ok_or_else(malformed_stream)?;
        if let Some(version) = runtime.get("cliVersion") {
            require(
                version.as_str() == Some(ZCODE_RELEASE_VERSION),
                "create result runtime does not match the qualified artifact",
            )
            .map_err(|_| runtime_mismatch())?;
        }
        let session = result
            .get("session")
            .and_then(Value::as_object)
            .ok_or_else(malformed_stream)?;
        for key in SESSION_KEYS {
            require(
                session.contains_key(*key),
                "create result session is missing a required key",
            )?;
        }
        require(
            session.get("sessionId").and_then(Value::as_str) == Some(self.session_id.as_str()),
            "create result session id does not match runtime-preferences",
        )?;
        admitted_settings_mode(result, &self.mode)?;
        require(
            session_model_matches(session.get("model"), &self.model, &self.provider),
            "create result model does not match the admitted plan",
        )?;
        let workspace = session
            .get("workspace")
            .and_then(Value::as_object)
            .ok_or_else(malformed_stream)?;
        require(
            workspace.get("workspacePath").and_then(Value::as_str) == Some(self.cwd.as_str()),
            "create result workspace does not match the admitted working resource",
        )?;
        self.phase = Phase::AwaitSubscribeResult;
        self.pending_request = Some((json!(2), RequestKind::Subscribe));
        Ok(ParserOutput {
            events: Vec::new(),
            writes: vec![encode_request(
                json!(2),
                "session/subscribe",
                json!({
                    "sessionId": self.session_id,
                    "deliveryKind": "desktop-continuous",
                    "includeSnapshot": false,
                    "afterSeq": 0
                }),
            )?],
        })
    }

    fn subscribe_result(&mut self, result: &Value) -> Result<ParserOutput, RuntimeFailure> {
        require(
            self.phase == Phase::AwaitSubscribeResult,
            "duplicate subscribe response",
        )?;
        require(
            result
                .get("eventSeq")
                .and_then(json_u64_value)
                .is_some_and(|value| value == 0),
            "subscribe result is not an empty snapshot",
        )?;
        self.phase = Phase::AwaitSendResult;
        self.pending_request = Some((json!(3), RequestKind::Send));
        Ok(ParserOutput {
            events: Vec::new(),
            writes: vec![encode_request(
                json!(3),
                "session/send",
                json!({
                    "sessionId": self.session_id,
                    "content": self.prompt
                }),
            )?],
        })
    }

    fn send_result(&mut self, result: &Value) -> Result<ParserOutput, RuntimeFailure> {
        require(
            self.phase == Phase::AwaitSendResult,
            "duplicate send response",
        )?;
        require(
            result.get("accepted") == Some(&Value::Bool(true)),
            "send result is not an enqueue receipt",
        )?;
        self.phase = Phase::Running;
        Ok(ParserOutput {
            events: Vec::new(),
            writes: Vec::new(),
        })
    }

    fn notification(&mut self, frame: &Value) -> Result<ParserOutput, RuntimeFailure> {
        require(
            frame.get("id").is_none(),
            "notification must not carry an id",
        )?;
        let method = frame
            .get("method")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        if method != "session/event" {
            return Ok(self.progress());
        }
        self.session_event(frame.get("params").ok_or_else(malformed_stream)?)
    }

    fn session_event(&mut self, params: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let params = params.as_object().ok_or_else(malformed_stream)?;
        require(
            matches!(self.phase, Phase::AwaitSendResult | Phase::Running),
            "session event arrived before prompt dispatch",
        )?;
        require(
            self.terminal.is_none() && self.phase != Phase::Complete,
            "session event arrived after terminal",
        )?;
        require(
            params.get("sessionId").and_then(Value::as_str) == Some(self.session_id.as_str()),
            "session event id does not match the admitted run",
        )?;
        let sequence = params
            .get("seq")
            .and_then(json_u64_value)
            .ok_or_else(malformed_stream)?;
        require(
            sequence == self.event_sequence + 1,
            "session event sequence is not contiguous",
        )?;
        self.event_sequence = sequence;
        let event_type = params
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(malformed_stream)?;
        let payload = params.get("payload").unwrap_or(&Value::Null);
        if !KNOWN_EVENT_TYPES.contains(&event_type) {
            if event_type.starts_with("zcode/") {
                let observation = self.activity.unknown(event_type)?;
                return Ok(ParserOutput {
                    events: vec![self.activity_event(observation)],
                    writes: Vec::new(),
                });
            }
            return Ok(self.progress());
        }
        self.known_event(event_type, payload)
    }

    fn known_event(
        &mut self,
        event_type: &str,
        payload: &Value,
    ) -> Result<ParserOutput, RuntimeFailure> {
        match event_type {
            "turn.started" | "session.updated" | "turn.terminal" => Ok(self.progress()),
            "model.streaming" => self.model_streaming(payload),
            "tool.updated" => self.tool_updated(payload),
            "turn.completed" => self.turn_completed(payload),
            "turn.failed" => self.turn_failed(payload),
            _ => Err(malformed_stream()),
        }
    }

    fn model_streaming(&mut self, payload: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let payload = payload.as_object().ok_or_else(malformed_stream)?;
        match payload.get("kind").and_then(Value::as_str) {
            Some("reasoning_delta") => {
                require(
                    payload.get("delta").and_then(Value::as_str).is_some(),
                    "reasoning delta is malformed",
                )?;
                Ok(ParserOutput {
                    events: vec![self.event(RuntimeEventKind::ReasoningProgress)],
                    writes: Vec::new(),
                })
            }
            Some("text_delta") => {
                let text = payload
                    .get("delta")
                    .and_then(Value::as_str)
                    .ok_or_else(malformed_stream)?;
                self.record_output(text)?;
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
            Some("tool_call") => Ok(self.progress()),
            _ => Err(malformed_stream()),
        }
    }

    fn tool_updated(&mut self, payload: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let payload = payload.as_object().ok_or_else(malformed_stream)?;
        let call_id = identifier(payload.get("toolCallId").and_then(Value::as_str))?.to_owned();
        let name = identifier(payload.get("toolName").and_then(Value::as_str))?;
        match payload.get("kind").and_then(Value::as_str) {
            Some("scheduled") => {
                require(
                    self.tool_calls.len() < MAXIMUM_TOOL_RECORDS,
                    "tool record bound exceeded",
                )?;
                require(
                    self.tool_calls.insert(call_id.clone()),
                    "duplicate tool call id",
                )?;
                let observation = self.activity.tool_started(&call_id, name)?;
                Ok(ParserOutput {
                    events: vec![self.activity_event(observation)],
                    writes: Vec::new(),
                })
            }
            Some("started" | "progress" | "batch") => {
                require(
                    self.tool_calls.contains(&call_id),
                    "tool progress has no admitted call",
                )?;
                Ok(self.progress())
            }
            Some("result") | Some("error") => {
                require(
                    self.tool_calls.contains(&call_id),
                    "tool result has no admitted call",
                )?;
                let status = if payload.get("kind").and_then(Value::as_str) == Some("error") {
                    let error = payload.get("error").ok_or_else(malformed_stream)?;
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
            _ => Err(malformed_stream()),
        }
    }

    fn turn_completed(&mut self, payload: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let payload = payload.as_object().ok_or_else(malformed_stream)?;
        require(
            payload.get("resultType").and_then(Value::as_str) == Some("success"),
            "turn completed with an unsupported result type",
        )?;
        let usage = payload
            .get("usage")
            .and_then(Value::as_object)
            .ok_or_else(malformed_stream)?;
        let input = token_count(usage.get("inputTokens"))?;
        let output = token_count(usage.get("outputTokens"))?;
        if let Some(total) = usage.get("totalTokens") {
            require(
                token_count(Some(total))? == input.saturating_add(output),
                "usage totals are inconsistent",
            )?;
        }
        self.complete_turn(
            TerminalStatus::Completed,
            Some(TokenUsage::new(Some(input), Some(output))),
        )
    }

    fn turn_failed(&mut self, payload: &Value) -> Result<ParserOutput, RuntimeFailure> {
        let payload = payload.as_object().ok_or_else(malformed_stream)?;
        let error = payload.get("error").ok_or_else(malformed_stream)?;
        safe_error(error)?;
        let code = error
            .get("code")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let diagnostic = if code == "MISSING_CREDENTIAL" {
            SafeDiagnostic::new(
                "swallowtail.zcode.app_server.credential_missing",
                "ZCode app-server reported a missing provider credential",
            )
        } else {
            SafeDiagnostic::new(
                "swallowtail.zcode.app_server.provider_failed",
                "ZCode app-server provider work reported a safe terminal failure",
            )
        };
        self.complete_turn(TerminalStatus::ProviderFailed(diagnostic), None)
    }

    fn complete_turn(
        &mut self,
        terminal: TerminalStatus,
        usage: Option<TokenUsage>,
    ) -> Result<ParserOutput, RuntimeFailure> {
        self.terminal = Some(terminal.clone());
        self.phase = Phase::Complete;
        let mut events = Vec::new();
        if let Some(usage) = usage {
            events.push(self.event(RuntimeEventKind::ProviderObservation(
                ProviderObservation::Usage(usage),
            )));
        }
        events.push(self.event(RuntimeEventKind::Progress));
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

fn runtime_preferences_reply(id: Value) -> Result<Vec<u8>, RuntimeFailure> {
    encode_result(
        id,
        json!({
            "nativeSearchEnhancementsEnabled": false,
            "memoryEnabled": false,
            "askUserQuestionAutoResolutionEnabled": false
        }),
    )
}

fn encode_request(id: Value, method: &str, params: Value) -> Result<Vec<u8>, RuntimeFailure> {
    encode_frame(json!({
        "id": id,
        "method": method,
        "params": params,
    }))
}

fn encode_result(id: Value, result: Value) -> Result<Vec<u8>, RuntimeFailure> {
    encode_frame(json!({
        "id": id,
        "result": result,
    }))
}

fn encode_frame(value: Value) -> Result<Vec<u8>, RuntimeFailure> {
    require(
        value.get("jsonrpc").is_none(),
        "encoder must not emit jsonrpc",
    )?;
    let mut bytes = serde_json::to_vec(&value).map_err(|_| malformed_stream())?;
    bytes.push(b'\n');
    Ok(bytes)
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
    value.and_then(json_u64_value).ok_or_else(malformed_stream)
}

fn json_u64_value(value: &Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_i64().and_then(|number| u64::try_from(number).ok()))
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
            "swallowtail.zcode.app_server.invalid_stream",
            message,
        ))
    }
}

fn malformed_stream() -> RuntimeFailure {
    failure(
        "swallowtail.zcode.app_server.malformed_stream",
        "ZCode app-server emitted malformed line-delimited JSON",
    )
}

fn frame_limit() -> RuntimeFailure {
    failure(
        "swallowtail.zcode.app_server.frame_limit",
        "ZCode app-server exceeded the frame bound",
    )
}

fn record_limit() -> RuntimeFailure {
    failure(
        "swallowtail.zcode.app_server.record_limit",
        "ZCode app-server exceeded the live notification bound",
    )
}

fn stream_limit() -> RuntimeFailure {
    failure(
        "swallowtail.zcode.app_server.stream_limit",
        "ZCode app-server exceeded the stream bound",
    )
}

fn output_limit() -> RuntimeFailure {
    failure(
        "swallowtail.zcode.app_server.output_limit",
        "ZCode app-server exceeded the bounded output projection",
    )
}

fn incomplete_stream() -> RuntimeFailure {
    failure(
        "swallowtail.zcode.app_server.incomplete_stream",
        "ZCode app-server stream ended before turn completion",
    )
}

fn unmatched_response() -> RuntimeFailure {
    failure(
        "swallowtail.zcode.app_server.response_unmatched",
        "ZCode app-server emitted a response without a pending request",
    )
}

fn provider_rpc_error() -> RuntimeFailure {
    failure(
        "swallowtail.zcode.app_server.provider_rpc_error",
        "ZCode app-server returned a safe provider error",
    )
}

fn runtime_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.zcode.app_server.runtime_mismatch",
        "ZCode app-server runtime does not match the qualified artifact",
    )
}

fn protocol_name_is_admitted(protocol: Option<&Value>) -> bool {
    let protocol = match protocol.and_then(Value::as_object) {
        Some(protocol) => protocol,
        None => return false,
    };
    match protocol.get("name").and_then(Value::as_str) {
        Some("zcode-app-server") => true,
        Some("ZCode Protocol") => protocol.get("version").and_then(json_u64_value) == Some(1),
        _ => false,
    }
}

fn session_model_matches(
    model: Option<&Value>,
    expected_model: &str,
    expected_provider: &str,
) -> bool {
    match model {
        Some(Value::String(model)) => model == expected_model,
        Some(Value::Object(model)) => {
            model.get("modelId").and_then(Value::as_str) == Some(expected_model)
                && model.get("providerId").and_then(Value::as_str) == Some(expected_provider)
        }
        _ => false,
    }
}

fn admitted_settings_mode(
    result: &serde_json::Map<String, Value>,
    expected: &str,
) -> Result<(), RuntimeFailure> {
    let settings = result
        .get("settings")
        .and_then(Value::as_object)
        .ok_or_else(malformed_stream)?;
    require(
        settings
            .get("mode")
            .and_then(Value::as_object)
            .and_then(|mode| mode.get("current"))
            .and_then(Value::as_str)
            == Some(expected),
        "create result mode does not match the admitted host mode",
    )?;
    if let Some(permission_mode) = settings
        .get("permission")
        .and_then(Value::as_object)
        .and_then(|permission| permission.get("mode"))
        .and_then(Value::as_str)
    {
        require(
            permission_mode == expected,
            "create result permission mode does not match the admitted host mode",
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::AppServerParser;
    use serde_json::{Value, json};
    use swallowtail_runtime::{
        ActivityKind, ActivityOperationId, OperationContent, ProcessExit, RuntimeEventKind,
        RuntimeRunId, TerminalStatus,
    };

    fn parser() -> AppServerParser {
        AppServerParser::new(
            ActivityOperationId::Run(RuntimeRunId::new("zcode.fixture-run").expect("run id")),
            "<fixture-cwd>".to_owned(),
            "fixture-provider".to_owned(),
            "fixture-model".to_owned(),
            "<redacted-prompt>".to_owned(),
            "plan".to_owned(),
        )
    }

    fn fixture_bytes(name: &str) -> &'static [u8] {
        match name {
            "handshake" => {
                include_bytes!("../tests/fixtures/zcode-runtime-0.16.3/handshake.jsonl").as_slice()
            }
            "text-success" => {
                include_bytes!("../tests/fixtures/zcode-runtime-0.16.3/text-success.jsonl")
                    .as_slice()
            }
            "tool-success" => {
                include_bytes!("../tests/fixtures/zcode-runtime-0.16.3/tool-success.jsonl")
                    .as_slice()
            }
            "tool-error" => {
                include_bytes!("../tests/fixtures/zcode-runtime-0.16.3/tool-error.jsonl").as_slice()
            }
            "missing-key" => {
                include_bytes!("../tests/fixtures/zcode-runtime-0.16.3/missing-key.jsonl")
                    .as_slice()
            }
            "unknown-event" => {
                include_bytes!("../tests/fixtures/zcode-runtime-0.16.3/unknown-event.jsonl")
                    .as_slice()
            }
            "create-without-preferences" => include_bytes!(
                "../tests/fixtures/zcode-runtime-0.16.3/create-without-preferences.jsonl"
            )
            .as_slice(),
            _ => panic!("unknown fixture {name}"),
        }
    }

    fn should_skip(frame: &Value) -> bool {
        match frame.get("method").and_then(Value::as_str) {
            Some("session/create" | "session/subscribe" | "session/send") => true,
            Some(_) => false,
            None => frame.get("id").and_then(Value::as_str).is_some(),
        }
    }

    fn replay(
        name: &str,
    ) -> (
        AppServerParser,
        Vec<swallowtail_runtime::RuntimeEvent>,
        Vec<Vec<u8>>,
    ) {
        let mut parser = parser();
        parser.create_request().expect("create request");
        let mut events = Vec::new();
        let mut writes = Vec::new();
        for (line_number, line) in fixture_bytes(name)
            .split_inclusive(|byte| *byte == b'\n')
            .enumerate()
        {
            if line.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            let frame: Value = serde_json::from_slice(line).expect("fixture JSON");
            if should_skip(&frame) {
                continue;
            }
            let parsed = parser.push(line).unwrap_or_else(|error| {
                panic!("{name} server frame {line_number} failed: {error}")
            });
            events.extend(parsed.events);
            writes.extend(parsed.writes);
        }
        (parser, events, writes)
    }

    fn encode(value: &Value) -> Vec<u8> {
        let mut bytes = serde_json::to_vec(value).expect("frame serializes");
        bytes.push(b'\n');
        bytes
    }

    #[test]
    fn text_fixture_binds_handshake_prompt_idle_and_terminal() {
        let (parser, events, writes) = replay("text-success");
        assert!(parser.is_complete());
        let terminal = parser
            .finish(ProcessExit::new(false, Some(143)))
            .expect("kill-after-complete still finishes as completed");
        assert_eq!(terminal.status, TerminalStatus::Completed);
        assert_eq!(
            terminal.output.as_ref().map(OperationContent::as_str),
            Some("fixture response")
        );
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
        assert!(
            events
                .iter()
                .any(|event| matches!(event.kind(), RuntimeEventKind::ProviderObservation(_)))
        );
        assert!(
            writes
                .iter()
                .any(|write| write_contains(write, b"session/subscribe"))
        );
        assert!(
            writes
                .iter()
                .any(|write| write_contains(write, b"session/send"))
        );
        assert!(
            writes
                .iter()
                .all(|write| !write_contains(write, b"jsonrpc"))
        );
        assert!(
            writes
                .iter()
                .all(|write| !write_contains(write, b"session/stop"))
        );
    }

    fn write_contains(write: &[u8], needle: &[u8]) -> bool {
        write.windows(needle.len()).any(|window| window == needle)
    }

    #[test]
    fn handshake_answers_runtime_preferences_before_create_result() {
        let (parser, _, writes) = replay("handshake");
        assert!(!parser.is_complete());
        assert!(
            writes
                .iter()
                .any(|write| write_contains(write, b"nativeSearchEnhancementsEnabled"))
        );
        assert!(
            writes
                .iter()
                .any(|write| write_contains(write, b"session/subscribe"))
        );
    }

    #[test]
    fn create_without_preferences_fails_closed() {
        let mut parser = parser();
        parser.create_request().expect("create request");
        let error = parser
            .push(
                fixture_bytes("create-without-preferences")
                    .split_inclusive(|byte| *byte == b'\n')
                    .nth(1)
                    .expect("create result line"),
            )
            .expect_err("create result before preferences is rejected");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.zcode.app_server.invalid_stream"
        );
        assert!(error.diagnostic().message().contains("runtime-preferences"));
    }

    #[test]
    fn jsonrpc_field_is_rejected() {
        let mut parser = parser();
        parser.create_request().expect("create request");
        let error = parser
            .push(&encode(&json!({"jsonrpc":"2.0","id":"server-0","method":"session/requestRuntimePreferences","params":{"sessionId":"fixture-session-0","scope":"runtime-materialization"}})))
            .expect_err("jsonrpc field is rejected");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.zcode.app_server.malformed_stream"
        );
    }

    #[test]
    fn tool_error_stays_provider_owned_and_loop_can_complete() {
        let (parser, events, _) = replay("tool-error");
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
    fn tool_success_projects_scheduled_and_result_without_input_bodies() {
        let (parser, events, writes) = replay("tool-success");
        parser
            .finish(ProcessExit::new(true, Some(0)))
            .expect("tool success fixture completes");
        assert!(
            events
                .iter()
                .any(|event| matches!(event.kind(), RuntimeEventKind::Activity(_)))
        );
        assert!(writes.iter().all(|write| !write_contains(write, b"yolo")));
    }

    #[test]
    fn missing_credential_is_a_safe_provider_terminal() {
        let (parser, _) = {
            let (parser, events, _) = replay("missing-key");
            (parser, events)
        };
        let terminal = parser
            .finish(ProcessExit::new(true, Some(0)))
            .expect("missing credential fixture completes");
        match terminal.status {
            TerminalStatus::ProviderFailed(diagnostic) => {
                assert_eq!(
                    diagnostic.code(),
                    "swallowtail.zcode.app_server.credential_missing"
                );
                assert!(!diagnostic.message().contains("sess_"));
            }
            other => panic!("unexpected terminal status: {other:?}"),
        }
    }

    #[test]
    fn unknown_event_is_observation_only() {
        let (parser, events, _) = replay("unknown-event");
        parser
            .finish(ProcessExit::new(true, Some(0)))
            .expect("unknown fixture completes");
        assert!(events.iter().any(|event| {
            matches!(event.kind(), RuntimeEventKind::Activity(activity) if matches!(activity.kind(), ActivityKind::Unknown(_)))
        }));
    }

    #[test]
    fn unscoped_unknown_event_is_content_free_progress() {
        let mut parser = parser();
        parser.create_request().expect("create request");
        for line in fixture_bytes("text-success").split_inclusive(|byte| *byte == b'\n') {
            if line.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            let frame: Value = serde_json::from_slice(line).expect("fixture JSON");
            if should_skip(&frame) {
                continue;
            }
            if frame.get("method").and_then(Value::as_str) == Some("session/event") {
                break;
            }
            parser.push(line).expect("prefix frames parse");
        }
        let parsed = parser
            .push(&encode(&json!({
                "method":"session/event",
                "params":{
                    "sessionId":"fixture-session-1",
                    "seq":1,
                    "type":"session.titleUpdated",
                    "payload":{"title":"<redacted-title>"}
                }
            })))
            .expect("unscoped unknown events are content-free progress");
        assert!(
            parsed
                .events
                .iter()
                .any(|event| matches!(event.kind(), RuntimeEventKind::Progress))
        );
    }

    #[test]
    fn non_session_notification_is_content_free_progress() {
        let mut parser = parser();
        parser.create_request().expect("create request");
        for line in fixture_bytes("handshake").split_inclusive(|byte| *byte == b'\n') {
            if line.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            let frame: Value = serde_json::from_slice(line).expect("fixture JSON");
            if should_skip(&frame) {
                continue;
            }
            parser.push(line).expect("handshake frames parse");
        }
        let parsed = parser
            .push(&encode(
                &json!({"method":"v4/telemetry/event","params":{"kind":"<redacted>"}}),
            ))
            .expect("non-session notifications are content-free progress");
        assert!(
            parsed
                .events
                .iter()
                .any(|event| matches!(event.kind(), RuntimeEventKind::Progress))
        );
    }

    #[test]
    fn installed_create_result_accepts_live_protocol_model_and_settings_mode() {
        let mut parser = AppServerParser::new(
            ActivityOperationId::Run(RuntimeRunId::new("zcode.fixture-run").expect("run id")),
            "<fixture-cwd>".to_owned(),
            "zai".to_owned(),
            "gemma4:12b".to_owned(),
            "<redacted-prompt>".to_owned(),
            "plan".to_owned(),
        );
        parser.create_request().expect("create request");
        parser
            .push(&encode(&json!({
                "id":"server-0",
                "method":"session/requestRuntimePreferences",
                "params":{"sessionId":"fixture-session-live","scope":"runtime-materialization"}
            })))
            .expect("preferences request");
        let parsed = parser
            .push(&encode(&json!({
                "id":1,
                "result":{
                    "messages":[],
                    "projection":{"status":"idle"},
                    "protocol":{"name":"ZCode Protocol","version":1},
                    "runtime":{"eventSeq":0,"stateRevision":0},
                    "session":{
                        "sessionId":"fixture-session-live",
                        "sessionKind":"interactive",
                        "status":"idle",
                        "mode":"build",
                        "model":{"modelId":"gemma4:12b","providerId":"zai"},
                        "title":"",
                        "traceId":"fixture-trace-live",
                        "target":null,
                        "workspace":{"workspacePath":"<fixture-cwd>","workspaceKey":"<fixture-cwd>"},
                        "createdAt":1,
                        "updatedAt":1
                    },
                    "settings":{
                        "mode":{"current":"plan"},
                        "model":{"current":{"modelId":"gemma4:12b","providerId":"zai"}},
                        "permission":{"mode":"plan"}
                    },
                    "slashCommands":[],
                    "todoGroups":[],
                    "todos":[]
                }
            })))
            .expect("installed create result is admitted");
        assert!(
            parsed
                .writes
                .iter()
                .any(|write| write_contains(write, b"session/subscribe"))
        );
    }

    #[test]
    fn later_runtime_preferences_are_answered_as_progress() {
        let mut parser = parser();
        parser.create_request().expect("create request");
        for line in fixture_bytes("handshake").split_inclusive(|byte| *byte == b'\n') {
            if line.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            let frame: Value = serde_json::from_slice(line).expect("fixture JSON");
            if should_skip(&frame) {
                continue;
            }
            parser.push(line).expect("handshake frames parse");
        }
        let parsed = parser
            .push(&encode(&json!({
                "id":"server-later",
                "method":"session/requestRuntimePreferences",
                "params":{"sessionId":"fixture-session-0","scope":"runtime-materialization"}
            })))
            .expect("later preferences stay on the admitted session");
        assert!(
            parsed
                .events
                .iter()
                .any(|event| matches!(event.kind(), RuntimeEventKind::Progress))
        );
        assert!(
            parsed
                .writes
                .iter()
                .any(|write| write_contains(write, br#""nativeSearchEnhancementsEnabled":false"#))
        );
    }

    #[test]
    fn later_user_execution_preferences_are_answered_as_progress() {
        let mut parser = parser();
        parser.create_request().expect("create request");
        for line in fixture_bytes("handshake").split_inclusive(|byte| *byte == b'\n') {
            if line.iter().all(u8::is_ascii_whitespace) {
                continue;
            }
            let frame: Value = serde_json::from_slice(line).expect("fixture JSON");
            if should_skip(&frame) {
                continue;
            }
            parser.push(line).expect("handshake frames parse");
        }
        let parsed = parser
            .push(&encode(&json!({
                "id":"server-user-execution",
                "method":"session/requestRuntimePreferences",
                "params":{"sessionId":"fixture-session-0","scope":"user-execution"}
            })))
            .expect("user-execution preferences stay on the admitted session");
        assert!(
            parsed
                .events
                .iter()
                .any(|event| matches!(event.kind(), RuntimeEventKind::Progress))
        );
        assert!(
            parsed
                .writes
                .iter()
                .any(|write| write_contains(write, br#""memoryEnabled":false"#))
        );
    }
}
