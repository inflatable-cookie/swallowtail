use super::{
    AcpBoundedText, AcpMessageChunk, AcpMessageRole, AcpSessionUpdate, ActivityDecodeError,
    ActivityDecodeErrorKind, ActivityDecodeLimits, DecodedSessionUpdate,
};
use fields::{error, object, optional_identifier, required_identifier, validate_identifier};
use serde_json::{Map, Value};

mod config;
mod content;
mod fields;
mod metadata;
mod tool;

pub(super) fn decode(
    params: &Value,
    limits: ActivityDecodeLimits,
) -> Result<DecodedSessionUpdate, ActivityDecodeError> {
    let encoded =
        serde_json::to_vec(params).map_err(|_| error(ActivityDecodeErrorKind::ContentInvalid))?;
    if encoded.len() > limits.maximum_update_bytes() {
        return Err(error(ActivityDecodeErrorKind::LimitExceeded));
    }
    let params = object(params, ActivityDecodeErrorKind::ContentInvalid)?;
    let session_id = required_identifier(
        params,
        "sessionId",
        ActivityDecodeErrorKind::SessionIdMissing,
        limits,
    )?;
    let update = params
        .get("update")
        .and_then(Value::as_object)
        .ok_or_else(|| error(ActivityDecodeErrorKind::UpdateKindMissing))?;
    let kind = update
        .get("sessionUpdate")
        .ok_or_else(|| error(ActivityDecodeErrorKind::UpdateKindMissing))?
        .as_str()
        .ok_or_else(|| error(ActivityDecodeErrorKind::UpdateKindInvalid))?;
    validate_identifier(kind, limits)
        .map_err(|_| error(ActivityDecodeErrorKind::UpdateKindInvalid))?;
    let update = match kind {
        "user_message_chunk" => message(update, AcpMessageRole::User, limits)?,
        "agent_message_chunk" => message(update, AcpMessageRole::Agent, limits)?,
        "agent_thought_chunk" => message(update, AcpMessageRole::Thought, limits)?,
        "tool_call" => AcpSessionUpdate::ToolCall(tool::create(update, limits)?),
        "tool_call_update" => AcpSessionUpdate::ToolCallUpdate(tool::update(update, limits)?),
        "plan" => AcpSessionUpdate::Plan(metadata::plan(update, limits)?),
        "available_commands_update" => {
            AcpSessionUpdate::AvailableCommands(metadata::commands(update, limits)?)
        }
        "current_mode_update" => AcpSessionUpdate::CurrentMode(required_identifier(
            update,
            "currentModeId",
            ActivityDecodeErrorKind::MetadataInvalid,
            limits,
        )?),
        "config_option_update" => AcpSessionUpdate::ConfigOptions(config::options(update, limits)?),
        "session_info_update" => metadata::session_info(update)?,
        "usage_update" => AcpSessionUpdate::Usage(metadata::usage(update, limits)?),
        _ => AcpSessionUpdate::Unknown {
            namespace: AcpBoundedText(kind.to_owned()),
        },
    };
    Ok(DecodedSessionUpdate { session_id, update })
}

fn message(
    update: &Map<String, Value>,
    role: AcpMessageRole,
    limits: ActivityDecodeLimits,
) -> Result<AcpSessionUpdate, ActivityDecodeError> {
    let message_id = optional_identifier(update, "messageId", limits)?;
    let content = update
        .get("content")
        .ok_or_else(|| error(ActivityDecodeErrorKind::ContentInvalid))
        .and_then(|value| content::block(value, limits))?;
    Ok(AcpSessionUpdate::Message(AcpMessageChunk {
        role,
        message_id,
        content,
    }))
}
