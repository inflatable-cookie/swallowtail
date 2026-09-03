use super::{
    ClaudeAgentSdkCallback, ClaudeAgentSdkCommand, ClaudeAgentSdkDiagnostic,
    ClaudeAgentSdkDiagnosticLevel, ClaudeAgentSdkEvent, ClaudeAgentSdkFailure,
    ClaudeAgentSdkResponse, MAXIMUM_COMMAND_ID_BYTES, MAXIMUM_FAILURE_CODE_BYTES,
    MAXIMUM_FAILURE_MESSAGE_BYTES, MAXIMUM_TEXT_BYTES, bounded_text, failure, required_bool,
};
use crate::sdk::protocol::{ClaudeAgentSdkProtocolFailure, ClaudeAgentSdkProtocolFailureKind};
use serde_json::Value;

pub(super) fn decode_response(
    value: &Value,
) -> Result<ClaudeAgentSdkResponse, ClaudeAgentSdkProtocolFailure> {
    let invalid = ClaudeAgentSdkProtocolFailureKind::InvalidResponse;
    let id = bounded_text(value, "id", MAXIMUM_COMMAND_ID_BYTES, invalid)?.to_owned();
    let command = bounded_text(value, "command", MAXIMUM_COMMAND_ID_BYTES, invalid)?;
    if ClaudeAgentSdkCommand::from_qualified(command).is_none() {
        return Err(failure(invalid));
    }
    let success = required_bool(value, "success", invalid)?;
    let data = value.get("data").cloned();
    match (success, &data, value.get("failure")) {
        (true, _, None) => {
            if data.as_ref().is_some_and(|data| !data.is_object()) {
                return Err(failure(invalid));
            }
            Ok(ClaudeAgentSdkResponse {
                id,
                command: command.to_owned(),
                success,
                data,
            })
        }
        (false, None, Some(record)) => {
            decode_failure(record, invalid)?;
            Ok(ClaudeAgentSdkResponse {
                id,
                command: command.to_owned(),
                success,
                data: None,
            })
        }
        _ => Err(failure(invalid)),
    }
}

pub(super) fn decode_event(
    value: &Value,
) -> Result<ClaudeAgentSdkEvent, ClaudeAgentSdkProtocolFailure> {
    let invalid = ClaudeAgentSdkProtocolFailureKind::InvalidEvent;
    match value.get("event").and_then(Value::as_str) {
        Some("turn_started") => Ok(ClaudeAgentSdkEvent::TurnStarted),
        Some("progress") => Ok(ClaudeAgentSdkEvent::Progress),
        Some("turn_failed") => Ok(ClaudeAgentSdkEvent::TurnFailed),
        Some("output_delta") => {
            let delta = value
                .get("delta")
                .and_then(Value::as_str)
                .ok_or_else(|| failure(invalid))?;
            if delta.len() > super::MAXIMUM_RECORD_BYTES {
                return Err(failure(invalid));
            }
            Ok(ClaudeAgentSdkEvent::OutputDelta(delta.to_owned()))
        }
        Some("tool_started") => Ok(ClaudeAgentSdkEvent::ToolStarted {
            call_id: bounded_text(value, "toolCallId", MAXIMUM_TEXT_BYTES, invalid)?.to_owned(),
            name: bounded_text(value, "toolName", MAXIMUM_TEXT_BYTES, invalid)?.to_owned(),
        }),
        Some("tool_ended") => Ok(ClaudeAgentSdkEvent::ToolEnded {
            call_id: bounded_text(value, "toolCallId", MAXIMUM_TEXT_BYTES, invalid)?.to_owned(),
            failed: required_bool(value, "isError", invalid)?,
        }),
        Some("turn_ended") => Ok(ClaudeAgentSdkEvent::TurnEnded {
            stop_reason: bounded_text(value, "stopReason", MAXIMUM_TEXT_BYTES, invalid)?.to_owned(),
            failed: required_bool(value, "isError", invalid)?,
        }),
        Some(_) => Err(failure(ClaudeAgentSdkProtocolFailureKind::UnknownRecord)),
        None => Err(failure(ClaudeAgentSdkProtocolFailureKind::MissingType)),
    }
}

pub(super) fn decode_callback(
    value: &Value,
) -> Result<ClaudeAgentSdkCallback, ClaudeAgentSdkProtocolFailure> {
    let invalid = ClaudeAgentSdkProtocolFailureKind::InvalidCallback;
    if value.get("callback").and_then(Value::as_str) != Some("can_use_tool") {
        return Err(failure(invalid));
    }
    Ok(ClaudeAgentSdkCallback {
        id: bounded_text(value, "id", MAXIMUM_COMMAND_ID_BYTES, invalid)?.to_owned(),
        tool_name: bounded_text(value, "toolName", MAXIMUM_TEXT_BYTES, invalid)?.to_owned(),
    })
}

pub(super) fn decode_terminal(
    value: &Value,
) -> Result<ClaudeAgentSdkFailure, ClaudeAgentSdkProtocolFailure> {
    let record = value
        .get("failure")
        .ok_or_else(|| failure(ClaudeAgentSdkProtocolFailureKind::InvalidTerminal))?;
    decode_failure(record, ClaudeAgentSdkProtocolFailureKind::InvalidTerminal)
}

pub(super) fn decode_diagnostic(
    value: &Value,
) -> Result<ClaudeAgentSdkDiagnostic, ClaudeAgentSdkProtocolFailure> {
    let invalid = ClaudeAgentSdkProtocolFailureKind::InvalidDiagnostic;
    let level = match value.get("level").and_then(Value::as_str) {
        Some("info") => ClaudeAgentSdkDiagnosticLevel::Info,
        Some("warning") => ClaudeAgentSdkDiagnosticLevel::Warning,
        Some("error") => ClaudeAgentSdkDiagnosticLevel::Error,
        _ => return Err(failure(invalid)),
    };
    bounded_text(value, "message", MAXIMUM_FAILURE_MESSAGE_BYTES, invalid)?;
    Ok(ClaudeAgentSdkDiagnostic {
        level,
        code: bounded_text(value, "code", MAXIMUM_FAILURE_CODE_BYTES, invalid)?.to_owned(),
    })
}

fn decode_failure(
    value: &Value,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<ClaudeAgentSdkFailure, ClaudeAgentSdkProtocolFailure> {
    bounded_text(value, "message", MAXIMUM_FAILURE_MESSAGE_BYTES, kind)?;
    Ok(ClaudeAgentSdkFailure {
        code: bounded_text(value, "code", MAXIMUM_FAILURE_CODE_BYTES, kind)?.to_owned(),
    })
}
