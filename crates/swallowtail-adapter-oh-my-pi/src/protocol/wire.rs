use super::{OhMyPiRpcProtocolFailure, OhMyPiRpcProtocolFailureKind, OhMyPiRpcRecordKind};
use base64::Engine as _;
use serde_json::Value;
use swallowtail_runtime::TokenUsage;

mod ui;

const MAXIMUM_RECORD_BYTES: usize = 1024 * 1024;
const MAXIMUM_REASSEMBLED_BYTES: usize = 64 * 1024 * 1024;
const MAXIMUM_CHUNKS: usize = 128;

pub(crate) struct OhMyPiRpcDecoder {
    buffer: Vec<u8>,
    chunks: Option<ChunkAssembly>,
}

struct ChunkAssembly {
    id: String,
    count: usize,
    byte_length: usize,
    next_index: usize,
    bytes: Vec<u8>,
}

impl OhMyPiRpcDecoder {
    pub(crate) const fn new() -> Self {
        Self {
            buffer: Vec::new(),
            chunks: None,
        }
    }

    pub(crate) fn push(
        &mut self,
        bytes: &[u8],
    ) -> Result<Vec<OhMyPiRpcRecord>, OhMyPiRpcProtocolFailure> {
        self.buffer.extend_from_slice(bytes);
        if self.buffer.len() > MAXIMUM_RECORD_BYTES && !self.buffer.contains(&b'\n') {
            return Err(failure(OhMyPiRpcProtocolFailureKind::RecordTooLarge));
        }
        let mut records = Vec::new();
        while let Some(end) = self.buffer.iter().position(|byte| *byte == b'\n') {
            if end > MAXIMUM_RECORD_BYTES {
                return Err(failure(OhMyPiRpcProtocolFailureKind::RecordTooLarge));
            }
            let mut line: Vec<_> = self.buffer.drain(..=end).collect();
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            if let Some(record) = self.decode_physical_record(&line)? {
                records.push(record);
            }
        }
        if self.buffer.len() > MAXIMUM_RECORD_BYTES {
            return Err(failure(OhMyPiRpcProtocolFailureKind::RecordTooLarge));
        }
        Ok(records)
    }

    pub(crate) fn finish(self) -> Result<(), OhMyPiRpcProtocolFailure> {
        if !self.buffer.is_empty() {
            Err(failure(OhMyPiRpcProtocolFailureKind::MissingLfDelimiter))
        } else if self.chunks.is_some() {
            Err(failure(OhMyPiRpcProtocolFailureKind::InvalidChunk))
        } else {
            Ok(())
        }
    }

    fn decode_physical_record(
        &mut self,
        bytes: &[u8],
    ) -> Result<Option<OhMyPiRpcRecord>, OhMyPiRpcProtocolFailure> {
        if bytes.is_empty() {
            return Err(failure(OhMyPiRpcProtocolFailureKind::EmptyRecord));
        }
        let value: Value = serde_json::from_slice(bytes)
            .map_err(|_| failure(OhMyPiRpcProtocolFailureKind::MalformedJson))?;
        if value.get("type").and_then(Value::as_str) == Some("rpc_chunk") {
            return self.push_chunk(&value);
        }
        if self.chunks.is_some() {
            return Err(failure(OhMyPiRpcProtocolFailureKind::InvalidChunk));
        }
        decode_value(&value).map(Some)
    }

    fn push_chunk(
        &mut self,
        value: &Value,
    ) -> Result<Option<OhMyPiRpcRecord>, OhMyPiRpcProtocolFailure> {
        let invalid = || failure(OhMyPiRpcProtocolFailureKind::InvalidChunk);
        let id = required_text(value, "chunkId", OhMyPiRpcProtocolFailureKind::InvalidChunk)?;
        if id.len() > 128 || id.chars().any(char::is_control) {
            return Err(invalid());
        }
        let index = usize::try_from(
            value
                .get("index")
                .and_then(Value::as_u64)
                .ok_or_else(invalid)?,
        )
        .map_err(|_| invalid())?;
        let count = usize::try_from(
            value
                .get("count")
                .and_then(Value::as_u64)
                .ok_or_else(invalid)?,
        )
        .map_err(|_| invalid())?;
        let byte_length = value
            .get("byteLength")
            .and_then(Value::as_u64)
            .ok_or_else(invalid)?;
        let byte_length = usize::try_from(byte_length).map_err(|_| invalid())?;
        if !(2..=MAXIMUM_CHUNKS).contains(&count)
            || byte_length <= MAXIMUM_RECORD_BYTES
            || byte_length > MAXIMUM_REASSEMBLED_BYTES
        {
            return Err(invalid());
        }
        let data = required_text(value, "data", OhMyPiRpcProtocolFailureKind::InvalidChunk)?;
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|_| invalid())?;
        let assembly = self.chunks.get_or_insert_with(|| ChunkAssembly {
            id: id.to_owned(),
            count,
            byte_length,
            next_index: 0,
            bytes: Vec::with_capacity(byte_length),
        });
        if assembly.id != id
            || assembly.count != count
            || assembly.byte_length != byte_length
            || assembly.next_index != index
            || assembly.bytes.len().saturating_add(decoded.len()) > byte_length
        {
            return Err(invalid());
        }
        assembly.bytes.extend_from_slice(&decoded);
        assembly.next_index += 1;
        if assembly.next_index != assembly.count {
            return Ok(None);
        }
        let assembly = self.chunks.take().expect("completed chunk assembly exists");
        if assembly.bytes.len() != assembly.byte_length {
            return Err(invalid());
        }
        let value: Value = serde_json::from_slice(&assembly.bytes)
            .map_err(|_| failure(OhMyPiRpcProtocolFailureKind::MalformedJson))?;
        if value.get("type").and_then(Value::as_str) == Some("rpc_chunk") {
            return Err(invalid());
        }
        decode_value(&value).map(Some)
    }
}

