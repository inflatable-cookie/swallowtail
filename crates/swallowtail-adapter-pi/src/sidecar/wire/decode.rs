use super::replay::decode_replay_item;
use super::{
    MAXIMUM_COMMAND_ID_BYTES, MAXIMUM_FAILURE_CODE_BYTES, MAXIMUM_FAILURE_MESSAGE_BYTES,
    PiSdkSidecarCommand, PiSdkSidecarDiagnostic, PiSdkSidecarDiagnosticLevel, PiSdkSidecarEvent,
    PiSdkSidecarFailure, PiSdkSidecarResponse, bounded_text, failure, required_text, required_u64,
};
use crate::sidecar::protocol::{PiSdkSidecarProtocolFailure, PiSdkSidecarProtocolFailureKind};
use serde_json::Value;
use swallowtail_runtime::TokenUsage;

pub(super) fn decode_response(
    value: &Value,
) -> Result<PiSdkSidecarResponse, PiSdkSidecarProtocolFailure> {
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

pub(super) fn decode_event(
    value: &Value,
) -> Result<PiSdkSidecarEvent, PiSdkSidecarProtocolFailure> {
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

pub(super) fn decode_terminal(
    value: &Value,
) -> Result<PiSdkSidecarFailure, PiSdkSidecarProtocolFailure> {
    let failure_record = value
        .get("failure")
        .ok_or_else(|| failure(PiSdkSidecarProtocolFailureKind::InvalidTerminal))?;
    decode_failure(
        failure_record,
        PiSdkSidecarProtocolFailureKind::InvalidTerminal,
    )
}

pub(super) fn decode_diagnostic(
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
        message: bounded_text(value, "message", MAXIMUM_FAILURE_MESSAGE_BYTES, invalid)?.to_owned(),
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
