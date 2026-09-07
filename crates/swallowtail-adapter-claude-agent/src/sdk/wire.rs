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
use std::collections::BTreeMap;

mod decode;

pub(crate) const MAXIMUM_RECORD_BYTES: usize = 1024 * 1024;
pub(crate) const MAXIMUM_COMMAND_ID_BYTES: usize = 128;
pub(crate) const MAXIMUM_FAILURE_CODE_BYTES: usize = 96;
pub(crate) const MAXIMUM_FAILURE_MESSAGE_BYTES: usize = 512;
pub(crate) const MAXIMUM_TEXT_BYTES: usize = 4096;
pub(crate) const MAXIMUM_MODEL_QUALIFICATION_ID_BYTES: usize = 128;
pub(crate) const MAXIMUM_MODEL_QUALIFICATION_CATALOGUE_SIZE: usize = 64;
pub(crate) const MODEL_QUALIFICATION_DIGEST: &str = "sha256:";
pub(crate) const MODEL_QUALIFICATION_DIGEST_HEX_BYTES: usize = 32;

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
    pub(crate) original_failure_code: Option<ClaudeAgentSdkFailureCode>,
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

pub(crate) struct ClaudeAgentSdkFailure {
    pub(crate) code: ClaudeAgentSdkFailureCode,
    pub(crate) original_code: Option<ClaudeAgentSdkFailureCode>,
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
    SdkVersionMismatch,
    SdkIdentityUnverifiable,
    NativeManifestUnavailable,
    NativeVersionMismatch,
    SelectedSkillInvalid,
    SelectedSkillDigestMismatch,
    SelectedSkillLimitExceeded,
    SelectedSkillReferenceInvalid,
    SelectedSkillPayloadNotText,
    CapabilitiesOverflow,
    CapabilitiesInvalid,
    AccountNotFirstParty,
    /// Retired compatibility code: subscription evidence is observational,
    /// so current open never emits this rejection.
    AccountNotSubscription,
    AlreadyOpen,
    NodeRuntimeUnsupported,
    ConstructionFailed,
    InitializationFailed,
    InitMissing,
    SessionRejectedTerminal,
    CwdMismatch,
    ModelMismatch,
    ModelMissing,
    SupportedModelRejected,
    SupportedModelsInvalid,
    AccountUnavailable,
    ResumeCwdMismatch,
    ResumeAccountMismatch,
    ResumeSessionUnknown,
    ResumeBoundaryInvalid,
    ResumePersistenceDisabled,
    ListingFailed,
    ListingInvalid,
    NativeChildUnavailable,
    NotOpen,
    TurnActive,
    PromptTooLarge,
    InterruptFailed,
    PermissionModeUnsupported,
    PermissionModeFailed,
    PermissionModeUnconfirmed,
    ModelChangeUnsupported,
    ModelChangeFailed,
    ModelChangeUnconfirmed,
    EffortUnconfirmed,
    McpServersInvalid,
    McpServerUndeclared,
    McpServerFailed,
    McpServerNeedsAuth,
    McpStatusInvalid,
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
            Self::SdkVersionMismatch => "sdk_version_mismatch",
            Self::SdkIdentityUnverifiable => "sdk_identity_unverifiable",
            Self::NativeManifestUnavailable => "native_manifest_unavailable",
            Self::NativeVersionMismatch => "native_version_mismatch",
            Self::SelectedSkillInvalid => "selected_skill_invalid",
            Self::SelectedSkillDigestMismatch => "selected_skill_digest_mismatch",
            Self::SelectedSkillLimitExceeded => "selected_skill_limit_exceeded",
            Self::SelectedSkillReferenceInvalid => "selected_skill_reference_invalid",
            Self::SelectedSkillPayloadNotText => "selected_skill_payload_not_text",
            Self::CapabilitiesOverflow => "capabilities_overflow",
            Self::CapabilitiesInvalid => "capabilities_invalid",
            Self::AccountNotFirstParty => "account_not_first_party",
            Self::AccountNotSubscription => "account_not_subscription",
            Self::AlreadyOpen => "already_open",
            Self::NodeRuntimeUnsupported => "node_runtime_unsupported",
            Self::ConstructionFailed => "construction_failed",
            Self::InitializationFailed => "initialization_failed",
            Self::InitMissing => "init_missing",
            Self::SessionRejectedTerminal => "session_rejected_terminal",
            Self::CwdMismatch => "cwd_mismatch",
            Self::ModelMismatch => "model_mismatch",
            Self::ModelMissing => "model_missing",
            Self::SupportedModelRejected => "supported_model_rejected",
            Self::SupportedModelsInvalid => "supported_models_invalid",
            Self::AccountUnavailable => "account_unavailable",
            Self::ResumeCwdMismatch => "resume_cwd_mismatch",
            Self::ResumeAccountMismatch => "resume_account_mismatch",
            Self::ResumeSessionUnknown => "resume_session_unknown",
            Self::ResumeBoundaryInvalid => "resume_boundary_invalid",
            Self::ResumePersistenceDisabled => "resume_persistence_disabled",
            Self::ListingFailed => "listing_failed",
            Self::ListingInvalid => "listing_invalid",
            Self::NativeChildUnavailable => "native_child_unavailable",
            Self::NotOpen => "not_open",
            Self::TurnActive => "turn_active",
            Self::PromptTooLarge => "prompt_too_large",
            Self::InterruptFailed => "interrupt_failed",
            Self::PermissionModeUnsupported => "permission_mode_unsupported",
            Self::PermissionModeFailed => "permission_mode_failed",
            Self::PermissionModeUnconfirmed => "permission_mode_unconfirmed",
            Self::ModelChangeUnsupported => "model_change_unsupported",
            Self::ModelChangeFailed => "model_change_failed",
            Self::ModelChangeUnconfirmed => "model_change_unconfirmed",
            Self::EffortUnconfirmed => "effort_unconfirmed",
            Self::McpServersInvalid => "mcp_servers_invalid",
            Self::McpServerUndeclared => "mcp_server_undeclared",
            Self::McpServerFailed => "mcp_server_failed",
            Self::McpServerNeedsAuth => "mcp_server_needs_auth",
            Self::McpStatusInvalid => "mcp_status_invalid",
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
            "sdk_version_mismatch" => Self::SdkVersionMismatch,
            "sdk_identity_unverifiable" => Self::SdkIdentityUnverifiable,
            "native_manifest_unavailable" => Self::NativeManifestUnavailable,
            "native_version_mismatch" => Self::NativeVersionMismatch,
            "selected_skill_invalid" => Self::SelectedSkillInvalid,
            "selected_skill_digest_mismatch" => Self::SelectedSkillDigestMismatch,
            "selected_skill_limit_exceeded" => Self::SelectedSkillLimitExceeded,
            "selected_skill_reference_invalid" => Self::SelectedSkillReferenceInvalid,
            "selected_skill_payload_not_text" => Self::SelectedSkillPayloadNotText,
            "capabilities_overflow" => Self::CapabilitiesOverflow,
            "capabilities_invalid" => Self::CapabilitiesInvalid,
            "account_not_first_party" => Self::AccountNotFirstParty,
            "account_not_subscription" => Self::AccountNotSubscription,
            "already_open" => Self::AlreadyOpen,
            "node_runtime_unsupported" => Self::NodeRuntimeUnsupported,
            "construction_failed" => Self::ConstructionFailed,
            "initialization_failed" => Self::InitializationFailed,
            "init_missing" => Self::InitMissing,
            "session_rejected_terminal" => Self::SessionRejectedTerminal,
            "cwd_mismatch" => Self::CwdMismatch,
            "model_mismatch" => Self::ModelMismatch,
            "model_missing" => Self::ModelMissing,
            "supported_model_rejected" => Self::SupportedModelRejected,
            "supported_models_invalid" => Self::SupportedModelsInvalid,
            "account_unavailable" => Self::AccountUnavailable,
            "resume_cwd_mismatch" => Self::ResumeCwdMismatch,
            "resume_account_mismatch" => Self::ResumeAccountMismatch,
            "resume_session_unknown" => Self::ResumeSessionUnknown,
            "resume_boundary_invalid" => Self::ResumeBoundaryInvalid,
            "resume_persistence_disabled" => Self::ResumePersistenceDisabled,
            "listing_failed" => Self::ListingFailed,
            "listing_invalid" => Self::ListingInvalid,
            "native_child_unavailable" => Self::NativeChildUnavailable,
            "not_open" => Self::NotOpen,
            "turn_active" => Self::TurnActive,
            "prompt_too_large" => Self::PromptTooLarge,
            "interrupt_failed" => Self::InterruptFailed,
            "permission_mode_unsupported" => Self::PermissionModeUnsupported,
            "permission_mode_failed" => Self::PermissionModeFailed,
            "permission_mode_unconfirmed" => Self::PermissionModeUnconfirmed,
            "model_change_unsupported" => Self::ModelChangeUnsupported,
            "model_change_failed" => Self::ModelChangeFailed,
            "model_change_unconfirmed" => Self::ModelChangeUnconfirmed,
            "effort_unconfirmed" => Self::EffortUnconfirmed,
            "mcp_servers_invalid" => Self::McpServersInvalid,
            "mcp_server_undeclared" => Self::McpServerUndeclared,
            "mcp_server_failed" => Self::McpServerFailed,
            "mcp_server_needs_auth" => Self::McpServerNeedsAuth,
            "mcp_status_invalid" => Self::McpStatusInvalid,
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

pub(crate) struct ClaudeAgentSdkDiagnostic {
    #[allow(dead_code)]
    pub(crate) level: ClaudeAgentSdkDiagnosticLevel,
    pub(crate) code: String,
    pub(crate) evidence: Option<ClaudeAgentSdkDiagnosticEvidence>,
}

pub(crate) enum ClaudeAgentSdkDiagnosticEvidence {
    ModelQualification(ClaudeAgentSdkModelQualificationEvidence),
    SdkIdentity(ClaudeAgentSdkIdentityEvidence),
}

pub(crate) struct ClaudeAgentSdkModelQualificationEvidence {
    pub(crate) requested_model: Option<String>,
    pub(crate) effective_model: Option<String>,
    pub(crate) catalogue_size: usize,
    pub(crate) catalogue_digest: String,
    pub(crate) requested_membership: bool,
    pub(crate) effective_membership: bool,
    pub(crate) query_source: String,
    pub(crate) phase: String,
    pub(crate) declared_sdk_version: String,
    pub(crate) loaded_sdk_version: String,
    pub(crate) native_version: String,
}

pub(crate) struct ClaudeAgentSdkIdentityEvidence {
    pub(crate) declared_sdk_package: String,
    pub(crate) declared_sdk_version: String,
    pub(crate) loaded_sdk_package: String,
    pub(crate) loaded_sdk_version: String,
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
    ToolStarted {
        call_id: String,
        name: String,
    },
    ToolEnded {
        call_id: String,
        failed: bool,
    },
    TurnEnded {
        stop_reason: String,
        failed: bool,
        subtype: Option<String>,
        num_turns: Option<u64>,
        duration_ms: Option<u64>,
        error_text_present: bool,
        error_text_type: String,
        result_field_presence: BTreeMap<String, bool>,
    },
    TurnFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClaudeAgentSdkCommand {
    Open,
    Query,
    Interrupt,
    SetPermissionMode,
    SetModel,
    ListSessions,
    Close,
}

impl ClaudeAgentSdkCommand {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Query => "query",
            Self::Interrupt => "interrupt",
            Self::SetPermissionMode => "set_permission_mode",
            Self::SetModel => "set_model",
            Self::ListSessions => "list_sessions",
            Self::Close => "close",
        }
    }

    fn from_qualified(name: &str) -> Option<Self> {
        Some(match name {
            "open" => Self::Open,
            "query" => Self::Query,
            "interrupt" => Self::Interrupt,
            "set_permission_mode" => Self::SetPermissionMode,
            "set_model" => Self::SetModel,
            "list_sessions" => Self::ListSessions,
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