pub(crate) enum OhMyPiRpcRecord {
    Response(OhMyPiRpcResponse),
    AgentEvent(OhMyPiAgentEvent),
    UiDialog(OhMyPiUiDialog),
    UiDisplay(OhMyPiUiDisplay),
    Lifecycle,
}

impl OhMyPiRpcRecord {
    pub(crate) const fn kind(&self) -> OhMyPiRpcRecordKind {
        match self {
            Self::Response(_) => OhMyPiRpcRecordKind::Response,
            Self::AgentEvent(_) => OhMyPiRpcRecordKind::AgentEvent,
            Self::UiDialog(_) => OhMyPiRpcRecordKind::ExtensionUiDialog,
            Self::UiDisplay(_) => OhMyPiRpcRecordKind::ExtensionUiDisplay,
            Self::Lifecycle => OhMyPiRpcRecordKind::Lifecycle,
        }
    }
}

pub(crate) struct OhMyPiRpcResponse {
    pub(crate) id: String,
    pub(crate) command: String,
    pub(crate) success: bool,
    pub(crate) data: Option<Value>,
}

pub(crate) enum OhMyPiAgentEvent {
    Started,
    Settled,
    MessageStarted,
    MessageEnded(Option<TokenUsage>),
    OutputDelta(String),
    ReasoningStarted,
    ReasoningDelta(String),
    ReasoningEnded,
    ToolStarted {
        call_id: String,
        name: String,
    },
    ToolUpdated {
        call_id: String,
        name: String,
    },
    ToolEnded {
        call_id: String,
        name: String,
        failed: bool,
    },
    CompactionStarted,
    CompactionEnded,
    Unknown(String),
    Progress,
    ProviderFailed,
    RetryObserved,
}

pub(crate) struct OhMyPiUiDialog {
    pub(crate) id: String,
    pub(crate) method: OhMyPiUiDialogMethod,
    pub(crate) title: String,
    pub(crate) prompt: Option<String>,
    pub(crate) options: Vec<String>,
    pub(crate) timeout_millis: Option<u64>,
}

#[derive(Clone, Copy)]
pub(crate) enum OhMyPiUiDialogMethod {
    Select,
    Confirm,
    Input,
    Editor,
}

pub(crate) struct OhMyPiUiDisplay {
    pub(crate) id: String,
    pub(crate) kind: OhMyPiUiDisplayKind,
    pub(crate) content: String,
}

#[derive(Clone, Copy)]
pub(crate) enum OhMyPiUiDisplayKind {
    Notification,
    Status,
    Widget,
    Title,
    EditorSuggestion,
}

fn decode_value(value: &Value) -> Result<OhMyPiRpcRecord, OhMyPiRpcProtocolFailure> {
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

fn decode_ready(value: &Value) -> Result<OhMyPiRpcRecord, OhMyPiRpcProtocolFailure> {
    let supported = value
        .get("supportedProtocolVersions")
        .and_then(Value::as_array)
        .ok_or_else(|| failure(OhMyPiRpcProtocolFailureKind::UnknownRecord))?;
    if value.get("protocolVersion").and_then(Value::as_u64) != Some(1)
        || supported.as_slice() != [Value::from(1), Value::from(2)]
        || value.get("maxFrameBytes").and_then(Value::as_u64) != Some(MAXIMUM_RECORD_BYTES as u64)
        || value
            .get("maxReassembledFrameBytes")
            .and_then(Value::as_u64)
            != Some(MAXIMUM_REASSEMBLED_BYTES as u64)
    {
        return Err(failure(OhMyPiRpcProtocolFailureKind::UnknownRecord));
    }
    Ok(OhMyPiRpcRecord::Lifecycle)
}

fn decode_response(value: &Value) -> Result<OhMyPiRpcResponse, OhMyPiRpcProtocolFailure> {
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

fn decode_event(kind: &str, value: &Value) -> Result<OhMyPiAgentEvent, OhMyPiRpcProtocolFailure> {
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

fn decode_message_end(value: &Value) -> Result<OhMyPiAgentEvent, OhMyPiRpcProtocolFailure> {
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

fn decode_message_start(value: &Value) -> Result<OhMyPiAgentEvent, OhMyPiRpcProtocolFailure> {
    let message = value
        .get("message")
        .ok_or_else(|| failure(OhMyPiRpcProtocolFailureKind::UnknownRecord))?;
    if message.get("role").and_then(Value::as_str) == Some("assistant") {
        Ok(OhMyPiAgentEvent::MessageStarted)
    } else {
        Ok(OhMyPiAgentEvent::Progress)
    }
}

fn decode_message_update(value: &Value) -> Result<OhMyPiAgentEvent, OhMyPiRpcProtocolFailure> {
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
enum ToolPhase {
    Started,
    Updated,
    Ended,
}

fn decode_tool(
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

fn bounded_namespace(value: &str) -> Option<String> {
    if value.is_empty() || value.len() > 96 || value.chars().any(char::is_control) {
        None
    } else {
        Some(value.to_owned())
    }
}

fn required_text<'a>(
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

fn required_u64(value: &Value, field: &str) -> Result<u64, OhMyPiRpcProtocolFailure> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| failure(OhMyPiRpcProtocolFailureKind::UnknownRecord))
}

fn failure(kind: OhMyPiRpcProtocolFailureKind) -> OhMyPiRpcProtocolFailure {
    OhMyPiRpcProtocolFailure::new(kind)
}
