use super::{
    OhMyPiRpcRecord, OhMyPiUiDialog, OhMyPiUiDialogMethod, OhMyPiUiDisplay, OhMyPiUiDisplayKind,
    required_text,
};
use crate::protocol::{OhMyPiRpcProtocolFailure, OhMyPiRpcProtocolFailureKind};
use serde_json::Value;

pub(super) fn decode_ui(value: &Value) -> Result<OhMyPiRpcRecord, OhMyPiRpcProtocolFailure> {
    let id = required_text(value, "id", OhMyPiRpcProtocolFailureKind::InvalidUiRequest)?.to_owned();
    let method = required_text(
        value,
        "method",
        OhMyPiRpcProtocolFailureKind::InvalidUiRequest,
    )?;
    match method {
        "select" | "confirm" | "input" | "editor" => {
            decode_dialog(id, method, value).map(OhMyPiRpcRecord::UiDialog)
        }
        "notify" | "setStatus" | "setWidget" | "setTitle" | "set_editor_text" => {
            decode_display(id, method, value).map(OhMyPiRpcRecord::UiDisplay)
        }
        _ => Err(failure()),
    }
}

fn decode_dialog(
    id: String,
    method: &str,
    value: &Value,
) -> Result<OhMyPiUiDialog, OhMyPiRpcProtocolFailure> {
    let method = match method {
        "select" => OhMyPiUiDialogMethod::Select,
        "confirm" => OhMyPiUiDialogMethod::Confirm,
        "input" => OhMyPiUiDialogMethod::Input,
        "editor" => OhMyPiUiDialogMethod::Editor,
        _ => return Err(failure()),
    };
    let title = required_text(
        value,
        "title",
        OhMyPiRpcProtocolFailureKind::InvalidUiRequest,
    )?;
    let prompt = ["message", "placeholder", "prefill"]
        .into_iter()
        .find_map(|field| value.get(field).and_then(Value::as_str))
        .map(str::to_owned);
    let options = value
        .get("options")
        .map(|options| {
            options
                .as_array()
                .ok_or_else(failure)?
                .iter()
                .map(|option| option.as_str().map(str::to_owned).ok_or_else(failure))
                .collect()
        })
        .transpose()?
        .unwrap_or_default();
    Ok(OhMyPiUiDialog {
        id,
        method,
        title: title.to_owned(),
        prompt,
        options,
        timeout_millis: value.get("timeout").and_then(Value::as_u64),
    })
}

fn decode_display(
    id: String,
    method: &str,
    value: &Value,
) -> Result<OhMyPiUiDisplay, OhMyPiRpcProtocolFailure> {
    let (kind, content) = match method {
        "notify" => (
            OhMyPiUiDisplayKind::Notification,
            optional_text(value, "message"),
        ),
        "setStatus" => (
            OhMyPiUiDisplayKind::Status,
            optional_text(value, "statusText"),
        ),
        "setWidget" => (
            OhMyPiUiDisplayKind::Widget,
            value
                .get("widgetLines")
                .and_then(Value::as_array)
                .map(|lines| {
                    lines
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .unwrap_or_default(),
        ),
        "setTitle" => (OhMyPiUiDisplayKind::Title, optional_text(value, "title")),
        "set_editor_text" => (
            OhMyPiUiDisplayKind::EditorSuggestion,
            optional_text(value, "text"),
        ),
        _ => return Err(failure()),
    };
    Ok(OhMyPiUiDisplay { id, kind, content })
}

fn optional_text(value: &Value, field: &str) -> String {
    value
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

fn failure() -> OhMyPiRpcProtocolFailure {
    OhMyPiRpcProtocolFailure::new(OhMyPiRpcProtocolFailureKind::InvalidUiRequest)
}
