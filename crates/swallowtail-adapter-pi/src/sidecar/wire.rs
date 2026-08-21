use super::protocol::{
    PiSdkSidecarProtocolFailure, PiSdkSidecarProtocolFailureKind, PiSdkSidecarRecordKind,
};
use serde_json::Value;
use swallowtail_runtime::TokenUsage;

mod decode;
// Card 091 consumes the typed replay payloads; the fresh-session driver fails
// closed on replay events without reading them.
#[allow(dead_code)]
pub(crate) mod replay;

pub(crate) use decode::decode_usage;
use replay::PiSdkReplayItem;

const MAXIMUM_RECORD_BYTES: usize = 1024 * 1024;
const MAXIMUM_COMMAND_ID_BYTES: usize = 128;
const MAXIMUM_FAILURE_CODE_BYTES: usize = 96;
const MAXIMUM_FAILURE_MESSAGE_BYTES: usize = 512;

pub(crate) struct PiSdkSidecarDecoder {
    buffer: Vec<u8>,
}

impl PiSdkSidecarDecoder {
    pub(crate) const fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub(crate) fn push(
        &mut self,
        bytes: &[u8],
    ) -> Result<Vec<PiSdkSidecarRecord>, PiSdkSidecarProtocolFailure> {
        self.buffer.extend_from_slice(bytes);
        if self.buffer.len() > MAXIMUM_RECORD_BYTES && !self.buffer.contains(&b'\n') {
            return Err(failure(PiSdkSidecarProtocolFailureKind::RecordTooLarge));
        }
        let mut records = Vec::new();
        while let Some(end) = self.buffer.iter().position(|byte| *byte == b'\n') {
            if end > MAXIMUM_RECORD_BYTES {
                return Err(failure(PiSdkSidecarProtocolFailureKind::RecordTooLarge));
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

    pub(crate) fn finish(self) -> Result<(), PiSdkSidecarProtocolFailure> {
        if self.buffer.is_empty() {
            Ok(())
        } else {
            Err(failure(PiSdkSidecarProtocolFailureKind::MissingLfDelimiter))
        }
    }
}

pub(crate) enum PiSdkSidecarRecord {
    Response(PiSdkSidecarResponse),
    Event(PiSdkSidecarEvent),
    // Redacted payloads are proven by the frozen corpus; the driver maps
    // terminal records to one distinct safe failure without surfacing them.
    #[allow(dead_code)]
    Terminal(PiSdkSidecarFailure),
    #[allow(dead_code)]
    Diagnostic(PiSdkSidecarDiagnostic),
}

impl PiSdkSidecarRecord {
    pub(crate) const fn kind(&self) -> PiSdkSidecarRecordKind {
        match self {
            Self::Response(_) => PiSdkSidecarRecordKind::Response,
            Self::Event(_) => PiSdkSidecarRecordKind::Event,
            Self::Terminal(_) => PiSdkSidecarRecordKind::Terminal,
            Self::Diagnostic(_) => PiSdkSidecarRecordKind::Diagnostic,
        }
    }
}

pub(crate) struct PiSdkSidecarResponse {
    pub(crate) id: String,
    pub(crate) command: String,
    pub(crate) success: bool,
    pub(crate) data: Option<Value>,
    // The bounded sidecar failure payload stays redacted; drivers observe
    // only the success flag.
    #[allow(dead_code)]
    pub(crate) failure: Option<PiSdkSidecarFailure>,
}

#[allow(dead_code)]
pub(crate) struct PiSdkSidecarFailure {
    pub(crate) code: String,
    pub(crate) message: String,
}

#[allow(dead_code)]
pub(crate) struct PiSdkSidecarDiagnostic {
    pub(crate) level: PiSdkSidecarDiagnosticLevel,
    pub(crate) code: String,
    pub(crate) message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PiSdkSidecarDiagnosticLevel {
    Info,
    Warning,
    Error,
}

pub(crate) enum PiSdkSidecarEvent {
    Started,
    TurnStarted,
    TurnEnded,
    Ended,
    Settled,
    Progress,
    MessageStarted,
    MessageEnded {
        stop_reason: String,
        usage: Option<TokenUsage>,
    },
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
    // Card 091 projects replay items during session load; fresh turns fail
    // closed before reading the payload.
    #[allow(dead_code)]
    ReplayItem {
        sequence: u64,
        item: PiSdkReplayItem,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PiSdkSidecarCommand {
    Bootstrap,
    SessionNew,
    SessionSwitch,
    SessionReplay,
    Prompt,
    Steer,
    FollowUp,
    Abort,
    State,
    Close,
}

impl PiSdkSidecarCommand {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Bootstrap => "bootstrap",
            Self::SessionNew => "session_new",
            Self::SessionSwitch => "session_switch",
            Self::SessionReplay => "session_replay",
            Self::Prompt => "prompt",
            Self::Steer => "steer",
            Self::FollowUp => "follow_up",
            Self::Abort => "abort",
            Self::State => "state",
            Self::Close => "close",
        }
    }

    fn from_qualified(name: &str) -> Option<Self> {
        Some(match name {
            "bootstrap" => Self::Bootstrap,
            "session_new" => Self::SessionNew,
            "session_switch" => Self::SessionSwitch,
            "session_replay" => Self::SessionReplay,
            "prompt" => Self::Prompt,
            "steer" => Self::Steer,
            "follow_up" => Self::FollowUp,
            "abort" => Self::Abort,
            "state" => Self::State,
            "close" => Self::Close,
            _ => return None,
        })
    }
}

pub(crate) fn encode_command(
    id: &str,
    command: PiSdkSidecarCommand,
    params: Value,
) -> Result<Vec<u8>, PiSdkSidecarProtocolFailure> {
    let record = serde_json::json!({
        "type": "command",
        "id": id,
        "command": command.as_str(),
        "params": params,
    });
    let mut bytes = serde_json::to_vec(&record)
        .map_err(|_| failure(PiSdkSidecarProtocolFailureKind::MalformedJson))?;
    if bytes.len() + 1 > MAXIMUM_RECORD_BYTES {
        return Err(failure(PiSdkSidecarProtocolFailureKind::RecordTooLarge));
    }
    bytes.push(b'\n');
    Ok(bytes)
}

pub(crate) fn decode_record(
    bytes: &[u8],
) -> Result<PiSdkSidecarRecord, PiSdkSidecarProtocolFailure> {
    if bytes.is_empty() {
        return Err(failure(PiSdkSidecarProtocolFailureKind::EmptyRecord));
    }
    let value: Value = serde_json::from_slice(bytes)
        .map_err(|_| failure(PiSdkSidecarProtocolFailureKind::MalformedJson))?;
    match value.get("type").and_then(Value::as_str) {
        Some("response") => decode::decode_response(&value).map(PiSdkSidecarRecord::Response),
        Some("event") => decode::decode_event(&value).map(PiSdkSidecarRecord::Event),
        Some("terminal") => decode::decode_terminal(&value).map(PiSdkSidecarRecord::Terminal),
        Some("diagnostic") => decode::decode_diagnostic(&value).map(PiSdkSidecarRecord::Diagnostic),
        Some(_) => Err(failure(PiSdkSidecarProtocolFailureKind::UnknownRecord)),
        None => Err(failure(PiSdkSidecarProtocolFailureKind::MissingType)),
    }
}

pub(crate) fn required_text<'a>(
    value: &'a Value,
    field: &str,
    kind: PiSdkSidecarProtocolFailureKind,
) -> Result<&'a str, PiSdkSidecarProtocolFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| failure(kind))
}

pub(crate) fn required_string<'a>(
    value: &'a Value,
    field: &str,
    kind: PiSdkSidecarProtocolFailureKind,
) -> Result<&'a str, PiSdkSidecarProtocolFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| failure(kind))
}

pub(super) fn bounded_text<'a>(
    value: &'a Value,
    field: &str,
    maximum: usize,
    kind: PiSdkSidecarProtocolFailureKind,
) -> Result<&'a str, PiSdkSidecarProtocolFailure> {
    required_text(value, field, kind).and_then(|text| {
        if text.len() > maximum || text.chars().any(char::is_control) {
            Err(failure(kind))
        } else {
            Ok(text)
        }
    })
}

pub(crate) fn required_u64(
    value: &Value,
    field: &str,
    kind: PiSdkSidecarProtocolFailureKind,
) -> Result<u64, PiSdkSidecarProtocolFailure> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| failure(kind))
}

pub(crate) fn failure(kind: PiSdkSidecarProtocolFailureKind) -> PiSdkSidecarProtocolFailure {
    PiSdkSidecarProtocolFailure::new(kind)
}

#[cfg(test)]
mod tests;
