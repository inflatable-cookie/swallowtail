//! Private strict LF-JSON wire between the driver and the Node sidecar.
//!
//! Every record is bounded and correlated. Unknown record types, unknown
//! event names, oversized records, and partial final records fail closed.
//! Raw SDK values, credentials, paths, and provider payloads never appear in
//! decoded records.

use super::protocol::{
    ClaudeAgentSdkProtocolFailure, ClaudeAgentSdkProtocolFailureKind, ClaudeAgentSdkRecordKind,
};
use serde_json::Value;

mod decode;

pub(crate) const MAXIMUM_RECORD_BYTES: usize = 1024 * 1024;
pub(crate) const MAXIMUM_COMMAND_ID_BYTES: usize = 128;
pub(crate) const MAXIMUM_FAILURE_CODE_BYTES: usize = 96;
pub(crate) const MAXIMUM_FAILURE_MESSAGE_BYTES: usize = 512;
pub(crate) const MAXIMUM_TEXT_BYTES: usize = 4096;

pub(crate) struct ClaudeAgentSdkDecoder {
    buffer: Vec<u8>,
}

impl ClaudeAgentSdkDecoder {
    pub(crate) const fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub(crate) fn push(
        &mut self,
        bytes: &[u8],
    ) -> Result<Vec<ClaudeAgentSdkRecord>, ClaudeAgentSdkProtocolFailure> {
        self.buffer.extend_from_slice(bytes);
        if self.buffer.len() > MAXIMUM_RECORD_BYTES && !self.buffer.contains(&b'\n') {
            return Err(failure(ClaudeAgentSdkProtocolFailureKind::RecordTooLarge));
        }
        let mut records = Vec::new();
        while let Some(end) = self.buffer.iter().position(|byte| *byte == b'\n') {
            if end > MAXIMUM_RECORD_BYTES {
                return Err(failure(ClaudeAgentSdkProtocolFailureKind::RecordTooLarge));
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

    pub(crate) fn finish(self) -> Result<(), ClaudeAgentSdkProtocolFailure> {
        if self.buffer.is_empty() {
            Ok(())
        } else {
            Err(failure(
                ClaudeAgentSdkProtocolFailureKind::MissingLfDelimiter,
            ))
        }
    }
}

pub(crate) enum ClaudeAgentSdkRecord {
    Response(ClaudeAgentSdkResponse),
    Event(ClaudeAgentSdkEvent),
    Callback(ClaudeAgentSdkCallback),
    // Terminal and diagnostic payloads stay redacted: the driver maps them to
    // one distinct safe failure or drops them without surfacing content. The
    // frozen corpus proves the decoded shape.
    #[allow(dead_code)]
    Terminal(ClaudeAgentSdkFailure),
    #[allow(dead_code)]
    Diagnostic(ClaudeAgentSdkDiagnostic),
}

impl ClaudeAgentSdkRecord {
    pub(crate) const fn kind(&self) -> ClaudeAgentSdkRecordKind {
        match self {
            Self::Response(_) => ClaudeAgentSdkRecordKind::Response,
            Self::Event(_) => ClaudeAgentSdkRecordKind::Event,
            Self::Callback(_) => ClaudeAgentSdkRecordKind::Callback,
            Self::Terminal(_) => ClaudeAgentSdkRecordKind::Terminal,
            Self::Diagnostic(_) => ClaudeAgentSdkRecordKind::Diagnostic,
        }
    }
}

pub(crate) struct ClaudeAgentSdkResponse {
    pub(crate) id: String,
    pub(crate) command: String,
    pub(crate) success: bool,
    pub(crate) data: Option<Value>,
}

pub(crate) struct ClaudeAgentSdkCallback {
    pub(crate) id: String,
    pub(crate) tool_name: String,
    pub(crate) bash_command: Option<ClaudeAgentSdkBashCommandView>,
}

/// Bounded consumer-visible fields for one admitted Bash call.
pub(crate) struct ClaudeAgentSdkBashCommandView {
    pub(crate) command: String,
    pub(crate) command_byte_length: usize,
    pub(crate) description: String,
    pub(crate) truncated: bool,
}

#[allow(dead_code)]
pub(crate) struct ClaudeAgentSdkFailure {
    pub(crate) code: String,
}

#[allow(dead_code)]
pub(crate) struct ClaudeAgentSdkDiagnostic {
    pub(crate) level: ClaudeAgentSdkDiagnosticLevel,
    pub(crate) code: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClaudeAgentSdkDiagnosticLevel {
    Info,
    Warning,
    Error,
}

pub(crate) enum ClaudeAgentSdkEvent {
    TurnStarted,
    Progress,
    OutputDelta(String),
    ToolStarted { call_id: String, name: String },
    ToolEnded { call_id: String, failed: bool },
    TurnEnded { stop_reason: String, failed: bool },
    TurnFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClaudeAgentSdkCommand {
    Open,
    Query,
    Interrupt,
    SetPermissionMode,
    Close,
}

impl ClaudeAgentSdkCommand {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Query => "query",
            Self::Interrupt => "interrupt",
            Self::SetPermissionMode => "set_permission_mode",
            Self::Close => "close",
        }
    }

    fn from_qualified(name: &str) -> Option<Self> {
        Some(match name {
            "open" => Self::Open,
            "query" => Self::Query,
            "interrupt" => Self::Interrupt,
            "set_permission_mode" => Self::SetPermissionMode,
            "close" => Self::Close,
            _ => return None,
        })
    }
}

/// Host decision for one correlated `canUseTool` admission request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClaudeAgentSdkToolDecision {
    Allow,
    Deny,
}

impl ClaudeAgentSdkToolDecision {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
        }
    }
}

