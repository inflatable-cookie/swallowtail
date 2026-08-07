use super::{OhMyPiRpcProtocolFailure, OhMyPiRpcProtocolFailureKind, OhMyPiRpcRecordKind};
use base64::Engine as _;
use serde_json::Value;
use swallowtail_runtime::TokenUsage;

mod ui;
mod decode;
use decode::*;

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
