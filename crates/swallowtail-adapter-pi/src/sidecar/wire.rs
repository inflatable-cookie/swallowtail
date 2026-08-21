use super::protocol::{
    PiSdkSidecarProtocolFailure, PiSdkSidecarProtocolFailureKind, PiSdkSidecarRecordKind,
};
use serde_json::Value;
use swallowtail_runtime::TokenUsage;

pub(crate) mod replay;

use replay::{PiSdkReplayItem, decode_replay_item};

const MAXIMUM_RECORD_BYTES: usize = 1024 * 1024;
const MAXIMUM_COMMAND_ID_BYTES: usize = 128;
const MAXIMUM_FAILURE_CODE_BYTES: usize = 96;
const MAXIMUM_FAILURE_MESSAGE_BYTES: usize = 512;

pub(crate) struct PiSdkSidecarDecoder {
    buffer: Vec<u8>,
}

impl PiSdkSidecarDecoder {
    pub(crate) const fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub(crate) fn push(
        &mut self,
        bytes: &[u8],
    ) -> Result<Vec<PiSdkSidecarRecord>, PiSdkSidecarProtocolFailure> {
        self.buffer.extend_from_slice(bytes);
        if self.buffer.len() > MAXIMUM_RECORD_BYTES && !self.buffer.contains(&b'\n') {
            return Err(failure(PiSdkSidecarProtocolFailureKind::RecordTooLarge));
        }
        let mut records = Vec::new();
        while let Some(end) = self.buffer.iter().position(|byte| *byte == b'\n') {
            if end > MAXIMUM_RECORD_BYTES {
                return Err(failure(PiSdkSidecarProtocolFailureKind::RecordTooLarge));
            }
            let mut line: Vec<_> = self.buffer.drain(..=end).collect();
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            records.push(decode_record(&line)?);
        }
        Ok(records)
    }

    pub(crate) fn finish(self) -> Result<(), PiSdkSidecarProtocolFailure> {
        if self.buffer.is_empty() {
            Ok(())
        } else {
            Err(failure(PiSdkSidecarProtocolFailureKind::MissingLfDelimiter))
        }
    }
}

pub(crate) enum PiSdkSidecarRecord {
    Response(PiSdkSidecarResponse),
    Event(PiSdkSidecarEvent),
    Terminal(PiSdkSidecarFailure),
    Diagnostic(PiSdkSidecarDiagnostic),
}

impl PiSdkSidecarRecord {
    pub(crate) const fn kind(&self) -> PiSdkSidecarRecordKind {
        match self {
            Self::Response(_) => PiSdkSidecarRecordKind::Response,
            Self::Event(_) => PiSdkSidecarRecordKind::Event,
            Self::Terminal(_) => PiSdkSidecarRecordKind::Terminal,
            Self::Diagnostic(_) => PiSdkSidecarRecordKind::Diagnostic,
        }
    }
}

pub(crate) struct PiSdkSidecarResponse {
    pub(crate) id: String,
    pub(crate) command: String,
    pub(crate) success: bool,
    pub(crate) data: Option<Value>,
    pub(crate) failure: Option<PiSdkSidecarFailure>,
}

pub(crate) struct PiSdkSidecarFailure {
    pub(crate) code: String,
    pub(crate) message: String,
}

