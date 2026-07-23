use super::{PiRpcProtocolFailure, PiRpcProtocolFailureKind, PiRpcRecordKind};
use serde_json::Value;

mod ui;

const MAXIMUM_RECORD_BYTES: usize = 1024 * 1024;

pub(crate) struct PiRpcDecoder {
    buffer: Vec<u8>,
}

impl PiRpcDecoder {
    pub(crate) const fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub(crate) fn push(&mut self, bytes: &[u8]) -> Result<Vec<PiRpcRecord>, PiRpcProtocolFailure> {
        self.buffer.extend_from_slice(bytes);
        if self.buffer.len() > MAXIMUM_RECORD_BYTES && !self.buffer.contains(&b'\n') {
            return Err(failure(PiRpcProtocolFailureKind::RecordTooLarge));
        }
        let mut records = Vec::new();
        while let Some(end) = self.buffer.iter().position(|byte| *byte == b'\n') {
            if end > MAXIMUM_RECORD_BYTES {
                return Err(failure(PiRpcProtocolFailureKind::RecordTooLarge));
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

    pub(crate) fn finish(self) -> Result<(), PiRpcProtocolFailure> {
        if self.buffer.is_empty() {
            Ok(())
        } else {
            Err(failure(PiRpcProtocolFailureKind::MissingLfDelimiter))
        }
    }
}

pub(crate) enum PiRpcRecord {
    Response(PiRpcResponse),
    AgentEvent(PiAgentEvent),
    UiDialog(PiUiDialog),
    UiDisplay(PiUiDisplay),
}

impl PiRpcRecord {
    pub(crate) const fn kind(&self) -> PiRpcRecordKind {
        match self {
            Self::Response(_) => PiRpcRecordKind::Response,
            Self::AgentEvent(_) => PiRpcRecordKind::AgentEvent,
            Self::UiDialog(_) => PiRpcRecordKind::ExtensionUiDialog,
            Self::UiDisplay(_) => PiRpcRecordKind::ExtensionUiDisplay,
        }
    }
}

pub(crate) struct PiRpcResponse {
    pub(crate) id: String,
    pub(crate) command: String,
    pub(crate) success: bool,
    pub(crate) data: Option<Value>,
}

pub(crate) enum PiAgentEvent {
    Started,
    Settled,
    OutputDelta(String),
    ReasoningDelta(String),
    Progress,
    ProviderFailed,
    RetryObserved,
}

pub(crate) struct PiUiDialog {
    pub(crate) id: String,
    pub(crate) method: PiUiDialogMethod,
    pub(crate) title: String,
    pub(crate) prompt: Option<String>,
    pub(crate) options: Vec<String>,
    pub(crate) timeout_millis: Option<u64>,
}

#[derive(Clone, Copy)]
pub(crate) enum PiUiDialogMethod {
    Select,
    Confirm,
    Input,
    Editor,
}

pub(crate) struct PiUiDisplay {
    pub(crate) id: String,
    pub(crate) kind: PiUiDisplayKind,
    pub(crate) content: String,
}

#[derive(Clone, Copy)]
pub(crate) enum PiUiDisplayKind {
    Notification,
    Status,
    Widget,
    Title,
    EditorSuggestion,
}

pub(crate) fn decode_record(bytes: &[u8]) -> Result<PiRpcRecord, PiRpcProtocolFailure> {
    if bytes.is_empty() {
        return Err(failure(PiRpcProtocolFailureKind::EmptyRecord));
    }
    let value: Value = serde_json::from_slice(bytes)
        .map_err(|_| failure(PiRpcProtocolFailureKind::MalformedJson))?;
    match value.get("type").and_then(Value::as_str) {
        Some("response") => decode_response(&value).map(PiRpcRecord::Response),
        Some("extension_ui_request") => ui::decode_ui(&value),
        Some(kind) => decode_event(kind, &value).map(PiRpcRecord::AgentEvent),
        None => Err(failure(PiRpcProtocolFailureKind::MissingType)),
    }
}

fn decode_response(value: &Value) -> Result<PiRpcResponse, PiRpcProtocolFailure> {
    Ok(PiRpcResponse {
        id: required_text(value, "id", PiRpcProtocolFailureKind::InvalidResponse)?.to_owned(),
        command: required_text(value, "command", PiRpcProtocolFailureKind::InvalidResponse)?
            .to_owned(),
        success: value
            .get("success")
            .and_then(Value::as_bool)
            .ok_or_else(|| failure(PiRpcProtocolFailureKind::InvalidResponse))?,
        data: value.get("data").cloned(),
    })
}

fn decode_event(kind: &str, value: &Value) -> Result<PiAgentEvent, PiRpcProtocolFailure> {
    match kind {
        "agent_start" => Ok(PiAgentEvent::Started),
        "agent_settled" => Ok(PiAgentEvent::Settled),
        "message_update" => decode_message_update(value),
        "message_end"
            if value
                .get("message")
                .and_then(|message| message.get("stopReason"))
                .and_then(Value::as_str)
                == Some("error") =>
        {
            Ok(PiAgentEvent::ProviderFailed)
        }
        "agent_end" if value.get("willRetry").and_then(Value::as_bool) == Some(true) => {
            Ok(PiAgentEvent::RetryObserved)
        }
        "auto_retry_start" | "auto_retry_end" => Ok(PiAgentEvent::RetryObserved),
        "agent_end"
        | "turn_start"
        | "turn_end"
        | "message_start"
        | "message_end"
        | "tool_execution_start"
        | "tool_execution_update"
        | "tool_execution_end"
        | "queue_update" => Ok(PiAgentEvent::Progress),
        "extension_error" => Ok(PiAgentEvent::ProviderFailed),
        _ => Err(failure(PiRpcProtocolFailureKind::UnknownRecord)),
    }
}

fn decode_message_update(value: &Value) -> Result<PiAgentEvent, PiRpcProtocolFailure> {
    let event = value
        .get("assistantMessageEvent")
        .ok_or_else(|| failure(PiRpcProtocolFailureKind::UnknownRecord))?;
    match event.get("type").and_then(Value::as_str) {
        Some("text_delta") => {
            required_text(event, "delta", PiRpcProtocolFailureKind::UnknownRecord)
                .map(|delta| PiAgentEvent::OutputDelta(delta.to_owned()))
        }
        Some("thinking_delta") => {
            required_text(event, "delta", PiRpcProtocolFailureKind::UnknownRecord)
                .map(|delta| PiAgentEvent::ReasoningDelta(delta.to_owned()))
        }
        Some(
            "start" | "text_start" | "text_end" | "thinking_start" | "thinking_end"
            | "toolcall_start" | "toolcall_delta" | "toolcall_end" | "done" | "error",
        ) => Ok(PiAgentEvent::Progress),
        _ => Err(failure(PiRpcProtocolFailureKind::UnknownRecord)),
    }
}

fn required_text<'a>(
    value: &'a Value,
    field: &str,
    kind: PiRpcProtocolFailureKind,
) -> Result<&'a str, PiRpcProtocolFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| failure(kind))
}

fn failure(kind: PiRpcProtocolFailureKind) -> PiRpcProtocolFailure {
    PiRpcProtocolFailure::new(kind)
}
