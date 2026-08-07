use super::ui;
use super::{
    OhMyPiAgentEvent, OhMyPiRpcProtocolFailure, OhMyPiRpcProtocolFailureKind, OhMyPiRpcRecord,
    OhMyPiRpcResponse,
};
use serde_json::Value;
use swallowtail_runtime::TokenUsage;

pub(super) fn decode_value(value: &Value) -> Result<OhMyPiRpcRecord, OhMyPiRpcProtocolFailure> {
    match value.get("type").and_then(Value::as_str) {
        Some("response") => decode_response(value).map(OhMyPiRpcRecord::Response),
        Some("extension_ui_request") => ui::decode_ui(value),
        Some("ready") => decode_ready(value),
        Some(
            "available_commands_update"
            | "prompt_result"
            | "command_output"
            | "config_update"
            | "session_info_update"
            | "model_changed"
            | "thinking_level_changed",
        ) => Ok(OhMyPiRpcRecord::Lifecycle),
        Some(kind) => decode_event(kind, value).map(OhMyPiRpcRecord::AgentEvent),
        None => Err(failure(OhMyPiRpcProtocolFailureKind::MissingType)),
    }
}

pub(super) fn decode_ready(value: &Value) -> Result<OhMyPiRpcRecord, OhMyPiRpcProtocolFailure> {
    let supported = value
        .get("supportedProtocolVersions")
        .and_then(Value::as_array)
        .ok_or_else(|| failure(OhMyPiRpcProtocolFailureKind::UnknownRecord))?;
    if value.get("protocolVersion").and_then(Value::as_u64) != Some(1)
        || supported.as_slice() != [Value::from(1), Value::from(2)]
        || value.get("maxFrameBytes").and_then(Value::as_u64)
            != Some(super::MAXIMUM_RECORD_BYTES as u64)
        || value
            .get("maxReassembledFrameBytes")
            .and_then(Value::as_u64)
            != Some(super::MAXIMUM_REASSEMBLED_BYTES as u64)
    {
        return Err(failure(OhMyPiRpcProtocolFailureKind::UnknownRecord));
    }
    Ok(OhMyPiRpcRecord::Lifecycle)
}

pub(super) fn decode_response(
    value: &Value,
) -> Result<OhMyPiRpcResponse, OhMyPiRpcProtocolFailure> {
    Ok(OhMyPiRpcResponse {
        id: required_text(value, "id", OhMyPiRpcProtocolFailureKind::InvalidResponse)?.to_owned(),
        command: required_text(
            value,
            "command",
            OhMyPiRpcProtocolFailureKind::InvalidResponse,
        )?
        .to_owned(),
        success: value
            .get("success")
            .and_then(Value::as_bool)
            .ok_or_else(|| failure(OhMyPiRpcProtocolFailureKind::InvalidResponse))?,
        data: value.get("data").cloned(),
    })
}

pub(super) fn decode_event(
    kind: &str,
    value: &Value,
) -> Result<OhMyPiAgentEvent, OhMyPiRpcProtocolFailure> {
    match kind {
        "agent_start" => Ok(OhMyPiAgentEvent::Started),
        "message_start" => decode_message_start(value),
        "message_update" => decode_message_update(value),
        "message_end" => decode_message_end(value),
        "tool_execution_start" => decode_tool(value, ToolPhase::Started),
        "tool_execution_update" => decode_tool(value, ToolPhase::Updated),
        "tool_execution_end" => decode_tool(value, ToolPhase::Ended),
        "auto_compaction_start" => Ok(OhMyPiAgentEvent::CompactionStarted),
        "auto_compaction_end" => Ok(OhMyPiAgentEvent::CompactionEnded),
        "auto_retry_start"
        | "auto_retry_end"
        | "summarization_retry_scheduled"
        | "summarization_retry_attempt_start"
        | "summarization_retry_finished" => Ok(OhMyPiAgentEvent::RetryObserved),
        "agent_end" => match value.get("isTerminal").and_then(Value::as_bool) {
            Some(true) => Ok(OhMyPiAgentEvent::Settled),
            Some(false) => Ok(OhMyPiAgentEvent::Progress),
            None => Err(failure(OhMyPiRpcProtocolFailureKind::UnknownRecord)),
        },
        "turn_start" | "turn_end" | "queue_update" => Ok(OhMyPiAgentEvent::Progress),
        "extension_error" => Ok(OhMyPiAgentEvent::ProviderFailed),
        _ => bounded_namespace(kind)
            .map(OhMyPiAgentEvent::Unknown)
            .ok_or_else(|| failure(OhMyPiRpcProtocolFailureKind::UnknownRecord)),
    }
}