pub(crate) struct PiSdkSidecarDiagnostic {
    pub(crate) level: PiSdkSidecarDiagnosticLevel,
    pub(crate) code: String,
    pub(crate) message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PiSdkSidecarDiagnosticLevel {
    Info,
    Warning,
    Error,
}

pub(crate) enum PiSdkSidecarEvent {
    Started,
    TurnStarted,
    TurnEnded,
    Ended,
    Settled,
    Progress,
    MessageStarted,
    MessageEnded {
        stop_reason: String,
        usage: Option<TokenUsage>,
    },
    OutputDelta(String),
    ReasoningStarted,
    ReasoningDelta(String),
    ReasoningEnded,
    ToolStarted {
        call_id: String,
        name: String,
    },
    ToolUpdated {
        call_id: String,
        name: String,
    },
    ToolEnded {
        call_id: String,
        name: String,
        failed: bool,
    },
    ReplayItem {
        sequence: u64,
        item: PiSdkReplayItem,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PiSdkSidecarCommand {
    Bootstrap,
    SessionNew,
    SessionSwitch,
    SessionReplay,
    Prompt,
    Steer,
    FollowUp,
    Abort,
    State,
    Close,
}

impl PiSdkSidecarCommand {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Bootstrap => "bootstrap",
            Self::SessionNew => "session_new",
            Self::SessionSwitch => "session_switch",
            Self::SessionReplay => "session_replay",
            Self::Prompt => "prompt",
            Self::Steer => "steer",
            Self::FollowUp => "follow_up",
            Self::Abort => "abort",
            Self::State => "state",
            Self::Close => "close",
        }
    }

