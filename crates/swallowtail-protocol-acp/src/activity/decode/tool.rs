use super::content;
use super::fields::{
    error, object, optional_array, optional_identifier, optional_text, required_identifier,
    required_str, required_text, required_u64,
};
use crate::activity::{
    AcpBoundedText, AcpToolCall, AcpToolCallContent, AcpToolCallLocation, AcpToolCallStatus,
    AcpToolCallUpdate, AcpToolKind, ActivityDecodeError, ActivityDecodeErrorKind,
    ActivityDecodeLimits,
};
use serde_json::{Map, Value};

pub(super) fn create(
    update: &Map<String, Value>,
    limits: ActivityDecodeLimits,
) -> Result<AcpToolCall, ActivityDecodeError> {
    Ok(AcpToolCall {
        tool_call_id: tool_id(update, limits)?,
        title: required_text(update, "title", ActivityDecodeErrorKind::ContentInvalid)?,
        kind: optional_kind(update, limits)?.unwrap_or_else(default_tool_kind),
        status: optional_status(update)?.unwrap_or(AcpToolCallStatus::Pending),
        content: optional_array(
            update,
            "content",
            ActivityDecodeErrorKind::ContentInvalid,
            limits,
        )?
        .map(|content| tool_content(content, limits))
        .transpose()?
        .unwrap_or_default(),
        locations: optional_array(
            update,
            "locations",
            ActivityDecodeErrorKind::ContentInvalid,
            limits,
        )?
        .map(|locations| tool_locations(locations, limits))
        .transpose()?
        .unwrap_or_default(),
    })
}

pub(super) fn update(
    update: &Map<String, Value>,
    limits: ActivityDecodeLimits,
) -> Result<AcpToolCallUpdate, ActivityDecodeError> {
    Ok(AcpToolCallUpdate {
        tool_call_id: tool_id(update, limits)?,
        title: optional_text(update, "title", ActivityDecodeErrorKind::ContentInvalid)?,
        kind: optional_kind(update, limits)?,
        status: optional_status(update)?,
        content_replacement: optional_array(
            update,
            "content",
            ActivityDecodeErrorKind::ContentInvalid,
            limits,
        )?
        .map(|content| tool_content(content, limits))
        .transpose()?,
        locations_replacement: optional_array(
            update,
            "locations",
            ActivityDecodeErrorKind::ContentInvalid,
            limits,
        )?
        .map(|locations| tool_locations(locations, limits))
        .transpose()?,
    })
}

fn tool_id(
    update: &Map<String, Value>,
    limits: ActivityDecodeLimits,
) -> Result<AcpBoundedText, ActivityDecodeError> {
    required_identifier(
        update,
        "toolCallId",
        ActivityDecodeErrorKind::ToolIdentityMissing,
        limits,
    )
}

fn optional_kind(
    update: &Map<String, Value>,
    limits: ActivityDecodeLimits,
) -> Result<Option<AcpToolKind>, ActivityDecodeError> {
    let Some(kind) = optional_identifier(update, "kind", limits)? else {
        return Ok(None);
    };
    let kind = match kind.as_str() {
        "read" => AcpToolKind::Read,
        "edit" => AcpToolKind::Edit,
        "delete" => AcpToolKind::Delete,
        "move" => AcpToolKind::Move,
        "search" => AcpToolKind::Search,
        "execute" => AcpToolKind::Execute,
        "think" => AcpToolKind::Think,
        "fetch" => AcpToolKind::Fetch,
        "switch_mode" => AcpToolKind::SwitchMode,
        _ => AcpToolKind::Other(kind),
    };
    Ok(Some(kind))
}

fn default_tool_kind() -> AcpToolKind {
    AcpToolKind::Other(AcpBoundedText("other".to_owned()))
}

fn optional_status(
    update: &Map<String, Value>,
) -> Result<Option<AcpToolCallStatus>, ActivityDecodeError> {
    let status = match update.get("status") {
        None | Some(Value::Null) => return Ok(None),
        Some(status) => status,
    };
    let status = status
        .as_str()
        .ok_or_else(|| error(ActivityDecodeErrorKind::ToolStatusInvalid))?;
    Ok(Some(match status {
        "pending" => AcpToolCallStatus::Pending,
        "in_progress" => AcpToolCallStatus::InProgress,
        "completed" => AcpToolCallStatus::Completed,
        "failed" => AcpToolCallStatus::Failed,
        _ => return Err(error(ActivityDecodeErrorKind::ToolStatusInvalid)),
    }))
}

fn tool_content(
    values: &[Value],
    limits: ActivityDecodeLimits,
) -> Result<Vec<AcpToolCallContent>, ActivityDecodeError> {
    values
        .iter()
        .map(|value| {
            let value = object(value, ActivityDecodeErrorKind::ContentInvalid)?;
            match required_str(value, "type", ActivityDecodeErrorKind::ContentInvalid)? {
                "content" => value
                    .get("content")
                    .ok_or_else(|| error(ActivityDecodeErrorKind::ContentInvalid))
                    .and_then(|content| content::block(content, limits))
                    .map(AcpToolCallContent::Content),
                "diff" => Ok(AcpToolCallContent::Diff {
                    path: required_text(value, "path", ActivityDecodeErrorKind::ContentInvalid)?,
                    old_text: optional_text(
                        value,
                        "oldText",
                        ActivityDecodeErrorKind::ContentInvalid,
                    )?,
                    new_text: required_text(
                        value,
                        "newText",
                        ActivityDecodeErrorKind::ContentInvalid,
                    )?,
                }),
                "terminal" => Ok(AcpToolCallContent::Terminal {
                    terminal_id: required_identifier(
                        value,
                        "terminalId",
                        ActivityDecodeErrorKind::ContentInvalid,
                        limits,
                    )?,
                }),
                _ => Err(error(ActivityDecodeErrorKind::ContentInvalid)),
            }
        })
        .collect()
}

fn tool_locations(
    values: &[Value],
    limits: ActivityDecodeLimits,
) -> Result<Vec<AcpToolCallLocation>, ActivityDecodeError> {
    values
        .iter()
        .map(|value| {
            let value = object(value, ActivityDecodeErrorKind::ContentInvalid)?;
            let line = value
                .get("line")
                .map(|_| {
                    required_u64(value, "line", ActivityDecodeErrorKind::ContentInvalid).and_then(
                        |line| {
                            u32::try_from(line)
                                .map_err(|_| error(ActivityDecodeErrorKind::ContentInvalid))
                        },
                    )
                })
                .transpose()?;
            Ok(AcpToolCallLocation {
                path: required_text(value, "path", ActivityDecodeErrorKind::ContentInvalid)?,
                line,
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .and_then(|locations| {
            if locations.len() > limits.maximum_collection_items() {
                Err(error(ActivityDecodeErrorKind::LimitExceeded))
            } else {
                Ok(locations)
            }
        })
}