pub(crate) fn encode_command(
    id: &str,
    command: ClaudeAgentSdkCommand,
    params: Value,
) -> Result<Vec<u8>, ClaudeAgentSdkProtocolFailure> {
    encode(serde_json::json!({
        "type": "command",
        "id": id,
        "command": command.as_str(),
        "params": params,
    }))
}

pub(crate) fn encode_callback_response(
    id: &str,
    decision: ClaudeAgentSdkToolDecision,
) -> Result<Vec<u8>, ClaudeAgentSdkProtocolFailure> {
    encode(serde_json::json!({
        "type": "callback_response",
        "id": id,
        "decision": decision.as_str(),
    }))
}

fn encode(record: Value) -> Result<Vec<u8>, ClaudeAgentSdkProtocolFailure> {
    let mut bytes = serde_json::to_vec(&record)
        .map_err(|_| failure(ClaudeAgentSdkProtocolFailureKind::MalformedJson))?;
    if bytes.len() + 1 > MAXIMUM_RECORD_BYTES {
        return Err(failure(ClaudeAgentSdkProtocolFailureKind::RecordTooLarge));
    }
    bytes.push(b'\n');
    Ok(bytes)
}

pub(crate) fn decode_record(
    bytes: &[u8],
) -> Result<ClaudeAgentSdkRecord, ClaudeAgentSdkProtocolFailure> {
    if bytes.is_empty() {
        return Err(failure(ClaudeAgentSdkProtocolFailureKind::EmptyRecord));
    }
    let value: Value = serde_json::from_slice(bytes)
        .map_err(|_| failure(ClaudeAgentSdkProtocolFailureKind::MalformedJson))?;
    match value.get("type").and_then(Value::as_str) {
        Some("response") => decode::decode_response(&value).map(ClaudeAgentSdkRecord::Response),
        Some("event") => decode::decode_event(&value).map(ClaudeAgentSdkRecord::Event),
        Some("callback") => decode::decode_callback(&value).map(ClaudeAgentSdkRecord::Callback),
        Some("terminal") => decode::decode_terminal(&value).map(ClaudeAgentSdkRecord::Terminal),
        Some("diagnostic") => {
            decode::decode_diagnostic(&value).map(ClaudeAgentSdkRecord::Diagnostic)
        }
        Some(_) => Err(failure(ClaudeAgentSdkProtocolFailureKind::UnknownRecord)),
        None => Err(failure(ClaudeAgentSdkProtocolFailureKind::MissingType)),
    }
}

pub(crate) fn bounded_text<'a>(
    value: &'a Value,
    field: &str,
    maximum: usize,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<&'a str, ClaudeAgentSdkProtocolFailure> {
    let text = value
        .get(field)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
        .ok_or_else(|| failure(kind))?;
    if text.len() > maximum || text.chars().any(char::is_control) {
        Err(failure(kind))
    } else {
        Ok(text)
    }
}

pub(crate) fn required_bool(
    value: &Value,
    field: &str,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<bool, ClaudeAgentSdkProtocolFailure> {
    value
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| failure(kind))
}

pub(crate) const fn failure(
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> ClaudeAgentSdkProtocolFailure {
    ClaudeAgentSdkProtocolFailure::new(kind)
}

#[cfg(test)]
mod wire_tests;