    fn from_qualified(name: &str) -> Option<Self> {
        Some(match name {
            "bootstrap" => Self::Bootstrap,
            "session_new" => Self::SessionNew,
            "session_switch" => Self::SessionSwitch,
            "session_replay" => Self::SessionReplay,
            "prompt" => Self::Prompt,
            "steer" => Self::Steer,
            "follow_up" => Self::FollowUp,
            "abort" => Self::Abort,
            "state" => Self::State,
            "close" => Self::Close,
            _ => return None,
        })
    }
}

pub(crate) fn encode_command(
    id: &str,
    command: PiSdkSidecarCommand,
    params: Value,
) -> Result<Vec<u8>, PiSdkSidecarProtocolFailure> {
    let record = serde_json::json!({
        "type": "command",
        "id": id,
        "command": command.as_str(),
        "params": params,
    });
    let mut bytes = serde_json::to_vec(&record)
        .map_err(|_| failure(PiSdkSidecarProtocolFailureKind::MalformedJson))?;
    if bytes.len() + 1 > MAXIMUM_RECORD_BYTES {
        return Err(failure(PiSdkSidecarProtocolFailureKind::RecordTooLarge));
    }
    bytes.push(b'\n');
    Ok(bytes)
}

pub(crate) fn decode_record(
    bytes: &[u8],
) -> Result<PiSdkSidecarRecord, PiSdkSidecarProtocolFailure> {
    if bytes.is_empty() {
        return Err(failure(PiSdkSidecarProtocolFailureKind::EmptyRecord));
    }
    let value: Value = serde_json::from_slice(bytes)
        .map_err(|_| failure(PiSdkSidecarProtocolFailureKind::MalformedJson))?;
    match value.get("type").and_then(Value::as_str) {
        Some("response") => decode_response(&value).map(PiSdkSidecarRecord::Response),
        Some("event") => decode_event(&value).map(PiSdkSidecarRecord::Event),
        Some("terminal") => decode_terminal(&value).map(PiSdkSidecarRecord::Terminal),
        Some("diagnostic") => decode_diagnostic(&value).map(PiSdkSidecarRecord::Diagnostic),
        Some(_) => Err(failure(PiSdkSidecarProtocolFailureKind::UnknownRecord)),
        None => Err(failure(PiSdkSidecarProtocolFailureKind::MissingType)),
    }
}

fn decode_response(value: &Value) -> Result<PiSdkSidecarResponse, PiSdkSidecarProtocolFailure> {
    let invalid = PiSdkSidecarProtocolFailureKind::InvalidResponse;
    let id = bounded_text(value, "id", MAXIMUM_COMMAND_ID_BYTES, invalid)?.to_owned();
    let command = required_text(value, "command", invalid)?;
    if PiSdkSidecarCommand::from_qualified(command).is_none() {
        return Err(failure(invalid));
    }
    let success = value
        .get("success")
        .and_then(Value::as_bool)
        .ok_or_else(|| failure(invalid))?;
    let data = value.get("data").cloned();
    match (success, &data, value.get("failure")) {
        (true, _, None) => {
            if data.as_ref().is_some_and(|data| !data.is_object()) {
                return Err(failure(invalid));
            }
            Ok(PiSdkSidecarResponse {
                id,
                command: command.to_owned(),
                success,
                data,
                failure: None,
            })
        }
        (false, None, Some(failure_record)) => Ok(PiSdkSidecarResponse {
            id,
            command: command.to_owned(),
            success,
            data: None,
            failure: Some(decode_failure(failure_record, invalid)?),
        }),
        _ => Err(failure(invalid)),
    }
}

fn decode_event(value: &Value) -> Result<PiSdkSidecarEvent, PiSdkSidecarProtocolFailure> {
    let invalid = PiSdkSidecarProtocolFailureKind::InvalidEvent;
    match value.get("event").and_then(Value::as_str) {
        Some("agent_start") => Ok(PiSdkSidecarEvent::Started),
        Some("turn_start") => Ok(PiSdkSidecarEvent::TurnStarted),
        Some("turn_end") => Ok(PiSdkSidecarEvent::TurnEnded),
        Some("agent_end") => Ok(PiSdkSidecarEvent::Ended),
        Some("agent_settled") => Ok(PiSdkSidecarEvent::Settled),
        Some("progress") => Ok(PiSdkSidecarEvent::Progress),
        Some("reasoning_start") => Ok(PiSdkSidecarEvent::ReasoningStarted),
        Some("reasoning_end") => Ok(PiSdkSidecarEvent::ReasoningEnded),
        Some("message_start") => {
            required_text(value, "role", invalid)?;
            Ok(PiSdkSidecarEvent::MessageStarted)
        }
        Some("message_end") => decode_message_end(value),
        Some("output_delta") => required_text(value, "delta", invalid)
            .map(|delta| PiSdkSidecarEvent::OutputDelta(delta.to_owned())),
        Some("reasoning_delta") => required_text(value, "delta", invalid)
            .map(|delta| PiSdkSidecarEvent::ReasoningDelta(delta.to_owned())),
        Some("tool_execution_start") => decode_tool(value, ToolPhase::Started),
        Some("tool_execution_update") => decode_tool(value, ToolPhase::Updated),
        Some("tool_execution_end") => decode_tool(value, ToolPhase::Ended),
        Some("replay_item") => decode_replay_item(value),
        Some(_) => Err(failure(PiSdkSidecarProtocolFailureKind::UnknownRecord)),
        None => Err(failure(PiSdkSidecarProtocolFailureKind::MissingType)),
    }
}

fn decode_message_end(value: &Value) -> Result<PiSdkSidecarEvent, PiSdkSidecarProtocolFailure> {
    let invalid = PiSdkSidecarProtocolFailureKind::InvalidEvent;
    if required_text(value, "role", invalid)? != "assistant" {
        return Err(failure(invalid));
    }
    let stop_reason = required_text(value, "stopReason", invalid)?.to_owned();
    let usage = value
        .get("usage")
        .map(|usage| decode_usage(usage, invalid))
        .transpose()?;
    Ok(PiSdkSidecarEvent::MessageEnded { stop_reason, usage })
}

#[derive(Clone, Copy)]
enum ToolPhase {
    Started,
    Updated,
    Ended,
}

fn decode_tool(
    value: &Value,
    phase: ToolPhase,
) -> Result<PiSdkSidecarEvent, PiSdkSidecarProtocolFailure> {
    let invalid = PiSdkSidecarProtocolFailureKind::InvalidEvent;
    let call_id = required_text(value, "toolCallId", invalid)?.to_owned();
    let name = required_text(value, "toolName", invalid)?.to_owned();
    Ok(match phase {
        ToolPhase::Started => PiSdkSidecarEvent::ToolStarted { call_id, name },
        ToolPhase::Updated => PiSdkSidecarEvent::ToolUpdated { call_id, name },
        ToolPhase::Ended => PiSdkSidecarEvent::ToolEnded {
            call_id,
            name,
            failed: value
                .get("isError")
                .and_then(Value::as_bool)
                .ok_or_else(|| failure(invalid))?,
        },
    })
}

fn decode_terminal(value: &Value) -> Result<PiSdkSidecarFailure, PiSdkSidecarProtocolFailure> {
    let failure_record = value
        .get("failure")
        .ok_or_else(|| failure(PiSdkSidecarProtocolFailureKind::InvalidTerminal))?;
    decode_failure(failure_record, PiSdkSidecarProtocolFailureKind::InvalidTerminal)
}

fn decode_diagnostic(
    value: &Value,
) -> Result<PiSdkSidecarDiagnostic, PiSdkSidecarProtocolFailure> {
    let invalid = PiSdkSidecarProtocolFailureKind::InvalidDiagnostic;
    let level = match value.get("level").and_then(Value::as_str) {
        Some("info") => PiSdkSidecarDiagnosticLevel::Info,
        Some("warning") => PiSdkSidecarDiagnosticLevel::Warning,
        Some("error") => PiSdkSidecarDiagnosticLevel::Error,
        _ => return Err(failure(invalid)),
    };
    Ok(PiSdkSidecarDiagnostic {
        level,
        code: bounded_text(value, "code", MAXIMUM_FAILURE_CODE_BYTES, invalid)?.to_owned(),
        message: bounded_text(value, "message", MAXIMUM_FAILURE_MESSAGE_BYTES, invalid)?
            .to_owned(),
    })
}

pub(crate) fn decode_failure(
    value: &Value,
    kind: PiSdkSidecarProtocolFailureKind,
) -> Result<PiSdkSidecarFailure, PiSdkSidecarProtocolFailure> {
    Ok(PiSdkSidecarFailure {
        code: bounded_text(value, "code", MAXIMUM_FAILURE_CODE_BYTES, kind)?.to_owned(),
        message: bounded_text(value, "message", MAXIMUM_FAILURE_MESSAGE_BYTES, kind)?.to_owned(),
    })
}

pub(crate) fn decode_usage(
    value: &Value,
    kind: PiSdkSidecarProtocolFailureKind,
) -> Result<TokenUsage, PiSdkSidecarProtocolFailure> {
    let input = required_u64(value, "input", kind)?;
    let output = required_u64(value, "output", kind)?;
    let cache_read = required_u64(value, "cacheRead", kind)?;
    let cache_write = required_u64(value, "cacheWrite", kind)?;
    Ok(TokenUsage::new(Some(input), Some(output))
        .with_cache_tokens(Some(cache_read), Some(cache_write)))
}

pub(crate) fn required_text<'a>(
    value: &'a Value,
    field: &str,
    kind: PiSdkSidecarProtocolFailureKind,
) -> Result<&'a str, PiSdkSidecarProtocolFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| failure(kind))
}

pub(crate) fn required_string<'a>(
    value: &'a Value,
    field: &str,
    kind: PiSdkSidecarProtocolFailureKind,
) -> Result<&'a str, PiSdkSidecarProtocolFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| failure(kind))
}

fn bounded_text<'a>(
    value: &'a Value,
    field: &str,
    maximum: usize,
    kind: PiSdkSidecarProtocolFailureKind,
) -> Result<&'a str, PiSdkSidecarProtocolFailure> {
    required_text(value, field, kind).and_then(|text| {
        if text.len() > maximum || text.chars().any(char::is_control) {
            Err(failure(kind))
        } else {
            Ok(text)
        }
    })
}

pub(crate) fn required_u64(
    value: &Value,
    field: &str,
    kind: PiSdkSidecarProtocolFailureKind,
) -> Result<u64, PiSdkSidecarProtocolFailure> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| failure(kind))
}

pub(crate) fn failure(kind: PiSdkSidecarProtocolFailureKind) -> PiSdkSidecarProtocolFailure {
    PiSdkSidecarProtocolFailure::new(kind)
}

#[cfg(test)]
mod tests;