pub(super) fn decode_message_end(
    value: &Value,
) -> Result<OhMyPiAgentEvent, OhMyPiRpcProtocolFailure> {
    let message = value
        .get("message")
        .ok_or_else(|| failure(OhMyPiRpcProtocolFailureKind::UnknownRecord))?;
    if message.get("role").and_then(Value::as_str) != Some("assistant") {
        return Ok(OhMyPiAgentEvent::Progress);
    }
    if message.get("stopReason").and_then(Value::as_str) == Some("error") {
        return Ok(OhMyPiAgentEvent::ProviderFailed);
    }
    let usage = message.get("usage");
    let Some(usage) = usage else {
        return Ok(OhMyPiAgentEvent::MessageEnded(None));
    };
    let input = required_u64(usage, "input")?;
    let output = required_u64(usage, "output")?;
    let cache_read = required_u64(usage, "cacheRead")?;
    let cache_write = required_u64(usage, "cacheWrite")?;
    Ok(OhMyPiAgentEvent::MessageEnded(Some(
        TokenUsage::new(Some(input), Some(output))
            .with_cache_tokens(Some(cache_read), Some(cache_write)),
    )))
}

pub(super) fn decode_message_start(
    value: &Value,
) -> Result<OhMyPiAgentEvent, OhMyPiRpcProtocolFailure> {
    let message = value
        .get("message")
        .ok_or_else(|| failure(OhMyPiRpcProtocolFailureKind::UnknownRecord))?;
    if message.get("role").and_then(Value::as_str) == Some("assistant") {
        Ok(OhMyPiAgentEvent::MessageStarted)
    } else {
        Ok(OhMyPiAgentEvent::Progress)
    }
}

pub(super) fn decode_message_update(
    value: &Value,
) -> Result<OhMyPiAgentEvent, OhMyPiRpcProtocolFailure> {
    let event = value
        .get("assistantMessageEvent")
        .ok_or_else(|| failure(OhMyPiRpcProtocolFailureKind::UnknownRecord))?;
    match event.get("type").and_then(Value::as_str) {
        Some("text_delta") => {
            required_text(event, "delta", OhMyPiRpcProtocolFailureKind::UnknownRecord)
                .map(|delta| OhMyPiAgentEvent::OutputDelta(delta.to_owned()))
        }
        Some("thinking_delta") => {
            required_text(event, "delta", OhMyPiRpcProtocolFailureKind::UnknownRecord)
                .map(|delta| OhMyPiAgentEvent::ReasoningDelta(delta.to_owned()))
        }
        Some("thinking_start") => Ok(OhMyPiAgentEvent::ReasoningStarted),
        Some("thinking_end") => Ok(OhMyPiAgentEvent::ReasoningEnded),
        Some(
            "start" | "text_start" | "text_end" | "toolcall_start" | "toolcall_delta"
            | "toolcall_end" | "done" | "error",
        ) => Ok(OhMyPiAgentEvent::Progress),
        _ => Err(failure(OhMyPiRpcProtocolFailureKind::UnknownRecord)),
    }
}

#[derive(Clone, Copy)]
pub(super) enum ToolPhase {
    Started,
    Updated,
    Ended,
}

pub(super) fn decode_tool(
    value: &Value,
    phase: ToolPhase,
) -> Result<OhMyPiAgentEvent, OhMyPiRpcProtocolFailure> {
    let call_id = required_text(
        value,
        "toolCallId",
        OhMyPiRpcProtocolFailureKind::UnknownRecord,
    )?
    .to_owned();
    let name = required_text(
        value,
        "toolName",
        OhMyPiRpcProtocolFailureKind::UnknownRecord,
    )?
    .to_owned();
    Ok(match phase {
        ToolPhase::Started => OhMyPiAgentEvent::ToolStarted { call_id, name },
        ToolPhase::Updated => OhMyPiAgentEvent::ToolUpdated { call_id, name },
        ToolPhase::Ended => OhMyPiAgentEvent::ToolEnded {
            call_id,
            name,
            failed: value.get("isError").and_then(Value::as_bool) == Some(true),
        },
    })
}

pub(super) fn bounded_namespace(value: &str) -> Option<String> {
    if value.is_empty() || value.len() > 96 || value.chars().any(char::is_control) {
        None
    } else {
        Some(value.to_owned())
    }
}

pub(super) fn required_text<'a>(
    value: &'a Value,
    field: &str,
    kind: OhMyPiRpcProtocolFailureKind,
) -> Result<&'a str, OhMyPiRpcProtocolFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| failure(kind))
}

pub(super) fn required_u64(value: &Value, field: &str) -> Result<u64, OhMyPiRpcProtocolFailure> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| failure(OhMyPiRpcProtocolFailureKind::UnknownRecord))
}

pub(super) fn failure(kind: OhMyPiRpcProtocolFailureKind) -> OhMyPiRpcProtocolFailure {
    OhMyPiRpcProtocolFailure::new(kind)
}
