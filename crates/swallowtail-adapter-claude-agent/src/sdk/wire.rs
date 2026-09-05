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
    pub(crate) failure_code: Option<ClaudeAgentSdkFailureCode>,
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
    pub(crate) code: ClaudeAgentSdkFailureCode,
}

/// Fixed sidecar failure vocabulary. Only these bounded labels may cross the
/// wire; provider messages, paths, and account values never do.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClaudeAgentSdkFailureCode {
    MissingEnvironment,
    InvalidCommand,
    ToolsInvalid,
    PermissionModeInvalid,
    PermissionModeRejected,
    SdkUnavailable,
    SdkExportMissing,
    NativeManifestUnavailable,
    NativeVersionMismatch,
    CapabilitiesOverflow,
    CapabilitiesInvalid,
    AccountNotFirstParty,
    AccountNotSubscription,
    AlreadyOpen,
    NodeRuntimeUnsupported,
    ConstructionFailed,
    InitializationFailed,
    CwdMismatch,
    ModelMismatch,
    ModelMissing,
    SupportedModelRejected,
    AccountUnavailable,
    NativeChildUnavailable,
    NotOpen,
    TurnActive,
    PromptTooLarge,
    InterruptFailed,
    PermissionModeUnsupported,
    PermissionModeFailed,
    PermissionModeUnconfirmed,
    UnknownCommand,
    CommandFailed,
    RecordTooLarge,
    EmptyRecord,
    MalformedJson,
    MissingType,
    UnknownRecord,
    CommandIdReused,
    TooManyPending,
    CallbackUnknown,
    CallbackInvalid,
    InternalError,
    UnknownMessage,
}

impl ClaudeAgentSdkFailureCode {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::MissingEnvironment => "missing_environment",
            Self::InvalidCommand => "invalid_command",
            Self::ToolsInvalid => "tools_invalid",
            Self::PermissionModeInvalid => "permission_mode_invalid",
            Self::PermissionModeRejected => "permission_mode_rejected",
            Self::SdkUnavailable => "sdk_unavailable",
            Self::SdkExportMissing => "sdk_export_missing",
            Self::NativeManifestUnavailable => "native_manifest_unavailable",
            Self::NativeVersionMismatch => "native_version_mismatch",
            Self::CapabilitiesOverflow => "capabilities_overflow",
            Self::CapabilitiesInvalid => "capabilities_invalid",
            Self::AccountNotFirstParty => "account_not_first_party",
            Self::AccountNotSubscription => "account_not_subscription",
            Self::AlreadyOpen => "already_open",
            Self::NodeRuntimeUnsupported => "node_runtime_unsupported",
            Self::ConstructionFailed => "construction_failed",
            Self::InitializationFailed => "initialization_failed",
            Self::CwdMismatch => "cwd_mismatch",
            Self::ModelMismatch => "model_mismatch",
            Self::ModelMissing => "model_missing",
            Self::SupportedModelRejected => "supported_model_rejected",
            Self::AccountUnavailable => "account_unavailable",
            Self::NativeChildUnavailable => "native_child_unavailable",
            Self::NotOpen => "not_open",
            Self::TurnActive => "turn_active",
            Self::PromptTooLarge => "prompt_too_large",
            Self::InterruptFailed => "interrupt_failed",
            Self::PermissionModeUnsupported => "permission_mode_unsupported",
            Self::PermissionModeFailed => "permission_mode_failed",
            Self::PermissionModeUnconfirmed => "permission_mode_unconfirmed",
            Self::UnknownCommand => "unknown_command",
            Self::CommandFailed => "command_failed",
            Self::RecordTooLarge => "record_too_large",
            Self::EmptyRecord => "empty_record",
            Self::MalformedJson => "malformed_json",
            Self::MissingType => "missing_type",
            Self::UnknownRecord => "unknown_record",
            Self::CommandIdReused => "command_id_reused",
            Self::TooManyPending => "too_many_pending",
            Self::CallbackUnknown => "callback_unknown",
            Self::CallbackInvalid => "callback_invalid",
            Self::InternalError => "internal_error",
            Self::UnknownMessage => "unknown_message",
        }
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "missing_environment" => Self::MissingEnvironment,
            "invalid_command" => Self::InvalidCommand,
            "tools_invalid" => Self::ToolsInvalid,
            "permission_mode_invalid" => Self::PermissionModeInvalid,
            "permission_mode_rejected" => Self::PermissionModeRejected,
            "sdk_unavailable" => Self::SdkUnavailable,
            "sdk_export_missing" => Self::SdkExportMissing,
            "native_manifest_unavailable" => Self::NativeManifestUnavailable,
            "native_version_mismatch" => Self::NativeVersionMismatch,
            "capabilities_overflow" => Self::CapabilitiesOverflow,
            "capabilities_invalid" => Self::CapabilitiesInvalid,
            "account_not_first_party" => Self::AccountNotFirstParty,
            "account_not_subscription" => Self::AccountNotSubscription,
            "already_open" => Self::AlreadyOpen,
            "node_runtime_unsupported" => Self::NodeRuntimeUnsupported,
            "construction_failed" => Self::ConstructionFailed,
            "initialization_failed" => Self::InitializationFailed,
            "cwd_mismatch" => Self::CwdMismatch,
            "model_mismatch" => Self::ModelMismatch,
            "model_missing" => Self::ModelMissing,
            "supported_model_rejected" => Self::SupportedModelRejected,
            "account_unavailable" => Self::AccountUnavailable,
            "native_child_unavailable" => Self::NativeChildUnavailable,
            "not_open" => Self::NotOpen,
            "turn_active" => Self::TurnActive,
            "prompt_too_large" => Self::PromptTooLarge,
            "interrupt_failed" => Self::InterruptFailed,
            "permission_mode_unsupported" => Self::PermissionModeUnsupported,
            "permission_mode_failed" => Self::PermissionModeFailed,
            "permission_mode_unconfirmed" => Self::PermissionModeUnconfirmed,
            "unknown_command" => Self::UnknownCommand,
            "command_failed" => Self::CommandFailed,
            "record_too_large" => Self::RecordTooLarge,
            "empty_record" => Self::EmptyRecord,
            "malformed_json" => Self::MalformedJson,
            "missing_type" => Self::MissingType,
            "unknown_record" => Self::UnknownRecord,
            "command_id_reused" => Self::CommandIdReused,
            "too_many_pending" => Self::TooManyPending,
            "callback_unknown" => Self::CallbackUnknown,
            "callback_invalid" => Self::CallbackInvalid,
            "internal_error" => Self::InternalError,
            "unknown_message" => Self::UnknownMessage,
            _ => return None,
        })
    }
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
