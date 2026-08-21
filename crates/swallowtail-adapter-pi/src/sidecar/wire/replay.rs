use super::super::protocol::{PiSdkSidecarProtocolFailure, PiSdkSidecarProtocolFailureKind};
use super::{
    PiSdkSidecarEvent, decode_usage, failure, required_string, required_text, required_u64,
};
use serde_json::Value;
use swallowtail_runtime::TokenUsage;

pub(crate) enum PiSdkReplayItem {
    // Replay projection carries text only; image counts, stop reasons, usage,
    // tool names, and failure flags stay inside the adapter by design.
    #[allow(dead_code)]
    User { text: String, images: u64 },
    #[allow(dead_code)]
    Assistant {
        parts: Vec<PiSdkReplayPart>,
        stop_reason: String,
        usage: Option<TokenUsage>,
    },
    #[allow(dead_code)]
    ToolResult {
        name: String,
        failed: bool,
        text: String,
    },
}

pub(crate) enum PiSdkReplayPart {
    Text(String),
    Reasoning(String),
    #[allow(dead_code)]
    ToolCall {
        name: String,
        arguments: Value,
    },
}

pub(crate) fn decode_replay_item(
    value: &Value,
) -> Result<PiSdkSidecarEvent, PiSdkSidecarProtocolFailure> {
    let invalid = PiSdkSidecarProtocolFailureKind::InvalidEvent;
    let sequence = required_u64(value, "sequence", invalid)?;
    let item = value.get("item").ok_or_else(|| failure(invalid))?;
    let item = match required_text(item, "kind", invalid)? {
        "user" => PiSdkReplayItem::User {
            text: required_string(item, "text", invalid)?.to_owned(),
            images: required_u64(item, "images", invalid)?,
        },
        "assistant" => {
            let parts = item
                .get("parts")
                .and_then(Value::as_array)
                .ok_or_else(|| failure(invalid))?
                .iter()
                .map(decode_replay_part)
                .collect::<Result<_, _>>()?;
            let usage = item
                .get("usage")
                .map(|usage| decode_usage(usage, invalid))
                .transpose()?;
            PiSdkReplayItem::Assistant {
                parts,
                stop_reason: required_text(item, "stopReason", invalid)?.to_owned(),
                usage,
            }
        }
        "tool_result" => PiSdkReplayItem::ToolResult {
            name: required_text(item, "toolName", invalid)?.to_owned(),
            failed: item
                .get("isError")
                .and_then(Value::as_bool)
                .ok_or_else(|| failure(invalid))?,
            text: required_string(item, "text", invalid)?.to_owned(),
        },
        _ => return Err(failure(PiSdkSidecarProtocolFailureKind::UnknownRecord)),
    };
    Ok(PiSdkSidecarEvent::ReplayItem { sequence, item })
}

fn decode_replay_part(value: &Value) -> Result<PiSdkReplayPart, PiSdkSidecarProtocolFailure> {
    let invalid = PiSdkSidecarProtocolFailureKind::InvalidEvent;
    match value.get("type").and_then(Value::as_str) {
        Some("text") => Ok(PiSdkReplayPart::Text(
            required_string(value, "text", invalid)?.to_owned(),
        )),
        Some("thinking") => Ok(PiSdkReplayPart::Reasoning(
            required_string(value, "thinking", invalid)?.to_owned(),
        )),
        Some("tool_call") => Ok(PiSdkReplayPart::ToolCall {
            name: required_text(value, "name", invalid)?.to_owned(),
            arguments: value
                .get("arguments")
                .filter(|arguments| arguments.is_object())
                .cloned()
                .ok_or_else(|| failure(invalid))?,
        }),
        _ => Err(failure(invalid)),
    }
}
